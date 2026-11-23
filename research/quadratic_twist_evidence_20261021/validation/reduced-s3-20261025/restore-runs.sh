#!/usr/bin/env bash
# File hydration/checksums only; mathematical replay is run-reduced-s3.sh.
set -euo pipefail
root=$(git rev-parse --show-toplevel)
study=$root/research/quadratic_twist_evidence_20261021
here=$study/validation/reduced-s3-20261025
out=${1:?usage: restore-runs.sh FRESH_OUTPUT_DIRECTORY}
test ! -e "$out"
mkdir -p "$out"
out=$(cd "$out" && pwd)
tar -xJf "$here/runs.tar.xz" -C "$out"
for relative in replay-final/cases replay-final/controls/false-remainder replay-final/controls/false-degree replay-final/controls/false-rank replay-final/controls/missing-case; do
  mkdir -p "$out/runs/$relative"
  cp -a -n "$study/native/reduced-s3-20261025/cases/." "$out/runs/$relative/"
done
rm "$out/runs/replay-final/controls/missing-case/p13-a0-b4-r7-size2-policy0.json"
cd "$out"
sha256sum -c "$here/RUNS.sha256"
