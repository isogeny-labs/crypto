#!/bin/sh
set -eu
here=${NIST_ECPP_BASE:-$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)}
repo=$(git -C "$here" rev-parse --show-toplevel)
: "${NIST_ECPP_OUT:?set fresh validation directory under this study runs}"
: "${NIST_ECPP_SAVED:=$here/runs/replay-004}"
: "${PARI_PREFIX:=/workspace/tools/sage_skeleton_env}"
checkpoint() {
  if [ -n "${NIST_CHECKPOINT_WRAPPER:-}" ]; then "$NIST_CHECKPOINT_WRAPPER" "$repo/scripts/checkpoint.sh" "$@";
  else "$repo/scripts/checkpoint.sh" "$@"; fi
}
case "$NIST_ECPP_OUT" in "$here"/runs/*) ;; *) exit 1 ;; esac
[ ! -e "$NIST_ECPP_OUT" ] || { echo 'output exists' >&2; exit 1; }
[ -s "$NIST_ECPP_SAVED/results/summary.json" ] || { echo 'complete saved pass required' >&2; exit 1; }
checkpoint gate
mkdir -p "$NIST_ECPP_OUT/source-snapshot" "$NIST_ECPP_OUT/bin" "$NIST_ECPP_OUT/controls" "$NIST_ECPP_OUT/cases" "$NIST_ECPP_OUT/results"
cp "$0" "$here/PROTOCOL.md" "$here/native.cpp" "$NIST_ECPP_SAVED/source-snapshot/common.hpp" "$NIST_ECPP_SAVED/source-snapshot/historical.tsv" "$NIST_ECPP_OUT/source-snapshot/"
cp "$NIST_ECPP_SAVED/cohort.tsv" "$NIST_ECPP_OUT/"
copy_case() {
  mkdir "$2"
  for period_file in input certificate.cformat prover.status prover.stdout checker.status checker.stdout; do cp "$1/$period_file" "$2/"; done
}
for period_control in prime101 prime65537 cm101; do copy_case "$NIST_ECPP_SAVED/controls/$period_control" "$NIST_ECPP_OUT/controls/$period_control"; done
period_index=0
while [ "$period_index" -lt 21 ]; do
  copy_case "$NIST_ECPP_SAVED/cases/curve-$period_index" "$NIST_ECPP_OUT/cases/curve-$period_index"
  period_index=$((period_index + 1))
done
cd "$NIST_ECPP_OUT"
find source-snapshot controls cases -type f -print0 | LC_ALL=C sort -z | xargs -0 sha256sum > saved-inputs.sha256
native() {
  period_phase=$1
  shift
  set +e
  (ulimit -c 0 && ulimit -v 786432 && timeout --kill-after=10s 300s "$@") > "$period_phase.stdout" 2> "$period_phase.stderr"
  period_status=$?
  set -e
  printf '%s\n' "$period_status" > "$period_phase.status"
  if [ "$period_status" -ne 0 ]; then checkpoint push; exit "$period_status"; fi
}
native build c++ -std=c++17 -O2 -Wall -Wextra -Werror -I"$PARI_PREFIX/include" -Isource-snapshot source-snapshot/native.cpp -L"$PARI_PREFIX/lib" -Wl,-rpath,"$PARI_PREFIX/lib" -lpari -lcrypto -o bin/native
sha256sum bin/native >> saved-inputs.sha256
for period_control in prime101 prime65537 cm101; do native "$period_control-admission" ./bin/native admit "$NIST_ECPP_OUT/controls/$period_control"; done
native binding-controls ./bin/native fixtures "$NIST_ECPP_OUT/controls/prime101" "$NIST_ECPP_OUT/controls"
checkpoint push
period_index=0
while [ "$period_index" -lt 21 ]; do
  period_directory="$NIST_ECPP_OUT/cases/curve-$period_index"
  native "parse-$period_index" ./bin/native parse-only "$period_directory"
  set +e
  (ulimit -c 0 && ulimit -v 786432 && timeout --kill-after=10s 300s ./bin/native admit "$period_directory") > "$period_directory/admission.stdout" 2> "$period_directory/admission.stderr"
  period_status=$?
  set -e
  printf '%s\n' "$period_status" > "$period_directory/admission.status"
  period_index=$((period_index + 1))
done
native saved-inputs-check sha256sum -c saved-inputs.sha256
native summary ./bin/native summarize "$NIST_ECPP_OUT" "$NIST_ECPP_OUT/results"
checkpoint push
printf 'Source-faithful saved-output validation completed; original checker failures retained.\n'
