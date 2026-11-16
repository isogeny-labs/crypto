#!/usr/bin/env bash
# Build the fall-degree Lean scoring (kernel `decide` over Data.lean) with the pinned
# toolchain. Usage (repo root): research/fall_degree_bounds_20261008/e12/run_lean.sh OUTDIR
set -uo pipefail
OUT=$1; mkdir -p "$OUT"
L=research/fall_degree_bounds_20261008/lean
TC=$(cat "$L/lean-toolchain")
if ! command -v elan >/dev/null; then
  curl -sSfL https://raw.githubusercontent.com/leanprover/elan/master/elan-init.sh -o "$OUT/elan-init.sh"
  sh "$OUT/elan-init.sh" -y --default-toolchain none >"$OUT/elan.log" 2>&1
  export PATH="$HOME/.elan/bin:$PATH"
fi
echo "$TC" >"$OUT/toolchain"
sha256sum "$L"/FallDegree/*.lean >"$OUT/sources.sha256"
( cd "$L" && lake build ) >"$OUT/lake.log" 2>&1
rc=$?
echo "exit=$rc" >"$OUT/status"
exit $rc
