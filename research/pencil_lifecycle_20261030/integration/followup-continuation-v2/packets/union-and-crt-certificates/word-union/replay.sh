#!/bin/bash
set -euo pipefail
cd "$(dirname "$0")"
mkdir replay2
for m in WordUnionPotential WordUnionTransport WordUnionState WordUnionGlobal WordUnionControls WordUnionReadback ExpectedFailure; do
 set +e
 bash check.sh "$m" > "replay2/$m.driver" 2>&1
 rc=$?
 set -e
 printf '%s\n' "$rc" > "replay2/$m.exit"
 receipt=$(tail -1 "replay2/$m.driver")
 cp "$receipt/stdout" "replay2/$m.stdout"
 cp "$receipt/stderr" "replay2/$m.stderr"
 if [ "$m" = ExpectedFailure ]; then test "$rc" = 1; else test "$rc" = 0; fi
done
rustc --edition=2024 check.rs -o replay2/check-native
replay2/check-native replay2 > replay2/acceptance.json
cat replay2/acceptance.json
