#!/bin/sh
# extra_stats.sh -- descriptive re-tabulation of summary.tsv (no new data); usage: extra_stats.sh summary.tsv
S=${1:-summary.tsv}
echo "## A. m=3, nT>0, fb!=twistmult: count-weighted mean of f*q/nT (configs with M_nd>0), by bits,nT,fb"
awk -F'\t' 'NR>1 && $5==3 && $12>0 && $6!="twistmult" && $23>0 {k=$2"\tnT="$12"\t"$6; w=$21+$23; s[k]+=$42*w; W[k]+=w; M[k]+=$23; E[k]+=$21; c[k]++}
 END{for(k in s) printf "%s\tconfigs=%d\tM_nd=%d\tE_nd=%d\tweighted_f_q_over_nT=%.4f\n",k,c[k],M[k],E[k],s[k]/W[k]}' $S | sort
echo "## B. H3 random FB: configs, ratio_naive in [0.7,1.4], ratio_ref in [0.7,1.4]"
awk -F'\t' 'NR>1 && $6=="random" {n++; if($46=="PASS")a++; if($47=="PASS")b++; if($46=="FAIL")print "H3_naive_FAIL",$1,$32,$33,$34,"ratio_ref",$35}
 END{print "random_configs",n,"naive_pass",a,"ref_pass",b}' $S
echo "## C. ratio_ref range (random FB): min,max"
awk -F'\t' 'NR>1 && $6=="random" {if(min==""||$35<min)min=$35; if($35>max)max=$35} END{print min,max}' $S
echo "## D. H2 point estimate > 5/q (resolvable configs), by nT,m,bits"
awk -F'\t' 'NR>1 && $43==1 {k="m="$5"\tnT="$12"\tbits="$2; r[k]++; if($44=="FAIL")f[k]++; if($45==1)g[k]++} END{for(k in r)printf "%s\tresolvable=%d\tpoint_gt_5/q=%d\tF4=%d\n",k,r[k],f[k]+0,g[k]+0}' $S | sort
echo "## E. gcd_odd>1 strata: configs and nondegenerate mixed counts"
awk -F'\t' 'NR>1 {k="gcdodd>1="($11>1)"\tm="$5"\tnT="$12; c[k]++; M[k]+=$23; Md[k]+=$24; S[k]+=$25} END{for(k in c)printf "%s\tconfigs=%d\tM_nd=%d\tM_deg=%d\tS_nd=%d\n",k,c[k],M[k],Md[k],S[k]}' $S | sort
echo "## F. curveflag counts"
awk -F'\t' 'NR>1{c[$13]++} END{for(k in c)print k,c[k]}' $S
echo "## G. targets_xR_in_V total, truncated configs"
awk -F'\t' 'NR>1{a+=$28; t+=$20} END{print "targets_xR_in_V_total",a,"truncated_configs",t}' $S
