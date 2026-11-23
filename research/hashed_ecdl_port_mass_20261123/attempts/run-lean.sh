#!/usr/bin/env bash
set -euo pipefail
ROOT=/tmp/ecdlp-port-mass-20261123
LABEL=$1
SOURCE=$2
RELATIVE=${3:-$(basename "$SOURCE")}
OUT="$ROOT/$LABEL"
mkdir "$OUT"
mkdir -p "$OUT/$(dirname "$RELATIVE")"
cp "$SOURCE" "$OUT/$RELATIVE"
chmod a-w "$OUT/$RELATIVE"
read -r LEAN_PATH < "$ROOT/lean-path.txt"
export LEAN_PATH
sha256sum "$ROOT/run-lean.sh" "$ROOT/lean-path.txt" "$OUT/$RELATIVE" /tmp/ecdlp-native-crypto-20261123/research/hashed_ecdl_port_mass_20261123/{PROOF-MAP.md,NATIVE-PLAN.md} /tmp/ecdlp-proof-check-20261123/lean-4.35.0-linux/bin/lean-kernel-original /tmp/ecdlp-proof-check-20261123/source/lean/.lake/packages/mathlib/Mathlib/Algebra/BigOperators/Group/Finset/Basic.lean /tmp/ecdlp-proof-check-20261123/source/lean/.lake/packages/mathlib/Mathlib/Data/Fintype/Basic.lean > "$OUT/input-before.sha256"
cd "$OUT"
date -u '+%Y-%m-%dT%H:%M:%SZ' > started-utc.txt
read -r PEAK < /sys/fs/cgroup/memory.current
STOP=0
if [ "$PEAK" -ge 15032385536 ]; then printf '75\n' > exit-code.txt; printf '1\n' > physical-stop.txt; exit 75; fi
setsid timeout --signal=TERM --kill-after=5 600 flock /tmp/ecdlp-proof-check-20261123/lean-worker.lock /tmp/ecdlp-proof-check-20261123/lean-4.35.0-linux/bin/lean-kernel-original -j1 -M0 -o "${RELATIVE%.lean}.olean" "$RELATIVE" > stdout.txt 2> stderr.txt &
PID=$!
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
printf '%s\n' "$CODE" > exit-code.txt
printf '%s\n' "$STOP" > physical-stop.txt
printf '%s\n' "$PEAK" > peak-cgroup-bytes.txt
date -u '+%Y-%m-%dT%H:%M:%SZ' > finished-utc.txt
sha256sum -c input-before.sha256 > input-after-check.log
cat stdout.txt stderr.txt
exit "$CODE"
