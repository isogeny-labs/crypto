#!/usr/bin/env bash
# Replay the frozen sentinel records (docs/ecbench/replay-sentinels.json) with
# `ecbench verify`, the same audit, comparator and legacy profiles as every
# other replay.  A pull request that changes what a registered method counts
# fails here when a committed record of that method stops replaying.
#
# Usage:
#   ecbench-replay-sentinels.sh replay [MANIFEST] [ECBENCH]   pull + replay each sentinel
#   ecbench-replay-sentinels.sh scan LIST OUT_DIR [ECBENCH]   print candidates of every listed session
#                                                             (regenerating the manifest)
# `replay` expects the sentinel sessions hydrated (the workflow pulls them);
# SENTINEL_PULL=1 pulls them here with tools/artifacts/artifacts.py.
set -euo pipefail
mode=${1:?usage: ecbench-replay-sentinels.sh replay|scan ...}
shift

# A session committed with gzip-compressed records is replayed from a
# decompressed copy (the bytes are the committed ones, decompressed), as
# ecbench-audit-sessions.sh does.  Prints the directory to verify.
hydrated_copy() {
  local dir=$1 tmp=$2 copy
  if [ -f "$dir/records.jsonl.gz" ] && [ ! -f "$dir/records.jsonl" ]; then
    copy="$tmp/$(echo "$dir" | tr / _)"
    rm -rf "$copy"; cp -r "$dir" "$copy"; gunzip "$copy"/*.gz
    echo "$copy"
  elif [ -f "$dir/ecbench_records.jsonl.gz" ] && [ ! -f "$dir/records.jsonl" ]; then
    copy="$tmp/$(echo "$dir" | tr / _)"
    rm -rf "$copy"; cp -r "$dir" "$copy"; gzip -dc "$dir/ecbench_records.jsonl.gz" > "$copy/records.jsonl"
    echo "$copy"
  else
    echo "$dir"
  fi
}

case "$mode" in
replay)
  manifest=${1:-docs/ecbench/replay-sentinels.json}
  ecbench=${2:-./target/release/ecbench}
  tmp=$(mktemp -d)
  trap 'rm -rf "$tmp"' EXIT
  test -s "$manifest" || { echo "::error::$manifest is missing" >&2; exit 1; }
  "$ecbench" sentinels plan --manifest "$manifest" > "$tmp/plan.tsv"
  total=$(wc -l < "$tmp/plan.tsv")
  if [ "${SENTINEL_PULL:-0}" = 1 ]; then
    cut -f1 "$tmp/plan.tsv" > "$tmp/sessions.txt"
    mapfile -t sessions < "$tmp/sessions.txt"
    python3 tools/artifacts/artifacts.py pull --jobs 16 "${sessions[@]}"
  fi
  failed=0; replays=0; start=$(date +%s)
  while IFS=$'\t' read -r dir seqs; do
    verify_dir=$(hydrated_copy "$dir" "$tmp")
    args=()
    for s in ${seqs//,/ }; do args+=(--replay-seq "$s"); done
    t0=$(date +%s.%N)
    if "$ecbench" verify --dir "$verify_dir" --replay 0 "${args[@]}" --allow-interrupted \
        --exit-code > "$tmp/receipt.json" 2> "$tmp/err.txt"; then
      status=ok
    else
      status=FAIL
    fi
    n=$(jq -r '[.replays[]?] | length' "$tmp/receipt.json" 2> /dev/null || echo 0)
    ok=$(jq -r '[.replays[]? | select(.reproduced)] | length' "$tmp/receipt.json" 2> /dev/null || echo 0)
    replays=$((replays + n))
    printf '%s %s (seq %s): %s of %s replays reproduced, %.1fs\n' "$status" "$dir" "$seqs" "$ok" "$n" \
      "$(awk -v a="$t0" -v b="$(date +%s.%N)" 'BEGIN { print b - a }')"
    if [ "$status" = FAIL ] || [ "$n" != "$ok" ]; then
      failed=$((failed + 1))
      echo "::error::sentinel replay failed for $dir (seq $seqs)" >&2
      jq -r '.problems[]?' "$tmp/receipt.json" 2> /dev/null | sed 's/^/    /' >&2 || true
      jq -r '.replays[]? | select(.reproduced | not) | "    seq \(.seq) \(.record_id): \(.detail)"' "$tmp/receipt.json" 2> /dev/null >&2 || true
      sed 's/^/    /' "$tmp/err.txt" | head -20 >&2
    fi
    if [ -n "${verify_dir##"$dir"}" ]; then rm -rf "$verify_dir"; fi
  done < "$tmp/plan.tsv"
  summary="replay sentinels: $total sessions, $replays records replayed, $failed sessions failing, $(( $(date +%s) - start )) s"
  echo "$summary"
  if [ -n "${GITHUB_STEP_SUMMARY:-}" ]; then echo "$summary" >> "$GITHUB_STEP_SUMMARY"; fi
  [ "$failed" -eq 0 ]
  ;;
scan)
  list=${1:?LIST}
  out=${2:?OUT_DIR}
  ecbench=${3:-./target/release/ecbench}
  mkdir -p "$out"
  tmp=$(mktemp -d)
  trap 'rm -rf "$tmp"' EXIT
  : > "$out/candidates.jsonl"
  while IFS= read -r dir; do
    if [ -d tools/artifacts ] && [ -f "$dir/.artifacts.json" ]; then
      python3 tools/artifacts/artifacts.py pull "$dir" > /dev/null 2>&1 || echo "pull failed: $dir" >&2
    fi
    [ -f "$dir/plan.json" ] || [ -f "$dir/plan.json.gz" ] || { echo "no plan: $dir" >&2; continue; }
    if [ ! -f "$dir/records.jsonl" ] && [ ! -f "$dir/records.jsonl.gz" ] && [ ! -f "$dir/ecbench_records.jsonl.gz" ]; then
      echo "no records: $dir" >&2; continue
    fi
    verify_dir=$(hydrated_copy "$dir" "$tmp")
    "$ecbench" sentinels scan --dir "$verify_dir" --label "$dir" >> "$out/candidates.jsonl" 2> "$tmp/err.txt" \
      || { echo "scan failed: $dir: $(head -c 200 "$tmp/err.txt")" >&2; }
    if [ "$verify_dir" != "$dir" ]; then rm -rf "$verify_dir"; fi
    # free what the pull fetched: the pointer keeps it recoverable
    # (never a tracked file: only what the pull fetched)
    if [ -f "$dir/.artifacts.json" ] && ! git ls-files --error-unmatch "$dir/records.jsonl" > /dev/null 2>&1; then
      rm -f "$dir/records.jsonl"
    fi
  done < "$list"
  ;;
*) echo "unknown mode $mode" >&2; exit 2 ;;
esac
