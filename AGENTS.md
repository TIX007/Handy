# AGENTS.md

本文件为在此代码库中工作的 AI 编码助手提供指导。

## Development Commands

**Prerequisites:**

- [Rust](https://rustup.rs/) (latest stable)
- [Bun](https://bun.sh/) package manager

**Core Development:**

```bash
# Install dependencies
bun install

# Run in development mode
bun run tauri dev
# If cmake error on macOS:
CMAKE_POLICY_VERSION_MINIMUM=3.5 bun run tauri dev

# Build for production
bun run tauri build

# Frontend only development
bun run dev        # Start Vite dev server
bun run build      # Build frontend (TypeScript + Vite)
bun run preview    # Preview built frontend
```

**Linting and Formatting (run before committing):**

```bash
bun run lint              # ESLint for frontend
bun run lint:fix          # ESLint with auto-fix
bun run format            # Prettier + cargo fmt
bun run format:check      # Check formatting without changes
bun run format:frontend   # Prettier only
bun run format:backend    # cargo fmt only
```

**Model Setup (Required for Development):**

```bash
mkdir -p src-tauri/resources/models
curl -o src-tauri/resources/models/silero_vad_v4.onnx https://blob.handy.computer/silero_vad_v4.onnx
```

For detailed platform-specific build setup, see [BUILD.md](BUILD.md).

## 架构概述

聆听是一款跨平台桌面语音转文字应用，使用 Tauri 2.x 构建（Rust 后端 + React/TypeScript 前端）。

### 后端结构 (src-tauri/src/)

- `lib.rs` - 主入口点，Tauri 设置，管理器初始化
- `managers/` - 核心业务逻辑：
  - `audio.rs` - 音频录制和设备管理
  - `model.rs` - 模型下载和管理
  - `transcription.rs` - 语音转文字处理管道
  - `history.rs` - 转录历史存储
- `audio_toolkit/` - 底层音频处理：
  - `audio/` - 设备枚举、录制、重采样
  - `vad/` - 语音活动检测 (Silero VAD)
- `commands/` - Tauri 命令处理器，用于前端通信
- `cli.rs` - CLI 参数定义 (clap derive)
- `shortcut.rs` - 全局键盘快捷键处理
- `settings.rs` - 应用设置管理
- `overlay.rs` - 录制悬浮窗（平台特定）
- `signal_handle.rs` - `send_transcription_input()` 可复用函数
- `utils.rs` - 平台检测辅助工具

### 前端结构 (src/)

- `App.tsx` - 主组件，包含引导流程
- `components/` - React UI 组件：
  - `settings/` - 设置界面
  - `model-selector/` - 模型管理界面
  - `onboarding/` - 首次运行体验
  - `overlay/` - 录制悬浮窗 UI
  - `update-checker/` - 应用更新通知
  - `shared/`, `ui/`, `icons/`, `footer/` - 共享组件
- `hooks/useSettings.ts` - 设置状态管理 hook
- `stores/settingsStore.ts` - Zustand 设置存储
- `bindings.ts` - 自动生成的 Tauri 类型绑定 (via tauri-specta)
- `overlay/` - 录制悬浮窗入口点
- `lib/types.ts` - 共享 TypeScript 类型定义

### 关键架构模式

**管理器模式：** 核心功能组织成管理器（音频、模型、转录），在启动时初始化并通过 Tauri 状态管理。

**命令-事件架构：** 前端 → 后端通过 Tauri 命令；后端 → 前端通过事件。

**管道处理：** 音频 → VAD → Whisper/Parakeet → 文本输出 → 剪贴板/粘贴

**状态流：** Zustand → Tauri 命令 → Rust 状态 → 持久化 (tauri-plugin-store)

### 技术栈

**核心库：**

- `whisper-rs` - 本地 Whisper 推理，支持 GPU 加速
- `cpal` - 跨平台音频 I/O
- `vad-rs` - 语音活动检测
- `rdev` - 全局键盘快捷键
- `rubato` - 音频重采样
- `rodio` - 反馈声音播放

### 应用流程

1. **初始化：** 应用启动时最小化到托盘，加载设置，初始化管理器
2. **模型设置：** 首次运行下载首选 Whisper 模型 (Small/Medium/Turbo/Large)
3. **录制：** 全局快捷键触发带 VAD 过滤的音频录制
4. **处理：** 音频发送到 Whisper 模型进行转录
5. **输出：** 文本粘贴到活动应用的系统剪贴板

### 设置系统

设置使用 Tauri 的 store 插件存储，支持响应式更新：

- 键盘快捷键（可配置，支持按住说话）
- 音频设备（麦克风/输出选择）
- 模型偏好（Small/Medium/Turbo/Large Whisper 变体）
- 音频反馈和翻译选项

### 单实例架构

应用强制单实例行为——在已运行时启动会将设置窗口置于前台，而不是创建新进程。远程控制标志（`--toggle-transcription` 等）通过启动第二个实例工作，该实例通过 `tauri_plugin_single_instance` 将参数发送到运行中的实例，然后退出。

## 国际化 (i18n)

所有面向用户的字符串必须使用 i18next 翻译。ESLint 强制执行此规则（JSX 中不允许硬编码字符串）。

**添加新文本：**

1. 添加键到 `src/i18n/locales/en/translation.json`
2. 在组件中使用：`const { t } = useTranslation(); t('key.path')`

**文件结构：**

```
src/i18n/
├── index.ts           # i18n 设置
├── languages.ts       # 语言元数据
└── locales/
    ├── en/translation.json  # 英语（源）
    ├── de/, es/, fr/, ja/, ru/, zh/, ...
    └── ...
```

有关翻译贡献指南，请参阅 [CONTRIBUTING_TRANSLATIONS.md](CONTRIBUTING_TRANSLATIONS.md)。

## 代码风格

**Rust：**

- 提交前运行 `cargo fmt` 和 `cargo clippy`
- 显式处理错误（避免在生产环境中使用 unwrap）
- 使用描述性名称，为公共 API 添加文档注释

**TypeScript/React：**

- 严格 TypeScript，避免 `any` 类型
- 使用 hooks 的函数组件
- 使用 Tailwind CSS 进行样式设计
- 路径别名：`@/` → `./src/`

## CLI 参数

聆听支持所有平台的命令行参数，用于与脚本、窗口管理器和自启动配置集成。

**实现：** `cli.rs`（定义）、`main.rs`（解析）、`lib.rs`（应用）、`signal_handle.rs`（共享逻辑）

| 标志                       | 描述                                                       |
| -------------------------- | ---------------------------------------------------------- |
| `--toggle-transcription`   | 在运行实例上切换录制开/关                                  |
| `--toggle-post-process`    | 切换带后处理的录制开/关                                    |
| `--cancel`                 | 取消运行实例上的当前操作                                   |
| `--start-hidden`           | 启动时不显示主窗口（托盘图标可见）                         |
| `--no-tray`                | 启动时不显示系统托盘（关闭窗口退出应用）                   |
| `--debug`                  | 启用调试模式，详细 (Trace) 日志                            |

**关键设计决策：**

- CLI 标志是运行时覆盖——它们不会修改持久化设置
- 远程控制标志通过 `tauri_plugin_single_instance` 工作：第二个实例发送参数，然后退出
- `signal_handle.rs` 中的 `send_transcription_input()` 在信号处理器和 CLI 之间共享

## 调试模式

访问调试功能：`Cmd+Shift+D` (macOS) 或 `Ctrl+Shift+D` (Windows/Linux)

## 平台说明

- **macOS**：Metal 加速，键盘快捷键需要辅助功能权限
- **Windows**：Vulkan 加速，代码签名
- **Linux**：OpenBLAS + Vulkan，有限的 Wayland 支持，overlay 使用 GTK layer shell（使用 `HANDY_NO_GTK_LAYER_SHELL=1` 禁用）

## 故障排除

请参阅 README.md 中的[故障排除](README.md#troubleshooting)部分。

## AI 编码助手的 GitHub 工作流程

**强制性。在本仓库中打开任何 PR、issue 或讨论之前：您必须阅读相关模板文件并严格遵循。** 包括看起来"仪式性"的部分——清单、AI 协助披露、"人工撰写描述"。不接受通用的摘要/测试计划布局。

- **打开 PR：** 阅读 [`.github/PULL_REQUEST_TEMPLATE.md`](.github/PULL_REQUEST_TEMPLATE.md)。那里列出的每个部分都是强制性的。如果某个部分需要人工撰写的段落（例如"人工撰写描述"），请留下明确的 TODO 占位符并请人类贡献者填写——不要代替他们发言。
- **打开 issue：** 阅读 [`.github/ISSUE_TEMPLATE/`](.github/ISSUE_TEMPLATE/)。空白 issue 被禁用；选择正确的模板（`bug_report.md` 用于错误报告）。功能请求不属于 issue——它们应该去 [Discussions](https://github.com/cjpais/Handy/discussions)（参见 `.github/ISSUE_TEMPLATE/config.yml`）。
- **提出功能：** 聆听处于功能冻结状态。新功能需要在 [Discussions](https://github.com/cjpais/Handy/discussions) 中获得社区支持，然后才能打开 PR——参见 PR 模板的"社区反馈"部分。
- **翻译：** 遵循 [CONTRIBUTING_TRANSLATIONS.md](CONTRIBUTING_TRANSLATIONS.md)。
- **完整贡献者工作流程：** [CONTRIBUTING.md](CONTRIBUTING.md)。

**提交：** 使用常规提交前缀（`feat:`、`fix:`、`docs:`、`refactor:`、`chore:`）。消息重点放在*为什么*，而不是*什么*。
