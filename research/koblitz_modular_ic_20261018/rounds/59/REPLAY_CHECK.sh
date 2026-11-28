#!/usr/bin/env bash
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd)
attempt=${1:?fresh attempt};raw="$here/runs/$attempt";[ ! -e "$raw" ];mkdir -p "$raw"
for name in CHECK_DRIVER.sh REPLAY_CHECK.sh COHORT59.rs PROTOCOL.md protocol.json RUST-PROTOCOL.md donors.json;do cp "$here/$name" "$raw/$name";done
cp "$here/runs/data59-02/data59.txt" "$raw/data59.txt"
cp "$here/runs/cohort59-01/certificate59.txt" "$raw/certificate59.txt"
cp "$here/runs/cohort59-01/cohort59" "$raw/cohort59"
exec bash "$raw/CHECK_DRIVER.sh" "$here" "$raw"
