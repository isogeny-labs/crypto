#!/usr/bin/env bash
# Thin bounded governance/proof-gate launcher.
set -u
set -o pipefail
ROOT=/tmp/ecdlp-precision-audit-20261123
LABEL=$1
shift
OUT="$ROOT/checks/$LABEL"
mkdir "$OUT" || exit 2
export PATH="$ROOT/bin:$PATH"
IFS= read -r LEAN_PATH < "$ROOT/history3-lean-path.txt"
export LEAN_PATH
date -u '+%Y-%m-%dT%H:%M:%SZ' > "$OUT/started-utc.txt"
printf '%q ' "$@" > "$OUT/command.txt"
setsid timeout --signal=TERM --kill-after=5 600 flock /tmp/ecdlp-proof-check-20261123/lean-worker.lock "$@" > "$OUT/stdout.txt" 2> "$OUT/stderr.txt" &
PID=$!
MAX_MEMORY=0
STOP=0
while kill -0 "$PID" 2>/dev/null; do
  read -r USED < /sys/fs/cgroup/memory.current
  if [ "$USED" -gt "$MAX_MEMORY" ]; then MAX_MEMORY="$USED"; fi
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
printf '%s\n' "$MAX_MEMORY" > "$OUT/peak-cgroup-bytes.txt"
printf 'Gate phase %s: exit %s; physical stop %s\n' "$LABEL" "$CODE" "$STOP"
cat "$OUT/stdout.txt" "$OUT/stderr.txt"
exit "$CODE"
