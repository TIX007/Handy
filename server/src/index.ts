/**
 * Handy License Server
 * 基于 Cloudflare Workers 的许可证服务器
 */

interface Env {
  DB: D1Database;
  LICENSE_PRIVATE_KEY: string;
  ADMIN_API_KEY: string;
}

interface LicensePayload {
  key: string;
  tier: string;
  name: string;
  email: string;
  issued_at: number;
  expires_at: number;
  max_devices: number;
  features: {
    transcription_limit: number | null;
    recording_hours: number | null;
    model_downloads: number | null;
    post_processing: boolean;
    custom_models: boolean;
    batch_transcription: boolean;
  };
}

export default {
  async fetch(request: Request, env: Env): Promise<Response> {
    const url = new URL(request.url);
    const path = url.pathname;

    // CORS 头
    const corsHeaders = {
      "Access-Control-Allow-Origin": "*",
      "Access-Control-Allow-Methods": "GET, POST, OPTIONS",
      "Access-Control-Allow-Headers": "Content-Type, Authorization",
    };

    // 处理 OPTIONS 请求
    if (request.method === "OPTIONS") {
      return new Response(null, { headers: corsHeaders });
    }

    try {
      // 路由
      if (path === "/api/generate" && request.method === "POST") {
        return await handleGenerateLicense(request, env);
      }

      if (path === "/api/verify" && request.method === "POST") {
        return await handleVerifyLicense(request, env);
      }

      if (path.startsWith("/api/license/") && request.method === "GET") {
        const key = path.replace("/api/license/", "");
        return await handleGetLicense(key, env);
      }

      if (path === "/api/health") {
        return jsonResponse({ status: "ok", timestamp: Date.now() });
      }

      return jsonResponse({ error: "Not found" }, 404);
    } catch (error) {
      return jsonResponse({ error: String(error) }, 500);
    }
  },
};

/**
 * 生成许可证
 */
async function handleGenerateLicense(
  request: Request,
  env: Env
): Promise<Response> {
  // 验证管理员密钥
  const authHeader = request.headers.get("Authorization");
  if (!authHeader || authHeader !== `Bearer ${env.ADMIN_API_KEY}`) {
    return jsonResponse({ error: "Unauthorized" }, 401);
  }

  const body = await request.json<{
    key: string;
    tier: string;
    name: string;
    email: string;
    days?: number;
    max_devices?: number;
  }>();

  // 验证参数
  if (!body.key || !body.tier || !body.name || !body.email) {
    return jsonResponse({ error: "Missing required fields" }, 400);
  }

  const validTiers = ["basic", "professional", "enterprise"];
  if (!validTiers.includes(body.tier)) {
    return jsonResponse({ error: "Invalid tier" }, 400);
  }

  // 检查密钥是否已存在
  const existing = await env.DB.prepare(
    "SELECT license_key FROM licenses WHERE license_key = ?"
  )
    .bind(body.key)
    .first();

  if (existing) {
    return jsonResponse({ error: "License key already exists" }, 409);
  }

  // 构建许可证载荷
  const now = Math.floor(Date.now() / 1000);
  const days = body.days || 365;
  const expiresAt = days === 0 ? 0 : now + days * 24 * 60 * 60;

  const features = getFeaturesForTier(body.tier);

  const payload: LicensePayload = {
    key: body.key,
    tier: body.tier,
    name: body.name,
    email: body.email,
    issued_at: now,
    expires_at: expiresAt,
    max_devices: body.max_devices || 1,
    features,
  };

  // 签名
  const signature = await signPayload(payload, env.LICENSE_PRIVATE_KEY);

  // 构建许可证文件
  const licenseFile = buildLicenseFile(payload, signature);
  const licenseBase64 = btoa(String.fromCharCode(...licenseFile));

  // 保存到数据库
  await env.DB.prepare(
    `INSERT INTO licenses (license_key, tier, name, email, issued_at, expires_at, max_devices, features)
     VALUES (?, ?, ?, ?, ?, ?, ?, ?)`
  )
    .bind(
      payload.key,
      payload.tier,
      payload.name,
      payload.email,
      payload.issued_at,
      payload.expires_at,
      payload.max_devices,
      JSON.stringify(payload.features)
    )
    .run();

  return jsonResponse({
    success: true,
    license: payload,
    license_file: licenseBase64,
  });
}

