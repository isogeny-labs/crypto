#!/usr/bin/env bash
set -euo pipefail

ROOT=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
EVIDENCE="$ROOT/evidence-v2"
TMP=$(mktemp -d)
trap 'rm -rf "$TMP"' EXIT

mkdir -p "$EVIDENCE"
cd "$ROOT"
sha256sum -c SOURCE_SHA256SUMS_2
gzip -dc evidence/candidate.txt.gzip > "$TMP/candidate.txt"
printf '%s  %s\n' \
  '9427504890b0f6ecbd96bf1c8a6baa4eb9c63ea1a6dd4608694bbcb0d48aba45' \
  "$TMP/candidate.txt" | sha256sum -c -

{
  date -u +%Y-%m-%dT%H:%M:%SZ
  git rev-parse HEAD
  rustc --version
  cargo --version
  echo 'seed_manifest=null'
  echo 'candidate_source=evidence/candidate.txt.gzip'
  echo 'candidate_sha256=9427504890b0f6ecbd96bf1c8a6baa4eb9c63ea1a6dd4608694bbcb0d48aba45'
  echo 'pair-transfer-verify candidate.txt > evidence-v2/verifier.txt'
  echo 'pair-transfer-analyze candidate.txt evidence-v2/verifier.txt > evidence-v2/analysis.txt'
} > "$EVIDENCE/RUN.txt"

cargo build --manifest-path native/Cargo.toml --release --locked \
  --bin pair-transfer-verify --bin pair-transfer-analyze
native/target/release/pair-transfer-verify \
  "$TMP/candidate.txt" > "$EVIDENCE/verifier.txt"
native/target/release/pair-transfer-analyze \
  "$TMP/candidate.txt" "$EVIDENCE/verifier.txt" > "$EVIDENCE/analysis.txt"

sha256sum \
  evidence/candidate.txt.gzip \
  "$EVIDENCE/verifier.txt" \
  "$EVIDENCE/analysis.txt" \
  "$EVIDENCE/RUN.txt" > "$EVIDENCE/SHA256SUMS"
