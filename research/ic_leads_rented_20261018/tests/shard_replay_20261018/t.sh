cd /home/user/crypto
export F4_F2_MAX_ROWS=50000000 F4_F2_MAX_COLS=50000000
for rng in "0 8" "40 44"; do set -- $rng
  for i in 0 1 2 3; do target/release/examples/dreg_ladder --cells 15:6:3 --m 4 --ffd-max 3 --seed 20261018 --shard $i/4 --first-draw $1 --max-draws $2 > /tmp/claude-0/-home-user-crypto/a42ede4f-e0d3-5bba-af4c-1210a64f87d5/scratchpad/shardtest/r$1.s$i.jsonl 2> /tmp/claude-0/-home-user-crypto/a42ede4f-e0d3-5bba-af4c-1210a64f87d5/scratchpad/shardtest/r$1.s$i.log & done; wait
done
echo done > /tmp/claude-0/-home-user-crypto/a42ede4f-e0d3-5bba-af4c-1210a64f87d5/scratchpad/shardtest/DONE
