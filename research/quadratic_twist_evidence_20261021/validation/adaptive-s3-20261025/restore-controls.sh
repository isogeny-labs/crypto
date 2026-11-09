#!/usr/bin/env bash
# Reconstruct every exact rejected payload from the canonical cases and overlays.
set -euo pipefail
root=$(git rev-parse --show-toplevel)
cd "$root"
study=research/quadratic_twist_evidence_20261021
validation=$study/validation/adaptive-s3-20261025
canonical=$study/native/adaptive-s3-20261025/cases
out=${1:?usage: restore-controls.sh FRESH_DIRECTORY}
test ! -e "$out"
mkdir -p "$out"
out=$(cd "$out" && pwd)
for control in false-witness false-degree missing-target false-source false-recovery; do
  cp -a "$canonical" "$out/$control"
done
tar -xJf "$validation/controls-overlays.tar.xz" -C "$out"
(cd "$out" && sha256sum -c "$root/$validation/controls-full.sha256")
printf '%s\n' 'PASS: all five original corrupted directory trees restored byte-for-byte'
