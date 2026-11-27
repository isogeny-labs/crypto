#!/bin/sh
set -eu
cd "$(dirname "$0")/../.."
s=research/relation_root_runtime_20261127
test ! -e "$s/raw/canonical-original.exit"
sha256sum -c "$s/CANONICAL_SOURCE.sha256" > "$s/raw/canonical-run-preflight.stdout"
export RUST_TEST_THREADS=1 RAYON_NUM_THREADS=1 OMP_NUM_THREADS=1 OPENBLAS_NUM_THREADS=1
for lane in original pairgraph; do
  if test "$lane" = original; then
    export RELATION_ROOT_INTEGRATION_OUT=/dev/shm/root-native-canonical-original-20261127
    test ! -e "$RELATION_ROOT_INTEGRATION_OUT"
  else
    export RELATION_PAIRGRAPH_OUT=/dev/shm/root-native-pairgraph-canonical-20261127
    test ! -e "$RELATION_PAIRGRAPH_OUT"
  fi
  set +e
  taskset -c 0 timeout --signal=TERM --kill-after=10 1200 prlimit --as=2147483648 --fsize=134217728 -- "/dev/shm/relation-root-workers-20261127/canonical-$lane" --test-threads=1 --nocapture > "$s/raw/canonical-$lane.stdout" 2> "$s/raw/canonical-$lane.stderr"
  relation_canonical_run_exit=$?
  set -e
  printf '%s\n' "$relation_canonical_run_exit" > "$s/raw/canonical-$lane.exit"
  test "$relation_canonical_run_exit" -eq 0
done
set +e
taskset -c 0 timeout --signal=TERM --kill-after=10 1200 prlimit --as=2147483648 --fsize=134217728 -- /dev/shm/relation-root-workers-20261127/baseline-audit /dev/shm/root-native-canonical-original-20261127 /workspace/fusion-qualification-66 /workspace/relation-supervisor-live/research/relation_fusion_20261117/MATCHED_PINS_BEFORE_85.json "$s/CANONICAL_ORIGINAL_AUDIT.json" > "$s/raw/canonical-original-audit.stdout" 2> "$s/raw/canonical-original-audit.stderr"
relation_canonical_run_exit=$?
set -e
printf '%s\n' "$relation_canonical_run_exit" > "$s/raw/canonical-original-audit.exit"
test "$relation_canonical_run_exit" -eq 0
sha256sum -c "$s/CANONICAL_SOURCE.sha256" > "$s/raw/canonical-run-postflight.stdout"
