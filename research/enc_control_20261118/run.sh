#!/usr/bin/env bash
# Formal data run, enc_control_20261118. Frozen prereg: PREREGISTRATION.md.
set -u
cd "$(dirname "$0")/../.."
BIN=${BIN:-target/release/examples/enc_control}
D=research/enc_control_20261118; RAW=$D/raw
ROWS=$RAW/rows.jsonl; LOG=$RAW/progress.txt
KMAX=${KMAX:-8}; BUDGET=${BUDGET:-300}
export RAYON_NUM_THREADS=2
: > $ROWS; : > $LOG
declare -A DEAD
run() { # cell-key arm k r draw [extra]
  local key=$1 arm=$2 k=$3 r=$4 dr=$5; shift 5
  if [ -n "${DEAD[$key]:-}" ]; then echo "SKIP $arm k=$k r=$r (censored earlier)" | tee -a $LOG; return; fi
  local s e rc; s=$(date +%s.%N)
  timeout $((BUDGET*4+120)) $BIN --arm $arm --k $k --r $r --draw $dr --budget $BUDGET --out $ROWS "$@" 2>>$LOG; rc=$?
  e=$(date +%s.%N)
  printf "== %s k=%s r=%s draw=%s rc=%s wall=%.1fs load=%s\n" $arm $k $r $dr $rc "$(echo "$e-$s"|bc)" "$(cut -d' ' -f1 /proc/loadavg)" | tee -a $LOG
  # censor: nonzero rc, or a row of this cell timed out / hit the bound
  if [ $rc -ne 0 ] || [ "$(jq -s --arg a $arm --argjson k $k --argjson r $r --argjson d $dr \
      '[.[]|select(.kind=="f4" and .arm==$a and .k==$k and .r==$r and .draw==$d and (.timed_out or .pairs_above_bound>0))]|length' $ROWS)" != 0 ]; then
    DEAD[$key]=1; echo "CENSORED $arm k=$k r=$r" | tee -a $LOG
  fi
}
for k in $(seq 2 $KMAX); do
  cells=()
  cells+=("tower-H|tower-H|0|0|--hky 1")
  for d in 0 1 2; do cells+=("coset|coset|0|$d|--hky 1"); done
  for d in 0 1 2; do cells+=("tower-rand|tower-rand|0|$d|"); done
  nd=2; [ $k -gt 6 ] && nd=1
  r=1; while [ $r -lt $((1<<k)) ]; do
    for d in $(seq 0 $((nd-1))); do cells+=("dose$r|dose|$r|$d|"); done; r=$((r*2)); done
  cells+=("bits-H|bits-H|0|0|")
  for d in 0 1 2; do cells+=("bits-rand|bits-rand|0|$d|"); done
  cells+=("randbase|randbase|0|0|")
  n=${#cells[@]}; off=$(( (k*7) % n ))
  for i in $(seq 0 $((n-1))); do
    IFS='|' read -r key arm r dr extra <<< "${cells[$(( (i+off) % n ))]}"
    run "$key" "$arm" "$k" "$r" "$dr" $extra
  done
done
echo DONE | tee -a $LOG
