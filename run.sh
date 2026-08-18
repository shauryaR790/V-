#!/usr/bin/env sh
# v++ launcher for GitHub Release bundles (Linux/macOS)
ROOT="$(cd "$(dirname "$0")" && pwd)"
export VPP_HOME="$ROOT"
export PATH="$ROOT:$PATH"

echo ""
echo " v++ is ready"
echo " ------------"
echo " vpp run examples/hello.vpp"
echo " vpp doctor"
echo ""

if [ -f "$ROOT/examples/hello.vpp" ]; then
  vpp run "$ROOT/examples/hello.vpp"
else
  vpp doctor
fi
