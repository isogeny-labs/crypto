#!/usr/bin/env bash
# Part 2 tables from cells/p2_* and the k=20 repro cells (jq + awk only).
cd "$(dirname "$0")/.."
{
for f in cells/p2_k*.json cells/repro_p2_*.json; do
  n=$(basename $f .json)
  jq -r --arg n "$n" '[$n,.k,.t,.guess,(if (.form|startswith("t0")) then "t0" else "plain" end),.columns,.base_points,.solving_degree_max,.poly_terms_per_specialisation,.muladds_per_target,"\(.solver_extra.max_rows_max)x\(.solver_extra.max_cols_max)",.solver_extra.max_nnz_max,.coverage_exact,.muladds_per_relation_at_exact_coverage,.hits,.hits_verified,"\(.planted_hits)/\(.planted_done)/\(.planted_exact)",.capped,.undecided,.targets_done]|@tsv' $f
done } > analysis/p2_cells.tsv
# ratios t0/plain per (k,t,G)
awk -F'\t' '{key=$2" "$3" "$4; if($5=="plain"){c[key]=$6;d[key]=$8;te[key]=$9;m[key]=$10;cv[key]=$13;pr[key]=$14}
 else {c0[key]=$6;d0[key]=$8;te0[key]=$9;m0[key]=$10;cv0[key]=$13;pr0[key]=$14}}
 END{for(k in c) if(k in c0) printf "%s\t%s/%s\t%s/%s\t%s/%s\t%.4f\t%.3f\t%.3f\n",k,c0[k],c[k],d0[k],d[k],te0[k],te[k],m0[k]/m[k],cv0[k]/cv[k],pr0[k]/pr[k]}' analysis/p2_cells.tsv | sort -k1,1n -k2,2n -k3,3n > analysis/p2_ratios.tsv
