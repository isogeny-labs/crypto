# Renders hits-by-n.csv (recorded values) as grouped bars: characters tested and characters with a T_n hit.
BEGIN{FS=","; W=900; H=520; X0=80; X1=870; Y0=430; Y1=70; ymax=5000}
NR>1{ key=$2; if(!(key in seen)){seen[key]=1; ns[++k]=key+0; for(a=k;a>1 && ns[a-1]>ns[a];a--){tmp=ns[a];ns[a]=ns[a-1];ns[a-1]=tmp}} tested[$1","$2]=$3; hit[$1","$2]=$5 }
function py(v){return Y0-v/ymax*(Y0-Y1)}
END{
 printf "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"%d\" height=\"%d\" viewBox=\"0 0 %d %d\" font-family=\"Helvetica, Arial, sans-serif\">\n",W,H,W,H;
 printf "<rect width=\"%d\" height=\"%d\" fill=\"#fff\"/>\n",W,H;
 printf "<text x=\"%d\" y=\"28\" font-size=\"16\" font-weight=\"bold\" text-anchor=\"middle\">Etale Z/n covers of genus-2 and hyperelliptic genus-3 curves over F_2: characters tested and T_n hits</text>\n",W/2;
 printf "<text x=\"%d\" y=\"47\" font-size=\"11.5\" fill=\"#444\" text-anchor=\"middle\">Exhaustive over 768 + 6,144 smooth models. Every group validated (free rank 1, torsion = L_B(1)); 0 Riemann-hypothesis failures. Data: hits-by-n.csv</text>\n",W/2;
 for(v=0;v<=ymax;v+=1000){y=py(v); printf "<line x1=\"%d\" y1=\"%.1f\" x2=\"%d\" y2=\"%.1f\" stroke=\"#e6e6e6\"/><text x=\"%d\" y=\"%.1f\" font-size=\"11\" text-anchor=\"end\">%d</text>\n",X0,y,X1,y,X0-6,y+4,v}
 printf "<line x1=\"%d\" y1=\"%d\" x2=\"%d\" y2=\"%d\" stroke=\"#333\"/><line x1=\"%d\" y1=\"%d\" x2=\"%d\" y2=\"%d\" stroke=\"#333\"/>\n",X0,Y0,X1,Y0,X0,Y0,X0,Y1;
 slot=(X1-X0)/k; bw=slot*0.32;
 for(i=1;i<=k;i++){ n=ns[i]; cx=X0+slot*(i-0.5);
   for(g=2;g<=3;g++){ key=g","n; if(!(key in tested)) continue; x=(g==2)?cx-bw-2:cx+2;
     col=(g==2)?"#a8dadc":"#cdb4db"; dark=(g==2)?"#1d3557":"#6a4c93";
     printf "<rect x=\"%.1f\" y=\"%.1f\" width=\"%.1f\" height=\"%.1f\" fill=\"%s\"/>\n",x,py(tested[key]),bw,Y0-py(tested[key]),col;
     if(hit[key]>0) printf "<rect x=\"%.1f\" y=\"%.1f\" width=\"%.1f\" height=\"%.1f\" fill=\"%s\"/>\n",x,py(hit[key]),bw,Y0-py(hit[key]),dark;
     printf "<text x=\"%.1f\" y=\"%.1f\" font-size=\"10.5\" text-anchor=\"middle\" fill=\"%s\">%d</text>\n",x+bw/2,py(tested[key])-4,(hit[key]>0)?"#c1121f":"#333",hit[key] }
   printf "<text x=\"%.1f\" y=\"%d\" font-size=\"12\" text-anchor=\"middle\">n = %s</text>\n",cx,Y0+18,n }
 printf "<text x=\"20\" y=\"%d\" font-size=\"12\" transform=\"rotate(-90 20 %d)\" text-anchor=\"middle\">characters chi of J_B(F_2)/n tested</text>\n",(Y0+Y1)/2,(Y0+Y1)/2;
 lx=X0+10; ly=H-40;
 printf "<rect x=\"%d\" y=\"%d\" width=\"12\" height=\"12\" fill=\"#a8dadc\"/><text x=\"%d\" y=\"%d\" font-size=\"12\">genus 2: tested</text>\n",lx,ly,lx+18,ly+10;
 printf "<rect x=\"%d\" y=\"%d\" width=\"12\" height=\"12\" fill=\"#1d3557\"/><text x=\"%d\" y=\"%d\" font-size=\"12\">genus 2: with T_n factor</text>\n",lx+150,ly,lx+168,ly+10;
 printf "<rect x=\"%d\" y=\"%d\" width=\"12\" height=\"12\" fill=\"#cdb4db\"/><text x=\"%d\" y=\"%d\" font-size=\"12\">genus 3 (hyperelliptic): tested</text>\n",lx+340,ly,lx+358,ly+10;
 printf "<rect x=\"%d\" y=\"%d\" width=\"12\" height=\"12\" fill=\"#6a4c93\"/><text x=\"%d\" y=\"%d\" font-size=\"12\">genus 3: with T_n factor</text>\n",lx+570,ly,lx+588,ly+10;
 printf "<text x=\"%d\" y=\"%d\" font-size=\"11\" fill=\"#555\">Number above each bar: characters with a T_n factor (red if nonzero). Run %s.</text>\n",lx,ly+30,RUN;
 print "</svg>"}
