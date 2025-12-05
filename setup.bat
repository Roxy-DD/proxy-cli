@echo off
if exist "%~dp0proxy.exe" (
    echo Found proxy.exe, skipping build...
    goto install
)

echo Building Proxy CLI...
cargo build
if %errorlevel% neq 0 (
    echo Build failed!
    pause
    exit /b %errorlevel%
)

:install
echo Installing Proxy CLI...
powershell -NoProfile -ExecutionPolicy Bypass -File "%~dp0install.ps1"
pause
