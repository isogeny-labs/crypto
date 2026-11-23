#!/usr/bin/env bash
# Writes units.tsv: arm bin q seed. Sizes largest first, seeds 1-6, REF and BLR adjacent per (q, seed).
for q in 4194271 2097091 1048507 524287 262111; do
  for s in 1 2 3 4 5 6; do
    printf 'REF\tref\t%s\t%s\n' $q $s
    printf 'BLR\tblr\t%s\t%s\n' $q $s
  done
done
