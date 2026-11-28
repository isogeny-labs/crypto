#!/usr/bin/env bash
# Frozen native analysis: exact A/A bytes, independent SQLite reconstruction.
set -u
cd /workspace/crypto-audit/research/bk_basefield_census_20261112
mode=$1
case "$mode" in preflight|main) ;; *) exit 2 ;; esac
mkdir -p analysis
mkdir "analysis/$mode" || exit 2
checker=/workspace/scratch/bk-base-native/analyze
timeout --signal=INT --kill-after=5s 600s "$checker" "runs/$mode/reference.jsonl" "runs/$mode/aa.jsonl" native/queries.sql > "analysis/$mode/analysis.jsonl" 2> "analysis/$mode/analysis.stderr"
rc=$?
printf '%s\n' "$rc" > "analysis/$mode/analysis.exit"
negative_ok=0
if [ "$mode" = preflight ]; then
  timeout --signal=INT --kill-after=5s 120s "$checker" --negative-control "runs/$mode/reference.jsonl" "runs/$mode/aa.jsonl" native/queries.sql > "analysis/$mode/negative.jsonl" 2> "analysis/$mode/negative.stderr"
  nrc=$?
  printf '%s\n' "$nrc" > "analysis/$mode/negative.exit"
  if [ "$nrc" -ne 1 ] || ! rg -q 'CHECK constraint failed: ok=1' "analysis/$mode/negative.stderr"; then negative_ok=1; fi
fi
test "$rc" -eq 0 && test "$negative_ok" -eq 0
