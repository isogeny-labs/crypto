#!/usr/bin/env bash
# Exploratory coset-union degree dial (ADDENDUM_cosetunion.md). Not preregistered.
set -u
cd "$(dirname "$0")/../.."
BIN=target/release/examples/enc_control
RAW=research/enc_control_20261118/raw; ROWS=$RAW/addendum_rows.jsonl; LOG=$RAW/addendum_progress.txt
export RAYON_NUM_THREADS=2
: > $ROWS; : > $LOG
for k in 4 5 6 7 8; do
  nd=2; [ $k -ge 8 ] && nd=1
  cells=("tower-H 0 0")
  for j in $(seq 0 $k); do for d in $(seq 0 $((nd-1))); do cells+=("cosetunion $j $d"); done; done
  n=${#cells[@]}; off=$(( (k*5) % n ))
  for i in $(seq 0 $((n-1))); do
    read -r arm j d <<< "${cells[$(( (i+off) % n ))]}"
    la=$(cut -d' ' -f1 /proc/loadavg)
    timeout 1500 $BIN --arm $arm --k $k --r $j --draw $d --budget 300 --random-targets 1 --out $ROWS 2>>$LOG
    echo "== $arm k=$k j=$j draw=$d rc=$? load=$la" | tee -a $LOG
  done
done
echo DONE | tee -a $LOG
