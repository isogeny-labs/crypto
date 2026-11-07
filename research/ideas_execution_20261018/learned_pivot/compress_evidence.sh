#!/usr/bin/env bash
set -eu
cd /workspace/crypto
study=research/ideas_execution_20261018/learned_pivot
find "$study/runs/frozen-v1" -type f \( -name '*.rows' -o -name 'pivots.bin' -o -name 'trace.jsonl' \) -print | sort > "$study/compression.paths.txt"
while IFS= read -r path; do sha256sum "$path"; done < "$study/compression.paths.txt" > "$study/compression.files.sha256"
while IFS= read -r path; do gzip -1 "$path"; done < "$study/compression.paths.txt"
printf 'path\texpected_sha256\tobserved_sha256\tstatus\n' > "$study/compression-verification.tsv"
while read -r hash path; do observed=$(gzip -dc "$path.gz" | sha256sum); observed=${observed%% *}; test "$observed" = "$hash"; printf '%s\t%s\t%s\tPASS\n' "$path.gz" "$hash" "$observed" >> "$study/compression-verification.tsv"; done < "$study/compression.files.sha256"
