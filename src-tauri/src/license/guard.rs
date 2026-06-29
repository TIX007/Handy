use anyhow::{anyhow, Result};

use super::types::{LicenseStatus, LicenseTier};

/// 功能门控检查器
pub struct LicenseGuard;

impl LicenseGuard {
    /// 检查功能是否可用
    pub fn check_feature(status: &LicenseStatus, feature: &str) -> Result<()> {
        // 宽限期内仍可使用原等级功能
        if status.is_expired && !status.is_grace_period {
            return Err(anyhow!("许可证已过期，请续期或升级许可证"));
        }

        let has_feature = match feature {
            "transcription" => true, // 所有等级都可以转录
            "post_processing" => status.features.post_processing,
            "custom_models" => status.features.custom_models,
            "batch_transcription" => status.features.batch_transcription,
            "model_downloads" => true, // 所有等级都可以下载，但有数量限制
            _ => return Err(anyhow!("未知功能: {}", feature)),
        };

        if !has_feature {
            return Err(anyhow!(
                "此功能需要 {} 许可证",
                Self::required_tier_for_feature(feature)
            ));
        }

        Ok(())
    }

    /// 检查使用限制
    pub fn check_usage_limit(status: &LicenseStatus, usage_type: &str) -> Result<()> {
        // 宽限期内仍按原等级限制
        if status.is_expired && !status.is_grace_period {
            return Err(anyhow!("许可证已过期，请续期或升级许可证"));
        }

        match usage_type {
            "transcriptions" => {
                if let Some(limit) = status.features.transcription_limit {
                    if status.usage.transcriptions >= limit {
                        return Err(anyhow!(
                            "本月转录次数已达上限 ({}次)，请升级许可证",
                            limit
                        ));
                    }
                }
            }
            "recording_seconds" => {
                if let Some(hours) = status.features.recording_hours {
                    let limit_seconds = hours * 3600.0;
                    if status.usage.recording_seconds >= limit_seconds {
                        return Err(anyhow!(
                            "本月录音时长已达上限 ({:.1}小时)，请升级许可证",
                            hours
                        ));
                    }
                }
            }
            "model_downloads" => {
                if let Some(limit) = status.features.model_downloads {
                    if status.usage.model_downloads >= limit {
                        return Err(anyhow!(
                            "可下载模型数量已达上限 ({}个)，请升级许可证",
                            limit
                        ));
                    }
                }
            }
            _ => return Err(anyhow!("未知使用类型: {}", usage_type)),
        }

        Ok(())
    }

    /// 获取功能所需的最低许可证等级
    fn required_tier_for_feature(feature: &str) -> &str {
        match feature {
            "post_processing" | "custom_models" => "Professional",
            "batch_transcription" => "Enterprise",
            _ => "Basic",
        }
    }

    /// 检查许可证是否过期
    pub fn is_expired(status: &LicenseStatus) -> bool {
        status.is_expired
    }

    /// 检查是否在宽限期内
    pub fn is_in_grace_period(status: &LicenseStatus) -> bool {
        status.is_grace_period
    }

    /// 获取剩余使用量
    pub fn get_remaining_usage(status: &LicenseStatus) -> RemainingUsage {
        RemainingUsage {
            transcriptions: status
                .features
                .transcription_limit
                .map(|limit| limit.saturating_sub(status.usage.transcriptions)),
            recording_hours: status.features.recording_hours.map(|limit| {
                let used_hours = status.usage.recording_seconds / 3600.0;
                (limit - used_hours).max(0.0)
            }),
            model_downloads: status
                .features
                .model_downloads
                .map(|limit| limit.saturating_sub(status.usage.model_downloads)),
        }
    }
}

/// 剩余使用量
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RemainingUsage {
    /// 剩余转录次数，None 表示无限制
    pub transcriptions: Option<u32>,
    /// 剩余录音时长（小时），None 表示无限制
    pub recording_hours: Option<f64>,
    /// 剩余可下载模型数，None 表示无限制
    pub model_downloads: Option<u32>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::license::types::{LicenseFeatures, UsageCounter};

    fn create_basic_status() -> LicenseStatus {
        LicenseStatus {
            is_activated: true,
            tier: LicenseTier::Basic,
            name: "Test".to_string(),
            email: "test@test.com".to_string(),
            key: "TEST-KEY".to_string(),
            is_expired: false,
            is_grace_period: false,
            grace_period_end: None,
            expires_at: Some(1751241600),
            usage: UsageCounter::new("2026-06".to_string()),
            features: LicenseFeatures::default(),
        }
    }

    fn create_pro_status() -> LicenseStatus {
        LicenseStatus {
            is_activated: true,
            tier: LicenseTier::Professional,
            name: "Test".to_string(),
            email: "test@test.com".to_string(),
            key: "TEST-KEY".to_string(),
            is_expired: false,
            is_grace_period: false,
            grace_period_end: None,
            expires_at: Some(1751241600),
            usage: UsageCounter::new("2026-06".to_string()),
            features: LicenseFeatures::for_tier(&LicenseTier::Professional),
        }
    }

    #[test]
    fn test_basic_feature_access() {
        let status = create_basic_status();

        // Basic 可以转录
        assert!(LicenseGuard::check_feature(&status, "transcription").is_ok());

        // Basic 不能后处理
        assert!(LicenseGuard::check_feature(&status, "post_processing").is_err());

        // Basic 不能使用自定义模型
        assert!(LicenseGuard::check_feature(&status, "custom_models").is_err());
    }

    #[test]
    fn test_pro_feature_access() {
        let status = create_pro_status();

        // Professional 可以后处理
        assert!(LicenseGuard::check_feature(&status, "post_processing").is_ok());

        // Professional 可以使用自定义模型
        assert!(LicenseGuard::check_feature(&status, "custom_models").is_ok());

        // Professional 不能批量转录
        assert!(LicenseGuard::check_feature(&status, "batch_transcription").is_err());
    }

    #[test]
    fn test_basic_usage_limits() {
        let mut status = create_basic_status();

        // 未达到限制
        assert!(LicenseGuard::check_usage_limit(&status, "transcriptions").is_ok());

        // 达到限制
        status.usage.transcriptions = 100;
        assert!(LicenseGuard::check_usage_limit(&status, "transcriptions").is_err());
    }

    #[test]
    fn test_pro_unlimited_usage() {
        let mut status = create_pro_status();

        // Professional 无限制
        status.usage.transcriptions = 10000;
        assert!(LicenseGuard::check_usage_limit(&status, "transcriptions").is_ok());
    }

    #[test]
    fn test_expired_license() {
        let mut status = create_basic_status();
        status.is_expired = true;
        status.is_grace_period = false;

        // 过期许可证不能使用任何功能
        assert!(LicenseGuard::check_feature(&status, "transcription").is_err());
    }

    #[test]
    fn test_grace_period() {
        let mut status = create_basic_status();
        status.is_expired = true;
        status.is_grace_period = true;

        // 宽限期内可以使用原等级功能
        assert!(LicenseGuard::check_feature(&status, "transcription").is_ok());
    }

    #[test]
    fn test_remaining_usage() {
        let mut status = create_basic_status();
        status.usage.transcriptions = 50;
        status.usage.recording_seconds = 1800.0; // 0.5 小时
        status.usage.model_downloads = 1;

        let remaining = LicenseGuard::get_remaining_usage(&status);
        assert_eq!(remaining.transcriptions, Some(50));
        assert!(remaining.recording_hours.is_some());
        assert!(remaining.recording_hours.unwrap() > 9.0);
        assert_eq!(remaining.model_downloads, Some(1));
    }
}
