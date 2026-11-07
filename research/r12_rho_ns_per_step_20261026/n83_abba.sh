#!/bin/zsh
W=/Volumes/SSD990-2/crypto/.claude/worktrees/wf_00569b82-b8b-5
B=$W/target/release/examples/n83_rho_optimization_eval
K=$W/research/n83_evaluation_20261012/optimization
C=$K/records/panel/kernel/configs
O=/tmp/claude-501/r12/n83
mkdir -p $O
i=0
for t in training_1 training_2; do
  for order in "wide fold fold wide" "fold wide wide fold"; do
    for b in ${=order}; do
      i=$((i+1))
      $B $K/fixtures/$t.jsonl 202610128311 $C/$b-L32-DP4-E0.json $O/${i}_${t}_$b.json > $O/${i}_${t}_$b.out 2>&1
    done
  done
done
echo finished > $O/DONE
