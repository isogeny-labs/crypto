#!/usr/bin/env bash
# E43 end to end (RESEARCH_GLV_INVARIANT_FACTOR_BASES.md §9.58, G2.258,
# G2.265-G2.274). gls = E25's ten curves through quartic_pipeline_gls (arms A
# and B of each record); e24 = E24's twelve curves through
# quartic_pipeline_v4. Every run --karatsuba, --rho-runs 16,
# --max-residuals 5000000, RAYON_NUM_THREADS=4 (fixes the target stream).
# Salt E43E0001 writes <set>_<solver>_p*s*.json; any other salt writes
# <set>_<solver>_<salt>_p*s*.json. Restart-safe: a record is written to
# .part and renamed when complete; complete records are skipped.
#   usage: run_e2e.sh <pinned bin dir> <gls|e24> <cpus> <salt> <solver> [<solver> ...]
set -u
cd "$(dirname "$0")/../.."
out=experiments/50_compose_fw/e2e
mkdir -p "$out"
bindir=$1; set=$2; cpus=$3; salt=$4
shift 4
export RAYON_NUM_THREADS=4
if [ "$set" = gls ]; then
  bin=$bindir/quartic_pipeline_gls; sizes="101 149 197 269 397"
else
  bin=$bindir/quartic_pipeline_v4; sizes="101 149 197 269 397 521"
fi
tag=""; [ "$salt" = E43E0001 ] || tag="_$salt"
for solver in "$@"; do
  for p in $sizes; do
    for s in 1 2; do
      f=$out/${set}_${solver}${tag}_p${p}s${s}.json
      [ -s "$f" ] && continue
      taskset -c "$cpus" "$bin" --sizes "$p" --first-seed "$s" --seeds 1 \
        --rho-runs 16 --max-residuals 5000000 --solver "$solver" --karatsuba \
        --stream-salt "$salt" --json "$f.part" \
        >>"$out/${set}_${solver}${tag}.log" 2>&1 && mv "$f.part" "$f"
    done
  done
done
echo "finished $set $salt $* $(date -u +%FT%TZ)" >>"$out/e2e.log"
