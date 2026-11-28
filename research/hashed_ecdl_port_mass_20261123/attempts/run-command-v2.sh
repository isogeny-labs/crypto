#!/usr/bin/env bash
# One bounded source-frozen qualification phase.
set -euo pipefail
ROOT=/tmp/ecdlp-port-mass-20261123
LABEL=$1
shift
OUT="$ROOT/$LABEL"
mkdir "$OUT"
export PATH="/tmp/ecdlp-precision-audit-20261123/bin:/tmp/ecdlp-native-tools-20261123/rust-1.99.1/bin:$PATH"
read -r LEAN_PATH < "$ROOT/lean-path.txt"
export LEAN_PATH
export CARGO_HOME=/tmp/ecdlp-native-tools-20261123/cargo-home
export CARGO_BUILD_JOBS=1
export CARGO_INCREMENTAL=0
printf '%q ' "$@" > "$OUT/command.txt"
printf '\n' >> "$OUT/command.txt"
cp "$ROOT/additional-inputs.sha256" "$OUT/additional-inputs.frozen.sha256"
chmod a-w "$OUT/additional-inputs.frozen.sha256"
sha256sum "$ROOT/run-command-v2.sh" "$ROOT/lean-path.txt" "$OUT/additional-inputs.frozen.sha256" > "$OUT/input-before.sha256"
cat "$OUT/additional-inputs.frozen.sha256" >> "$OUT/input-before.sha256"
sha256sum -c "$OUT/input-before.sha256" > "$OUT/input-precheck.log"
date -u '+%Y-%m-%dT%H:%M:%SZ' > "$OUT/started-utc.txt"
read -r INITIAL < /sys/fs/cgroup/memory.current
if [ "$INITIAL" -ge 15032385536 ]; then
  printf '75\n' > "$OUT/exit-code.txt"
  printf '1\n' > "$OUT/physical-stop.txt"
  printf '%s\n' "$INITIAL" > "$OUT/peak-cgroup-bytes.txt"
  exit 75
fi
setsid timeout --signal=TERM --kill-after=5 600 flock /tmp/ecdlp-proof-check-20261123/lean-worker.lock "$@" > "$OUT/stdout.txt" 2> "$OUT/stderr.txt" &
PID=$!
PEAK=$INITIAL
STOP=0
while kill -0 "$PID" 2>/dev/null; do
  read -r USED < /sys/fs/cgroup/memory.current
  if [ "$USED" -gt "$PEAK" ]; then PEAK=$USED; fi
  if [ "$USED" -ge 15032385536 ]; then STOP=1; kill -TERM -- "-$PID" 2>/dev/null || true; break; fi
  sleep 0.25
done
set +e
wait "$PID"
CODE=$?
set -e
date -u '+%Y-%m-%dT%H:%M:%SZ' > "$OUT/finished-utc.txt"
printf '%s\n' "$CODE" > "$OUT/exit-code.txt"
printf '%s\n' "$STOP" > "$OUT/physical-stop.txt"
printf '%s\n' "$PEAK" > "$OUT/peak-cgroup-bytes.txt"
sha256sum -c "$OUT/input-before.sha256" > "$OUT/input-after-check.log"
cat "$OUT/stdout.txt" "$OUT/stderr.txt"
exit "$CODE"
