#!/bin/sh
set -eu
cd "$(dirname "$0")/../.."
s=research/relation_root_runtime_20261127
test ! -e "$s/raw/pruning-v2-unit.exit"
test ! -e /dev/shm/joint-pruning-v2-matrix-20261127
sha256sum -c "$s/PRUNING_V2_SOURCE.sha256" > "$s/raw/pruning-v2-run-preflight.stdout"
export RAYON_NUM_THREADS=1 OMP_NUM_THREADS=1 OPENBLAS_NUM_THREADS=1
set +e
taskset -c 0 timeout --signal=TERM --kill-after=10 1200 prlimit --as=2147483648 --fsize=134217728 -- /dev/shm/relation-root-workers-20261127/pruning-v2-unit --test-threads=1 --nocapture > "$s/raw/pruning-v2-unit.stdout" 2> "$s/raw/pruning-v2-unit.stderr"
relation_v2_exit=$?
set -e
printf '%s\n' "$relation_v2_exit" > "$s/raw/pruning-v2-unit.exit"
test "$relation_v2_exit" -eq 0
for mode in scan replay; do
set +e
taskset -c 0 timeout --signal=TERM --kill-after=10 1200 prlimit --as=2147483648 --fsize=134217728 -- /dev/shm/relation-root-workers-20261127/pruning-v2 "$mode" "$s/PRUNING_INPUTS.json" /dev/shm/joint-pruning-v2-matrix-20261127 > "$s/raw/pruning-v2-$mode.stdout" 2> "$s/raw/pruning-v2-$mode.stderr"
relation_v2_exit=$?
set -e
printf '%s\n' "$relation_v2_exit" > "$s/raw/pruning-v2-$mode.exit"
test "$relation_v2_exit" -eq 0
done
sha256sum -c "$s/PRUNING_V2_SOURCE.sha256" > "$s/raw/pruning-v2-run-postflight.stdout"
