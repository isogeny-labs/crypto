#!/usr/bin/env bash
# Run emin.c on the first U unsatisfiable seeds at size n (lfd_small.make_instance seeds 1, 2, ...).
# Usage: ./run_emin.sh N U EMAX MAXCOLS   -> appends JSON lines to out/c/emin_n<N>.jsonl
set -euo pipefail
n=$1; U=$2; emax=$3; maxcols=$4
out=out/c/emin_n$n.jsonl; mkdir -p out/c out/c/cert
found=0; seed=0
while [ "$found" -lt "$U" ]; do
  seed=$((seed+1))
  line=$(python3 emit_instance.py "$n" "$seed")
  nz=$(echo "$line" | ./emin count)
  [ "$nz" = "0" ] || continue
  found=$((found+1))
  cert=out/c/cert/n${n}_s${seed}; mkdir -p "$cert"
  start=$(date +%s.%N)
  res=$(echo "$line" | ./emin solve "$emax" "$maxcols" "$seed" "$cert" 2> "$cert/log.txt")
  end=$(date +%s.%N)
  echo "{\"seed\": $seed, \"wall_s\": $(awk -v a="$start" -v b="$end" "BEGIN{printf \"%.2f\", b-a}"), \"result\": $res}" | tee -a "$out"
done
