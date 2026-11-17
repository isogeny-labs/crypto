#!/usr/bin/env bash
# E31 end to end (RESEARCH_GLV_INVARIANT_FACTOR_BASES.md §9.22, G2.93-G2.94):
# E25's ten curves, both arms, rerun with the chain solve.  Same command as
# experiments/30_gls_v4/run_all.sh plus --solver; RAYON_NUM_THREADS=4 is the
# recording host's count (§9.19), which fixes the target stream.  Restart-safe.
set -u
cd "$(dirname "$0")/../.."
out=experiments/31_solve_cost
solver=${SOLVER:-chain}
export RAYON_NUM_THREADS=4
for p in 101 149 197 269 397; do
  for s in 1 2; do
    f=$out/e2e_${solver}_p${p}s${s}.json
    [ -s "$f" ] && continue
    ./target/release/examples/quartic_pipeline_gls --sizes "$p" --first-seed "$s" --seeds 1 \
      --rho-runs 16 --max-residuals 5000000 --solver "$solver" --json "$f.part" \
      >>"$out/e2e_${solver}.log" 2>&1 && mv "$f.part" "$f"
  done
done
echo "finished $(date -u +%FT%TZ)" >>"$out/e2e_${solver}.log"
