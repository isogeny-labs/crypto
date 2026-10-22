# Renders hits-by-n.csv (recorded values) as grouped bars per n: characters tested and characters
# with a T_n factor, for genus 2, hyperelliptic genus 3 and plane-quartic genus 3.
BEGIN{FS=","; W=960; H=540; X0=80; X1=930; Y0=440; Y1=74; ymax=10000
 fam[1]="g2"; fam[2]="g3h"; fam[3]="g3q"
 light["g2"]="#a8dadc"; dark["g2"]="#1d3557"; name["g2"]="genus 2"
 light["g3h"]="#cdb4db"; dark["g3h"]="#6a4c93"; name["g3h"]="genus 3, hyperelliptic"
 light["g3q"]="#ffd6a5"; dark["g3q"]="#e85d04"; name["g3q"]="genus 3, plane quartic"}
NR>1{ key=$2; if(!(key in seen)){seen[key]=1; ns[++k]=key+0; for(a=k;a>1 && ns[a-1]>ns[a];a--){tmp=ns[a];ns[a]=ns[a-1];ns[a-1]=tmp}} tested[$1","$2]=$3; hit[$1","$2]=$5 }
function py(v){return Y0-v/ymax*(Y0-Y1)}
END{
 printf "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"%d\" height=\"%d\" viewBox=\"0 0 %d %d\" font-family=\"Helvetica, Arial, sans-serif\">\n",W,H,W,H;
 printf "<rect width=\"%d\" height=\"%d\" fill=\"#fff\"/>\n",W,H;
 printf "<text x=\"%d\" y=\"28\" font-size=\"16\" font-weight=\"bold\" text-anchor=\"middle\">Etale Z/n covers of every genus-2 and genus-3 curve over F_2: characters tested and T_n hits</text>\n",W/2;
 printf "<text x=\"%d\" y=\"47\" font-size=\"11.5\" fill=\"#444\" text-anchor=\"middle\">768 + 6,144 + 10,920 smooth models. Every group validated; 0 RH, functional-equation, N_6 or exact/float failures. Data: hits-by-n.csv</text>\n",W/2;
 for(v=0;v<=ymax;v+=2000){y=py(v); printf "<line x1=\"%d\" y1=\"%.1f\" x2=\"%d\" y2=\"%.1f\" stroke=\"#e6e6e6\"/><text x=\"%d\" y=\"%.1f\" font-size=\"11\" text-anchor=\"end\">%d</text>\n",X0,y,X1,y,X0-6,y+4,v}
 printf "<line x1=\"%d\" y1=\"%d\" x2=\"%d\" y2=\"%d\" stroke=\"#333\"/><line x1=\"%d\" y1=\"%d\" x2=\"%d\" y2=\"%d\" stroke=\"#333\"/>\n",X0,Y0,X1,Y0,X0,Y0,X0,Y1;
 slot=(X1-X0)/k; bw=slot*0.26;
 for(i=1;i<=k;i++){ n=ns[i]; cx=X0+slot*(i-0.5);
   for(f=1;f<=3;f++){ g=fam[f]; key=g","n; if(!(key in tested)) continue; x=cx+(f-2)*(bw+1)-bw/2;
     printf "<rect x=\"%.1f\" y=\"%.1f\" width=\"%.1f\" height=\"%.1f\" fill=\"%s\"/>\n",x,py(tested[key]),bw,Y0-py(tested[key]),light[g];
     if(hit[key]>0) printf "<rect x=\"%.1f\" y=\"%.1f\" width=\"%.1f\" height=\"%.1f\" fill=\"%s\"/>\n",x,py(hit[key]),bw,Y0-py(hit[key]),dark[g];
     printf "<text x=\"%.1f\" y=\"%.1f\" font-size=\"9.5\" text-anchor=\"middle\" fill=\"%s\">%d</text>\n",x+bw/2,py(tested[key])-4,(hit[key]>0)?"#c1121f":"#333",hit[key] }
   printf "<text x=\"%.1f\" y=\"%d\" font-size=\"12\" text-anchor=\"middle\">%s</text>\n",cx,Y0+18,n }
 printf "<text x=\"%d\" y=\"%d\" font-size=\"12\" text-anchor=\"middle\">prime n dividing #J_B(F_2)</text>\n",(X0+X1)/2,Y0+36;
 printf "<text x=\"20\" y=\"%d\" font-size=\"12\" transform=\"rotate(-90 20 %d)\" text-anchor=\"middle\">characters chi of J_B(F_2)/n tested</text>\n",(Y0+Y1)/2,(Y0+Y1)/2;
 lx=X0; ly=H-42;
 for(f=1;f<=3;f++){ g=fam[f]; ox=lx+(f-1)*290;
   printf "<rect x=\"%d\" y=\"%d\" width=\"12\" height=\"12\" fill=\"%s\"/><rect x=\"%d\" y=\"%d\" width=\"12\" height=\"12\" fill=\"%s\"/><text x=\"%d\" y=\"%d\" font-size=\"12\">%s: tested / with T_n factor</text>\n",ox,ly,light[g],ox+14,ly,dark[g],ox+32,ly+10,name[g] }
 printf "<text x=\"%d\" y=\"%d\" font-size=\"11\" fill=\"#555\">Number above each bar: characters with a T_n factor (red if nonzero). Run %s.</text>\n",lx,ly+30,RUN;
 print "</svg>"}
