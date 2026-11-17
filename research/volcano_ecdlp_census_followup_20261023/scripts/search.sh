#!/bin/sh
# usage: search.sh V5|V6 ; logs every seed to census/search_log.jsonl; writes census/<V>/root.txt and seed.txt on acceptance.
. "$(dirname "$0")/env.sh"
V=$1
case $V in
 V5) ELL=2; DEPTH=6; BASE=20261401; EXTRA="--j0";;
 V6) ELL=3; DEPTH=4; BASE=20261501; EXTRA="--crater split";;
esac
mkdir -p "$STUDY/census/$V"; LOG=$STUDY/census/search_log.jsonl
i=0
while [ $i -lt 400 ]; do
  S=$((BASE+i)); i=$((i+1))
  line=$($VR --bits 26 --ell $ELL --depth $DEPTH $EXTRA --seed $S --min-n 262144 --max-cofactor 64 --trials 1 --per-level 1 --controls 0 --out $SP/t/s_$V 2>&1 | grep '^curve:')
  [ -z "$line" ] && { echo "{\"V\":\"$V\",\"seed\":$S,\"accepted\":false,\"result\":\"no-curve\"}" >> $LOG; continue; }
  P=$(echo "$line" | sed 's/.*p=\([0-9]*\) .*/\1/'); A=$(echo "$line" | sed 's/.* a=\([0-9]*\) .*/\1/'); B=$(echo "$line" | sed 's/.* b=\([0-9]*\) .*/\1/')
  $CA volcano --p $P --a $A --b $B --ell $ELL --seed 20261023 --trials 1 --per-level 0 --controls 0 --radius 40 --max-vertices 20000 > $SP/t/w_$V.json 2>/dev/null || { echo "{\"V\":\"$V\",\"seed\":$S,\"p\":$P,\"accepted\":false,\"result\":\"ca-failed\"}" >> $LOG; continue; }
  R=$(jq -c '{p,a,b,order,n,cofactor,trace,depth,crater,vertices:.walk.vertices,per_level:.walk.per_level,special:.walk.special_j_skipped,verified:.walk.edges_verified}' $SP/t/w_$V.json)
  # reason: the first failing criterion
  why=$(jq -r --arg V $V --argjson ell $ELL --argjson P $P '
     (.walk.per_level|add) as $lv |
     if (.walk.vertices!=$lv) or (.walk.vertices>=20000) then "walk-incomplete"
     elif .cofactor>64 then "cofactor>64"
     elif .n<262144 then "n<2^18"
     elif (.n % $ell)==0 then "ell|n"
     elif $V=="V5" then
        (if .depth!=6 then "depth!=6"
         elif ($P%8)!=1 then "p!=1mod8"
         elif ((.n-1)%3)!=0 then "3-does-not-divide-n-1 (no rho.cubic)"
         elif .walk.per_level!=[1,1,2,4,8,16,32] then "per-level-not-1-1-2-4-8-16-32"
         elif .walk.special_j_skipped!=.walk.per_level[1] then "special-j-skipped-unexpected"
         else "ok" end)
     else
        (if .depth!=4 then "depth!=4"
         elif .crater!="split" then "crater-not-split"
         elif .walk.special_j_skipped!=0 then "special-j-skipped"
         elif .walk.vertices>2000 then "vertices>2000"
         else "ok" end)
     end' $SP/t/w_$V.json)
  ok=false; [ "$why" = ok ] && ok=true
  echo "{\"V\":\"$V\",\"seed\":$S,\"accepted\":$ok,\"reason\":\"$why\",\"walk\":$R}" >> $LOG
  if [ "$ok" = true ]; then echo "$line" > $STUDY/census/$V/root.txt; echo "$S" > $STUDY/census/$V/seed.txt; echo "$V accepted seed $S"; break; fi
done
