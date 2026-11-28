#!/usr/bin/env bash
# PROTOCOL section 2: chunked, resumable prime-order search. usage: search.sh G P LO HI
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"
d=research/genus_ladder_scaling_20261119
B=/tmp/claude-0/-home-user/c9b2fe43-c085-5dfa-9cde-b0311430c5d1/scratchpad/bin/genus_ladder.po
G=$1; P=$2; LO=$3; HI=$4
seeds=$(python3 -c "print(' '.join(str(x) for x in range($LO, $HI + 1)))")
read -ra S <<< "$seeds"
for w in 0 1 2 3; do
  (
    mine=(); for ((i = w; i < ${#S[@]}; i += 4)); do mine+=("${S[$i]}"); done
    for ((j = 0; j < ${#mine[@]}; j += 28)); do
      chunk=("${mine[@]:j:28}"); list=$(IFS=/; echo "${chunk[*]}")
      out=$d/search_g${G}_p${P}_r${LO}-${HI}_w${w}_c${j}.jsonl
      [ -s "$out" ] && grep -q '"cell"' "$out" && continue
      $B run --primes $P --genera $G --max-h 1 --curves 100000 --targets 1 --seeds 1 --arms n --threads 1 \
        --curve-seeds $P=$list --out $out > ${out%.jsonl}.log 2>&1
    done
  ) &
done
wait
echo done
