#!/usr/bin/env bash
# E27 IC side (RESEARCH_GLV_INVARIANT_FACTOR_BASES.md §9.25, G2.98): E31's
# solver-A runs (experiments/31_solve_cost/run_e2e.sh, --solver chain) on
# E25's ten curves, both arms, with every F_{p^4} product priced by the
# Karatsuba tower (--karatsuba).  RAYON_NUM_THREADS=4 fixes the target
# stream (§9.19).  Restart-safe.
set -u
cd "$(dirname "$0")/../.."
out=experiments/32_walk_karatsuba
export RAYON_NUM_THREADS=4
for p in 101 149 197 269 397; do
  for s in 1 2; do
    f=$out/e2e_chain_karatsuba_p${p}s${s}.json
    [ -s "$f" ] && continue
    ./target/release/examples/quartic_pipeline_gls --sizes "$p" --first-seed "$s" --seeds 1 \
      --rho-runs 16 --max-residuals 5000000 --solver chain --karatsuba --json "$f.part" \
      >>"$out/e2e_chain_karatsuba.log" 2>&1 && mv "$f.part" "$f"
  done
done
echo "finished $(date -u +%FT%TZ)" >>"$out/e2e_chain_karatsuba.log"
