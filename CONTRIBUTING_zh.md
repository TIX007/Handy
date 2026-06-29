# 为 Handy 做贡献

感谢你有兴趣为 Handy 做贡献！本指南将帮助你开始为这个开源语音转文字应用做贡献。

## ⚠️ 功能冻结

**Handy 目前处于功能冻结状态。** 如果你提交的 PR 是社区未要求的新功能，它将被拒绝。如果社区已经要求过，或者你已经明确获得了支持，它可能仍会被考虑。

**Bug 修复是首要任务。** 有 60 多个 issues 需要修复。请将你的贡献集中在修复 bug 和提高稳定性上。

## 📖 理念

Handy 旨在成为最容易被 fork 的语音转文字应用。目标是创建一个有用的工具，同时也是他人构建的基础——一个模式良好、简单且服务社区的代码库。我们优先考虑：

- **简洁**：清晰、可维护的代码优于巧妙的解决方案
- **可扩展性**：让他人容易 fork 和自定义
- **隐私**：保持一切本地和离线
- **可访问性**：免费工具应该属于每个人

## 🚀 入门

### 前提条件

在开始之前，确保你已安装以下内容：

- [Rust](https://rustup.rs/)（最新稳定版）
- [Bun](https://bun.sh/) 包管理器
- 平台特定的构建工具（参见 [BUILD.md](BUILD.md)）

### 搭建开发环境

1. **在 GitHub 上 Fork 仓库**

2. **克隆你的 fork**：

   ```bash
   git clone git@github.com:YOUR_USERNAME/Handy.git
   cd Handy
   ```

3. **添加上游远程仓库**：

   ```bash
   git remote add upstream git@github.com:cjpais/Handy.git
   ```

4. **安装依赖**：

   ```bash
   bun install
   ```

5. **下载所需模型**：

   ```bash
   mkdir -p src-tauri/resources/models
   curl -o src-tauri/resources/models/silero_vad_v4.onnx https://blob.handy.computer/silero_vad_v4.onnx
   ```

6. **以开发模式运行**：
   ```bash
   bun run tauri dev
   # 如果在 macOS 上遇到 cmake 错误：
   CMAKE_POLICY_VERSION_MINIMUM=3.5 bun run tauri dev
   ```

详细的平台特定设置说明请参阅 [BUILD.md](BUILD.md)。

### 理解代码库

Handy 遵循清晰的架构模式：

**后端（Rust - `src-tauri/src/`）：**

- `lib.rs` - 主应用入口点，包含 Tauri 设置
- `managers/` - 核心业务逻辑（音频、模型、转录）
- `audio_toolkit/` - 底层音频处理（录音、VAD）
- `commands/` - Tauri 命令处理器，用于前端通信
- `shortcut.rs` - 全局键盘快捷键处理
- `settings.rs` - 应用设置管理

**前端（React/TypeScript - `src/`）：**

- `App.tsx` - 主应用组件
- `components/` - React UI 组件
- `hooks/` - 可复用的 React hooks
- `lib/types.ts` - 共享的 TypeScript 类型

更多详情请参阅 [README.md](README.md) 或 [AGENTS.md](AGENTS.md) 中的架构部分。

## 🐛 报告 Bug

### 提交 Bug 报告之前

1. **搜索现有 issues**：[github.com/cjpais/Handy/issues](https://github.com/cjpais/Handy/issues)
2. **查看讨论**：[github.com/cjpais/Handy/discussions](https://github.com/cjpais/Handy/discussions)
3. **尝试最新版本**，看看问题是否已修复
4. **启用调试模式**（`Cmd/Ctrl+Shift+D`）以收集诊断信息

### 提交 Bug 报告

创建 bug 报告时，请包含：

**系统信息：**

- 应用版本（在设置或关于部分找到）
- 操作系统（例如 macOS 14.1、Windows 11、Ubuntu 22.04）
- CPU（例如 Apple M2、Intel i7-12700K、AMD Ryzen 7 5800X）
- GPU（例如 Apple M2 GPU、NVIDIA RTX 4080、Intel UHD Graphics）

**Bug 详情：**

- 清晰描述 bug
- 重现步骤
- 预期行为
- 实际行为
- 如适用，提供截图或日志
- 如相关，提供调试模式的信息

创建 issue 时请使用 [Bug 报告模板](.github/ISSUE_TEMPLATE/bug_report.md)。

## 💡 建议功能

我们使用 GitHub Discussions 进行功能请求，而不是 issues。这使 issues 专注于 bug 和可操作的任务，同时允许更开放的功能讨论。

### 建议功能之前

1. **搜索现有讨论**：[github.com/cjpais/Handy/discussions](https://github.com/cjpais/Handy/discussions)
2. **查看常见功能请求**：
   - [后处理 / 编辑转录](https://github.com/cjpais/Handy/discussions/168)
   - [键盘快捷键 / 热键](https://github.com/cjpais/Handy/discussions/211)

### 提交功能请求

1. 前往 [Discussions](https://github.com/cjpais/Handy/discussions)
2. 点击"新建讨论"
3. 选择适当的类别（Ideas、Feature Requests 等）
4. 描述你的功能想法，包括：
   - 你试图解决的问题
   - 你提出的解决方案
   - 你考虑过的任何替代方案
   - 它如何符合 Handy 的理念

## 🔧 代码贡献

### 开始之前

**这很关键：** 在编写任何代码之前，请执行以下操作：

1. **搜索现有 issues 和 PRs** - 同时查看打开和关闭的 issues 和 pull requests。可能已经有人处理过这个问题，或者有关闭的理由。
   - [打开的 issues](https://github.com/cjpais/Handy/issues)
   - [关闭的 issues](https://github.com/cjpais/Handy/issues?q=is%3Aissue+is%3Aclosed)
   - [打开的 PRs](https://github.com/cjpais/Handy/pulls)
   - [关闭的 PRs](https://github.com/cjpais/Handy/pulls?q=is%3Apr+is%3Aclosed)

2. **如果之前被关闭过** - 如果你想重新审视已关闭的 issue 或 PR，你需要：
   - 提供强有力的论据说明为什么应该重新考虑
   - 首先通过 [Discussions](https://github.com/cjpais/Handy/discussions) 收集社区反馈
   - 在你的 PR 中链接到该讨论

3. **获取功能的社区反馈** - 有社区支持的 PR **更有可能被合并**。开始讨论，获取反馈，并在你的 PR 中链接到它。这有助于确保 Handy 保持专注和有用，不会变得臃肿。

社区反馈对于保持 Handy 对每个人都是最好的至关重要。它有助于优先处理最重要的事情，防止功能蔓延。

### 开发工作流程

1. **创建功能分支**：

   ```bash
   git checkout -b feature/your-feature-name
   # 或者
   git checkout -b fix/your-bug-fix
   ```

2. **进行更改**：
   - 编写清晰、可维护的代码
   - 遵循现有的代码风格和模式
   - 为复杂逻辑添加注释
   - 保持提交专注和原子化

3. **彻底测试**：
   - 在目标平台上测试
   - 验证现有功能仍然正常工作
   - 测试边界情况和错误条件
   - 使用调试模式验证音频/转录行为

4. **提交更改**：

   ```bash
   git add .
   git commit -m "feat: 添加你的功能描述"
   # 或者
   git commit -m "fix: 描述 bug 修复"
   ```

   使用约定式提交消息：
   - `feat:` 用于新功能
   - `fix:` 用于 bug 修复
   - `docs:` 用于文档更改
   - `refactor:` 用于代码重构
   - `test:` 用于测试添加/更改
   - `chore:` 用于维护任务

5. **保持你的 fork 更新**：

   ```bash
   git fetch upstream
   git rebase upstream/main
   ```

6. **推送到你的 fork**：

   ```bash
   git push origin feature/your-feature-name
   ```

7. **创建 Pull Request**：
   - 前往 [Handy 仓库](https://github.com/cjpais/Handy)
   - 点击"新建 Pull Request"
   - 选择你的 fork 和分支
   - 完整填写 PR 模板，包括：
     - 清晰描述更改
     - 链接到相关 issues 或讨论
     - **社区反馈**（对功能尤其重要）
     - 你如何测试更改
     - 如适用，提供截图/视频
     - 破坏性更改（如有）

   **记住：** 有社区支持的 PR 会被优先处理。如果你还没有开始，请开始一个[讨论](https://github.com/cjpais/Handy/discussions)以在你的 PR 之前或同时收集反馈。收集反馈不是强制要求，但它肯定会帮助你的 PR 更快被合并。

### AI 辅助披露

**欢迎使用 AI 辅助的 PR！** 使用任何能帮助你贡献的工具，只需坦诚说明。

在你的 PR 描述中，请包含：

- 是否使用了 AI（是/否）
- 使用了哪些工具（例如 "Claude Code"、"GitHub Copilot"、"ChatGPT"）
- 使用程度如何（例如 "生成了样板代码"、"帮助调试"、"编写了大部分代码"）

### 代码风格指南

**Rust：**

- 遵循标准 Rust 格式（`cargo fmt`）
- 运行 `cargo clippy` 并处理警告
- 使用描述性的变量和函数名
- 为公共 API 添加文档注释
- 显式处理错误（避免在生产代码中使用 unwrap）

**TypeScript/React：**

- 严格使用 TypeScript，避免 `any` 类型
- 遵循 React hooks 最佳实践
- 使用函数组件
- 保持组件小而专注
- 使用 Tailwind CSS 进行样式设置

**通用：**

- 编写自文档化的代码
- 为非显而易见的逻辑添加注释
- 保持函数小且单一职责
- 优先考虑可读性而非巧妙性

### 测试你的更改

**手动测试：**

- 以开发模式运行应用：`bun run tauri dev`
- 在启用调试模式的情况下测试你的更改
- 如果可能，在多个平台上验证
- 使用不同的音频设备测试
- 尝试各种转录场景

**生产构建：**

```bash
bun run tauri build
```

测试生产构建以确保它按预期工作。

## 📝 文档贡献

文档改进非常有价值！你可以通过以下方式贡献：

- 改进 README.md、BUILD.md 或本 CONTRIBUTING.md
- 添加代码注释和文档注释
- 创建教程或指南
- 改进错误消息
- 更新项目网站内容

## 🤝 社区准则

- **尊重和包容** - 我们欢迎所有技能水平的贡献者
- **耐心** - 这由一个小团队维护，回复可能需要时间
- **建设性** - 专注于解决方案和改进
- **协作性** - 帮助他人并分享知识
- **先搜索** - 在创建新的之前检查现有 issues/讨论

## 🎯 好的首次 Issues

如果你是项目新手，请查找标记为 `good first issue` 或 `help wanted` 的 issues。这些通常是：

- 定义明确且范围适当
- 适合学习代码库
- 有导师支持

## 📞 获取帮助

- **Discord**：加入我们的 [Discord 社区](https://discord.com/invite/WVBeWsNXK4)
- **Discussions**：在 [GitHub Discussions](https://github.com/cjpais/Handy/discussions) 中提问
- **邮件**：联系 [contact@handy.computer](mailto:contact@handy.computer)

## 📜 许可证

为 Handy 做贡献，即表示你同意你的贡献将在 MIT 许可证下授权。详见 [LICENSE](LICENSE)。

---

**感谢你为 Handy 做贡献！** 你的努力帮助使语音转文字技术对每个人来说更可访问、更私密、更可扩展。
