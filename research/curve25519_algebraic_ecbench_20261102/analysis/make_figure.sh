#!/bin/sh
# make_figure.sh band.json out.svg : grouped bars of mean S per arm at w = 0, 0.1, 1 (log scale). jq + awk only.
set -e
exec > "$2"
jq -r '.rows[] | [.arm, .mean_s[0], .mean_s[1], .mean_s[2]] | @tsv' "$1" | awk -F'\t' '
function lg(v){return log(v)/log(10)}
{arm[NR]=$1; s[NR,1]=$2; s[NR,2]=$3; s[NR,3]=$4; n=NR; for(j=1;j<=3;j++){ if($(j+1)+0>0){ if(min==""||$(j+1)<min)min=$(j+1); if($(j+1)>max)max=$(j+1)}}}
END{
 W=900;H=470;L=80;R=20;T=50;B=120; pw=W-L-R; ph=H-T-B;
 lo=int(lg(min)-0.999); if(lg(min)<0&&lo>lg(min))lo=lo-1; hi=int(lg(max))+1;
 col[1]="#1b6ca8";col[2]="#e08a1e";col[3]="#a3262a";
 printf "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"%d\" height=\"%d\" viewBox=\"0 0 %d %d\" font-family=\"Helvetica,Arial,sans-serif\" font-size=\"12\">\n",W,H,W,H;
 printf "<rect width=\"100%%\" height=\"100%%\" fill=\"white\"/>\n";
 printf "<text x=\"%d\" y=\"20\" font-size=\"14\" font-weight=\"bold\">k = 20 Curve25519 miniature: mean S per arm, priced at w = 0, 0.1, 1 GAE per native operation (log scale)</text>\n",L;
 printf "<text x=\"%d\" y=\"37\" fill=\"#555\">local L0 run; full solves; one curve; S = counted GAE / sqrt(r); lower is better</text>\n",L;
 for(e=lo;e<=hi;e++){y=T+ph-(e-lo)/(hi-lo)*ph; printf "<line x1=\"%d\" x2=\"%d\" y1=\"%.1f\" y2=\"%.1f\" stroke=\"#ddd\"/><text x=\"%d\" y=\"%.1f\" text-anchor=\"end\">1e%d</text>\n",L,W-R,y,y,L-6,y+4,e}
 gw=pw/n; bw=gw/4.5;
 for(i=1;i<=n;i++){ gx=L+(i-1)*gw;
  for(j=1;j<=3;j++){ v=s[i,j]; if(v+0<=0)continue; h=(lg(v)-lo)/(hi-lo)*ph; x=gx+bw*0.5+(j-1)*bw; y=T+ph-h;
   printf "<rect x=\"%.1f\" y=\"%.1f\" width=\"%.1f\" height=\"%.1f\" fill=\"%s\"/>\n",x,y,bw-2,h,col[j];
   printf "<text x=\"%.1f\" y=\"%.1f\" font-size=\"9\" text-anchor=\"middle\">%.3g</text>\n",x+bw/2-1,y-3,v }
  printf "<text x=\"%.1f\" y=\"%d\" text-anchor=\"end\" transform=\"rotate(-30 %.1f %d)\">%s</text>\n",gx+gw/2+10,T+ph+16,gx+gw/2+10,T+ph+16,arm[i] }
 lx=L; ly=H-22;
 split("w = 0 (F4 work unpriced),w = 0.1,w = 1",nm,",");
 for(j=1;j<=3;j++){ printf "<rect x=\"%d\" y=\"%d\" width=\"12\" height=\"12\" fill=\"%s\"/><text x=\"%d\" y=\"%d\">%s</text>\n",lx+(j-1)*230,ly,col[j],lx+(j-1)*230+18,ly+11,nm[j] }
 printf "</svg>\n" }'
