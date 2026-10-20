#!/usr/bin/env bash
# Amendments 5 and 5a: runs the jobs of ${JOBS:-jobs_ext2.txt}. Up to four
# workers. The container's CPU set changes while the queue runs (4, then 1, then
# 2 CPUs were seen), so a worker reads the CPUs it is allowed *at each job
# start*, pins its job to the CPU of its own index, and waits while it has none.
# A job whose session is complete is skipped; a launch that leaves no complete
# session is retried (up to 5 times, 30 s apart) after the half-written session
# directory is moved to sessions-interrupted/ (outside sessions/, so no fit reads
# it). A job not started before DEADLINE (epoch seconds) is listed in
# sessions/NOT_RUN5.txt. Thin orchestration; every number is ecbench's.
set -uo pipefail
cd "$(dirname "$0")"
BIN=../../target/release/ecbench
DEADLINE=${DEADLINE:?epoch seconds}
JOBS=${JOBS:-jobs_ext2.txt}
mkdir -p sessions sessions-interrupted
allowed() { # prints the allowed CPUs, one per line
  for part in $(grep Cpus_allowed_list /proc/self/status | awk '{print $2}' | tr ',' ' '); do
    if [[ $part == *-* ]]; then seq "${part%-*}" "${part#*-}"; else echo "$part"; fi
  done
}
[ -f sessions/.next5 ] || echo 1 > sessions/.next5
next() { ( flock 9; n=$(cat sessions/.next5); echo $((n + 1)) > sessions/.next5; echo "$n" ) 9>sessions/.next5.lock; }
worker() {
  local w=$1 cpu
  while :; do
    # wait until this worker's index has a CPU
    while :; do
      mapfile -t A < <(allowed)
      [ "${#A[@]}" -gt "$w" ] && break
      sleep 60
    done
    local i job; i=$(next); job=$(sed -n "${i}p" "$JOBS")
    [ -z "$job" ] && return
    grep -q "complete ->" "sessions/$job.log" 2>/dev/null && continue
    if [ "$(date +%s)" -ge "$DEADLINE" ]; then echo "$job not started before the deadline" >> sessions/NOT_RUN5.txt; continue; fi
    local attempt
    for attempt in 1 2 3 4 5; do
      mapfile -t A < <(allowed)
      cpu=${A[$((w % ${#A[@]}))]}
      [ -e "sessions/$job" ] && mv "sessions/$job" "sessions-interrupted/$job.$(date +%s)"
      echo "$(date -u +%FT%TZ) start $job cpu $cpu attempt $attempt" >> sessions/queue.log
      "$BIN" run --spec "specs/SPEC-$job.json" --out "sessions/$job" --cpus "$cpu" \
        --lock "/tmp/ecbench-k4exp-$w.lock" --allow-busy > "sessions/$job.log" 2>&1
      if grep -q "complete ->" "sessions/$job.log"; then
        echo "$(date -u +%FT%TZ) end $job complete" >> sessions/queue.log; break
      fi
      echo "$(date -u +%FT%TZ) end $job FAILED attempt $attempt: $(tail -n 1 "sessions/$job.log" | cut -c1-120)" >> sessions/queue.log
      cp "sessions/$job.log" "sessions-interrupted/$job.attempt$attempt.$(date +%s).log" 2>/dev/null
      sleep 30
    done
  done
}
for w in 0 1 2 3; do worker "$w" & done
wait
echo "$(date -u +%FT%TZ) queue 5 done" >> sessions/queue.log
