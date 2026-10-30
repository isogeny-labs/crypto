#!/usr/bin/env bash
# File-inventory metadata only. Raw records have the separate frozen manifest.
set -euo pipefail
here=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
tmp=$(mktemp)
stored=$(mktemp)
trap 'rm -f "$tmp" "$stored"' EXIT
cd "$here"
awk '{print "./" $3}' stored-artifact-bindings.txt > "$stored"
find . -type f \
    ! -path '*/.lake/*' ! -path '*/target/*' \
    ! -path './packet-unit/records/*' ! -path './packet-unit/mode-records/*' \
    ! -path './onelevel-unit/onelevel' ! -path './onelevel-unit/onelevel.run-001' \
    ! -path './onelevel-unit/run-002/onelevel' ! -path './onelevel-unit/run-003/onelevel' \
    ! -name 'source-manifest.sha256' ! -name '.artifacts.json' ! -name '.gitignore' \
    ! -name '*.tsv' ! -name '*.png' -print0 \
    | LC_ALL=C sort -z | while IFS= read -r -d '' file; do
        if grep -Fxq -- "$file" "$stored"; then continue; fi
        sha256sum "$file"
      done > "$tmp"
cp "$tmp" source-manifest.sha256
sha256sum --quiet -c source-manifest.sha256
printf 'PASS source manifest: %s files\n' "$(wc -l < source-manifest.sha256)"
