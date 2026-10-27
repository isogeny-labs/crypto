function px(v){return X0+(v-nmin)/(nmax-nmin)*(X1-X0)}
function py(g){return Y0-(log(g)/log(10)-lmin)/(lmax-lmin)*(Y0-Y1)}
BEGIN{FS=","; X0=95; X1=860; Y0=470; Y1=60; nmin=100; nmax=600; lmin=2; lmax=5}
NR>1 && $2=="Koblitz"{n[++k]=$3; gm[k]=$7; et[k]=$11; kr[k]=$12; nm[k]=$1}
END{
 print "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"920\" height=\"620\" viewBox=\"0 0 920 620\" font-family=\"Helvetica, Arial, sans-serif\">";
 print "<rect width=\"920\" height=\"620\" fill=\"#ffffff\"/>";
 printf "<text x=\"460\" y=\"26\" font-size=\"16\" text-anchor=\"middle\" font-weight=\"bold\">Cover genus needed to beat matched rho on deployed Koblitz curves (derived, not measured)</text>\n";
 printf "<text x=\"460\" y=\"45\" font-size=\"11.5\" text-anchor=\"middle\" fill=\"#444\">Budget: largest g with L_{2^g}[1/2, sqrt 2] below matched rho, o(1) dropped. Data: cover-genus-budget.csv (study ecdlp_leads_20261018)</text>\n";
 for(e=2;e<=5;e++){ y=py(10^e); printf "<line x1=\"%d\" y1=\"%.1f\" x2=\"%d\" y2=\"%.1f\" stroke=\"#d0d0d0\"/>\n",X0,y,X1,y; printf "<text x=\"%d\" y=\"%.1f\" font-size=\"12\" text-anchor=\"end\">10^%d</text>\n",X0-8,y+4,e;
   if(e<5) for(m=2;m<10;m++){ y2=py(m*10^e); printf "<line x1=\"%d\" y1=\"%.1f\" x2=\"%d\" y2=\"%.1f\" stroke=\"#f0f0f0\"/>\n",X0,y2,X1,y2 } }
 for(v=100;v<=600;v+=100){ x=px(v); printf "<line x1=\"%.1f\" y1=\"%d\" x2=\"%.1f\" y2=\"%d\" stroke=\"#ececec\"/>\n",x,Y0,x,Y1; printf "<text x=\"%.1f\" y=\"%d\" font-size=\"12\" text-anchor=\"middle\">%d</text>\n",x,Y0+18,v }
 printf "<line x1=\"%d\" y1=\"%d\" x2=\"%d\" y2=\"%d\" stroke=\"#333\"/><line x1=\"%d\" y1=\"%d\" x2=\"%d\" y2=\"%d\" stroke=\"#333\"/>\n",X0,Y0,X1,Y0,X0,Y0,X0,Y1;
 printf "<text x=\"%d\" y=\"%d\" font-size=\"13\" text-anchor=\"middle\">extension degree n of F_{2^n}</text>\n",(X0+X1)/2,Y0+40;
 printf "<text x=\"26\" y=\"%d\" font-size=\"13\" text-anchor=\"middle\" transform=\"rotate(-90 26 %d)\">genus g of the covering curve over F_2 (log scale)</text>\n",(Y0+Y1)/2,(Y0+Y1)/2;
 printf "<line x1=\"%.1f\" y1=\"%.1f\" x2=\"%.1f\" y2=\"%.1f\" stroke=\"#888\" stroke-dasharray=\"5 4\" stroke-width=\"1.5\"/>\n",px(110),py(110),px(600),py(600);
 path=sprintf("M %.1f %.1f",px(n[1]),py(gm[1])); for(i=2;i<=k;i++) path=path sprintf(" L %.1f %.1f",px(n[i]),py(gm[i]));
 printf "<path d=\"%s L %.1f %d L %.1f %d Z\" fill=\"#2a9d8f\" fill-opacity=\"0.12\" stroke=\"none\"/>\n",path,px(n[k]),Y0,px(n[1]),Y0;
 printf "<path d=\"%s\" fill=\"none\" stroke=\"#2a9d8f\" stroke-width=\"2.5\"/>\n",path;
 for(i=1;i<=k;i++){ printf "<circle cx=\"%.1f\" cy=\"%.1f\" r=\"4\" fill=\"#2a9d8f\"/>\n",px(n[i]),py(gm[i]);
   printf "<rect x=\"%.1f\" y=\"%.1f\" width=\"9\" height=\"9\" fill=\"#e76f51\"/>\n",px(n[i])-4.5,py(kr[i])-4.5;
   printf "<path d=\"M %.1f %.1f l 6 10 l -12 0 Z\" fill=\"#264653\"/>\n",px(n[i]),py(et[i])-6;
   printf "<text x=\"%.1f\" y=\"%d\" font-size=\"11\" text-anchor=\"middle\" fill=\"#333\">%s</text>\n",px(n[i]),Y0-8-(i%2==1?0:14),nm[i] }
 lx=110; ly=548;
 printf "<rect x=\"%d\" y=\"%d\" width=\"760\" height=\"62\" fill=\"#fff\" stroke=\"#bbb\"/>\n",lx-10,ly-18;
 printf "<line x1=\"%d\" y1=\"%d\" x2=\"%d\" y2=\"%d\" stroke=\"#2a9d8f\" stroke-width=\"2.5\"/><text x=\"%d\" y=\"%d\" font-size=\"12\">budget g_max(n); shaded region beats rho</text>\n",lx,ly,lx+24,ly,lx+32,ly+4;
 printf "<path d=\"M %d %d l 6 10 l -12 0 Z\" fill=\"#264653\"/><text x=\"%d\" y=\"%d\" font-size=\"12\">L8 etale route: 1 + n(g0 - 1), optimistic g0</text>\n",lx+12,ly+14,lx+32,ly+24;
 printf "<rect x=\"%d\" y=\"%d\" width=\"9\" height=\"9\" fill=\"#e76f51\"/><text x=\"%d\" y=\"%d\" font-size=\"12\">L7 ramified Klein family: 1 + d(n - 1)/2</text>\n",lx+388,ly-8,lx+412,ly;
 printf "<line x1=\"%d\" y1=\"%d\" x2=\"%d\" y2=\"%d\" stroke=\"#888\" stroke-dasharray=\"5 4\" stroke-width=\"1.5\"/><text x=\"%d\" y=\"%d\" font-size=\"12\">g = n floor (realised only at n = 3)</text>\n",lx+380,ly+20,lx+404,ly+20,lx+412,ly+24;
 printf "<text x=\"%d\" y=\"%d\" font-size=\"11\" fill=\"#555\">d = ord_n(2). GHS is degenerate on Koblitz curves (magic number 1). L6 modular route ~ n^3: off scale.</text>\n",X0+8,Y1+14;
 print "</svg>"}
