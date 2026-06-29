import React from "react";
import { useTranslation } from "react-i18next";
import { Mic, Clock, Download } from "lucide-react";
import { useLicense } from "@/hooks/useLicense";
import { SettingsGroup } from "@/components/ui/SettingsGroup";
import { SettingContainer } from "@/components/ui/SettingContainer";

export const UsageStats: React.FC = () => {
  const { t } = useTranslation();
  const { usage, remaining, status } = useLicense();

  if (!usage || !status) return null;

  // 格式化时长
  const formatDuration = (seconds: number) => {
    const hours = Math.floor(seconds / 3600);
    const minutes = Math.floor((seconds % 3600) / 60);

    if (hours > 0) {
      return `${hours}h ${minutes}m`;
    }
    return `${minutes}m`;
  };

  // 计算进度百分比
  const getProgress = (used: number, limit: number | null) => {
    if (limit === null) return 0; // 无限制
    return Math.min((used / limit) * 100, 100);
  };

  // 获取进度条颜色
  const getProgressColor = (percentage: number) => {
    if (percentage >= 90) return "bg-red-500";
    if (percentage >= 70) return "bg-yellow-500";
    return "bg-green-500";
  };

  const transcriptionsProgress = getProgress(
    usage.transcriptions,
    status.features.transcription_limit
  );
  const recordingProgress = getProgress(
    usage.recording_seconds,
    status.features.recording_hours ? status.features.recording_hours * 3600 : null
  );
  const modelDownloadsProgress = getProgress(
    usage.model_downloads,
    status.features.model_downloads
  );

  return (
    <SettingsGroup title={t("license.usage")}>
      <div className="text-xs text-mid-gray/50 mb-4">
        {t("license.currentMonth")}: {usage.month}
      </div>

      {/* 转录次数 */}
      <SettingContainer
        title={t("license.transcriptions")}
        description={
          remaining?.transcriptions !== null
            ? t("license.remaining", { count: remaining?.transcriptions ?? 0 })
            : t("license.unlimited")
        }
        grouped={true}
      >
        <div className="w-full max-w-xs">
          <div className="flex justify-between text-sm mb-1">
            <span>{usage.transcriptions}</span>
            {status.features.transcription_limit !== null && (
              <span className="text-mid-gray/50">
                / {status.features.transcription_limit}
              </span>
            )}
          </div>
          {status.features.transcription_limit !== null && (
            <div className="w-full bg-mid-gray/20 rounded-full h-2">
              <div
                className={`h-2 rounded-full transition-all ${getProgressColor(transcriptionsProgress)}`}
                style={{ width: `${transcriptionsProgress}%` }}
              />
            </div>
          )}
        </div>
      </SettingContainer>

      {/* 录音时长 */}
      <SettingContainer
        title={t("license.recordingTime")}
        description={
          remaining?.recording_hours !== null
            ? t("license.remainingHours", {
                hours: (remaining?.recording_hours ?? 0).toFixed(1),
              })
            : t("license.unlimited")
        }
        grouped={true}
      >
        <div className="w-full max-w-xs">
          <div className="flex justify-between text-sm mb-1">
            <span>{formatDuration(usage.recording_seconds)}</span>
            {status.features.recording_hours !== null && (
              <span className="text-mid-gray/50">
                / {status.features.recording_hours}h
              </span>
            )}
          </div>
          {status.features.recording_hours !== null && (
            <div className="w-full bg-mid-gray/20 rounded-full h-2">
              <div
                className={`h-2 rounded-full transition-all ${getProgressColor(recordingProgress)}`}
                style={{ width: `${recordingProgress}%` }}
              />
            </div>
          )}
        </div>
      </SettingContainer>

      {/* 模型下载 */}
      <SettingContainer
        title={t("license.modelDownloads")}
        description={
          remaining?.model_downloads !== null
            ? t("license.remaining", { count: remaining?.model_downloads ?? 0 })
            : t("license.unlimited")
        }
        grouped={true}
      >
        <div className="w-full max-w-xs">
          <div className="flex justify-between text-sm mb-1">
            <span>{usage.model_downloads}</span>
            {status.features.model_downloads !== null && (
              <span className="text-mid-gray/50">
                / {status.features.model_downloads}
              </span>
            )}
          </div>
          {status.features.model_downloads !== null && (
            <div className="w-full bg-mid-gray/20 rounded-full h-2">
              <div
                className={`h-2 rounded-full transition-all ${getProgressColor(modelDownloadsProgress)}`}
                style={{ width: `${modelDownloadsProgress}%` }}
              />
            </div>
          )}
        </div>
      </SettingContainer>

      {/* 功能权限 */}
      <div className="mt-4 pt-4 border-t border-mid-gray/20">
        <h4 className="text-sm font-medium mb-3">{t("license.features")}</h4>
        <div className="grid grid-cols-2 gap-2">
          <FeatureItem
            name={t("license.feature.postProcessing")}
            enabled={status.features.post_processing}
          />
          <FeatureItem
            name={t("license.feature.customModels")}
            enabled={status.features.custom_models}
          />
          <FeatureItem
            name={t("license.feature.batchTranscription")}
            enabled={status.features.batch_transcription}
          />
        </div>
      </div>
    </SettingsGroup>
  );
};

// 功能项组件
const FeatureItem: React.FC<{ name: string; enabled: boolean }> = ({
  name,
  enabled,
}) => (
  <div className="flex items-center gap-2">
    <div
      className={`w-2 h-2 rounded-full ${
        enabled ? "bg-green-500" : "bg-mid-gray/30"
      }`}
    />
    <span
      className={`text-sm ${
        enabled ? "text-foreground" : "text-mid-gray/50"
      }`}
    >
      {name}
    </span>
  </div>
);
