@echo off
REM v++ one-click launcher (GitHub Release zip)
cd /d "%~dp0"
set "VPP_HOME=%~dp0"
set "PATH=%VPP_HOME%;%PATH%"

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
