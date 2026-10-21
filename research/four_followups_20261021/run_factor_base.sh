#!/usr/bin/env bash
set -euo pipefail

study=research/four_followups_20261021
binary="$study/target/release/factor-base"
test -x "$binary"
mkdir -p "$study/results"

run_arm() {
  local denominator="$1" jumps="$2" bits="$3"
  local stem="$study/results/fb${denominator}-j${jumps}-bits${bits}"
  if /usr/bin/time -f 'elapsed_s=%e exit=%x' "$binary" "$bits" "$denominator" "$jumps" > "$stem.jsonl" 2> "$stem.time"; then
    printf '%s status=ok rows=%s\n' "$stem" "$(wc -l < "$stem.jsonl")"
  else
    local status=$?
    printf '%s status=failed exit=%s rows=%s\n' "$stem" "$status" "$(wc -l < "$stem.jsonl")"
    return "$status"
  fi
}

arm() {
  local denominator="$1" jumps="$2" bits
  for bits in 16 20 24 28 32 36 40; do
    run_arm "$denominator" "$jumps" "$bits"
  done
}

arm 1 16 & p1=$!
arm 2 16 & p2=$!
arm 4 16 & p3=$!
arm 1 8  & p4=$!
arm 1 32 & p5=$!
status=0
for pid in "$p1" "$p2" "$p3" "$p4" "$p5"; do
  wait "$pid" || status=1
done
exit "$status"
