#!/usr/bin/env bash
# Primary rows of PROTOCOL.md: the 18 round-seven rows, fb-walk, 5 IC and 40
# rho trials, full-width rho, one thread. Writes raw/g{2,3,4}.jsonl and logs.
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd)
root=$(cd "$here/../.." && pwd)
bin="$root/target/release/examples/hyperelliptic_ic_vs_rho_bitops"
[ -x "$bin" ] || { echo "missing $bin" >&2; exit 2; }
mkdir -p "$here/raw"
export RAYON_NUM_THREADS=1
run() { # genus primes
  "$bin" --genus "$1" --p "$2" --arms fb-walk --ic-trials 5 --rho-trials 40 \
    --rho-variant full-width --json "$here/raw/g$1.jsonl" >"$here/raw/g$1.txt" 2>&1
  echo "DONE g$1"
}
run 2 41,61,101,151,211,251 &
run 3 23,31,41,61,101,151,211 &
run 4 17,23,31,41,61 &
wait
