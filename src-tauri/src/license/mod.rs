pub mod commands;
pub mod counter;
pub mod fingerprint;
pub mod guard;
pub mod types;
pub mod verify;

use std::sync::Arc;

use anyhow::{anyhow, Result};
use rusqlite::Connection;
use std::sync::RwLock;
use tauri::AppHandle;

use self::counter::UsageCounterManager;
use self::fingerprint::{generate_fingerprint, HardwareFingerprint};
use self::guard::{LicenseGuard, RemainingUsage};
use self::types::{LicenseFeatures, LicensePayload, LicenseStatus, LicenseTier, UsageCounter};
use self::verify::verify_license;

/// 许可证管理器
pub struct LicenseManager {
    app_handle: AppHandle,
    license: RwLock<Option<LicensePayload>>,
    fingerprint: HardwareFingerprint,
    counter_manager: UsageCounterManager,
}

impl LicenseManager {
    /// 创建新的许可证管理器
    pub fn new(app_handle: &AppHandle) -> Result<Self> {
        // 获取应用数据目录
        let app_dir = app_handle
            .path()
            .app_data_dir()
            .map_err(|e| anyhow!("无法获取应用数据目录: {}", e))?;

        // 确保目录存在
        std::fs::create_dir_all(&app_dir)?;

        // 打开或创建许可证数据库
        let db_path = app_dir.join("license.db");
        let conn = Connection::open(&db_path)?;

        let counter_manager = UsageCounterManager::new(conn)?;

        // 采集硬件指纹
        let fingerprint = generate_fingerprint()?;

        // 尝试加载已保存的许可证
        let license = Self::load_license_from_store(app_handle)?;

        let manager = Self {
            app_handle: app_handle.clone(),
            license: RwLock::new(license),
            fingerprint,
            counter_manager,
        };

        Ok(manager)
    }

    /// 从存储加载许可证
    fn load_license_from_store(app_handle: &AppHandle) -> Result<Option<LicensePayload>> {
        let store = tauri_plugin_store::StoreBuilder::new(
            app_handle,
            "license_store.json",
        )
        .build();

        match store {
            Ok(store) => {
                if let Some(Some(data)) = store.get("license_data") {
                    if let Ok(license) = serde_json::from_value::<LicensePayload>(data) {
                        return Ok(Some(license));
                    }
                }
                Ok(None)
            }
            Err(_) => Ok(None),
        }
    }

    /// 保存许可证到存储
    fn save_license_to_store(&self, license: &Option<LicensePayload>) -> Result<()> {
        let store = tauri_plugin_store::StoreBuilder::new(
            &self.app_handle,
            "license_store.json",
        )
        .build()?;

        match license {
            Some(license) => {
                let data = serde_json::to_value(license)?;
                store.set("license_data", data);
            }
            None => {
                store.delete("license_data");
            }
        }

        store.save()?;
        Ok(())
    }

    /// 激活许可证
    pub fn activate(&self, license_data: &[u8]) -> Result<LicensePayload> {
        // 验证许可证签名
        let payload = verify_license(license_data)?;

        // 检查设备绑定
        if let Some(stored_fingerprint) = self.get_stored_fingerprint(&payload.key) {
            if stored_fingerprint != self.fingerprint.fingerprint {
                return Err(anyhow!("许可证已绑定到其他设备"));
            }
        } else {
            // 首次激活，存储指纹
            self.store_fingerprint(&payload.key, &self.fingerprint.fingerprint)?;
        }

        // 保存许可证
        *self.license.write().map_err(|e| anyhow!("锁错误: {}", e))? = Some(payload.clone());
        self.save_license_to_store(&Some(payload.clone()))?;

        Ok(payload)
    }

    /// 停用许可证
    pub fn deactivate(&self) -> Result<()> {
        let mut license = self.license.write().map_err(|e| anyhow!("锁错误: {}", e))?;

        if let Some(ref license_data) = *license {
            // 删除设备指纹
            self.remove_stored_fingerprint(&license_data.key)?;
        }

        *license = None;
        self.save_license_to_store(&None)?;

        Ok(())
    }

