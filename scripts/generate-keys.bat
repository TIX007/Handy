@echo off
setlocal

REM RSA 密钥对生成脚本 (Windows)
REM 用于生成许可证签名所需的 RSA-2048 密钥对

set SCRIPT_DIR=%~dp0
set PROJECT_ROOT=%SCRIPT_DIR%..
set KEYS_DIR=%PROJECT_ROOT%\src-tauri\resources\license

echo ==========================================
echo   RSA 密钥对生成工具
echo ==========================================
echo.

REM 创建目录
if not exist "%KEYS_DIR%" mkdir "%KEYS_DIR%"

REM 检查是否已存在密钥
if exist "%KEYS_DIR%\private_key.pem" (
    echo 警告：私钥文件已存在！
    set /p OVERWRITE="是否覆盖？(y/N): "
    if /i not "%OVERWRITE%"=="y" (
        echo 操作已取消
        exit /b 0
    )
)

echo 正在生成 RSA-2048 密钥对...

REM 生成私钥
openssl genrsa -out "%KEYS_DIR%\private_key.pem" 2048 2>nul
if errorlevel 1 (
    echo 错误：无法生成私钥，请确保已安装 OpenSSL
    exit /b 1
)
echo ✓ 私钥已生成: %KEYS_DIR%\private_key.pem

REM 从私钥导出公钥
openssl rsa -in "%KEYS_DIR%\private_key.pem" -pubout -out "%KEYS_DIR%\public_key.pem" 2>nul
if errorlevel 1 (
    echo 错误：无法生成公钥
    exit /b 1
)
echo ✓ 公钥已生成: %KEYS_DIR%\public_key.pem

echo.
echo ==========================================
echo   密钥对生成完成！
echo ==========================================
echo.
echo 重要提示：
echo 1. 私钥 (private_key.pem) 用于生成许可证，请妥善保管
echo 2. 公钥 (public_key.pem) 将嵌入到应用程序中
echo 3. 请将私钥存储在安全的位置，不要提交到版本控制
echo.
echo 私钥位置: %KEYS_DIR%\private_key.pem
echo 公钥位置: %KEYS_DIR%\public_key.pem
echo.

REM 显示密钥信息
echo 密钥信息：
openssl rsa -in "%KEYS_DIR%\private_key.pem" -text -noout 2>nul | findstr /B "Private-Key"
echo.

endlocal
