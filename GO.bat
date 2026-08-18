@echo off
REM v++ one-click launcher (GitHub Release zip)
cd /d "%~dp0"
set "VPP_HOME=%~dp0"
set "PATH=%VPP_HOME%;%PATH%"

REM Strip "downloaded from the internet" flags (helps vpp.exe after first run)
powershell -NoProfile -ExecutionPolicy Bypass -Command "Get-ChildItem -LiteralPath '%~dp0' -Recurse | Unblock-File -ErrorAction SilentlyContinue" >nul 2>&1

if exist "%VPP_HOME%llvm\bin\clang.exe" (
    set "LLVM_SYS_221_PREFIX=%VPP_HOME%llvm"
    set "PATH=%VPP_HOME%llvm\bin;%PATH%"
)

echo.
echo  v++ is ready
echo  ------------
echo  vpp run examples\hello.vpp
echo  vpp doctor
echo.

if exist "examples\hello.vpp" (
    vpp run examples\hello.vpp
) else (
    vpp doctor
)

echo.
pause
