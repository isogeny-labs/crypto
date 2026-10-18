#!/usr/bin/env bash
# Means over the recorded lines of run.sh's raw files (awk only).
set -euo pipefail
OUT=${1:?outdir}
R="$OUT/raw"
awk '{d=$5+0; R[d]+=$8; P[d]+=$13; n[d]++}
  END{print "degree systems rows pivots zero"; for(d=2;d<=12;d++) if(n[d]) printf "%d %d %.2f %.2f %.2f\n",d,n[d],R[d]/n[d],P[d]/n[d],(R[d]-P[d])/n[d]}' \
  "$R/f4steps.txt" > "$OUT/f4_by_degree.txt"
grep '^LEADS' "$R/lin_breakdown_p101.log" | awk '
  $2 ~ /^nf=/ {split($2,a,"="); nf+=a[2]; n1++; s=$0; sub(/.*\[/,"",s); sub(/\].*/,"",s); m=split(s,v,", "); for(k=1;k<=m;k++) dc[k]+=v[k]}
  $2 ~ /^charpoly=/ {split($2,a,"="); cp+=a[2]; n2++}
  $2 ~ /^after_nf=/ {split($2,a,"="); an+=a[2]; split($3,b,"="); ro+=b[2]; n3++}
  END{printf "systems %d\nnf %.0f\ncharpoly_calls %d\ncharpoly_per_call %.0f\ncharpoly_per_system %.0f\nafter_nf %.0f\nroots %.0f\n",n1,nf/n1,n2,cp/n2,cp/n1,an/n3,ro/n3;
      for(k=1;k<=m;k++) printf "dense_cols_x%d %.2f\n",k,dc[k]/n1}' > "$OUT/lin_breakdown.txt"
# Per-test cost parts at p = 101 (stock build) and the trace comparison.
jq -r '.[] | "\(.p) \(.c_cov.mean) \(.f4_muls.mean) \(.lin_muls.mean) \(.weil_muls.mean) \(.post_muls.mean) \(.delta.mean) \(.planted_found) \(.mismatches)"' "$R/ccov.json" \
  | awk 'BEGIN{print "p c_cov f4 lin weil post delta planted mismatches"} {print}' > "$OUT/ccov_stock.txt"
jq -r '.[] | "\(.p) \(.c_cov.mean) \(.f4_muls.mean) \(.replayed) \(.trace_mismatches) \(.planted_found) \(.mismatches)"' "$R/ccov_trace.json" \
  | awk 'BEGIN{print "p c_cov f4 replayed trace_mismatches planted mismatches"} {print}' > "$OUT/ccov_trace.txt"
# Linear-stage parts: the remainder after NF, charpoly and root finding is the
# n·δ² combination plus the eigenvector solves.
awk 'NR==FNR{v[$1]=$2; next} $1==101{printf "f4 %.0f\ncharpoly %.0f\ncombination_and_eigenvectors %.0f\nnormal_forms %.0f\nroot_finding %.0f\nweil_and_post %.0f\nlin_total %.0f\nc_cov %.0f\n", $3, v["charpoly_per_system"], v["after_nf"]-v["charpoly_per_system"], v["nf"], v["roots"], $5+$6, $4, $2}' \
  "$OUT/lin_breakdown.txt" "$OUT/ccov_stock.txt" > "$OUT/breakdown_p101.txt"
