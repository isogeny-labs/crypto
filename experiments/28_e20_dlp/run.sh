#!/bin/bash
# E20 (RESEARCH_GLV_INVARIANT_FACTOR_BASES.md §9.7): double large primes on the
# tau_T-folded D4 base, charged at E19's weighted C (1.40e9 per trial).
# p = 2053 (registered) dropped: the three-point oracle table does not fit.
export RAYON_NUM_THREADS=1 OMP_NUM_THREADS=1
cd "$(dirname "$0")/../.."
B=./target/release/examples/ic_next10_01_fp4_ladder
O=experiments/28_e20_dlp
for p in 269 521 1033; do
  for s in 1 2; do
    grep -q "\"p\":$p,.*\"seed\":$s," $O/ladder.jsonl 2>/dev/null && continue
    prlimit --as=4000000000 -- nice $B ladder --p $p --seed $s --rho-runs 16 \
      --arms d4dlp,rho --c4-d4 1.40e9 --json $O/ladder.jsonl > /dev/null 2>> $O/stderr.log \
      || echo "FAIL p=$p s=$s rc=$?" >> $O/stderr.log
    echo "done p=$p s=$s $(date -u +%FT%TZ)" >> $O/progress.log
  done
done
