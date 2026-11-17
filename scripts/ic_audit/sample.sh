#!/bin/sh
# sample.sh <row_id> <N>   candidates (run ids) on stdin, one per line, in source order.
# Prints the N candidates with the smallest sha256(seed|row_id|run_id), as frozen in REPLAY_SEED.md.
seed=$(sed -n 's/^    seed = \(.*\)$/\1/p' "$(dirname "$0")/../REPLAY_SEED.md")
row="$1"; n="$2"
while IFS= read -r id; do
  k=$(printf '%s|%s|%s' "$seed" "$row" "$id" | sha256sum | cut -d' ' -f1)
  printf '%s\t%s\n' "$k" "$id"
done | sort | head -n "$n" | cut -f2
