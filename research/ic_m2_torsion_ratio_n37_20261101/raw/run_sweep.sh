#!/bin/sh
# Job queue for the frozen protocol: d in {10, 8, 7}, 6 seeds, torsion on and off, 4 at a time,
# 1800 s cap per run. Resumable: a job with a status line is skipped. Thin orchestration only; every number comes from the producer's JSON.
B=${B:-/home/user/bin/ic_m2_cycle}; D=$(cd "$(dirname "$0")" && pwd)
: > "$D/jobs.txt"
for d in 10 8 7; do for seed in 20261024 20261101 20261102 20261103 20261104 20261105; do
  echo "$d $seed 1" >> "$D/jobs.txt"; echo "$d $seed 0" >> "$D/jobs.txt"; done; done
run() { d=$1; seed=$2; t=$3; s=$(date +%s)
  timeout 1800 "$B" 0 37 "$d" 1 "$seed" "$t" "$D/d${d}_t${t}_seed${seed}.json" > "$D/d${d}_t${t}_seed${seed}.log" 2>&1
  echo "d=$d t=$t seed=$seed exit=$? wall=$(( $(date +%s) - s ))" >> "$D/status.txt"; }
export -f run 2>/dev/null
while read d seed t; do
  # resumable: skip a job that already has a status line (it finished before an interruption)
  grep -q "^d=$d t=$t seed=$seed " "$D/status.txt" 2>/dev/null && continue
  while [ "$(jobs -r | wc -l)" -ge 4 ]; do sleep 2; done
  run "$d" "$seed" "$t" &
done < "$D/jobs.txt"
wait
echo "sweep done" >> "$D/status.txt"
