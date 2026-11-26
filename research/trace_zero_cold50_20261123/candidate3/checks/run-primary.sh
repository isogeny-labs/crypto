#!/usr/bin/env bash
set -euo pipefail
cd /workspace/crypto
trace_root=research/trace_zero_cold50_20261123/candidate3
trace_binary="$trace_root/bin/elimination"
export TRACE_BASELINE_EXE="$PWD/$trace_root/bin/baseline"
export TRACE_INCUMBENT_EXE="$PWD/$trace_root/bin/incumbent"
export VALGRIND=/workspace/scratch/profile-tools/root/usr/bin/valgrind
export VALGRIND_LIB=/workspace/scratch/profile-tools/root/usr/libexec/valgrind
export RAYON_NUM_THREADS=1 OMP_NUM_THREADS=1
trace_start=$(cat "$trace_root/start.epoch")
trace_remaining=$((600-$(date -u +%s)+trace_start))
if [ "$trace_remaining" -le 0 ];then exit 8;fi
set +e
taskset -c 0 bash -c 'ulimit -v 2097152; exec timeout "$1" "$2" run "$3" "$4" "$5"' trace-primary "$trace_remaining" "$trace_binary" research/trace_zero_projected_20261115/inputs /workspace/scratch/trace-cold50-candidate3-primary "$trace_root/rows.jsonl" > "$trace_root/run.stdout" 2> "$trace_root/run.stderr"
trace_exit=$?
set -e
printf '%s\n' "$trace_exit" > "$trace_root/run.exit"
date -u +%Y-%m-%dT%H:%M:%SZ > "$trace_root/end.txt"
exit "$trace_exit"
