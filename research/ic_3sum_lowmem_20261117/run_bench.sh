#!/bin/bash
# usage: run_bench.sh  (from repo root, after cargo build --release --example ic_3sum_lowmem)
E=target/release/examples/ic_3sum_lowmem
O=research/ic_3sum_lowmem_20261117
: > $O/bench.jsonl
# (bits B n) : n ~ targets the adaptive crate path needed at that B (baseline_*.jsonl)
for cfg in "28 480 900" "28 960 220" "28 1920 55" "28 3840 14"; do
  set -- $cfg; bits=$1; b=$2; n=$3
  for rep in 1 2 3; do
    $E --mode bench --oracle table  --bits $bits --b $b --n $n >> $O/bench.jsonl
    for div in 1 4 16; do
      ch=$(( (n+div-1)/div ))
      $E --mode bench --oracle stream --bits $bits --b $b --n $n --chunk $ch >> $O/bench.jsonl
    done
  done
done
# nested (O(B) memory, O(B^2)/target) at small B
for cfg in "28 60 600" "28 120 300" "28 240 100"; do
  set -- $cfg
  for rep in 1 2; do
    $E --mode bench --oracle table  --bits $1 --b $2 --n $3 >> $O/bench.jsonl
    $E --mode bench --oracle nested --bits $1 --b $2 --n $3 >> $O/bench.jsonl
  done
done
