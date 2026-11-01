#!/usr/bin/env bash
set -euo pipefail
root=$(git rev-parse --show-toplevel)
cd "$root"
study=research/quadratic_twist_evidence_20261021
validation=$study/validation/cm8-transfer-20261025
canonical=$study/native/cm8-transfer-20261025/cases
out=${1:?usage: restore-runs.sh FRESH_DIRECTORY}
test ! -e "$out"
mkdir -p "$out/runs/final/controls" "$out/runs/first"
out=$(cd "$out" && pwd)
cp -a "$canonical" "$out/runs/final/cases"
cp -a "$canonical" "$out/runs/first/cases"
for control in false-map false-subgroup false-source missing-arm missing-target false-degree false-witness false-row false-log missing-query false-scalar; do
 cp -a "$canonical" "$out/runs/final/controls/$control"
done
tar -xJf "$validation/runs.tar.xz" -C "$out"
(cd "$out" && sha256sum -c "$root/$validation/RUNS.sha256")
printf '%s\n' 'PASS: every original raw record and altered payload restored byte-for-byte'
