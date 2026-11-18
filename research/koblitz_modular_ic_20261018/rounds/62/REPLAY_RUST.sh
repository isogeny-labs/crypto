#!/usr/bin/env bash
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd)
attempt=${1:?fresh attempt};raw="$here/runs/$attempt"
[ ! -e "$raw" ];mkdir -p "$raw"
cp "$here/RUST_DRIVER.sh" "$raw/driver.sh"
cp "$here/CHECK62.rs" "$raw/input.rs"
cp "$here/PROTOCOL.md" "$raw/PROTOCOL.md"
cp "$here/protocol.json" "$raw/protocol.json"
cp "$here/RUST-PROTOCOL.md" "$raw/RUST-PROTOCOL.md"
cp "$here/donors.json" "$raw/donors.json"
cp "$0" "$raw/launcher.sh"
exec bash "$raw/driver.sh" "$here" "$raw"
