#!/usr/bin/env bash
# E41 validator replay (run from the repository root).
#   BIN=<rebuilt e41_run>  DRV=<rebuilt v41_replay>  research/e41_twist24_shortfall_20261102/replay/replay.sh <part>
# parts: bin-identity, bin-control, bin-kar, bin-aa, bin-school, drv-<run> (run = s0, aa, kar, school, e34)
# Every process is single-threaded (RAYON_NUM_THREADS=1) and niced; outputs go under replay/ only.
set -euo pipefail
R=research/e41_twist24_shortfall_20261102/replay
mkdir -p "$R/bin"
export RAYON_NUM_THREADS=1
case "$1" in
  bin-identity) nice -n 5 "$BIN" --stage identity --json "$R/bin/identity.json" > "$R/bin/identity.log" ;;
  bin-control)  nice -n 5 "$BIN" --stage control  --json "$R/bin/control.json"  > "$R/bin/control.log" ;;
  bin-kar)      nice -n 5 "$BIN" --stage main --stream main --arith karatsuba --walks 32 --json "$R/bin/main_karatsuba.json" > "$R/bin/main_karatsuba.log" ;;
  bin-aa)       nice -n 5 "$BIN" --stage main --stream aa --arith school --walks 128 --json "$R/bin/aa_school.json" > "$R/bin/aa_school.log" ;;
  bin-school)   nice -n 5 "$BIN" --stage main --stream main --arith school --walks 128 --json "$R/bin/main_school.json" > "$R/bin/main_school.log" ;;
  drv-*)        run="${1#drv-}"; nice -n 5 "$DRV" "$R/sel_${run}.txt" > "$R/out_${run}.tsv" ;;
  *) echo "unknown part $1" >&2; exit 2 ;;
esac
