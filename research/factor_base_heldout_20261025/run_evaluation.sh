#!/usr/bin/env bash
set -u
study=research/factor_base_heldout_20261025
binary="$study/target/release/heldout"
selection="$study/results/selection.json"
test -s "$selection" || { echo 'training selection missing' >&2; exit 2; }
policy=$(jq -er '.selected_policy' "$selection")
denominator=$(jq -er --arg policy "$policy" '.candidate_scores[] | select(.policy == $policy) | .base_denominator' "$selection")
acceptance=$(jq -er --arg policy "$policy" '.candidate_scores[] | select(.policy == $policy) | .large_prime_acceptance' "$selection")
jumps=$(jq -er --arg policy "$policy" '.candidate_scores[] | select(.policy == $policy) | .jumps' "$selection")
sha256sum "$selection" > "$study/results/selection.SHA256"
status=0
for bits in 16 20 24 28 32 36 40; do
  stem="$study/results/raw/evaluation-bits${bits}"
  if /usr/bin/time -f 'elapsed_s=%e exit=%x' "$binary" "$bits" "$denominator" "$jumps" "$acceptance" evaluation > "$stem.jsonl" 2> "$stem.time"; then
    printf '%s rows=%s status=ok\n' "$stem" "$(wc -l < "$stem.jsonl")"
  else
    code=$?
    printf '%s rows=%s status=failed exit=%s\n' "$stem" "$(wc -l < "$stem.jsonl")" "$code"
    status=1
  fi
done
exit "$status"
