#!/usr/bin/env bash
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd)
attempt=${1:?fresh attempt};raw="$here/runs/$attempt"
[ ! -e "$raw" ];mkdir -p "$raw"
cp "$here/LEAN_DRIVER.sh" "$raw/driver.sh"
cp "$here/lean/Restriction57.lean" "$raw/Restriction57.lean"
cp "$here/LEAN-PROTOCOL.md" "$raw/LEAN-PROTOCOL.md"
cp "$0" "$raw/launcher.sh"
exec bash "$raw/driver.sh" "$here" "$raw" "$attempt"
