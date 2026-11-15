#!/bin/bash
# usage: bash run_all.sh ; single-threaded program, run sequentially
B=./target/release/sd_static
ulimit -v 6000000
T=data/wall_times.txt; : > $T
run() { # m k p count tag
  local s=$(date +%s.%N)
  $B --m $1 --k $2 --p $3 --count $4 --seed $((1000*$1+$2)) >> data/$5.jsonl
  local e=$(date +%s.%N)
  echo "$5 m=$1 k=$2 p=$3 count=$4 wall_s=$(echo "$e - $s" | bc)" >> $T
}
rm -f data/m2_p12289.jsonl data/m2_pmin.jsonl data/m3_p12289.jsonl
for k in 3 4 5 6 7 8 9 10 11 12; do run 2 $k 12289 20 m2_p12289; done
for kp in 3:17 4:17 5:97 6:193 7:257 8:257; do run 2 ${kp%%:*} ${kp##*:} 50 m2_pmin; done
for k in 2 3 4 5 6; do run 3 $k 12289 20 m3_p12289; done
