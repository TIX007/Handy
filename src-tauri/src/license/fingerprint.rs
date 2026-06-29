use anyhow::Result;
use sha2::{Digest, Sha256};

/// 硬件指纹采集模块
/// 采集多个硬件信号，SHA-256 哈希后取前 32 字节
/// 采用 3/5 匹配策略：5 个信号中有 3 个匹配即认为同一台机器

/// 单个硬件信号
#[derive(Debug, Clone)]
pub struct HardwareSignal {
    pub name: String,
    pub value: String,
}

/// 硬件指纹
#[derive(Debug, Clone)]
pub struct HardwareFingerprint {
    pub signals: Vec<HardwareSignal>,
    pub fingerprint: String,
}

/// 采集硬件信号
pub fn collect_hardware_signals() -> Vec<HardwareSignal> {
    let mut signals = Vec::new();

    // CPU ID
    if let Some(cpu_id) = get_cpu_id() {
        signals.push(HardwareSignal {
            name: "cpu_id".to_string(),
            value: cpu_id,
        });
    }

    // 主板序列号
    if let Some(board_serial) = get_board_serial() {
        signals.push(HardwareSignal {
            name: "board_serial".to_string(),
            value: board_serial,
        });
    }

    // MAC 地址（第一个非回环网卡）
    if let Some(mac_address) = get_mac_address() {
        signals.push(HardwareSignal {
            name: "mac_address".to_string(),
            value: mac_address,
        });
    }

    // 系统 UUID
    if let Some(system_uuid) = get_system_uuid() {
        signals.push(HardwareSignal {
            name: "system_uuid".to_string(),
            value: system_uuid,
        });
    }

    // 磁盘序列号（系统盘）
    if let Some(disk_serial) = get_disk_serial() {
        signals.push(HardwareSignal {
            name: "disk_serial".to_string(),
            value: disk_serial,
        });
    }

    signals
}

/// 生成硬件指纹
pub fn generate_fingerprint() -> Result<HardwareFingerprint> {
    let signals = collect_hardware_signals();

    if signals.is_empty() {
        return Err(anyhow::anyhow!("无法采集任何硬件信号"));
    }

    // 将所有信号值组合后哈希
    let combined = signals
        .iter()
        .map(|s| format!("{}={}", s.name, s.value))
        .collect::<Vec<_>>()
        .join("|");

    let mut hasher = Sha256::new();
    hasher.update(combined.as_bytes());
    let hash = hasher.finalize();

    // 取前 32 字节作为指纹
    let fingerprint = hex::encode(&hash[..32]);

    Ok(HardwareFingerprint {
        signals,
        fingerprint,
    })
}

/// 验证两个指纹是否匹配（3/5 策略）
pub fn fingerprints_match(stored: &[HardwareSignal], current: &[HardwareSignal]) -> bool {
    if stored.is_empty() || current.is_empty() {
        return false;
    }

    let match_count = stored
        .iter()
        .filter(|s| current.iter().any(|c| c.name == s.name && c.value == s.value))
        .count();

    // 至少匹配 3 个信号，或如果信号总数少于 3，则全部匹配
    let required = std::cmp::min(3, std::cmp::min(stored.len(), current.len()));
    match_count >= required
}

// ===== 平台特定的硬件信号采集 =====

#[cfg(target_os = "windows")]
fn get_cpu_id() -> Option<String> {
    use std::process::Command;
    let output = Command::new("wmic")
        .args(["cpu", "get", "ProcessorId"])
        .output()
        .ok()?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    stdout
        .lines()
        .skip(1)
        .next()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}

#[cfg(target_os = "macos")]
fn get_cpu_id() -> Option<String> {
    use std::process::Command;
    let output = Command::new("sysctl")
        .args(["-n", "machdep.cpu.brand_string"])
        .output()
        .ok()?;
    String::from_utf8_lossy(&output.stdout).trim().to_string().into()
}

#[cfg(target_os = "linux")]
fn get_cpu_id() -> Option<String> {
    use std::process::Command;
    let output = Command::new("cat")
        .args(["/proc/cpuinfo"])
        .output()
        .ok()?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    stdout
        .lines()
        .find(|line| line.starts_with("model name"))
        .and_then(|line| line.split(':').nth(1))
        .map(|s| s.trim().to_string())
}

#[cfg(target_os = "windows")]
fn get_board_serial() -> Option<String> {
    use std::process::Command;
    let output = Command::new("wmic")
        .args(["baseboard", "get", "SerialNumber"])
        .output()
        .ok()?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    stdout
        .lines()
        .skip(1)
        .next()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}

#[cfg(target_os = "macos")]
fn get_board_serial() -> Option<String> {
    use std::process::Command;
    let output = Command::new("ioreg")
        .args(["-c", "IOPlatformExpertDevice", "-d", "2"])
        .output()
        .ok()?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    stdout
        .lines()
        .find(|line| line.contains("IOPlatformSerialNumber"))
        .and_then(|line| line.split('"').nth(3))
        .map(|s| s.to_string())
}

#[cfg(target_os = "linux")]
fn get_board_serial() -> Option<String> {
    use std::process::Command;
    let output = Command::new("cat")
        .args(["/sys/class/dmi/id/board_serial"])
        .output()
        .ok()?;
    String::from_utf8_lossy(&output.stdout).trim().to_string().into()
}

