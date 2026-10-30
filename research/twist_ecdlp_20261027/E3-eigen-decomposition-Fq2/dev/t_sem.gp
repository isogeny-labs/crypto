default(parisizemax,2000000000);
read("e3_lib.gp");
ctx(61,54,41);
semaev_init();
tg=draw_targets(5,5);
N=12;
r=cellrun(N,4,tg[1],tg[2],tg[3]);
{for(ti=1,5,
  A=tg[1][ti]; B=tg[2][ti];
  R=ellmul(CE,CGE,A); xA=lift(R[1]);
  t0=getabstime(); s=semD2(xA,N); t1=getabstime();
  print("D2 target ",ti," semaev=",#s," table=",r[3][ti,2]," ms=",t1-t0);
  Rp=target_pt(A,B); XR=Rp[1];
  t0=getabstime(); s=semX4(XR,N); t1=getabstime();
  print("X4 target ",ti," semaev=",#s," table=",sum(j=1,3,r[5][ti,j])," ms=",t1-t0);
);}
quit;
