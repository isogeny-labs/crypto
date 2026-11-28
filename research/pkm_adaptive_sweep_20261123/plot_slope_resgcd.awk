# usage: awk -f plot_slope_resgcd.awk t_scaling_per_call.tsv slope_points.tsv > FIG.svg
function lx(v,a,b,x0,x1){return x0+(v-a)/(b-a)*(x1-x0)}
function ly(v,a,b,y0,y1){return y0-(log(v)/log(10)-a)/(b-a)*(y0-y1)}
BEGIN{FS="\t"; W=980;H=440
 col["cand-f4"]="#c2410c"; col["int-f4"]="#7c2d12"; col["cand-resgcd"]="#1d4ed8"; col["int-resgcd"]="#60a5fa"; col["cand-scan"]="#15803d"; col["cand-grid"]="#6b7280"
 nm["cand-f4"]="F4, Vélu base"; nm["int-f4"]="F4, interval base"; nm["cand-resgcd"]="resgcd, Vélu base (identical, hidden)"; nm["int-resgcd"]="resgcd, interval base"; nm["cand-scan"]="scan (O(|V|))"; nm["cand-grid"]="grid (|V|²/2)"}
FNR==NR{L=$1; T=$2+0; V=$3+0; pts[L]=pts[L] sprintf("%.1f,%.1f ", lx(T,4.5,12.5,70,450), ly(V,3,9.5,380,50)); labs[L]=1; next}
{ b=$1+0; a=$3; if(a=="cand-f4"||a=="int-f4"){ s[b" "$2" "a]=$5; m[b" "$2" "a]=$4; keyb[b" "$2]=b } }
END{
 print "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"" W "\" height=\"" H "\" font-family=\"sans-serif\" font-size=\"12\">"
 print "<rect width=\"100%\" height=\"100%\" fill=\"#ffffff\"/>"
 # panel 1 axes
 print "<text x=\"260\" y=\"25\" text-anchor=\"middle\" font-size=\"14\" font-weight=\"bold\">m=2 solver cost per call vs tower height t (16-bit curve)</text>"
 print "<line x1=\"70\" y1=\"380\" x2=\"450\" y2=\"380\" stroke=\"#000\"/><line x1=\"70\" y1=\"50\" x2=\"70\" y2=\"380\" stroke=\"#000\"/>"
 for(t=5;t<=12;t++){x=lx(t,4.5,12.5,70,450); print "<line x1=\""x"\" y1=\"380\" x2=\""x"\" y2=\"385\" stroke=\"#000\"/><text x=\""x"\" y=\"398\" text-anchor=\"middle\">"t"</text>"}
 for(e=3;e<=9;e++){y=ly(10^e,3,9.5,380,50); print "<line x1=\"65\" y1=\""y"\" x2=\"450\" y2=\""y"\" stroke=\"#e5e7eb\"/><text x=\"60\" y=\""y+4"\" text-anchor=\"end\">1e"e"</text>"}
 print "<text x=\"260\" y=\"420\" text-anchor=\"middle\">t  (|V| = 2^t)</text><text x=\"18\" y=\"215\" transform=\"rotate(-90 18 215)\" text-anchor=\"middle\">F_p ops per solver call</text>"
 i=0; for(L in labs){ print "<polyline fill=\"none\" stroke-width=\"2\" stroke=\"" col[L] "\" points=\"" pts[L] "\"/>"; n=split(pts[L],P," "); for(j=1;j<=n;j++){split(P[j],q,","); print "<circle cx=\""q[1]"\" cy=\""q[2]"\" r=\"3\" fill=\""col[L]"\"/>"}
   ly0=60+16*i; print "<rect x=\"300\" y=\""ly0-9"\" width=\"12\" height=\"3\" fill=\""col[L]"\"/><text x=\"318\" y=\""ly0"\">"nm[L]"</text>"; i++ }
 # panel 2
 X0=560; X1=950
 print "<text x=\"755\" y=\"25\" text-anchor=\"middle\" font-size=\"14\" font-weight=\"bold\">F4 closed pipeline, t=5: slope over 13–16 bits</text>"
 print "<line x1=\""X0"\" y1=\"380\" x2=\""X1"\" y2=\"380\" stroke=\"#000\"/><line x1=\""X0"\" y1=\"50\" x2=\""X0"\" y2=\"380\" stroke=\"#000\"/>"
 for(bb=13;bb<=16;bb++){x=lx(bb,12.5,16.5,X0,X1); print "<line x1=\""x"\" y1=\"380\" x2=\""x"\" y2=\"385\" stroke=\"#000\"/><text x=\""x"\" y=\"398\" text-anchor=\"middle\">"bb"</text>"}
 for(e=-2;e<=9;e++){y=ly(10^e,-2,9.5,380,50); print "<line x1=\""X0-5"\" y1=\""y"\" x2=\""X1"\" y2=\""y"\" stroke=\"#e5e7eb\"/><text x=\""X0-8"\" y=\""y+4"\" text-anchor=\"end\">1e"e"</text>"}
 print "<text x=\"755\" y=\"420\" text-anchor=\"middle\">field bits</text>"
 for(k in keyb){ bb=keyb[k]; x=lx(bb,12.5,16.5,X0,X1)+((k~/02$|76$/)?6:((k~/01$|21$/)?0:-6))
   r=m[k" cand-f4"]/m[k" int-f4"]; print "<circle cx=\""x"\" cy=\""ly(r,-2,9.5,380,50)"\" r=\"4\" fill=\"#7c3aed\"/>"
   print "<circle cx=\""x"\" cy=\""ly(s[k" cand-f4"],-2,9.5,380,50)"\" r=\"4\" fill=\"#c2410c\"/>"
   print "<circle cx=\""x"\" cy=\""ly(s[k" int-f4"],-2,9.5,380,50)"\" r=\"4\" fill=\"#7c2d12\"/>" }
 y1=ly(1,-2,9.5,380,50); print "<line x1=\""X0"\" y1=\""y1"\" x2=\""X1"\" y2=\""y1"\" stroke=\"#111\" stroke-dasharray=\"5 4\"/><text x=\""X1-4"\" y=\""y1-5"\" text-anchor=\"end\">rho (s/rho = 1)</text>"
 print "<circle cx=\"580\" cy=\"262\" r=\"4\" fill=\"#c2410c\"/><text x=\"590\" y=\"266\">s_ops/rho, Vélu base (slope +0.64 bits/bit)</text>"
 print "<circle cx=\"580\" cy=\"280\" r=\"4\" fill=\"#7c2d12\"/><text x=\"590\" y=\"284\">s_ops/rho, interval base (slope +0.52)</text>"
 print "<circle cx=\"580\" cy=\"298\" r=\"4\" fill=\"#7c3aed\"/><text x=\"590\" y=\"302\">F4 multiply-add ratio Vélu/interval (slope +0.07)</text>"
 print "</svg>" }
