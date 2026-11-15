#!/bin/bash
# Tables (markdown) and checks from rows.tsv and lead D's k=20 rows. awk only.
cd "$(dirname "$0")"
./make_rows.sh
LD=../../curve25519_cofactor_shared_20261102/analysis/f4cost_shared.jsonl
# k=20 reference rows: lead D's full (non-prefix, non-trace) rows, same column layout, prefixed ref20_
jq -r 'select((.failed|not) and (.run|test("prefix|trace")|not)) | [
  ("ref20_"+.run), .k, .form, .t, .guess, .lifts_mode, "full",
  .targets_done, .capped, .hits, .hits_verified, .hits_exact,
  .calls_per_target, .muladds_per_target, .solving_degree_max, .solving_degree_mean,
  .solver_extra.max_rows_max, .solver_extra.max_cols_max, .solver_extra.max_nnz_max, .undecided,
  .columns, .covered_points, .covered_points_all_cosets, (.covered_projected_points // 0),
  (.covered_projected_points / .coverage_projected | round),
  (.setup_native.pkm_fiber_size // 0), (.setup_native.pkm_fiber_complete // 0),
  (.uptime | capture("load average: (?<a>[0-9.]+)").a), .f4_calls, .f4_muladds, .wall_s
] | @tsv' $LD > ref20.tsv
cat rows.tsv ref20.tsv > allrows.tsv
# base-signature check (all runs of a cell: same columns/covered/allco/proj) -> base_check.txt
awk -F'\t' '$1 !~ /^ref20_/ {key=$2" "$3" t="$4" g="$5; sig=$21" "$22" "$23" "$24; if(!((key,sig) in s)){s[key,sig]=1; n[key]++; last[key]=sig}}
  END{for(k in n) printf "k=%s columns/covered/allcosets/projected = %s  distinct-base-signatures=%d\n", k, last[k], n[k]}' rows.tsv | sort > base_check.txt
if grep -v 'distinct-base-signatures=1$' base_check.txt | grep -q .; then echo "BASE SIGNATURE MISMATCH" >&2; fi

fmt='function f(x){ if (x=="" ) return "n/a"; if (x+0>=1e5 || (x+0<1e-2 && x+0>0)) return sprintf("%.3g",x); if (x==int(x)) return sprintf("%d",x); return sprintf("%.4g",x) }'

# Table A: all runs
{
echo "| k | form | t | G | lifts | run | targets | capped | hits | verified | exact | calls/tgt | mul-adds/tgt | max deg | mean deg | max rows | max cols | max nnz | undec. | fiber | mul-adds/hit | mul-adds/tgt / (cov/r) | upper bd. (all cosets) | load |"
echo "|--:|:--|:-:|:-:|:-:|:--|--:|:-:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|--:|--:|--:|--:|"
sort -t$'\t' -k2,2n -k3,3 -k4,4n -k6,6n -k7,7 rows.tsv | awk -F'\t' "$fmt"'
 { cov=($6==0)?$22:$24; r=$25; pr=(cov>0)? $14/(cov/r):""; ub=($23>0)?$14/($23/r):"";
   mph=($10>0)? $30/$10 : "";
   printf "| %d | %s | %d | %d | %d | %s | %d | %s | %d | %d | %d | %s | %s | %d | %.2f | %d | %d | %d | %d | %d%s | %s | %s | %s | %s |\n",
     $2,$3,$4,$5,$6,$7,$8,($9=="true"?"yes":"no"),$10,$11,$12,f($13),f($14),$15,$16,$17,$18,$19,$20,$26,($27==1?"c":"i"),(mph==""?"undef":f(mph)),f(pr),f(ub),$28 }'
} > table_all.md

# Table B: matched comparison per cell (prefix rows where they exist)
awk -F'\t' "$fmt"'
 $1 !~ /^ref20_/ { cell=$2"|"$3"|"$4"|"$5; L=$6; if($7=="full"){hasfull[cell,L]=1; for(j=1;j<=31;j++) F[cell,L,"f",j]=$j; nfull[cell,L]=$8}
                   else { hasp[cell,L]=1; for(j=1;j<=31;j++) F[cell,L,"p",j]=$j; npre[cell,L]=$8 } cells[cell]=1 }
 END{
  for(c in cells){ nmin=1e9;
    for(L=0;L<=2;L++) if(hasfull[c,L] && nfull[c,L]+0<nmin) nmin=nfull[c,L]+0;
    for(L=0;L<=2;L++){ T[L]=""; if(hasp[c,L] && npre[c,L]+0==nmin) T[L]="p"; else if(hasfull[c,L] && nfull[c,L]+0==nmin) T[L]="f" }
    split(c,cc,"|");
    h1=(T[0]!="")&&(T[1]!=""); h2=(T[1]!="")&&(T[2]!=""); h02=(T[0]!="")&&(T[2]!="");
    s=sprintf("%05d %s %d|| | %d | %s t=%d G=%d | %d |", cc[1], cc[2], cc[3], cc[1],cc[2],cc[3],cc[4], (nmin<1e9?nmin:0));
    if(h1){ c1=F[c,1,T[1],13]/F[c,0,T[0],13]; m1=F[c,1,T[1],14]/F[c,0,T[0],14] }
    s=s sprintf(" %s | %s |", (h1? f(c1):"n/a"), (h1? f(m1):"n/a"));
    cov0=""; if(T[0]!="") cov0=F[c,0,T[0],22]; else if(T[1]!="") cov0=F[c,1,T[1],22];
    proj=(T[1]!="")? F[c,1,T[1],24] : 0;
    yg=(cov0>0 && proj>0)? proj/cov0 : ""; ygs=(cov0==0 && proj>0)? "cov=0" : "n/a";
    s=s sprintf(" %s |", (yg==""?ygs:f(yg)));
    s=s sprintf(" %s |", ((h1&&yg!="")? f(m1/yg):ygs));
    s=s sprintf(" %s | %s |", (h2? f(F[c,2,T[2],14]/F[c,1,T[1],14]):"n/a"), (h02? f(F[c,2,T[2],14]/F[c,0,T[0],14]):"n/a"));
    d0=(T[0]!="")?F[c,0,T[0],15]:"-"; d1=(T[1]!="")?F[c,1,T[1],15]:"-"; d2=(T[2]!="")?F[c,2,T[2],15]:"-";
    s=s sprintf(" %s/%s/%s |", d0,d1,d2);
    if(h2) s=s sprintf(" %s/%s/%s |", f(F[c,2,T[2],17]/F[c,1,T[1],17]), f(F[c,2,T[2],18]/F[c,1,T[1],18]), f(F[c,2,T[2],19]/F[c,1,T[1],19])); else s=s " n/a |";
    for(L=0;L<=2;L++) hh[L]=(T[L]!="")?F[c,L,T[L],10]:"-";
    s=s sprintf(" %s/%s/%s |", hh[0],hh[1],hh[2]);
    for(L=0;L<=2;L++) hv[L]=(T[L]!="")?F[c,L,T[L],11]"-"F[c,L,T[L],12]:"-";
    s=s sprintf(" %s/%s/%s |", hv[0],hv[1],hv[2]);
    if(T[1]!="") { r=F[c,1,T[1],25]; s=s sprintf(" %.3g |", nmin*F[c,1,T[1],24]/r) } else s=s " n/a |";
    print s }
 }' rows.tsv | sort | sed 's/^[^|]*|| //' > table_matched.body
{
echo "| k | cell | matched targets | calls 1/0 | mul-adds 1/0 | yield gain proj/cov (expected) | per-relation 1/0 | mul-adds 2/1 | mul-adds 2/0 | max deg 0/1/2 | rows/cols/nnz 2/1 | hits 0/1/2 | verified-exact 0/1/2 | expected lifts=1 hits at coverage |"
echo "|--:|:--|--:|--:|--:|--:|--:|--:|--:|:-:|:-:|:-:|:-:|--:|"
cat table_matched.body
} > table_matched.md; rm -f table_matched.body

# Table C: k-scaling of mul-adds per target (full rows only; k=20 = lead D's rows, isogeny t=4 reproduced here)
awk -F'\t' "$fmt"'
 $7=="full" && $1 !~ /^ref20_.*_t4_/ || ($1 ~ /^k20_/) { key=$3" t="$4" G="$5" | "$6; k=$2; m[key,k]=$14; nt[key,k]=$8; d[key,k]=$15; keys[key]=1 }
 END{ for(q in keys){
   s=sprintf("%s %s |", q, q); sub(/^[^|]*\| [^ ]* /, "", s);
   o=sprintf("%s |", q);
   for(k=20;k<=28;k+=4){ if((q,k) in m) o=o sprintf(" %s (%d) |", f(m[q,k]), nt[q,k]); else o=o " n/a |" }
   o=o sprintf(" %s | %s |", (((q,24) in m)&&((q,20) in m))? f(m[q,24]/m[q,20]):"n/a", (((q,28) in m)&&((q,24) in m))? f(m[q,28]/m[q,24]):"n/a");
   o=o sprintf(" %s/%s/%s |", (((q,20) in m)?d[q,20]:"-"), (((q,24) in m)?d[q,24]:"-"), (((q,28) in m)?d[q,28]:"-"));
   print "| " o } }' allrows.tsv | sort > table_scaling.body
{
echo "| cell | lifts | mul-adds/tgt k=20 (targets) | k=24 | k=28 | 24/20 | 28/24 | max deg 20/24/28 |"
echo "|:--|:-:|--:|--:|--:|--:|--:|:-:|"
cat table_scaling.body
} > table_scaling.md; rm -f table_scaling.body
echo done

# Table D: per-relation cost at exact coverage across k (lifts 0 and 1), full rows
awk -F'\t' '
 $7=="full" && $6<2 && (($1 ~ /^ref20_/ && $1 !~ /_t4_/) || $1 ~ /^k(20|24|28)_/) { cov=($6==0)?$22:$24; q=$3" t="$4" G="$5" | "$6; k=$2; keys[q]=1; cv[q,k]=cov/$25; pr[q,k]=(cov>0)? $14/(cov/$25) : -1 }
 function g(q,k){ if(!((q,k) in pr)) return "n/a"; if(pr[q,k]<0) return "undef (cov 0)"; return sprintf("%.3g", pr[q,k]) }
 function ratio(q,a,b){ if(!((q,a) in pr)||!((q,b) in pr)||pr[q,a]<0||pr[q,b]<0) return "n/a"; return sprintf("%.3g", pr[q,b]/pr[q,a]) }
 END{ for(q in keys) printf "| %s | %s | %s | %s | %s | %s |\n", q, g(q,20), g(q,24), g(q,28), ratio(q,20,24), ratio(q,24,28) }' allrows.tsv | sort > table_perrel.body
{
echo "| cell | lifts | mul-adds/tgt / (cov/r), k=20 | k=24 | k=28 | 24/20 | 28/24 |"
echo "|:--|:-:|--:|--:|--:|--:|--:|"
cat table_perrel.body
} > table_perrel.md; rm -f table_perrel.body
