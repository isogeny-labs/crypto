#!/usr/bin/env bash
# Step 1 of the red-team plan (#2628 REPORT.md §3): full replay of the claim's clean sessions
# with the claim binary (built from e403626cb). Resumable: skips sessions with a receipt.
set -u
cd "$(git -C "$(dirname "$0")" rev-parse --show-toplevel)"
for s in d-ladder e-m15; do
  out=research/gaudry_k4_char2_replication_20261026/audits/replay-$s.json
  [ -s "$out" ] && continue
  ./target/release/ecbench verify --dir research/gaudry_k4_char2_replication_20261026/claim-snapshot/research/gaudry_k4_char2_crossover_20261015/sessions/$s --replay-all --exit-code --out "$out" > research/gaudry_k4_char2_replication_20261026/logs/replay-$s.log 2>&1
  echo "$s exit $?" >> research/gaudry_k4_char2_replication_20261026/logs/replay.status
done
echo DONE >> research/gaudry_k4_char2_replication_20261026/logs/replay.status
