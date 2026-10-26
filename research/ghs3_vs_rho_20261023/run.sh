#!/bin/sh
# Frozen driver for PROTOCOL.md: 21 jobs (b = 14..20, curves 0-2), at most 3 at once,
# single-threaded, nice, 4.5 GB address-space cap, 90-minute wall limit per job.
# Usage (from the repository root): sh research/ghs3_vs_rho_20261023/run.sh
set -eu
BIN=./target/release/examples/ghs3_vs_rho_20261023
OUT=research/ghs3_vs_rho_20261023/raw
mkdir -p $OUT
cx_of() { if [ $1 -le 17 ]; then echo 16; else echo 8; fi; }
k_r2() { awk -v b=$1 'BEGIN{printf "%.4f", exp(0.2669 - 0.0941*(b-20))}'; }
for b in 20 19 18 17 16 15 14; do
  for c in 0 1 2; do
    echo "$b $c $(k_r2 $b) $(cx_of $b)"
  done
done | xargs -P 3 -L 1 sh -c '
  f='"$OUT"'/b$0_c$1.jsonl
  [ -s $f ] && exit 0
  S=$(date +%s)
  RAYON_NUM_THREADS=1 nice timeout 5400 prlimit --as=4500000000 -- '"$BIN"' --bits $0 --curve $1 \
    --kappa $2 --cx $3 --streams w --walks 32 --las lpsgebw2@$3 --rho-targets 8 --out $f \
    2>>'"$OUT"'/progress.log || { echo "FAILED rc=$? b$0 c$1" >>'"$OUT"'/progress.log; exit 1; }
  echo "JOB $(( $(date +%s) - S ))s b$0 c$1" >>'"$OUT"'/progress.log
'
