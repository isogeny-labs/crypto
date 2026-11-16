#!/usr/bin/env bash
# cells.tsv -> table_rows.md, pair_rows.md, scale_rows.md, repro_rows.md (awk). Run make_tsv.sh first.
set -eu
D=$(cd "$(dirname "$0")" && pwd); T=$D/cells.tsv
sci() { :; }
# 1. full table: this-study cells only
awk -F'\t' 'function s(x){ if(x==0)return "0"; return sprintf("%.3g",x)}
NR>1 && $2=="this"{ pr=($17>0)?$10/$17:0; pra=($17>0)?$11/$17:0;
 printf "| %s | %s | %d | %d | %d | %d | %d/%d | %s | %s | %s | %.4g | %d | %d | %d/%d | %.4g | %s | %s | %.3f%% | %.3f%% | %s | %s |\n",
  $1,($1~/^main/)?"main":(($1~/^repB/)?"repB":"repC"),$3,$4,$5,$6,$7,$8,($9=="true")?"CAP":"-",s($10),s($11),$12,$13,$14,$15,$16,$17,s((($6==1)?pra:pr)),
  ($15>0)?s($10*$7/$15):"-",$18*100,$19*100,$22,$23 }' "$T" > "$D/table_rows.md"
# 2. trace pairs
awk -F'\t' 'NR>1 && $2=="this"{ key=substr($1,1,4)" "$3" "$4" "$5; if($6==0){m0[key]=$10;h0[key]=$15;v0[key]=$16;c0[key]=$9;u0[key]=$18}
 else {m1[key]=$10;a1[key]=$11;h1[key]=$15;v1[key]=$16;c1[key]=$9;rp[key]=$20;rf[key]=$21;u1[key]=$18;ord[++n]=key} }
 END{ for(i=1;i<=n;i++){k=ord[i]; split(k,a," "); cap=(c0[k]=="true"||c1[k]=="true")?"CAP":"-"; fb=rp[k]-rf[k];
  printf "| %s | %s | %s | %s | %s | %.3f | %.3f | %d / %d (%.2f%%) | %s (%d/%d) | %.3f%% | %.3f%% | %.2f |\n", a[1],a[2],a[3],a[4],cap,m1[k]/m0[k],a1[k]/m0[k],fb,rp[k],(rp[k]>0?100*fb/rp[k]:0),(h0[k]==h1[k]&&v0[k]==v1[k])?"equal":"DIFFER",h0[k],h1[k],u0[k]*100,u1[k]*100,(u0[k]>0)?u1[k]/u0[k]:0 } }' "$T" > "$D/pair_rows.md"
# 3. scaling: per (t,G): plain mul, cov, PR(cov) at k=20(lead B),24,28; slopes
awk -F'\t' 'function lg(x){return log(x)/log(2)}
function row(name,y1,y2,y3,   s1,s2,al,note){ if(y1<=0||y2<=0||y3<=0){printf "| %s | %.3g | %.3g | %.3g | - | - | - | zero or missing |\n",name,y1,y2,y3;return}
 s1=(lg(y2)-lg(y1))/4; s2=(lg(y3)-lg(y2))/4; d=s1-s2; if(d<0)d=-d;
 if(d<=0.5){al=sprintf("%.2f",(lg(y3)-lg(y1))/8);note="fit"} else {al="-";note="slopes differ, no single exponent"}
 printf "| %s | %.3g | %.3g | %.3g | %.3g | %.3g | %s (s1 %.2f, s2 %.2f) | %s |\n",name,y1,y2,y3,y2/y1,y3/y2,al,s1,s2,note }
NR>1{ if($2=="leadB"){m[$4" "$5" 20"]=$10;c[$4" "$5" 20"]=$17}
 else if($2=="this" && $1~/^main/ && $6==0){m[$4" "$5" "$3]=$10;c[$4" "$5" "$3]=$17;ok[$4" "$5]=1}
 if($2=="this"&&$1~/^main/&&$6==0&&$9=="true")capd[$4" "$5" "$3]=1 }
 END{ for(t=6;t<=10;t+=2){ for(i=0;i<2;i++){ G=(i==0)?4:t-1; kk=t" "G; if(!ok[kk])continue;
   row("mul/tgt t="t" G="G, m[kk" 20"],m[kk" 24"],m[kk" 28"]);
   } } }' "$T" > "$D/scale_mul.md"
