#!/bin/bash
# jq + awk SVG: multiply-adds per target (log10), matched targets, lifts=0/1/2 per cell.
D="$(cd "$(dirname "$0")" && pwd)"
jq -s -r '
 [.[]|select(.failed|not)|select(.trace|not)] as $a |
 ($a[]|select(.lifts_mode==2)) as $b |
 (["0","1"] | map(. as $l | $a[]|select(.form==$b.form and .t==$b.t and .guess==$b.guess and (.lifts_mode|tostring)==$l and .targets_done==$b.targets_done)) + [$b]) |
 .[] | "\(.form)_t\(.t)\t\(.lifts_mode)\t\(.muladds_per_target)\t\(.targets_done)"' $D/f4cost_shared.jsonl > $D/fig_data.tsv
awk -F'\t' '
BEGIN{W=760;H=420;L=70;R=20;T=50;B=70; ymin=5; ymax=12;
 print "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 " W " " H "\" font-family=\"sans-serif\" font-size=\"12\">";
 print "<rect width=\"100%\" height=\"100%\" fill=\"#ffffff\"/>";
 print "<text x=\"" L "\" y=\"22\" font-size=\"15\" font-weight=\"bold\" fill=\"#111\">Shared system (lifts=2) vs 8 separate systems (lifts=1) vs blind (lifts=0)</text>";
 print "<text x=\"" L "\" y=\"40\" fill=\"#444\">F4 multiply-adds per target, log10, arms on the same targets (k=20 cells; lifts=2 capped in 3 cells, see table)</text>";
 col[0]="#6b7280"; col[1]="#2563eb"; col[2]="#d97706";
 pw=W-L-R; ph=H-T-B;
 for(y=ymin;y<=ymax;y++){yy=T+ph*(ymax-y)/(ymax-ymin); print "<line x1=\"" L "\" x2=\"" W-R "\" y1=\"" yy "\" y2=\"" yy "\" stroke=\"#e5e7eb\"/>"; print "<text x=\"" L-8 "\" y=\"" yy+4 "\" text-anchor=\"end\" fill=\"#444\">1e" y "</text>"}
}
{ c=$1; if(!(c in idx)){idx[c]=n++; names[n-1]=c; tg[c]=$4} v[c,$2]=$3 }
END{
 gw=pw/n;
 for(i=0;i<n;i++){c=names[i]; gx=L+gw*i;
  for(a=0;a<3;a++){ val=v[c,a]; h=ph*(log(val)/log(10)-ymin)/(ymax-ymin); bx=gx+gw*0.15+a*gw*0.23; bw=gw*0.2;
   print "<rect x=\"" bx "\" y=\"" T+ph-h "\" width=\"" bw "\" height=\"" h "\" fill=\"" col[a] "\"/>";
   printf "<text x=\"%f\" y=\"%f\" text-anchor=\"middle\" font-size=\"10\" fill=\"#111\">%.1e</text>\n", bx+bw/2, T+ph-h-4, val }
  print "<text x=\"" gx+gw/2 "\" y=\"" H-B+18 "\" text-anchor=\"middle\" fill=\"#111\">" c "</text>";
  print "<text x=\"" gx+gw/2 "\" y=\"" H-B+33 "\" text-anchor=\"middle\" fill=\"#444\">" tg[c] " targets</text>" }
 lx=L; ly=H-14;
 print "<rect x=\"" lx "\" y=\"" ly-10 "\" width=\"12\" height=\"12\" fill=\"#6b7280\"/><text x=\"" lx+18 "\" y=\"" ly "\" fill=\"#111\">lifts=0 (blind, 1 system)</text>";
 print "<rect x=\"" lx+190 "\" y=\"" ly-10 "\" width=\"12\" height=\"12\" fill=\"#2563eb\"/><text x=\"" lx+208 "\" y=\"" ly "\" fill=\"#111\">lifts=1 (8 systems)</text>";
 print "<rect x=\"" lx+360 "\" y=\"" ly-10 "\" width=\"12\" height=\"12\" fill=\"#d97706\"/><text x=\"" lx+378 "\" y=\"" ly "\" fill=\"#111\">lifts=2 (one shared system)</text>";
 print "</svg>" }' $D/fig_data.tsv > $D/shared_vs_separate.svg
echo ok
