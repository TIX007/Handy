-- Handy 许可证数据库 Schema

-- 许可证表
CREATE TABLE IF NOT EXISTS licenses (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    license_key TEXT UNIQUE NOT NULL,
    tier TEXT NOT NULL DEFAULT 'basic',
    name TEXT,
    email TEXT,
    issued_at INTEGER NOT NULL,
    expires_at INTEGER,
    max_devices INTEGER DEFAULT 1,
    features JSON,
    is_active BOOLEAN DEFAULT 1,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- 设备绑定表
CREATE TABLE IF NOT EXISTS device_bindings (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    license_key TEXT NOT NULL,
    fingerprint TEXT NOT NULL,
    device_name TEXT,
    bound_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    last_seen DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (license_key) REFERENCES licenses(license_key),
    UNIQUE(license_key, fingerprint)
);

-- 使用统计表
CREATE TABLE IF NOT EXISTS usage_stats (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    license_key TEXT NOT NULL,
    month TEXT NOT NULL,  -- 格式: YYYY-MM
    transcriptions INTEGER DEFAULT 0,
    recording_seconds REAL DEFAULT 0,
    model_downloads INTEGER DEFAULT 0,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (license_key) REFERENCES licenses(license_key),
    UNIQUE(license_key, month)
);

-- 创建索引
CREATE INDEX IF NOT EXISTS idx_licenses_key ON licenses(license_key);
CREATE INDEX IF NOT EXISTS idx_device_bindings_license ON device_bindings(license_key);
CREATE INDEX IF NOT EXISTS idx_device_bindings_fingerprint ON device_bindings(fingerprint);
CREATE INDEX IF NOT EXISTS idx_usage_stats_license ON usage_stats(license_key);
CREATE INDEX IF NOT EXISTS idx_usage_stats_month ON usage_stats(month);
