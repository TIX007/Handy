@echo off
setlocal

REM 许可证生成脚本 (Windows)
REM 用于生成签名的许可证文件

set SCRIPT_DIR=%~dp0
set PROJECT_ROOT=%SCRIPT_DIR%..

echo ==========================================
echo   许可证生成工具
echo ==========================================
echo.

REM 检查参数
if "%~4"=="" (
    echo 用法: %0 ^<密钥^> ^<等级^> ^<用户名^> ^<邮箱^> [选项]
    echo.
    echo 参数:
    echo   密钥      许可证密钥 (例如: XXXX-XXXX-XXXX-XXXX)
    echo   等级      basic, professional, enterprise
    echo   用户名    用户名称
    echo   邮箱      用户邮箱
    echo.
    echo 选项:
    echo   --days N           有效期天数 (默认: 365, 0=永不过期)
    echo   --max-devices N    最大设备数 (默认: 1)
    echo   --output FILE      输出文件 (默认: license.lic)
    echo.
    echo 示例:
    echo   %0 XXXX-XXXX-XXXX-XXXX professional "张三" zhangsan@example.com
    echo   %0 XXXX-XXXX-XXXX-XXXX enterprise "李四" lisi@example.com --days 730 --max-devices 3
    exit /b 1
)

set KEY=%~1
set TIER=%~2
set NAME=%~3
set EMAIL=%~4
shift /1
shift /1
shift /1
shift /1

REM 构建生成器
echo 正在构建许可证生成工具...
cd /d "%PROJECT_ROOT%\tools\license-generator"
cargo build --release 2>nul
if errorlevel 1 (
    echo 错误：无法构建许可证生成工具
    exit /b 1
)

REM 生成许可证
echo 正在生成许可证...
.\target\release\license-generator.exe ^
    --key "%KEY%" ^
    --tier "%TIER%" ^
    --name "%NAME%" ^
    --email "%EMAIL%" ^
    --private-key "%PROJECT_ROOT%\src-tauri\resources\license\private_key.pem" ^
    --output "%PROJECT_ROOT%\license.lic" ^
    %*

if errorlevel 1 (
    echo 错误：许可证生成失败
    exit /b 1
)

echo.
echo ✓ 许可证已生成: %PROJECT_ROOT%\license.lic
echo.

endlocal
