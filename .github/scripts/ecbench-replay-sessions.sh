#!/usr/bin/env bash
# Replay every measured run of every committed ecbench session named in a list
# file (one directory per line) and require each replay to reproduce the
# record bit for bit (`ecbench verify --replay-all --allow-interrupted
# --exit-code`).  An interrupted session is evidence of the interruption:
# every per-record check applies, only completeness does not.  Which sessions
# go in the list is decided by ecbench-replay-plan.sh (weekly rotation) or
# LONG_SESSIONS.tsv (on request); this script replays what it is given.
# Usage: ecbench-replay-sessions.sh LIST [ECBENCH]
# REPLAY_JOBS sets the parallelism (default: nproc).  Every failing session is
# named, then the script exits 1.
set -euo pipefail
list=${1:?usage: ecbench-replay-sessions.sh LIST [ECBENCH]}
ecbench=${2:-./target/release/ecbench}
test -f "$list" || { echo "::error::$list is missing" >&2; exit 1; }
count=$(grep -c . "$list" || true)
failed=$(mktemp)
export ecbench failed
echo "replaying $count sessions"
if [[ "$count" -gt 0 ]]; then
tr '\n' '\0' < "$list" | xargs -0 -P "${REPLAY_JOBS:-$(nproc)}" -n 1 bash -c '
  out="$failed.$$"
  dir=$1
  # Gzip-compressed records are replayed from a decompressed copy, as the
  # integrity audit does: the checks then run on exactly the committed bytes.
  if [ -f "$1/records.jsonl.gz" ] && [ ! -f "$1/records.jsonl" ]; then
    dir="$out.session"
    cp -r "$1" "$dir"
    gunzip "$dir"/*.gz
  elif [ -f "$1/ecbench_records.jsonl.gz" ] && [ ! -f "$1/records.jsonl" ]; then
    dir="$out.session"
    cp -r "$1" "$dir"
    gzip -dc "$dir/ecbench_records.jsonl.gz" > "$dir/records.jsonl"
  fi
  echo "replaying $1"
  if ! "$ecbench" verify --dir "$dir" --replay-all --allow-interrupted --exit-code > "$out.json" 2> "$out.err"; then
    msg=$(echo "$1"; { jq -r ".problems[]?" "$out.json" 2> /dev/null || true; cat "$out.err"; } | head -n 40 | sed "s/^/    /")
    printf "%s\n" "$msg" >> "$failed"
  fi
  rm -rf "$out.json" "$out.err" "$out.session"
' replay
fi
if [ -s "$failed" ]; then
  echo "::error::committed ecbench sessions that do not replay exactly on this host:" >&2
  cat "$failed" >&2
  rm -f "$failed"
  exit 1
fi
rm -f "$failed"
summary="$count committed ecbench sessions replay every measured run exactly (--replay-all --allow-interrupted)"
echo "$summary"
if [[ -n "${GITHUB_STEP_SUMMARY:-}" ]]; then
  echo "$summary" >> "$GITHUB_STEP_SUMMARY"
fi
