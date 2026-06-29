use serde::{Deserialize, Serialize};

/// 许可证等级
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum LicenseTier {
    Basic,
    Professional,
    Enterprise,
}

impl LicenseTier {
    pub fn display_name(&self) -> &str {
        match self {
            Self::Basic => "Basic",
            Self::Professional => "Professional",
            Self::Enterprise => "Enterprise",
        }
    }
}

/// 许可证功能配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicenseFeatures {
    /// 每月转录次数限制，None 表示无限制
    pub transcription_limit: Option<u32>,
    /// 录音时长限制（小时/月），None 表示无限制
    pub recording_hours: Option<f64>,
    /// 可下载模型数量限制，None 表示无限制
    pub model_downloads: Option<u32>,
    /// 是否支持后处理
    pub post_processing: bool,
    /// 是否支持自定义模型
    pub custom_models: bool,
    /// 是否支持批量转录
    pub batch_transcription: bool,
}

impl Default for LicenseFeatures {
    fn default() -> Self {
        Self {
            transcription_limit: Some(100),
            recording_hours: Some(10.0),
            model_downloads: Some(2),
            post_processing: false,
            custom_models: false,
            batch_transcription: false,
        }
    }
}

impl LicenseFeatures {
    pub fn for_tier(tier: &LicenseTier) -> Self {
        match tier {
            LicenseTier::Basic => Self::default(),
            LicenseTier::Professional => Self {
                transcription_limit: None,
                recording_hours: None,
                model_downloads: Some(5),
                post_processing: true,
                custom_models: true,
                batch_transcription: false,
            },
            LicenseTier::Enterprise => Self {
                transcription_limit: None,
                recording_hours: None,
                model_downloads: None,
                post_processing: true,
                custom_models: true,
                batch_transcription: true,
            },
        }
    }
}

/// 许可证载荷（签名内容）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicensePayload {
    /// 许可证密钥
    pub key: String,
    /// 许可证等级
    pub tier: LicenseTier,
    /// 用户名称
    pub name: String,
    /// 用户邮箱
    pub email: String,
    /// 签发时间（Unix 时间戳）
    pub issued_at: i64,
    /// 过期时间（Unix 时间戳），0 表示永不过期
    pub expires_at: i64,
    /// 最大设备数
    pub max_devices: u32,
    /// 功能配置
    pub features: LicenseFeatures,
}

/// 使用计数器（按月统计）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageCounter {
    /// 月份标识，格式 "2026-06"
    pub month: String,
    /// 转录次数
    pub transcriptions: u32,
    /// 录音总时长（秒）
    pub recording_seconds: f64,
    /// 模型下载次数
    pub model_downloads: u32,
}

impl UsageCounter {
    pub fn new(month: String) -> Self {
        Self {
            month,
            transcriptions: 0,
            recording_seconds: 0.0,
            model_downloads: 0,
        }
    }

    pub fn current_month() -> String {
        let now = chrono::Utc::now();
        now.format("%Y-%m").to_string()
    }
}

/// 许可证状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicenseStatus {
    /// 是否已激活
    pub is_activated: bool,
    /// 许可证等级
    pub tier: LicenseTier,
    /// 用户名称
    pub name: String,
    /// 用户邮箱
    pub email: String,
    /// 许可证密钥
    pub key: String,
    /// 是否已过期
    pub is_expired: bool,
    /// 是否在宽限期内
    pub is_grace_period: bool,
    /// 宽限期结束时间
    pub grace_period_end: Option<i64>,
    /// 过期时间
    pub expires_at: Option<i64>,
    /// 当前月份使用统计
    pub usage: UsageCounter,
    /// 功能配置
    pub features: LicenseFeatures,
}

impl Default for LicenseStatus {
    fn default() -> Self {
        Self {
            is_activated: false,
            tier: LicenseTier::Basic,
            name: String::new(),
            email: String::new(),
            key: String::new(),
            is_expired: false,
            is_grace_period: false,
            grace_period_end: None,
            expires_at: None,
            usage: UsageCounter::new(UsageCounter::current_month()),
            features: LicenseFeatures::default(),
        }
    }
}

/// 许可证文件格式
#[derive(Debug, Clone)]
pub struct LicenseFile {
    pub payload: LicensePayload,
    pub signature: Vec<u8>,
}

/// 许可证文件头
pub const LICENSE_MAGIC: &[u8; 4] = b"HLIC";
pub const LICENSE_VERSION: u8 = 0x01;

/// 宽限期时长（72 小时）
pub const GRACE_PERIOD_SECONDS: i64 = 72 * 60 * 60;
