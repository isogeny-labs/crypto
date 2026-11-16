#!/usr/bin/env bash
set -u
out=$1 binary=$2 variant=$3 arm=$4
mkdir -p "$out"
vector=1 batch=1 scalar=0
case "$variant" in
  baseline) vector=0; batch=0; scalar=1 ;;
  portable) vector=0 ;;
  scalar) scalar=1 ;;
  scalar-setup) batch=0 ;;
esac
cmd=(env RAYON_NUM_THREADS=1 KIC_GF2_WRITE_TABLE_ONCE=1 "KIC_GF2_VECTOR_TABLES=$vector" "KIC_ROOTFIND_BATCH_SETUP=$batch" "KIC_GENERALIZED_SCALAR_GRAY=$scalar"
  timeout 300 valgrind --tool=callgrind --instr-atstart=no --separate-threads=no "--callgrind-out-file=$out/region.callgrind" "$binary" --profile 1 --arm "$arm")
printf '%q ' "${cmd[@]}" > "$out/command.txt"
printf '\n' >> "$out/command.txt"
"${cmd[@]}" > "$out/run.jsonl" 2> "$out/stderr.txt"
code=$?
printf '%s\n' "$code" > "$out/exit.txt"
exit "$code"
