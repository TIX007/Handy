# 为 Handy 贡献翻译

感谢你帮助翻译 Handy！本指南说明如何添加或改进翻译。

## 快速开始

1. Fork 仓库
2. 将英文翻译文件复制到你的语言文件夹
3. 翻译值（不是键！）
4. 提交 pull request

## 文件结构

翻译文件位于：

```
src/i18n/locales/
├── en/
│   └── translation.json    # 英文（源）
├── vi/
│   └── translation.json    # 越南语
├── fr/
│   └── translation.json    # 法语
└── [your-language]/
    └── translation.json    # 你的贡献！
```

## 添加新语言

### 第 1 步：创建语言文件夹

使用 [ISO 639-1 语言代码](https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes) 创建新文件夹：

```bash
mkdir src/i18n/locales/[language-code]
```

示例：

- `de` 代表德语
- `es` 代表西班牙语
- `ja` 代表日语
- `zh` 代表中文
- `ko` 代表韩语
- `pt` 代表葡萄牙语

### 第 2 步：复制英文文件

```bash
cp src/i18n/locales/en/translation.json src/i18n/locales/[language-code]/translation.json
```

### 第 3 步：翻译值

打开文件，只翻译**值**（右侧），而不是键（左侧）：

```json
{
  "sidebar": {
    "general": "General",      // ← 翻译这个值
    "advanced": "Advanced",    // ← 翻译这个值
    ...
  }
}
```

**重要：**

- 保持所有键完全相同
- 保留文本中的任何 `{{variables}}`（例如 `{{error}}`、`{{model}}`）
- 保持 JSON 结构和格式完整

### 第 4 步：注册你的语言

编辑 `src/i18n/languages.ts` 并添加你的语言元数据：

```typescript
export const LANGUAGE_METADATA: Record<
  string,
  { name: string; nativeName: string }
> = {
  en: { name: "English", nativeName: "English" },
  es: { name: "Spanish", nativeName: "Español" },
  fr: { name: "French", nativeName: "Français" },
  vi: { name: "Vietnamese", nativeName: "Tiếng Việt" },
  de: { name: "German", nativeName: "Deutsch" }, // ← 添加你的语言
};
```

### 第 5 步：测试你的翻译

1. 运行应用：`bun run tauri dev`
2. 前往设置 → 通用 → 应用语言
3. 选择你的语言
4. 验证所有文本显示正确

### 第 6 步：提交 Pull Request

1. 提交你的更改
2. 推送到你的 fork
3. 打开一个 pull request，包含：
   - 标题中的语言名称（例如 "Add German translation"）
   - 关于翻译的任何说明

## 改进现有翻译

发现拼写错误或更好的翻译？

1. 编辑相关的 `translation.json` 文件
2. 提交一个 PR，简要描述更改

## 翻译指南

### 应该做：

- 使用自然、地道的语言
- 保持翻译简洁（UI 空间有限）
- 匹配英文文本的语气（友好、清晰）
- 在适当时保留技术术语（例如 "API"、"GPU"）

### 不应该做：

- 翻译品牌名称（Handy、Whisper.cpp、OpenAI）
- 更改或删除 `{{variables}}`
- 修改 JSON 键
- 添加额外的空格或格式

### 处理变量

某些字符串包含变量，如 `{{error}}` 或 `{{model}}`。请原样保留这些变量：

```json
// 英文
"downloadModel": "Failed to download model: {{error}}"

// 法语（正确）
"downloadModel": "Échec du téléchargement du modèle : {{error}}"

// 法语（错误 - 不要翻译变量！）
"downloadModel": "Échec du téléchargement du modèle : {{erreur}}"
```

### 处理复数

某些语言有复杂的复数规则。目前，请使用适用于所有情况的一般形式。我们将来可能会添加适当的复数支持。

## 有问题？

- 在 GitHub 上提交 issue
- 在现有的翻译 PR 中加入讨论

## 当前支持的语言

| 语言 | 代码 | 状态 |
|------|------|------|
| 英语 | `en` | 完成（源） |
| 中文 | `zh` | 完成 |
| 法语 | `fr` | 完成 |
| 德语 | `de` | 完成 |
| 日语 | `ja` | 完成 |
| 西班牙语 | `es` | 完成 |
| 越南语 | `vi` | 完成 |

## 请求的语言

我们希望得到以下语言的帮助：

- 韩语（`ko`）
- 葡萄牙语（`pt`）
- 以及其他更多语言！

---

感谢你让 Handy 对全世界更多人可访问！
