#!/usr/bin/env bash
# Writes launch/units.tsv: one line per unit "arm binary q seed c_D" (PROTOCOL.md, "Units").
# Sizes and seeds are the frozen N4 v2 grid's (k = 12..22; seeds 1..5 for k <= 16, 1..3 for k >= 17).
# c rule: c_D = 1.55 for k >= 18 (q >= 2^17), else 2.0; at k <= 17 CRULE = BASE and BOTH = CHAIN,
# so those duplicate units are not run.
set -euo pipefail
cd "$(dirname "$0")"
small="65479 32719 16363 8191 4051"; mid="131071"; big="4194271 2097091 1048507 524287 262111"
{
  for q in $big; do for s in 1 2 3; do
    echo "BASE v2 $q $s 2.0"; echo "CHAIN v3 $q $s 2.0"; echo "CRULE v2 $q $s 1.55"; echo "BOTH v3 $q $s 1.55"
  done; done
  for q in $mid; do for s in 1 2 3; do echo "BASE v2 $q $s 2.0"; echo "CHAIN v3 $q $s 2.0"; done; done
  for q in $small; do for s in 1 2 3 4 5; do echo "BASE v2 $q $s 2.0"; echo "CHAIN v3 $q $s 2.0"; done; done
} > units.tsv
wc -l < units.tsv
