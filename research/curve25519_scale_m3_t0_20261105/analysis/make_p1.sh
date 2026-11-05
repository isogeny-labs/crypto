#!/usr/bin/env bash
# Part 1 tables from cells/p1_* and repro_p1 (jq + awk only).
cd "$(dirname "$0")/.."
{
for f in cells/p1_k*.json cells/repro_p1_*.json; do
  [ -e "$f" ] || continue
  case $f in *.fail.json.json) continue;; esac
  n=$(basename $f .json); n=${n%.fail}
  # name k form t G m mode status targets cols pts mul/tgt deg calls hits hv und cov perrel obs wall
  jq -r --arg n "$n" 'if (.skipped//false) then [$n,"-","-","-","-","-","-","skipped: "+.reason,"-","-","-","-","-","-","-","-","-","-","-","-","-"]
   elif (.failed//false) then [$n,.k,.form,.t,.guess,.m,.mode,"failed:"+.why+" rc="+(.rc|tostring),"-","-","-","-","-","-","-","-","-","-","-","-","-"]
   else [$n,.k,.form,.t,.guess,.m,.mode,(if .capped then "capped" else "ok" end),.targets_done,.columns,.base_points,.muladds_per_target,.solving_degree_max,.calls_per_target,.hits,.hits_verified,.undecided,.coverage_exact,
     (if (.f4_muladds>0 and .undecided<.targets_done) then .muladds_per_relation_at_exact_coverage else "-" end),
     (if (.hits>0 and .f4_muladds>0) then (.f4_muladds/.hits) else "-" end),.wall_s] end | map(tostring) | join("\t")' $f
done } | sort -t$'\t' -k2,2n -k3,3 -k4,4n -k5,5n -k6,6n -k7,7 > analysis/p1_cells.tsv
# compare m3 against m2 per (k,form,t,G)
awk -F'\t' '$6==2{pm[$2" "$3" "$4" "$5]=$19; hm[$2" "$3" "$4" "$5]=$12}
 $6==3{k=$2" "$3" "$4" "$5; if($19!="-" && (k in pm) && pm[k]!="-") printf "%s\t%s\t%.3e\t%.3e\t%.2f\n",k,$7,$19,pm[k],$19/pm[k]}' analysis/p1_cells.tsv > analysis/p1_compare.tsv
