#!/bin/sh
# Thin native build/replay orchestration, not a benchmark launcher.
set -eu
study_dir=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
repo_dir=$(git -C "$study_dir" rev-parse --show-toplevel)
out_dir=${1:?usage: sh primitive_gate.sh ABSOLUTE_OUTPUT_DIRECTORY}
sh "$study_dir/rational_gate.sh" "$out_dir"
out_dir=$(CDPATH= cd -- "$out_dir" && pwd)
cmp "$out_dir/rank.jsonl" "$study_dir/RATIONAL-RANK-VERIFIED.jsonl"
"$out_dir/rank-gate" --primitive > "$out_dir/primitive-rank.jsonl" 2> "$out_dir/primitive-rank.stderr"
cmp "$out_dir/primitive-rank.jsonl" "$study_dir/PRIMITIVE-RANK.jsonl"
jq -s -e 'length==36 and all(.rank==(.full_rank-.off_rank))' "$out_dir/primitive-rank.jsonl" > "$out_dir/primitive-rank-check.txt"
cd "$repo_dir"
cargo test --release --manifest-path crates/hec-native-ic/Cargo.toml --example heldout \
  rational_solve::tests::normalized_single_target_solves -- --nocapture > "$out_dir/integration.txt" 2>&1
shasum -a 256 "$study_dir/rational_solve.rs" "$study_dir/anchor.rs" \
  "$study_dir/heldout.rs" "$study_dir/primitive_gate.sh" >> "$out_dir/sha256.txt"
printf 'Primitive replay and single-target integration gates passed: %s\n' "$out_dir"
