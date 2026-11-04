#!/bin/sh
# Job queue for the frozen protocol: 8 q values x 6 seeds at d = 12 on a = 0, n = 37, 4 at a time,
# 1800 s cap per run. Resumable: a job with a status line is skipped. Thin orchestration only.
B=${B:-/home/user/bin/ic_mix_cycle}; D=$(cd "$(dirname "$0")" && pwd)
: > "$D/jobs.txt"
for qi in 0 1 2 3 4 5 6 7; do for seed in 20261024 20261101 20261102 20261103 20261104 20261105; do
  echo "$qi $seed" >> "$D/jobs.txt"; done; done
qval() { case $1 in 0) echo 0;; 1) echo 0.000083;; 2) echo 0.00025;; 3) echo 0.00075;; 4) echo 0.0022;; 5) echo 0.008;; 6) echo 0.025;; 7) echo 1;; esac; }
run() { qi=$1; seed=$2; s=$(date +%s)
  timeout 1800 "$B" 0 37 12 1 "$seed" "$(qval $qi)" "$D/q${qi}_seed${seed}.json" > "$D/q${qi}_seed${seed}.log" 2>&1
  echo "q=$qi seed=$seed exit=$? wall=$(( $(date +%s) - s ))" >> "$D/status.txt"; }
while read qi seed; do
  grep -q "^q=$qi seed=$seed " "$D/status.txt" 2>/dev/null && continue
  while [ "$(jobs -r | wc -l)" -ge 4 ]; do sleep 2; done
  run "$qi" "$seed" &
done < "$D/jobs.txt"
wait
echo "sweep done" >> "$D/status.txt"
