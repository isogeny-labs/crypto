#!/usr/bin/env bash
# Frozen rule (PROTOCOL s2): N starts at 1500, extends by 500-seed blocks until enough admitted. Resumable.
cd /home/user/crypto/research/genus_ladder_scaling_20261119
adm() { cat search_g$1_p$2_*.jsonl 2>/dev/null | grep -o '"admitted":[0-9]*' | awk -F: '{s+=$2}END{print s+0}'; }
for cell in "5 4099 3" "5 2357 5" "4 54293 3"; do
  set -- $cell; G=$1; P=$2; NEED=$3
  bash search.sh $G $P 1 1500
  lo=1501
  while [ "$(adm $G $P)" -lt "$NEED" ]; do
    if [ "$lo" -gt 3000 ] && [ "$(adm $G $P)" -eq 0 ]; then echo "cell g$G p$P: nothing admitted by seed 3000, stopping"; break; fi
    bash search.sh $G $P $lo $((lo+499)); lo=$((lo+500))
  done
  echo "cell g$G p$P admitted $(adm $G $P) covered 1-$((lo-1))"
done
