#!/bin/sh
set -eu
cd "$(dirname "$0")/../.."
s=research/relation_f4_syzygy_pairs_20261127
bin=/dev/shm/relation-root-build-20261127/release/relation-f4-syzygy-pairs
export RAYON_NUM_THREADS=1 OMP_NUM_THREADS=1 OPENBLAS_NUM_THREADS=1 RUST_TEST_THREADS=1
sha256sum -c "$s/SOURCE.sha256" > "$s/raw/run-preflight.stdout"
sha256sum -c "$s/EXECUTABLE.sha256" > "$s/raw/worker-preflight.stdout"
taskset -c 0 timeout --signal=TERM --kill-after=10 60 prlimit --as=6442450944 --fsize=134217728 -- "$bin" prepare "$s/data" > "$s/raw/prepare.stdout" 2> "$s/raw/prepare.stderr"
find "$s/data" -type f -print0 | sort -z | xargs -0 sha256sum > "$s/PREPARED.sha256"
for p in 2147483647 9007199254740881; do
  for n in 3 4 6; do
    id=p$p-n$n
    set +e
    taskset -c 0 timeout --signal=TERM --kill-after=10 600 prlimit --as=6442450944 --fsize=134217728 -- "$bin" case "$s/data" "$id" > "$s/raw/$id.stdout" 2> "$s/raw/$id.stderr"
    relation_pair_case_exit=$?
    set -e
    printf '%s\n' "$relation_pair_case_exit" > "$s/raw/$id.exit"
  done
done
set +e
taskset -c 0 timeout --signal=TERM --kill-after=10 600 prlimit --as=6442450944 --fsize=134217728 -- "$bin" replay "$s/data" "$s/OUTCOME.json" > "$s/raw/replay.stdout" 2> "$s/raw/replay.stderr"
relation_pair_replay_exit=$?
set -e
printf '%s\n' "$relation_pair_replay_exit" > "$s/raw/replay.exit"
sha256sum -c "$s/SOURCE.sha256" > "$s/raw/run-postflight.stdout"
sha256sum -c "$s/PREPARED.sha256" > "$s/raw/prepared-postflight.stdout"
sha256sum -c "$s/EXECUTABLE.sha256" > "$s/raw/worker-postflight.stdout"
sha256sum -c research/relation_f4_consequence_trace_20261127/PAYLOAD.sha256 > "$s/raw/previous-evidence-postflight.stdout"
test "$relation_pair_replay_exit" -eq 0
