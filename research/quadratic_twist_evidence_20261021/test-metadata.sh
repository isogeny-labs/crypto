#!/usr/bin/env bash
set -euo pipefail
study_dir="$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)"
repo_dir="$(git -C "$study_dir" rev-parse --show-toplevel)"
input_dir="${1:?usage: test-metadata.sh NATIVE_RUN FRESH_METADATA_DIRECTORY}"
output_dir="${2:?fresh metadata output required}"
[[ ! -e "$output_dir" ]] || { echo "Refusing to overwrite $output_dir" >&2; exit 2; }
scratch_dir="$(mktemp -d "${TMPDIR:-/tmp}/twist-metadata.XXXXXX")"
trap 'rm -rf -- "$scratch_dir"' EXIT
cargo run --release --locked --manifest-path "$study_dir/native/metadata/Cargo.toml" -- "$repo_dir" "$input_dir/results.json" "$output_dir" > "$scratch_dir/positive.log" 2>&1
cp "$scratch_dir/positive.log" "$output_dir/replay.log"
cmp "$output_dir/curve-models.json" "$repo_dir/docs/curves/sources/quadratic-twist-native-20261021.json"
for name in symmetric-s3 vieta-cancellation twist-equation identity-index; do
  cmp "$output_dir/$name.json" "$study_dir/certificates/$name.json"
done
jq -e '.new_registry_rows == 0 and .new_alias_keys == 0' "$output_dir/registry-update.json"
cp "$input_dir/parity.json" "$input_dir/independent-ic-audit.json" "$input_dir/basis.json" "$scratch_dir/"
jq '.exhaustive.counts.curve_models += 1' "$input_dir/results.json" > "$scratch_dir/results.json"
if cargo run --release --locked --manifest-path "$study_dir/native/metadata/Cargo.toml" -- "$repo_dir" "$scratch_dir/results.json" "$scratch_dir/rejected-output" > "$output_dir/source-binding-negative.log" 2>&1; then
  echo 'FAIL: changed native input accepted for metadata'; exit 1
fi
grep -q 'native result source binding' "$output_dir/source-binding-negative.log"
[[ ! -e "$scratch_dir/rejected-output" ]] || { echo 'FAIL: metadata emitted before source validation'; exit 1; }
echo 'PASS: certificate/catalog byte replay, registered representations, and source-binding rejection'
