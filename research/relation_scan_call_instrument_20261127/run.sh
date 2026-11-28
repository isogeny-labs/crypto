#!/bin/sh
set -eu
cd "$(dirname "$0")/../.."
s=research/relation_scan_call_instrument_20261127
w=/dev/shm/relation-scan-call-workers-20261127
test ! -e "$s/raw/unit-root-worker.exit"
sha256sum -c "$s/SOURCE_V2.sha256" > "$s/raw/run-preflight.stdout"
export RUST_TEST_THREADS=1 RAYON_NUM_THREADS=1 OMP_NUM_THREADS=1 OPENBLAS_NUM_THREADS=1 RUST_TEST_THREADS=1
step() {
  relation_scan_call_step=$1
  shift
  set +e
  taskset -c 0 timeout --signal=TERM --kill-after=10 1800 prlimit --as=2147483648 --fsize=134217728 -- "$@" > "$s/raw/$relation_scan_call_step.stdout" 2> "$s/raw/$relation_scan_call_step.stderr"
  relation_scan_call_exit=$?
  set -e
  printf '%s\n' "$relation_scan_call_exit" > "$s/raw/$relation_scan_call_step.exit"
  test "$relation_scan_call_exit" -eq 0
}
for name in root-worker root-transport-qualify root-supervised-compare root-supervised-analyze root-supervised-fixture; do
  step "unit-$name" "$w/$name-unit" --test-threads=1 --nocapture
done
step transport "$w/root-transport-qualify" "$w/root-worker" /workspace/fusion-qualification-66 "$s/SYNTHETIC_INPUT_PINS.json" /dev/shm/relation-scan-call-transport-20261127
sha256sum -c "$s/SOURCE_V2.sha256" > "$s/raw/run-postflight.stdout"
