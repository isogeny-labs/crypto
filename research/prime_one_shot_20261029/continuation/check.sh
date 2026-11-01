#!/usr/bin/env bash
# Frozen research-artifact audit and native record checks; no timing experiment.
set -euo pipefail
export LC_ALL=C
here=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
out=${1:-/tmp/prime-continuation-audit}
mkdir -p "$out"
(cd "$here" && sha256sum --quiet -c source-manifest.sha256)
(cd "$here/solver" && sha256sum --quiet -c files.sha256)
g++ -std=c++17 -O2 -Wall -Wextra -Wpedantic -Werror "$here/reviews/verify_records.cpp" -o "$out/verify_records"
"$out/verify_records" "$here/solver/checks"
bash "$here/canonical/unbound-check/check.sh" "$out/staircase"
bash "$here/tensor-review/check_hermite.sh" "$out/hermite"
echo 'PASS continuation hashes, independent fixed records, staircase and Hermite replay'
echo 'No scientific timing or original sub-exponential algorithm established by these checks'
