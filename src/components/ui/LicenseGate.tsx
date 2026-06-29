import React from "react";
import { useLicense } from "@/hooks/useLicense";
import { UpgradePrompt } from "./UpgradePrompt";

interface LicenseGateProps {
  feature: string;
  children: React.ReactNode;
  fallback?: React.ReactNode;
  showUpgradePrompt?: boolean;
}

export const LicenseGate: React.FC<LicenseGateProps> = ({
  feature,
  children,
  fallback,
  showUpgradePrompt = true,
}) => {
  const { hasFeature, isActivated } = useLicense();

  if (!hasFeature(feature)) {
    if (fallback) {
      return <>{fallback}</>;
    }
    if (showUpgradePrompt) {
      return <UpgradePrompt feature={feature} />;
    }
    return null;
  }

  return <>{children}</>;
};

interface UsageGateProps {
  usageType: string;
  children: React.ReactNode;
  fallback?: React.ReactNode;
  showUpgradePrompt?: boolean;
}

export const UsageGate: React.FC<UsageGateProps> = ({
  usageType,
  children,
  fallback,
  showUpgradePrompt = true,
}) => {
  const { hasUsageRemaining } = useLicense();

  if (!hasUsageRemaining(usageType)) {
    if (fallback) {
      return <>{fallback}</>;
    }
    if (showUpgradePrompt) {
      return <UpgradePrompt usageType={usageType} />;
    }
    return null;
  }

  return <>{children}</>;
};
