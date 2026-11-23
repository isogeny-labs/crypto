#!/usr/bin/env bash
# Same unchanged native worker argv for both source variants.
set -euo pipefail
ulimit -v 4194304
export OMP_NUM_THREADS=1 RAYON_NUM_THREADS=1
exec timeout --signal=TERM --kill-after=5 120 taskset -c 4 "$@"
