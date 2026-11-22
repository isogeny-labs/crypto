#!/bin/bash
# Validator replay of every committed session of research/ic_ksum_prune_20261115 (and, with a second
# argument, of sibling sessions recorded by an older binary). The committed files are never touched:
# each session is copied to a scratch directory, records.jsonl.gz is gunzipped there, and
# `ecbench verify --replay-all --exit-code` runs on the copy under the shared CPU lock.
# usage: replay.sh <ecbench> <scratch dir> <out dir> <session dir>...
set -u
BIN=$1; SCR=$2; OUT=$3; shift 3
LOCK=/tmp/claude-0/-home-user/cba6917f-779a-544b-9ecf-84e512b45b33/scratchpad/cpu.lock
export RAYON_NUM_THREADS=1
mkdir -p "$SCR" "$OUT"
for d in "$@"; do
  n=$(basename "$d"); c="$SCR/$n"; rm -rf "$c"; mkdir -p "$c"
  cp -r "$d"/session.json "$d"/spec.json "$d"/plan.json "$d"/host.json "$d"/exec "$c"/
  if [ -f "$d/records.jsonl" ]; then cp "$d/records.jsonl" "$c/"; else gunzip -c "$d/records.jsonl.gz" > "$c/records.jsonl"; fi
  s=$(date +%s)
  flock "$LOCK" "$BIN" verify --dir "$c" --replay-all --exit-code --out "$OUT/$n.verify.json" > /dev/null 2> "$OUT/$n.verify.err"
  rc=$?
  printf '%s\trc=%d\t%ds\n' "$n" "$rc" $(( $(date +%s) - s )) >> "$OUT/summary.txt"
  gzip -n -9 -f "$OUT/$n.verify.json"
  rm -rf "$c"
done
