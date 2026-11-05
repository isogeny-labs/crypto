#!/usr/bin/env bash
# cells/*.json + lead B / lead C reference cells -> cells.tsv (jq). One row per cell.
set -eu
D=$(cd "$(dirname "$0")" && pwd)
B=$D/../../curve25519_guess_optimum_20261102/analysis/cells.jsonl
{
printf 'id\tsrc\tk\tt\tG\ttr\ttd\treq\tcap\tmul\tamort\tcalls\tdeg\tund\thits\tver\tcov\tunch\tunch_all\treplays\trefuted\telapsed\tload\tlearn\tfmul\n'
jq -r '[.id,"this",.k,.t,.guess,(if .trace then 1 else 0 end),.targets_done,.targets_requested,.capped,.muladds_per_target,.amortised_muladds_per_target,.calls_per_target,.solving_degree_max,.undecided,.hits,.hits_verified,.coverage_exact,
 (((.native.pkm_system_fp_mults_uncharged//0)+(.native.pkm_grid_fp_mults_uncharged//0))/.f4_muladds),
 (((.native.pkm_system_fp_mults_uncharged//0)+(.native.pkm_grid_fp_mults_uncharged//0)+(.setup_native.pkm_trace_learn_spec_fp_mults_uncharged//0))/(.f4_muladds+(.trace_learn_muladds//0))),
 (.native.pkm_trace_replays//0),(.native.pkm_trace_refuted//0),.elapsed_s,(.load_at_start|split(",")[0]),(.trace_learn_muladds//0),.f4_muladds]|@tsv' "$D"/cells/*.json
jq -r 'select(.stage=="s1" and (.t==6 or .t==8 or .t==10) and (.guess==4 or .guess==(.t-1)))|["leadB-s1-t\(.t)-g\(.guess)","leadB",.k,.t,.guess,0,.targets_done,.targets_requested,.capped,.muladds_per_target,.amortised_muladds_per_target,.calls_per_target,.solving_degree_max,.undecided,.hits,.hits_verified,.coverage_exact,
 (((.native.pkm_system_fp_mults_uncharged//0)+(.native.pkm_grid_fp_mults_uncharged//0))/.f4_muladds),0,0,0,.elapsed_s,(.load_at_start|split(",")[0]),0,.f4_muladds]|@tsv' "$B"
} > "$D/cells.tsv"
wc -l "$D/cells.tsv"
