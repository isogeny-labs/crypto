#!/usr/bin/env bash
# Thin orchestration of native build/test/run commands, with exclusive records.
set -u
set -o pipefail
ROOT=/tmp/ecdlp-baseline-cohort-20261123
LABEL=$1
LIMIT=$2
shift 2
OUT="$ROOT/$LABEL"
mkdir "$OUT" || exit 2
date -u '+%Y-%m-%dT%H:%M:%SZ' > "$OUT/started-utc.txt"
printf '%q ' "$@" > "$OUT/command.txt"
setsid timeout --signal=TERM --kill-after=10 "$LIMIT" flock /tmp/ecdlp-proof-check-20261123/lean-worker.lock "$@" > "$OUT/stdout.txt" 2> "$OUT/stderr.txt" &
PID=$!
PEAK=0
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
tail -n 14 "$OUT/stdout.txt" "$OUT/stderr.txt"
exit "$CODE"
