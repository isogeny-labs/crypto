#!/usr/bin/env bash
# SVG figure from analysis/p1_cells.tsv and analysis/p2_ratios.tsv (awk only).
cd "$(dirname "$0")"
awk -F'\t' '
function lg(x){return log(x)/log(10)}
BEGIN{W=860;H=640}
FILENAME ~ /p1_cells/ { if($19!="-" && $19+0>0 && $4==6 && $3=="isogeny"){n++; lab[n]="k"$2" G"$5" "($6==2?"m2":"m3 "$7); val[n]=lg($19+0); typ[n]=$6==2?2:($7=="chain"?31:32)} next}
FILENAME ~ /p2_ratios/ { split($1,kk," "); if(kk[1]!=20){q++; rl[q]="k"kk[1]" t"kk[2]" G"kk[3]; rv[q]=$7+0; cv[q]=$6+0} next}
END{
 printf "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 %d %d\" width=\"%d\" font-family=\"sans-serif\" font-size=\"11\">\n",W,H,W
 print "<rect width=\"100%\" height=\"100%\" fill=\"#ffffff\"/>"
 print "<text x=\"20\" y=\"22\" font-size=\"14\" font-weight=\"bold\">Panel A: per-relation F4 multiply-adds at coverage_exact, isogeny t = 6 (log10)</text>"
 # panel A: horizontal bars, x from 0 to 16 decades
 x0=190; sc=34; y=45
 for(i=1;i<=n;i++){
   col=(typ[i]==2?"#1f6fb2":(typ[i]==31?"#c4561a":"#6a3d9a"))
   printf "<text x=\"%d\" y=\"%d\" text-anchor=\"end\">%s</text>\n",x0-6,y+12,lab[i]
   printf "<rect x=\"%d\" y=\"%d\" width=\"%.1f\" height=\"14\" fill=\"%s\"/>\n",x0,y,val[i]*sc/1,col
   printf "<text x=\"%.1f\" y=\"%d\">1e%.1f</text>\n",x0+val[i]*sc+4,y+12,val[i]
   y+=20
 }
 for(d=0;d<=14;d+=2){printf "<line x1=\"%d\" y1=\"40\" x2=\"%d\" y2=\"%d\" stroke=\"#cccccc\"/><text x=\"%d\" y=\"%d\" text-anchor=\"middle\">1e%d</text>\n",x0+d*sc,x0+d*sc,y,x0+d*sc,y+14,d}
 printf "<text x=\"%d\" y=\"%d\">blue m = 2 full, orange m = 3 chain, purple m = 3 full; only cells with F4 cost and coverage</text>\n",x0,y+30
 # panel B
 y2=y+75
 printf "<text x=\"20\" y=\"%d\" font-size=\"14\" font-weight=\"bold\">Panel B: per-relation ratio t0-bits / bits (m = 2, k = 24, 28); the line is 1</text>\n",y2
 bx=60; by=y2+140; bs=24
 printf "<line x1=\"%d\" y1=\"%d\" x2=\"%d\" y2=\"%d\" stroke=\"#000000\"/>\n",bx,by,bx+q*58+10,by
 printf "<line x1=\"%d\" y1=\"%d\" x2=\"%d\" y2=\"%d\" stroke=\"#c00000\" stroke-dasharray=\"4\"/>\n",bx,by-bs,bx+q*58+10,by-bs
 printf "<text x=\"%d\" y=\"%d\" fill=\"#c00000\">1</text>\n",bx-14,by-bs+4
 for(i=1;i<=q;i++){h=rv[i]*bs; printf "<rect x=\"%d\" y=\"%.1f\" width=\"36\" height=\"%.1f\" fill=\"#2a8a5a\"/><text x=\"%d\" y=\"%.1f\" text-anchor=\"middle\">%.2f</text><text x=\"%d\" y=\"%d\" text-anchor=\"middle\" font-size=\"9\">%s</text>\n",bx+(i-1)*58+10,by-h,h,bx+(i-1)*58+28,by-h-4,rv[i],bx+(i-1)*58+28,by+14,rl[i]}
 print "</svg>"
}' p1_cells.tsv p2_ratios.tsv > ratios_and_costs.svg
