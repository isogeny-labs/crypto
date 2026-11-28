#!/bin/sh
set -eu
cd "$(dirname "$0")/../.."
s=research/relation_canonical_scan_20261127
w=/dev/shm/relation-canonical-scan-workers-20261127
test -e "$s/CANDIDATE_PRECALL.json"
test ! -e "$s/raw/candidate.exit"
sha256sum -c "$s/CANDIDATE_SOURCE.sha256" > "$s/raw/candidate-preflight.stdout"
export RUST_TEST_THREADS=1 RAYON_NUM_THREADS=1 OMP_NUM_THREADS=1 OPENBLAS_NUM_THREADS=1
step() {
  relation_scan_name=$1
  shift
  set +e
  taskset -c 0 timeout --signal=TERM --kill-after=10 1200 prlimit --as=2147483648 --fsize=134217728 -- "$@" > "$s/raw/$relation_scan_name.stdout" 2> "$s/raw/$relation_scan_name.stderr"
  relation_scan_exit=$?
  set -e
  printf '%s\n' "$relation_scan_exit" > "$s/raw/$relation_scan_name.exit"
  test "$relation_scan_exit" -eq 0
}
step candidate-unit "$w/candidate-unit" --test-threads=1 --nocapture
step candidate "$w/candidate" /dev/shm/relation-canonical-scan-candidate-20261127
export RELATION_ROOT_INTEGRATION_OUT=/dev/shm/relation-canonical-scan-native-original-20261127
test ! -e "$RELATION_ROOT_INTEGRATION_OUT"
step native-original "$w/native-original" --test-threads=1 --nocapture
export RELATION_PAIRGRAPH_OUT=/dev/shm/relation-canonical-scan-native-pairgraph-20261127
test ! -e "$RELATION_PAIRGRAPH_OUT"
step native-pairgraph "$w/native-pairgraph" --test-threads=1 --nocapture
refs=research/relation_root_runtime_20261127/reference-inputs
pins=research/relation_root_runtime_20261127/REFERENCE_PINS.json
step original-audit /dev/shm/relation-root-workers-20261127/baseline-audit "$RELATION_ROOT_INTEGRATION_OUT" "$refs" "$pins" "$s/ORIGINAL_AUDIT.json"
step pairgraph-audit /dev/shm/relation-root-workers-20261127/pairgraph-audit "$RELATION_PAIRGRAPH_OUT" "$refs" "$pins" "$s/PAIRGRAPH_AUDIT.json"
sha256sum -c "$s/CANDIDATE_SOURCE.sha256" > "$s/raw/candidate-postflight.stdout"
