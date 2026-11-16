#!/usr/bin/env bash
# Restore exact raw records, hydrating duplicate case copies from canonical Git files.
set -euo pipefail
root=$(git rev-parse --show-toplevel)
cd "$root"
study=research/quadratic_twist_evidence_20261021
validation=$study/validation/signed-cache-20261025
canonical=$study/native/signed-cache-20261025/cases
out=${1:?usage: restore-runs.sh FRESH_DIRECTORY}
test ! -e "$out"
mkdir -p "$out/runs/final/controls"
out=$(cd "$out" && pwd)
cp -a "$canonical" "$out/runs/final/cases"
for control in false-sign false-point missing-entry false-demand false-source missing-failed-query false-recovery; do
  cp -a "$canonical" "$out/runs/final/controls/$control"
done
tar -xJf "$validation/runs.tar.xz" -C "$out"
(cd "$out" && sha256sum -c "$root/$validation/RUNS.sha256")
printf '%s\n' 'PASS: every original raw file and precise corrupted payload restored byte-for-byte'
