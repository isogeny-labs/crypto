#!/bin/sh
set -eu
here=${NIST_SEA_BASE:-$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)}
repo=$(git -C "$here" rev-parse --show-toplevel)
: "${NIST_SEA_OUT:?set a fresh output directory under this study's runs}"
: "${NIST_SEA_BIN:=/workspace/nist-four-target-followup/replay_sea}"
: "${NIST_SEA_FEATURES:=$here/../factor_followup_20261026/runs/full21-features-003/features}"
checkpoint() {
    if [ -n "${NIST_CHECKPOINT_WRAPPER:-}" ]; then
        "$NIST_CHECKPOINT_WRAPPER" "$repo/scripts/checkpoint.sh" "$@"
    else
        "$repo/scripts/checkpoint.sh" "$@"
    fi
}
case "$NIST_SEA_OUT" in "$here"/runs/*) ;; *) exit 1;; esac
[ ! -e "$NIST_SEA_OUT" ] || { echo 'refusing existing output' >&2; exit 1; }
checkpoint gate
mkdir -p "$NIST_SEA_OUT/a" "$NIST_SEA_OUT/b" "$NIST_SEA_OUT/source-snapshot" "$NIST_SEA_OUT/input-features"
cp "$here/PROTOCOL.md" "$here/replay.cpp" "$0" "$NIST_SEA_OUT/source-snapshot/"
cp "$here/../native/common.hpp" "$here/../inputs/historical.tsv" "$NIST_SEA_OUT/source-snapshot/"
cp "$NIST_SEA_BIN" "$NIST_SEA_OUT/replay-native"
for source in "$NIST_SEA_FEATURES"/curve-*.features.tsv; do
    cp "$source" "$NIST_SEA_OUT/input-features/"
done
sha256sum "$NIST_SEA_BIN" "$here/replay.cpp" "$here/PROTOCOL.md" "$here/../inputs/historical.tsv" > "$NIST_SEA_OUT/native-input.sha256"
checkpoint push
set +e
"$NIST_SEA_BIN" --self-test > "$NIST_SEA_OUT/controls.stdout" 2> "$NIST_SEA_OUT/controls.stderr"
status=$?
set -e
printf '%s\n' "$status" > "$NIST_SEA_OUT/controls.status"
if [ "$status" -ne 0 ]; then checkpoint push; exit "$status"; fi
for arm in a b; do
    set +e
    "$NIST_SEA_BIN" "$here/../inputs/historical.tsv" "$NIST_SEA_OUT/input-features" "$NIST_SEA_OUT/$arm" \
        > "$NIST_SEA_OUT/$arm.stdout" 2> "$NIST_SEA_OUT/$arm.stderr"
    status=$?
    set -e
    printf '%s\n' "$status" > "$NIST_SEA_OUT/$arm.status"
    if [ "$status" -ne 0 ]; then checkpoint push; exit "$status"; fi
done
set +e
cmp "$NIST_SEA_OUT/a/coverage.tsv" "$NIST_SEA_OUT/b/coverage.tsv" && \
cmp "$NIST_SEA_OUT/a/summary.json" "$NIST_SEA_OUT/b/summary.json"
status=$?
set -e
printf '%s\n' "$status" > "$NIST_SEA_OUT/determinism.status"
checkpoint push
exit "$status"
