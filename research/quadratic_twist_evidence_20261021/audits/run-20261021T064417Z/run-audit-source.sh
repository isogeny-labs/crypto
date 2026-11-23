#!/usr/bin/env bash
set -euo pipefail
study_dir="$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)"
audit_dir="${1:-$study_dir/audits/run-$(date -u +%Y%m%dT%H%M%SZ)}"
if [[ -e "$audit_dir" ]]; then
  echo "Refusing to overwrite an existing audit directory: $audit_dir" >&2
  exit 2
fi
mkdir -p "$audit_dir"
scratch_dir="$(mktemp -d "${TMPDIR:-/tmp}/quadratic-twist-audit.XXXXXX")"
trap 'rm -rf -- "$scratch_dir"' EXIT
exec > >(tee "$audit_dir/run.log") 2>&1
date -u '+Started UTC: %Y-%m-%dT%H:%M:%SZ'
g++ --version | head -1
openssl version
(cd "$study_dir/historical" && sha256sum -c source-and-evidence.tar.xz.sha256)
tar -xJf "$study_dir/historical/source-and-evidence.tar.xz" -C "$scratch_dir"
g++ -std=c++17 -O2 -Wall -Wextra -Werror "$study_dir/audit.cpp" -lcrypto -o "$scratch_dir/audit"
"$scratch_dir/audit" "$scratch_dir" "$audit_dir/receipt.json"
date -u '+Finished UTC: %Y-%m-%dT%H:%M:%SZ'
printf 'Audit exit code: 0\n'
(cd "$study_dir" && sha256sum audit.cpp run-audit.sh historical/source-and-evidence.tar.xz) > "$audit_dir/inputs.sha256"
