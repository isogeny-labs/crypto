\\ Data loading shared by summarize.gp and visit.gp. Reads only inputs/*.tsv and ../inputs/historical.tsv.
rdt(f)={my(v=readstr(f),R=List());for(i=1,#v,if(#v[i]>0&&Vec(v[i])[1]!="#",listput(R,strsplit(v[i],"\t"))));Vec(R)};
STREAMS=["prime192v2","P-224","prime239v3","P-256","P-384","P-521"];
sid(s)={for(i=1,#STREAMS,if(STREAMS[i]==s,return(i)));error("stream ",s)};
hmap(l)={my(k=strsplit(l,"-")[1]);if(k=="p192",1,k=="p224",2,k=="p239",3,k=="p256",4,k=="p384",5,6)};
H=rdt("../inputs/historical.tsv");
\\ SL[s] = last published index = stream prefix length (1-based); PUB[s] = sorted published indices
SL=vector(6);PUBL=vector(6,i,List());
{for(i=1,#H,my(s=hmap(H[i][1]),ix=eval(H[i][5]));
  if(H[i][3]=="published",listput(PUBL[s],ix));SL[s]=max(SL[s],ix))};
PUB=vector(6,i,vecsort(Vec(PUBL[i])));
A=rdt("inputs/acc69.tsv");      \\ stream index label branches order_bits
Bn=rdt("inputs/near_branches.tsv"); \\ stream ts index side p order cof
\\ POS: Map [s,index] -> [s,index,label69|near_extra, branch list, pub, n_bits, p_bits, cofactor]
POS=Map();
{for(i=1,#Bn,my(r=Bn[i],s=sid(r[1]),ix=eval(r[3]),p=eval(r[5]),u=eval(r[6]),h=eval(r[7]),n=u/h,key=[s,ix],cur);
  cur=if(mapisdefined(POS,key),mapget(POS,key),[s,ix,"near_extra",List(),0,0,#binary(p),h]);
  listput(cur[4],r[4]);
  if(r[4]=="canonical"||cur[6]==0,cur[6]=#binary(n));
  mapput(POS,key,cur))};
{for(i=1,#A,my(r=A[i],s=sid(r[1]),ix=eval(r[2]),key=[s,ix],cur=mapget(POS,key));cur[3]=r[3];mapput(POS,key,cur))};
{for(s=1,6,for(j=1,#PUB[s],my(key=[s,PUB[s][j]],cur=mapget(POS,key));cur[5]=1;mapput(POS,key,cur)))};
PM=Mat(POS);NP=matsize(PM)[1];
PV=vector(NP,k,PM[k,2]);
\\ eligibility sets: e=1 retro21 (published+retrospective omissions), 2 canonical-root prime-order,
\\ 3 width-fit prime-order (either root), 4 all 69 prime-order, 5 all 99 near-prime
inE(e,v)={my(lab=v[3],br=v[4],hasc=0);for(i=1,#br,if(br[i]=="canonical",hasc=1));
 if(e==1,return(lab=="published"||lab=="retrospective_omission"));
 if(e==2,return(hasc&&lab!="near_extra"&&lab!="opposite_root_only"));
 if(e==3,return(lab!="near_extra"&&lab!="order_too_wide_only"));
 if(e==4,return(lab!="near_extra"));
 1};
ENAMES=["E1_retro21","E2_canonroot_prime","E3_fit_anyroot","E4_prime69","E5_near99"];
ELIG=vector(5,e,vector(6,s,List()));
{for(k=1,NP,my(v=PV[k]);for(e=1,5,if(inE(e,v),listput(ELIG[e][v[1]],v[2]))))};
{for(e=1,5,for(s=1,6,ELIG[e][s]=vecsort(Vec(ELIG[e][s]))))};
isPub(s,ix)=vecsearch(PUB[s],ix)>0;
