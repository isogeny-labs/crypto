#!/usr/bin/env bash
# Same native worker argv; foreground timeout stays in the guarded job group.
set -euo pipefail
ulimit -v 4194304
export OMP_NUM_THREADS=1 RAYON_NUM_THREADS=1
exec timeout --foreground --signal=TERM --kill-after=5 120 taskset -c 4 "$@"
