#!/usr/bin/env bash
# Storage metadata audit, not an S3 download or mathematical verifier.
set -euo pipefail
here=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
cd "$here"
count=0
while read -r expected_hash expected_bytes file; do
    if [ -f "$file" ]; then
        actual_hash=$(sha256sum "$file" | cut -d ' ' -f 1)
        actual_bytes=$(wc -c < "$file")
    else
        manifest="$(dirname "$file")/.artifacts.json"
        [ -f "$manifest" ] || { printf 'missing payload and pointer: %s\n' "$file" >&2; exit 1; }
        key=$(basename "$file")
        actual_hash=$(jq -er --arg key "$key" '.files[$key].sha256' "$manifest")
        actual_bytes=$(jq -er --arg key "$key" '.files[$key].bytes' "$manifest")
    fi
    [ "$actual_hash" = "$expected_hash" ] && [ "$actual_bytes" = "$expected_bytes" ] \
        || { printf 'stored binding mismatch: %s\n' "$file" >&2; exit 1; }
    count=$((count + 1))
done < stored-artifact-bindings.txt
printf 'PASS %s stored bindings; local bytes or pointer metadata checked, no S3 readback\n' "$count"
