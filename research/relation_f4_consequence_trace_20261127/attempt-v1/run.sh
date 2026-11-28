#!/bin/sh
set -eu
cd "$(dirname "$0")/../.."
s=research/relation_f4_consequence_trace_20261127
bin=/dev/shm/relation-root-build-20261127/release/relation-f4-consequence-trace
export RAYON_NUM_THREADS=1 OMP_NUM_THREADS=1 OPENBLAS_NUM_THREADS=1 RUST_TEST_THREADS=1
sha256sum -c "$s/SOURCE.sha256" > "$s/raw/run-preflight.stdout"
sha256sum -c "$s/EXECUTABLE.sha256" > "$s/raw/executable-preflight.stdout"
taskset -c 0 timeout --signal=TERM --kill-after=10 60 prlimit --as=6442450944 --fsize=134217728 -- "$bin" prepare research/relation_scan_call_instrument_20261127/SYNTHETIC_INPUT_PINS.json "$s/data" > "$s/raw/prepare.stdout" 2> "$s/raw/prepare.stderr"
find "$s/data" -type f -print0 | sort -z | xargs -0 sha256sum > "$s/PREPARED.sha256"
for p in 2147483647 9007199254740881; do
  for n in 3 4 6; do
    id=p$p-n$n
    set +e
    taskset -c 0 timeout --signal=TERM --kill-after=10 360 prlimit --as=6442450944 --fsize=134217728 -- "$bin" case "$s/data" "$id" > "$s/raw/$id.stdout" 2> "$s/raw/$id.stderr"
    relation_trace_case_exit=$?
    set -e
    printf '%s\n' "$relation_trace_case_exit" > "$s/raw/$id.exit"
  done
done
set +e
taskset -c 0 timeout --signal=TERM --kill-after=10 60 prlimit --as=6442450944 --fsize=134217728 -- "$bin" analyze "$s/data" "$s/OUTCOME.json" > "$s/raw/analyze.stdout" 2> "$s/raw/analyze.stderr"
relation_trace_analyze_exit=$?
set -e
printf '%s\n' "$relation_trace_analyze_exit" > "$s/raw/analyze.exit"
sha256sum -c "$s/PREPARED.sha256" > "$s/raw/prepared-postflight.stdout"
sha256sum -c "$s/SOURCE.sha256" > "$s/raw/run-postflight.stdout"
sha256sum -c "$s/EXECUTABLE.sha256" > "$s/raw/executable-postflight.stdout"
test "$relation_trace_analyze_exit" -eq 0
