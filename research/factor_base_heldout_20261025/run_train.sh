#!/usr/bin/env bash
set -u
study=research/factor_base_heldout_20261025
binary="$study/target/release/heldout"
mkdir -p "$study/results/raw"
status=0
for denominator in 2 4; do
  for bits in 16 20 24 28 32 36 40; do
    stem="$study/results/raw/train-fb${denominator}-both-j16-bits${bits}"
    if test -s "$stem.time" && test -s "$stem.jsonl" && test "$(wc -l < "$stem.jsonl")" -eq 120 && rg -q 'exit=0' "$stem.time"; then
      printf '%s rows=120 status=already-complete\n' "$stem"
      continue
    fi
    if /usr/bin/time -f 'elapsed_s=%e exit=%x' "$binary" "$bits" "$denominator" 16 both train > "$stem.jsonl" 2> "$stem.time"; then
      printf '%s rows=%s status=ok\n' "$stem" "$(wc -l < "$stem.jsonl")"
    else
      code=$?
      printf '%s rows=%s status=failed exit=%s\n' "$stem" "$(wc -l < "$stem.jsonl")" "$code"
      status=1
    fi
  done
done
exit "$status"
