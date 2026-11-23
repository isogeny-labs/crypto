#!/usr/bin/env bash
# Verify unchanged original payload hashes, including explicitly gzipped files.
set -euo pipefail
directory=${1:?provide package directory}
manifest=${2:?provide original manifest filename}
cd "$directory"
count=0
while read -r expected relative; do
  if [[ -f "$relative" ]]; then
    actual=$(sha256sum "$relative" | cut -d ' ' -f 1)
  elif [[ -f "$relative.gz" ]]; then
    actual=$(gzip -dc "$relative.gz" | sha256sum | cut -d ' ' -f 1)
  else
    printf 'Missing original payload: %s\n' "$relative" >&2
    exit 1
  fi
  [[ "$actual" == "$expected" ]] || { printf 'Hash mismatch: %s\n' "$relative" >&2; exit 1; }
  count=$((count+1))
done < "$manifest"
printf 'PASS all%s original payloads, including any decompressed files\n' "$count"
