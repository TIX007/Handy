#!/bin/bash

# RSA 密钥对生成脚本
# 用于生成许可证签名所需的 RSA-2048 密钥对

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
KEYS_DIR="$PROJECT_ROOT/src-tauri/resources/license"

echo "=========================================="
echo "  RSA 密钥对生成工具"
echo "=========================================="
echo ""

# 创建目录
mkdir -p "$KEYS_DIR"

# 检查是否已存在密钥
if [ -f "$KEYS_DIR/private_key.pem" ] || [ -f "$KEYS_DIR/public_key.pem" ]; then
    echo "警告：密钥文件已存在！"
    read -p "是否覆盖？(y/N): " -n 1 -r
    echo ""
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        echo "操作已取消"
        exit 0
    fi
fi

echo "正在生成 RSA-2048 密钥对..."

# 生成私钥
openssl genrsa -out "$KEYS_DIR/private_key.pem" 2048 2>/dev/null
echo "✓ 私钥已生成: $KEYS_DIR/private_key.pem"

# 从私钥导出公钥
openssl rsa -in "$KEYS_DIR/private_key.pem" -pubout -out "$KEYS_DIR/public_key.pem" 2>/dev/null
echo "✓ 公钥已生成: $KEYS_DIR/public_key.pem"

echo ""
echo "=========================================="
echo "  密钥对生成完成！"
echo "=========================================="
echo ""
echo "重要提示："
echo "1. 私钥 (private_key.pem) 用于生成许可证，请妥善保管"
echo "2. 公钥 (public_key.pem) 将嵌入到应用程序中"
echo "3. 请将私钥存储在安全的位置，不要提交到版本控制"
echo ""
echo "私钥位置: $KEYS_DIR/private_key.pem"
echo "公钥位置: $KEYS_DIR/public_key.pem"
echo ""

# 显示密钥信息
echo "密钥信息："
openssl rsa -in "$KEYS_DIR/private_key.pem" -text -noout 2>/dev/null | head -3
echo ""
