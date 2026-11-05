#!/usr/bin/env bash
set -euo pipefail
D=/var/tmp/batch-inverse-independent-20261101
export LEAN_SYSROOT=/var/tmp/pencil-same-support-lean-local-v1-wco_em6r/toolchain
export LEAN_PATH="$(cat "$D/lean-path.txt")"
cd "$D/input"
for m in Wide Inversion FastBatchInverse FastBatchControls FastBatchReadback ExpectedFailure; do
 set +e
 timeout 120 "$LEAN_SYSROOT/bin/lean" -j1 -M4096 -o "$m.olean" "$m.lean" > "$D/logs/$m.stdout" 2> "$D/logs/$m.stderr"
 code=$?
 set -e
 printf '%s\n' "$code" > "$D/logs/$m.exit"
 if [ "$m" != ExpectedFailure ] && [ "$code" != 0 ]; then exit "$code"; fi
 if [ "$m" = ExpectedFailure ] && [ "$code" != 1 ]; then exit 99; fi
done
sha256sum -c "$D/before.sha256" > "$D/bindings-check.txt" 2>&1
