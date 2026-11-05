#!/usr/bin/env bash
# E33 end to end (RESEARCH_GLV_INVARIANT_FACTOR_BASES.md §9.35, G2.124,
# G2.128-G2.132): E27's command (E31's pipeline with --karatsuba) with solver
# B counted (chain-sparsest-counted) and B' (chain-sparsest-inc), and B''
# (chain-sparsest-est) only when Stage A carried it, on E25's ten curves,
# both arms.  RAYON_NUM_THREADS=4 fixes the target stream (§9.19); taskset
# keeps the run on two cores while E32 runs.  Restart-safe.
#   usage: run_e2e.sh <binary> <solver> [<solver> ...]
set -u
cd "$(dirname "$0")/../.."
out=experiments/37_solver_b_pivot_count
bin=$1
shift
export RAYON_NUM_THREADS=4
for solver in "$@"; do
  for p in 101 149 197 269 397; do
    for s in 1 2; do
      f=$out/e2e_${solver}_karatsuba_p${p}s${s}.json
      [ -s "$f" ] && continue
      taskset -c 0,1 "$bin" --sizes "$p" --first-seed "$s" --seeds 1 \
        --rho-runs 16 --max-residuals 5000000 --solver "$solver" --karatsuba --json "$f.part" \
        >>"$out/e2e_${solver}.log" 2>&1 && mv "$f.part" "$f"
    done
  done
done
echo "finished $(date -u +%FT%TZ)" >>"$out/e2e.log"
