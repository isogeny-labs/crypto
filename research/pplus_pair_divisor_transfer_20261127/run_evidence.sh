#!/usr/bin/env bash
set -euo pipefail

ROOT=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
EVIDENCE="$ROOT/evidence"
TARGET="$ROOT/native/target"

mkdir -p "$EVIDENCE"
cd "$ROOT"
sha256sum -c SOURCE_SHA256SUMS

{
  date -u +%Y-%m-%dT%H:%M:%SZ
  git rev-parse HEAD
  rustc --version
  cargo --version
  echo 'seed_manifest=null'
  echo 'cargo build --release --locked --bins'
  echo 'pair-transfer-candidate > evidence/candidate.txt'
  echo 'pair-transfer-verify evidence/candidate.txt > evidence/verifier.txt'
  echo 'pair-transfer-analyze evidence/candidate.txt evidence/verifier.txt > evidence/analysis.txt'
} > "$EVIDENCE/RUN.txt"

cargo build --manifest-path native/Cargo.toml --release --locked --bins
"$TARGET/release/pair-transfer-candidate" > "$EVIDENCE/candidate.txt"
"$TARGET/release/pair-transfer-verify" "$EVIDENCE/candidate.txt" > "$EVIDENCE/verifier.txt"
"$TARGET/release/pair-transfer-analyze" \
  "$EVIDENCE/candidate.txt" "$EVIDENCE/verifier.txt" > "$EVIDENCE/analysis.txt"

sha256sum \
  "$EVIDENCE/candidate.txt" \
  "$EVIDENCE/verifier.txt" \
  "$EVIDENCE/analysis.txt" \
  "$EVIDENCE/RUN.txt" > "$EVIDENCE/SHA256SUMS"

gzip -n -9 -c "$EVIDENCE/candidate.txt" > "$EVIDENCE/candidate.txt.gzip"
rm "$EVIDENCE/candidate.txt"
