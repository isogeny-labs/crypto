#!/usr/bin/env bash
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"
study=research/curve25519_torus_native_20261115/section-search-v1
context=research/curve25519_torus_native_20261115/section-operator-v1
cmp "$study/DEPENDENCY_MANIFEST.json" "$context/lake-manifest.json"
cmp "$study/lean-toolchain" "$context/lean-toolchain"
section_search_lean_path=$(cd "$context" && lake env printenv LEAN_PATH)
LEAN_PATH="$section_search_lean_path" bash tools/lean/proof-gate.sh \
  --out "${1:?receipt directory required}" "$study/SectionSearchAssembly.lean"
