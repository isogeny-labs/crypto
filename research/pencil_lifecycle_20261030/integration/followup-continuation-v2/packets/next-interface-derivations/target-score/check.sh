#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")"
cargo build --offline --locked --target-dir "${1:?fresh build directory required}"
"$1/debug/pencil-target-score-controls"
