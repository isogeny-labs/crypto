\\ e3_tables.gp -- table arms D_m, X_2m, X_m, NC1, NC2, NC3, P5 for one curve. Env: P, C, FAMILY (A|B), E3OUT.
default(parisizemax,2000000000);
read("e3_grid.gp");
read("e3_lib.gp");
P=eval(getenv("P")); C=eval(getenv("C")); FAM=genv("FAMILY","A");
NPERM=2000;
cr=curve_row(P,C);
ctx(P,cr[3],cr[4]);
if(Cl!=cr[5] || Clp!=cr[6], error("curve table mismatch"));
m=famM(FAM); m2=2*m; NL=famN(FAM); T=TNUM;
print("tables p=",P," c=",C," fam=",FAM," a=",cr[3]," b=",cr[4]," #E=",Cl," #E'=",Clp," d=",Cd," m=",m," m2=",m2," T=",T);
tg=draw_targets(7000+10*P+C,T);
Av=tg[1]; Bv=tg[2]; B2v=tg[3];
\\ two-sample permutation test on the difference of means, NPERM label shuffles (seed 9002), returns two-sided p-value (add-one)
permtest(x,y) = {
  my(n1=#x, n2=#y, pool=concat(x,y), n=#pool, s=sum(i=1,n,pool[i]), obs, cnt=0, tot);
  tot=s;
  obs=abs(sum(i=1,n1,x[i])/n1 - (tot-sum(i=1,n1,x[i]))/n2);
  if(tot==0 || vecmax(pool)==vecmin(pool), return(1.0));
  for(r=1,NPERM,
    my(pp=pool, s1=0);
    for(i=1,n1, my(k=i+random(n-i+1), tmp=pp[i]); pp[i]=pp[k]; pp[k]=tmp; s1+=pp[i]);
    my(st=abs(s1/n1-(tot-s1)/n2));
    if(st>=obs-1e-12, cnt++));
  (cnt+1)/(NPERM+1)
}
fn=Str(OUTDIR,"/tables_p",P,"_c",C,"_",FAM,".csv"); system(Str("rm -f ",fn));
write(fn,"p,c,m,N,T,l,lp,NE,NT,D1,D2,D3,D4,D5,D6,X1,X2,X3,X4,X5,Xm1,Xm2,nh,mult,p5,nc1,fm,nc3m,nc3z,WDm_code,WDm_form,WX_code,WX_form,WXm_code,WXm_form,nc2_p,nc2_meanA,nc2_meanB,Xalt1,Xalt2,Xalt3,Xalt4,Xalt5,ms");
setrand(9002);
{
foreach(NL,N,
  my(t0=getabstime());
  my(r=cellrun(N,m2,Av,Bv,B2v));
  my(rc=cellrun(N,m,Av,Bv,B2v));
  my(NE=r[1], NT=r[2]);
  my(Dtot=vector(6,j, if(j<=m2, sum(ti=1,T,r[3][ti,j]),0)));
  my(Xtot=vector(5,j, if(j<=m2-1, sum(ti=1,T,r[5][ti,j]),0)));
  my(Xalt=vector(5,j, if(j<=m2-1, sum(ti=1,T,r[6][ti,j]),0)));
  my(Xmc=vector(2,j, if(j<=m-1, sum(ti=1,T,rc[5][ti,j]),0)));
  my(WDm=workloop(NE,m-1), WDmf=binomial(NE,m-1)*2^(m-1), WXc=r[14], WXf=binomial(N,m2-1)*2^(m2-1), WXm=rc[14], WXmf=binomial(N,m-1)*2^(m-1));
  my(Xmain=vector(T,ti,sum(j=1,m2-1,r[5][ti,j])), Xaltv=vector(T,ti,sum(j=1,m2-1,r[6][ti,j])));
  my(pv=permtest(Xmain,Xaltv));
  write(fn,P,",",C,",",m,",",N,",",T,",",Cl,",",Clp,",",NE,",",NT,",",Dtot[1],",",Dtot[2],",",Dtot[3],",",Dtot[4],",",Dtot[5],",",Dtot[6],",",
    Xtot[1],",",Xtot[2],",",Xtot[3],",",Xtot[4],",",Xtot[5],",",Xmc[1],",",Xmc[2],",",r[9],",",r[8],",",r[10],",",r[11],",",r[12],",",r[15],",",r[16],",",
    WDm,",",WDmf,",",WXc,",",WXf,",",WXm,",",WXmf,",",fmt(pv),",",fmt(vecsum(Xmain)/T),",",fmt(vecsum(Xaltv)/T),",",Xalt[1],",",Xalt[2],",",Xalt[3],",",Xalt[4],",",Xalt[5],",",getabstime()-t0);
  \\ per-target file
  my(pf=Str(OUTDIR,"/pt_p",P,"_c",C,"_",FAM,"_N",N,".csv")); system(Str("rm -f ",pf));
  write(pf,"ti,A,B,Dm,Xgen,Xex,Xall,Xmc,Xalt");
  for(ti=1,T, write(pf,ti,",",Av[ti],",",Bv[ti],",",r[3][ti,m],",",sum(j=1,m,r[5][ti,j]),",",r[5][ti,m],",",Xmain[ti],",",sum(j=1,m-1,rc[5][ti,j]),",",Xaltv[ti]));
  print("cell N=",N," NE=",NE," NT=",NT," Dm_tot=",Dtot[m]," Xgen_tot=",sum(j=1,m,Xtot[j])," Xex_tot=",Xtot[m]," Xall=",vecsum(Xtot)," Xm_ctrl=",vecsum(Xmc)," nh=",r[9]," mult=",r[8]," p5=",r[10]," nc1=",r[11]," fm=",r[12]," nc3(B!=0)=",r[15]," nc3(B=0)=",r[16]," WDm=",WDm,"/",WDmf," WX=",WXc,"/",WXf," nc2_p=",fmt(pv)," ms=",getabstime()-t0);
);
}
quit;
