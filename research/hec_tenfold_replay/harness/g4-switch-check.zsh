#!/bin/zsh
set -euo pipefail
cd "$(dirname "$0")/.."
b=$(mktemp -d);echo "$b"
awk '$3=="selected"{print $1,$2,$5}' results/g4-rank-panel-curves.txt > "$b/selected"
rustc sources/lib-table.rs --edition=2021 --crate-name hec_native_ic --crate-type rlib -C opt-level=3 -A warnings -o "$b/libhec_native_ic.rlib"
rustc harness/g4-rank-verify.rs --edition=2021 -C opt-level=3 --extern "hec_native_ic=$b/libhec_native_ic.rlib" -o "$b/verify"
for arm in full early half late fixed; do
 ruby harness/g4-switch.rb "$arm" | c++ -std=c++17 -O3 -x c++ - -o "$b/$arm"
 "$b/$arm" < "$b/selected" > "$b/$arm.out"
 rg '^VALUES' "$b/$arm.out" > "$b/$arm.values"
 "$b/verify" results/g4-rank-panel-curves.txt "$b/$arm.values"
done
for arm in full early half late fixed; do
  awk -v arm="$arm" '$1=="EXT"{print arm,$0}' "$b/$arm.out"
done | cmp - results/g4-switch-ranks.txt
ruby harness/g4-carry.rb | c++ -std=c++17 -O3 -x c++ - -o "$b/carry"
"$b/carry" < "$b/selected" > "$b/carry.out"
rg '^STAGE' "$b/carry.out" | cmp - results/g4-hermite-stages.txt
rg '^VALUES' "$b/carry.out" > "$b/carry.values"
"$b/verify" results/g4-rank-panel-curves.txt "$b/carry.values"
