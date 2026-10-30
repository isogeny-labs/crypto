#!/usr/bin/env bash
# File-integrity metadata only; no research computation.
set -euo pipefail
here=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
tmp=$(mktemp)
trap 'rm -f "$tmp"' EXIT
cd "$here"
find . -type f ! -path '*/.lake/*' ! -path '*/target/*' \
  ! -name 'source-manifest.sha256' ! -name '.artifacts.json' ! -name '.gitignore' \
  -print0 | LC_ALL=C sort -z | xargs -0 sha256sum > "$tmp"
cp "$tmp" source-manifest.sha256
sha256sum --quiet -c source-manifest.sha256
