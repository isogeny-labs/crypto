#!/bin/bash
# aggregation of out/summary.txt counts across p (post-run; no new experiment). Writes out/totals.txt
cd "$(dirname "$(readlink -f "$0")")"; OUT=${E6_OUT:-out}
S=$OUT/summary.txt
sum() { grep "^SUMMARY" $S | tr ' ' '\n' | grep "^$1=" | cut -d= -f2 | awk '{s+=$1} END{print s+0}'; }
{
for k in curves A_all_ok B0_n B0_ok B1_identity_nosol B1_g2zero B1_singular B1_sextic_nsf C_target_n match_target nJ_eq_prod quotients_ok nE_div_nJ ident_ok C_control_n match_control control_nJ_eq_prod C2_n brute_eq_pari N1_eq_p+1_target; do echo "TOTAL $k=$(sum $k)"; done
for k in hits pairs flagged_isomorphic; do
  echo "CTOTAL POS_NEG_NULL $k: POS=$(grep '^CSUMMARY POS' $S | tr ' ' '\n' | grep "^$k=" | cut -d= -f2 | awk '{s+=$1}END{print s+0}') NEGGAL=$(grep '^CSUMMARY NEGGAL' $S | tr ' ' '\n' | grep "^$k=" | cut -d= -f2 | awk '{s+=$1}END{print s+0}') NULL=$(grep '^CSUMMARY NULL' $S | tr ' ' '\n' | grep "^$k=" | cut -d= -f2 | awk '{s+=$1}END{print s+0}')"
done
echo "CTOTAL POS_n=$(grep '^CSUMMARY POS' $S | tr ' ' '\n' | grep '^n=' | cut -d= -f2 | awk '{s+=$1}END{print s}')  NEGGAL_pairs=$(grep '^CSUMMARY NEGGAL' $S | tr ' ' '\n' | grep '^pairs=' | cut -d= -f2 | awk '{s+=$1}END{print s}')"
} > $OUT/totals.txt
