# 构建说明

本指南介绍如何在不同平台上设置开发环境并从源码构建 Handy。

## 前提条件

### 所有平台

- [Rust](https://rustup.rs/)（最新稳定版）
- [Bun](https://bun.sh/) 包管理器
- [Tauri 前提条件](https://tauri.app/start/prerequisites/)

### 平台特定要求

#### macOS

- Xcode 命令行工具
- 安装命令：`xcode-select --install`

##### Intel Mac (x86_64)

预编译的 ONNX Runtime 二进制文件不适用于 Intel Mac。通过 Homebrew 安装 ONNX Runtime 并动态链接：

```bash
brew install onnxruntime
ORT_LIB_LOCATION=$(brew --prefix onnxruntime)/lib ORT_PREFER_DYNAMIC_LINK=1 bun run tauri dev
```

生产构建使用相同的环境变量：

```bash
ORT_LIB_LOCATION=$(brew --prefix onnxruntime)/lib ORT_PREFER_DYNAMIC_LINK=1 bun run tauri build
```

#### Windows

- Microsoft C++ 构建工具
- Visual Studio 2019/2022，包含 C++ 开发工具
- 或 Visual Studio Build Tools 2019/2022

#### Linux

- 构建基础工具
- ALSA 开发库
- 安装命令：

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

### 4. 生产构建

```bash
bun run tauri build
```

这会编译一个发布二进制文件并生成平台特定的包（Linux 上的 deb、rpm、AppImage；macOS 上的 dmg；Windows 上的 msi）。

## Linux 安装（从源码）

原始二进制文件（`src-tauri/target/release/handy`）无法单独运行——它需要 Tauri 资源文件（托盘图标、声音、VAD 模型）位于预期路径。

**从 deb 包安装**（适用于任何 Linux 发行版）：

```bash
cd /tmp
ar x /path/to/Handy/src-tauri/target/release/bundle/deb/Handy_*_amd64.deb data.tar.gz
tar xzf data.tar.gz
sudo cp usr/bin/handy /usr/bin/
sudo cp -r usr/lib/Handy /usr/lib/
sudo cp -r usr/share/icons/hicolor/* /usr/share/icons/hicolor/
sudo cp usr/share/applications/Handy.desktop /usr/share/applications/
```

后续重新构建后，只需重新复制二进制文件：

```bash
sudo cp src-tauri/target/release/handy /usr/bin/
```

只有当上游资源发生变化（新图标、声音等）时，才需要重新复制资源。

## 故障排除

### AppImage 构建在 Arch / 滚动发行版上失败

`linuxdeploy` 自带的 `strip` 二进制文件太旧，无法处理在滚动发行版（Arch、CachyOS、Manjaro、EndeavourOS）上使用较新工具链构建的系统库。

Tauri 的错误信息：

```
Bundling Handy_*_amd64.AppImage
failed to bundle project `failed to run linuxdeploy`
```

Tauri 吞掉了真正的 linuxdeploy 错误。要查看它，请手动运行 linuxdeploy：

```bash
cd src-tauri/target/release/bundle/appimage
~/.cache/tauri/linuxdeploy-x86_64.AppImage --appimage-extract-and-run \
  --appdir Handy.AppDir --plugin gtk --output appimage
```

**解决方案：** 二进制文件、deb 和 rpm 包都可以正常构建——只有 AppImage 步骤失败。要跳过它：

```bash
bun run tauri build -- --bundles deb
```

然后使用上面的 deb 提取方法安装。
