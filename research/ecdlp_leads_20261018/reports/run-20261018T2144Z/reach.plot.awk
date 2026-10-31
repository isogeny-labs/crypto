# Renders reach.csv (recorded: odd primes dividing L_B(1) and L_B(-1) over every genus-4 curve
# over F_2, and the n with n | P_B(lambda) for lambda of order 3) against the deployed Koblitz
# sizes allowing g0 = 4 in cover-genus-budget.csv (derived budget). Usage:
#   awk -v RUN=... -f reach.plot.awk ../../cover-genus-budget.csv reach.csv > reach.svg
BEGIN{FS=","; W=960; H=430; X0=250; X1=930; lo=log(2.5); hi=log(1400); wb=(1+sqrt(2))^8
 rowy["hyperelliptic,L1"]=160; rowl["hyperelliptic,L1"]="hyperelliptic: n | L(1) = L(-1) set"
 rowy["canonical_genus4,L1"]=210; rowl["canonical_genus4,L1"]="canonical (trigonal): n | L(1)"
 rowy["canonical_genus4,Lm1"]=260; rowl["canonical_genus4,Lm1"]="canonical (trigonal): n | L(-1)"
 rowy["any,P_ord3"]=320; rowl["any,P_ord3"]="any: n | P(lambda), ord lambda = 3"
 nr=split("hyperelliptic,L1|canonical_genus4,L1|canonical_genus4,Lm1|any,P_ord3",rows,"|")}
function px(n){return X0+(log(n)-lo)/(hi-lo)*(X1-X0)}
FNR==NR{ if(FNR>1 && $2=="Koblitz"){ g0=int(1+($7-1)/$3); if(g0>=4){ nd++; dn[nd]=$1; dv[nd]=$3+0; gm[nd]=g0 } } next }
FNR>1{ key=($2=="P_ord3") ? "any,P_ord3" : $1","$2; if(!((key SUBSEP $3) in seen)){ seen[key,$3]=1; np[key]++; pr[key,np[key]]=$3+0 } }
END{
 printf "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"%d\" height=\"%d\" viewBox=\"0 0 %d %d\" font-family=\"Helvetica, Arial, sans-serif\">\n",W,H,W,H;
 printf "<rect width=\"%d\" height=\"%d\" fill=\"#fff\"/>\n",W,H;
 printf "<text x=\"%d\" y=\"28\" font-size=\"16\" font-weight=\"bold\" text-anchor=\"middle\">Genus-4 bases over F_2: which n can an etale Z/n cover reach?</text>\n",W/2;
 printf "<text x=\"%d\" y=\"47\" font-size=\"11.5\" fill=\"#444\" text-anchor=\"middle\">Every genus-4 curve over F_2 (49,152 hyperelliptic + 55,368 canonical models; 620 L-polynomials, matching Xarles 2020).</text>\n",W/2;
 printf "<text x=\"%d\" y=\"63\" font-size=\"11.5\" fill=\"#444\" text-anchor=\"middle\">Dots: odd primes n with n | L(1) (untwisted) or n | L(-1) (Frobenius acting by -1). Squares: n | P(lambda) with ord lambda = 3. Grey: Weil bound.</text>\n",W/2;
 ytop=118; ybot=360;
 split("3 5 10 20 50 100 200 500 1000",tk," ");
 for(i=1;i<=9;i++){x=px(tk[i]); printf "<line x1=\"%.1f\" y1=\"%d\" x2=\"%.1f\" y2=\"%d\" stroke=\"#eee\"/><text x=\"%.1f\" y=\"%d\" font-size=\"11\" text-anchor=\"middle\">%s</text>\n",x,ytop,x,ybot,x,ybot+16,tk[i]}
 printf "<line x1=\"%d\" y1=\"%d\" x2=\"%d\" y2=\"%d\" stroke=\"#333\"/>\n",X0,ybot,X1,ybot;
 printf "<text x=\"%d\" y=\"%d\" font-size=\"12\" text-anchor=\"middle\">n (log scale)</text>\n",(X0+X1)/2,ybot+34;
 for(r=1;r<=nr;r++){ key=rows[r]; y=rowy[key];
   printf "<rect x=\"%.1f\" y=\"%d\" width=\"%.1f\" height=\"16\" fill=\"#ececec\"/>\n",px(2.5),y-8,px(wb)-px(2.5);
   printf "<text x=\"%d\" y=\"%d\" font-size=\"12\" text-anchor=\"end\">%s</text>\n",X0-10,y+4,rowl[key];
   mx=0;
   for(i=1;i<=np[key];i++){ p=pr[key,i]; x=px(p); if(p>mx) mx=p;
     if(key=="any,P_ord3") printf "<rect x=\"%.1f\" y=\"%d\" width=\"10\" height=\"10\" fill=\"#f4a261\" stroke=\"#b5651d\"/>\n",x-5,y-5;
     else printf "<circle cx=\"%.1f\" cy=\"%d\" r=\"4\" fill=\"#457b9d\"/>\n",x,y }
   if(mx>0 && key!="any,P_ord3") printf "<text x=\"%.1f\" y=\"%d\" font-size=\"10\" fill=\"#333\">%d</text>\n",px(mx)+7,y-6,mx }
 printf "<text x=\"%d\" y=\"%d\" font-size=\"10.5\" fill=\"#b5651d\">excluded for a = 0 (point-count lemma)</text>\n",X0+4,rowy["any,P_ord3"]+22;
 for(d=1;d<=nd;d++){ x=px(dv[d]); stagger=13*(d%3); closed=(gm[d]<=4); col=closed?"#c1121f":"#2a9d8f";
   printf "<line x1=\"%.1f\" y1=\"%d\" x2=\"%.1f\" y2=\"%d\" stroke=\"%s\" stroke-dasharray=\"4 3\"/>\n",x,ytop-4,x,ybot,col;
   printf "<text x=\"%.1f\" y=\"%d\" font-size=\"10.5\" text-anchor=\"middle\" fill=\"%s\">%s, g0 \342\211\244 %d</text>\n",x,ytop-8-stagger,col,dn[d],gm[d] }
 printf "<text x=\"%d\" y=\"%d\" font-size=\"11\" fill=\"#555\">Red dashed: closed (budget allows g0 &lt;= 4 and no genus-&lt;=4 base reaches n). Teal dashed: g0 = 4 is closed too, g0 &gt;= 5 open. Run %s.</text>\n",30,H-12,RUN;
 print "</svg>"}
