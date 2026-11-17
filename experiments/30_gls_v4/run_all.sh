#!/usr/bin/env bash
# E25 (RESEARCH_GLV_INVARIANT_FACTOR_BASES.md §9.15): one curve per invocation
# (both arms, A then B), in registered order; a curve whose JSON exists is
# skipped (restart-safe).
set -u
cd "$(dirname "$0")/../.."
out=experiments/30_gls_v4
for p in 101 149 197 269 397; do
  for s in 1 2; do
    f=$out/p${p}s${s}.json
    [ -s "$f" ] && continue
    ./target/release/examples/quartic_pipeline_gls --sizes "$p" --first-seed "$s" --seeds 1 \
      --rho-runs 16 --max-residuals 5000000 --json "$f.part" >>"$out/run.log" 2>&1 && mv "$f.part" "$f"
  done
done
echo "finished $(date -u +%FT%TZ)" >>"$out/run.log"
