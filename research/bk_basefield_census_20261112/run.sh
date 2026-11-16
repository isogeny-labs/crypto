#!/usr/bin/env bash
# Local zero-paid checkpoint, exact computations only; never retry a finished call.
set -u
cd /workspace/crypto-audit/research/bk_basefield_census_20261112
mode=$1
case "$mode" in preflight) protocol=protocol-preflight.json ;; main) protocol=protocol.json ;; *) exit 2 ;; esac
python3 metadata.py admit "$mode" || exit 2
mkdir -p runs
mkdir "runs/$mode" || exit 2
native=/workspace/scratch/bk-base-native/census
failed=0
call() {
  local label=$1
  local cap=$2
  shift 2
  printf '%s\0' "$native" "$@" > "runs/$mode/$label.argv.nul"
  date -u +%Y-%m-%dT%H:%M:%SZ > "runs/$mode/$label.start"
  timeout --signal=INT --kill-after=5s "${cap}s" "$native" "$@" > "runs/$mode/$label.jsonl" 2> "runs/$mode/$label.stderr"
  local rc=$?
  printf '%s\n' "$rc" > "runs/$mode/$label.exit"
  date -u +%Y-%m-%dT%H:%M:%SZ > "runs/$mode/$label.end"
  if [ "$rc" -ne 0 ]; then failed=1; fi
}
if [ "$mode" = preflight ]; then
  call controls 15 --controls
  call negative 15 --controls
  call reference 30 --limit 14
  call aa 30 --limit 14
else
  call reference 900 --limit 500
  call aa 900 --limit 500
fi
bash analyze.sh "$mode"
arc=$?
printf '%s\n' "$arc" > "runs/$mode/analysis.exit"
python3 metadata.py export "$mode"
erc=$?
if [ "$mode" = preflight ]; then export_file=export-preflight.json; else export_file=export.json; fi
python3 /workspace/crypto-autoresearcher-current/tools/experiment_protocol.py check-export "$export_file" --protocol "$protocol" --verify-files > "evidence/$mode-export-check.log" 2>&1
vrc=$?
printf '%s\n' "$vrc" > "evidence/$mode-export-check.exit"
test "$failed" -eq 0 && test "$arc" -eq 0 && test "$erc" -eq 0 && test "$vrc" -eq 0
