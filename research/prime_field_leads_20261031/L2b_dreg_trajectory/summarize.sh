#!/bin/bash
# Per (p,arm): n_ok, n_total, median/min/max of maxdeg, median of maxrows, maxcols, sumrows, basis.
# Median of ok runs only; timeouts counted in n_total and listed in cells.csv.
D=$(cd "$(dirname "$0")" && pwd)
RES=${L2B_RESULTS:-$D/results}
echo "p,log2p,N,arm,n_total,n_ok,maxdeg_med,maxdeg_min,maxdeg_max,maxrows_med,maxcols_med,lastrows_med,lastcols_med,sumrows_med,basis_values" > $RES/summary.csv
for key in $(cut -d, -f1,4 $RES/cells.csv | sort -u | sort -t, -k1,1n -k2,2); do
 p=${key%,*}; arm=${key#*,}
 awk -F, -v p=$p -v arm=$arm '
  function med(a,n,  i,j,t){ for(i=1;i<=n;i++)for(j=i+1;j<=n;j++) if(a[j]<a[i]){t=a[i];a[i]=a[j];a[j]=t}; return (n%2)?a[(n+1)/2]:(a[n/2]+a[n/2+1])/2 }
  $1==p && $4==arm { nt++; N=$5; if($6=="ok"){ k++; md[k]=$7; mr[k]=$10; mc[k]=$11; lr[k]=$12; lc[k]=$13; sr[k]=$14; bs[$16]++ } }
  END { for(b in bs) bl=bl b ":" bs[b] ";";
   if(k>0) printf "%d,%.3f,%d,%s,%d,%d,%s,%s,%s,%s,%s,%s,%s,%s,%s\n", p, log(p)/log(2), N, arm, nt, k, med(md,k), mind(md,k), maxd(md,k), med(mr,k), med(mc,k), med(lr,k), med(lc,k), med(sr,k), bl;
   else printf "%d,%.3f,%d,%s,%d,0,NA,NA,NA,NA,NA,NA,NA,NA,NA\n", p, log(p)/log(2), N, arm, nt }
  function mind(a,n,  i,m){m=a[1];for(i=2;i<=n;i++)if(a[i]<m)m=a[i];return m}
  function maxd(a,n,  i,m){m=a[1];for(i=2;i<=n;i++)if(a[i]>m)m=a[i];return m}
 ' $RES/cells.csv >> $RES/summary.csv
done
cat $RES/summary.csv
