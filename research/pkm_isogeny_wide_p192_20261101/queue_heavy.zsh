#!/bin/zsh
# Sequential queue for the remaining heavy cells (one at a time; the
# parallel runs on 2026-11-02 evening died together - likely memory).
set -u
cd /Users/adamburan/ic-work/crypto
B=/Volumes/SSD990-2/llm/build/crypto-target-tests/release/examples/pkm_tower_pilot
OUT=research/pkm_isogeny_wide_p192_20261101
REM=research/pkm_kummer_remeasure_t1_14_20261101

# 1. wait for the m=4 run (pid $1) to finish
while kill -0 "$1" 2>/dev/null; do sleep 60; done

# 2. m=2 t=12 (N=24) at P-192 - the t=11 row landed, t=12 died mid-run
$B --builder wide --kinds isogeny --m 2 --controls tower --t-min 12 \
  --max-t 12 --planted 1 --random 0 --ladder-t none --budget 86400 \
  --p P-192 --engine generic --verify \
  --out $OUT/rows_m2_isogeny_t12.jsonl >> $OUT/run_t12.log 2>&1

# 3. m=3 t=6 (N=18) at P-192
$B --builder wide --kinds isogeny --m 3 --controls tower --t-min 6 \
  --max-t 6 --max-t-m3 6 --planted 1 --random 0 --ladder-t none \
  --budget 172800 --p P-192 --engine generic --verify \
  --out $OUT/rows_m3_isogeny_t6.jsonl >> $OUT/run_m3_t6.log 2>&1

# 4. the Kummer ladder tail at p0: t=12 (again; the row was lost twice),
#    then t=13, t=14
$B --kinds kummer --m 2 --controls tower --t-min 12 --max-t 14 \
  --planted 1 --random 0 --ladder-t none --budget 432000 --seed 20261101 \
  --out $REM/remeasure_m2_kummer_tail.jsonl >> $REM/run_tail.log 2>&1

echo "queue done $(date)" >> $REM/run_tail.log
