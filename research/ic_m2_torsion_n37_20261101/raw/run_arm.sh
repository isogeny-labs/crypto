#!/bin/sh
# usage: run_arm.sh <torsion 0|1> ; a=0 n=37, one target per process, 4 at a time, 1800 s cap each.
# Thin orchestration only; every number comes from the producer's JSON.
T=$1; B=${B:-/dev/shm/cargo-target/release/examples/ic_m2_cycle}; D=$(dirname "$0")
run() { seed=$1; s=$(date +%s)
  timeout 1800 "$B" 0 37 0 1 "$seed" "$T" "$D/t${T}_seed${seed}.json" > "$D/t${T}_seed${seed}.log" 2>&1
  echo "$seed exit=$? wall=$(( $(date +%s) - s ))" >> "$D/t${T}_status.txt"; }
for batch in "20261024 20261101 20261102 20261103" "20261104 20261105 20261106 20261107"; do
  for seed in $batch; do run $seed & done; wait
done
echo "arm $T done" >> "$D/t${T}_status.txt"
