#!/bin/sh
# Job queue for the frozen protocol: d in {12, 10, 8}, 6 seeds, 4 at a time, 1800 s cap per run.
# Resumable: a job with a status line is skipped. Thin orchestration only.
B=${B:-/home/user/bin/ic_m3_cycle}; D=$(cd "$(dirname "$0")" && pwd)
: > "$D/jobs.txt"
for d in 12 10 8; do for seed in 20261024 20261101 20261102 20261103 20261104 20261105; do
  echo "$d $seed" >> "$D/jobs.txt"; done; done
run() { d=$1; seed=$2; s=$(date +%s)
  timeout 1800 "$B" 0 37 "$d" 1 "$seed" "$D/d${d}_seed${seed}.json" > "$D/d${d}_seed${seed}.log" 2>&1
  echo "d=$d seed=$seed exit=$? wall=$(( $(date +%s) - s ))" >> "$D/status.txt"; }
while read d seed; do
  grep -q "^d=$d seed=$seed " "$D/status.txt" 2>/dev/null && continue
  while [ "$(jobs -r | wc -l)" -ge 4 ]; do sleep 2; done
  run "$d" "$seed" &
done < "$D/jobs.txt"
wait
echo "sweep done" >> "$D/status.txt"
