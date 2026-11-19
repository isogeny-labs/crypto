#!/usr/bin/env bash
# EXP-PKM-RESULTANT-001: the protocol's cells, in order. Rows in runs/.
set -euo pipefail
cd "$(dirname "$0")"; mkdir -p runs
B=../../target/release/examples/pkm_algebraic_oracle
sha256sum "$B" | sed 's#  .*#  pkm_algebraic_oracle#' > runs/binary.sha256
P1=1399680001; P3=3221225473; SEED=20261114
deg() { python3 -c "print(','.join(['2']*$1))"; }
cell() { # name p t vkind
  "$B" --p "$2" --m 2 --degrees "$(deg "$3")" --membership resultant --vkind "$4" \
    --planted 4 --random 2 --seed $SEED --cell "$1" --out "runs/$1.jsonl" --resume 1 > /dev/null
  echo "done $1 $(date -u +%FT%TZ)" >> runs/progress.txt
}
for t in 4 5 6 7 8 9 10; do cell "P1-m2-t$t-res" $P1 $t coset; done
for t in $(seq 8 16); do cell "P3-m2-t$t-res" $P3 $t coset; cell "P3-m2-t$t-res-random" $P3 $t random; done
echo "end $(date -u +%FT%TZ)" >> runs/progress.txt
