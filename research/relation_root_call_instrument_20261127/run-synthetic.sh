#!/bin/sh
set -eu
cd "$(dirname "$0")/../.."
s=research/relation_root_call_instrument_20261127
w=/dev/shm/relation-call-instrument-workers-20261127
test "$(cat "$s/raw/transport.exit")" = 0
test ! -e "$s/raw/fixture-neutral.exit"
sha256sum -c "$s/SOURCE_V2.sha256" > "$s/raw/synthetic-preflight.stdout"
export RUST_TEST_THREADS=1 RAYON_NUM_THREADS=1 OMP_NUM_THREADS=1 OPENBLAS_NUM_THREADS=1
step() {
  relation_synthetic_name=$1
  shift
  set +e
  taskset -c 0 timeout --signal=TERM --kill-after=10 1800 prlimit --as=2147483648 --fsize=134217728 -- "$@" > "$s/raw/$relation_synthetic_name.stdout" 2> "$s/raw/$relation_synthetic_name.stderr"
  relation_synthetic_exit=$?
  set -e
  printf '%s\n' "$relation_synthetic_exit" > "$s/raw/$relation_synthetic_name.exit"
  test "$relation_synthetic_exit" -eq 0
}
for scenario in neutral hypothetical20; do
  cohort=/dev/shm/relation-call-$scenario-20261127
  analysis=/dev/shm/relation-call-$scenario-analysis-20261127
  test ! -e "$cohort"
  test ! -e "$analysis"
  step "fixture-$scenario" "$w/root-supervised-fixture" /workspace/native-supervised-qualification-240 "$s/SYNTHETIC_INPUT_PINS.json" /workspace/flat-worker-matrix-281 /dev/shm/relation-call-transport-20261127 "$cohort" "$scenario"
  step "analyze-$scenario" "$w/root-supervised-analyze" "$cohort" "$cohort/monitor.jsonl" "$cohort/prospective-pins.json" "$analysis"
done
sha256sum -c "$s/SOURCE_V2.sha256" > "$s/raw/synthetic-postflight.stdout"
