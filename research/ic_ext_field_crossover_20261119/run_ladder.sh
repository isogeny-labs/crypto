#!/bin/bash
# Single-thread ladder. Usage: ./run_ladder.sh   (from this directory)
BIN=/home/user/crypto/target/release/examples/gaudry_cubic_bench
export RAYON_NUM_THREADS=1
cd "$(dirname "$0")"; mkdir -p raw
run() { # arm p seed extra
  out=raw/$1_p$2_s$3
  [ -f $out.json ] && return
  start=$(date +%s)
  timeout ${CAP:-900} taskset -c ${CPU:-0} $BIN --groebner --sparse-la $4 --p $2 --seed $3 --json $out.json > $out.log 2>&1
  echo "$1 p=$2 s=$3 exit=$? wall=$(( $(date +%s)-start ))s" >> raw/ladder.status
}
for p in 523 1039 2083 4003; do
  for s in 1 2; do run NL $p $s ""; run LP $p $s "--lp-rule --max-lp 2"; done
done
run LP 8101 1 "--lp-rule --max-lp 2"
run NL 8101 1 ""
