# Handy

[![Discord](https://img.shields.io/badge/Discord-%235865F2.svg?style=for-the-badge&logo=discord&logoColor=white)](https://discord.com/invite/WVBeWsNXK4)

**一款免费、开源、可扩展的语音转文字应用，完全离线运行。**

Handy 是一款跨平台桌面应用，提供简单、注重隐私的语音转录功能。按下快捷键，说话，你的文字就会出现在任何文本框中。这一切都发生在你自己的电脑上，不会向云端发送任何信息。

## 为什么选择 Handy？

Handy 的诞生是为了填补真正开源、可扩展的语音转文字工具的空白。正如 [handy.computer](https://handy.computer) 所述：

- **免费**：辅助工具应该属于每个人，而不是被付费墙阻挡
- **开源**：我们可以一起走得更远。为自己扩展 Handy，为更大的事业做出贡献
- **隐私**：你的声音留在你的电脑上。无需将音频发送到云端即可获得转录
- **简单**：一个工具，一个功能。转录你说的话，放入文本框

Handy 并不想成为最好的语音转文字应用——它想成为最容易被 fork 的应用。

## 工作原理

1. **按下**可配置的键盘快捷键开始/停止录音（或使用按住说话模式）
2. **说话**，在快捷键激活时说出你的话
3. **松开**，Handy 使用 Whisper 处理你的语音
4. **获得**转录文字，直接粘贴到你正在使用的任何应用中

整个过程完全在本地进行：

- 使用 Silero 的 VAD（语音活动检测）过滤静音
- 转录使用你选择的模型：
  - **Whisper 模型**（Small/Medium/Turbo/Large），在可用时支持 GPU 加速
  - **Parakeet V3** - CPU 优化模型，性能出色，支持自动语言检测
- 支持 Windows、macOS 和 Linux

## 快速开始

### 安装

1. 从[发布页面](https://github.com/cjpais/Handy/releases)或[官网](https://handy.computer)下载最新版本
   - **macOS**：也可通过 [Homebrew cask](https://formulae.brew.sh/cask/handy) 安装：`brew install --cask handy`
   - **Windows**：也可通过 [winget](https://github.com/microsoft/winget-pkgs) 安装：`winget install cjpais.Handy` \
     **注意：** Homebrew cask 和 winget 包不由 Handy 开发者维护。
2. 安装应用
3. 启动 Handy 并授予必要的系统权限（麦克风、辅助功能）
4. 在设置中配置你偏好的键盘快捷键
5. 开始转录！

### 开发环境搭建

详细的构建说明（包括平台特定要求）请参阅 [BUILD.md](BUILD.md)。

## 集成

<a href="https://www.raycast.com/mattiacolombomc/handy" title="安装 Handy Raycast 扩展"><img src="https://www.raycast.com/mattiacolombomc/handy/install_button@2x.png?v=1.1" height="64" style="height: 64px;" alt="安装 Handy Raycast 扩展" /></a>

从 [Raycast](https://www.raycast.com) 控制 Handy —— 开始/停止录音、浏览转录历史、管理词典、切换模型和语言。

[源码](https://github.com/mattiacolombomc/raycast-handy) · 由 [@mattiacolombomc](https://github.com/mattiacolombomc) 开发

## 架构

Handy 作为 Tauri 应用构建，结合了：

- **前端**：React + TypeScript，使用 Tailwind CSS 构建设置界面
- **后端**：Rust，用于系统集成、音频处理和机器学习推理
- **核心库**：
  - `whisper-rs`：使用 Whisper 模型进行本地语音识别
  - `transcribe-rs`：使用 Parakeet 模型进行 CPU 优化的语音识别
  - `cpal`：跨平台音频 I/O
  - `vad-rs`：语音活动检测
  - `rdev`：全局键盘快捷键和系统事件
  - `rubato`：音频重采样

### 调试模式

Handy 包含一个高级调试模式，用于开发和故障排除。通过以下快捷键访问：

- **macOS**：`Cmd+Shift+D`
- **Windows/Linux**：`Ctrl+Shift+D`

### CLI 参数

Handy 支持命令行标志，用于控制正在运行的实例和自定义启动行为。这些在所有平台（macOS、Windows、Linux）上都有效。

**远程控制标志**（通过单实例插件发送到已运行的实例）：

```bash
handy --toggle-transcription    # 切换录音开/关
handy --toggle-post-process     # 切换带后处理的录音开/关
handy --cancel                  # 取消当前操作
```

**启动标志：**

```bash
handy --start-hidden            # 启动时不显示主窗口
handy --no-tray                 # 启动时不显示系统托盘图标
handy --debug                   # 启用调试模式，显示详细日志
handy --help                    # 显示所有可用标志
```

标志可以组合使用，适用于自启动场景：

```bash
handy --start-hidden --no-tray
```

> **macOS 提示：** 当 Handy 作为应用包安装时，直接调用二进制文件：
>
> ```bash
> /Applications/Handy.app/Contents/MacOS/Handy --toggle-transcription
> ```

## 已知问题和当前限制

该项目正在积极开发中，存在一些[已知问题](https://github.com/cjpais/Handy/issues)。我们相信透明度的重要性：

### 主要问题（需要帮助）

**Whisper 模型崩溃：**

- Whisper 模型在某些系统配置上会崩溃（Windows 和 Linux）
- 并非所有系统都受影响——问题取决于配置
  - 如果你遇到崩溃并且是开发者，请帮助修复并提供调试日志！

**Wayland 支持（Linux）：**

- 对 Wayland 显示服务器的支持有限
- 需要 [`wtype`](https://github.com/atx/wtype) 或 [`dotool`](https://sr.ht/~geb/dotool/) 才能正常进行文本输入（安装说明见下方 [Linux 说明](#linux-说明)）

### Linux 说明

**文本输入工具：**

为了在 Linux 上实现可靠的文本输入，请为你的显示服务器安装相应的工具：

| 显示服务器 | 推荐工具 | 安装命令 |
|------------|----------|----------|
| X11 | `xdotool` | `sudo apt install xdotool` |
| Wayland | `wtype` | `sudo apt install wtype` |
| 两者 | `dotool` | `sudo apt install dotool`（需要 `input` 组） |

- **X11**：安装 `xdotool` 以支持直接输入和剪贴板粘贴快捷键
- **Wayland**：安装 `wtype`（首选）或 `dotool` 以使文本输入正常工作
- **dotool 设置**：需要将你的用户添加到 `input` 组：`sudo usermod -aG input $USER`（然后注销并重新登录）

没有这些工具，Handy 会回退到 enigo，这可能在 Wayland 上兼容性有限。

**其他说明：**

- **运行时库依赖（`libgtk-layer-shell.so.0`）**：
  - Handy 在 Linux 上链接 `gtk-layer-shell`。如果启动失败并显示 `error while loading shared libraries: libgtk-layer-shell.so.0`，请为你的发行版安装运行时包：

    | 发行版 | 要安装的包 | 示例命令 |
    |--------|-----------|----------|
    | Ubuntu/Debian | `libgtk-layer-shell0` | `sudo apt install libgtk-layer-shell0` |
    | Fedora/RHEL | `gtk-layer-shell` | `sudo dnf install gtk-layer-shell` |
    | Arch Linux | `gtk-layer-shell` | `sudo pacman -S gtk-layer-shell` |

  - 在 Ubuntu/Debian 上从源码构建时，你可能还需要 `libgtk-layer-shell-dev`。

- 录制悬浮窗在 Linux 上默认禁用（`Overlay Position: None`），因为某些合成器会将其视为活动窗口。当悬浮窗可见时，它可能会抢占焦点，阻止 Handy 粘贴回触发转录的应用。如果你仍然启用悬浮窗，请注意基于剪贴板的粘贴可能会失败或粘贴到错误的窗口。
- 如果你在使用应用时遇到问题，设置环境变量 `WEBKIT_DISABLE_DMABUF_RENDERER=1` 可能会有所帮助
- 如果 Handy 在 Linux 上无法可靠启动，请参阅[故障排除 → Linux 启动崩溃或不稳定](#linux-启动崩溃或不稳定)。
- **全局键盘快捷键（Wayland）：** 在 Wayland 上，系统级快捷键必须通过你的桌面环境或窗口管理器配置。使用 [CLI 标志](#cli-参数) 作为自定义快捷键的命令。

  **GNOME：**
  1. 打开 **设置 > 键盘 > 键盘快捷键 > 自定义快捷键**
  2. 点击 **+** 按钮添加新快捷键
  3. 将 **名称** 设置为 `Toggle Handy Transcription`
  4. 将 **命令** 设置为 `handy --toggle-transcription`
  5. 点击 **设置快捷键** 并按下你想要的组合键（例如 `Super+O`）

  **KDE Plasma：**
  1. 打开 **系统设置 > 快捷键 > 自定义快捷键**
  2. 点击 **编辑 > 新建 > 全局快捷键 > 命令/URL**
  3. 命名为 `Toggle Handy Transcription`
  4. 在 **触发器** 选项卡中，设置你想要的组合键
  5. 在 **操作** 选项卡中，将命令设置为 `handy --toggle-transcription`

  **Sway / i3：**

  添加到你的配置文件（`~/.config/sway/config` 或 `~/.config/i3/config`）：

  ```ini
  bindsym $mod+o exec handy --toggle-transcription
  ```

  **Hyprland：**

  添加到你的配置文件（`~/.config/hypr/hyprland.conf`）：

  ```ini
  bind = $mainMod, O, exec, handy --toggle-transcription
  ```

- 你也可以通过 Unix 信号在 Handy 外部管理全局快捷键，这让 Wayland 窗口管理器或其他快捷键守护进程可以保持对键绑定的所有权：

  | 信号 | 操作 | 示例 |
  |------|------|------|
  | `SIGUSR2` | 切换转录 | `pkill -USR2 -n handy` |
  | `SIGUSR1` | 切换带后处理的转录 | `pkill -USR1 -n handy` |

  示例 Sway 配置：

  ```ini
  bindsym $mod+o exec pkill -USR2 -n handy
  bindsym $mod+p exec pkill -USR1 -n handy
  ```

  这里的 `pkill` 只是传递信号——它不会终止进程。

**悬浮窗和粘贴问题（Linux）：**

- 录制悬浮窗可能会干扰在 Linux（X11）上将转录文本粘贴到目标应用
- **解决方案：** 打开 **设置 > 高级**，将 **"悬浮窗位置"** 设置为 **"无"** 以禁用悬浮窗
- 如果你仍然想要录制状态的音频确认，请启用 **"音频反馈"**（也在高级设置中）
- 从旧版本升级或从其他平台导入设置的用户可能需要手动应用此更改

### 平台支持

- **macOS（Intel 和 Apple Silicon）**
- **x64 Windows**
- **x64 Linux**

### 系统要求/建议

以下是在你自己的机器上运行 Handy 的建议。如果你不满足系统要求，应用的性能可能会下降。我们正在努力改进在各种计算机和硬件上的性能。

**Whisper 模型：**

- **macOS**：M 系列 Mac，Intel Mac
- **Windows**：Intel、AMD 或 NVIDIA GPU
- **Linux**：Intel、AMD 或 NVIDIA GPU
  - Ubuntu 22.04、24.04

**Parakeet V3 模型：**

- **仅 CPU 运行** - 可在各种硬件上运行
- **最低要求**：Intel Skylake（第 6 代）或同等 AMD 处理器
- **性能**：在中端硬件上约 5 倍实时速度（在 i5 上测试）
- **自动语言检测** - 无需手动选择语言

## 路线图和活跃开发

我们正在积极开发多个功能和改进。欢迎贡献和反馈！

### 进行中

**调试日志：**

- 添加调试日志到文件以帮助诊断问题

**macOS 键盘改进：**

- 支持 Globe 键作为转录触发器
- 重写 macOS 的全局快捷键处理，可能也适用于其他操作系统

**可选分析：**

- 收集匿名使用数据以帮助改进 Handy
- 隐私优先的方法，明确选择加入

**设置重构：**

- 清理和重构变得臃肿和混乱的设置系统
- 实现更好的设置管理抽象

**Tauri 命令清理：**

- 抽象和组织 Tauri 命令模式
- 研究 tauri-specta 以改进类型安全和组织

## 验证发布签名

Handy 发布工件使用 Tauri 的更新器签名格式签名。公钥存储在 [`src-tauri/tauri.conf.json`](src-tauri/tauri.conf.json) 的 `plugins.updater.pubkey` 下。

要手动验证发布，将 `ARTIFACT` 设置为你下载的文件名，将 `src-tauri/tauri.conf.json` 中的 `pubkey` 值保存到 `handy.pub.b64`，然后从 base64 解码公钥和匹配的 `.sig` 文件，并使用 `minisign` 验证工件：

```bash
# 替换为你下载的文件
ARTIFACT="Handy_0.8.1_amd64.AppImage"

python3 - "$ARTIFACT" <<'PY'
import base64, pathlib, sys

artifact = sys.argv[1]

pub = pathlib.Path("handy.pub.b64").read_text().strip()
pathlib.Path("handy.pub").write_bytes(base64.b64decode(pub))

sig = pathlib.Path(f"{artifact}.sig").read_text().strip()
pathlib.Path(f"{artifact}.minisig").write_bytes(base64.b64decode(sig))
PY

minisign -Vm "$ARTIFACT" \
  -p handy.pub \
  -x "$ARTIFACT.minisig"
```

成功时，`minisign` 会打印：

```text
Signature and comment signature verified
```

不要对这些 `.sig` 文件使用 `gpg`。

## 故障排除

### 手动安装模型（适用于代理用户或网络受限环境）

如果你在代理、防火墙或受限网络环境后面，Handy 无法自动下载模型，你可以手动下载和安装它们。URL 可以从任何浏览器公开访问。

#### 第 1 步：找到你的应用数据目录

1. 打开 Handy 设置
2. 导航到 **关于** 部分
3. 复制那里显示的"应用数据目录"路径，或使用快捷键：
   - **macOS**：`Cmd+Shift+D` 打开调试菜单
   - **Windows/Linux**：`Ctrl+Shift+D` 打开调试菜单

典型路径为：

- **macOS**：`~/Library/Application Support/com.pais.handy/`
- **Windows**：`C:\Users\{username}\AppData\Roaming\com.pais.handy\`
- **Linux**：`~/.config/com.pais.handy/`

#### 第 2 步：创建模型目录

在你的应用数据目录中，如果 `models` 文件夹不存在，请创建它：

```bash
# macOS/Linux
mkdir -p ~/Library/Application\ Support/com.pais.handy/models

# Windows (PowerShell)
New-Item -ItemType Directory -Force -Path "$env:APPDATA\com.pais.handy\models"
```

#### 第 3 步：下载模型文件

从下面下载你想要的模型

**Whisper 模型（单个 .bin 文件）：**

- Small (487 MB): `https://blob.handy.computer/ggml-small.bin`
- Medium (492 MB): `https://blob.handy.computer/whisper-medium-q4_1.bin`
- Turbo (1600 MB): `https://blob.handy.computer/ggml-large-v3-turbo.bin`
- Large (1100 MB): `https://blob.handy.computer/ggml-large-v3-q5_0.bin`

**Parakeet 模型（压缩包）：**

- V2 (473 MB): `https://blob.handy.computer/parakeet-v2-int8.tar.gz`
- V3 (478 MB): `https://blob.handy.computer/parakeet-v3-int8.tar.gz`

#### 第 4 步：安装模型

**Whisper 模型（.bin 文件）：**

只需将 `.bin` 文件直接放入 `models` 目录：

```
{app_data_dir}/models/
├── ggml-small.bin
├── whisper-medium-q4_1.bin
├── ggml-large-v3-turbo.bin
└── ggml-large-v3-q5_0.bin
```

**Parakeet 模型（.tar.gz 压缩包）：**

1. 解压 `.tar.gz` 文件
2. 将**解压后的目录**放入 `models` 文件夹
3. 目录名称必须完全匹配：
   - **Parakeet V2**：`parakeet-tdt-0.6b-v2-int8`
   - **Parakeet V3**：`parakeet-tdt-0.6b-v3-int8`

最终结构应如下所示：

```
{app_data_dir}/models/
├── parakeet-tdt-0.6b-v2-int8/     （包含模型文件的目录）
│   ├── （模型文件）
│   └── （配置文件）
└── parakeet-tdt-0.6b-v3-int8/     （包含模型文件的目录）
    ├── （模型文件）
    └── （配置文件）
```

**重要说明：**

- 对于 Parakeet 模型，解压后的目录名称**必须**完全匹配上述名称
- 不要重命名 Whisper 模型的 `.bin` 文件——使用下载 URL 中的确切文件名
- 放置文件后，重启 Handy 以检测新模型

#### 第 5 步：验证安装

1. 重启 Handy
2. 打开设置 → 模型
3. 你手动安装的模型现在应该显示为"已下载"
4. 选择你想使用的模型并测试转录

### 自定义 Whisper 模型

Handy 可以自动发现放在 `models` 目录中的自定义 Whisper GGML 模型。这对于想要使用未包含在默认模型列表中的微调或社区模型的用户很有用。

**如何使用：**

1. 获取 GGML `.bin` 格式的 Whisper 模型（例如从 [Hugging Face](https://huggingface.co/models?search=whisper%20ggml)）
2. 将 `.bin` 文件放入你的 `models` 目录（路径见上文）
3. 重启 Handy 以发现新模型
4. 模型将出现在模型设置页面的"自定义模型"部分

**重要：**

- 社区模型由用户提供，可能不会获得故障排除帮助
- 模型必须是有效的 Whisper GGML 格式（`.bin` 文件）
- 模型名称从文件名派生（例如 `my-custom-model.bin` → "My Custom Model"）

### Linux 启动崩溃或不稳定

如果 Handy 在 Linux 上无法可靠启动——例如，启动后不久崩溃、从不显示窗口或报告 Wayland 协议错误——请按顺序尝试以下步骤。

**1. 安装（或重新安装）`gtk-layer-shell`**

Handy 使用 `gtk-layer-shell` 作为录制悬浮窗，并在运行时链接它。缺失或损坏的安装是启动失败的最常见原因，可能表现为崩溃或在显示任何窗口之前挂起。确保为你的发行版安装了运行时包：

| 发行版 | 要安装的包 | 示例命令 |
|--------|-----------|----------|
| Ubuntu/Debian | `libgtk-layer-shell0` | `sudo apt install libgtk-layer-shell0` |
| Fedora/RHEL | `gtk-layer-shell` | `sudo dnf install gtk-layer-shell` |
| Arch Linux | `gtk-layer-shell` | `sudo pacman -S gtk-layer-shell` |

如果已经安装但仍然出现启动问题，请尝试重新安装（例如 `sudo pacman -S gtk-layer-shell`），以防库文件被部分升级损坏。

**2. 禁用 GTK layer shell 悬浮窗（`HANDY_NO_GTK_LAYER_SHELL`）**

如果安装库没有帮助，你可以完全跳过 `gtk-layer-shell` 初始化作为解决方案。在某些合成器（特别是 Wayland 下的 KDE Plasma）上，据报道它与录制悬浮窗交互不良。设置此变量后，悬浮窗会回退到普通的置顶窗口：

```bash
HANDY_NO_GTK_LAYER_SHELL=1 handy
```

**3. 禁用 WebKit DMA-BUF 渲染器（`WEBKIT_DISABLE_DMABUF_RENDERER`）**

在某些 GPU/驱动组合上，WebKitGTK DMA-BUF 渲染器可能导致窗口无法渲染或崩溃。尝试：

```bash
WEBKIT_DISABLE_DMABUF_RENDERER=1 handy
```

**使解决方案永久生效**

找到有效的标志后，从你的 shell 配置文件（`~/.bashrc`、`~/.zshenv` 等）或启动 Handy 的桌面自启动条目中导出它。如果你从 `.desktop` 文件启动 Handy，可以在 `Exec=` 行前添加前缀，例如：

```ini
Exec=env HANDY_NO_GTK_LAYER_SHELL=1 handy
```

如果解决方案对你有帮助，请[提交 issue](https://github.com/cjpais/Handy/issues) 描述你的发行版、桌面环境和会话类型——这些信息帮助我们缩小底层 bug 的范围。

### 如何贡献

1. **查看现有 issues**：[github.com/cjpais/Handy/issues](https://github.com/cjpais/Handy/issues)
2. **Fork 仓库**并创建功能分支
3. 在目标平台上**彻底测试**
4. **提交 pull request**，清楚描述更改
5. **加入讨论**：联系 [contact@handy.computer](mailto:contact@handy.computer)

目标是创建一个有用的工具，同时也是他人构建的基础——一个模式良好、简单且服务社区的代码库。

## 赞助商

<div align="center">
  我们感谢赞助商的支持，他们帮助 Handy 成为可能：
  <br><br>
  <a href="https://wordcab.com">
    <img src="sponsor-images/wordcab.png" alt="Wordcab" width="120" height="120">
  </a>
  &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;
  <a href="https://github.com/epicenter-so/epicenter">
    <img src="sponsor-images/epicenter.png" alt="Epicenter" width="120" height="120">
  </a>
  &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;
  <a href="https://boltai.com?utm_source=handy">
    <img src="sponsor-images/boltai.jpg" alt="Bolt AI" width="120" height="120">
  </a>
</div>

## 相关项目

- **[Handy CLI](https://github.com/cjpais/handy-cli)** - 原始的 Python 命令行版本
- **[handy.computer](https://handy.computer)** - 项目网站，包含演示和文档

## 许可证

MIT 许可证 - 详见 [LICENSE](LICENSE) 文件。

Handy 是开源软件，但 Handy 名称、Logo、图标和品牌资产不是开源的。非官方的 fork、重写和重新分发必须使用自己的品牌，不得暗示背书或关联。

## 致谢

- **OpenAI 的 Whisper** 语音识别模型
- **whisper.cpp 和 ggml** 出色的跨平台 Whisper 推理/加速
- **Silero** 优秀的轻量级 VAD
- **Tauri** 团队出色的基于 Rust 的应用框架
- **社区贡献者**帮助 Handy 变得更好
