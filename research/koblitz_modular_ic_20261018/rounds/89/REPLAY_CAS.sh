#!/usr/bin/env bash
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd)
attempt=${1:?fresh attempt};input=${2:?input};marker=${3:?exact marker}
raw="$here/runs/$attempt"
[ ! -e "$raw" ];mkdir -p "$raw"
cp "$here/NATIVE_DRIVER.sh" "$raw/driver.sh"
cp "$here/$input" "$raw/input.sing"
cp "$here/../50/runs/rr-pid50-01/rr-pid50.sing" "$raw/rr-pid50.sing"
cp "$here/../50/source-data/relative-infinity50.sing" "$raw/relative-infinity50.sing"
cp "$here/PROTOCOL.md" "$raw/PROTOCOL.md"
cp "$here/protocol.json" "$raw/protocol.json"
cp "$here/donors.json" "$raw/donors.json"
cp "$0" "$raw/launcher.sh"
exec bash "$raw/driver.sh" "$here" "$raw" "$input" "$marker"
