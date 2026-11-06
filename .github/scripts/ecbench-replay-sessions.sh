#!/usr/bin/env bash
# Replay every measured run of every committed ecbench session named in a list
# file (one directory per line) and require each replay to reproduce the
# record bit for bit (`ecbench verify --replay-all --allow-interrupted
# --exit-code`).  An interrupted session is evidence of the interruption:
# every per-record check applies, only completeness does not.
#
# Excluded, because they cannot be replayed whole: the hash-pinned incomplete
# scratch sessions (no spec or host capsule; INCOMPLETE_SESSIONS.tsv) and the
# owner-dispositioned defective sessions (verify fails by design; they are
# checked against their pinned defect by ecbench-audit-sessions.sh and
# DEFECTIVE_SESSIONS.tsv).  Both lists are read as the audit script reads them.
#
# Usage: ecbench-replay-sessions.sh LIST [ECBENCH] [INCOMPLETE_TSV] [DEFECTIVE_TSV]
# REPLAY_SHARD=K/N replays only shard K (0-based) of N, so N jobs together
# replay the whole list.  A replay costs about what the recorded run cost, so
# shards are balanced by the recorded solver wall time of the verified records
# (sum of time.solve_wall_ns; record bytes, a .gz file counting 8x, when a
# session has none): largest session first, each into the lightest shard so
# far, ties to the lower shard and sessions in list order, so every shard
# computes the same assignment.  REPLAY_JOBS sets the parallelism (default: nproc).
# Every failing session is named, then the script exits 1.
set -euo pipefail
shard=${REPLAY_SHARD:-0/1}
shard_k=${shard%/*}; shard_n=${shard#*/}
if [[ ! "$shard_k" =~ ^[0-9]+$ || ! "$shard_n" =~ ^[1-9][0-9]*$ || "$shard_k" -ge "$shard_n" ]]; then
  echo "::error::REPLAY_SHARD must be K/N with 0 <= K < N, got $shard" >&2; exit 1
fi
list=${1:?usage: ecbench-replay-sessions.sh LIST [ECBENCH] [INCOMPLETE_TSV] [DEFECTIVE_TSV]}
ecbench=${2:-./target/release/ecbench}
incomplete=${3:-docs/ecbench/INCOMPLETE_SESSIONS.tsv}
defective=${4:-docs/ecbench/DEFECTIVE_SESSIONS.tsv}
test -s "$list" || { echo "::error::$list is empty" >&2; exit 1; }
test -s "$incomplete" || { echo "::error::$incomplete is missing" >&2; exit 1; }
test -s "$defective" || { echo "::error::$defective is missing" >&2; exit 1; }
excluded=$(mktemp)
cut -f1 "$incomplete" "$defective" | grep -v '^#' | grep -v '^$' | sort -u > "$excluded"
weights=$(mktemp)
index=0
skipped=0
while IFS= read -r dir; do
  if grep -Fxq "$dir" "$excluded"; then
    skipped=$((skipped + 1))
    continue
  fi
  rec=
  for c in records.jsonl records.jsonl.gz ecbench_records.jsonl.gz; do
    if [[ -f "$dir/$c" ]]; then rec="$dir/$c"; break; fi
  done
  weight=0
  if [[ -n "$rec" ]]; then
    if [[ "$rec" == *.gz ]]; then reader=(gzip -dc "$rec"); else reader=(cat "$rec"); fi
    # milliseconds of recorded solver wall time over the verified records
    weight=$("${reader[@]}" | { grep -F '"status":"verified"' || true; } | { grep -oP '"solve_wall_ns":\K[0-9]+' || true; } \
      | awk '{ s += $1 } END { printf "%d", s / 1e6 }')
  fi
  if [[ "${weight:-0}" -le 0 ]]; then
    weight=$(find "$dir" -maxdepth 1 -type f -printf '%s %f\n' | awk '{ b += ($2 ~ /\.gz$/) ? 8 * $1 : $1 } END { printf "%d", b / 1e6 + 1 }')
  fi
  printf '%s\t%s\t%s\n' "$weight" "$index" "$dir" >> "$weights"
  index=$((index + 1))
done < "$list"
mine=$(mktemp)
replayed=0
while IFS= read -r dir; do
  printf '%s\0' "$dir" >> "$mine"
  replayed=$((replayed + 1))
done < <(sort -t $'\t' -k1,1nr -k2,2n "$weights" | awk -F '\t' -v n="$shard_n" -v k="$shard_k" '
  { best = 0; for (i = 1; i < n; i++) if (load[i] < load[best]) best = i
    load[best] += $1; if (best == k) print $3 }')
rm -f "$weights" "$excluded"
failed=$(mktemp)
export ecbench failed
echo "shard $shard: replaying $replayed of $index sessions ($skipped excluded: incomplete scratch or pinned defective)"
if [[ "$replayed" -gt 0 ]]; then
xargs -0 -P "${REPLAY_JOBS:-$(nproc)}" -n 1 bash -c '
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
' replay < "$mine"
fi
rm -f "$mine"
if [ -s "$failed" ]; then
  echo "::error::committed ecbench sessions that do not replay exactly on this host:" >&2
  cat "$failed" >&2
  rm -f "$failed"
  exit 1
fi
rm -f "$failed"
summary="shard $shard: $replayed of $index committed ecbench sessions replay every measured run exactly (--replay-all --allow-interrupted); $skipped excluded (hash-pinned incomplete scratch or pinned defective)"
echo "$summary"
if [[ -n "${GITHUB_STEP_SUMMARY:-}" ]]; then
  echo "$summary" >> "$GITHUB_STEP_SUMMARY"
fi
