\\ g2_semaev.gp -- PC5 (Semaev solvers vs group-law tables), wall time of D_2, T_2, P (class S and class F), PROJ, D_4, X_4, and NC3 (timer null).
\\ Cells (p,N), curve c=1, TS=10 targets, 7 interleaved repeats, arm order permuted by setrand(5100+rep).
default(parisizemax,2000000000);
read("g2_lib.gp");
TS=10; NREP=7; C1=1; TQ99=3.707;   \\ two-sided 99% Student t quantile, 6 d.o.f.
CELLS=[[101,12],[101,16],[211,16],[211,24],[401,16]];
XCELLMAXN=16;      \\ X_4 timed only where N<=16
if(g2dev(), CELLS=[[101,12]]; NREP=2);
RIN=["D2",500,"D2c",500,"T2",500,"PROJ",200,"PS",200,"PF",200,"D4",10,"X4",1];
ARMS=["D2","D2c","T2","PROJ","PS","PF","D4","X4"];
inner(arm) = {for(i=1,#RIN\2, if(RIN[2*i-1]==arm, return(RIN[2*i]))); error("arm")}
fs=Str(OUTDIR,"/semaev_runs.csv"); system(Str("rm -f ",fs));
write(fs,"p,N,rep,arm,ms_total,inner,TS,load1,nproc");
fc=Str(OUTDIR,"/semaev_pc5.csv"); system(Str("rm -f ",fc));
write(fc,"p,N,NE,NT,TS,D2_set_mismatch,T2_set_mismatch,X4_set_mismatch,D4_set_mismatch,PF_set_mismatch,PS_set_mismatch,proj_pt_mismatch,D2_rel,T2_rel,X4_rel,D4_rel");
fm=Str(OUTDIR,"/semaev_summary.csv"); system(Str("rm -f ",fm));
write(fm,"p,N,NE,NT,arm,n_reps,median_ms_per_target,min_ms_per_target,solves_per_target,median_us_per_solve,rel_per_target");
fr=Str(OUTDIR,"/semaev_ratios.csv"); system(Str("rm -f ",fr));
write(fr,"p,N,ratio,n_reps,median,min,max,mean,sd,tlo99,thi99");
loadavg1() = {my(s=externstr("cat /proc/loadavg")); eval(strsplit(s[1]," ")[1])}
nproc_() = eval(externstr("nproc")[1])
median_(v) = {my(s=vecsort(v), n=#s); if(n%2==1, s[(n+1)/2], (s[n/2]+s[n/2+1])/2)}
{
foreach(CELLS,cl,
  my(p=cl[1], N=cl[2], cr=curve_row(p,C1));
  ctx(p,cr[3],cr[4]); proj_init(); semaev_init();
  my(tg=draw_targets(7000+10*p+C1,TNUM));
  my(Av=vectorsmall(TS,i,tg[1][i]), Bv=vectorsmall(TS,i,tg[2][i]), B2=vectorsmall(TS,i,tg[3][i]));
  my(S=cellsub(N,4), NE=S[1], NT=S[2]);
  my(Rt=cellcount(S,Av,Bv), Dsets=Rt[3], Tsets=Rt[4]);
  my(rx=cellrun(N,4,Av,Bv,B2), hits=rx[17]);
  \\ inputs
  my(R2f=vector(TS,i,target_pt(Av[i],Bv[i])), R2s=vector(TS,i,ellmul(CE2,CGE2,Av[i])));
  my(xA=vector(TS,i,ffint(R2s[i][1])), XB=vector(TS,i,lift(ellmul(CET,CGT,Bv[i])[1])), XR=vector(TS,i,R2f[i][1]));
  \\ ---- PC5 correctness
  my(mD=0, mT=0, mX=0, m4=0, mPF=0, mPS=0, mproj=0, relD=0, relT=0, relX=0, rel4=0);
  my(Dm=vector(TS,i,List()), Tm=vector(TS,i,List()), Xm=vector(TS,i,List()), D4m=vector(TS,i,List()));
  for(ti=1,TS,
    for(i=1,#Dsets[ti], my(c=Dsets[ti][i]); if(c%8==2, listput(Dm[ti],c\8), if(c%8==4, listput(D4m[ti],c\8))));
    for(i=1,#Tsets[ti], my(c=Tsets[ti][i]); if(c%8==2, listput(Tm[ti],c\8))));
  for(i=1,#hits, listput(Xm[hits[i][1]],hits[i][2]));
  for(ti=1,TS,
    my(s1=semD2(xA[ti],N), s2=semT2(XB[ti],N), s4=semD4(xA[ti],N), s5=semX4(XR[ti],N));
    if(s1!=Vec(Set(Vec(Dm[ti]))), mD++); if(s2!=Vec(Set(Vec(Tm[ti]))), mT++);
    if(s4!=Vec(Set(Vec(D4m[ti]))), m4++); if(s5!=Vec(Set(Vec(Xm[ti]))), mX++);
    relD+=#s1; relT+=#s2; rel4+=#s4; relX+=#s5;
    \\ P arms through the F_{p^2} projection
    my(pp=projpts(R2f[ti]), xa=ffint(pp[1][1]), xb=(Cd*ffint(pp[2][1]))%Cp);
    if(xa!=xA[ti] || xb!=XB[ti], mproj++);
    if(semD2(xa,N)!=s1 || semT2(xb,N)!=s2, mPF++);
    my(ps=projpts(R2s[ti]), xs=ffint(ps[1][1]));
    if(ps[2]!=[0] || semD2(xs,N)!=s1, mPS++));
  print("PC5 p=",p," N=",N," NE=",NE," NT=",NT," TS=",TS," set mismatches D2/T2/X4/D4=",mD,"/",mT,"/",mX,"/",m4," P-F/P-S/proj-point mismatches=",mPF,"/",mPS,"/",mproj," relations D2/T2/X4/D4=",relD,"/",relT,"/",relX,"/",rel4);
  write(fc,p,",",N,",",NE,",",NT,",",TS,",",mD,",",mT,",",mX,",",m4,",",mPF,",",mPS,",",mproj,",",relD,",",relT,",",relX,",",rel4);
  \\ ---- timing
  my(arms=if(N<=XCELLMAXN, ARMS, vecextract(ARMS,"1..7")), na=#arms, times=matrix(NREP,na)); \\ ms per run
  my(skip=vector(na));
  for(rep=1,NREP,
    setrand(5100+rep);
    my(key=vector(na,i,random(10^9)), ord=vecsort(key,,1));
    my(l1=loadavg1(), np=nproc_());
    for(oi=1,na,
      my(ai=ord[oi], arm=arms[ai], R=inner(arm), t0, t1, acc=0);
      t0=getabstime();
      for(r=1,R,
        for(ti=1,TS,
          if(arm=="D2" || arm=="D2c", semD2(xA[ti],N),
          if(arm=="T2", semT2(XB[ti],N),
          if(arm=="PROJ", my(pp=projpts(R2f[ti])); acc+=ffint(pp[1][1])+ffint(pp[2][1]),
          if(arm=="PS", my(ps=projpts(R2s[ti])); semD2(ffint(ps[1][1]),N),
          if(arm=="PF", my(pp=projpts(R2f[ti])); semD2(ffint(pp[1][1]),N); semT2((Cd*ffint(pp[2][1]))%Cp,N),
          if(arm=="D4", semD4(xA[ti],N),
          if(arm=="X4", semX4(XR[ti],N))))))))));
      t1=getabstime();
      times[rep,ai]=t1-t0;
      write(fs,p,",",N,",",rep,",",arm,",",t1-t0,",",R,",",TS,",",l1,",",np)));
  \\ summary per arm
  my(solves=[NE,NE,NT,0,NE,NE+NT,binomial(NE,3),binomial(N,3)], relpt=[relD/TS,relD/TS,relT/TS,0,relD/TS,(relD+relT)/TS,rel4/TS,relX/TS]);
  for(ai=1,na,
    my(R=inner(arms[ai]), v=vector(NREP,rep,times[rep,ai]/(R*TS)), med=median_(v), mn=vecmin(v));
    write(fm,p,",",N,",",NE,",",NT,",",arms[ai],",",NREP,",",fmt(med),",",fmt(mn),",",solves[ai],",",if(solves[ai]>0,fmt(1000*med/solves[ai]),-1),",",fmt(relpt[ai]));
    print("WALL p=",p," N=",N," arm=",arms[ai]," median ms/target=",fmt(med)," min=",fmt(mn)," solves/target=",solves[ai]," us/solve=",if(solves[ai]>0,fmt(1000*med/solves[ai]),-1)));
  \\ ratios per repeat
  my(idx=Map(), ratios=List());
  for(ai=1,na, mapput(idx,arms[ai],ai));
  my(pairs=[["PS","D2"],["PF","D2"],["PROJ","D2"],["D2","D2c"],["D4","D2"],["T2","D2"]]);
  if(N<=XCELLMAXN, pairs=concat(pairs,[["PF","X4"],["X4","D4"],["X4","D2"]]));
  foreach(pairs,pr,
    my(a=mapget(idx,pr[1]), b=mapget(idx,pr[2]), Ra=inner(pr[1]), Rb=inner(pr[2]));
    if(vecmin(vector(NREP,rep,times[rep,b]))<=0 || vecmin(vector(NREP,rep,times[rep,a]))<=0, write(fr,p,",",N,",",pr[1],"/",pr[2],",",NREP,",NA,NA,NA,NA,NA,NA,NA"); print("RATIO p=",p," N=",N," ",pr[1],"/",pr[2]," NA (a timed block measured 0 ms)"); next);
    my(v=vector(NREP,rep,(times[rep,a]/Ra)/(times[rep,b]/Rb)));
    my(mu=vecsum(v)/NREP, sd=sqrt(sum(i=1,NREP,(v[i]-mu)^2)/(NREP-1)), hw=TQ99*sd/sqrt(NREP));
    write(fr,p,",",N,",",pr[1],"/",pr[2],",",NREP,",",fmt(median_(v)),",",fmt(vecmin(v)),",",fmt(vecmax(v)),",",fmt(mu),",",fmt(sd),",",fmt(mu-hw),",",fmt(mu+hw));
    print("RATIO p=",p," N=",N," ",pr[1],"/",pr[2]," median=",fmt(median_(v))," mean=",fmt(mu)," 99% t-interval=[",fmt(mu-hw),",",fmt(mu+hw),"] min=",fmt(vecmin(v))," max=",fmt(vecmax(v))));
);
}
quit;
