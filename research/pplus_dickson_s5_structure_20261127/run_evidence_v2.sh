#!/usr/bin/env bash
set -euo pipefail

script_dir=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
repo_root=$(git -C "$script_dir" rev-parse --show-toplevel)
crate_dir="$script_dir/native"
evidence_dir="$script_dir/evidence-v2"
manifest="$script_dir/SOURCE_SHA256SUMS_2"
qualified_source_commit=18dca898e2b895a31f7651e230498baf583c2eb8

if [[ -e "$evidence_dir" ]]; then
  echo "refusing to overwrite $evidence_dir" >&2
  exit 2
fi

grep -F 'COVERAGE_INVALID_STOP' "$script_dir/evidence/STATUS.md" >/dev/null
if ! git -C "$repo_root" merge-base --is-ancestor "$qualified_source_commit" HEAD; then
  echo "qualified v2 source commit is not an ancestor of HEAD" >&2
  exit 2
fi

(
  cd "$repo_root"
  sha256sum --check --strict "$manifest"
)

cargo build --manifest-path "$crate_dir/Cargo.toml" --locked --release
mkdir "$evidence_dir"

"$crate_dir/target/release/dickson-s5-structure" \
  >"$evidence_dir/candidate.txt"
"$crate_dir/target/release/dickson-s5-structure-verify" \
  "$evidence_dir/candidate.txt" >"$evidence_dir/verifier.txt"
"$crate_dir/target/release/dickson-s5-structure-analyze" \
  "$evidence_dir/candidate.txt" "$evidence_dir/verifier.txt" \
  >"$evidence_dir/analysis.txt"

(
  cd "$repo_root"
  sha256sum --check --strict "$manifest"
)

{
  printf 'schema=pplus-dickson-s5-run-v2\n'
  printf 'qualified_source_commit=%s\n' "$qualified_source_commit"
  printf 'head_commit=%s\n' "$(git -C "$repo_root" rev-parse HEAD)"
  printf 'seed=none\n'
  printf 'candidate_exit=0\nverifier_exit=0\nanalyzer_exit=0\n'
  rustc --version
  cargo --version
} >"$evidence_dir/RUN.txt"

(
  cd "$evidence_dir"
  sha256sum RUN.txt candidate.txt verifier.txt analysis.txt >SHA256SUMS
  sha256sum --check --strict SHA256SUMS
)

grep -F 'schema=pplus-dickson-s5-analysis-v2' "$evidence_dir/analysis.txt"
grep -F 'coverage_complete=1' "$evidence_dir/analysis.txt"
grep -F 'verdict=STRUCTURE_ONLY_STOP' "$evidence_dir/analysis.txt"
echo "v2 evidence complete: $evidence_dir"
