#!/bin/sh
# Frozen analysis (PROTOCOL.md). awk over runs_m3.txt (d seed trials adds wall rels rank K anchors ver rhover bad incons)
# and runs_m2_ref.txt (d seed trials adds wall rels K ver). Bootstrap: 10,000 resamples over pairs with an
# explicit Park-Miller generator seeded 20261101 (mawk srand() is not reproducible). Writes ratios.csv.
TMP=$(mktemp -d)
BOOT='function nxt(){S=(S*16807)%2147483647; return S/2147483647}
 {lr[NR]=log($2/$1); g+=lr[NR]; n=NR}
 END{S=20261101; for(i=0;i<50;i++)nxt(); for(b=1;b<=10000;b++){s=0;for(i=1;i<=n;i++){s+=lr[int(nxt()*n)+1]} print exp(s/n) > F} close(F);
  cmd="sort -g " F " | sed -n \"250p;9750p\" | tr \"\\n\" \" \""; cmd | getline r; close(cmd); split(r,a," ");
  printf "%.3f,%.3f,%.3f", exp(g/n), a[1], a[2]}'
echo "# frozen analysis output (Park-Miller seed 20261101, 10000 resamples)"
echo "## per d: m=3 (this protocol) and m=2 torsion-off reference (merged records), same seeds"
for d in 12 10 8; do
  awk -v d=$d '$1==d{n++; t+=$3; a+=$4; w+=$5; r+=$6; k+=$7/$8; if(n==1||$3<mn)mn=$3; if($3>mx)mx=$3; an+=$9}
    END{printf "d=%02d m=3: finished=%d/6 mean_trials=%.0f (%d..%d) mean_adds=%.0f mean_wall=%.1fs mean_rels=%.0f mean_rank/K=%.2f mean_anchors=%.0f\n", d,n,t/n,mn,mx,a/n,w/n,r/n,k/n,an/n}' runs_m3.txt
  awk -v d=$d '$1==d{n++; t+=$3; a+=$4; w+=$5; r+=$6; if(n==1||$3<mn)mn=$3; if($3>mx)mx=$3}
    END{printf "d=%02d m=2: finished=%d/6 mean_trials=%.0f (%d..%d) mean_adds=%.0f mean_wall=%.1fs mean_rels=%.0f\n", d,n,t/n,mn,mx,a/n,w/n,r/n}' runs_m2_ref.txt
done
echo "## per-seed ratios m=3 / m=2 (trials, group additions, wall)"
for d in 12 10 8; do
  awk -v d=$d 'NR==FNR{ if($1==d){t2[$2]=$3;a2[$2]=$4;w2[$2]=$5} next} $1==d && ($2 in t2){printf "%s %d %d %d %d %d %.3f %.3f %.3f\n", $2,$3,t2[$2],$4,a2[$2],0,$3/t2[$2],$4/a2[$2],$5/w2[$2]}' runs_m2_ref.txt runs_m3.txt | sort > $TMP/pr_$d.txt
  awk -v d=$d '{printf "d=%s seed=%s trials m3=%d m2=%d ratio=%.2f | adds ratio=%.2f | wall ratio=%.2f\n", d,$1,$2,$3,$7,$8,$9}' $TMP/pr_$d.txt
done
echo "## per d: pairs, geometric mean m3/m2 with 95% bootstrap (csv)"
echo "d,K,pairs,trials_geomean,trials_lo,trials_hi,adds_geomean,adds_lo,adds_hi,wall_geomean,wall_lo,wall_hi" > ratios.csv
for d in 12 10 8; do
  K=$(awk -v d=$d '$1==d{print $8; exit}' runs_m3.txt); n=$(wc -l < $TMP/pr_$d.txt)
  tr=$(awk '{print $3, $2}' $TMP/pr_$d.txt | awk -v F=$TMP/bt_$d "$BOOT")
  ad=$(awk '{print $5, $4}' $TMP/pr_$d.txt | awk -v F=$TMP/ba_$d "$BOOT")
  wl=$(awk '{printf "1 %.6f\n", $9}' $TMP/pr_$d.txt | awk -v F=$TMP/bw_$d "$BOOT")
  echo "$d,$K,$n,$tr,$ad,$wl" >> ratios.csv
done
cat ratios.csv
rm -rf $TMP
