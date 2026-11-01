#!/usr/bin/env bash
# Write validator-report-benchmarks.json: validator-report.json with each
# raw_data `path` re-based to be relative to benchmarks/20261022-ic-generator-orbit/
# (where the benchmarks row's VALIDATION.json lives), plus `rebased_for`.
# Nothing else changes: headlines, findings and every other field are copied.
set -euo pipefail
V="$(cd "$(dirname "$0")" && pwd)"
SRC="$V/validator-report.json"
jq --arg sha "$(sha256sum "$SRC" | cut -c1-64)" '
  .raw_data |= map(.path = "../../research/ecbench_generator_orbit_20261022/" + .path)
  | .rebased_for = { directory: "benchmarks/20261022-ic-generator-orbit/",
                     copies: "research/ecbench_generator_orbit_20261022/validation/validator-report.json",
                     copies_sha256: $sha,
                     changed: "raw_data[].path only (re-based to this directory); raw_data_paths_relative_to and every other field are copied unchanged from the source report and describe it" }
' "$SRC" > "$V/validator-report-benchmarks.json"
sha256sum "$V/validator-report-benchmarks.json"
