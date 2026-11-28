#!/bin/sh
set -eu
cd "$(dirname "$0")/../.."
study=research/relation_root_runtime_20261127
test ! -e "$study/raw/native.exit"
test ! -e /dev/shm/root-native-matrix-20261127
sha256sum -c "$study/BUILD_SOURCE.sha256" > "$study/raw/native-source-preflight.stdout"
export RUST_TEST_THREADS=1 RAYON_NUM_THREADS=1 OMP_NUM_THREADS=1 OPENBLAS_NUM_THREADS=1
export RELATION_ROOT_INTEGRATION_OUT=/dev/shm/root-native-matrix-20261127
taskset -c 0 /dev/shm/relation-root-workers-20261127/baseline-integration --list > "$study/raw/native-roster.stdout"
set +e
taskset -c 0 timeout --signal=TERM --kill-after=10 1200 prlimit --as=2147483648 --fsize=134217728 -- /dev/shm/relation-root-workers-20261127/baseline-integration --test-threads=1 --nocapture > "$study/raw/native.stdout" 2> "$study/raw/native.stderr"
result=$?
set -e
printf '%s\n' "$result" > "$study/raw/native.exit"
sha256sum -c "$study/BUILD_SOURCE.sha256" > "$study/raw/native-source-postflight.stdout"
test "$result" = 0
set +e
taskset -c 0 timeout --signal=TERM --kill-after=10 1200 prlimit --as=2147483648 --fsize=134217728 -- /dev/shm/relation-root-workers-20261127/baseline-audit /dev/shm/root-native-matrix-20261127 /workspace/fusion-qualification-66 /workspace/relation-supervisor-live/research/relation_fusion_20261117/MATCHED_PINS_BEFORE_85.json "$study/BASELINE_AUDIT.json" > "$study/raw/native-audit.stdout" 2> "$study/raw/native-audit.stderr"
result=$?
set -e
printf '%s\n' "$result" > "$study/raw/native-audit.exit"
exit "$result"
