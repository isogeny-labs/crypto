#!/bin/bash
# SVG: log10 F4 multiply-adds per target, lifts 0/1/2, per shape and k (awk only).
cd "$(dirname "$0")"
awk -F'\t' '
function ypos(v){ return top + (ymax - log(v)/log(10)) / (ymax-ymin) * H }
$7=="full" && (($1 ~ /^ref20_/ && $1 !~ /_t4_/) || $1 ~ /^k(20|24|28)_/) { sh=$3" t="$4" G="$5; key=sh SUBSEP $2 SUBSEP $6; v[key]=$14; n[key]=$8; cap[key]=($9=="true"); shapes[sh]=1 }
BEGIN{ W=1180; Hh=520; top=70; H=360; left=70; ymin=5; ymax=12; split("isogeny t=4 G=0,isogeny t=5 G=0,isogeny t=6 G=0,bits t=5 G=3,bits t=6 G=3",order,","); 
  col[0]="#4c78a8"; col[1]="#e08a1e"; col[2]="#8e4b9e" }
END{
  printf "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 %d %d\" width=\"%d\" height=\"%d\" font-family=\"sans-serif\" font-size=\"11\">\n", W, Hh, W, Hh;
  printf "<rect width=\"%d\" height=\"%d\" fill=\"#ffffff\"/>\n", W, Hh;
  printf "<text x=\"%d\" y=\"24\" font-size=\"15\" font-weight=\"bold\">F4 multiply-adds per target by lifts arm, k = 20, 24, 28 (log scale)</text>\n", left;
  printf "<text x=\"%d\" y=\"42\" fill=\"#444\">Bars: lifts=0 (blue), lifts=1 (orange), lifts=2 (purple). Pale bar with dashed outline = cell truncated by the 600 s cap (mean over the targets done). Missing bar = no data. k=20: lead D rows (isogeny t=4: this study).</text>\n", left;
  for(e=ymin;e<=ymax;e++){ y=ypos(10^e); printf "<line x1=\"%d\" y1=\"%.1f\" x2=\"%d\" y2=\"%.1f\" stroke=\"#ddd\"/>\n<text x=\"%d\" y=\"%.1f\" text-anchor=\"end\" fill=\"#444\">1e%d</text>\n", left, y, W-20, y, left-6, y+4, e }
  pw=(W-left-20)/5; 
  for(i=1;i<=5;i++){ sh=order[i]; x0=left+(i-1)*pw;
    printf "<text x=\"%.1f\" y=\"%d\" text-anchor=\"middle\" font-weight=\"bold\">%s</text>\n", x0+pw/2, top+H+36, sh;
    bw=(pw-30)/9-3;
    for(kk=0;kk<3;kk++){ k=20+4*kk;
      printf "<text x=\"%.1f\" y=\"%d\" text-anchor=\"middle\" fill=\"#444\">k=%d</text>\n", x0+10+kk*3*(bw+3)+1.5*(bw+3), top+H+16, k;
      for(L=0;L<3;L++){ key=sh SUBSEP k SUBSEP L; x=x0+10+(kk*3+L)*(bw+3);
        if(key in v){ y=ypos(v[key]); h=top+H-y;
          printf "<rect x=\"%.1f\" y=\"%.1f\" width=\"%.1f\" height=\"%.1f\" fill=\"%s\"%s/>\n", x, y, bw, h, col[L], (cap[key]? " fill-opacity=\"0.45\" stroke=\"#000\" stroke-dasharray=\"3,2\"":"");
          printf "<text x=\"%.1f\" y=\"%.1f\" text-anchor=\"middle\" font-size=\"9\" transform=\"rotate(-90 %.1f %.1f)\" fill=\"#222\">%.2g (%d)</text>\n", x+bw/2+3, y-3, x+bw/2+3, y-3, v[key], n[key];
        } } } }
  printf "<line x1=\"%d\" y1=\"%d\" x2=\"%d\" y2=\"%d\" stroke=\"#000\"/>\n", left, top+H, W-20, top+H;
  printf "<text x=\"%d\" y=\"%d\" fill=\"#444\">Label above each bar: mean F4 multiply-adds per target (targets done). Source: analysis/rows.tsv, analysis/figure_data.tsv.</text>\n", left, Hh-12;
  printf "</svg>\n"; }' allrows.tsv > scale_lifts.svg
awk -F'\t' '$7=="full"{print $2"\t"$3" t="$4" G="$5"\t"$6"\t"$8"\t"$9"\t"$14}' allrows.tsv | sort -k1,1n -k2,2 -k3,3n > figure_data.tsv
