#!/usr/bin/env bash
# Validator replay of E31 end to end (GLV note §9.23; experiments/31_solve_cost/run_e2e.sh's
# arguments).  BIN is a fresh build of examples/quartic_pipeline_gls from a tree whose build
# inputs equal the recorded binary commit a8f52422d5.  RAYON_NUM_THREADS=4 fixes the target
# stream (collect_and_solve draws 2 x threads targets per round from the run's seeded RNG).
# Outputs go to replay/e2e/; logs to replay/e2e/*.log.  Restart-safe.
#
#   BIN=/path/to/quartic_pipeline_gls benchmarks/20261024-e31-chain-solve/replay/replay_e2e.sh
set -u
here="$(cd "$(dirname "$0")" && pwd)"
out="$here/e2e"
mkdir -p "$out"
: "${BIN:?set BIN to the freshly built quartic_pipeline_gls}"
export RAYON_NUM_THREADS=4
run() { # solver p seed
  local f="$out/e2e_$1_p$2s$3.json"
  [ -s "$f" ] && return 0
  "$BIN" --sizes "$2" --first-seed "$3" --seeds 1 --rho-runs 16 --max-residuals 5000000 \
    --solver "$1" --json "$f.part" >>"$out/e2e_$1.log" 2>&1 && mv "$f.part" "$f"
}
# 1. the two required samples: solver A at a small p, solver B at a large p
run chain 101 1
run chain-sparsest 397 1
# 2. A/A control: the existing solver through the E31 dispatch must reproduce E25 exactly
run macaulay 101 2
# 3. the remaining E31 records
for p in 101 149 197 269 397; do
  for s in 1 2; do
    run chain "$p" "$s"
    run chain-sparsest "$p" "$s"
  done
done
echo "finished $(date -u +%FT%TZ)" >>"$out/finished.log"
