#!/bin/sh
# opcounts_summary.sh -- totals JSON from opcounts.tsv (operation counts primary) and summary.tsv (wall seconds, secondary, contention-affected)
awk -F'\t' 'FNR==NR{ if(FNR>1&&$1!~/^TOTAL/){m=$2; tb[m]+=$5; pt[m]+=$6; lk[m]+=$7; sol[m]+=$8; bf[m]+=$9; c[m]++; td[m]+=$4} next}
 FNR>1{m=$5; tab[m]+=$48; loop[m]+=$49}
 END{printf "{\n\"note\":\"operation counts derived by construction from recorded n, m, targets_done and solution counts; verified exactly against instrumented gp runs (opcounts_verify.txt); wall seconds are secondary and contention-affected (~20 agents on 4 cores)\",\n";
  for(m=2;m<=3;m++){ printf "\"m%d\":{\"configs\":%d,\"targets\":%d,\"pkeys_table_build\":%d,\"pkeys_per_target_loops\":%d,\"pkeys_total\":%d,\"lookups_upper_bound\":%d,\"distinct_solutions_reported\":%d,\"bruteforce_equivalent_tuples\":%.0f,\"bruteforce_over_pkeys_ratio\":%.1f,\"wall_table_s\":%.1f,\"wall_target_loop_s\":%.1f}%s\n", m,c[m],td[m],tb[m],pt[m],tb[m]+pt[m],lk[m],sol[m],bf[m],bf[m]/(tb[m]+pt[m]),tab[m],loop[m],(m==2?",":",")}
  tp=tb[2]+pt[2]+tb[3]+pt[3]; tbf=bf[2]+bf[3];
  printf "\"all\":{\"configs\":%d,\"targets\":%d,\"pkeys_total\":%d,\"distinct_solutions_reported\":%d,\"bruteforce_equivalent_tuples\":%.0f}\n}\n",c[2]+c[3],td[2]+td[3],tp,sol[2]+sol[3],tbf}' ${1:-opcounts.tsv} ${2:-summary.tsv}
