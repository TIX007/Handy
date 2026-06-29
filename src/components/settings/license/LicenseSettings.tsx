import React from "react";
import { useTranslation } from "react-i18next";
import { useLicense } from "@/hooks/useLicense";
import { LicenseActivation } from "./LicenseActivation";
import { LicenseInfo } from "./LicenseInfo";
import { UsageStats } from "./UsageStats";
import { SettingsGroup } from "@/components/ui/SettingsGroup";

export const LicenseSettings: React.FC = () => {
  const { t } = useTranslation();
  const { isActivated, isLoading, error, clearError } = useLicense();

  if (isLoading) {
    return (
      <div className="max-w-3xl w-full mx-auto space-y-6">
        <div className="flex items-center justify-center p-8">
          <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-logo-primary"></div>
        </div>
      </div>
    );
  }

  return (
    <div className="max-w-3xl w-full mx-auto space-y-6">
      {error && (
        <div className="bg-red-500/10 border border-red-500/20 rounded-lg p-4">
          <p className="text-red-400 text-sm">{error}</p>
          <button
            onClick={clearError}
            className="text-red-400/60 hover:text-red-400 text-xs mt-1"
          >
            {t("common.close")}
          </button>
        </div>
      )}

      {!isActivated ? (
        <LicenseActivation />
      ) : (
        <>
          <LicenseInfo />
          <UsageStats />
        </>
      )}
    </div>
  );
};
