use anyhow::{anyhow, Result};
use chrono::Utc;
use clap::Parser;
use rsa::pkcs1::DecodeRsaPrivateKey;
use rsa::sha2::Sha256;
use rsa::signature::Signer;
use rsa::{Pkcs1v15Sign, RsaPrivateKey};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// 许可证生成工具
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// 许可证密钥 (XXXX-XXXX-XXXX-XXXX)
    #[arg(short, long)]
    key: String,

    /// 许可证等级 (basic, professional, enterprise)
    #[arg(short, long, value_enum, default_value = "professional")]
    tier: Tier,

    /// 用户名称
    #[arg(long)]
    name: String,

    /// 用户邮箱
    #[arg(long)]
    email: String,

    /// 有效期天数 (0 表示永不过期)
    #[arg(long, default_value = "365")]
    days: u32,

    /// 最大设备数
    #[arg(long, default_value = "1")]
    max_devices: u32,

    /// 私钥文件路径
    #[arg(long, default_value = "src-tauri/resources/license/private_key.pem")]
    private_key: PathBuf,

    /// 输出文件路径
    #[arg(short, long, default_value = "license.lic")]
    output: PathBuf,

    /// 每月转录次数限制 (0 表示无限制)
    #[arg(long)]
    transcription_limit: Option<u32>,

    /// 录音时长限制（小时/月，0 表示无限制）
    #[arg(long)]
    recording_hours: Option<f64>,

    /// 可下载模型数量限制 (0 表示无限制)
    #[arg(long)]
    model_downloads: Option<u32>,

    /// 是否包含后处理功能
    #[arg(long)]
    post_processing: bool,

    /// 是否包含自定义模型功能
    #[arg(long)]
    custom_models: bool,

    /// 是否包含批量转录功能
    #[arg(long)]
    batch_transcription: bool,
}

#[derive(Debug, Clone, clap::ValueEnum)]
enum Tier {
    Basic,
    Professional,
    Enterprise,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct LicensePayload {
    key: String,
    tier: String,
    name: String,
    email: String,
    issued_at: i64,
    expires_at: i64,
    max_devices: u32,
    features: LicenseFeatures,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct LicenseFeatures {
    transcription_limit: Option<u32>,
    recording_hours: Option<f64>,
    model_downloads: Option<u32>,
    post_processing: bool,
    custom_models: bool,
    batch_transcription: bool,
}

const LICENSE_MAGIC: &[u8; 4] = b"HLIC";
const LICENSE_VERSION: u8 = 0x01;

fn main() -> Result<()> {
    let args = Args::parse();

    // 验证参数
    validate_args(&args)?;

    // 加载私钥
    let private_key = load_private_key(&args.private_key)?;

    // 构建许可证载荷
    let payload = build_payload(&args)?;

    // 签名
    let signature = sign_payload(&payload, &private_key)?;

    // 构建许可证文件
    let license_file = build_license_file(&payload, &signature)?;

    // 写入文件
    std::fs::write(&args.output, &license_file)?;

    println!("✓ 许可证已生成: {}", args.output.display());
    println!("  密钥: {}", args.key);
    println!("  等级: {:?}", args.tier);
    println!("  用户: {} <{}>", args.name, args.email);

    Ok(())
}

fn validate_args(args: &Args) -> Result<()> {
    // 验证密钥格式
    if !args.key.chars().all(|c| c.is_ascii_alphanumeric() || c == '-') {
        return Err(anyhow!("许可证密钥只能包含字母、数字和连字符"));
    }

    // 验证邮箱格式
    if !args.email.contains('@') {
        return Err(anyhow!("邮箱格式无效"));
    }

    // 验证私钥文件存在
    if !args.private_key.exists() {
        return Err(anyhow!("私钥文件不存在: {}", args.private_key.display()));
    }

    Ok(())
}

fn load_private_key(path: &PathBuf) -> Result<RsaPrivateKey> {
    let pem_content = std::fs::read_to_string(path)?;
    RsaPrivateKey::from_pkcs1_pem(&pem_content)
        .map_err(|e| anyhow!("无法加载私钥: {}", e))
}

fn build_payload(args: &Args) -> Result<LicensePayload> {
    let now = Utc::now().timestamp();
    let expires_at = if args.days == 0 {
        0
    } else {
        now + (args.days as i64 * 24 * 60 * 60)
    };

    let tier = match args.tier {
        Tier::Basic => "basic",
        Tier::Professional => "professional",
        Tier::Enterprise => "enterprise",
    };

    let features = match args.tier {
        Tier::Basic => LicenseFeatures {
            transcription_limit: Some(args.transcription_limit.unwrap_or(100)),
            recording_hours: Some(args.recording_hours.unwrap_or(10.0)),
            model_downloads: Some(args.model_downloads.unwrap_or(2)),
            post_processing: false,
            custom_models: false,
            batch_transcription: false,
        },
        Tier::Professional => LicenseFeatures {
            transcription_limit: None,
            recording_hours: None,
            model_downloads: Some(args.model_downloads.unwrap_or(5)),
            post_processing: true,
            custom_models: true,
            batch_transcription: false,
        },
        Tier::Enterprise => LicenseFeatures {
            transcription_limit: None,
            recording_hours: None,
            model_downloads: None,
            post_processing: true,
            custom_models: true,
            batch_transcription: true,
        },
    };

    Ok(LicensePayload {
        key: args.key.clone(),
        tier: tier.to_string(),
        name: args.name.clone(),
        email: args.email.clone(),
        issued_at: now,
        expires_at,
        max_devices: args.max_devices,
        features,
    })
}

fn sign_payload(payload: &LicensePayload, private_key: &RsaPrivateKey) -> Result<Vec<u8>> {
    let payload_bytes = serde_json::to_vec(payload)?;
    let signing_key = rsa::pss::SigningKey::<Sha256>::new(private_key.clone());
    let signature = signing_key.sign(&payload_bytes);
    Ok(signature.as_ref().to_vec())
}

fn build_license_file(payload: &LicensePayload, signature: &[u8]) -> Result<Vec<u8>> {
    let payload_bytes = serde_json::to_vec(payload)?;

    let mut result = Vec::new();
    result.extend_from_slice(LICENSE_MAGIC);
    result.push(LICENSE_VERSION);
    result.extend_from_slice(&(payload_bytes.len() as u32).to_le_bytes());
    result.extend_from_slice(&payload_bytes);
    result.extend_from_slice(signature);

    Ok(result)
}
