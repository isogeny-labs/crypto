#!/usr/bin/env bash
# Tables and guess.svg from f4cost_guess.jsonl (jq + awk only).
set -eu
D=$(cd "$(dirname "$0")" && pwd); J=$D/f4cost_guess.jsonl
jq -r '
 def lam: if .form=="bits" then ({"6":0.009527,"7":0.049892,"8":0.179462}[.t|tostring])
          else ({"6":0.009527,"7":0.062651,"8":0.246092}[.t|tostring]) end;
 def f(x): x|if .==null then "n/a" else (.*1000|round/1000|tostring) end;
 def sci: if .==null then "n/a" else (if .==0 then "0" else (log10|floor) as $e | "\((./pow(10;$e))*100|round/100)e\($e)" end) end;
 [.form,(.t|tostring),(.guess|tostring),"\(.targets_done)/\(.targets_requested)",(if .capped then "CAP" else "-" end),
  (.muladds_per_target|sci), (.calls_per_target|tostring), ((.f4_muladds/.f4_calls)|sci),
  ((pow(2;.guess)*.f4_muladds/.f4_calls)|sci), (.solving_degree_max|tostring), (.undecided|tostring),
  "\(.hits)/\(.hits_verified)", ((.muladds_per_target/lam)|sci), (.coverage_exact|tostring|.[0:8]),
  (if .coverage_exact>0 then ((.muladds_per_target/.coverage_exact)|sci) else "n/a" end)] | "| "+join(" | ")+" |"
' "$J" > "$D/table_rows.md"
# argmin per (form,t) among uncapped cells and among all cells
jq -r '[.form,.t,.guess,.capped,.muladds_per_target]|@tsv' "$J" | sort -k1,1 -k2,2n -k3,3n |
awk -F'\t' '{k=$1" t="$2; if(!(k in b)||$5+0<b[k]+0){b[k]=$5;g[k]=$3;c[k]=$4}} END{for(k in b)print k" argmin_G="g[k]" muladds/target="b[k]" capped="c[k]}' | sort > "$D/argmin.txt"
# SVG, log y
jq -r '[.form,.t,.guess,.muladds_per_target,.capped]|@tsv' "$J" | sort -k1,1 -k2,2n -k3,3n | awk -F'\t' '
BEGIN{W=760;H=460;L=70;R=20;T=30;B=50;lo=1e5;hi=1e12}
{n++;f[n]=$1;t[n]=$2;g[n]=$3;v[n]=$4;cp[n]=$5}
function X(g){return L+(g)*(W-L-R)/7}
function Y(v){return T+(H-T-B)*(1-(log(v)/log(10)-log(lo)/log(10))/(log(hi)/log(10)-log(lo)/log(10)))}
END{
 printf "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 %d %d\" font-family=\"sans-serif\" font-size=\"11\">\n",W,H
 printf "<rect width=\"%d\" height=\"%d\" fill=\"white\"/>\n",W,H
 for(e=5;e<=12;e++){y=Y(10^e);printf "<line x1=\"%d\" x2=\"%d\" y1=\"%.1f\" y2=\"%.1f\" stroke=\"#ddd\"/><text x=\"%d\" y=\"%.1f\" text-anchor=\"end\">1e%d</text>\n",L,W-R,y,y,L-5,y+4,e}
 for(gg=0;gg<=7;gg++)printf "<text x=\"%.1f\" y=\"%d\" text-anchor=\"middle\">%d</text>\n",X(gg),H-B+16,gg
 printf "<text x=\"%d\" y=\"%d\" text-anchor=\"middle\">guessed levels/bits G</text>\n",(L+W-R)/2,H-10
 printf "<text x=\"14\" y=\"%d\" transform=\"rotate(-90 14 %d)\" text-anchor=\"middle\">F4 multiply-adds per target (log)</text>\n",H/2,H/2
 printf "<text x=\"%d\" y=\"18\" font-size=\"13\">Guessing depth, k=20, m=2 (solid bits, dashed isogeny; open marker = capped sample)</text>\n",L
 split("#1f77b4 #ff7f0e #2ca02c",col," ")
 for(i=1;i<=n;i++){key=f[i]" "t[i]; if(!(key in seen)){seen[key]=1;keys[++nk]=key}; pts[key]=pts[key]sprintf("%.1f,%.1f ",X(g[i]),Y(v[i]))}
 for(j=1;j<=nk;j++){key=keys[j];split(key,a," ");c=col[a[2]-5];d=(a[1]=="isogeny")?" stroke-dasharray=\"5,3\"":""
  printf "<polyline points=\"%s\" fill=\"none\" stroke=\"%s\" stroke-width=\"2\"%s/>\n",pts[key],c,d
  ly=T+8+(j-1)*14; printf "<line x1=\"%d\" x2=\"%d\" y1=\"%d\" y2=\"%d\" stroke=\"%s\" stroke-width=\"2\"%s/><text x=\"%d\" y=\"%d\">%s t=%s</text>\n",W-170,W-145,ly,ly,c,d,W-140,ly+4,a[1],a[2]}
 for(i=1;i<=n;i++){c=col[t[i]-5];fl=(cp[i]=="true")?"white":c;printf "<circle cx=\"%.1f\" cy=\"%.1f\" r=\"3.5\" fill=\"%s\" stroke=\"%s\"/>\n",X(g[i]),Y(v[i]),fl,c}
 print "</svg>"}' > "$D/guess.svg"
