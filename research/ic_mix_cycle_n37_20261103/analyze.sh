#!/bin/sh
# Frozen analysis (PROTOCOL.md). awk over runs.txt (qi seed trials adds wall rels_m2src rels_m3src closed_by rank ver rhover bad incons).
# Bootstrap: 10,000 resamples over seeds with an explicit Park-Miller generator seeded 20261101. Writes ratios.csv.
TMP=$(mktemp -d)
QV="0 0.000083 0.00025 0.00075 0.0022 0.008 0.025 1"
BOOT='function nxt(){S=(S*16807)%2147483647; return S/2147483647}
 {lr[NR]=log($2/$1); g+=lr[NR]; n=NR}
 END{S=20261101; for(i=0;i<50;i++)nxt(); for(b=1;b<=10000;b++){s=0;for(i=1;i<=n;i++){s+=lr[int(nxt()*n)+1]} print exp(s/n) > F} close(F);
  cmd="sort -g " F " | sed -n \"250p;9750p\" | tr \"\\n\" \" \""; cmd | getline r; close(cmd); split(r,a," ");
  printf "%.3f,%.3f,%.3f", exp(g/n), a[1], a[2]}'
echo "# frozen analysis output (Park-Miller seed 20261101, 10000 resamples)"
echo "## per q: finished, mean trials (range), mean relations by source, share closed by source (A = m=2, B = 2+1), mean rank"
i=0
for qv in $QV; do
  awk -v qi=$i -v qv=$qv '$1==qi{n++; t+=$3; a+=$6; b+=$7; if($8==2)ca++; else cb++; rk+=$9; if(n==1||$3<mn)mn=$3; if($3>mx)mx=$3}
    END{printf "q=%-7s (idx %d): finished=%d/6 mean_trials=%.0f (%d..%d) rels_A=%.0f rels_B=%.0f closed_by_A=%d/6 closed_by_B=%d/6 mean_rank=%.0f\n", qv,qi,n,t/n,mn,mx,a/n,b/n,ca+0,cb+0,rk/n}' runs.txt
  i=$((i+1))
done
echo "## per-seed trials ratio to q=0, paired geometric mean vs q=0 with 95% bootstrap (csv)"
echo "q_idx,q,finished,mean_trials,geomean_vs_q0,ci_lo,ci_hi" > ratios.csv
awk '$1==0{print $2, $3}' runs.txt | sort > $TMP/base.txt
i=0
for qv in $QV; do
  awk -v qi=$i 'NR==FNR{b[$1]=$2; next} $1==qi && ($2 in b){print b[$2], $3}' $TMP/base.txt runs.txt > $TMP/pr_$i.txt
  mt=$(awk -v qi=$i '$1==qi{n++;t+=$3} END{printf "%.0f", t/n}' runs.txt)
  fin=$(wc -l < $TMP/pr_$i.txt)
  g=$(awk -v F=$TMP/b_$i "$BOOT" $TMP/pr_$i.txt)
  echo "$i,$qv,$fin,$mt,$g" >> ratios.csv
  i=$((i+1))
done
cat ratios.csv
echo "## per-seed ratios vs q=0 (trials)"
for i in 1 2 3 4 5 6 7; do awk -v qi=$i 'NR==FNR{b[$1]=$2; next} $1==qi{printf "q_idx=%d seed=%s ratio=%.2f\n", qi,$2,$3/b[$2]}' $TMP/base.txt runs.txt; done | awk '{split($3,a,"="); k=$1; if(!(k in mn)||a[2]<mn[k])mn[k]=a[2]; if(a[2]>mx[k])mx[k]=a[2]} END{for(k in mn) printf "%s per-seed ratio range %.2f..%.2f\n", k, mn[k], mx[k]}' | sort
rm -rf $TMP
