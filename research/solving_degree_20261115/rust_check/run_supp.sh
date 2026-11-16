#!/bin/bash
# supplementary runs with larger primes (unit instances are rare at p ~ N^m)
B=./target/release/sd_static
ulimit -v 6000000
T=data/wall_times_supp.txt; : > $T
run() { local s=$(date +%s.%N)
  $B --m $1 --k $2 --p $3 --count $4 --seed $((7000+1000*$1+$2)) >> data/$5.jsonl
  local e=$(date +%s.%N); echo "$5 m=$1 k=$2 p=$3 count=$4 wall_s=$(echo "$e - $s" | bc)" >> $T; }
rm -f data/m2_p1073750017.jsonl data/m3_p1073750017.jsonl data/m2_p1073153.jsonl data/m2_p257.jsonl data/m2_p17_k3_many.jsonl
for k in 3 4 5 6 7 8 9 10 11 12; do run 2 $k 1073750017 20 m2_p1073750017; done
for k in 2 3 4 5 6; do run 3 $k 1073750017 20 m3_p1073750017; done
for k in 3 4 5 6 7 8 9 10 12; do run 2 $k 1073153 20 m2_p1073153; done
for k in 3 4 5 6 7 8; do run 2 $k 257 200 m2_p257; done
run 2 3 17 20000 m2_p17_k3_many