fn get_mac_address() -> Option<String> {
    // 使用 getmac 命令或读取网络接口
    #[cfg(target_os = "windows")]
    {
        use std::process::Command;
        let output = Command::new("getmac").output().ok()?;
        let stdout = String::from_utf8_lossy(&output.stdout);
        stdout
            .lines()
            .skip(3)
            .find(|line| !line.contains("N/A"))
            .and_then(|line| line.split_whitespace().next())
            .map(|s| s.replace("-", ":").to_string())
    }

    #[cfg(not(target_os = "windows"))]
    {
        use std::process::Command;
        let output = Command::new("cat")
            .args(["/sys/class/net/eth0/address"])
            .output()
            .ok()?;
        String::from_utf8_lossy(&output.stdout).trim().to_string().into()
    }
}

#[cfg(target_os = "windows")]
fn get_system_uuid() -> Option<String> {
    use std::process::Command;
    let output = Command::new("wmic")
        .args(["csproduct", "get", "UUID"])
        .output()
        .ok()?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    stdout
        .lines()
        .skip(1)
        .next()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}

#[cfg(target_os = "macos")]
fn get_system_uuid() -> Option<String> {
    use std::process::Command;
    let output = Command::new("ioreg")
        .args(["-c", "IOPlatformExpertDevice", "-d", "2"])
        .output()
        .ok()?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    stdout
        .lines()
        .find(|line| line.contains("IOPlatformUUID"))
        .and_then(|line| line.split('"').nth(3))
        .map(|s| s.to_string())
}

#[cfg(target_os = "linux")]
fn get_system_uuid() -> Option<String> {
    use std::process::Command;
    let output = Command::new("cat")
        .args(["/sys/class/dmi/id/product_uuid"])
        .output()
        .ok()?;
    String::from_utf8_lossy(&output.stdout).trim().to_string().into()
}

#[cfg(target_os = "windows")]
fn get_disk_serial() -> Option<String> {
    use std::process::Command;
    let output = Command::new("wmic")
        .args(["diskdrive", "where", "BootDevice=true", "get", "SerialNumber"])
        .output()
        .ok()?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    stdout
        .lines()
        .skip(1)
        .next()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}

#[cfg(target_os = "macos")]
fn get_disk_serial() -> Option<String> {
    // macOS 不直接暴露磁盘序列号
    None
}

#[cfg(target_os = "linux")]
fn get_disk_serial() -> Option<String> {
    use std::process::Command;
    let output = Command::new("lsblk")
        .args(["-ndo", "SERIAL", "/dev/sda"])
        .output()
        .ok()?;
    String::from_utf8_lossy(&output.stdout).trim().to_string().into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collect_hardware_signals() {
        let signals = collect_hardware_signals();
        // 至少应该能采集到一个信号
        println!("采集到 {} 个硬件信号", signals.len());
        for signal in &signals {
            println!("  {}: {}", signal.name, signal.value);
        }
    }

    #[test]
    fn test_generate_fingerprint() {
        let result = generate_fingerprint();
        assert!(result.is_ok(), "应该能生成硬件指纹");

        let fp = result.unwrap();
        assert!(!fp.fingerprint.is_empty(), "指纹不应为空");
        assert_eq!(fp.fingerprint.len(), 64, "SHA-256 哈希应该是 64 个十六进制字符");
    }

    #[test]
    fn test_fingerprints_match() {
        let signals1 = vec![
            HardwareSignal {
                name: "cpu_id".to_string(),
                value: "ABC123".to_string(),
            },
            HardwareSignal {
                name: "board_serial".to_string(),
                value: "XYZ789".to_string(),
            },
            HardwareSignal {
                name: "mac_address".to_string(),
                value: "00:11:22:33:44:55".to_string(),
            },
        ];

        let signals2 = signals1.clone();

        assert!(fingerprints_match(&signals1, &signals2));
    }

    #[test]
    fn test_fingerprints_partial_match() {
        let signals1 = vec![
            HardwareSignal {
                name: "cpu_id".to_string(),
                value: "ABC123".to_string(),
            },
            HardwareSignal {
                name: "board_serial".to_string(),
                value: "XYZ789".to_string(),
            },
            HardwareSignal {
                name: "mac_address".to_string(),
                value: "00:11:22:33:44:55".to_string(),
            },
            HardwareSignal {
                name: "system_uuid".to_string(),
                value: "UUID-123".to_string(),
            },
            HardwareSignal {
                name: "disk_serial".to_string(),
                value: "DISK-456".to_string(),
            },
        ];

        // 只匹配 3 个信号（CPU、主板、MAC 变了）
        let signals2 = vec![
            HardwareSignal {
                name: "cpu_id".to_string(),
                value: "DIFFERENT".to_string(),
            },
            HardwareSignal {
                name: "board_serial".to_string(),
                value: "DIFFERENT".to_string(),
            },
            HardwareSignal {
                name: "mac_address".to_string(),
                value: "DIFFERENT".to_string(),
            },
            HardwareSignal {
                name: "system_uuid".to_string(),
                value: "UUID-123".to_string(),
            },
            HardwareSignal {
                name: "disk_serial".to_string(),
                value: "DISK-456".to_string(),
            },
        ];

        // 只有 2 个匹配，不够 3 个
        assert!(!fingerprints_match(&signals1, &signals2));
    }
}
