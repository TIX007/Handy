use std::sync::Arc;

use tauri::State;

use super::guard::RemainingUsage;
use super::types::{LicensePayload, LicenseStatus, UsageCounter};
use super::LicenseManager;

/// 激活许可证
#[tauri::command]
#[specta::specta]
pub fn activate_license(
    license_manager: State<'_, Arc<LicenseManager>>,
    license_data: Vec<u8>,
) -> Result<LicensePayload, String> {
    license_manager
        .activate(&license_data)
        .map_err(|e| e.to_string())
}

/// 停用许可证
#[tauri::command]
#[specta::specta]
pub fn deactivate_license(
    license_manager: State<'_, Arc<LicenseManager>>,
) -> Result<(), String> {
    license_manager.deactivate().map_err(|e| e.to_string())
}

/// 获取许可证状态
#[tauri::command]
#[specta::specta]
pub fn get_license_status(
    license_manager: State<'_, Arc<LicenseManager>>,
) -> Result<LicenseStatus, String> {
    Ok(license_manager.get_status())
}

/// 获取使用统计
#[tauri::command]
#[specta::specta]
pub fn get_usage_stats(
    license_manager: State<'_, Arc<LicenseManager>>,
) -> Result<UsageCounter, String> {
    Ok(license_manager.get_usage_stats())
}

/// 获取剩余使用量
#[tauri::command]
#[specta::specta]
pub fn get_remaining_usage(
    license_manager: State<'_, Arc<LicenseManager>>,
) -> Result<RemainingUsage, String> {
    Ok(license_manager.get_remaining_usage())
}

/// 检查功能是否可用（供前端调用）
#[tauri::command]
#[specta::specta]
pub fn check_license_feature(
    license_manager: State<'_, Arc<LicenseManager>>,
    feature: String,
) -> Result<bool, String> {
    match license_manager.check_feature(&feature) {
        Ok(()) => Ok(true),
        Err(_) => Ok(false),
    }
}

/// 检查使用限制（供前端调用）
#[tauri::command]
#[specta::specta]
pub fn check_license_usage_limit(
    license_manager: State<'_, Arc<LicenseManager>>,
    usage_type: String,
) -> Result<bool, String> {
    match license_manager.check_usage_limit(&usage_type) {
        Ok(()) => Ok(true),
        Err(_) => Ok(false),
    }
}
