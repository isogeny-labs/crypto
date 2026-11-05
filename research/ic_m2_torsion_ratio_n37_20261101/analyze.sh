#!/bin/sh
# Frozen analysis (PROTOCOL.md). awk over runs.txt (d torsion seed trials wall rels K largest ver rhover),
# plus the merged d = 12 reference (../ic_m2_torsion_n37_20261101/pairs.txt). h = 596.
# Bootstrap: 10,000 resamples over pairs with an explicit Park-Miller generator seeded 20261101
# (mawk's srand() is not reproducible across runs, so it is not used). Writes ratios.csv.
H=596; TMP=$(mktemp -d)
BOOT='function nxt(){S=(S*16807)%2147483647; return S/2147483647}
 {lr[NR]=log($3/$2); g+=lr[NR]; n=NR}
 END{S=20261101; for(i=0;i<50;i++)nxt(); f=F; for(b=1;b<=10000;b++){s=0;for(i=1;i<=n;i++){s+=lr[int(nxt()*n)+1]} print exp(s/n) > f} close(f);
  cmd="sort -g " f " | sed -n \"250p;9750p\" | tr \"\\n\" \" \""; cmd | getline r; close(cmd); split(r,a," ");
  printf "%d,%s,%.2f,%d,%.3f,%.3f,%.3f\n", D, K, H/K, n, exp(g/n), a[1], a[2]}'
echo "# frozen analysis output (bootstrap: Park-Miller seed 20261101, 10000 resamples, h=$H)"
echo "## per (d, arm)"
awk -v H=$H '{k=$1" "$2; n[k]++; tr[k]+=$4; w[k]+=$5; r[k]+=$6; l[k]+=$8; K[$1]=$7; if(!(k in mn)||$4<mn[k])mn[k]=$4; if($4>mx[k])mx[k]=$4}
 END{for(k in n){split(k,a," "); printf "d=%02d torsion=%s K=%s h/K=%.2f finished=%d/6 mean_trials=%.0f (%d..%d) mean_rels=%.0f mean_largest=%.1f mean_wall=%.1fs\n", a[1],(a[2]==1?"on":"off"),K[a[1]],H/K[a[1]],n[k],tr[k]/n[k],mn[k],mx[k],r[k]/n[k],l[k]/n[k],w[k]/n[k]}}' runs.txt | sort -r
echo "## paired ON/OFF trials per seed"
for d in 10 8 7; do
  awk -v d=$d '$1==d&&$2==0{off[$3]=$4} $1==d&&$2==1{on[$3]=$4} END{for(s in off) if(s in on) printf "%s %d %d\n", s, off[s], on[s]}' runs.txt | sort > $TMP/pr_$d.txt
  awk -v d=$d '{printf "d=%s seed=%s off=%d on=%d on/off=%.2f\n", d,$1,$2,$3,$3/$2}' $TMP/pr_$d.txt
done
echo "## per d: pairs, geometric mean ON/OFF, 95% bootstrap interval (csv: d,K,h_over_K,pairs,geomean,lo,hi)"
echo "d,K,h_over_K,pairs,geomean_on_over_off,ci_lo,ci_hi" > ratios.csv
awk '{print $1, $2, $7}' ../ic_m2_torsion_n37_20261101/pairs.txt > $TMP/pr_12.txt
awk -v D=12 -v K=2011 -v H=$H -v F=$TMP/b12 "$BOOT" $TMP/pr_12.txt >> ratios.csv
for d in 10 8 7; do K=$(awk -v d=$d '$1==d{print $7; exit}' runs.txt); awk -v D=$d -v K=$K -v H=$H -v F=$TMP/b$d "$BOOT" $TMP/pr_$d.txt >> ratios.csv; done
cat ratios.csv
rm -rf $TMP
