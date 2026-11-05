#!/usr/bin/env bash
set -euo pipefail
D=/var/tmp/strict-storage-independent-20261101
export LEAN_SYSROOT=/var/tmp/pencil-same-support-lean-local-v1-wco_em6r/toolchain
export LEAN_PATH="$(cat "$D/lean-path.txt")"
cd "$D/input"
for m in StrictStep StrictRun StrictEntry StrictFallback StrictControls StrictReadback StrictExpectedFailure; do
 set +e
 timeout 120 "$LEAN_SYSROOT/bin/lean" -j1 -M4096 -o "$m.olean" "$m.lean" > "$D/logs/$m.stdout" 2> "$D/logs/$m.stderr"
 code=$?
 set -e
 printf '%s\n' "$code" > "$D/logs/$m.exit"
 if [ "$m" != StrictExpectedFailure ] && [ "$code" != 0 ]; then exit "$code"; fi
 if [ "$m" = StrictExpectedFailure ] && [ "$code" != 1 ]; then exit 99; fi
done
sha256sum -c "$D/before.sha256" > "$D/bindings-check.txt" 2>&1
