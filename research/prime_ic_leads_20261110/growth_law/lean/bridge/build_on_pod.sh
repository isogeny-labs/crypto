#!/usr/bin/env bash
# Run on a pod with >= 16 GB RAM and >= 20 GB disk (not on the dev host: Mathlib does not fit).
# Usage: tar xzf bridge.tgz -C /work && bash /work/bridge/build_on_pod.sh   (writes /work/out/)
set -uo pipefail
cd "$(dirname "$0")"
OUT=${OUT:-/work/out}; mkdir -p "$OUT"
command -v elan >/dev/null || curl https://raw.githubusercontent.com/leanprover/elan/master/elan-init.sh -sSf | sh -s -- -y --default-toolchain none
export PATH="$HOME/.elan/bin:$PATH"
{ lean --version; lake --version; } > "$OUT/versions.log" 2>&1
lake update > "$OUT/update.log" 2>&1; echo "update rc=$?" >> "$OUT/update.log"
lake exe cache get > "$OUT/cache.log" 2>&1; echo "cache rc=$?" >> "$OUT/cache.log"
git -C .lake/packages/mathlib rev-parse HEAD > "$OUT/mathlib_rev.txt" 2>&1
lake build > "$OUT/build.log" 2>&1; echo "build rc=$?" | tee -a "$OUT/build.log"
lake env lean Bridge.lean > "$OUT/axioms.log" 2>&1; echo "lean rc=$?" >> "$OUT/axioms.log"
grep -n "sorry" Bridge.lean GrowthLaw.lean > "$OUT/sorry_grep.txt"; echo "grep rc=$? (1 = no sorry)" >> "$OUT/sorry_grep.txt"
sha256sum Bridge.lean GrowthLaw.lean lakefile.toml lean-toolchain lake-manifest.json > "$OUT/SOURCE.sha256"
