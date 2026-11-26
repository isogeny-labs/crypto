#!/usr/bin/env bash
set -euo pipefail
cd /workspace/crypto
trace_study=research/trace_zero_cold15m_20261124/candidate2
export TRACE_BASELINE_EXE="$PWD/$trace_study/bin/baseline" TRACE_INCUMBENT_EXE="$PWD/$trace_study/bin/incumbent"
export VALGRIND=/workspace/scratch/profile-tools/root/usr/bin/valgrind VALGRIND_LIB=/workspace/scratch/profile-tools/root/usr/libexec/valgrind RAYON_NUM_THREADS=1 OMP_NUM_THREADS=1
trace_start=$(cat "$trace_study/start.epoch")
trace_remaining=$((600-$(date -u +%s)+trace_start))
if [ "$trace_remaining" -le 0 ]; then exit 8; fi
set +e
taskset -c 0 bash -c 'ulimit -v 2097152; exec timeout "$1" "$2" run "$3" "$4" "$5"' trace-primary "$trace_remaining" "$trace_study/bin/elimination" research/trace_zero_projected_20261115/inputs /workspace/scratch/cold15m-candidate2-primary "$trace_study/rows.jsonl" > "$trace_study/run.stdout" 2> "$trace_study/run.stderr"
trace_status=$?
set -e
printf '%s\n' "$trace_status" > "$trace_study/run.exit"
date -u +%Y-%m-%dT%H:%M:%SZ > "$trace_study/end.txt"
exit "$trace_status"
