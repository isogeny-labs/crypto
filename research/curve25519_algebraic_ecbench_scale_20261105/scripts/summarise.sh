#!/bin/bash
# summarise.sh : analysis/perchunk.tsv -> analysis/summary.tsv (per k, arm: verified/chunks, mean S at w=0,0.1,1 over verified chunks,
# paired proj-a and rho-neg means on the same chunks, ratios, and all-chunk generic means). awk only.
cd "$(dirname "$0")/.."
awk -F'\t' '
{ k=$1; c=$2; a=$3; v=$4; n=$5; key=k SUBSEP c SUBSEP a
  runs[k,a]++; if (v+0>0) { fin[k,a]++; ok[key]=1; for(j=0;j<3;j++) s[key,j]=$(7+j) }
  if (!((k,a) in seen)) { seen[k,a]=1; arms[k]=arms[k] " " a }
  chunks[k,c]=1 }
END {
 print "k\tarm\tverified/chunks\tS0\tS0.1\tS1\tprojA_paired(0,0.1,1)\trho_paired(0,0.1,1)\tx_projA(0,0.1,1)\tx_rho(0,0.1,1)"
 nk=split("24 28 32",KS," ")
 for (i=1;i<=nk;i++) { k=KS[i]; na=split(arms[k],AR," ")
  for (q=1;q<=na;q++) { a=AR[q]; m=0; for (j=0;j<3;j++){sa[j]=0;sp[j]=0;sr[j]=0}
   for (c=0;c<6;c++) { key=k SUBSEP c SUBSEP a; if (!(key in ok)) continue; m++
     for (j=0;j<3;j++){ sa[j]+=s[key,j]; sp[j]+=s[k SUBSEP c SUBSEP "proj-a",j]; sr[j]+=s[k SUBSEP c SUBSEP "rho-neg",j] } }
   if (m==0) { printf "%s\t%s\t0/%d\t-\t-\t-\t-\t-\t-\t-\n",k,a,runs[k,a]; continue }
   printf "%s\t%s\t%d/%d",k,a,m,runs[k,a]
   for (j=0;j<3;j++) printf "\t%.6g",sa[j]/m
   printf "\t%.5g/%.5g/%.5g\t%.5g/%.5g/%.5g",sp[0]/m,sp[1]/m,sp[2]/m,sr[0]/m,sr[1]/m,sr[2]/m
   printf "\t%.4g/%.4g/%.4g\t%.4g/%.4g/%.4g\n",sa[0]/sp[0],sa[1]/sp[1],sa[2]/sp[2],sa[0]/sr[0],sa[1]/sr[1],sa[2]/sr[2] } }
}' analysis/perchunk.tsv > analysis/summary.tsv
cat analysis/summary.tsv
