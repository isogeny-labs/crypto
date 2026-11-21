#!/usr/bin/env bash
set -euo pipefail
study=$(cd "$(dirname "$0")/.." && pwd)
case "$1" in control|boundary) ;; *) exit 2;; esac
task_case=$1
task_build=/workspace/scratch/msolve-degree-native-build
task_prefix="$study/evidence/native/$task_case"
[ ! -e "$task_prefix.exit" ]
mkdir -p "$study/evidence/native"
python3 /workspace/scratch/check-msolve-degree-predecessor.py > "$task_prefix.before-pins.json"
date -u +%FT%TZ > "$task_prefix.start.utc"
printf '%s\n' "$task_build/msolve -t 1 -l 2 -c 0 -v 0 -g 2 -s 8 -f inputs/$task_case.ms -o evidence/native/$task_case.out" > "$task_prefix.argv.txt"
set +e
LD_LIBRARY_PATH=/workspace/msolve-deps/root/usr/lib/x86_64-linux-gnu \
OMP_NUM_THREADS=1 ASAN_OPTIONS=detect_leaks=0:abort_on_error=1:symbolize=1 \
timeout 30s "$task_build/msolve" -t 1 -l 2 -c 0 -v 0 -g 2 -s 8 \
  -f "$study/inputs/$task_case.ms" -o "$task_prefix.out" \
  > "$task_prefix.stdout" 2> "$task_prefix.stderr"
task_rc=$?
printf '%s\n' "$task_rc" > "$task_prefix.exit"
date -u +%FT%TZ > "$task_prefix.end.utc"
"$task_build/analyze" "$task_case" "$study/inputs/$task_case.ms" "$task_prefix" \
  "$study/evidence/native" > "$task_prefix.analyzer.stdout" 2> "$task_prefix.analyzer.stderr"
task_analysis_rc=$?
printf '%s\n' "$task_analysis_rc" > "$task_prefix.analyzer.exit"
set -e
python3 /workspace/scratch/check-msolve-degree-predecessor.py > "$task_prefix.after-pins.json"
exit "$task_analysis_rc"
