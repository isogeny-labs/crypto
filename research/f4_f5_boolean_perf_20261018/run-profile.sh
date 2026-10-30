#!/usr/bin/env bash
set -u
# Thin orchestration only. Native analyzer validates and compares all records.
out=$1
binary=$2
shift 2
mkdir -p "$out"
printf '%q ' env RAYON_NUM_THREADS=1 "KIC_GF2_WRITE_TABLE_ONCE=${KIC_GF2_WRITE_TABLE_ONCE:-0}" timeout 300 valgrind --tool=callgrind --instr-atstart=no --separate-threads=no "--callgrind-out-file=$out/region.callgrind" "$binary" --profile 1 "$@" > "$out/command.txt"
printf '\n' >> "$out/command.txt"
RAYON_NUM_THREADS=1 timeout 300 valgrind --tool=callgrind --instr-atstart=no \
    --separate-threads=no --callgrind-out-file="$out/region.callgrind" \
    "$binary" --profile 1 "$@" > "$out/run.jsonl" 2> "$out/stderr.txt"
status=$?
printf '%s\n' "$status" > "$out/exit.txt"
exit "$status"
