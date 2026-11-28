#!/usr/bin/env bash
set -euo pipefail

ROOT=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
TMP=$(mktemp -d)
trap 'rm -rf "$TMP"' EXIT

cd "$ROOT"
sha256sum -c SOURCE_SHA256SUMS_2
gzip -dc evidence/candidate.txt.gzip > "$TMP/candidate.txt"
printf '%s  %s\n' \
  '9427504890b0f6ecbd96bf1c8a6baa4eb9c63ea1a6dd4608694bbcb0d48aba45' \
  "$TMP/candidate.txt" | sha256sum -c -

cargo build --manifest-path native/Cargo.toml --release --locked \
  --bin pair-transfer-verify --bin pair-transfer-analyze
native/target/release/pair-transfer-verify "$TMP/candidate.txt" > "$TMP/verifier.txt"
native/target/release/pair-transfer-analyze \
  "$TMP/candidate.txt" "$TMP/verifier.txt" > "$TMP/analysis.txt"
cmp evidence-v2/verifier.txt "$TMP/verifier.txt"
cmp evidence-v2/analysis.txt "$TMP/analysis.txt"
