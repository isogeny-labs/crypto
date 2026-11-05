#!/usr/bin/env bash
set -euo pipefail
study_dir="$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)"
output_dir="${1:-$study_dir/audits/controls-$(date -u +%Y%m%dT%H%M%SZ)}"
if [[ -e "$output_dir" ]]; then
  echo "Refusing to overwrite an existing control directory" >&2
  exit 2
fi
mkdir -p "$output_dir"
scratch_dir="$(mktemp -d "${TMPDIR:-/tmp}/quadratic-twist-controls.XXXXXX")"
trap 'rm -rf -- "$scratch_dir"' EXIT
exec > >(tee "$output_dir/run.log") 2>&1
g++ -std=c++17 -O2 -Wall -Wextra -Werror "$study_dir/audit.cpp" -lcrypto -o "$scratch_dir/audit"
mkdir "$scratch_dir/good"
tar -xJf "$study_dir/historical/source-and-evidence.tar.xz" -C "$scratch_dir/good"
"$scratch_dir/audit" "$scratch_dir/good" "$output_dir/positive.json"
cp -a "$scratch_dir/good" "$scratch_dir/corrupt-source"
printf '\ncorruption control\n' >> "$scratch_dir/corrupt-source/experiments.py"
if "$scratch_dir/audit" "$scratch_dir/corrupt-source" "$output_dir/must-not-exist.json" > "$output_dir/corrupt-source.log" 2>&1; then
  echo 'FAIL: corrupted source was accepted' >&2
  exit 1
fi
grep -F 'checksum mismatch: experiments.py' "$output_dir/corrupt-source.log"
cp -a "$scratch_dir/good" "$scratch_dir/false-relation"
jq '.pairings[0].index_calculus.independent_relations[0].rhs = 999' \
  "$scratch_dir/false-relation/results/results.json" > "$scratch_dir/changed.json"
mv "$scratch_dir/changed.json" "$scratch_dir/false-relation/results/results.json"
new_digest="$(sha256sum "$scratch_dir/false-relation/results/results.json" | cut -d ' ' -f1)"
sed -i "s/^[a-f0-9]*  results\/results.json$/$new_digest  results\/results.json/" "$scratch_dir/false-relation/SHA256SUMS"
if "$scratch_dir/audit" "$scratch_dir/false-relation" "$output_dir/must-not-exist.json" > "$output_dir/false-relation.log" 2>&1; then
  echo 'FAIL: false algebraic relation was accepted' >&2
  exit 1
fi
grep -F 'relation equation' "$output_dir/false-relation.log"
printf 'PASS: original accepted, source corruption rejected, false relation rejected after updating its checksum\n'
