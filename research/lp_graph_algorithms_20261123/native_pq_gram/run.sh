#!/usr/bin/env bash
# Run focused native Rust tests on the ORIGINAL PQ source modules.
# A small harness replaces only root-crate dependencies, not either module.
set -euo pipefail
here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
repo="$(cd "$here/../../.." && pwd)"
mkdir -p "$here/src/cryptanalysis"
for module in pq_sparse_la pq_wiedemann; do
  source_path="$repo/src/cryptanalysis/$module.rs"
  test -f "$source_path"
  ln -sfn "$source_path" "$here/src/cryptanalysis/$module.rs"
  test "$(readlink -f "$here/src/cryptanalysis/$module.rs")" = "$(readlink -f "$source_path")"
done
echo "Source identity:"
git -C "$repo" rev-parse HEAD
sha256sum "$repo/src/cryptanalysis/pq_sparse_la.rs" "$repo/src/cryptanalysis/pq_wiedemann.rs"
echo "Native source-bound gate (shimmed root dependencies, NOT full-crate build)"
CARGO_BUILD_JOBS=2 cargo test --manifest-path "$here/Cargo.toml" --lib -- --nocapture
