#!/bin/bash
# The F4 runs of PROTOCOL.md §2.3, in the protocol's order. From anywhere:
#
#     BIN=/path/to/pkm_tower_pilot research/pkm_p224_kummer_shape_20261022/run_f4.sh main
#     BIN=... research/pkm_p224_kummer_shape_20261022/run_f4.sh frontier   # R18 alone
#
# Each run writes runs/<id>.jsonl (rows) and runs/<id>.log (stderr);
# progress.txt records start, end and exit status. No run starts later than
# 7 hours after the group started (PROTOCOL.md §2.3, time rule).
set -u
here=$(cd "$(dirname "$0")" && pwd)
bin=${BIN:?set BIN to the pkm_tower_pilot binary}
group=${1:?main or frontier}
cd "$here/runs"
SL20=1032193; SL24=16760833; SL26=67104769; SL31=2147352577; SL32=4293918721
SL48=281474975662081; SL61=2305843009146585089; P0=786433
SEED=1347112276
start=$(date +%s)
deadline=$((start + 7 * 3600))
run() {
  name=$1; budget=$2; shift 2
  if [ "$(date +%s)" -gt "$deadline" ]; then
    echo "$(date -u +%FT%TZ) not run (deadline) $name: $*" >> ../progress.txt
    return
  fi
  echo "$(date -u +%FT%TZ) start $name: $*" >> ../progress.txt
  nice -n 10 "$bin" --engine generic --budget "$budget" --ladder-t none --planted 0 --random 2 \
    --seed $SEED --kinds kummer "$@" --out "$name.jsonl" 2> "$name.log"
  echo "$(date -u +%FT%TZ) end $name exit $?" >> ../progress.txt
}
M2=(--m 2 --controls tower,null --t-min 1)
M3=(--m 3 --controls tower,null --null-chain --t-min 2 --max-t-m3 4)
M4=(--m 4 --controls tower,null --null-chain --t-min 2 --max-t-m4 3)
if [ "$group" = frontier ]; then
  ulimit -v 9000000
  export RAYON_NUM_THREADS=2
  run R18-SL24-m2-N24 14400 --p $SL24 --m 2 --controls tower --t-min 12 --max-t 12 --targets 0 --max-nnz 1800000000
  exit 0
fi
ulimit -v 4000000
export RAYON_NUM_THREADS=1
# R0: positive control, the isogeny kind at p0
run R0-isogeny-p0-m2 1800 --p $P0 --kinds isogeny --m 2 --controls tower --t-min 2 --max-t 5
run R1-SL26-m2 1800 --p $SL26 "${M2[@]}" --max-t 10
run R2-SL31-m2 1800 --p $SL31 "${M2[@]}" --max-t 10
run R3-SL32-m2 1800 --p $SL32 "${M2[@]}" --max-t 10
run R4-SL20-m2-N16-20 1800 --p $SL20 --m 2 --controls tower --t-min 8 --max-t 10
run R5-SL26-m34 1800 --p $SL26 "${M3[@]}"
run R5b-SL26-m4 1800 --p $SL26 "${M4[@]}"
run R6-SL31-m3 1800 --p $SL31 "${M3[@]}"
run R7-SL32-m3 1800 --p $SL32 "${M3[@]}"
run R8-SL31-m4 1800 --p $SL31 "${M4[@]}"
run R9-SL48-m2 1800 --p $SL48 --m 2 --controls tower --t-min 1 --max-t 9 --verify
run R9n-SL48-m2-null 1800 --p $SL48 --m 2 --controls null --t-min 1 --max-t 7 --verify
run R10-SL48-m34 1800 --p $SL48 --m 3 --controls tower --t-min 2 --max-t-m3 4 --verify
run R10b-SL48-m4 1800 --p $SL48 --m 4 --controls tower --t-min 2 --max-t-m4 2 --verify
run R11-SL61-m2 1800 --p $SL61 --m 2 --controls tower --t-min 1 --max-t 9 --verify
run R11n-SL61-m2-null 1800 --p $SL61 --m 2 --controls null --t-min 1 --max-t 7 --verify
run R12-SL61-m3 1800 --p $SL61 --m 3 --controls tower --t-min 2 --max-t-m3 4 --verify
run R12b-SL61-m4 1800 --p $SL61 --m 4 --controls tower --t-min 2 --max-t-m4 2 --verify
run R13-P224-m2 1800 --p P-224 --m 2 --controls tower --t-min 1 --max-t 8 --verify
run R13n-P224-m2-null 1800 --p P-224 --m 2 --controls null --t-min 1 --max-t 7 --verify
run R14-P224-m3 1800 --p P-224 --m 3 --controls tower --t-min 2 --max-t-m3 4 --verify
run R14b-P224-m4 1800 --p P-224 --m 4 --controls tower --t-min 2 --max-t-m4 2 --verify
run R15-P224-m2-N18 7200 --p P-224 --m 2 --controls tower --t-min 9 --max-t 9 --targets 0 --verify
run R16-SL24-m2-N20-22 7200 --p $SL24 --m 2 --controls tower --t-min 10 --max-t 11 --targets 0
run R16n-SL24-m2-N22-null 7200 --p $SL24 --m 2 --controls null --t-min 11 --max-t 11 --targets 0
run R17-SL31-m3-N15 7200 --p $SL31 --m 3 --controls tower --t-min 5 --max-t-m3 5 --targets 0
run R19-SL26-m2-N22 7200 --p $SL26 --m 2 --controls tower --t-min 11 --max-t 11 --targets 0
echo "$(date -u +%FT%TZ) group main done" >> ../progress.txt
