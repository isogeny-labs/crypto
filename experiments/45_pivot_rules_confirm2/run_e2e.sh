#!/usr/bin/env bash
# E38 end to end (RESEARCH_GLV_INVARIANT_FACTOR_BASES.md §9.46, G2.177,
# G2.183–G2.189): every arm on the fresh target stream (salt 0xE38E0001),
# Karatsuba products for every arm.  gls = E25's ten curves through
# quartic_pipeline_gls (arms A and B of each record); e24 = E24's twelve
# curves through quartic_pipeline_v4.  RAYON_NUM_THREADS=4 fixes the target
# stream (§9.19); taskset keeps the run on the given cores.  Restart-safe.
#   usage: run_e2e.sh <pinned bin dir> <gls|e24> <cpus> <solver> [<solver> ...]
set -u
cd "$(dirname "$0")/../.."
out=experiments/45_pivot_rules_confirm2/e2e
mkdir -p "$out"
bindir=$1; set=$2; cpus=$3
shift 3
export RAYON_NUM_THREADS=4
if [ "$set" = gls ]; then
  bin=$bindir/quartic_pipeline_gls; sizes="101 149 197 269 397"
else
  bin=$bindir/quartic_pipeline_v4; sizes="101 149 197 269 397 521"
fi
for solver in "$@"; do
  for p in $sizes; do
    for s in 1 2; do
      f=$out/${set}_${solver}_p${p}s${s}.json
      [ -s "$f" ] && continue
      taskset -c "$cpus" "$bin" --sizes "$p" --first-seed "$s" --seeds 1 \
        --rho-runs 16 --max-residuals 5000000 --solver "$solver" --karatsuba \
        --stream-salt E38E0001 --json "$f.part" \
        >>"$out/${set}_${solver}.log" 2>&1 && mv "$f.part" "$f"
    done
  done
done
echo "finished $set $* $(date -u +%FT%TZ)" >>"$out/e2e.log"
