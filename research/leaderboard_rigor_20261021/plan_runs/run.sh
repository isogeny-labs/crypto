#!/bin/bash
# `ecbench verify --replay 12` (current main binary) on every complete session the board cites that the replay plan
# does not list yet; one TSV line per session: dir, replays, reproduced, ok.
cd /home/user/crypto
EB=target-main/release/ecbench; OUT=research/leaderboard_rigor_20261021/plan_runs
for s in research/ecbench_n37_rank_columns_20261004/sessions/mac_arm64_l0_01 \
         research/ecbench_n37_k8_k16_20261004/sessions/mac_arm64_l0_01 \
         research/ecbench_n37_online_ir_20261004/sessions/mac_arm64_l0_01 \
         research/ecbench_n37_native_online_wall_20261004/sessions/hosted_ubuntu_01 \
         research/ecbench_n41_n53_shared_rank_20261005/sessions/mac-l0-02 \
         research/ecbench_conductor_gap_20261012/sessions/ell8209-q29 \
         research/ecbench_conductor_gap_20261012/sessions/ell131101-q37 \
         research/ecbench_rho_optimized_20261014/sessions/run-20261014T012000Z \
         research/ic_oracle_setup_blocked_20261020/sessions/A-1k-8k \
         research/ic_oracle_setup_blocked_20261020/sessions/B-16k-r2 \
         research/f6_ic_ecbench_ladder_20261005/sessions/n7 \
         research/f6_ic_ecbench_ladder_20261005/sessions/n23 \
         research/f4_f5_f6_cross_family_20261011/sessions/R13 \
         research/f4_f5_f6_cross_family_20261011/sessions/R19 \
         research/f4_f5_f6_cross_family_20261011/sessions/X19; do
  tag=$(echo "$s" | sed 's#research/##; s#/sessions/#__#; s#/#_#g')
  timeout 1500 $EB verify --dir "$s" --replay 12 --out "$OUT/$tag.receipt.json" > /dev/null 2> "$OUT/$tag.stderr"
  if [ -s "$OUT/$tag.receipt.json" ]; then
    jq -r --arg d "$s" '[$d, (.replays|length), ([.replays[]|select(.reproduced)]|length), .ok]|@tsv' "$OUT/$tag.receipt.json" >> $OUT/results.tsv
  else echo -e "$s\tNO_RECEIPT\t\t" >> $OUT/results.tsv; fi
done
echo done >> $OUT/results.tsv
