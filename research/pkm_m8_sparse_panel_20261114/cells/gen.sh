#!/usr/bin/env bash
# Derive the data-seed cells of protocol v2.2 (master seed 20261114). One line per cell: slug t ratio nsat nunsat planted.
# No --min-prime: every rule starts at 2^(t*ratio) >= 2^18, the v2.2 floor.
set -u
B="$(git rev-parse --show-toplevel)/target/release/examples/pkm_m8_panel"
D="$(dirname "$0")"
SEED=20261114
gen() {
  slug=$1 t=$2 r=$3 ns=$4 nu=$5 pl=$6
  out="$D/$slug"; mkdir -p "$out"
  { echo "== $slug $(date -u +%FT%TZ)";
    "$B" cell --t "$t" --ratio "$r" --master-seed $SEED --out "$out" &&
    "$B" targets --cell "$out" --n-sat "$ns" --n-unsat "$nu" --planted "$pl" --master-seed $SEED --m 8 --out "$out" &&
    "$B" export --cell "$out" --targets "$out" --out "$out" --format both &&
    echo "== $slug done $(date -u +%FT%TZ)"; } > "$out/gen.log" 2>&1
  echo "$slug rc=$?"
}
gen t2r9  2 9  4 4 1 &
gen t2r10 2 10 4 4 1 &
gen t2r11 2 11 4 4 1 &
gen t3r8  3 8  4 4 1 &
gen t3r7  3 7  4 4 1 &
gen t3r9  3 9  4 4 1 &
gen t4r7  4 7  1 0 0 &
wait
