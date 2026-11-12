#!/usr/bin/env bash
# Development check of the `multiples` presentation control against the `tower`
# control, on this host (4 cores, no isolation): Kummer towers at P1 = 15*2^27+1,
# the fall-degree study's seed 0x504B4D54, two random targets per cell
# (`--planted 0 --random 2`, as stage A/C), closure engine (`--engine hky`).
# Not an admitted run: no isolation tier, no bench.record, no timing claim.
set -euo pipefail
cd "$(dirname "$0")/../../.."
BIN=target/release/examples/pkm_tower_pilot
OUT=research/prime_ic_leads_20261110/devcheck
P=2013265921
SEED=1347106132
run() { # name, then flags
  local name=$1; shift
  echo "== $name: $*" | tee -a "$OUT/commands.log"
  CHECKPOINT_SKIP=devcheck "$BIN" --engine hky --kinds kummer --controls tower,multiples \
    --planted 0 --random 2 --ladder-t none --p "$P" --seed "$SEED" "$@" --out "$OUT/$name.jsonl" 2>&1 | tail -3
}
run m2_t2-10  --m 2 --t-min 2 --max-t 10 --hky-cmax 7 --budget 600
run m3_t2-3   --m 3 --t-min 2 --max-t 3 --max-t-m3 3 --hky-cmax 8 --budget 900
run m4_t2-3   --m 4 --t-min 2 --max-t 3 --max-t-m4 3 --hky-cmax 8 --budget 1200
run m5_t2-3   --m 5 --t-min 2 --max-t 3 --max-t-m4 3 --hky-cmax 9 --budget 1800
