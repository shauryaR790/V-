@echo off
set "PATH=%USERPROFILE%\.cargo\bin;%PATH%"
set "VPP=%~dp0target\debug\vpp.exe"
if not exist "%VPP%" (
    echo Building vpp...
    pushd "%~dp0"
    cargo build
    popd
)
"%VPP%" %*
