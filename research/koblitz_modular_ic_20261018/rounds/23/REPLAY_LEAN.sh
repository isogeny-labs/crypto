#!/usr/bin/env bash
# Compiler verification only;all project imports are rebuilt from this source.
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd)
study=$(cd "$here/../.." && pwd)
source_cache=${KOBLITZ_DEP_CACHE:-/home/ec2-user/crypto-modular-ic/research/koblitz_modular_ic_20261018/lean/.lake}
lean_bin=${KOBLITZ_LEAN_BIN:-/home/ec2-user/.elan/toolchains/leanprover--lean4---v4.34.1/bin/lean}
build=${1:?supply a fresh persistent proof output directory}
mkdir -p "$build"
roots="$here/../22/runs/proof-cache-source-roots.txt"
deps=$(awk 'NR!=10' "$roots" | paste -sd ':' -)
export LEAN_PATH="$build:$deps"
cd "$study/lean"
for module in KoblitzRound21Equivariant KoblitzRound22 KoblitzRound22Base KoblitzRound22SS KoblitzRound23Local;do
 printf 'BUILD %s\n' "$module"
 sha256sum "$module.lean"
 timeout 1200 "$lean_bin" -j1 -M4096 -o "$build/$module.olean" "$module.lean"
done
repo=$(git rev-parse --show-toplevel)
timeout 1200 "$lean_bin" -j1 -M4096 --run "$repo/tools/lean/AuditAxioms.lean" KoblitzRound23Local
