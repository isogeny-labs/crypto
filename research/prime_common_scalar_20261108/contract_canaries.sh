#!/usr/bin/env bash
set -euo pipefail
checker=${1:?native contract checker}
out=${2:?fresh output directory}
mkdir "$out"
here=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
root=$(git -C "$here" rev-parse --show-toplevel)
cd "$root"
schema_root=/workspace/crypto-autoresearcher/schemas
for which in protocol_hash missing_run counted_total artifact_hash; do
  case "$which" in
    protocol_hash) filter='.protocol.sha256=("0"*64)' ;;
    missing_run) filter='.runs |= .[0:1]' ;;
    counted_total) filter='.runs[0].measurements.counted.rows_completed=20927' ;;
    artifact_hash) filter='.files[0].sha256=("0"*64)' ;;
  esac
  jq "$filter" "$here/export.json" > "$out/$which.json"
  if "$checker" check-export "$here/protocol.json" "$out/$which.json" "$schema_root/experiment-protocol.v1.json" "$schema_root/experiment-export.v1.json" > "$out/$which.log" 2>&1; then
    echo "FAIL accepted $which";exit 1
  fi
  echo "rejected $which"
done
