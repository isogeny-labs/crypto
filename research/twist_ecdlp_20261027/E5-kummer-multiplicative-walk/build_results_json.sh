#!/bin/bash
# Builds results.json = analysis statistics (out/results.json, analyze.gp v2) + operation counts (out/opcounts_pooled.txt, ops_table.gp). No number is typed by hand.
set -eu
cd "$(dirname "$0")"
awk 'NR>2 {printf "%s{\"arm\":\"%s\",\"trials\":%s,\"total_steps\":%s,\"total_M\":%s,\"total_S\":%s,\"total_I\":%s,\"total_cand_tuples\":%s,\"mean_steps_per_trial\":%s,\"mean_M_per_trial\":%s,\"mean_S_per_trial\":%s,\"mean_I_per_trial\":%s}", (NR>3?",":""), $1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11}' out/opcounts_pooled.txt | { echo '['; cat; echo ']'; } > out/opcounts_pooled.json
jq --slurpfile ops out/opcounts_pooled.json '. + {primary_cost_metric:"counted field operations (M,S,I per trial) and walk steps; wall time secondary and contention-affected", operation_counts_pooled_b_ge_28_inst_1_2:$ops[0], operation_counts_per_cell_file:"out/opcounts.csv", prereg_prediction_H5_no_improvement:"held (see RESULTS.md)"}' out/results.json > results.json
jq -e . results.json > /dev/null && echo results.json ok
