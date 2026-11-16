#!/usr/bin/env bash
# Red-team controls for research/jv_cover_crossover_20261101 (<= 2 threads throughout).
set -u
B=/tmp/claude-0/-home-user/e05ef163-36d2-5181-b9b7-869571ded707/scratchpad/redteam/jv_cover_bin
OUT=/home/user/crypto/research/jv_cover_crossover_20261101/review/red_team/controls
mkdir -p $OUT; cd /home/user/crypto
log=$OUT/walls.txt
{ echo "host nproc $(nproc)"; sha256sum $B; echo "T0 $(date -u +%FT%TZ)"; } >> $log
T0=$(date +%s)
run() { local label=$1; shift; local a=$(date +%s); "$@" > $OUT/$label.log 2>&1; echo "$label exit $? wall_s $(( $(date +%s)-a )) at_s $(( $(date +%s)-T0 ))" >> $log; }
# 1. F4 multiplication counter under 1 vs 2 rayon threads (same instance): global batch difference vs per-test sum
run f4check_t1 env RAYON_NUM_THREADS=1 $B --exp dlp --sizes 53 --first-seed 2 --seeds 1 --rho-runs 0 --rho-ref 1.3 --json $OUT/f4check_t1.json
run f4check_t2 env RAYON_NUM_THREADS=2 $B --exp dlp --sizes 53 --first-seed 2 --seeds 1 --rho-runs 0 --rho-ref 1.3 --json $OUT/f4check_t2.json
# 2. rhodp null at p = 251 (the calibration instrument, dp_bits 12), 2 walkers, 48 runs per seed
for s in 1 2; do run rhodp_null_p251_s$s $B --exp rhodp --sizes 251 --first-seed $s --seeds 1 --rho-runs 48 --dp-bits 12 --threads 2 --json $OUT/rhodp_null_p251_s$s.json; done
# 2b. single walker, 24 runs
run rhodp_null_p251_s1_t1 $B --exp rhodp --sizes 251 --first-seed 1 --seeds 1 --rho-runs 24 --dp-bits 12 --threads 1 --json $OUT/rhodp_null_p251_s1_t1.json
# 3. strong rho at p = 251: negation arm (study settings) and the no-negation, no-batch arm; seeds 1-2 in parallel
for r in $(seq 0 31); do run rhostrong_neg_p251_run$r $B --exp rhostrong --sizes 251 --first-seed 1 --seeds 2 --neg 1 --batch 16 --r 256 --dp-bits 12 --run $r --json $OUT/rhostrong_neg_p251_run$r.json; done
for r in $(seq 0 23); do run rhostrong_noneg_p251_run$r $B --exp rhostrong --sizes 251 --first-seed 1 --seeds 2 --neg 0 --batch 1 --r 32 --dp-bits 12 --run $r --json $OUT/rhostrong_noneg_p251_run$r.json; done
# 4. reproduction control with the predecessor's linear stage default (JV_LIN=sparse), p = 1009 seed 1
run repro_sparse_p1009_s1 env JV_LIN=sparse RAYON_NUM_THREADS=2 $B --exp dlp --sizes 1009 --first-seed 1 --seeds 1 --rho-runs 0 --rho-ref 1.3 --check-every 64 --json $OUT/repro_sparse_p1009_s1.json
echo "ALL DONE total_s $(( $(date +%s)-T0 ))" >> $log
