#!/bin/sh
set -eu
here=${NIST_FACTOR_BASE:-$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)}
repo=$(git -C "$here" rev-parse --show-toplevel)
: "${NIST_FACTOR_OUT:?set a fresh output directory under this follow-up runs}"
: "${NIST_NFS_PREVIOUS:=$here/runs/yafu-nfs-014}"
: "${NIST_REFINER:=/workspace/nist-four-target-followup/refine_factor3}"
: "${NIST_VERIFIER:=/workspace/skip-factor-hints/verify_factor_hint}"
checkpoint() {
  if [ -n "${NIST_CHECKPOINT_WRAPPER:-}" ]; then
    "$NIST_CHECKPOINT_WRAPPER" "$repo/scripts/checkpoint.sh" "$@"
  else
    "$repo/scripts/checkpoint.sh" "$@"
  fi
}
case "$NIST_FACTOR_OUT" in "$here"/runs/*) ;; *) exit 1;; esac
case "$NIST_NFS_PREVIOUS" in "$here"/runs/*) ;; *) exit 1;; esac
[ ! -e "$NIST_FACTOR_OUT" ] || { echo 'output exists' >&2; exit 1; }
[ -f "$NIST_NFS_PREVIOUS/run.status" ] || { echo 'backend is not closed' >&2; exit 1; }
[ -x "$NIST_REFINER" ] && [ -x "$NIST_VERIFIER" ]
checkpoint gate
mkdir -p "$NIST_FACTOR_OUT/source-snapshot"
cp "$0" "$here/FACTORING_PROTOCOL.md" "$here/refine_factor.cpp" "$here/../native/verify_factor_hint.cpp" "$here/../native/common.hpp" "$here/../inputs/historical.tsv" "$NIST_FACTOR_OUT/source-snapshot/"
cp "$here/partial_hints/curve-20-plus.json" "$NIST_FACTOR_OUT/source-snapshot/partial-full-target-hint.json"
cp "$NIST_NFS_PREVIOUS/run.stdout" "$NIST_NFS_PREVIOUS/run.status" "$NIST_FACTOR_OUT/"
cp "$NIST_REFINER" "$NIST_FACTOR_OUT/refine-native"
cp "$NIST_VERIFIER" "$NIST_FACTOR_OUT/verify-native"
rg '^(PRP|P|C|U)[0-9]+ = [0-9]+$' "$NIST_FACTOR_OUT/run.stdout" > "$NIST_FACTOR_OUT/backend-factor-proposals.log"
cd "$NIST_FACTOR_OUT"
sha256sum refine-native verify-native source-snapshot/* run.stdout run.status backend-factor-proposals.log > native-input.sha256
checkpoint push
set +e
./refine-native source-snapshot/partial-full-target-hint.json backend-factor-proposals.log refined-full-target-hint.json > refinement.stdout 2> refinement.stderr
status=$?
set -e
printf '%s\n' "$status" > refinement.status
if [ "$status" -ne 0 ]; then checkpoint push; exit "$status"; fi
set +e
./verify-native source-snapshot/historical.tsv 20 plus refined-full-target-hint.json > admission.json 2> admission.stderr
status=$?
set -e
printf '%s\n' "$status" > admission.status
if [ "$status" -eq 0 ]; then
  if [ -e "$here/results/curve-20-plus.json" ]; then
    cmp admission.json "$here/results/curve-20-plus.json"
  else
    cp admission.json "$here/results/curve-20-plus.json"
  fi
fi
checkpoint push
exit "$status"
