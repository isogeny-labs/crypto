#!/usr/bin/env bash
# Exclusive phase records and process-group guard. Inner timeouts use --foreground.
set -u
set -o pipefail
ROOT=${ECDLP_PHASE_ROOT:?explicit phase-output root required}
LABEL=$1
LIMIT=$2
shift 2
OUT="$ROOT/$LABEL"
mkdir "$OUT" || exit 2
date -u '+%Y-%m-%dT%H:%M:%SZ' > "$OUT/started-utc.txt"
printf '%q ' "$@" > "$OUT/command.txt"
read -r INITIAL < /sys/fs/cgroup/memory.current
if [ "$INITIAL" -ge 15032385536 ]; then
  printf '%s\n' 75 > "$OUT/exit-code.txt"
  printf '%s\n' 1 > "$OUT/physical-stop.txt"
  printf '%s\n' "$INITIAL" > "$OUT/peak-cgroup-bytes.txt"
  date -u '+%Y-%m-%dT%H:%M:%SZ' > "$OUT/finished-utc.txt"
  exit 75
fi
setsid timeout --signal=TERM --kill-after=10 "$LIMIT" flock /tmp/ecdlp-proof-check-20261123/lean-worker.lock "$@" > "$OUT/stdout.txt" 2> "$OUT/stderr.txt" &
PID=$!
printf '%s\n' "$PID" > "$OUT/process-group-leader.txt"
PEAK=$INITIAL
STOP=0
while kill -0 "$PID" 2>/dev/null; do
  read -r USED < /sys/fs/cgroup/memory.current
  if [ "$USED" -gt "$PEAK" ]; then PEAK="$USED"; fi
  if [ "$USED" -ge 15032385536 ]; then
    STOP=1
    kill -TERM -- "-$PID" 2>/dev/null || true
    break
  fi
  sleep 0.25
done
wait "$PID"
CODE=$?
date -u '+%Y-%m-%dT%H:%M:%SZ' > "$OUT/finished-utc.txt"
printf '%s\n' "$CODE" > "$OUT/exit-code.txt"
printf '%s\n' "$STOP" > "$OUT/physical-stop.txt"
printf '%s\n' "$PEAK" > "$OUT/peak-cgroup-bytes.txt"
printf 'Phase %s: exit %s; physical stop %s\n' "$LABEL" "$CODE" "$STOP"
tail -n 10 "$OUT/stdout.txt" "$OUT/stderr.txt"
exit "$CODE"
