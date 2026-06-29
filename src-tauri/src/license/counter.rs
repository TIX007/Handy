use anyhow::Result;
use hmac::{Hmac, Mac};
use rusqlite::{params, Connection};
use sha2::Sha256;

use super::types::UsageCounter;

type HmacSha256 = Hmac<Sha256>;

/// 使用计数器管理器
/// 使用 SQLite 存储月度使用统计，HMAC 保护防篡改
pub struct UsageCounterManager {
    conn: Connection,
}

impl UsageCounterManager {
    /// 创建新的计数器管理器
    pub fn new(conn: Connection) -> Result<Self> {
        let manager = Self { conn };
        manager.init_tables()?;
        Ok(manager)
    }

    /// 初始化数据库表
    fn init_tables(&self) -> Result<()> {
        self.conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS license_data (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL
            );",
        )?;
        Ok(())
    }

    /// 获取或创建 HMAC 密钥
    fn get_or_create_hmac_key(&self) -> Result<Vec<u8>> {
        // 尝试获取现有密钥
        let result = self.conn.query_row(
            "SELECT value FROM license_data WHERE key = 'hmac_key'",
            [],
            |row| row.get::<_, String>(0),
        );

        match result {
            Ok(key_hex) => Ok(hex::decode(key_hex)?),
            Err(_) => {
                // 生成新密钥
                use rand::Rng;
                let mut rng = rand::thread_rng();
                let key: Vec<u8> = (0..32).map(|_| rng.gen()).collect();
                let key_hex = hex::encode(&key);

                self.conn.execute(
                    "INSERT OR REPLACE INTO license_data (key, value) VALUES ('hmac_key', ?1)",
                    params![key_hex],
                )?;

                Ok(key)
            }
        }
    }

    /// 计算 HMAC
    fn compute_hmac(&self, data: &[u8]) -> Result<Vec<u8>> {
        let key = self.get_or_create_hmac_key()?;
        let mut mac = HmacSha256::new_from_slice(&key)?;
        mac.update(data);
        Ok(mac.finalize().into_bytes().to_vec())
    }

    /// 保存计数器（带 HMAC 保护）
    pub fn save_counter(&self, counter: &UsageCounter) -> Result<()> {
        let counter_json = serde_json::to_string(counter)?;
        let hmac = self.compute_hmac(counter_json.as_bytes())?;
        let hmac_hex = hex::encode(&hmac);

        self.conn.execute(
            "INSERT OR REPLACE INTO license_data (key, value) VALUES ('counter_hmac', ?1)",
            params![hmac_hex],
        )?;
        self.conn.execute(
            "INSERT OR REPLACE INTO license_data (key, value) VALUES ('counter_data', ?1)",
            params![counter_json],
        )?;

        Ok(())
    }

    /// 加载并验证计数器
    pub fn load_counter(&self) -> Result<UsageCounter> {
        let stored_mac: String = self.conn.query_row(
            "SELECT value FROM license_data WHERE key = 'counter_hmac'",
            [],
            |row| row.get(0),
        );

        let counter_json: String = self.conn.query_row(
            "SELECT value FROM license_data WHERE key = 'counter_data'",
            [],
            |row| row.get(0),
        );

        // 如果没有存储的计数器，创建新的
        let (stored_mac, counter_json) = match (stored_mac, counter_json) {
            (Ok(mac), Ok(json)) => (mac, json),
            _ => {
                let counter = UsageCounter::new(UsageCounter::current_month());
                self.save_counter(&counter)?;
                return Ok(counter);
            }
        };

        // 验证 HMAC
        let computed = self.compute_hmac(counter_json.as_bytes())?;
        let computed_hex = hex::encode(&computed);

        if computed_hex != stored_mac {
            // 被篡改，重置为当前月的零值
            log::warn!("使用计数器 HMAC 不匹配，可能被篡改，已重置");
            let counter = UsageCounter::new(UsageCounter::current_month());
            self.save_counter(&counter)?;
            return Ok(counter);
        }

        let counter: UsageCounter = serde_json::from_str(&counter_json)?;

        // 检查是否需要重置（新月份）
        let current_month = UsageCounter::current_month();
        if counter.month != current_month {
            let new_counter = UsageCounter::new(current_month);
            self.save_counter(&new_counter)?;
            return Ok(new_counter);
        }

        Ok(counter)
    }

    /// 增加使用计数
    pub fn increment_usage(
        &self,
        usage_type: &str,
        amount: f64,
    ) -> Result<UsageCounter> {
        let mut counter = self.load_counter()?;

        match usage_type {
            "transcriptions" => {
                counter.transcriptions += amount as u32;
            }
            "recording_seconds" => {
                counter.recording_seconds += amount;
            }
            "model_downloads" => {
                counter.model_downloads += amount as u32;
            }
            _ => {
                return Err(anyhow::anyhow!("未知的使用类型: {}", usage_type));
            }
        }

        self.save_counter(&counter)?;
        Ok(counter)
    }

    /// 重置计数器
    pub fn reset_counter(&self) -> Result<UsageCounter> {
        let counter = UsageCounter::new(UsageCounter::current_month());
        self.save_counter(&counter)?;
        Ok(counter)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;

    fn create_test_manager() -> UsageCounterManager {
        let conn = Connection::open_in_memory().unwrap();
        UsageCounterManager::new(conn).unwrap()
    }

    #[test]
    fn test_save_and_load_counter() {
        let manager = create_test_manager();

        let counter = UsageCounter {
            month: "2026-06".to_string(),
            transcriptions: 10,
            recording_seconds: 300.0,
            model_downloads: 2,
        };

        manager.save_counter(&counter).unwrap();
        let loaded = manager.load_counter().unwrap();

        assert_eq!(loaded.month, counter.month);
        assert_eq!(loaded.transcriptions, counter.transcriptions);
        assert_eq!(loaded.recording_seconds, counter.recording_seconds);
        assert_eq!(loaded.model_downloads, counter.model_downloads);
    }

    #[test]
    fn test_increment_usage() {
        let manager = create_test_manager();

        // 先保存一个初始计数器
        let counter = UsageCounter::new(UsageCounter::current_month());
        manager.save_counter(&counter).unwrap();

        // 增加转录次数
        let updated = manager.increment_usage("transcriptions", 5.0).unwrap();
        assert_eq!(updated.transcriptions, 5);

        // 再次增加
        let updated = manager.increment_usage("transcriptions", 3.0).unwrap();
        assert_eq!(updated.transcriptions, 8);

        // 增加录音时长
        let updated = manager.increment_usage("recording_seconds", 120.0).unwrap();
        assert_eq!(updated.recording_seconds, 120.0);
    }

    #[test]
    fn test_tamper_detection() {
        let manager = create_test_manager();

        let counter = UsageCounter {
            month: "2026-06".to_string(),
            transcriptions: 100,
            recording_seconds: 3600.0,
            model_downloads: 5,
        };

        manager.save_counter(&counter).unwrap();

        // 直接修改数据库中的计数器数据（模拟篡改）
        manager.conn.execute(
            "UPDATE license_data SET value = '{\"month\":\"2026-06\",\"transcriptions\":999,\"recording_seconds\":0,\"model_downloads\":0}' WHERE key = 'counter_data'",
            [],
        ).unwrap();

        // 加载应该检测到篡改并重置
        let loaded = manager.load_counter().unwrap();
        assert_eq!(loaded.transcriptions, 0, "应该重置为 0");
    }

    #[test]
    fn test_monthly_reset() {
        let manager = create_test_manager();

        // 保存一个旧月份的计数器
        let counter = UsageCounter {
            month: "2026-01".to_string(),
            transcriptions: 100,
            recording_seconds: 3600.0,
            model_downloads: 5,
        };

        manager.save_counter(&counter).unwrap();

        // 加载应该返回当前月的新计数器
        let loaded = manager.load_counter().unwrap();
        assert_eq!(loaded.month, UsageCounter::current_month());
        assert_eq!(loaded.transcriptions, 0);
    }
}
