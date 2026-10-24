#!/usr/bin/env bash
# Integrity-audit every committed ecbench session named in a list file (one
# directory per line), in parallel: hashes, seals, plan, answers, derived
# figures and grading, no replays (--replay 0).  An interrupted session,
# including one never finalised because its runner was killed, is accepted
# as evidence of the interruption (--allow-interrupted); every per-record
# check still applies.  Every failing session is named, then the script
# exits 1. Frozen scratch sessions without spec/host capsules are listed by
# hash and reported as incomplete evidence, never counted as audited passes.
# Usage: ecbench-audit-sessions.sh LIST [ECBENCH] [INCOMPLETE_TSV]
# AUDIT_SHARD=K/N audits only every N-th session from the K-th (0-based), so N
# jobs together audit the whole list; the incomplete-session pins are still
# checked against the whole list in every shard.
set -euo pipefail
shard=${AUDIT_SHARD:-0/1}
shard_k=${shard%/*}; shard_n=${shard#*/}
if [[ ! "$shard_k" =~ ^[0-9]+$ || ! "$shard_n" =~ ^[1-9][0-9]*$ || "$shard_k" -ge "$shard_n" ]]; then
  echo "::error::AUDIT_SHARD must be K/N with 0 <= K < N, got $shard" >&2; exit 1
fi
list=${1:?usage: ecbench-audit-sessions.sh LIST [ECBENCH] [INCOMPLETE_TSV]}
ecbench=${2:-./target/release/ecbench}
incomplete=${3:-docs/ecbench/INCOMPLETE_SESSIONS.tsv}
test -s "$list" || { echo "::error::$list is empty" >&2; exit 1; }
test -s "$incomplete" || { echo "::error::$incomplete is missing" >&2; exit 1; }
excluded_list=$(mktemp)
matched_list=$(mktemp)
while IFS=$'\t' read -r dir session_hash plan_hash records_hash extra; do
  [[ -z "$dir" || "$dir" == \#* ]] && continue
  if grep -Fxq "$dir" "$excluded_list" || [[ -n "${extra:-}" ]]; then
    echo "::error::duplicate or malformed incomplete-session row: $dir" >&2; exit 1
  fi
  for entry in "session.json:$session_hash" "plan.json:$plan_hash" "records.jsonl:$records_hash"; do
    name=${entry%%:*}; expected=${entry#*:}
    if [[ ! "$expected" =~ ^[0-9a-f]{64}$ || ! -f "$dir/$name" ]]; then
      echo "::error::missing or invalid pin for $dir/$name" >&2; exit 1
    fi
    actual=$(sha256sum "$dir/$name"); actual=${actual%% *}
    if [[ "$actual" != "$expected" ]]; then
      echo "::error::incomplete-session bytes changed: $dir/$name" >&2; exit 1
    fi
  done
  if [[ -e "$dir/spec.json" || -e "$dir/host.json" ]]; then
    echo "::error::incomplete-session exception needs review: $dir has a spec or host capsule" >&2; exit 1
  fi
  if ! jq -e '.schema == "ecbench.session/v1"' "$dir/session.json" > /dev/null; then
    echo "::error::incomplete-session entry is not ecbench: $dir" >&2; exit 1
  fi
  printf '%s\n' "$dir" >> "$excluded_list"
done < "$incomplete"

audit_list=$(mktemp)
audited=0
incomplete_count=0
index=0
while IFS= read -r dir; do
  if grep -Fxq "$dir" "$excluded_list"; then
    printf '%s\n' "$dir" >> "$matched_list"
    incomplete_count=$((incomplete_count + 1))
    echo "::warning::$dir is hash-pinned incomplete scratch evidence (spec.json and host.json absent)" >&2
  else
    if (( index % shard_n == shard_k )); then
      printf '%s\0' "$dir" >> "$audit_list"
      audited=$((audited + 1))
    fi
    index=$((index + 1))
  fi
done < "$list"
if ! cmp -s <(sort "$excluded_list") <(sort "$matched_list"); then
  echo "::error::incomplete-session pins and committed session list disagree" >&2
  diff -u <(sort "$excluded_list") <(sort "$matched_list") >&2 || true
  exit 1
fi
rm -f "$excluded_list" "$matched_list"
failed=$(mktemp)
export ecbench failed
# xargs runs one audit per session; a failure is recorded, not fatal, so
# every failing session is reported.
if [[ "$audited" -gt 0 ]]; then
xargs -0 -P "$(nproc)" -n 1 bash -c '
  out="$failed.$$"
  dir=$1
  # A session committed with gzip-compressed records.jsonl.gz / plan.json.gz
  # (too large to commit raw) is audited from a decompressed copy; every
  # check then runs on exactly the committed bytes, decompressed.
  if [ -f "$1/records.jsonl.gz" ] && [ ! -f "$1/records.jsonl" ]; then
    dir="$out.session"
    cp -r "$1" "$dir"
    gunzip "$dir"/*.gz
  fi
  if ! "$ecbench" verify --dir "$dir" --replay 0 --allow-interrupted --exit-code > "$out.json" 2> "$out.err"; then
    msg=$(echo "$1"; { jq -r ".problems[]" "$out.json" 2> /dev/null || true; cat "$out.err"; } | sed "s/^/    /")
    printf "%s\n" "$msg" >> "$failed"
  fi
  rm -rf "$out.json" "$out.err" "$out.session"
' audit < "$audit_list"
fi
rm -f "$audit_list"
if [ -s "$failed" ]; then
  echo "::error::committed ecbench sessions failing integrity:" >&2
  cat "$failed" >&2
  rm -f "$failed"
  exit 1
fi
rm -f "$failed"
echo "shard $shard: $audited of $index committed ecbench sessions pass integrity (--replay 0 --allow-interrupted); $incomplete_count hash-pinned scratch sessions remain incomplete"
