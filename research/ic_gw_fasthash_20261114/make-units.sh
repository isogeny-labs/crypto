#!/usr/bin/env bash
# Writes units.tsv: pass arm bin q seed. Pass 1 rotates the arm order [REF HASH COMBO] by (seed-1); pass 2
# reverses pass 1's order for the same (q, seed). Sizes largest first. A/A arm REF2 (second REF run, same
# binary) at q = 4194271 and 2097091, seed 1: last in the (q,1) block of pass 1, first in pass 2.
arms=(REF HASH COMBO)
for pass in 1 2; do
  for q in 4194271 2097091 1048507 524287 262111; do
    for s in 1 2 3; do
      o=(); for i in 0 1 2; do o+=("${arms[$(( (i + s - 1) % 3 ))]}"); done
      if [ $pass = 2 ]; then o=("${o[2]}" "${o[1]}" "${o[0]}"); fi
      aa=0; if [ $s = 1 ] && { [ $q = 4194271 ] || [ $q = 2097091 ]; }; then aa=1; fi
      [ $pass = 2 ] && [ $aa = 1 ] && printf '%s\tREF2\tref\t%s\t%s\n' $pass $q $s
      for a in "${o[@]}"; do
        case $a in REF) b=ref;; HASH) b=hash;; COMBO) b=combo;; esac
        printf '%s\t%s\t%s\t%s\t%s\n' $pass $a $b $q $s
      done
      [ $pass = 1 ] && [ $aa = 1 ] && printf '%s\tREF2\tref\t%s\t%s\n' $pass $q $s
    done
  done
done
