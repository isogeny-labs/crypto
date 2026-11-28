#!/bin/bash
set -eu
relation_opt_study=$(cd "$(dirname "$0")" && pwd)
relation_opt_repo=$(cd "$relation_opt_study/../.." && pwd)
relation_opt_version=$1
test "$relation_opt_version" = current || test "$relation_opt_version" = historical
relation_opt_workers=/dev/shm/relation-opt-$relation_opt_version-workers-20261127
relation_opt_out=/dev/shm/relation-opt-$relation_opt_version-qualification-20261127
test ! -e "$relation_opt_out"
mkdir "$relation_opt_out"
export RUST_TEST_THREADS=1 RAYON_NUM_THREADS=1 OMP_NUM_THREADS=1 OPENBLAS_NUM_THREADS=1
relation_opt_step() {
  relation_opt_name=$1
  shift
  set +e
  taskset -c 0 timeout --signal=TERM --kill-after=10 1200 prlimit --as=2147483648 --fsize=134217728 -- "$@" > "$relation_opt_study/raw/$relation_opt_version-$relation_opt_name.stdout" 2> "$relation_opt_study/raw/$relation_opt_version-$relation_opt_name.stderr"
  relation_opt_exit=$?
  set -e
  printf '%s\n' "$relation_opt_exit" > "$relation_opt_study/raw/$relation_opt_version-$relation_opt_name.exit"
  test "$relation_opt_exit" -eq 0
}
relation_opt_qualifier=/dev/shm/relation-scan-call-workers-20261127/root-transport-qualify
if test "$relation_opt_version" = historical; then
  relation_opt_qualifier=/dev/shm/relation-call-instrument-workers-20261127/root-transport-qualify
fi
relation_opt_step unit-run "$relation_opt_workers/root-worker-unit" --test-threads=1 --nocapture
relation_opt_step transport "$relation_opt_qualifier" "$relation_opt_workers/root-worker" /workspace/fusion-qualification-66 "$relation_opt_study/${relation_opt_version^^}_INPUT_PINS.json" "$relation_opt_out/transport"
relation_opt_step native-original-run env RELATION_ROOT_INTEGRATION_OUT="$relation_opt_out/native-original" "$relation_opt_workers/native-original" --test-threads=1 --nocapture
relation_opt_step native-pairgraph-run env RELATION_PAIRGRAPH_OUT="$relation_opt_out/native-pairgraph" "$relation_opt_workers/native-pairgraph" --test-threads=1 --nocapture
relation_opt_refs="$relation_opt_repo/research/relation_root_runtime_20261127/reference-inputs"
relation_opt_refpins="$relation_opt_repo/research/relation_root_runtime_20261127/REFERENCE_PINS.json"
relation_opt_step original-audit /dev/shm/relation-root-workers-20261127/baseline-audit "$relation_opt_out/native-original" "$relation_opt_refs" "$relation_opt_refpins" "$relation_opt_study/${relation_opt_version^^}_ORIGINAL_AUDIT.json"
relation_opt_step pairgraph-audit /dev/shm/relation-root-workers-20261127/pairgraph-audit "$relation_opt_out/native-pairgraph" "$relation_opt_refs" "$relation_opt_refpins" "$relation_opt_study/${relation_opt_version^^}_PAIRGRAPH_AUDIT.json"
