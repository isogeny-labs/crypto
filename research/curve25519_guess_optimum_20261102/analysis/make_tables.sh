#!/usr/bin/env bash
# table.md, argmin.md, guess.svg from cells.jsonl + support.jsonl (jq + awk only).
set -eu
D=$(cd "$(dirname "$0")" && pwd); J=$D/cells.jsonl; S=$D/support.jsonl
sci='def sci: if .==null then "n/a" elif .==0 then "0" else (log10|floor) as $e | "\((./pow(10;$e))*100|round/100)e\($e)" end;'
jq -r --slurpfile sup "$S" "$sci"'
 def lam($k;$t): [$sup[]|select(.k==$k and .t==$t)|.lambda][0];
 [.stage,(.k|tostring),(.t|tostring),(.guess|tostring),"\(.targets_done)/\(.targets_requested)",(if .capped then "CAP" else "-" end),
  (.muladds_per_target|sci),(.calls_per_target*100|round/100|tostring),(.solving_degree_max|tostring),(.undecided|tostring),
  "\(.hits)/\(.hits_verified)",(lam(.k;.t)|.*1e5|round/1e5|tostring),((.muladds_per_target/lam(.k;.t))|sci),
  (.coverage_exact|.*1e5|round/1e5|tostring),((.muladds_per_target/.coverage_exact)|sci),
  (if .hits>0 then (.muladds_per_hit|sci) else "-" end),
  (((.native.pkm_system_fp_mults_uncharged//0)+(.native.pkm_grid_fp_mults_uncharged//0))/.f4_muladds*100|.*1000|round/1000|tostring)+"%",
  (.load_at_start|split(",")[0])] | "| "+join(" | ")+" |"' "$J" > "$D/table_rows.md"
# argmin per (stage,k,t) over non-repro cells, plus bracketing
jq -r 'select(.stage!="repro")|[.stage,.k,.t,.guess,.muladds_per_target,.capped]|@tsv' "$J" | sort -k1,1 -k2,2n -k3,3n -k4,4n | awk -F'\t' '
function flush(){ if(n==0)return; best=1; for(i=2;i<=n;i++) if(v[i]<v[best]) best=i;
 br=(best>1&&best<n)?"bracketed":"edge"; if(best==n&&n>1)br="edge (largest G measured)"; if(best==1)br="edge (smallest G measured)";
 anycap=0; for(i=1;i<=n;i++) if(c[i]=="true")anycap=1;
 printf "| %s | %s | %s | %s..%s | %s | %.4g | %s | %s |\n", st,kk,tt,g[1],g[n],g[best],v[best],br,(anycap?"capped cells":"no cap"); n=0 }
{ key=$1" "$2" "$3; if(key!=prev){flush(); prev=key; st=$1;kk=$2;tt=$3} n++; g[n]=$4; v[n]=$5; c[n]=$6 } END{flush()}' > "$D/argmin_rows.md"
# monotone steps: ratio cost(G)/cost(G+1) per consecutive pair
jq -r 'select(.stage!="repro")|[.stage,.k,.t,.guess,.muladds_per_target,.capped]|@tsv' "$J" | sort -k1,1 -k2,2n -k3,3n -k4,4n | awk -F'\t' '
{key=$1" k="$2" t="$3; if(key==pk){printf "| %s | G=%d to %d | %.2f | %s |\n",key,pg,$4,pv/$5,(pc=="true"||$6=="true")?"capped":"-"} pk=key;pg=$4;pv=$5;pc=$6}' > "$D/ratio_rows.md"
# SVG: stage 1 and stage 2, per-relation-free: muladds per target vs G/t... (G on x)
jq -r 'select(.stage!="repro")|[.stage,.k,.t,.guess,.muladds_per_target,.capped]|@tsv' "$J" | sort -k1,1 -k2,2n -k3,3n -k4,4n | awk -F'\t' '
BEGIN{W=820;H=480;L=70;R=210;T=34;B=52;lo=1e5;hi=2e11}
{n++;st[n]=$1;kk[n]=$2;tt[n]=$3;g[n]=$4;v[n]=$5;cp[n]=$6}
function X(x){return L+(x-3)*(W-L-R)/12}
function Y(v){return T+(H-T-B)*(1-(log(v)-log(lo))/(log(hi)-log(lo)))}
END{
 printf "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 %d %d\" font-family=\"sans-serif\" font-size=\"11\">\n<rect width=\"%d\" height=\"%d\" fill=\"white\"/>\n",W,H,W,H
 for(e=5;e<=11;e++){y=Y(10^e);printf "<line x1=\"%d\" x2=\"%d\" y1=\"%.1f\" y2=\"%.1f\" stroke=\"#ddd\"/><text x=\"%d\" y=\"%.1f\" text-anchor=\"end\">1e%d</text>\n",L,W-R,y,y,L-5,y+4,e}
 for(x=4;x<=14;x++)printf "<text x=\"%.1f\" y=\"%d\" text-anchor=\"middle\">%d</text>\n",X(x),H-B+16,x
 printf "<text x=\"%d\" y=\"%d\" text-anchor=\"middle\">guessed levels G</text>\n",(L+W-R)/2,H-12
 printf "<text x=\"14\" y=\"%d\" transform=\"rotate(-90 14 %d)\" text-anchor=\"middle\">F4 multiply-adds per target (log)</text>\n",H/2,H/2
 printf "<text x=\"%d\" y=\"20\" font-size=\"13\">Isogeny form guessing, m=2 (solid k=20 t=6..10; k=24 dashed, k=28 dotted; open = capped)</text>\n",L
 split("#1f77b4 #ff7f0e #2ca02c #d62728 #9467bd #8c564b #17becf",col," ")
 for(i=1;i<=n;i++){key=st[i]" "kk[i]" "tt[i]; if(!(key in seen)){seen[key]=1;keys[++nk]=key;ks[nk]=kk[i];ts[nk]=tt[i];sg[nk]=st[i]}; pts[key]=pts[key]sprintf("%.1f,%.1f ",X(g[i]),Y(v[i]))}
 for(j=1;j<=nk;j++){c=col[(j-1)%7+1]; d=(ks[j]==24)?" stroke-dasharray=\"6,3\"":((ks[j]==28)?" stroke-dasharray=\"2,3\"":"")
  printf "<polyline points=\"%s\" fill=\"none\" stroke=\"%s\" stroke-width=\"2\"%s/>\n",pts[keys[j]],c,d
  ly=T+8+(j-1)*14; printf "<line x1=\"%d\" x2=\"%d\" y1=\"%d\" y2=\"%d\" stroke=\"%s\" stroke-width=\"2\"%s/><text x=\"%d\" y=\"%d\">k=%s t=%s (%s)</text>\n",W-R+10,W-R+35,ly,ly,c,d,W-R+40,ly+4,ks[j],ts[j],sg[j]}
 for(i=1;i<=n;i++){key=st[i]" "kk[i]" "tt[i];for(j=1;j<=nk;j++)if(keys[j]==key)c=col[(j-1)%7+1];fl=(cp[i]=="true")?"white":c;printf "<circle cx=\"%.1f\" cy=\"%.1f\" r=\"3.5\" fill=\"%s\" stroke=\"%s\"/>\n",X(g[i]),Y(v[i]),fl,c}
 print "</svg>"}' > "$D/guess_optimum.svg"