awk -F'\t' 'function lg(x){return log(x)/log(2)}
function row(name,y1,y2,y3,   s1,s2,al,note){ if(y1<=0||y2<=0||y3<=0){printf "| %s | %.3g | %.3g | %.3g | - | - | - | zero or missing |\n",name,y1,y2,y3;return}
 s1=(lg(y2)-lg(y1))/4; s2=(lg(y3)-lg(y2))/4; d=s1-s2; if(d<0)d=-d;
 if(d<=0.5){al=sprintf("%.2f",(lg(y3)-lg(y1))/8);note="fit"} else {al="-";note="slopes differ, no single exponent"}
 printf "| %s | %.3g | %.3g | %.3g | %.3g | %.3g | %s (s1 %.2f, s2 %.2f) | %s |\n",name,y1,y2,y3,y2/y1,y3/y2,al,s1,s2,note }
NR>1{ if($2=="leadB"){m[$4" "$5" 20"]=$10;c[$4" "$5" 20"]=$17}
 else if($2=="this" && $1~/^main/ && $6==0){m[$4" "$5" "$3]=$10;c[$4" "$5" "$3]=$17;ok[$4" "$5]=1} }
 END{ for(t=6;t<=10;t+=2){ for(i=0;i<2;i++){ G=(i==0)?4:t-1; kk=t" "G; if(!ok[kk])continue;
   if(i==0){row("coverage_exact t="t, c[kk" 20"],c[kk" 24"],c[kk" 28"])}
   row("PR(cov)=mul/cov t="t" G="G, m[kk" 20"]/c[kk" 20"],m[kk" 24"]/c[kk" 24"],m[kk" 28"]/c[kk" 28"]);
   } } }' "$T" > "$D/scale_cov_pr.md"
# 4. trace ratio across k: raw trace1/trace0 within source
awk -F'\t' 'NR>1&&$2=="this"{ if($1~/^main/){s="m"}else if($1~/^repB/){s="B"}else{s="C"}
 key=s" "$3" "$4" "$5; if($6==0)p[key]=$10; else {r[key]=$10/p[key];a[key]=$11/p[key]} }
 END{ for(t=6;t<=10;t+=2) for(i=0;i<2;i++){G=(i==0)?4:t-1; r20="-";a20="-";
   for(s in sx){} 
   k20="-"; kB="B 20 "t" "G; kC="C 20 "t" "G; if(kB in r){r20=sprintf("%.3f",r[kB]);a20=sprintf("%.3f",a[kB])} else if(kC in r){r20=sprintf("%.3f",r[kC]);a20=sprintf("%.3f",a[kC])}
   k24="m 24 "t" "G; k28="m 28 "t" "G;
   printf "| t=%d G=%d | %s | %s | %s | %s | %s | %s |\n",t,G,r20,(k24 in r)?sprintf("%.3f",r[k24]):"-",(k28 in r)?sprintf("%.3f",r[k28]):"-",a20,(k24 in a)?sprintf("%.3f",a[k24]):"-",(k28 in a)?sprintf("%.3f",a[k28]):"-" } }' "$T" > "$D/scale_trace.md"
echo done
# 5. trace arm across k (two points k=24,28) and priced trace ratio at w = 0, 0.1, 1 (uncharged native work added at weight w to each arm)
awk -F'\t' 'NR>1&&$2=="this"{ s=substr($1,1,4); key=s" "$3" "$4" "$5; if($6==0){p[key]=$10;u0[key]=$18*$25;f0[key]=$25} else {q[key]=$10;qa[key]=$11;cv[key]=$17;u1[key]=$18*$25;f1[key]=$25;ord[++n]=key} }
 END{ for(i=1;i<=n;i++){k=ord[i]; split(k,a," "); w1=(f1[k]+u1[k])/(f0[k]+u0[k]); w01=(f1[k]+0.1*u1[k])/(f0[k]+0.1*u0[k]);
  printf "| %s | %s | %s | %s | %.3g | %.3g | %.3g | %.3g | %.3f | %.3f | %.3f |\n",a[1],a[2],a[3],a[4],q[k],qa[k],p[k]/cv[k],qa[k]/cv[k],f1[k]/f0[k],w01,w1 } }' "$T" > "$D/trace_rows.md"
# 6. full table sorted (seed in place of the id)
awk -F'|' 'BEGIN{OFS="|"}{id=$2; sub(/^ */,"",id); sub(/ *$/,"",id); n=split(id,a,"-s"); $2=" "a[n]" "; print}' "$D/table_rows.md" | sort -t'|' -k3,3 -k4,4n -k5,5n -k6,6n -k7,7n > "$D/table_sorted.md"
