#!/usr/bin/env bash
# E30 (RESEARCH_GLV_INVARIANT_FACTOR_BASES.md §9.28): E27's Karatsuba solver-A
# runs (experiments/32_walk_karatsuba/run_e2e.sh) with the trivial-product
# audit counter on (--audit).  RAYON_NUM_THREADS=4 fixes the stream.
set -u
cd "$(dirname "$0")/../.."
out=experiments/35_invariants_lite
export RAYON_NUM_THREADS=4
for p in 101 149 197 269 397; do
  for s in 1 2; do
    f=$out/e2e_audit_p${p}s${s}.json
    [ -s "$f" ] && continue
    ./target/release/examples/quartic_pipeline_gls --sizes "$p" --first-seed "$s" --seeds 1 \
      --rho-runs 16 --max-residuals 5000000 --solver chain --karatsuba --audit --json "$f.part" \
      >>"$out/e2e_audit.log" 2>&1 && mv "$f.part" "$f"
  done
done
echo "finished $(date -u +%FT%TZ)" >>"$out/e2e_audit.log"
