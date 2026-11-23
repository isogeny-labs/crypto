#!/bin/bash
set -euo pipefail
cd "$(dirname "$0")"
mkdir replay1
for m in PegcdTotal PegcdStored PegcdEndpoint PegcdTotalControls PegcdTotalReadback ExpectedFailure; do
 set +e
 bash check.sh "$m" > "replay1/$m.driver" 2>&1
 rc=$?
 set -e
 printf '%s\n' "$rc" > "replay1/$m.exit"
 receipt=$(tail -1 "replay1/$m.driver")
 cp "$receipt/stdout" "replay1/$m.stdout"
 cp "$receipt/stderr" "replay1/$m.stderr"
 if [ "$m" = ExpectedFailure ]; then test "$rc" = 1; else test "$rc" = 0; fi
done
rustc --edition=2024 check.rs -o replay1/check-native
replay1/check-native replay1 > replay1/acceptance.json
cat replay1/acceptance.json
