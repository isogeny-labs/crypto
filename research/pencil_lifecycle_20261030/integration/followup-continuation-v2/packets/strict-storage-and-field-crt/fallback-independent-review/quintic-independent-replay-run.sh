#!/bin/bash
set -euo pipefail
D=$(cat /var/tmp/quintic-independent-review-path)
export LEAN_SYSROOT=/var/tmp/pencil-same-support-lean-local-v1-wco_em6r/toolchain
export LEAN_PATH=$(cat "$D/lean-path.txt")
cd "$D/input"
for m in ReductionStep ReductionRun ReductionEntry FallbackReduced ReductionExecutionControls ReductionExecutionReadback ExpectedFailure; do
 set +e
 timeout 120 "$LEAN_SYSROOT/bin/lean" -j1 -M4096 -o "$D/input/$m.olean" "$D/input/$m.lean" > "$D/logs/$m.stdout" 2> "$D/logs/$m.stderr"
 rc=$?
 set -e
 printf '%s\n' "$rc" > "$D/logs/$m.exit"
 printf '%s %s\n' "$m" "$rc"
 if [ "$m" = ExpectedFailure ]; then test "$rc" = 1; else test "$rc" = 0; fi
done
sha256sum -c "$D/before.sha256" > "$D/bindings-check.txt"
sha256sum -c "$D/toolchain.sha256" > "$D/toolchain-check.txt"
rustc --edition=2024 "$D/input/check.rs" -o "$D/check-native"
"$D/check-native" "$D/logs" > "$D/acceptance.json"
cat "$D/acceptance.json"
