#!/usr/bin/env bash
# Link exact production sources without copying, patching, or formatting them.
set -euo pipefail
here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
root="$(cd "$here/../../.." && pwd)"
src="$here/src/cryptanalysis"
mkdir -p "$src"
for file in large_prime_filter.rs large_prime_filter_mst.rs large_prime_graph.rs jv_large_prime_rank_bound.rs; do
  test -f "$root/src/cryptanalysis/$file"
  ln -sfn "$root/src/cryptanalysis/$file" "$src/$file"
done
for dir in large_prime_filter large_prime_graph; do
  test -d "$root/src/cryptanalysis/$dir"
  ln -sfn "$root/src/cryptanalysis/$dir" "$src/$dir"
done
echo "Native source paths (symlinks resolve to original production files):"
for file in large_prime_filter.rs large_prime_filter_mst.rs large_prime_graph.rs jv_large_prime_rank_bound.rs; do
  readlink -f "$src/$file"
done
cargo test --manifest-path "$here/Cargo.toml" --test large_prime_graph --release -- --nocapture
