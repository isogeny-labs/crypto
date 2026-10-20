#!/usr/bin/env bash
# Amendment 5: runs jobs_ext.txt, one worker per allowed CPU. A job not started
# before DEADLINE (epoch seconds) is listed in sessions/NOT_RUN5.txt. Resumable:
# a job whose session is complete is skipped; an interrupted session directory is
# moved to sessions-interrupted/ (outside sessions/, so no fit reads it) and the
# job is run again. Thin orchestration; every number is ecbench's.
set -uo pipefail
cd "$(dirname "$0")"
BIN=../../target/release/ecbench
DEADLINE=${DEADLINE:?epoch seconds}
mkdir -p sessions sessions-interrupted
cpus=()
for part in $(grep Cpus_allowed_list /proc/self/status | awk '{print $2}' | tr ',' ' '); do
  if [[ $part == *-* ]]; then for ((c=${part%-*}; c<=${part#*-}; c++)); do cpus+=($c); done; else cpus+=($part); fi
done
[ -f sessions/.next5 ] || echo 1 > sessions/.next5
next() { ( flock 9; n=$(cat sessions/.next5); echo $((n + 1)) > sessions/.next5; echo "$n" ) 9>sessions/.next5.lock; }
worker() {
  local cpu=$1
  while :; do
    local i job; i=$(next); job=$(sed -n "${i}p" jobs_ext.txt)
    [ -z "$job" ] && return
    if grep -q "complete ->" "sessions/$job.log" 2>/dev/null; then continue; fi
    if [ "$(date +%s)" -ge "$DEADLINE" ]; then echo "$job not started before the deadline" >> sessions/NOT_RUN5.txt; continue; fi
    if [ -e "sessions/$job" ]; then mv "sessions/$job" "sessions-interrupted/$job.$(date +%s)"; fi
    echo "$(date -u +%FT%TZ) start $job cpu $cpu" >> sessions/queue.log
    "$BIN" run --spec "specs/SPEC-$job.json" --out "sessions/$job" --cpus "$cpu" \
      --lock "/tmp/ecbench-k4exp-$cpu.lock" --allow-busy > "sessions/$job.log" 2>&1
    echo "$(date -u +%FT%TZ) end $job exit $?" >> sessions/queue.log
  done
}
for c in "${cpus[@]}"; do worker "$c" & done
wait
echo "$(date -u +%FT%TZ) queue 5 done" >> sessions/queue.log
