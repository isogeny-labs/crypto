#!/usr/bin/env bash
# units.tsv: "label arm bin q seed pass". Largest size first. Pass 1: REF,CHAIN for odd seeds, CHAIN,REF for even;
# pass 2: reversed. All of pass 1 first, then pass 2, then the A/A units (REF twice, adjacent). PROTOCOL.md.
set -euo pipefail
cd "$(dirname "$0")"
{
for p in 1 2; do
  for q in 4194271 2097091 1048507 524287 262111; do
    for s in 1 2 3; do
      if [ $(( (s + p) % 2 )) -eq 0 ]; then o="REF CHAIN"; else o="CHAIN REF"; fi
      # pass 1 odd seed: REF first (s+p even); pass 1 even seed: CHAIN first; pass 2 reversed
      for a in $o; do b=v3; [ $a = CHAIN ] && b=v3c; echo "${a}_p${p} $a $b $q $s $p"; done
    done
  done
done
for q in 4194271 2097091; do for k in 1 2; do echo "AA${k} REF v3 $q 1 3"; done; done
} > units.tsv
wc -l < units.tsv
