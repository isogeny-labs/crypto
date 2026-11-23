#!/usr/bin/env bash
# Recompute target relevance; preserve the first audit's conservative nulls.
set -euo pipefail
root=$(git rev-parse --show-toplevel)
cd "$root"
study=research/ic_algebraic_crossover_20261023
scratch=$(mktemp -d)
trap 'rm -rf "$scratch"' EXIT
for run in fixture-subgroups-20261025 fixture-subgroups-enumerated-20261025; do
  receipt="$study/pipeline-receipts/$run"
  sha256sum -c "$receipt/receipt.sha256"
  pin=$(cat "$receipt/source-commit.txt")
  git show "$pin:examples/ic_fixture_subgroups.rs" > "$scratch/source.rs"
  expected=$(sha256sum "$scratch/source.rs")
  recorded=$(rg '  examples/ic_fixture_subgroups.rs$' "$receipt/hashes.txt")
  test "${expected%% *}" = "${recorded%% *}"
done
cargo build --release --example ic_fresh_relevance --example ic_fixture_subgroups
bin_root=${CARGO_TARGET_DIR:-target}/release/examples
(ulimit -v 12582912; timeout 900 "$bin_root/ic_fresh_relevance" "$study/inputs/fresh") > "$scratch/fresh.json"
cmp "$scratch/fresh.json" "$study/pipeline-receipts/fresh-target-relevance-20261025/receipt.json"
(ulimit -v 12582912; timeout 900 "$bin_root/ic_fixture_subgroups" "$study") > "$scratch/subgroups.json"
cmp "$scratch/subgroups.json" "$study/pipeline-receipts/fixture-subgroups-enumerated-20261025/receipt.json"
printf '%s\n' 'PASS: original receipt hashes/source pins and current native lift/subgroup checks reproduce. Rebuild v1 at its recorded pin for its conservative-null output.'
