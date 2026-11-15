#!/usr/bin/env bash
# Full run with the frozen binary: every cell, score, hashes, provenance.
set -euo pipefail
cd "$(dirname "$0")"
R=${1:?run dir}
B=./target/release/odd_n_trace
mkdir -p "$R"
sha256sum -c --quiet FREEZE.sha256
git rev-parse HEAD > "$R/commit.txt"
{ hostname; nproc; rustc --version; uname -r; } > "$R/host.txt"
sha256sum "$B" > "$R/binary.sha256"
$B selftest > "$R/selftest.txt"
for c in "3 3" "3 4" "3 5" "5 3" "4 3" "4 4" "5 4" "5 5"; do
  s=$SECONDS
  $B run $c "$R" 2>> "$R/run.log"
  echo "cell $c $((SECONDS-s))s" >> "$R/run.log"
done
$B score "$R" > /dev/null
(cd "$R" && sha256sum cell_*.jsonl summary.json summary.txt > outputs.sha256)
