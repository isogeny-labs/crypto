#!/usr/bin/env bash
set -euo pipefail
out=/home/ec2-user/ml-dataset-requirements-audit-20261107/evidence/retained-cells.jsonl
: > "$out"
for pair in koblitz23-m3-15-enum:26 binary23-m3-15-enum:26 binary47-m4-f6-enum:4 koblitz47-m4-f6-enum:16 prime61-root-enum:4 n24-n25-cross-family:1 n26-n31-cross-family:1; do
 name=${pair%:*}; count=${pair#*:}
 for ((i=0;i<count;i++)); do
  if [[ "$name" == n24-* || "$name" == n26-* ]]; then root=/home/ec2-user/ecdlp/${name}-20261101; else printf -v shard '%02d' "$i"; root=/mnt/spot-codex-state/tmp/ml-dataset-${name}-20261031/shard-${shard}/runner/attempt; fi
  for manifest in "$root"/cells/*/manifest.json; do
   sha=$(sha256sum "$manifest"); sha=${sha%% *}
   jq -c --arg path "$manifest" --arg cohort "$name" --arg sha "$sha" '{cohort:$cohort,manifest:$path,manifest_sha256:$sha,schema,source_commit,binary_sha256,config:{family:.config.family,n:.config.n,m:.config.m,twist:.config.twist,curve_seed:.config.curve_seed,generator_index:.config.generator_index,key_index:.config.key_index,solver:.config.solver,factor_base_size:.config.factor_base_size,factor_base_policy:.config.factor_base_policy,degree_bound:.config.degree_bound,work_limit:.config.work_limit},outcome:{status:.outcome.status,solver_completed:.outcome.solver_completed,known_answer_verified:.outcome.known_answer_verified,pdp_witness_verified:.outcome.pdp_witness_verified,exhaustive_no_decomposition:.outcome.exhaustive_no_decomposition,scalar_recovered_without_witness:.outcome.scalar_recovered_without_witness}}' "$manifest" >> "$out"
  done
 done
done
jq -s '{rows:length,cohorts:(group_by(.cohort)|map({cohort:.[0].cohort,rows:length,families:([.[].config.family]|unique),n:([.[].config.n]|unique),m:([.[].config.m]|unique),twists:([.[].config.twist]|unique),methods:([.[].config.solver]|unique),generator_indices:([.[].config.generator_index]|unique),key_indices:([.[].config.key_index]|unique),factor_base_sizes:([.[].config.factor_base_size]|unique),explicit_factor_base_policies:([.[].config.factor_base_policy]|unique),statuses:(group_by(.outcome.status)|map({status:.[0].outcome.status,rows:length})),solver_completed_true:([.[]|select(.outcome.solver_completed==true)]|length),known_answer_verified_true:([.[]|select(.outcome.known_answer_verified==true)]|length),pdp_witness_verified_true:([.[]|select(.outcome.pdp_witness_verified==true)]|length),scalar_recovered_without_witness_true:([.[]|select(.outcome.scalar_recovered_without_witness==true)]|length)}))}' "$out" > /home/ec2-user/ml-dataset-requirements-audit-20261107/evidence/retained-cell-summary.json
