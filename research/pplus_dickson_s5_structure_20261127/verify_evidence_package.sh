#!/usr/bin/env bash
set -euo pipefail

if [[ $# -ne 1 ]]; then
  echo "usage: $0 EVIDENCE_DIRECTORY" >&2
  exit 2
fi

evidence_dir=$1
(
  cd "$evidence_dir"
  sha256sum --check --strict PACKAGE_SHA256SUMS
)

expected=$(awk '$2 == "candidate.txt" { print $1 }' "$evidence_dir/SHA256SUMS")
actual=$(gzip --decompress --stdout "$evidence_dir/candidate.txt.gzip" | sha256sum | awk '{ print $1 }')
if [[ -z "$expected" || "$actual" != "$expected" ]]; then
  echo "decompressed candidate digest mismatch" >&2
  exit 2
fi

echo "package and uncompressed candidate verified: $evidence_dir"
