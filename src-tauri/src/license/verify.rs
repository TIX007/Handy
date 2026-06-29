use anyhow::{anyhow, Result};
use rsa::pkcs1::DecodeRsaPublicKey;
use rsa::sha2::Sha256;
use rsa::signature::Verifier;
use rsa::{Pkcs1v15Sign, RsaPublicKey};

use super::types::{LicenseFile, LicensePayload, LICENSE_MAGIC, LICENSE_VERSION};

/// 嵌入的 RSA 公钥（编译时包含）
/// 生成密钥对后，将公钥保存为 PEM 格式并放在此处
const PUBLIC_KEY_PEM: &[u8] = include_bytes!("../../resources/license/public_key.pem");

/// 解析许可证文件
pub fn parse_license_file(data: &[u8]) -> Result<LicenseFile> {
    // 检查最小长度
    if data.len() < 13 {
        return Err(anyhow!("许可证文件太小"));
    }

    // 检查魔数
    if &data[0..4] != LICENSE_MAGIC {
        return Err(anyhow!("无效的许可证文件格式"));
    }

    // 检查版本
    let version = data[4];
    if version != LICENSE_VERSION {
        return Err(anyhow!("不支持的许可证版本: {}", version));
    }

    // 读取 payload 长度（小端序）
    let payload_len = u32::from_le_bytes([data[5], data[6], data[7], data[8]]) as usize;

    // 检查数据完整性
    let expected_len = 9 + payload_len + 256; // header + payload + signature
    if data.len() < expected_len {
        return Err(anyhow!("许可证文件数据不完整"));
    }

    // 解析 payload
    let payload_bytes = &data[9..9 + payload_len];
    let payload: LicensePayload = serde_json::from_slice(payload_bytes)
        .map_err(|e| anyhow!("无法解析许可证内容: {}", e))?;

    // 读取签名
    let signature = data[9 + payload_len..9 + payload_len + 256].to_vec();

    Ok(LicenseFile { payload, signature })
}

/// 验证许可证签名
pub fn verify_license_signature(license_file: &LicenseFile) -> Result<bool> {
    // 加载公钥
    let public_key = RsaPublicKey::from_pkcs1_pem(
        std::str::from_utf8(PUBLIC_KEY_PEM)
            .map_err(|e| anyhow!("公钥格式错误: {}", e))?
    )
    .map_err(|e| anyhow!("无法加载公钥: {}", e))?;

    // 序列化 payload 用于验证
    let payload_bytes = serde_json::to_vec(&license_file.payload)
        .map_err(|e| anyhow!("无法序列化许可证内容: {}", e))?;

    // 使用 PKCS1v15 + SHA256 验证签名
    let padding = Pkcs1v15Sign::new::<Sha256>();
    match public_key.verify(padding, &payload_bytes, &license_file.signature) {
        Ok(()) => Ok(true),
        Err(_) => Ok(false),
    }
}

/// 完整的许可证验证流程
pub fn verify_license(data: &[u8]) -> Result<LicensePayload> {
    // 解析文件
    let license_file = parse_license_file(data)?;

    // 验证签名
    let is_valid = verify_license_signature(&license_file)?;
    if !is_valid {
        return Err(anyhow!("许可证签名验证失败"));
    }

    Ok(license_file.payload)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::license::types::{LicenseFeatures, LicenseTier};

    #[test]
    fn test_parse_license_file_invalid() {
        // 太小
        assert!(parse_license_file(&[0u8; 5]).is_err());
        // 错误的魔数
        assert!(parse_license_file(&[0u8; 20]).is_err());
    }

    #[test]
    fn test_parse_license_file_valid_structure() {
        let payload = LicensePayload {
            key: "TEST-KEY-1234".to_string(),
            tier: LicenseTier::Basic,
            name: "Test User".to_string(),
            email: "test@example.com".to_string(),
            issued_at: 1719705600,
            expires_at: 1751241600,
            max_devices: 1,
            features: LicenseFeatures::default(),
        };

        let payload_bytes = serde_json::to_vec(&payload).unwrap();

        let mut data = Vec::new();
        data.extend_from_slice(LICENSE_MAGIC);
        data.push(LICENSE_VERSION);
        data.extend_from_slice(&(payload_bytes.len() as u32).to_le_bytes());
        data.extend_from_slice(&payload_bytes);
        data.extend_from_slice(&[0u8; 256]); // 签名占位

        let result = parse_license_file(&data);
        assert!(result.is_ok());

        let file = result.unwrap();
        assert_eq!(file.payload.key, "TEST-KEY-1234");
        assert_eq!(file.payload.tier, LicenseTier::Basic);
    }
}