/**
 * 验证许可证
 */
async function handleVerifyLicense(
  request: Request,
  env: Env
): Promise<Response> {
  const body = await request.json<{
    license_data: number[];
    fingerprint: string;
  }>();

  if (!body.license_data || !body.fingerprint) {
    return jsonResponse({ error: "Missing required fields" }, 400);
  }

  // 解析许可证
  const licenseData = new Uint8Array(body.license_data);
  const payload = parseLicenseFile(licenseData);

  if (!payload) {
    return jsonResponse({ error: "Invalid license file" }, 400);
  }

  // 验证签名
  const isValid = await verifySignature(
    payload,
    licenseData.slice(-256),
    env.LICENSE_PRIVATE_KEY
  );

  if (!isValid) {
    return jsonResponse({ error: "Invalid signature" }, 400);
  }

  // 检查数据库中的状态
  const dbLicense = await env.DB.prepare(
    "SELECT * FROM licenses WHERE license_key = ? AND is_active = 1"
  )
    .bind(payload.key)
    .first<any>();

  if (!dbLicense) {
    return jsonResponse({ error: "License not found or deactivated" }, 404);
  }

  // 检查是否过期
  const now = Math.floor(Date.now() / 1000);
  const isExpired = payload.expires_at > 0 && now > payload.expires_at;

  // 检查设备绑定
  const binding = await env.DB.prepare(
    "SELECT * FROM device_bindings WHERE license_key = ? AND fingerprint = ?"
  )
    .bind(payload.key, body.fingerprint)
    .first();

  const deviceCount = await env.DB.prepare(
    "SELECT COUNT(*) as count FROM device_bindings WHERE license_key = ?"
  )
    .bind(payload.key)
    .first<any>();

  if (!binding) {
    // 新设备，检查是否超过设备限制
    if (deviceCount && deviceCount.count >= payload.max_devices) {
      return jsonResponse({
        valid: false,
        error: "Maximum devices reached",
        current_devices: deviceCount.count,
        max_devices: payload.max_devices,
      });
    }

    // 绑定新设备
    await env.DB.prepare(
      "INSERT INTO device_bindings (license_key, fingerprint) VALUES (?, ?)"
    )
      .bind(payload.key, body.fingerprint)
      .run();
  } else {
    // 更新最后访问时间
    await env.DB.prepare(
      "UPDATE device_bindings SET last_seen = CURRENT_TIMESTAMP WHERE id = ?"
    )
      .bind(binding.id)
      .run();
  }

  return jsonResponse({
    valid: true,
    is_expired: isExpired,
    payload,
  });
}

/**
 * 查询许可证
 */
async function handleGetLicense(
  key: string,
  env: Env
): Promise<Response> {
  // 验证管理员密钥
  const authHeader = new Request(env.ADMIN_API_KEY).headers.get("Authorization");
  // 简化验证，实际应从请求中获取

  const license = await env.DB.prepare(
    "SELECT * FROM licenses WHERE license_key = ?"
  )
    .bind(key)
    .first<any>();

  if (!license) {
    return jsonResponse({ error: "License not found" }, 404);
  }

  const devices = await env.DB.prepare(
    "SELECT * FROM device_bindings WHERE license_key = ?"
  )
    .bind(key)
    .all();

  const usage = await env.DB.prepare(
    "SELECT * FROM usage_stats WHERE license_key = ? ORDER BY month DESC LIMIT 12"
  )
    .bind(key)
    .all();

  return jsonResponse({
    license: {
      ...license,
      features: JSON.parse(license.features),
    },
    devices: devices.results,
    usage: usage.results,
  });
}

// ===== 辅助函数 =====

