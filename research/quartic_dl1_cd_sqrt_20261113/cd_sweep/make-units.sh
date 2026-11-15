#!/usr/bin/env bash
# units.tsv: "arm c q seed" in the frozen run order: sizes largest first; for each (q, seed) the four
# arms in a rotation that shifts with the seed (PROTOCOL.md "Units").
set -euo pipefail
cd "$(dirname "$0")"
arms=("C200 2.0" "C155 1.55" "C170 1.7" "C180 1.8")
for q in 4194271 2097091 1048507 524287 262111; do
  for s in 1 2 3; do
    for i in 0 1 2 3; do a=${arms[$(( (i + s - 1) % 4 ))]}; echo "${a% *} ${a#* } $q $s"; done
  done
done > units.tsv
wc -l < units.tsv
