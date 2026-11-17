#!/usr/bin/env bash
# Frozen launcher for PROTOCOL_Q4_SEARCH.md. Usage: run.sh <output-dir> (must not exist).
set -euo pipefail
here="$(cd "$(dirname "$0")" && pwd)"
out="$1"
[ -e "$out" ] && { echo "refusing: $out exists" >&2; exit 2; }
mkdir -p "$out"; out="$(cd "$out" && pwd)"
cargo build --release --locked --manifest-path "$here/search/Cargo.toml"
cargo test --release --locked --manifest-path "$here/search/Cargo.toml" > "$out/cargo-test.log" 2>&1
bin="$here/search/target/release/isomu_search"
{
  echo "commit $(git -C "$here" rev-parse HEAD)"
  echo "dirty $(git -C "$here" status --porcelain -- . | grep -v '^?? results/' | wc -l)"
  echo "start_utc $(date -u +%Y-%m-%dT%H:%M:%SZ)"
  echo "uname $(uname -srm)"; echo "nproc $(nproc)"; echo "rustc $(rustc -V)"
  echo "binary_sha256 $(sha256sum "$bin" | cut -d' ' -f1)"
} > "$out/host.txt"
"$bin" "$out" 2> "$out/progress.log"
"$bin" "$out" analyze
echo "end_utc $(date -u +%Y-%m-%dT%H:%M:%SZ)" >> "$out/host.txt"
(cd "$out" && sha256sum ./*.csv ./*.json > SHA256SUMS)
cat "$out/q4_verdicts.json"
