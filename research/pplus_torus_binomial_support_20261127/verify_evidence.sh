#!/usr/bin/env bash
set -euo pipefail

study_dir=$(cd "$(dirname "$0")" && pwd)
repo_dir=$(cd "$study_dir/../.." && pwd)
native_dir="$study_dir/native"
evidence_dir="$study_dir/evidence-local"
target_dir="$repo_dir/target/pplus-torus-binomial-support-verify"
scratch_dir=$(mktemp -d)
trap 'rm -rf "$scratch_dir"' EXIT

(
  cd "$repo_dir"
  sha256sum -c "$study_dir/SOURCE_SHA256SUMS"
)
(
  cd "$evidence_dir"
  sha256sum -c SHA256SUMS
)

gzip -cd "$evidence_dir/candidate-g.txt.gz" > "$scratch_dir/candidate-g.txt"
gzip -cd "$evidence_dir/candidate-g17.txt.gz" > "$scratch_dir/candidate-g17.txt"

test "$(sha256sum "$scratch_dir/candidate-g.txt" | awk '{print $1}')" = \
  "$(sed -n 's/^candidate_g_uncompressed_sha256=//p' "$evidence_dir/RUN.txt")"
test "$(sha256sum "$scratch_dir/candidate-g17.txt" | awk '{print $1}')" = \
  "$(sed -n 's/^candidate_g17_uncompressed_sha256=//p' "$evidence_dir/RUN.txt")"

cargo build --release --locked --manifest-path "$native_dir/Cargo.toml" --target-dir "$target_dir"
verifier="$target_dir/release/torus-support-verify"
analyzer="$target_dir/release/torus-support-analyze"

"$verifier" --candidate "$scratch_dir/candidate-g.txt" > "$scratch_dir/verifier-g.txt"
"$verifier" --candidate "$scratch_dir/candidate-g17.txt" > "$scratch_dir/verifier-g17.txt"
cmp "$scratch_dir/verifier-g.txt" "$evidence_dir/verifier-g.txt"
cmp "$scratch_dir/verifier-g17.txt" "$evidence_dir/verifier-g17.txt"

"$analyzer" \
  --candidate-g "$scratch_dir/candidate-g.txt" \
  --verify-g "$scratch_dir/verifier-g.txt" \
  --candidate-g17 "$scratch_dir/candidate-g17.txt" \
  --verify-g17 "$scratch_dir/verifier-g17.txt" \
  > "$scratch_dir/analysis.txt"
cmp "$scratch_dir/analysis.txt" "$evidence_dir/analysis.txt"

printf 'evidence replay verified\n'
