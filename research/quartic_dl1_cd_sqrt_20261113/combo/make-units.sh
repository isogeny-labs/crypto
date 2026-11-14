#!/usr/bin/env bash
# units.tsv: "arm bin c q seed", largest size first, the three arms rotated by seed (PROTOCOL.md).
set -euo pipefail
cd "$(dirname "$0")"
arms=("BASE v2 2.0" "C170 v2 1.7" "B170 v3 1.7")
for q in 4194271 2097091 1048507 524287 262111; do
  for s in 1 2 3; do
    for i in 0 1 2; do echo "${arms[$(( (i + s - 1) % 3 ))]} $q $s"; done
  done
done > units.tsv
wc -l < units.tsv
