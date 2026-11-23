#!/usr/bin/env bash
# Indicative, non-isolated local wall-time A/B of the descent arms.
# A = production (staircase F4, no trace), B = traced (descent-finder C1).
# One process per arm run, 1 rayon thread, frozen linear-stage build,
# order A,B,A,B,... for ROUNDS rounds per stream and hook setting.
# The host is shared with other agents; no CPU pinning or isolation.
# Usage: run_ab.sh BIN_DIR ROUNDS HOOKS(0|1) OUTDIR
set -euo pipefail
B=${1:?bin}; ROUNDS=${2:?rounds}; HOOKS=${3:?hooks}; OUT=${4:?outdir}
lane=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
R=$lane/../..
mkdir -p "$OUT"
sha256sum "$B/descent_walltime" > "$OUT/binary.sha256"
for s in "389 3415 $R/runpod-deferred-pair-20261117-50/public.json" "401 1506 $R/nagao-e2e/public.json"; do
  set -- $s
  for r in $(seq 1 "$ROUNDS"); do
    for arm in production traced; do
      f="$OUT/seed$1-hooks$HOOKS-r$r-$arm.json"
      echo "$(date -u +%FT%T.%NZ) start $f load=$(cut -d' ' -f1-3 /proc/loadavg)" >> "$OUT/order.log"
      "$B/descent_walltime" "$3" "$1" "$2" "$arm" "$HOOKS" "$f" > /dev/null
      echo "$(date -u +%FT%T.%NZ) end   $f load=$(cut -d' ' -f1-3 /proc/loadavg)" >> "$OUT/order.log"
    done
  done
done
