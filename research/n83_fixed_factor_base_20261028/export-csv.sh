#!/bin/bash
# Thin format-conversion driver. The native reader decodes the point records.
set -euo pipefail
[[ $# == 3 ]] || { echo 'Usage: export-csv.sh NATIVE_BINARY FAMILY OUTPUT.csv' >&2; exit 2; }
binary=$1
family=$2
output=$3
case "$family" in koblitz|binary|prime) ;; *) echo 'Unknown family' >&2; exit 2;; esac
here=$(cd "$(dirname "$0")" && pwd)
[[ ! -e "$output" ]] || { echo 'Output already exists' >&2; exit 2; }
parts=("$here/data/$family"/part-*.bin.zst)
[[ ${#parts[@]} == 128 ]] || { echo 'Hydrate all128 shards first' >&2; exit 2; }
printf 'x,y\n' > "$output"
for part in "${parts[@]}"; do
  "$binary" csv "$part" | sed '1d' >> "$output"
done
