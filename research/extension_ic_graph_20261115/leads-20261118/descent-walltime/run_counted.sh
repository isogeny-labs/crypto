#!/usr/bin/env bash
# Exact counted work per arm (counted linear-stage build, jv_bitops hooks on),
# one process per arm, on the two reconstructed local streams.
# Usage: run_counted.sh BIN_DIR
set -euo pipefail
B=${1:?bin dir}
lane=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
R=$lane/../..
mkdir -p "$lane/runs/counted"
for s in "389 3415 $R/runpod-deferred-pair-20261117-50/public.json" "401 1506 $R/nagao-e2e/public.json"; do
  set -- $s
  for arm in production traced; do
    "$B/descent_walltime_counted" "$3" "$1" "$2" "$arm" 1 "$lane/runs/counted/seed$1-n$2-$arm.json" > /dev/null
  done
done
