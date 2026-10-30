#!/usr/bin/env bash
# Recompile every transitive project import; reuse pinned third-party cache.
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd)
study=$(cd "$here/../.." && pwd)
lean_bin=${KOBLITZ_LEAN_BIN:-/home/ec2-user/.elan/toolchains/leanprover--lean4---v4.34.1/bin/lean}
build=${1:?fresh persistent proof output directory required}
mkdir -p "$build"
deps=$(awk 'NR!=10' "$here/../22/runs/proof-cache-source-roots.txt" | paste -sd ':' -)
export LEAN_PATH="$build:$deps"
cd "$study/lean"
for module in KoblitzRound21Equivariant KoblitzRound22 KoblitzRound22Base KoblitzRound24Geometry;do
 printf 'BUILD %s\n' "$module"
 sha256sum "$module.lean"
 timeout 1200 "$lean_bin" -j1 -M4096 -o "$build/$module.olean" "$module.lean"
done
repo=$(git rev-parse --show-toplevel)
timeout 1200 "$lean_bin" -j1 -M4096 --run "$repo/tools/lean/AuditAxioms.lean" KoblitzRound24Geometry
