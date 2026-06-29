import React, { useState, useCallback } from "react";
import { useTranslation } from "react-i18next";
import { Upload, Key } from "lucide-react";
import { useLicense } from "@/hooks/useLicense";
import { SettingsGroup } from "@/components/ui/SettingsGroup";
import { SettingContainer } from "@/components/ui/SettingContainer";

export const LicenseActivation: React.FC = () => {
  const { t } = useTranslation();
  const { activate, isLoading, error } = useLicense();
  const [licenseKey, setLicenseKey] = useState("");
  const [dragActive, setDragActive] = useState(false);

  // 处理文件上传
  const handleFileUpload = useCallback(
    async (file: File) => {
      try {
        const buffer = await file.arrayBuffer();
        const data = Array.from(new Uint8Array(buffer));
        const success = await activate(data);
        if (success) {
          setLicenseKey("");
        }
      } catch (err) {
        console.error("Failed to read license file:", err);
      }
    },
    [activate]
  );

  // 处理拖放
  const handleDrag = useCallback((e: React.DragEvent) => {
    e.preventDefault();
    e.stopPropagation();
    if (e.type === "dragenter" || e.type === "dragover") {
      setDragActive(true);
    } else if (e.type === "dragleave") {
      setDragActive(false);
    }
  }, []);

  const handleDrop = useCallback(
    (e: React.DragEvent) => {
      e.preventDefault();
      e.stopPropagation();
      setDragActive(false);

      if (e.dataTransfer.files && e.dataTransfer.files[0]) {
        handleFileUpload(e.dataTransfer.files[0]);
      }
    },
    [handleFileUpload]
  );

  // 处理文件选择
  const handleFileSelect = useCallback(
    (e: React.ChangeEvent<HTMLInputElement>) => {
      if (e.target.files && e.target.files[0]) {
        handleFileUpload(e.target.files[0]);
      }
    },
    [handleFileUpload]
  );

  // 处理许可证密钥激活（预留）
  const handleKeyActivation = useCallback(async () => {
    // TODO: 实现许可证密钥激活逻辑
    console.log("License key activation not yet implemented:", licenseKey);
  }, [licenseKey]);

  return (
    <SettingsGroup title={t("license.activate")}>
      {/* 文件上传区域 */}
      <div
        className={`border-2 border-dashed rounded-lg p-8 text-center transition-colors ${
          dragActive
            ? "border-logo-primary bg-logo-primary/10"
            : "border-mid-gray/30 hover:border-mid-gray/50"
        }`}
        onDragEnter={handleDrag}
        onDragLeave={handleDrag}
        onDragOver={handleDrag}
        onDrop={handleDrop}
      >
        <Upload className="mx-auto h-12 w-12 text-mid-gray/50 mb-4" />
        <p className="text-sm text-mid-gray/70 mb-4">
          {t("license.dragDropLicense")}
        </p>
        <label className="inline-flex items-center gap-2 px-4 py-2 bg-logo-primary/80 hover:bg-logo-primary text-white rounded-lg cursor-pointer transition-colors">
          <Upload className="h-4 w-4" />
          {t("license.selectFile")}
          <input
            type="file"
            className="hidden"
            accept=".lic,.license"
            onChange={handleFileSelect}
          />
        </label>
      </div>

      {/* 分隔线 */}
      <div className="flex items-center gap-4 my-4">
        <div className="flex-1 h-px bg-mid-gray/20"></div>
        <span className="text-xs text-mid-gray/50">{t("common.or")}</span>
        <div className="flex-1 h-px bg-mid-gray/20"></div>
      </div>

      {/* 许可证密钥输入 */}
      <SettingContainer
        title={t("license.enterKey")}
        description={t("license.enterKeyDescription")}
        grouped={true}
      >
        <div className="flex gap-2 w-full max-w-md">
          <div className="relative flex-1">
            <Key className="absolute left-3 top-1/2 transform -translate-y-1/2 h-4 w-4 text-mid-gray/50" />
            <input
              type="text"
              value={licenseKey}
              onChange={(e) => setLicenseKey(e.target.value)}
              placeholder="XXXX-XXXX-XXXX-XXXX"
              className="w-full pl-10 pr-3 py-2 bg-background border border-mid-gray/20 rounded-lg text-sm focus:outline-none focus:border-logo-primary"
              disabled={isLoading}
            />
          </div>
          <button
            onClick={handleKeyActivation}
            disabled={!licenseKey || isLoading}
            className="px-4 py-2 bg-logo-primary/80 hover:bg-logo-primary text-white rounded-lg text-sm transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
          >
            {t("license.activate")}
          </button>
        </div>
      </SettingContainer>

      {/* 错误提示 */}
      {error && (
        <div className="mt-4 p-3 bg-red-500/10 border border-red-500/20 rounded-lg">
          <p className="text-red-400 text-sm">{error}</p>
        </div>
      )}

      {/* 帮助文本 */}
      <div className="mt-4 p-4 bg-mid-gray/5 rounded-lg">
        <p className="text-xs text-mid-gray/60">
          {t("license.activationHelp")}
        </p>
      </div>
    </SettingsGroup>
  );
};
