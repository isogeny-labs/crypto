#!/bin/bash
# Find unsatisfiable T-type n=31,k=16 seeds (positive control T31); appends to out/instances.jsonl
cd "$(dirname "$0")"
L=../../../lfd_lower_bound_20261111; found=0
for s in $(seq 13 400); do
  line=$(python3 $L/emit_instance_T.py 31 $s 16); c=$(echo "$line" | $L/emin count)
  echo "{\"type\": \"T\", \"n\": 31, \"k\": 16, \"seed\": $s, \"sols\": $c, \"line\": \"$line\"}" >> out/instances.jsonl
  [ "$c" = 0 ] && found=$((found+1)); [ $found -ge 2 ] && break
done
