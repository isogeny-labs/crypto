#!/usr/bin/env bash
# DEVIATIONS D1: chunked re-scan. usage: search_chunks.sh P LO HI RES (RES = residue mod 4, or "all")
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"
d=research/genus4_prime_order_20261118
B=/tmp/claude-0/-home-user/c9b2fe43-c085-5dfa-9cde-b0311430c5d1/scratchpad/bin/genus_ladder.po
P=$1; LO=$2; HI=$3; RES=$4
seeds=$(python3 -c "print(' '.join(str(x) for x in range($LO, $HI + 1) if '$RES' == 'all' or x % 4 == int('$RES')))")
read -ra S <<< "$seeds"
for w in 0 1 2 3; do
  (
    mine=(); for ((i = w; i < ${#S[@]}; i += 4)); do mine+=("${S[$i]}"); done
    for ((j = 0; j < ${#mine[@]}; j += 28)); do
      chunk=("${mine[@]:j:28}"); list=$(IFS=/; echo "${chunk[*]}")
      out=$d/search_p${P}_r${LO}-${HI}-${RES}_w${w}_c${j}.jsonl
      [ -s "$out" ] && grep -q '"cell"' "$out" && continue
      $B run --primes $P --genera 4 --max-h 1 --curves 100000 --targets 1 --seeds 1 --arms n --threads 1 \
        --curve-seeds $P=$list --out $out > ${out%.jsonl}.log 2>&1
    done
  ) &
done
wait
echo done
