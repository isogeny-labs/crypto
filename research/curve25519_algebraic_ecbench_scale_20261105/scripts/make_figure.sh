#!/bin/sh
# make_figure.sh figure_data.tsv out.svg : mean S at w = 0.1 against k per arm (log y). awk only.
awk -F'\t' '
function lg(v){return log(v)/log(10)}
{ n++; a[n]=$1; k[n]=$2; v[n]=$3; f[n]=$4; if(!($1 in idx)){idx[$1]=++na; nm[na]=$1} }
END{
 W=900;H=520;L=80;R=210;T=60;B=70; pw=W-L-R; ph=H-T-B; lo=0; hi=8;
 col["rho-neg"]="#1b6ca8"; col["proj-a"]="#2e8b57"; col["pkm-iso-t6-g4"]="#e08a1e"; col["pkm-iso-t10-g9-tr"]="#a3262a";
 printf "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"%d\" height=\"%d\" viewBox=\"0 0 %d %d\" font-family=\"Helvetica,Arial,sans-serif\" font-size=\"12\">\n",W,H,W,H;
 printf "<rect width=\"100%%\" height=\"100%%\" fill=\"white\"/>\n";
 printf "<text x=\"%d\" y=\"22\" font-size=\"14\" font-weight=\"bold\">Curve25519 miniatures: mean S at w = 0.1 against k (log scale)</text>\n",L;
 printf "<text x=\"%d\" y=\"40\" fill=\"#555\">local L0; k = 20 from lead A; open marker = F4-term estimate (not measured); t10 means are lower bounds (censored)</text>\n",L;
 for(e=lo;e<=hi;e++){y=T+ph-(e-lo)/(hi-lo)*ph; printf "<line x1=\"%d\" x2=\"%d\" y1=\"%.1f\" y2=\"%.1f\" stroke=\"#ddd\"/><text x=\"%d\" y=\"%.1f\" text-anchor=\"end\">1e%d</text>\n",L,L+pw,y,y,L-6,y+4,e}
 split("20 24 28 32",KS," ");
 for(i=1;i<=4;i++){x=L+(KS[i]-18)/(34-18)*pw; printf "<line x1=\"%.1f\" x2=\"%.1f\" y1=\"%d\" y2=\"%d\" stroke=\"#eee\"/><text x=\"%.1f\" y=\"%d\" text-anchor=\"middle\">k = %d</text>\n",x,x,T,T+ph,x,T+ph+18,KS[i]}
 for(q=1;q<=na;q++){ s=nm[q]; path=""; prev=0
  for(i=1;i<=n;i++) if(a[i]==s){ x=L+(k[i]-18)/(34-18)*pw; y=T+ph-(lg(v[i])-lo)/(hi-lo)*ph;
    if(f[i]=="estimate"){ printf "<circle cx=\"%.1f\" cy=\"%.1f\" r=\"5\" fill=\"white\" stroke=\"%s\" stroke-width=\"2\"/>\n",x,y,col[s]; if(px!="" && ps==s) printf "<line x1=\"%.1f\" y1=\"%.1f\" x2=\"%.1f\" y2=\"%.1f\" stroke=\"%s\" stroke-dasharray=\"4 3\"/>\n",px,py,x,y,col[s] }
    else { printf "<circle cx=\"%.1f\" cy=\"%.1f\" r=\"4\" fill=\"%s\"/>\n",x,y,col[s]; if(px!="" && ps==s) printf "<line x1=\"%.1f\" y1=\"%.1f\" x2=\"%.1f\" y2=\"%.1f\" stroke=\"%s\" stroke-width=\"1.5\"/>\n",px,py,x,y,col[s] }
    printf "<text x=\"%.1f\" y=\"%.1f\" font-size=\"10\" text-anchor=\"middle\">%.3g</text>\n",x,y-9,v[i]; px=x; py=y; ps=s }
  px=""; ps="";
  ly=T+20+(q-1)*20; printf "<rect x=\"%d\" y=\"%d\" width=\"12\" height=\"12\" fill=\"%s\"/><text x=\"%d\" y=\"%d\">%s</text>\n",L+pw+20,ly-10,col[s],L+pw+38,ly,s }
 printf "<text x=\"%d\" y=\"%d\" text-anchor=\"middle\" transform=\"rotate(-90 22 %d)\">S = (GAE + w x native ops) / sqrt(r)</text>\n",22,T+ph/2,T+ph/2;
 printf "</svg>\n" }' "$1" > "$2"
