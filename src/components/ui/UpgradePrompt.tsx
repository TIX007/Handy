import React from "react";
import { useTranslation } from "react-i18next";
import { ArrowUpRight, Lock } from "lucide-react";

interface UpgradePromptProps {
  feature?: string;
  usageType?: string;
  compact?: boolean;
}

export const UpgradePrompt: React.FC<UpgradePromptProps> = ({
  feature,
  usageType,
  compact = false,
}) => {
  const { t } = useTranslation();

  // 获取提示信息
  const getMessage = () => {
    if (feature) {
      return t("license.upgradePromptFeature", { feature: t(`license.feature.${feature}`) });
    }
    if (usageType) {
      return t("license.upgradePromptUsage", { type: t(`license.${usageType}`) });
    }
    return t("license.upgradePrompt");
  };

  // 获取所需等级
  const getRequiredTier = () => {
    if (feature === "post_processing" || feature === "custom_models") {
      return "Professional";
    }
    if (feature === "batch_transcription") {
      return "Enterprise";
    }
    return null;
  };

  const requiredTier = getRequiredTier();

  if (compact) {
    return (
      <div className="flex items-center gap-2 px-3 py-2 bg-logo-primary/10 border border-logo-primary/20 rounded-lg">
        <Lock className="h-4 w-4 text-logo-primary" />
        <span className="text-sm text-logo-primary">{getMessage()}</span>
      </div>
    );
  }

  return (
    <div className="p-4 bg-gradient-to-r from-logo-primary/10 to-logo-secondary/10 border border-logo-primary/20 rounded-lg">
      <div className="flex items-start gap-3">
        <div className="p-2 bg-logo-primary/20 rounded-lg">
          <Lock className="h-5 w-5 text-logo-primary" />
        </div>
        <div className="flex-1">
          <h4 className="font-medium text-foreground mb-1">
            {t("license.upgradeRequired")}
          </h4>
          <p className="text-sm text-mid-gray/70 mb-3">{getMessage()}</p>
          {requiredTier && (
            <p className="text-xs text-mid-gray/50 mb-3">
              {t("license.requiredTier")}:{" "}
              <span className="font-medium text-logo-primary">
                {requiredTier}
              </span>
            </p>
          )}
          <a
            href="https://handy.computer/pricing"
            target="_blank"
            rel="noopener noreferrer"
            className="inline-flex items-center gap-1 px-4 py-2 bg-logo-primary hover:bg-logo-primary/90 text-white rounded-lg text-sm transition-colors"
          >
            {t("license.upgrade")}
            <ArrowUpRight className="h-4 w-4" />
          </a>
        </div>
      </div>
    </div>
  );
};
