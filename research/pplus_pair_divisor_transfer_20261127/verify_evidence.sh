#!/usr/bin/env bash
set -euo pipefail

ROOT=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
TMP=$(mktemp -d)
trap 'rm -rf "$TMP"' EXIT

cd "$ROOT"
sha256sum -c SOURCE_SHA256SUMS
gzip -dc evidence/candidate.txt.gzip > "$TMP/candidate.txt"

cargo build --manifest-path native/Cargo.toml --release --locked \
  --bin pair-transfer-verify --bin pair-transfer-analyze
native/target/release/pair-transfer-verify "$TMP/candidate.txt" > "$TMP/verifier.txt"
native/target/release/pair-transfer-analyze \
  "$TMP/candidate.txt" "$TMP/verifier.txt" > "$TMP/analysis.txt"

cmp evidence/verifier.txt "$TMP/verifier.txt"
cmp evidence/analysis.txt "$TMP/analysis.txt"
