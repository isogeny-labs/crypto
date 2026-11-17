#!/usr/bin/env bash
set -euo pipefail
src=$(cd "$(dirname "$0")" && pwd)
dst=${1:?new scratch directory required}
test ! -e "$dst"
mkdir -p "$dst"
cp -a "$src/." "$dst/"
while IFS= read -r -d '' item; do mv "$item" "${item%.txt}"; done < <(find "$dst" -type f -name '*.lean.txt' -print0)
