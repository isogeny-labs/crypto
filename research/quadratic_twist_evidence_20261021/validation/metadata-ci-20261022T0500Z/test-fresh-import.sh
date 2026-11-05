#!/usr/bin/env bash
# Integration regression: append from the original 2,572-model catalog.
set -euo pipefail
record_dir="$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)"
repo_dir="$(git -C "$record_dir" rev-parse --show-toplevel)"
study_dir="$repo_dir/research/quadratic_twist_evidence_20261021"
fixture_dir="$(mktemp -d "${TMPDIR:-/tmp}/twist-fresh-import.XXXXXX")"
trap 'rm -rf -- "$fixture_dir"' EXIT
mkdir -p "$fixture_dir/docs/curves" "$fixture_dir/src/cryptanalysis" \
  "$fixture_dir/research/quadratic_twist_evidence_20261021/lean"
git -C "$repo_dir" show f8e72869e284a99f16d17b545f0d7666ef943c35:docs/curves/registry.json \
  > "$fixture_dir/docs/curves/registry.json"
git -C "$repo_dir" show f8e72869e284a99f16d17b545f0d7666ef943c35:src/cryptanalysis/curve_aliases.json \
  > "$fixture_dir/src/cryptanalysis/curve_aliases.json"
cp "$study_dir/lean/TwistProof.lean" \
  "$fixture_dir/research/quadratic_twist_evidence_20261021/lean/TwistProof.lean"
cargo run --release --locked --manifest-path "$study_dir/native/metadata/Cargo.toml" -- \
  "$fixture_dir" "$study_dir/native/runs/run-20261021T150125Z/results.json" \
  "$record_dir/fresh-import" --write-registry
cmp "$fixture_dir/docs/curves/registry.json" "$repo_dir/docs/curves/registry.json"
cmp "$record_dir/fresh-import/curve-models.json" \
  "$repo_dir/docs/curves/sources/quadratic-twist-native-20261021.json"
node --input-type=module - "$fixture_dir/src/cryptanalysis/curve_aliases.json" \
  "$repo_dir/src/cryptanalysis/curve_aliases.json" <<'NODE'
import fs from "node:fs";
import assert from "node:assert/strict";
const [left, right] = process.argv.slice(2).map((file) => JSON.parse(fs.readFileSync(file)));
assert.deepEqual(left, right, "fresh import changed an alias mapping");
console.log("PASS: fresh native import matches all 4,493 canonical registry rows byte-for-byte; aliases equal");
NODE