    /// 获取许可证状态
    pub fn get_status(&self) -> LicenseStatus {
        let license = self.license.read().ok();
        let current_counter = self.counter_manager.load_counter().unwrap_or_else(|_| {
            UsageCounter::new(UsageCounter::current_month())
        });

        match license.as_ref().and_then(|l| l.as_ref()) {
            Some(license) => {
                let now = chrono::Utc::now().timestamp();
                let is_expired = license.expires_at > 0 && now > license.expires_at;
                let grace_period_end = if is_expired {
                    Some(license.expires_at + types::GRACE_PERIOD_SECONDS)
                } else {
                    None
                };
                let is_grace_period = is_expired
                    && grace_period_end
                        .map(|end| now < end)
                        .unwrap_or(false);

                LicenseStatus {
                    is_activated: true,
                    tier: license.tier.clone(),
                    name: license.name.clone(),
                    email: license.email.clone(),
                    key: license.key.clone(),
                    is_expired,
                    is_grace_period,
                    grace_period_end,
                    expires_at: if license.expires_at > 0 {
                        Some(license.expires_at)
                    } else {
                        None
                    },
                    usage: current_counter,
                    features: license.features.clone(),
                }
            }
            None => LicenseStatus {
                is_activated: false,
                tier: LicenseTier::Basic,
                usage: current_counter,
                features: LicenseFeatures::default(),
                ..Default::default()
            },
        }
    }

    /// 检查功能是否可用
    pub fn check_feature(&self, feature: &str) -> Result<()> {
        let status = self.get_status();
        LicenseGuard::check_feature(&status, feature)
    }

    /// 检查使用限制
    pub fn check_usage_limit(&self, usage_type: &str) -> Result<()> {
        let status = self.get_status();
        LicenseGuard::check_usage_limit(&status, usage_type)
    }

    /// 增加使用计数
    pub fn increment_usage(&self, usage_type: &str, amount: f64) -> Result<()> {
        self.counter_manager.increment_usage(usage_type, amount)?;
        Ok(())
    }

    /// 获取剩余使用量
    pub fn get_remaining_usage(&self) -> RemainingUsage {
        let status = self.get_status();
        LicenseGuard::get_remaining_usage(&status)
    }

    /// 获取使用统计
    pub fn get_usage_stats(&self) -> UsageCounter {
        self.counter_manager
            .load_counter()
            .unwrap_or_else(|_| UsageCounter::new(UsageCounter::current_month()))
    }

    // ===== 设备指纹管理 =====

    fn get_stored_fingerprint(&self, license_key: &str) -> Option<String> {
        let store = tauri_plugin_store::StoreBuilder::new(
            &self.app_handle,
            "license_store.json",
        )
        .build()
        .ok()?;

        let key = format!("fingerprint_{}", license_key);
        store.get(&key).and_then(|v| v.and_then(|v| serde_json::from_value(v).ok()))
    }

    fn store_fingerprint(&self, license_key: &str, fingerprint: &str) -> Result<()> {
        let store = tauri_plugin_store::StoreBuilder::new(
            &self.app_handle,
            "license_store.json",
        )
        .build()?;

        let key = format!("fingerprint_{}", license_key);
        store.set(&key, serde_json::Value::String(fingerprint.to_string()));
        store.save()?;

        Ok(())
    }

    fn remove_stored_fingerprint(&self, license_key: &str) -> Result<()> {
        let store = tauri_plugin_store::StoreBuilder::new(
            &self.app_handle,
            "license_store.json",
        )
        .build()?;

        let key = format!("fingerprint_{}", license_key);
        store.delete(&key);
        store.save()?;

        Ok(())
    }
}

/// 初始化许可证管理器并注册到 Tauri
pub fn init_license_manager(app_handle: &AppHandle) -> Result<Arc<LicenseManager>> {
    let manager = LicenseManager::new(app_handle)?;
    let manager = Arc::new(manager);
    app_handle.manage(manager.clone());
    Ok(manager)
}
