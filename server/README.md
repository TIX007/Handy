# Handy License Server

基于 Cloudflare Workers 的许可证服务器，用于生成和验证 Handy 许可证。

## 功能

- 生成签名的许可证文件
- 验证许可证有效性
- 设备绑定管理
- 使用统计查询

## 技术栈

- **Cloudflare Workers**: 边缘计算，零运维
- **D1 数据库**: SQLite 兼容，存储许可证数据
- **RSA-2048**: 许可证签名验证

## 部署

### 1. 安装 Wrangler CLI

```bash
npm install -g wrangler
```

### 2. 登录 Cloudflare

```bash
wrangler login
```

### 3. 创建 D1 数据库

```bash
wrangler d1 create handy-licenses
```

将返回的数据库 ID 填入 `wrangler.toml`。

### 4. 初始化数据库

```bash
wrangler d1 execute handy-licenses --file=./schema.sql
```

### 5. 配置密钥

```bash
# 上传私钥（用于签名）
wrangler secret put LICENSE_PRIVATE_KEY
# 粘贴 private_key.pem 的内容

# 设置管理员密钥
wrangler secret put ADMIN_API_KEY
# 输入一个安全的 API 密钥
```

### 6. 部署

```bash
wrangler deploy
```

## API 端点

### 生成许可证

```http
POST /api/generate
Authorization: Bearer <ADMIN_API_KEY>
Content-Type: application/json

{
  "key": "XXXX-XXXX-XXXX-XXXX",
  "tier": "professional",
  "name": "用户名称",
  "email": "user@example.com",
  "days": 365,
  "max_devices": 1
}
```

### 验证许可证

```http
POST /api/verify
Content-Type: application/json

{
  "license_data": [base64 encoded license file],
  "fingerprint": "device fingerprint hash"
}
```

### 查询许可证

```http
GET /api/license/:key
Authorization: Bearer <ADMIN_API_KEY>
```

## 本地开发

```bash
# 启动本地开发服务器
wrangler dev

# 运行测试
npm test
```

## 环境变量

| 变量名 | 说明 |
|--------|------|
| `LICENSE_PRIVATE_KEY` | RSA 私钥（PEM 格式） |
| `ADMIN_API_KEY` | 管理员 API 密钥 |
