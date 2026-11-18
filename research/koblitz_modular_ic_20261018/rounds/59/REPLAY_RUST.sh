#!/usr/bin/env bash
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd)
attempt=${1:?fresh attempt};raw="$here/runs/$attempt"
[ ! -e "$raw" ];mkdir -p "$raw"
for name in RUST_DRIVER.sh REPLAY_RUST.sh COHORT59.rs PROTOCOL.md protocol.json RUST-PROTOCOL.md donors.json;do cp "$here/$name" "$raw/$name";done
cp "$here/runs/data59-02/data59.txt" "$raw/data59.txt"
exec bash "$raw/RUST_DRIVER.sh" "$here" "$raw"
