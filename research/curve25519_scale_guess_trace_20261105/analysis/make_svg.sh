#!/usr/bin/env bash
# scale.svg from cells.tsv (awk only). Panel A: F4 multiply-adds per target vs k (log), trace 0 solid, trace 1 dashed.
# Panel B: trace 1 / trace 0 per-target ratio vs k with the 0.29-0.45 band.
set -eu
D=$(cd "$(dirname "$0")" && pwd)
awk -F'\t' '
function X(k,x0,w){return x0+(k-18)/12*w}
function colr(t){return (t==6)?"#1b6ca8":(t==8)?"#c9731a":"#2a8a4a"}
NR>1{ if($2=="leadB"){m0[$4" "$5" "$3]=$10}
 else if($2=="this"){ s=($1~/^main/)?"m":($1~/^repB/)?"B":"C"; key=$4" "$5" "$3;
   if($6==0){ if(s=="m")m0[key]=$10; p[s" "key]=$10 } else { if(s=="m")m1[key]=$10; r[s" "key]=$10/p[s" "key]; m1k[s" "key]=$10 } } }
END{
 W=980;H=470; printf "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 %d %d\" font-family=\"sans-serif\" font-size=\"11\">\n",W,H;
 printf "<rect width=\"%d\" height=\"%d\" fill=\"#fff\"/>\n",W,H;
 printf "<text x=\"10\" y=\"18\" font-size=\"13\" font-weight=\"bold\">Scale-up S1: isogeny form, m=2, full, lifts 0, 100 targets/cell (k=20 plain: lead B; k=20 trace: reproduction cells)</text>\n";
 # panel A
 x0=60;w=380;y0=50;h=340; lo=4;hi=11; # log10
 printf "<text x=\"%d\" y=\"40\" font-weight=\"bold\">A. F4 multiply-adds per target vs k (solid: trace 0, dashed: trace 1)</text>\n",x0;
 printf "<rect x=\"%d\" y=\"%d\" width=\"%d\" height=\"%d\" fill=\"none\" stroke=\"#888\"/>\n",x0,y0,w,h;
 for(e=lo;e<=hi;e++){y=y0+h-(e-lo)/(hi-lo)*h; printf "<line x1=\"%d\" x2=\"%d\" y1=\"%.1f\" y2=\"%.1f\" stroke=\"#ddd\"/><text x=\"%d\" y=\"%.1f\" text-anchor=\"end\">1e%d</text>\n",x0,x0+w,y,y,x0-4,y+4,e}
 for(k=20;k<=28;k+=4){printf "<text x=\"%.1f\" y=\"%d\" text-anchor=\"middle\">k=%d</text>\n",X(k,x0,w),y0+h+16,k}
 ly=y0+14;
 for(t=6;t<=10;t+=2) for(i=0;i<2;i++){G=(i==0)?4:t-1;
  for(arm=0;arm<2;arm++){ path="";dots="";
   for(k=20;k<=28;k+=4){ key=t" "G" "k; v=(arm==0)?m0[key]:m1[key];
     if(arm==1&&k==20){ s=(G==t-1)?"B":"C"; kk=s" "t" "G" 20"; v=((t==10||(t==6&&G==5)))?m1k["B "t" "G" 20"]:((G==4&&(t==6||t==8))?m1k["C "t" "G" 20"]:0) }
     if(v>0){px=X(k,x0,w);py=y0+h-(log(v)/log(10)-lo)/(hi-lo)*h; path=path (path==""?"M":"L") sprintf("%.1f %.1f ",px,py); dots=dots sprintf("<circle cx=\"%.1f\" cy=\"%.1f\" r=\"3\" fill=\"%s\"/>",px,py,colr(t)) } }
   if(path!="") printf "<path d=\"%s\" fill=\"none\" stroke=\"%s\" stroke-width=\"1.5\" %s/>%s\n",path,colr(t),(arm==1)?"stroke-dasharray=\"5 3\"":"",dots }
  printf "<text x=\"%d\" y=\"%d\" fill=\"%s\">t=%d G=%d</text>\n",x0+w+8,y0+14+(((t-6)/2*2+i)*14),colr(t),t,G;
  kk=t" "G" 28"; v=m0[kk]; if(v>0) printf "<text x=\"%.1f\" y=\"%.1f\" font-size=\"9\" fill=\"%s\">%d/%d</text>\n",X(28,x0,w)+6,y0+h-(log(v)/log(10)-lo)/(hi-lo)*h-4,colr(t),t,G }
 # panel B
 x1=600;w1=330; rlo=0.25;rhi=0.55;
 printf "<text x=\"%d\" y=\"40\" font-weight=\"bold\">B. trace 1 / trace 0 per target (learning excluded)</text>\n",x1;
 printf "<rect x=\"%d\" y=\"%d\" width=\"%d\" height=\"%d\" fill=\"none\" stroke=\"#888\"/>\n",x1,y0,w1,h;
 yb=y0+h-(0.45-rlo)/(rhi-rlo)*h; ya=y0+h-(0.29-rlo)/(rhi-rlo)*h;
 printf "<rect x=\"%d\" y=\"%.1f\" width=\"%d\" height=\"%.1f\" fill=\"#cfe8cf\" opacity=\"0.6\"/><text x=\"%d\" y=\"%.1f\">frozen band 0.29-0.45</text>\n",x1,yb,w1,ya-yb,x1+4,ya-4;
 for(v=0.25;v<=0.551;v+=0.05){y=y0+h-(v-rlo)/(rhi-rlo)*h; printf "<text x=\"%d\" y=\"%.1f\" text-anchor=\"end\">%.2f</text>\n",x1-4,y+4,v}
 for(k=20;k<=28;k+=4){printf "<text x=\"%.1f\" y=\"%d\" text-anchor=\"middle\">k=%d</text>\n",X(k,x1,w1),y0+h+16,k}
 for(t=6;t<=10;t+=2) for(i=0;i<2;i++){G=(i==0)?4:t-1; path="";dots="";
   for(k=20;k<=28;k+=4){ v=0; if(k==20){ if(t==10||(t==6&&G==5))v=r["B "t" "G" 20"]; else if(G==4&&(t==6||t==8))v=r["C "t" "G" 20"] } else v=r["m "t" "G" "k];
     if(v>0){px=X(k,x1,w1);py=y0+h-(v-rlo)/(rhi-rlo)*h; path=path (path==""?"M":"L") sprintf("%.1f %.1f ",px,py); dots=dots sprintf("<circle cx=\"%.1f\" cy=\"%.1f\" r=\"3\" fill=\"%s\"/>",px,py,colr(t)) } }
   if(path!="") printf "<path d=\"%s\" fill=\"none\" stroke=\"%s\" stroke-width=\"1.5\" %s/>%s\n",path,colr(t),(i==1)?"stroke-dasharray=\"5 3\"":"",dots }
 printf "<text x=\"%d\" y=\"%d\">colour = t (6 blue, 8 orange, 10 green); solid G=4, dashed G=t-1 (panel B)</text>\n",x1,y0+h+36;
 printf "<text x=\"10\" y=\"%d\" font-size=\"10\" fill=\"#555\">Single seed per cell; k=24/28 differ in curve and target set. Missing points = no cell measured (no k=20 trace pair at that (t,G)) or a failure. Shared box; wall time not shown.</text>\n",H-8;
 print "</svg>" }' "$D/cells.tsv" > "$D/scale.svg"
echo wrote scale.svg
