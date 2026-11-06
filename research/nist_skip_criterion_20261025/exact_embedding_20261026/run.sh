#!/bin/sh
set -eu
here=${NIST_ORDER_BASE:-$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)}
repo=$(git -C "$here" rev-parse --show-toplevel)
: "${NIST_ORDER_OUT:?set a fresh path under this study runs}"
: "${PARI_PREFIX:=/workspace/tools/sage_skeleton_env}"
: "${NIST_ORDER_FACTORS:=$here/../factor_followup_20261026/runs/full84-replay-016/input-factors}"
checkpoint() {
  if [ -n "${NIST_CHECKPOINT_WRAPPER:-}" ]; then
    "$NIST_CHECKPOINT_WRAPPER" "$repo/scripts/checkpoint.sh" "$@"
  else
    "$repo/scripts/checkpoint.sh" "$@"
  fi
}
case "$NIST_ORDER_OUT" in "$here"/runs/*) ;; *) exit 1 ;; esac
[ ! -e "$NIST_ORDER_OUT" ] || { echo 'output already exists' >&2; exit 1; }
checkpoint gate
mkdir -p "$NIST_ORDER_OUT/source-snapshot" "$NIST_ORDER_OUT/input-factors" "$NIST_ORDER_OUT/bin" "$NIST_ORDER_OUT/degrees"
cp "$0" "$here/PROTOCOL.md" "$here/order_common.hpp" "$here/compute.cpp" "$here/verify.cpp" "$here/../native/common.hpp" "$here/../inputs/historical.tsv" "$NIST_ORDER_OUT/source-snapshot/"
cp "$NIST_ORDER_FACTORS"/*-minus.json "$NIST_ORDER_OUT/input-factors/"
cd "$NIST_ORDER_OUT"
run_native() {
  label=$1
  shift
  set +e
  timeout 300s "$@" > "$label.stdout" 2> "$label.stderr"
  status=$?
  set -e
  printf '%s\n' "$status" > "$label.status"
  if [ "$status" -ne 0 ]; then checkpoint push; exit "$status"; fi
}
for unit in compute verify; do
  run_native "build-$unit" c++ -std=c++17 -O2 -Wall -Wextra -Werror -I"$PARI_PREFIX/include" -Isource-snapshot "source-snapshot/$unit.cpp" -L"$PARI_PREFIX/lib" -Wl,-rpath,"$PARI_PREFIX/lib" -lpari -lcrypto -o "bin/$unit"
done
sha256sum source-snapshot/* input-factors/* bin/* > native-input.sha256
checkpoint push
run_native compute-controls ./bin/compute --self-test
run_native verifier-controls ./bin/verify --self-test
run_native compute ./bin/compute source-snapshot/historical.tsv input-factors degrees
run_native verify ./bin/verify source-snapshot/historical.tsv input-factors degrees verification.json
checkpoint push
printf 'Exact all21 order computation and separate direct-power verification completed.\n'
