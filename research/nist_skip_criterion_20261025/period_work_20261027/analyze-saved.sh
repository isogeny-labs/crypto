#!/bin/sh
# Thin orchestration of one native analysis of existing packets.
set -eu
: "${PROFILE_BASE:?absolute study path}"
: "${PROFILE_BINARY:?frozen native binary}"
: "${PROFILE_OUT:?fresh run output}"
repo=$(git -C "$PROFILE_BASE" rev-parse --show-toplevel)
checkpoint() {
    if [ -n "${NIST_CHECKPOINT_WRAPPER:-}" ]; then "$NIST_CHECKPOINT_WRAPPER" "$repo/scripts/checkpoint.sh" "$@";
    else "$repo/scripts/checkpoint.sh" "$@"; fi
}
case "$PROFILE_OUT" in "$PROFILE_BASE"/runs/*) ;; *) exit 2 ;; esac
[ ! -e "$PROFILE_OUT" ]
[ "$(basename "$PROFILE_BINARY")" = profiler ]
checkpoint gate
(cd "$PROFILE_BASE" && sha256sum -c input.sha256)
(cd "$(dirname "$PROFILE_BINARY")" && sha256sum -c "$PROFILE_BASE/binary.sha256")
mkdir -p "$PROFILE_OUT/source-snapshot" "$PROFILE_OUT/bin"
cp "$PROFILE_BASE/PROTOCOL.md" "$PROFILE_BASE/plan.json" "$PROFILE_BASE/input.sha256" "$PROFILE_BASE/profiler.cpp" "$0" "$PROFILE_OUT/source-snapshot/"
cp "$PROFILE_BINARY" "$PROFILE_OUT/bin/profiler"
(cd "$PROFILE_OUT/bin" && sha256sum -c "$PROFILE_BASE/binary.sha256")
git -C "$repo" rev-parse HEAD > "$PROFILE_OUT/source-commit.txt"
printf '%s\n' 'profiler STUDY ../period_ecpp_20261026/runs/replay-004 FRESH_RESULTS; ulimit-v786432; timeout60s; no legacy invocation' > "$PROFILE_OUT/command.txt"
set +e
(ulimit -c 0 && ulimit -v 786432 && timeout --kill-after=5s 60s "$PROFILE_OUT/bin/profiler" "$PROFILE_BASE" "$PROFILE_BASE/../period_ecpp_20261026/runs/replay-004" "$PROFILE_OUT/results") > "$PROFILE_OUT/stdout" 2> "$PROFILE_OUT/stderr"
profile_status=$?
set -e
printf '%s\n' "$profile_status" > "$PROFILE_OUT/status"
(cd "$PROFILE_BASE" && sha256sum -c input.sha256) > "$PROFILE_OUT/input-recheck.txt"
checkpoint push
exit "$profile_status"
