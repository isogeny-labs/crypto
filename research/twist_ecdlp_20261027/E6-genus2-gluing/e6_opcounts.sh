#!/bin/bash
# e6_opcounts.sh -- OPERATION COUNTS derived only from the raw files OUT/main_p*.txt and OUT/ctrl_p*.txt
# (post-hoc helper; no randomness, no new experiment).  Usage: E6_OUT=out E6_PS=101,211,... bash e6_opcounts.sh
#   poly_evals_Fp_deg3  : integer-polynomial evaluations at u=0..p-1 in affcount loops: 2 per Stage-C record
#                         (quotient curves n1,n2) + 2 per PC1 control sample
#   poly_evals_Fp_deg6  : 1 per brute record (affcount of the sextic, N1)
#   poly_evals_Fp2_deg6 : p^2 per brute record (brutecount N2 loop over a+b*w in F_{p^2})
#   cand_perm_tuples    : root-permutation candidates: B0 scans 6 per curve, B1 re-interpolates n_equiv per curve
#   model_*             : task cost model (rho steps sqrt(pi*l/2); exponent-1 IC = p), NOT measured
cd "$(dirname "$0")"
OUT=${E6_OUT:-out}; PS=${E6_PS:-101,211,409,1009,4001,10007,20011}
{
for p in ${PS//,/ }; do
  awk -v p=$p '
  function fld(k,  i){for(i=1;i<=NF;i++) if(index($i,k"=")==1) return substr($i,length(k)+2); return ""}
  function med(a,n,  i,j,t){for(i=2;i<=n;i++){t=a[i];for(j=i-1;j>=1&&a[j]>t;j--)a[j+1]=a[j];a[j+1]=t} return a[int((n+1)/2)]}
  $1=="A"{nA++} $1=="B0"{nEq+=fld("n_equiv")}
  $1=="C"{nC++} $1=="C2"{nC2++}
  $1=="D"{n++; nE=fld("nE"); rl[n]=fld("rho_steps")+0; rf[n]=sqrt(3.141592653589793*nE/2); rj[n]=sqrt(3.141592653589793*nE*(2*p+2-nE)/2)}
  END{ml=med(rl,n);mf=med(rf,n);mj=med(rj,n);
   printf "MAIN p=%d curves=%d cand_perm_tuples=%d C_records=%d brute_records=%d poly_evals_Fp_deg3_main=%d poly_evals_Fp_deg6=%d poly_evals_Fp2_deg6=%d model_rho_ops_largest_prime_median=%.6g model_rho_ops_full_group_median=%.6g model_rho_ops_on_Jac_median=%.6g model_IC_ops_exponent1=%d model_ratio_IC_over_rho_full_median=%.6g model_ratio_IC_over_rho_largest_prime_median=%.6g\n",
     p,nA,6*nA+nEq,nC,nC2,2*p*nC,p*nC2,p*p*nC2,ml,mf,mj,p,p/mf,p/ml}' $OUT/main_p$p.txt
  [ -f $OUT/ctrl_p$p.txt ] && awk -v p=$p '
  function fld(k,  i){for(i=1;i<=NF;i++) if(index($i,k"=")==1) return substr($i,length(k)+2); return ""}
  $1=="POS"{pn=fld("n")} $1=="NEGGAL"{nn=fld("pairs")} $1=="NULL"{ln=fld("n");lt=fld("tries")}
  END{printf "CTRL p=%d PC1_samples=%d PC1_poly_evals_Fp_deg3=%d PC1_hyperellcharpoly_calls=%d NC1_pairs=%d NC2_accepted_charpolys=%d NC2_sampler_tries=%d\n",p,pn,2*p*pn,pn,nn,ln,lt}' $OUT/ctrl_p$p.txt
done
} > "$OUT/.opc_tmp"
cat "$OUT/.opc_tmp"
awk '
  /^MAIN/{for(i=2;i<=NF;i++){split($i,kv,"="); if(kv[1]=="curves"||kv[1]=="cand_perm_tuples"||kv[1]=="C_records"||kv[1]=="brute_records"||kv[1]=="poly_evals_Fp_deg3_main"||kv[1]=="poly_evals_Fp_deg6"||kv[1]=="poly_evals_Fp2_deg6") t[kv[1]]+=kv[2]}}
  /^CTRL/{for(i=2;i<=NF;i++){split($i,kv,"="); if(kv[1]!="p") c[kv[1]]+=kv[2]}}
  END{s="TOTAL_MAIN"; for(k in t) s=s" "k"="t[k]; print s; s="TOTAL_CTRL"; for(k in c) s=s" "k"="c[k]; print s}' "$OUT/.opc_tmp"
rm -f "$OUT/.opc_tmp"
