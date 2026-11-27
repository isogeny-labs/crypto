#!/usr/bin/env bash
set -euo pipefail
set -o noclobber
cd /workspace/crypto
trace_study="$(cd -- "$(dirname -- "$0")/.." && pwd)"
trace_binary="$trace_study/bin/elimination"
export TRACE_BASELINE_EXE="$trace_study/bin/baseline" TRACE_INCUMBENT_EXE="$trace_study/bin/incumbent"
export VALGRIND=/workspace/scratch/profile-tools/root/usr/bin/valgrind VALGRIND_LIB=/workspace/scratch/profile-tools/root/usr/libexec/valgrind RAYON_NUM_THREADS=1 OMP_NUM_THREADS=1
for trace_exe in "$TRACE_BASELINE_EXE" "$TRACE_INCUMBENT_EXE" "$trace_binary" "$trace_study/bin/preflight_guard"; do test -x "$trace_exe"; done
sha256sum -c "$trace_study/checks/executables.sha256"
trace_start=$(cat "$trace_study/start.epoch")
trace_remaining=$((600-$(date -u +%s)+trace_start))
test "$trace_remaining" -gt 0
taskset -c 0 bash -c 'ulimit -v 2097152; exec timeout "$1" "$2" calibrate "$3" "$4"' trace-cal "$trace_remaining" "$trace_binary" research/trace_zero_projected_20261115/inputs /workspace/scratch/cold10m-calibration > "$trace_study/calibration.json" 2> "$trace_study/calibration.stderr"
trace_remaining=$((600-$(date -u +%s)+trace_start))
test "$trace_remaining" -gt 0
taskset -c 0 bash -c 'ulimit -v 2097152; exec timeout "$1" "$2" pilot "$3" "$4"' trace-pilot "$trace_remaining" "$trace_binary" research/trace_zero_projected_20261115/inputs /workspace/scratch/cold10m-pilot > "$trace_study/preflight.json" 2> "$trace_study/preflight.stderr"
"$trace_study/bin/preflight_guard" "$trace_study"
