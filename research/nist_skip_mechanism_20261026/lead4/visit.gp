\\ visit.gp: evaluates the family frozen in PROTOCOL.md on the committed tables.
\\ Run only as:  ../../../scripts/checkpoint.sh gate && gp -q -s 300M visit.gp
read("lib.gp");
read("visitlib.gp");
NREP=1000;
OUT="out/results.txt";
OUTM="out/strict_models.tsv";
OUTN="out/null.tsv";
{my(ex=1);iferr(readstr(OUT),err,ex=0);if(ex,error("refusing to overwrite ",OUT))}
w(s)=write(OUT,s);
w("# visit.gp results; PROTOCOL.md frozen before this file was written");
w(Str("inputs sha256 recorded by shell at run time (out/inputs.sha256)"));
\\ ---- per cohort, per stream data
EL=vector(5,e,vector(6,s,ELIG[e][s]));
YM=vector(5,e,vector(6,s,my(el=EL[e][s],m=0);for(j=1,#el,if(isPub(s,el[j]),m+=2^(j-1)));m));
KEYV(e,s,kap)={my(el=EL[e][s],v=vector(#el));for(j=1,#el,my(r=mapget(POS,[s,el[j]]),br=r[4],hc=0,ho=0,k1,k2);
  for(i=1,#br,if(br[i]=="canonical",hc=1,ho=1));
  k1=sign(r[6]-r[7])+1;k2=if(hc&&ho,2,hc,0,1);
  v[j]=if(kap==1,k1,kap==2,k2,3*k1+k2));v};
KV=vector(5,e,vector(6,s,vector(3,kap,KEYV(e,s,kap))));
\\ ---- strict model list: 1080 T (TM x stop) then 15 Q
NT=#TM*2;NM=NT+#QM;
mname(mi)=if(mi<=NT,my(t=TM[(mi+1)\2],st=(mi%2==1));Str("T r=",t[1]," phi=",t[2]," a=",t[3]," stop=",if(st,"count","none")),my(q=QM[mi-NT]);Str("Q K",q[1]," q=",q[2]));
strictmask(e,s,mi,Ym)={if(mi<=NT,my(t=TM[(mi+1)\2],st=(mi%2==1));simT(EL[e][s],Ym,t[1],t[2],t[3],st,0)[1],my(q=QM[mi-NT]);simQ(EL[e][s],KV[e][s][q[1]],q[2]))};
SPM=vector(5,e,vector(NM,mi,vector(6,s,strictmask(e,s,mi,YM[e][s]))));
S1(e,Yv)={my(best=10^6);for(mi=1,NM,my(d=0,P=SPM[e][mi]);for(s=1,6,d+=hammingweight(bitxor(P[s],Yv[s])));if(d<best,best=d));best};
\\ ---- permissive
S2(e,Yv)={my(best=10^6);for(mi=1,#TM,my(t=TM[mi],hid=0,ok=1);
  for(s=1,6,my(R=simT(EL[e][s],Yv[s],t[1],t[2],t[3],0,1));if(R[3]<hammingweight(Yv[s]),ok=0;break);hid+=R[2]);
  if(ok&&hid<best,best=hid));best};
\\ ---- real data
w("");w("## strict results on real labels (measured)");
w("cohort\t|E|\tmodels\texact(D=0)\tTP10\tminD\tminFP_given_TP10\tV_at_minFP\tV/|E|");
OBS1=vector(5);OBS2=vector(5);
{for(e=1,5,my(nE=sum(s=1,6,#EL[e][s]),nex=0,ntp=0,bd=10^6,bfp=10^6,bv=0,bestm="");
  for(mi=1,NM,my(P=SPM[e][mi],d=0,tp=0,v=0);
    for(s=1,6,d+=hammingweight(bitxor(P[s],YM[e][s]));tp+=hammingweight(bitand(P[s],YM[e][s]));v+=hammingweight(P[s]));
    write(OUTM,Str(ENAMES[e],"\t",mname(mi),"\t",d,"\t",tp,"\t",v-tp,"\t",v));
    if(d==0,nex++);
    if(tp==10,ntp++;if(v-10<bfp,bfp=v-10;bv=v;bestm=mname(mi)));
    if(d<bd,bd=d));
  OBS1[e]=bd;
  w(Str(ENAMES[e],"\t",nE,"\t",NM,"\t",nex,"\t",ntp,"\t",bd,"\t",bfp,"\t",bv,"\t",if(bv,bv/nE+0.,"NA"),"\tmodel_at_minFP: ",bestm)))};
w("");w("## permissive results on real labels (measured)");
w("cohort\t|E|\tvalid_models\tX0(baseline hidden rejections)\tS2*(min hidden rejections)");
{for(e=1,5,my(nE=sum(s=1,6,#EL[e][s]),nv=0,best=10^6,bm="");
  for(mi=1,#TM,my(t=TM[mi],hid=0,ok=1);
    for(s=1,6,my(R=simT(EL[e][s],YM[e][s],t[1],t[2],t[3],0,1));if(R[3]<hammingweight(YM[e][s]),ok=0;break);hid+=R[2]);
    if(ok,nv++;if(hid<best,best=hid;bm=Str("r=",t[1]," phi=",t[2]," a=",t[3]))));
  OBS2[e]=best;
  w(Str(ENAMES[e],"\t",nE,"\t",nv,"\t",nE-10,"\t",best,"\tmodel: ",bm)))};
\\ ---- null calibration
setrand(20261026);
N1=matrix(5,NREP);N2=matrix(5,NREP);
{for(rep=1,NREP,for(e=1,5,my(Yv=vector(6,s,rsub(#EL[e][s],#PUB[s])));N1[e,rep]=S1(e,Yv);N2[e,rep]=S2(e,Yv)))};
w("");w(Str("## calibration: N=",NREP," count-preserving random labellings (measured)"));
w("cohort\tobsS1\tP(S1<=obs)\tnull_P(S1=0)\tnull_median_S1\tobsS2\tP(S2<=obs)\tnull_median_S2");
{for(e=1,5,my(c1=1,c2=1,z=0,a1=vector(NREP,i,N1[e,i]),a2=vector(NREP,i,N2[e,i]));
  for(i=1,NREP,if(N1[e,i]<=OBS1[e],c1++);if(N2[e,i]<=OBS2[e],c2++);if(N1[e,i]==0,z++);write(OUTN,Str(ENAMES[e],"\t",i,"\t",N1[e,i],"\t",N2[e,i])));
  w(Str(ENAMES[e],"\t",OBS1[e],"\t",c1/(NREP+1.),"\t",z/NREP+0.,"\t",vecsort(a1)[NREP\2],"\t",OBS2[e],"\t",c2/(NREP+1.),"\t",vecsort(a2)[NREP\2])))};
S1r(e,Yv)={my(best=10^6);for(mi=1,NM,my(dd=0);for(s=1,6,dd+=hammingweight(bitxor(strictmask(e,s,mi,Yv[s]),Yv[s])));if(dd<best,best=dd));best};
\\ ---- PC1 positive control (E4): planted = strict prediction of T r=1 phi=0 a=3 stop=count
{my(e=4,Pp=vector(6,s,simT(EL[e][s],YM[e][s],1,0,3,1,0)[1]),kp=vector(6,s,hammingweight(Pp[s])),d=0,c=1);
  d=S1r(e,Pp);
  for(rep=1,NREP,my(Yv=vector(6,s,rsub(#EL[e][s],kp[s])));if(S1r(e,Yv)<=d,c++));
  w("");w(Str("## PC1 positive control (E4 planted; counts ",kp,")"));
  w(Str("planted S1*=",d," P(S1<=planted)=",c/(NREP+1.)," (need S1*=0 and P<=0.01)"))};
w("done");
