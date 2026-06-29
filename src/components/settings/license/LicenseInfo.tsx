import React from "react";
import { useTranslation } from "react-i18next";
import { Shield, User, Mail, Calendar, CheckCircle, AlertCircle, Clock } from "lucide-react";
import { useLicense } from "@/hooks/useLicense";
import { SettingsGroup } from "@/components/ui/SettingsGroup";
import { SettingContainer } from "@/components/ui/SettingContainer";

export const LicenseInfo: React.FC = () => {
  const { t } = useTranslation();
  const {
    status,
    tier,
    tierName,
    isExpired,
    isGracePeriod,
    deactivate,
    isLoading,
  } = useLicense();

  if (!status) return null;

  // 格式化过期时间
  const formatExpiry = (timestamp: number | null) => {
    if (!timestamp) return t("license.neverExpires");
    const date = new Date(timestamp * 1000);
    return date.toLocaleDateString();
  };

  // 获取状态图标和颜色
  const getStatusDisplay = () => {
    if (isExpired && !isGracePeriod) {
      return {
        icon: AlertCircle,
        color: "text-red-400",
        bgColor: "bg-red-500/10",
        text: t("license.expired"),
      };
    }
    if (isGracePeriod) {
      return {
        icon: Clock,
        color: "text-yellow-400",
        bgColor: "bg-yellow-500/10",
        text: t("license.gracePeriod"),
      };
    }
    return {
      icon: CheckCircle,
      color: "text-green-400",
      bgColor: "bg-green-500/10",
      text: t("license.active"),
    };
  };

  const statusDisplay = getStatusDisplay();
  const StatusIcon = statusDisplay.icon;

  return (
    <SettingsGroup title={t("license.status")}>
      {/* 许可证状态 */}
      <div className={`${statusDisplay.bgColor} rounded-lg p-4 mb-4`}>
        <div className="flex items-center gap-3">
          <StatusIcon className={`h-5 w-5 ${statusDisplay.color}`} />
          <div>
            <p className={`font-medium ${statusDisplay.color}`}>
              {statusDisplay.text}
            </p>
            {isGracePeriod && status.grace_period_end && (
              <p className="text-xs text-yellow-400/70 mt-1">
                {t("license.gracePeriodEnds", {
                  date: formatExpiry(status.grace_period_end),
                })}
              </p>
            )}
          </div>
        </div>
      </div>

      {/* 许可证等级 */}
      <SettingContainer
        title={t("license.tier")}
        description={t("license.tierDescription")}
        grouped={true}
      >
        <div className="flex items-center gap-2">
          <Shield className="h-4 w-4 text-logo-primary" />
          <span className="font-medium">{tierName}</span>
        </div>
      </SettingContainer>

      {/* 用户信息 */}
      {status.name && (
        <SettingContainer
          title={t("license.name")}
          description=""
          grouped={true}
        >
          <div className="flex items-center gap-2">
            <User className="h-4 w-4 text-mid-gray/50" />
            <span>{status.name}</span>
          </div>
        </SettingContainer>
      )}

      {status.email && (
        <SettingContainer
          title={t("license.email")}
          description=""
          grouped={true}
        >
          <div className="flex items-center gap-2">
            <Mail className="h-4 w-4 text-mid-gray/50" />
            <span>{status.email}</span>
          </div>
        </SettingContainer>
      )}

      {/* 过期时间 */}
      <SettingContainer
        title={t("license.expires")}
        description=""
        grouped={true}
      >
        <div className="flex items-center gap-2">
          <Calendar className="h-4 w-4 text-mid-gray/50" />
          <span>{formatExpiry(status.expires_at)}</span>
        </div>
      </SettingContainer>

      {/* 许可证密钥 */}
      {status.key && (
        <SettingContainer
          title={t("license.licenseKey")}
          description=""
          grouped={true}
        >
          <span className="font-mono text-sm text-mid-gray/70">
            {status.key}
          </span>
        </SettingContainer>
      )}

      {/* 停用按钮 */}
      <div className="mt-4 pt-4 border-t border-mid-gray/20">
        <button
          onClick={deactivate}
          disabled={isLoading}
          className="px-4 py-2 text-red-400 hover:text-red-300 hover:bg-red-500/10 rounded-lg text-sm transition-colors disabled:opacity-50"
        >
          {t("license.deactivate")}
        </button>
      </div>
    </SettingsGroup>
  );
};
