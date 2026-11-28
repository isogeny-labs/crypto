#!/bin/bash
# Frozen driver (PROTOCOL.md): one instance per p, 16 measured trials after 2 warm-ups.
cd /home/user/crypto
for P in 65537 131101 262147; do
  target/release/examples/kappa_g3 research/genus_ladder_g3_highp_20261121/run_$P.jsonl \
    research/genus_ladder_kappa_20261122/raw/kappa_$P.jsonl 16 \
    > research/genus_ladder_kappa_20261122/raw/kappa_$P.log 2>&1
done
echo done > research/genus_ladder_kappa_20261122/raw/all_done
