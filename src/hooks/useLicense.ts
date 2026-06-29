import { useEffect } from "react";
import {
  useLicenseStore,
  type LicenseStatus,
  type UsageCounter,
  type RemainingUsage,
  type LicenseTier,
} from "@/stores/licenseStore";

export interface UseLicenseReturn {
  // 状态
  status: LicenseStatus | null;
  usage: UsageCounter | null;
  remaining: RemainingUsage | null;
  isLoading: boolean;
  error: string | null;

  // 许可证信息
  isActivated: boolean;
  tier: LicenseTier;
  tierName: string;
  isExpired: boolean;
  isGracePeriod: boolean;

  // 操作
  activate: (licenseData: number[]) => Promise<boolean>;
  deactivate: () => Promise<void>;
  refreshStatus: () => Promise<void>;
  refreshUsage: () => Promise<void>;
  clearError: () => void;

  // 功能检查
  hasFeature: (feature: string) => boolean;
  hasUsageRemaining: (usageType: string) => boolean;
  getRemainingTranscriptions: () => number | null;
  getRemainingRecordingHours: () => number | null;
  getRemainingModelDownloads: () => number | null;
}

const TIER_NAMES: Record<LicenseTier, string> = {
  basic: "Basic",
  professional: "Professional",
  enterprise: "Enterprise",
};

export const useLicense = (): UseLicenseReturn => {
  const store = useLicenseStore();

  // 自动初始化
  useEffect(() => {
    if (!store.status && !store.isLoading) {
      store.initialize();
    }
  }, [store.initialize, store.status, store.isLoading]);

  // 计算派生状态
  const isActivated = store.status?.is_activated ?? false;
  const tier = store.status?.tier ?? "basic";
  const tierName = TIER_NAMES[tier];
  const isExpired = store.status?.is_expired ?? false;
  const isGracePeriod = store.status?.is_grace_period ?? false;

  // 检查功能是否可用
  const hasFeature = (feature: string): boolean => {
    if (!store.status) return false;

    // 宽限期内仍可使用原等级功能
    if (isExpired && !isGracePeriod) return false;

    const features = store.status.features;
    switch (feature) {
      case "transcription":
        return true;
      case "post_processing":
        return features.post_processing;
      case "custom_models":
        return features.custom_models;
      case "batch_transcription":
        return features.batch_transcription;
      default:
        return false;
    }
  };

  // 检查是否还有使用额度
  const hasUsageRemaining = (usageType: string): boolean => {
    if (!store.status) return false;

    // 宽限期内仍按原等级限制
    if (isExpired && !isGracePeriod) return false;

    const features = store.status.features;
    const usage = store.status.usage;

    switch (usageType) {
      case "transcriptions":
        return (
          features.transcription_limit === null ||
          usage.transcriptions < features.transcription_limit
        );
      case "recording_seconds":
        return (
          features.recording_hours === null ||
          usage.recording_seconds < features.recording_hours * 3600
        );
      case "model_downloads":
        return (
          features.model_downloads === null ||
          usage.model_downloads < features.model_downloads
        );
      default:
        return false;
    }
  };

  // 获取剩余转录次数
  const getRemainingTranscriptions = (): number | null => {
    if (!store.remaining) return null;
    return store.remaining.transcriptions;
  };

  // 获取剩余录音时长（小时）
  const getRemainingRecordingHours = (): number | null => {
    if (!store.remaining) return null;
    return store.remaining.recording_hours;
  };

  // 获取剩余可下载模型数
  const getRemainingModelDownloads = (): number | null => {
    if (!store.remaining) return null;
    return store.remaining.model_downloads;
  };

  return {
    // 状态
    status: store.status,
    usage: store.usage,
    remaining: store.remaining,
    isLoading: store.isLoading,
    error: store.error,

    // 许可证信息
    isActivated,
    tier,
    tierName,
    isExpired,
    isGracePeriod,

    // 操作
    activate: store.activate,
    deactivate: store.deactivate,
    refreshStatus: store.refreshStatus,
    refreshUsage: store.refreshUsage,
    clearError: store.clearError,

    // 功能检查
    hasFeature,
    hasUsageRemaining,
    getRemainingTranscriptions,
    getRemainingRecordingHours,
    getRemainingModelDownloads,
  };
};
