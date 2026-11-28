#!/bin/sh
set -eu
cd "$(dirname "$0")/../.."
s=research/relation_scan_call_instrument_20261127
w=/dev/shm/relation-scan-call-workers-20261127
test "$(cat "$s/raw/analyze-neutral.exit")" = 0
test "$(cat "$s/raw/analyze-hypothetical20.exit")" = 0
test ! -e "$s/raw/canary-clock.exit"
export RUST_TEST_THREADS=1 RAYON_NUM_THREADS=1 OMP_NUM_THREADS=1 OPENBLAS_NUM_THREADS=1
for kind in clock duplicate; do
  cohort=/dev/shm/relation-scan-call-canary-$kind-20261127
  analysis=/dev/shm/relation-scan-call-canary-$kind-analysis-20261127
  test ! -e "$cohort"
  test ! -e "$analysis"
  taskset -c 0 timeout --signal=TERM --kill-after=10 1800 prlimit --as=2147483648 --fsize=134217728 -- "$w/root-supervised-fixture" --mutate /dev/shm/relation-scan-call-neutral-20261127 "$cohort" "$kind" > "$s/raw/mutate-$kind.stdout" 2> "$s/raw/mutate-$kind.stderr"
  set +e
  taskset -c 0 timeout --signal=TERM --kill-after=10 1800 prlimit --as=2147483648 --fsize=134217728 -- "$w/root-supervised-analyze" "$cohort" "$cohort/monitor.jsonl" "$cohort/prospective-pins.json" "$analysis" > "$s/raw/canary-$kind.stdout" 2> "$s/raw/canary-$kind.stderr"
  relation_canary_exit=$?
  set -e
  printf '%s\n' "$relation_canary_exit" > "$s/raw/canary-$kind.exit"
  test "$relation_canary_exit" -eq 1
  test ! -e "$analysis/analysis.json"
done
rg 'complete phase/boundary binding' "$s/raw/canary-clock.stderr"
rg 'missing/duplicate cell' "$s/raw/canary-duplicate.stderr"
sha256sum -c "$s/SOURCE_V2.sha256" > "$s/raw/canary-source-postflight.stdout"
