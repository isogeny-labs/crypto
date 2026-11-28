#!/usr/bin/env bash
# IC replays of the nonce plan (out/plan.json) with the validator's build; flags as run.sh.
set -u
B=/tmp/claude-0/-home-user/c9b2fe43-c085-5dfa-9cde-b0311430c5d1/scratchpad/fuval/bin/genus_ladder.val
O=/home/user/crypto/research/genus4_followup_20261118/validation/out/replay
PL=/home/user/crypto/research/genus4_followup_20261118/validation/out/plan.json
mkdir -p $O
c="--genera 4 --max-h 2"
echo "start $(date -u +%FT%TZ) $(sha256sum $B)"
python3 -c "import json; [print(c[1], c[2]) for c in json.load(open('$PL'))['B_ic_cells']]" | while read cs k; do
  $B run --primes 16411 --curves 1 --curve-seeds 16411=$cs --targets 1 --target-from $k --seeds 2 --arms wx --cc 2.0 $c \
    --threads 1 --out $O/b_${cs}_k$k.jsonl > $O/b_${cs}_k$k.log 2>&1 || echo "FAIL b $cs $k"
done
python3 -c "import json; [print(a['cc'], a['cell'][1], a['cell'][2]) for a in json.load(open('$PL'))['A_cells']]" | while read cc cs k; do
  $B run --primes 8209 --curves 1 --curve-seeds 8209=$cs --targets 1 --target-from $k --seeds 2 --arms x --cc $cc $c \
    --threads 1 --out $O/a_cc${cc}_${cs}_k$k.jsonl > $O/a_cc${cc}_${cs}_k$k.log 2>&1 || echo "FAIL a $cc $cs $k"
done
echo "end $(date -u +%FT%TZ)"
