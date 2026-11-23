#!/bin/sh
# Reproduce correctness/count evidence; no elapsed-time speed claim.
set -eu
study_dir=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
repo_dir=$(git -C "$study_dir" rev-parse --show-toplevel)
out_dir=${1:?usage: sh rational_gate.sh ABSOLUTE_OUTPUT_DIRECTORY}
mkdir -p "$out_dir"
out_dir=$(CDPATH= cd -- "$out_dir" && pwd)
cd "$repo_dir"
rustc --version > "$out_dir/rustc-version.txt"
cargo build --release --lib --manifest-path crates/hec-native-ic/Cargo.toml \
  --message-format=json > "$out_dir/build.jsonl" 2> "$out_dir/build.stderr"
rlib=$(jq -r 'select(.reason=="compiler-artifact" and .target.name=="hec_native_ic")|.filenames[]|select(endswith(".rlib"))' "$out_dir/build.jsonl")
test -f "$rlib"
deps_dir=$(dirname -- "$rlib")
rustc --edition=2021 --test -O "$study_dir/rational.rs" \
  --extern "hec_native_ic=$rlib" -L "dependency=$deps_dir" -o "$out_dir/algebra-gate"
"$out_dir/algebra-gate" --nocapture --test-threads=1 > "$out_dir/algebra.txt" 2>&1
rustc --edition=2021 -O "$study_dir/rational_rank.rs" \
  --extern "hec_native_ic=$rlib" -L "dependency=$deps_dir" -o "$out_dir/rank-gate" 2> "$out_dir/rank-build.stderr"
"$out_dir/rank-gate" > "$out_dir/rank.jsonl" 2> "$out_dir/rank.stderr"
jq -s -e 'length==36 and all(.rank==(.full_rank-.off_rank))' "$out_dir/rank.jsonl" > "$out_dir/rank-check.txt"
shasum -a 256 "$study_dir/rational.rs" "$study_dir/rational_rank.rs" \
  "$study_dir/dense.rs" crates/hec-native-ic/src/*.rs "$rlib" \
  "$out_dir/algebra-gate" "$out_dir/rank-gate" > "$out_dir/sha256.txt"
git rev-parse HEAD > "$out_dir/checkout-commit.txt"
printf 'Correctness/count gates passed; artifacts: %s\n' "$out_dir"
