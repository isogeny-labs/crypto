#!/bin/sh
# opcounts.sh -- operation-count metrics for E1 (derived from recorded n, m, targets_done and solution counts in summary.tsv; no new gp run)
# Detector A cost model (counted by construction from e1_census.gp, verified by opcount_verify.gp on instrumented configs, see opcounts.txt):
#   m=3 table build : n(n+1)/2 pkeys calls (each = one S_3 root-set evaluation: 1 inversion + O(1) field mults)
#   m=3 per target  : n pkeys calls + <=2n sorted-table lookups (bisection, ~log2(#entries) comparisons each)
#   m=2 per target  : n pkeys calls + <=2n hash-map lookups
#   brute-force equivalent (Oracle B, not run at scale): m=2 n(n+1)/2 S_3 evals per target; m=3 n(n+1)(n+2)/6 closed resultants per target
S=${1:-summary.tsv}
awk -F'\t' 'BEGIN{OFS="\t"; print "tag","m","n","targets_done","pkeys_table_build","pkeys_per_targets","lookups_max_per_targets","distinct_solutions","bruteforce_tuples_per_targets"}
 NR>1{m=$5;n=$14;td=$18;sol=$21+$22+$23+$24;
  tb=(m==3)?n*(n+1)/2:0; pt=n*td; lk=2*n*td;
  bf=(m==3)?n*(n+1)*(n+2)/6*td:n*(n+1)/2*td;
  print $1,m,n,td,tb,pt,lk,sol,bf;
  T_tb[m]+=tb;T_pt[m]+=pt;T_lk[m]+=lk;T_sol[m]+=sol;T_bf[m]+=bf;T_cfg[m]++;T_td[m]+=td}
 END{for(m=2;m<=3;m++) print "TOTAL_m"m, T_cfg[m]" configs", "", T_td[m]" targets", T_tb[m], T_pt[m], T_lk[m], T_sol[m], T_bf[m]}' $S
