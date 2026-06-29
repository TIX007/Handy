#!/bin/bash

# Handy 许可证服务器部署脚本

set -e

echo "=========================================="
echo "  Handy 许可证服务器部署"
echo "=========================================="
echo ""

# 检查是否安装了 wrangler
if ! command -v wrangler &> /dev/null; then
    echo "错误：未安装 wrangler CLI"
    echo "请运行: npm install -g wrangler"
    exit 1
fi

# 检查是否已登录
echo "检查 Cloudflare 登录状态..."
if ! wrangler whoami &> /dev/null; then
    echo "请先登录 Cloudflare:"
    wrangler login
fi

echo "✓ 已登录 Cloudflare"
echo ""

# 创建 D1 数据库
echo "正在创建 D1 数据库..."
DB_OUTPUT=$(wrangler d1 create handy-licenses 2>&1) || true
echo "$DB_OUTPUT"

# 提取数据库 ID
DB_ID=$(echo "$DB_OUTPUT" | grep -oP 'database_id = "\K[^"]+' || true)

if [ -z "$DB_ID" ]; then
    echo "警告：无法自动提取数据库 ID"
    echo "请手动更新 wrangler.toml 中的 database_id"
else
    echo "✓ 数据库 ID: $DB_ID"
    # 更新 wrangler.toml
    sed -i "s/YOUR_DATABASE_ID/$DB_ID/g" wrangler.toml
    echo "✓ 已更新 wrangler.toml"
fi

echo ""

# 初始化数据库
echo "正在初始化数据库..."
wrangler d1 execute handy-licenses --file=./schema.sql
echo "✓ 数据库初始化完成"

echo ""

# 设置密钥
echo "正在设置密钥..."
echo "请粘贴 private_key.pem 的内容（按 Ctrl+D 结束）:"
wrangler secret put LICENSE_PRIVATE_KEY

echo ""
echo "请设置管理员 API 密钥:"
wrangler secret put ADMIN_API_KEY

echo ""

# 部署
echo "正在部署..."
wrangler deploy

echo ""
echo "=========================================="
echo "  部署完成！"
echo "=========================================="
echo ""
echo "请记录以下信息："
echo "1. 服务器 URL（查看上面的输出）"
echo "2. 管理员 API 密钥"
echo ""
