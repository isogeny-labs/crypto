#!/usr/bin/env bash
set -euo pipefail

study=research/four_followups_20261021
primary="$study/results/matched"
replay="$study/results/matched-replay"
mkdir -p "$replay"

# Force the committed input archive path even when an extracted scratch copy
# exists on the executing host.
FOUR_FOLLOWUPS_ARCHIVE_INPUT=1 FOUR_FOLLOWUPS_OUTPUT_DIR="$replay" \
  bash "$study/run_factor_base.sh"

for first in "$primary"/fb*-bits*.jsonl; do
  second="$replay/${first##*/}"
  cmp "$first" "$second"
  printf '%s OK\n' "${first##*/}"
done
