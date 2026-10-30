#!/usr/bin/env bash
# post-hoc extraction of min/max of recorded point estimates from out/results.csv (no new computation beyond min/max)
cd "$(dirname "$0")/.."
for arm in "PF(j=m;k=m)" "PF(j=m;k=1)" "PF best (sweep)" "X coupled E-sets" "X coupled E3 gen" "pair(m;m) accounting" "PF(m;m)/(Z_D(m2)/Z_D(m))" "Z_D(m2)/Z_D(m)"; do
  for fam in A B; do
    awk -F, -v arm="$arm" -v fam="$fam" '$1==fam && $4=="primary" && $11==arm && $8==1 && $12!="NA" && $12!="inf" {v=$12+0; if(n==0||v<mn){mn=v;mnc=$3"/N"$5} if(n==0||v>mx){mx=v;mxc=$3"/N"$5} n++} END{printf "%s fam=%s primary pooled powered cells=%d min=%s (p/N %s) max=%s (p/N %s)\n", arm, fam, n, mn, mnc, mx, mxc}' out/results.csv
  done
done
echo "--- upper end of 99% curve-level interval, minimum over powered primary pooled cells, arms 1-4 (anything <1 would falsify H5)"
for arm in "PF(j=m;k=m)" "PF(j=m;k=1)" "PF best (sweep)" "X coupled E-sets"; do
  awk -F, -v arm="$arm" '$4=="primary" && $11==arm && $8==1 && $14!="NA" && $14!="inf" {v=$14+0; if(n==0||v<mn){mn=v;c=$1"/"$3"/N"$5} n++} END{printf "%s min hi99=%s at %s over %d cells\n", arm, mn, c, n}' out/results.csv
  awk -F, -v arm="$arm" '$4=="primary" && $11==arm && $8==1 && $13!="NA" {v=$13+0; if(n==0||v<mn){mn=v;c=$1"/"$3"/N"$5} n++} END{printf "%s min lo99=%s at %s over %d cells\n", arm, mn, c, n}' out/results.csv
done
echo "--- pair(m;m) accounting: number of powered primary pooled cells with point<1"
awk -F, '$4=="primary" && $11=="pair(m;m) accounting" && $8==1 {n++; if($12+0<1) k++} END{print k" of "n}' out/results.csv
echo "--- load average per timing repeat (semaev_runs.csv): min/max load1"
awk -F, 'NR>1{v=$8+0; if(n==0||v<mn)mn=v; if(n==0||v>mx)mx=v; n++} END{print "load1 min="mn" max="mx" over "n" timed blocks"}' out/semaev_runs.csv
