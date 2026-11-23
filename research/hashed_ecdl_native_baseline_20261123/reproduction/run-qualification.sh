#!/usr/bin/env bash
# Thin launcher for the frozen, existing native Rust test executables.
set -u
set -o pipefail
set -C
ROOT=/tmp/ecdlp-native-bootstrap-20261123
OUT="$ROOT/qualification"
cd /tmp/ecdlp-native-crypto-20261123/crates/hashed-ecdl-tensor || exit 2
sha256sum -c "$OUT/executables.sha256" > "$OUT/before-call-hashes.txt" || exit 2
date -u '+%Y-%m-%dT%H:%M:%SZ' > "$OUT/started-utc.txt"
printf 'native-test-output-transport-ok\n' > "$OUT/transport-control.stdout"
printf 'index\tname\tstatus\texit_code\n' > "$OUT/rows.tsv"
START=$SECONDS
MAX_MEMORY=0
STOP=0
while IFS=$'\t' read -r INDEX NAME EXE; do
  REMAINING=$((600-(SECONDS-START)))
  if [ "$STOP" -eq 1 ] || [ "$REMAINING" -le 0 ]; then
    printf '%s\t%s\tnot_run_budget\t\n' "$INDEX" "$NAME" >> "$OUT/rows.tsv"
    continue
  fi
  if [ "$REMAINING" -gt 300 ]; then REMAINING=300; fi
  setsid timeout --signal=TERM --kill-after=5 "$REMAINING" "$EXE" --test-threads=1 > "$OUT/$INDEX-$NAME.stdout" 2> "$OUT/$INDEX-$NAME.stderr" &
  PID=$!
  PHYSICAL_STOP=0
  while kill -0 "$PID" 2>/dev/null; do
    read -r USED < /sys/fs/cgroup/memory.current
    if [ "$USED" -gt "$MAX_MEMORY" ]; then MAX_MEMORY="$USED"; fi
    if [ "$USED" -ge 15032385536 ]; then
      PHYSICAL_STOP=1
      STOP=1
      kill -TERM -- "-$PID" 2>/dev/null || true
      break
    fi
    sleep 0.25
  done
  wait "$PID"
  CODE=$?
  STATUS=failed
  if [ "$CODE" -eq 0 ]; then STATUS=passed; fi
  if [ "$CODE" -eq 124 ]; then STATUS=timeout; fi
  if [ "$PHYSICAL_STOP" -eq 1 ]; then STATUS=physical_memory_stop; fi
  printf '%s\t%s\t%s\t%s\n' "$INDEX" "$NAME" "$STATUS" "$CODE" >> "$OUT/rows.tsv"
  printf '%s: %s (exit %s)\n' "$NAME" "$STATUS" "$CODE"
done < "$OUT/schedule.tsv"
sha256sum -c "$OUT/executables.sha256" > "$OUT/after-call-hashes.txt"
HASH_CODE=$?
date -u '+%Y-%m-%dT%H:%M:%SZ' > "$OUT/finished-utc.txt"
printf '%s\n' "$MAX_MEMORY" > "$OUT/peak-cgroup-bytes.txt"
printf '%s\n' "$HASH_CODE" > "$OUT/post-call-hash-exit.txt"
if [ "$HASH_CODE" -ne 0 ]; then exit 2; fi
if awk -F '\t' 'NR>1 && $3!="passed" {bad=1} END{exit !bad}' "$OUT/rows.tsv"; then exit 1; fi
