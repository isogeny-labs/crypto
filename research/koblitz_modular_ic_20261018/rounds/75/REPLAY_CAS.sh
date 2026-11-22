#!/usr/bin/env bash
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd)
attempt=${1:?fresh attempt};input=${2:?input};marker=${3:?exact marker}
raw="$here/runs/$attempt"
[ ! -e "$raw" ];mkdir -p "$raw"
cp "$here/NATIVE_DRIVER.sh" "$raw/driver.sh"
cp "$here/$input" "$raw/input.sing"
cp "$here/PROTOCOL.md" "$raw/PROTOCOL.md"
cp "$here/protocol.json" "$raw/protocol.json"
cp "$here/donors.json" "$raw/donors.json"
cp "$0" "$raw/launcher.sh"
exec bash "$raw/driver.sh" "$here" "$raw" "$input" "$marker"
