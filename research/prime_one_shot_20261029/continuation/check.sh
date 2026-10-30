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
echo 'PASS continuation hashes, independent fixed records and retained staircase'
echo 'No scientific timing, asymptotic validation or sub-exponential solver established'
