#!/usr/bin/env bash
set -euo pipefail
D=/var/tmp/pencil-nonmonic-division-20261101-v1
out=$(mktemp -d "$D/final-replay-XXXXXXXX")
for m in NonmonicCoefficient NonmonicStep NonmonicLoop NonmonicExact NonmonicControls NonmonicStorage NonmonicWord NonmonicWordControls NonmonicReadback; do
 bash "$D/check.sh" "$m" > "$out/$m.log"
 tail -1 "$out/$m.log" >> "$out/attempts.txt"
done
printf '%s\n' "$out"
