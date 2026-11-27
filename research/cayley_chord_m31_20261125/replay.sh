#!/usr/bin/env bash
set -euo pipefail
cd -- "$(dirname -- "$0")"
if [[ ! -f evidence/cases-run1.jsonl || ! -f evidence/cases-run2.jsonl ]]; then
  python3 ../../tools/artifacts/artifacts.py pull evidence --source git
fi
sha256sum -c PUBLICATION.sha256
task_rustc="${RUSTC:-rustc}"
mkdir -p replay-output
(
  ulimit -v 1048576
  timeout 120 "$task_rustc" --edition=2021 -O -Dwarnings -Ccodegen-units=1 \
    -Clink-arg=-Wl,--threads=1 main.rs -o replay-output/cayley-chord-check
)
(
  ulimit -v 524288
  timeout 60 replay-output/cayley-chord-check replay-output/cases-run1.jsonl > replay-output/result-run1.json
  timeout 60 replay-output/cayley-chord-check replay-output/cases-run2.jsonl > replay-output/result-run2.json
)
cmp replay-output/cases-run1.jsonl replay-output/cases-run2.jsonl
cmp replay-output/result-run1.json replay-output/result-run2.json
cmp evidence/cases-run1.jsonl replay-output/cases-run1.jsonl
cmp evidence/result-run1.json replay-output/result-run1.json
cat replay-output/result-run1.json
