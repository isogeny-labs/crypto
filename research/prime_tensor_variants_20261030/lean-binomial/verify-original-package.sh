#!/usr/bin/env bash
set -euo pipefail
here=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
cd "$here"
count=0
while read -r expected file; do
    if [ -f "$file" ]; then
        actual=$(sha256sum "$file" | cut -d ' ' -f 1)
    elif [ -f "$file.gz" ]; then
        actual=$(gzip -dc "$file.gz" | sha256sum | cut -d ' ' -f 1)
    else
        printf 'missing original package payload: %s\n' "$file" >&2
        exit 1
    fi
    [ "$actual" = "$expected" ] || { printf 'hash mismatch: %s\n' "$file" >&2; exit 1; }
    count=$((count + 1))
done < PACKAGE_SHA256.txt
printf 'PASS %s original package hashes, including exact decompressed logs\n' "$count"
