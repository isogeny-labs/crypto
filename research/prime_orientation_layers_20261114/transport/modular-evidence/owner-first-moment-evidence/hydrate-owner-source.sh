#!/usr/bin/env bash
set -euo pipefail
artifact_dir=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
source_copy="$artifact_dir/source-003.lean.txt"
archive_copy="$artifact_dir/PolynomialFirstMoment.lean"
if [[ -e "$archive_copy" ]]; then
  cmp -- "$source_copy" "$archive_copy"
else
  cp -- "$source_copy" "$archive_copy"
fi
sha256sum -- "$archive_copy"
