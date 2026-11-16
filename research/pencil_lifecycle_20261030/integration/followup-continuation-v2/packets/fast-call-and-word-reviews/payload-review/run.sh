#!/bin/bash
set -euo pipefail
D=$(cd "$(dirname "$0")" && pwd)
export LEAN_SYSROOT=/var/tmp/pencil-same-support-lean-local-v1-wco_em6r/toolchain
export LEAN_PATH=$(cat "$D/lean-path.txt")
cd "$D/input"
for m in FastPreludeExpr FastPreludeDecode FastPreludeControls FastPreludeReadback FastPayloadWords FastPayloadPolynomial FastPayloadControls FastPayloadReadback ExpectedFailure; do
 set +e
 timeout 120 "$LEAN_SYSROOT/bin/lean" -j1 -M4096 -o "$D/input/$m.olean" "$D/input/$m.lean" > "$D/logs/$m.stdout" 2> "$D/logs/$m.stderr"
 rc=$?
 set -e
 printf '%s\n' "$rc" > "$D/logs/$m.exit"
 printf '%s %s\n' "$m" "$rc"
 if [ "$m" = ExpectedFailure ]; then test "$rc" = 1; else test "$rc" = 0; fi
done
for h in helpers-before unions-before sources-before toolchain; do sha256sum -c "$D/metadata/$h.sha256" > "$D/metadata/$h.check"; done
