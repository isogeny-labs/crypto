#!/usr/bin/env bash
# Lossless archive packaging only. Native recorded values are unchanged.
set -eu
cd /workspace/crypto
study=research/ideas_execution_20261018/bounded_memory
manifest="$study/uncompressed-archive-sha256-v2.txt"
if [ -e "$manifest" ]; then echo 'archive manifest already exists' >&2; exit 2; fi
while IFS= read -r artifact; do
 if [ -f "$artifact" ]; then
  # An interrupted packaging process may leave a partial gzip beside its raw file.
  if [ -f "$artifact.gz" ]; then rm "$artifact.gz"; fi
  sha256sum "$artifact" >> "$manifest"
  gzip -n -1 "$artifact"
 else
  gzip -t "$artifact.gz"
  printf '%s  %s\n' "$(gzip -dc "$artifact.gz" | sha256sum | cut -d' ' -f1)" "$artifact" >> "$manifest"
 fi
done < "$study/archive-paths.txt"
