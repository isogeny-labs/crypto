# Renders reach.csv (recorded: odd primes dividing L_B(1) or L_B(-1) over every genus-2 and genus-3
# curve over F_2, and the n with a T_n hit) against the deployed Koblitz sizes of
# cover-genus-budget.csv (derived budget). Usage:
#   awk -v RUN=... -f reach.plot.awk ../../cover-genus-budget.csv reach.csv > reach.svg
BEGIN{FS=","; W=960; H=470; X0=230; X1=930; lo=log(2.5); hi=log(720)
 rowy["g2,L1"]=160; rowl["g2,L1"]="genus 2: n | L(1)"; wb["g2,L1"]=(1+sqrt(2))^4
 rowy["g3h,L1"]=210; rowl["g3h,L1"]="genus 3 hyperell.: n | L(1)"; wb["g3h,L1"]=(1+sqrt(2))^6
 rowy["g3q,L1"]=260; rowl["g3q,L1"]="plane quartic: n | L(1)"; wb["g3q,L1"]=(1+sqrt(2))^6
 rowy["g3h,Lm1"]=310; rowl["g3h,Lm1"]="genus 3 hyperell.: n | L(-1)"; wb["g3h,Lm1"]=(1+sqrt(2))^6
 rowy["g3q,Lm1"]=360; rowl["g3q,Lm1"]="plane quartic: n | L(-1)"; wb["g3q,Lm1"]=(1+sqrt(2))^6
 nr=split("g2,L1|g3h,L1|g3q,L1|g3h,Lm1|g3q,Lm1",rows,"|")}
function px(n){return X0+(log(n)-lo)/(hi-lo)*(X1-X0)}
FNR==NR{ if(FNR>1 && $2=="Koblitz"){ nd++; dn[nd]=$1; dv[nd]=$3+0; g0[nd]=int(1+($7-1)/$3) } next }
FNR>1{ key=$1","$2; np[key]++; pr[key,np[key]]=$3+0; ht[key,np[key]]=$4+0 }
END{
 printf "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"%d\" height=\"%d\" viewBox=\"0 0 %d %d\" font-family=\"Helvetica, Arial, sans-serif\">\n",W,H,W,H;
 printf "<rect width=\"%d\" height=\"%d\" fill=\"#fff\"/>\n",W,H;
 printf "<text x=\"%d\" y=\"28\" font-size=\"16\" font-weight=\"bold\" text-anchor=\"middle\">Which n can an etale Z/n cover of a genus-2 or genus-3 curve over F_2 reach?</text>\n",W/2;
 printf "<text x=\"%d\" y=\"47\" font-size=\"11.5\" fill=\"#444\" text-anchor=\"middle\">Dots: odd primes dividing L_B(1) (untwisted covers) or L_B(-1) (Frobenius acting by -1) for some curve; red: a character with a T_n factor exists.</text>\n",W/2;
 printf "<text x=\"%d\" y=\"63\" font-size=\"11.5\" fill=\"#444\" text-anchor=\"middle\">Grey band: Weil bound (1+sqrt 2)^(2g). Dashed: deployed Koblitz n, with the largest base genus g0 the L(1/2) budget allows.</text>\n",W/2;
 ytop=128; ybot=400;
 split("3 5 10 20 50 100 200 500",tk," ");
 for(i=1;i<=8;i++){x=px(tk[i]); printf "<line x1=\"%.1f\" y1=\"%d\" x2=\"%.1f\" y2=\"%d\" stroke=\"#eee\"/><text x=\"%.1f\" y=\"%d\" font-size=\"11\" text-anchor=\"middle\">%s</text>\n",x,ytop,x,ybot,x,ybot+16,tk[i]}
 printf "<line x1=\"%d\" y1=\"%d\" x2=\"%d\" y2=\"%d\" stroke=\"#333\"/>\n",X0,ybot,X1,ybot;
 printf "<text x=\"%d\" y=\"%d\" font-size=\"12\" text-anchor=\"middle\">n (log scale)</text>\n",(X0+X1)/2,ybot+34;
 for(r=1;r<=nr;r++){ key=rows[r]; y=rowy[key];
   printf "<rect x=\"%.1f\" y=\"%d\" width=\"%.1f\" height=\"16\" fill=\"#ececec\"/>\n",px(2.5),y-8,px(wb[key])-px(2.5);
   printf "<text x=\"%d\" y=\"%d\" font-size=\"12\" text-anchor=\"end\">%s</text>\n",X0-10,y+4,rowl[key];
   for(i=1;i<=np[key];i++){ p=pr[key,i]; x=px(p);
     if(ht[key,i]) printf "<circle cx=\"%.1f\" cy=\"%d\" r=\"6.5\" fill=\"#c1121f\"/>\n",x,y;
     else printf "<circle cx=\"%.1f\" cy=\"%d\" r=\"4\" fill=\"#457b9d\"/>\n",x,y }
   if(np[key]>0){ p=pr[key,np[key]]; printf "<text x=\"%.1f\" y=\"%d\" font-size=\"10\" fill=\"#333\">%d</text>\n",px(p)+7,y-6,p } }
 for(d=1;d<=nd;d++){ x=px(dv[d]); stagger=13*(d%3);
   closed=(g0[d]<=3); col=closed?"#c1121f":"#2a9d8f";
   printf "<line x1=\"%.1f\" y1=\"%d\" x2=\"%.1f\" y2=\"%d\" stroke=\"%s\" stroke-dasharray=\"4 3\"/>\n",x,ytop-4,x,ybot,col;
   printf "<text x=\"%.1f\" y=\"%d\" font-size=\"10.5\" text-anchor=\"middle\" fill=\"%s\">%s, g0 \342\211\244 %d</text>\n",x,ytop-8-stagger,col,dn[d],g0[d] }
 printf "<text x=\"%d\" y=\"%d\" font-size=\"11\" fill=\"#555\">Red dashed: closed (budget allows g0 &lt;= 3 and no genus-&lt;=3 curve reaches n). Teal dashed: open, needs g0 &gt;= 4. Run %s.</text>\n",X0-200,H-12,RUN;
 print "</svg>"}
