import { create } from "zustand";
import { subscribeWithSelector } from "zustand/middleware";
import { commands } from "@/bindings";

// 从 Rust 类型导出的许可证类型
export type LicenseTier = "basic" | "professional" | "enterprise";

export interface LicenseFeatures {
  transcription_limit: number | null;
  recording_hours: number | null;
  model_downloads: number | null;
  post_processing: boolean;
  custom_models: boolean;
  batch_transcription: boolean;
}

export interface LicensePayload {
  key: string;
  tier: LicenseTier;
  name: string;
  email: string;
  issued_at: number;
  expires_at: number;
  max_devices: number;
  features: LicenseFeatures;
}

export interface LicenseStatus {
  is_activated: boolean;
  tier: LicenseTier;
  name: string;
  email: string;
  key: string;
  is_expired: boolean;
  is_grace_period: boolean;
  grace_period_end: number | null;
  expires_at: number | null;
  usage: UsageCounter;
  features: LicenseFeatures;
}

export interface UsageCounter {
  month: string;
  transcriptions: number;
  recording_seconds: number;
  model_downloads: number;
}

export interface RemainingUsage {
  transcriptions: number | null;
  recording_hours: number | null;
  model_downloads: number | null;
}

interface LicenseStore {
  // 状态
  status: LicenseStatus | null;
  usage: UsageCounter | null;
  remaining: RemainingUsage | null;
  isLoading: boolean;
  error: string | null;

  // 操作
  initialize: () => Promise<void>;
  activate: (licenseData: number[]) => Promise<boolean>;
  deactivate: () => Promise<void>;
  refreshStatus: () => Promise<void>;
  refreshUsage: () => Promise<void>;
  refreshRemaining: () => Promise<void>;
  checkFeature: (feature: string) => Promise<boolean>;
  checkUsageLimit: (usageType: string) => Promise<boolean>;
  clearError: () => void;
}

export const useLicenseStore = create<LicenseStore>()(
  subscribeWithSelector((set, get) => ({
    // 初始状态
    status: null,
    usage: null,
    remaining: null,
    isLoading: false,
    error: null,

    // 初始化
    initialize: async () => {
      set({ isLoading: true, error: null });
      try {
        await Promise.all([
          get().refreshStatus(),
          get().refreshUsage(),
          get().refreshRemaining(),
        ]);
      } catch (error) {
        console.error("Failed to initialize license store:", error);
        set({ error: String(error) });
      } finally {
        set({ isLoading: false });
      }
    },

    // 激活许可证
    activate: async (licenseData: number[]) => {
      set({ isLoading: true, error: null });
      try {
        const result = await commands.activateLicense(licenseData);
        if (result.status === "ok") {
          await get().refreshStatus();
          await get().refreshUsage();
          await get().refreshRemaining();
          return true;
        } else {
          set({ error: result.error });
          return false;
        }
      } catch (error) {
        set({ error: String(error) });
        return false;
      } finally {
        set({ isLoading: false });
      }
    },

    // 停用许可证
    deactivate: async () => {
      set({ isLoading: true, error: null });
      try {
        const result = await commands.deactivateLicense();
        if (result.status === "ok") {
          await get().refreshStatus();
          await get().refreshUsage();
          await get().refreshRemaining();
        } else {
          set({ error: result.error });
        }
      } catch (error) {
        set({ error: String(error) });
      } finally {
        set({ isLoading: false });
      }
    },

    // 刷新许可证状态
    refreshStatus: async () => {
      try {
        const result = await commands.getLicenseStatus();
        if (result.status === "ok") {
          set({ status: result.data });
        } else {
          set({ error: result.error });
        }
      } catch (error) {
        console.error("Failed to refresh license status:", error);
      }
    },

    // 刷新使用统计
    refreshUsage: async () => {
      try {
        const result = await commands.getUsageStats();
        if (result.status === "ok") {
          set({ usage: result.data });
        }
      } catch (error) {
        console.error("Failed to refresh usage stats:", error);
      }
    },

    // 刷新剩余使用量
    refreshRemaining: async () => {
      try {
        const result = await commands.getRemainingUsage();
        if (result.status === "ok") {
          set({ remaining: result.data });
        }
      } catch (error) {
        console.error("Failed to refresh remaining usage:", error);
      }
    },

    // 检查功能是否可用
    checkFeature: async (feature: string) => {
      try {
        const result = await commands.checkLicenseFeature(feature);
        return result.status === "ok" && result.data;
      } catch {
        return false;
      }
    },

    // 检查使用限制
    checkUsageLimit: async (usageType: string) => {
      try {
        const result = await commands.checkLicenseUsageLimit(usageType);
        return result.status === "ok" && result.data;
      } catch {
        return false;
      }
    },

    // 清除错误
    clearError: () => set({ error: null }),
  }))
);
