#!/bin/zsh
set -euo pipefail
cd "$(dirname "$0")/.."
b=$(mktemp -d); echo "$b"
awk '$3=="selected"{print $1,$2,$5}' results/g4-rank-panel-curves.txt > "$b/selected"
rustc sources/lib-table.rs --edition=2021 --crate-name hec_native_ic --crate-type rlib -C opt-level=3 -A warnings -o "$b/libhec_native_ic.rlib"
rustc harness/g4-rank-verify.rs --edition=2021 -C opt-level=3 --extern "hec_native_ic=$b/libhec_native_ic.rlib" -o "$b/verify"
for variant in tangent osculating combined; do
  case "$variant" in
    tangent) generator=(harness/g4-rank-panel.rb extend tangent);;
    osculating) generator=(harness/g4-rank-panel.rb extend tangent osculate);;
    combined) generator=(harness/g4-hermite.rb);;
  esac
  ruby "${generator[@]}" | c++ -std=c++17 -O3 -x c++ - -o "$b/$variant"
  "$b/$variant" < "$b/selected" > "$b/$variant.out"
  rg '^VALUES ' "$b/$variant.out" > "$b/values"
  "$b/verify" results/g4-rank-panel-curves.txt "$b/values"
  if [[ $variant == combined ]]; then
    rg '^STAGE ' "$b/$variant.out" | cmp - results/g4-hermite-stages.txt
  else
    rg '^EXT ' "$b/$variant.out" | cmp - "results/g4-$variant-rank.txt"
  fi
done
