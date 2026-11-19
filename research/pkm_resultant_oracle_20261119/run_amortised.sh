#!/usr/bin/env bash
# EXP-PKM-RESULTANT-001 Amendment 1: (a) frozen cells rerun through the refactored oracle into runs/aa/
# (must reproduce the committed field_ops), (b) the amortised oracle on the same cells into runs/amortised/.
set -euo pipefail
cd "$(dirname "$0")"; mkdir -p runs/aa runs/amortised
B=../../target/release/examples/pkm_algebraic_oracle
sha256sum "$B" | sed 's#  .*#  pkm_algebraic_oracle#' > runs/amortised/binary.sha256
P1=1399680001; P3=3221225473; SEED=20261114
deg() { python3 -c "print(','.join(['2']*$1))"; }
cell() { # dir membership name p t vkind
  "$B" --p "$4" --m 2 --degrees "$(deg "$5")" --membership "$2" --vkind "$6" \
    --planted 4 --random 2 --seed $SEED --cell "$3" --out "runs/$1/$3.jsonl" --resume 1 > /dev/null
  echo "done $1/$3 $(date -u +%FT%TZ)" >> runs/amortised/progress.txt
}
for t in 4 8 12 16; do cell aa resultant "P3-m2-t$t-res" $P3 $t coset; done
cell aa resultant P1-m2-t6-res $P1 6 coset
for t in 4 5 6 7 8 9 10; do cell amortised resultant-amortized "P1-m2-t$t-res" $P1 $t coset; done
for t in $(seq 8 16); do
  cell amortised resultant-amortized "P3-m2-t$t-res" $P3 $t coset
  cell amortised resultant-amortized "P3-m2-t$t-res-random" $P3 $t random
done
echo "end $(date -u +%FT%TZ)" >> runs/amortised/progress.txt
