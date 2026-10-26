#!/usr/bin/env bash
set -euo pipefail
round_dir=$(cd "$(dirname "$0")" && pwd)
study_dir=$(cd "$round_dir/../.." && pwd)
expected=${1:-"$round_dir/runs"}
replay_dir=${2:-$(mktemp -d)}
mkdir -p "$replay_dir"
cp -R "$expected/census" "$expected/census6" "$replay_dir/"
cp "$expected/census-commit.txt" "$expected/census6-commit.txt" "$replay_dir/"
cargo build --release --locked --manifest-path "$study_dir/native/Cargo.toml"
runner="$study_dir/native/target/release/koblitz-modular-carriers"
"$runner" --conic-twists "$replay_dir"
"$runner" --polarized-extended "$replay_dir"
"$runner" --genus5-census "$replay_dir"
"$runner" --genus6-census "$replay_dir"
"$runner" --genus4-bounds "$replay_dir"
"$runner" --determinant-screen "$replay_dir"
"$runner" --n5-hermite-relations "$replay_dir"
"$runner" --polarization-vectors "$replay_dir"
"$runner" --verify-polarized "$replay_dir/gl2b-integral-polarizations.json"
"$runner" --verify-n5 "$replay_dir/n5d-geometric-relations.json"
for name in dp3-conic-twists gl2b-integral-polarizations gl3-genus5-census dp4-genus6-census mr3-determinant-screen g4-carrier-bounds n5d-geometric-relations gl4-short-vectors; do
  cmp "$expected/$name.json" "$replay_dir/$name.json"
done
printf 'PASS: eight final native outputs replay byte for byte; independent matrix and divisor checks pass.\n'
