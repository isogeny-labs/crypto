#!/bin/sh
set -eu
here=${NIST_FACTOR_BASE:-$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)}
repo=$(git -C "$here" rev-parse --show-toplevel)
: "${NIST_FACTOR_OUT:?set a fresh output directory under this follow-up runs}"
: "${PARI_PREFIX:=/workspace/tools/sage_skeleton_env}"
checkpoint() {
  if [ -n "${NIST_CHECKPOINT_WRAPPER:-}" ]; then
    "$NIST_CHECKPOINT_WRAPPER" "$repo/scripts/checkpoint.sh" "$@"
  else
    "$repo/scripts/checkpoint.sh" "$@"
  fi
}
case "$NIST_FACTOR_OUT" in "$here"/runs/*) ;; *) exit 1;; esac
[ ! -e "$NIST_FACTOR_OUT" ] || { echo 'output exists' >&2; exit 1; }
[ -s "$here/results/curve-20-plus.json" ] || { echo 'missing accepted final record' >&2; exit 1; }
checkpoint gate
mkdir -p "$NIST_FACTOR_OUT/source-snapshot" "$NIST_FACTOR_OUT/input-factors" "$NIST_FACTOR_OUT/input-features" "$NIST_FACTOR_OUT/empty-hints" "$NIST_FACTOR_OUT/features" "$NIST_FACTOR_OUT/summary" "$NIST_FACTOR_OUT/search" "$NIST_FACTOR_OUT/bin"
cp "$0" "$here/FULL84_REPLAY_PROTOCOL.md" "$here/summarize_factors.cpp" "$here/../native/extend_features.cpp" "$here/../native/search.cpp" "$here/../native/common.hpp" "$here/../inputs/historical.tsv" "$NIST_FACTOR_OUT/source-snapshot/"
cp "$here/runs/full21-features-003/input-factors/"*.json "$NIST_FACTOR_OUT/input-factors/"
cp "$here/results/curve-20-plus.json" "$NIST_FACTOR_OUT/input-factors/"
cp "$here/runs/full21-features-003/features/"*.features.tsv "$NIST_FACTOR_OUT/input-features/"
for unit in extend_features summarize_factors; do
  c++ -std=c++17 -O2 -Wall -Wextra -Werror -I"$PARI_PREFIX/include" -I"$NIST_FACTOR_OUT/source-snapshot" "$NIST_FACTOR_OUT/source-snapshot/$unit.cpp" -L"$PARI_PREFIX/lib" -Wl,-rpath,"$PARI_PREFIX/lib" -lpari -lcrypto -o "$NIST_FACTOR_OUT/bin/$unit"
done
c++ -std=c++17 -O2 -Wall -Wextra -Werror "$NIST_FACTOR_OUT/source-snapshot/search.cpp" -o "$NIST_FACTOR_OUT/bin/search"
cd "$NIST_FACTOR_OUT"
sha256sum source-snapshot/* input-factors/* input-features/* bin/* > native-input.sha256
checkpoint push
run_native() {
  label=$1
  shift
  set +e
  "$@" > "$label.stdout" 2> "$label.stderr"
  status=$?
  set -e
  printf '%s\n' "$status" > "$label.status"
  if [ "$status" -ne 0 ]; then checkpoint push; exit "$status"; fi
}
run_native controls ./bin/search --self-test
run_native features ./bin/extend_features source-snapshot/historical.tsv input-features input-factors empty-hints features 21
if ! rg -q '^plus: complete for entire cohort = 1$' features.stderr; then
  printf 'plus family is not complete across all21 controls\n' > family-admission.stderr
  checkpoint push
  exit 1
fi
run_native summary ./bin/summarize_factors source-snapshot/historical.tsv input-factors summary
checkpoint push
run_native search ./bin/search source-snapshot/historical.tsv features search
checkpoint push
printf 'Native full84 replay completed; canonical publication still requires inspection.\n'
