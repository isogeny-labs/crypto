#!/bin/zsh
set -euo pipefail
cd "$(dirname "$0")/.."
b=$(mktemp -d); echo "$b"
awk '$3=="selected"{print $1,$2,$5}' results/g4-rank-panel-curves.txt > "$b/selected"
for arm in reference direct rare; do
  ruby harness/g4-hermite-quotient.rb "$arm" rows audit | c++ -std=c++17 -O3 -x c++ - -o "$b/$arm"
  "$b/$arm" < "$b/selected" > "$b/$arm.out"
done
cmp "$b/reference.out" "$b/direct.out"
rg '^ROW ' "$b/direct.out" | shasum -a 256
rg '^STAGE' "$b/rare.out" | cmp - results/g4-quotient-rare-stages.txt
rg '^VALUES' "$b/rare.out" > "$b/values"
rustc sources/lib-table.rs --edition=2021 --crate-name hec_native_ic --crate-type rlib -C opt-level=3 -A warnings -o "$b/libhec_native_ic.rlib"
rustc harness/g4-rank-verify.rs --edition=2021 -C opt-level=3 --extern "hec_native_ic=$b/libhec_native_ic.rlib" -o "$b/verify"
"$b/verify" results/g4-rank-panel-curves.txt "$b/values"
