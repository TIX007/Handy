# 构建说明

本指南介绍如何设置开发环境并从源代码在不同平台上构建聆听。

## Prerequisites

### All Platforms

- [Rust](https://rustup.rs/) (latest stable)
- [Bun](https://bun.sh/) package manager
- [Tauri Prerequisites](https://tauri.app/start/prerequisites/)

### Platform-Specific Requirements

#### macOS

- Xcode Command Line Tools
- Install with: `xcode-select --install`

##### Intel Mac (x86_64)

Prebuilt ONNX Runtime binaries are not available for Intel Macs. Install ONNX Runtime via Homebrew and link dynamically:

```bash
brew install onnxruntime
ORT_LIB_LOCATION=$(brew --prefix onnxruntime)/lib ORT_PREFER_DYNAMIC_LINK=1 bun run tauri dev
```

The same environment variables apply for production builds:

```bash
ORT_LIB_LOCATION=$(brew --prefix onnxruntime)/lib ORT_PREFER_DYNAMIC_LINK=1 bun run tauri build
```

#### Windows

- Microsoft C++ Build Tools
- Visual Studio 2019/2022 with C++ development tools
- Or Visual Studio Build Tools 2019/2022

#### Linux

- Build essentials
- ALSA development libraries
- Install with:

  ```bash
  # Ubuntu/Debian
  sudo apt update
  sudo apt install build-essential libasound2-dev pkg-config libssl-dev libvulkan-dev vulkan-tools glslc libgtk-3-dev libwebkit2gtk-4.1-dev libayatana-appindicator3-dev librsvg2-dev libgtk-layer-shell0 libgtk-layer-shell-dev patchelf cmake

  # Fedora/RHEL
  sudo dnf groupinstall "Development Tools"
  sudo dnf install alsa-lib-devel pkgconf openssl-devel vulkan-devel \
    gtk3-devel webkit2gtk4.1-devel libappindicator-gtk3-devel librsvg2-devel \
    gtk-layer-shell gtk-layer-shell-devel \
    cmake

  # Arch Linux
  sudo pacman -S base-devel alsa-lib pkgconf openssl vulkan-devel \
    gtk3 webkit2gtk-4.1 libappindicator-gtk3 librsvg gtk-layer-shell \
    cmake
  ```

## 设置说明

### 1. 克隆仓库

```bash
git clone git@github.com:cjpais/Handy.git
cd Handy
```

### 2. 安装依赖

```bash
bun install
```

### 3. 启动开发服务器

```bash
bun tauri dev
```

### 4. 生产环境构建

```bash
bun run tauri build
```

这将编译发布二进制文件并生成平台特定的捆绑包（Linux 上的 deb、rpm、AppImage；macOS 上的 dmg；Windows 上的 msi）。

## Linux 安装（从源代码）

原始二进制文件 (`src-tauri/target/release/listening`) 无法单独运行——它需要 Tauri 资源文件（托盘图标、声音、VAD 模型）位于预期路径。

**从 deb 包安装**（适用于任何 Linux 发行版）：

```bash
cd /tmp
ar x /path/to/聆听/src-tauri/target/release/bundle/deb/聆听_*_amd64.deb data.tar.gz
tar xzf data.tar.gz
sudo cp usr/bin/listening /usr/bin/
sudo cp -r usr/lib/聆听 /usr/lib/
sudo cp -r usr/share/icons/hicolor/* /usr/share/icons/hicolor/
sudo cp usr/share/applications/聆听.desktop /usr/share/applications/
```

后续重新构建后，只需重新复制二进制文件：

```bash
sudo cp src-tauri/target/release/listening /usr/bin/
```

资源文件仅在上游更改时需要重新复制（新图标、声音等）。

## 故障排除

### AppImage 构建在 Arch / 滚动发布发行版上失败

`linuxdeploy` 捆绑了自己的 `strip` 二进制文件，该文件太旧，无法处理在滚动发布发行版（Arch、CachyOS、Manjaro、EndeavourOS）上使用新工具链构建的系统库。

Tauri 的错误：

```
Bundling 聆听_*_amd64.AppImage
failed to bundle project `failed to run linuxdeploy`
```

Tauri 吞掉了真正的 linuxdeploy 错误。要查看它，请手动运行 linuxdeploy：

```bash
cd src-tauri/target/release/bundle/appimage
~/.cache/tauri/linuxdeploy-x86_64.AppImage --appimage-extract-and-run \
  --appdir 聆听.AppDir --plugin gtk --output appimage
```

**解决方法：** 二进制文件、deb 和 rpm 包都可以正常构建——只有 AppImage 步骤失败。要跳过它：

```bash
bun run tauri build -- --bundles deb
```

Then install using the deb extraction method above.
