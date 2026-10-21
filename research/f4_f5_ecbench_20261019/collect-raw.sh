#!/usr/bin/env bash
# Administrative copy only; never changes sealed session records.
set -euo pipefail
source_tree=${1:?fetched ephemeral tree} label=${2:?archive label}
study=research/f4_f5_ecbench_20261019
destination="$study/runs/$label"
test ! -e "$destination"
test -f "$source_tree/status.json"
test -d "$source_tree/out"
mkdir -p "$destination"
# Keep exact runner commands, patch and status. An environment-export file
# is intentionally excluded; no secrets or credential material are archived.
for name in job.json status.json cmd.sh patch.diff job.log; do
  if [ -f "$source_tree/$name" ]; then cp -a "$source_tree/$name" "$destination/"; fi
done
# Executables remain in the immutable S3 job tree with their recorded hashes.
# All raw records, rejected retries, sources, commands and audits are copied.
rsync -a --exclude='/baseline-benchmark' --exclude='/candidate-benchmark' \
  --exclude='/ecbench-binary' "$source_tree/out/" "$destination/out/"
