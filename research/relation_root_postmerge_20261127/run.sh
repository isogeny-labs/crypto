#!/bin/sh
set -eu
cd "$(dirname "$0")/../.."
s=research/relation_root_postmerge_20261127
test "$(cat "$s/raw/build.exit")" = 0
test -e "$s/PRECALL.json"
test ! -e "$s/raw/original.exit"
sha256sum -c "$s/SOURCE.sha256" > "$s/raw/run-preflight.stdout"
export RUST_TEST_THREADS=1 RAYON_NUM_THREADS=1 OMP_NUM_THREADS=1 OPENBLAS_NUM_THREADS=1
for lane in original pairgraph; do
  if test "$lane" = original; then
    export RELATION_ROOT_INTEGRATION_OUT=/dev/shm/relation-postmerge-original-20261127
    test ! -e "$RELATION_ROOT_INTEGRATION_OUT"
  else
    export RELATION_PAIRGRAPH_OUT=/dev/shm/relation-postmerge-pairgraph-20261127
    test ! -e "$RELATION_PAIRGRAPH_OUT"
  fi
  set +e
  taskset -c 0 timeout --signal=TERM --kill-after=10 1200 prlimit --as=2147483648 --fsize=134217728 -- "/dev/shm/relation-root-workers-20261127/postmerge-$lane" --test-threads=1 --nocapture > "$s/raw/$lane.stdout" 2> "$s/raw/$lane.stderr"
  relation_postmerge_run_exit=$?
  set -e
  printf '%s\n' "$relation_postmerge_run_exit" > "$s/raw/$lane.exit"
  test "$relation_postmerge_run_exit" -eq 0
done
refs=research/relation_root_runtime_20261127/reference-inputs
pins=research/relation_root_runtime_20261127/REFERENCE_PINS.json
for lane in original pairgraph; do
  if test "$lane" = original; then
    auditor=/dev/shm/relation-root-workers-20261127/baseline-audit
  else
    auditor=/dev/shm/relation-root-workers-20261127/pairgraph-audit
  fi
  set +e
  taskset -c 0 timeout --signal=TERM --kill-after=10 1200 prlimit --as=2147483648 --fsize=134217728 -- "$auditor" "/dev/shm/relation-postmerge-$lane-20261127" "$refs" "$pins" "$s/$lane-AUDIT.json" > "$s/raw/$lane-audit.stdout" 2> "$s/raw/$lane-audit.stderr"
  relation_postmerge_run_exit=$?
  set -e
  printf '%s\n' "$relation_postmerge_run_exit" > "$s/raw/$lane-audit.exit"
  test "$relation_postmerge_run_exit" -eq 0
done
sha256sum -c "$s/SOURCE.sha256" > "$s/raw/run-postflight.stdout"
