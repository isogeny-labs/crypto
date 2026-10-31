#!/usr/bin/env bash
# Thin download/hash orchestration; no research arithmetic is delegated.
set -euo pipefail
study=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
destination=${1:?usage: bash verify-upstream.sh NEW_DOWNLOAD_DIRECTORY}
mkdir -- "$destination"
destination=$(CDPATH= cd -- "$destination" && pwd)
source_url=https://raw.githubusercontent.com/aburan28/ghs-trapdoor/b314cf87940cfb7199cd98261ba6d3910f275c90/checker/research/results
for name in quotient-reconstruction.json quotient-divisor-inputs.json quotient-divisor-verification.json symmetric-kummer-inversion-sage.json; do
  curl --fail --silent --show-error --location "$source_url/$name" --output "$destination/$name"
done
cd -- "$destination"
sha256sum --check "$study/reports/run-20261025T1845Z/input-hashes.sha256"
