#!/usr/bin/env bash
# Amendment 4: runs the jobs listed in sessions/jobs4.txt, one worker per
# allowed CPU, no start deadline. Thin orchestration; every number is ecbench's.
set -uo pipefail
cd "$(dirname "$0")"
BIN=../../target/release/ecbench
# Expand the allowed CPU list ("0-3" or "0,2-3") into an array.
cpus=()
for part in $(grep Cpus_allowed_list /proc/self/status | awk '{print $2}' | tr ',' ' '); do
  if [[ $part == *-* ]]; then for ((c=${part%-*}; c<=${part#*-}; c++)); do cpus+=($c); done; else cpus+=($part); fi
done
[ -f sessions/.next4 ] || echo 1 > sessions/.next4
next() { ( flock 9; n=$(cat sessions/.next4); echo $((n + 1)) > sessions/.next4; echo "$n" ) 9>sessions/.next4.lock; }
worker() {
  local cpu=$1
  while :; do
    local i job; i=$(next); job=$(sed -n "${i}p" sessions/jobs4.txt)
    [ -z "$job" ] && return
    echo "$(date -u +%FT%TZ) start $job cpu $cpu" >> sessions/queue.log
    "$BIN" run --spec "specs/SPEC-$job.json" --out "sessions/$job" --cpus "$cpu" \
      --lock "/tmp/ecbench-k4exp-$cpu.lock" --allow-busy > "sessions/$job.log" 2>&1
    echo "$(date -u +%FT%TZ) end $job exit $?" >> sessions/queue.log
  done
}
for c in "${cpus[@]}"; do worker "$c" & done
wait
echo "$(date -u +%FT%TZ) queue 4 done" >> sessions/queue.log
