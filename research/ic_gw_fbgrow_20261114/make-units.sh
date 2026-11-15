#!/usr/bin/env bash
# units.tsv: "arm bin q seed args". Largest size first; per (q, seed) the three arms in rotated order (PROTOCOL.md).
# args after "ic <q> <seed>": D cS cD gamma block [frac max_attempts heavy_W]  (REF is v3 with its original 3-argument form)
set -euo pipefail
cd "$(dirname "$0")"
arms=("REF v3 D 3.0 1.7" "G160 v3g D 3.0 1.6 4.0 4 0.05 12 50" "G155 v3g D 3.0 1.55 4.0 4 0.05 12 60")
for q in 4194271 2097091 1048507 524287 262111; do
  for s in 1 2 3; do
    for i in 0 1 2; do
      a=${arms[$(( (i + s - 1) % 3 ))]}
      set -- $a
      arm=$1; bin=$2; shift 2
      echo "$arm $bin $q $s $*"
    done
  done
done > units.tsv
wc -l < units.tsv
