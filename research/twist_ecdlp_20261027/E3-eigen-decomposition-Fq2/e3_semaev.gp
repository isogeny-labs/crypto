\\ e3_semaev.gp -- PC2 (Semaev solver vs group-law table, D_2 and X_4) and wall time, cells (p,N) = (101,12),(101,16),(211,16), curve c=1, T_S=10
default(parisizemax,2000000000);
read("e3_grid.gp");
read("e3_lib.gp");
TS=10; REPD=50;     \\ REPD: D_2 timing repeats (getabstime has 1 ms granularity)
CELLS=[[101,12],[101,16],[211,16]]; C1=1;
if(genv("E3DEV","0")=="1", CELLS=[[61,12]]);
fn=Str(OUTDIR,"/semaev.csv"); system(Str("rm -f ",fn));
write(fn,"p,N,ti,NE,Dsem,Dtab,Dsets_equal,Xsem,Xtab,Xsets_equal,D_ms_per_solve_batch,X_ms,Dsolves,Xsolves");
\\ summary rows go to out/semaev_summary.csv
fs=Str(OUTDIR,"/semaev_summary.csv"); system(Str("rm -f ",fs));
write(fs,"p,N,NE,NT,TS,D_mismatch_targets,X_mismatch_targets,D_set_mismatch,X_set_mismatch,D_total_ms,X_total_ms,D_solves,X_solves,D_us_per_solve,X_us_per_solve,ratio_X_over_D_per_solve,D_us_per_rel,X_us_per_rel,D_rel,X_rel");
{
foreach(CELLS,cl,
  my(p=cl[1], N=cl[2], cr=curve_row(p,C1));
  ctx(p,cr[3],cr[4]); semaev_init();
  my(tg=draw_targets(7000+10*p+C1,TNUM));
  my(Av=vectorsmall(TS,i,tg[1][i]), Bv=vectorsmall(TS,i,tg[2][i]), B2v=vectorsmall(TS,i,tg[3][i]));
  my(r=cellrun(N,4,Av,Bv,B2v), NE=r[1], NT=r[2]);
  \\ table x-set masks per target
  my(Dmasks=vector(TS,i,List()), Xmasks=vector(TS,i,List()));
  my(Dl=r[18], Hl=r[17]);
  for(i=1,#Dl, if(Dl[i][2]==2, listput(Dmasks[Dl[i][1]], Dl[i][3])));
  for(i=1,#Hl, listput(Xmasks[Hl[i][1]], Hl[i][2]));
  my(dmis=0, xmis=0, dsm=0, xsm=0, dms=0, xms=0, drel=0, xrel=0);
  for(ti=1,TS,
    my(A=Av[ti], B=Bv[ti]);
    my(R=ellmul(CE,CGE,A), xA=lift(R[1]));
    my(Rp=target_pt(A,B), XR=Rp[1]);
    my(t0=getabstime(), s, s2);
    s=semD2(xA,N);
    for(rep=2,REPD, semD2(xA,N));
    my(t1=getabstime());
    s2=semX4(XR,N);
    my(t2=getabstime());
    my(Dtab=r[3][ti,2], Xtab=sum(j=1,3,r[5][ti,j]));
    my(dm=Set(Vec(Dmasks[ti])), xm=Set(Vec(Xmasks[ti])));
    my(deq=(Set(s)==dm), xeq=(Set(s2)==xm));
    if(#s!=Dtab, dmis++); if(#s2!=Xtab, xmis++);
    if(!deq, dsm++); if(!xeq, xsm++);
    dms+=t1-t0; xms+=t2-t1; drel+=#s; xrel+=#s2;
    write(fn,p,",",N,",",ti,",",NE,",",#s,",",Dtab,",",deq,",",#s2,",",Xtab,",",xeq,",",t1-t0,",",t2-t1,",",NE*REPD,",",binomial(N,3)));
  my(Dsolves=NE*TS*REPD, Xsolves=binomial(N,3)*TS);
  my(Dus=1000.0*dms/Dsolves, Xus=1000.0*xms/Xsolves);
  \\ per relation: D time is per REPD repeats, so one run = dms/REPD
  my(Drel_us=if(drel>0,1000.0*(dms/REPD)/drel,-1), Xrel_us=if(xrel>0,1000.0*xms/xrel,-1));
  print("SEMAEV p=",p," N=",N," NE=",NE," NT=",NT," TS=",TS," D_2 count mismatches=",dmis,"/",TS," X_4 count mismatches=",xmis,"/",TS," D set mismatches=",dsm," X set mismatches=",xsm," D ms(total, ",REPD," reps)=",dms," X ms(total)=",xms," us/solve D=",fmt(Dus)," X=",fmt(Xus)," ratio=",if(Dus>0,fmt(Xus/Dus),"inf")," relations D=",drel," X=",xrel);
  write(fs,p,",",N,",",NE,",",NT,",",TS,",",dmis,",",xmis,",",dsm,",",xsm,",",dms,",",xms,",",Dsolves,",",Xsolves,",",fmt(Dus),",",fmt(Xus),",",if(Dus>0,fmt(Xus/Dus),-1),",",fmt(Drel_us),",",fmt(Xrel_us),",",drel,",",xrel));
}
quit;
