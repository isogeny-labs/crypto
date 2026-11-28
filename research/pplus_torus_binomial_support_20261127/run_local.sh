#!/usr/bin/env bash
set -euo pipefail

study_dir=$(cd "$(dirname "$0")" && pwd)
repo_dir=$(cd "$study_dir/../.." && pwd)
native_dir="$study_dir/native"
evidence_dir="$study_dir/evidence-local"
target_dir="$repo_dir/target/pplus-torus-binomial-support"

test ! -e "$evidence_dir"
mkdir -p "$evidence_dir"

(
  cd "$repo_dir"
  sha256sum -c "$study_dir/SOURCE_SHA256SUMS"
)

cargo build --release --locked --manifest-path "$native_dir/Cargo.toml" --target-dir "$target_dir"

candidate="$target_dir/release/torus-support-candidate"
verifier="$target_dir/release/torus-support-verify"
analyzer="$target_dir/release/torus-support-analyze"

"$candidate" --target g > "$evidence_dir/candidate-g.txt"
"$candidate" --target g17 > "$evidence_dir/candidate-g17.txt"
"$verifier" --candidate "$evidence_dir/candidate-g.txt" > "$evidence_dir/verifier-g.txt"
"$verifier" --candidate "$evidence_dir/candidate-g17.txt" > "$evidence_dir/verifier-g17.txt"
"$analyzer" \
  --candidate-g "$evidence_dir/candidate-g.txt" \
  --verify-g "$evidence_dir/verifier-g.txt" \
  --candidate-g17 "$evidence_dir/candidate-g17.txt" \
  --verify-g17 "$evidence_dir/verifier-g17.txt" \
  > "$evidence_dir/analysis.txt"

candidate_g_sha=$(sha256sum "$evidence_dir/candidate-g.txt" | awk '{print $1}')
candidate_g17_sha=$(sha256sum "$evidence_dir/candidate-g17.txt" | awk '{print $1}')
candidate_g_bytes=$(wc -c < "$evidence_dir/candidate-g.txt")
candidate_g17_bytes=$(wc -c < "$evidence_dir/candidate-g17.txt")

gzip -n "$evidence_dir/candidate-g.txt"
gzip -n "$evidence_dir/candidate-g17.txt"

{
  printf 'backend=local\n'
  printf 'source_commit=%s\n' "$(git -C "$repo_dir" rev-parse HEAD)"
  printf 'rustc=%s\n' "$(rustc --version)"
  printf 'cargo=%s\n' "$(cargo --version)"
  printf 'candidate_g_uncompressed_sha256=%s\n' "$candidate_g_sha"
  printf 'candidate_g_uncompressed_bytes=%s\n' "$candidate_g_bytes"
  printf 'candidate_g17_uncompressed_sha256=%s\n' "$candidate_g17_sha"
  printf 'candidate_g17_uncompressed_bytes=%s\n' "$candidate_g17_bytes"
  printf 'seed_manifest=null\n'
  printf 'cross_target_amortization=0\n'
  printf 'commands=candidate(g);candidate(g17);verify(g);verify(g17);analyze\n'
} > "$evidence_dir/RUN.txt"

(
  cd "$evidence_dir"
  sha256sum \
    RUN.txt \
    analysis.txt \
    candidate-g.txt.gz \
    candidate-g17.txt.gz \
    verifier-g.txt \
    verifier-g17.txt \
    > SHA256SUMS
)

printf 'local evidence written to %s\n' "$evidence_dir"
