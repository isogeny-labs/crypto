#!/usr/bin/env bash
# PROTOCOL sections 3-4. Counts are deterministic; wall time is not a measurement.
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"
d=research/genus4_followup_20261118
B=/tmp/claude-0/-home-user/c9b2fe43-c085-5dfa-9cde-b0311430c5d1/scratchpad/bin/genus_ladder.fu
c="--genera 4 --max-h 2"
P16="--primes 16411 --curves 3 --curve-seeds 16411=172/180/2640"
# Part A: tuning on p = 8209
for cc in 0.5 0.7 1.0 1.4 2.0; do
  $B run --primes 8209 --curves 6 --targets 3 --seeds 2 --arms x --cc $cc $c --threads 4 \
    --curve-seeds 8209=363/393/407/457/474/519 --out $d/a_cc$cc.jsonl > $d/a_cc$cc.log 2>&1
done
ccs=$(python3 $d/select.py)
echo "cc*=$ccs"
# Part B: IC arms, k = 0..17
$B run $P16 --targets 18 --seeds 2 --arms wx --cc 2.0 $c --threads 4 --out $d/b_wx.jsonl > $d/b_wx.log 2>&1
if [ "$ccs" != "2.0" ]; then
  $B run $P16 --targets 18 --seeds 2 --arms x --cc "$ccs" $c --threads 4 --out $d/b_xstar.jsonl > $d/b_xstar.log 2>&1
fi
# Part B: G3 determinism re-run (curve 172, k = 0, seed 1) beside the new Q runs (k = 3..17)
$B run --primes 16411 --curves 1 --curve-seeds 16411=172 --targets 1 --seeds 1 --arms Q --cc 2.0 $c --threads 1 \
  --rho-cap 5e8 --out $d/b_q_g3.jsonl > $d/b_q_g3.log 2>&1 &
$B run $P16 --targets 15 --target-from 3 --seeds 2 --arms Q --cc 2.0 $c --threads 4 --rho-cap 5e8 \
  --out $d/b_q.jsonl > $d/b_q.log 2>&1
wait
echo done
