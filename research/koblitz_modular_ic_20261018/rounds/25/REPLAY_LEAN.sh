#!/usr/bin/env bash
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd)
repo=$(git -C "$here" rev-parse --show-toplevel)
build=${1:?fresh persistent output directory}
mkdir -p "$build"
deps=$(awk 'NR!=10' "$here/../22/runs/proof-cache-source-roots.txt" | paste -sd ':' -)
export LEAN_PATH="$build:$deps"
lean_bin=${KOBLITZ_LEAN_BIN:-/home/ec2-user/.elan/toolchains/leanprover--lean4---v4.34.1/bin/lean}
cd "$here/lean"
sha256sum KoblitzDeck25.lean
timeout 1200 "$lean_bin" -j1 -M4096 -o "$build/KoblitzDeck25.olean" KoblitzDeck25.lean
timeout 1200 "$lean_bin" -j1 -M4096 --run "$repo/tools/lean/AuditAxioms.lean" KoblitzDeck25