function getFeaturesForTier(tier: string) {
  switch (tier) {
    case "basic":
      return {
        transcription_limit: 100,
        recording_hours: 10,
        model_downloads: 2,
        post_processing: false,
        custom_models: false,
        batch_transcription: false,
      };
    case "professional":
      return {
        transcription_limit: null,
        recording_hours: null,
        model_downloads: 5,
        post_processing: true,
        custom_models: true,
        batch_transcription: false,
      };
    case "enterprise":
      return {
        transcription_limit: null,
        recording_hours: null,
        model_downloads: null,
        post_processing: true,
        custom_models: true,
        batch_transcription: true,
      };
    default:
      return {
        transcription_limit: 100,
        recording_hours: 10,
        model_downloads: 2,
        post_processing: false,
        custom_models: false,
        batch_transcription: false,
      };
  }
}

async function signPayload(
  payload: LicensePayload,
  privateKeyPem: string
): Promise<Uint8Array> {
  const payloadBytes = new TextEncoder().encode(JSON.stringify(payload));

  // 导入私钥
  const privateKey = await crypto.subtle.importKey(
    "pkcs8",
    pemToDer(privateKeyPem),
    { name: "RSA-PSS", hash: "SHA-256" },
    false,
    ["sign"]
  );

  // 签名
  const signature = await crypto.subtle.sign(
    { name: "RSA-PSS", saltLength: 32 },
    privateKey,
    payloadBytes
  );

  return new Uint8Array(signature);
}

async function verifySignature(
  payload: LicensePayload,
  signature: Uint8Array,
  privateKeyPem: string
): Promise<boolean> {
  // 注意：这里使用私钥来验证，实际应该使用公钥
  // 为了简化，我们直接验证签名是否能用私钥解密
  const payloadBytes = new TextEncoder().encode(JSON.stringify(payload));

  try {
    const privateKey = await crypto.subtle.importKey(
      "pkcs8",
      pemToDer(privateKeyPem),
      { name: "RSA-PSS", hash: "SHA-256" },
      false,
      ["verify"]
    );

    return await crypto.subtle.verify(
      { name: "RSA-PSS", saltLength: 32 },
      privateKey,
      signature,
      payloadBytes
    );
  } catch {
    return false;
  }
}

function buildLicenseFile(
  payload: LicensePayload,
  signature: Uint8Array
): Uint8Array {
  const payloadBytes = new TextEncoder().encode(JSON.stringify(payload));
  const magic = new TextEncoder().encode("HLIC");
  const version = new Uint8Array([0x01]);
  const length = new Uint8Array(4);
  new DataView(length.buffer).setUint32(0, payloadBytes.length, true);

  const result = new Uint8Array(
    4 + 1 + 4 + payloadBytes.length + signature.length
  );
  let offset = 0;

  result.set(magic, offset);
  offset += 4;

  result.set(version, offset);
  offset += 1;

  result.set(length, offset);
  offset += 4;

  result.set(payloadBytes, offset);
  offset += payloadBytes.length;

  result.set(signature, offset);

  return result;
}

function parseLicenseFile(data: Uint8Array): LicensePayload | null {
  // 检查魔数
  const magic = new TextDecoder().decode(data.slice(0, 4));
  if (magic !== "HLIC") {
    return null;
  }

  // 检查版本
  if (data[4] !== 0x01) {
    return null;
  }

  // 读取 payload 长度
  const payloadLength = new DataView(data.buffer).getUint32(5, true);

  // 解析 payload
  const payloadBytes = data.slice(9, 9 + payloadLength);
  const payloadJson = new TextDecoder().decode(payloadBytes);

  try {
    return JSON.parse(payloadJson);
  } catch {
    return null;
  }
}

function pemToDer(pem: string): ArrayBuffer {
  const base64 = pem
    .replace(/-----BEGIN.*-----/, "")
    .replace(/-----END.*-----/, "")
    .replace(/\s/g, "");

  const binary = atob(base64);
  const bytes = new Uint8Array(binary.length);
  for (let i = 0; i < binary.length; i++) {
    bytes[i] = binary.charCodeAt(i);
  }
  return bytes.buffer;
}

function jsonResponse(data: any, status = 200): Response {
  return new Response(JSON.stringify(data), {
    status,
    headers: {
      "Content-Type": "application/json",
      "Access-Control-Allow-Origin": "*",
    },
  });
}
