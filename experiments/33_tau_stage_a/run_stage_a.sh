#!/usr/bin/env bash
# E28 Stage A (RESEARCH_GLV_INVARIANT_FACTOR_BASES.md §9.26, G2.101-G2.102):
# E19's sizes, seeds 1-2, both folds on one exact-oracle stream, 3 weighted
# D4 solves per curve as the solve-cost control, 16 walks per fold.
# Restart-safe: one curve per invocation.
set -u
cd "$(dirname "$0")/../.."
out=experiments/33_tau_stage_a
export RAYON_NUM_THREADS=4
for p in 53 101 149 197 269; do
  for s in 1 2; do
    f=$out/stage_a_p${p}s${s}.json
    [ -s "$f" ] && continue
    ./target/release/examples/e28_tau --stage a --sizes "$p" --seeds "$s-$s" --control 3 \
      --rho-runs 16 --json "$f.part" >>"$out/stage_a.log" 2>&1 && mv "$f.part" "$f"
  done
done
echo "finished $(date -u +%FT%TZ)" >>"$out/stage_a.log"
