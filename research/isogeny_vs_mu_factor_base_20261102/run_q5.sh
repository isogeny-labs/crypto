#!/usr/bin/env bash
# Frozen launcher for PROTOCOL_Q5_MSOLVE.md. Usage: run.sh <output-dir> (must not exist).
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
"$bin" "$out" msolve 2> "$out/progress.log"
"$bin" "$out" analyze_msolve
echo "end_utc $(date -u +%Y-%m-%dT%H:%M:%SZ)" >> "$out/host.txt"
rm -f "$out/q5_tmp.ms" "$out/q5_tmp.ms.out"; echo "msolve $(dpkg-query -W -f="${Version}" msolve) $(sha256sum "$(command -v msolve)" | cut -d" " -f1)" >> "$out/host.txt"
(cd "$out" && sha256sum ./*.csv ./*.json > SHA256SUMS)
cat "$out/q5_verdicts.json"
