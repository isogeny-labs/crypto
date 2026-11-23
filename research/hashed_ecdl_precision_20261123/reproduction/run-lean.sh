#!/usr/bin/env bash
# Thin proof compiler orchestration; no scientific measurement.
set -u
set -o pipefail
ROOT=/tmp/ecdlp-precision-audit-20261123
LABEL=$1
SOURCE=$2
RELATIVE=${3:-$(basename "$SOURCE")}
OUT="$ROOT/checks/$LABEL"
mkdir -p "$ROOT/checks"
mkdir "$OUT" || exit 2
mkdir -p "$OUT/$(dirname "$RELATIVE")"
cp "$SOURCE" "$OUT/$RELATIVE"
INPUT="$OUT/$RELATIVE"
chmod a-w "$INPUT"
sha256sum "$INPUT" > "$OUT/input-before.sha256"
IFS= read -r LEAN_PATH < "$ROOT/lean-path.txt"
export LEAN_PATH
cd "$OUT" || exit 2
date -u '+%Y-%m-%dT%H:%M:%SZ' > "$OUT/started-utc.txt"
setsid timeout --signal=TERM --kill-after=5 600 flock /tmp/ecdlp-proof-check-20261123/lean-worker.lock /tmp/ecdlp-proof-check-20261123/lean-4.35.0-linux/bin/lean-kernel-original -j1 -M0 -o "$OUT/${RELATIVE%.lean}.olean" "$INPUT" > "$OUT/stdout.txt" 2> "$OUT/stderr.txt" &
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
printf '%s\n' "$CODE" > "$OUT/exit-code.txt"
printf '%s\n' "$MAX_MEMORY" > "$OUT/peak-cgroup-bytes.txt"
printf '%s\n' "$STOP" > "$OUT/physical-stop.txt"
date -u '+%Y-%m-%dT%H:%M:%SZ' > "$OUT/finished-utc.txt"
sha256sum -c "$OUT/input-before.sha256" > "$OUT/input-after-check.txt"
printf 'Lean phase %s: exit %s; physical stop %s\n' "$LABEL" "$CODE" "$STOP"
cat "$OUT/stdout.txt" "$OUT/stderr.txt"
exit "$CODE"
