\\ g2_pcsynth.gp -- POSITIVE CONTROL run first: the interval code of g2_ana_lib.gp must report rho<1 when an injected arm is truly cheaper.
\\ Synthetic per-target counts (fixed seed), two synthetic "curves"; arm = P-S whose yields are injected (x1 null, x2 cheaper) ; baseline D*.
default(parisizemax,2000000000);
read("g2_ana_lib.gp");
read("g2_lib.gp");
m=2; m2=4; T=200; NEs=[12,14]; NTs=[10,12];
setrand(777);
function_mk(inj) = {
  my(CV=List(), FR=List(), WL=List());
  for(c=1,2,
    my(NE=NEs[c], NT=NTs[c], rows=vector(T), Fs=vector(NF), Wt=vector(NW));
    for(ti=1,T,
      my(f=vector(NF));
      for(j=1,m2, f[j]=random(1+j); f[18+j]=sum(i=1,inj,random(1+j)));   \\ D_j ~ uniform{0..j}; S_j = sum of inj independent draws (mean inj x baseline), independent of D_j
      for(k=1,m2, f[6+k]=random(3));
      f[30]=f[m]*f[7]; f[31]=f[m]*f[6+m];
      rows[ti]=f; Fs+=f);
    for(j=1,6, Wt[j]=T*wformula(NE,j-1); Wt[6+j]=T*wformula(NT,j-1));
    listput(CV,[Fs,Wt]); listput(FR,rows); listput(WL,Wt));
  [Vec(CV),Vec(FR),Vec(WL)]
}
{
fn=Str(OUTDIR,"/pcsynth.csv"); system(Str("rm -f ",fn));
write(fn,"inj,level,rho_point,lo99,hi99,nvalid,expect_true_rho");
allok=1;
foreach([1,2],inj,
  my(D=function_mk(inj), CV=D[1], FR=D[2], WL=D[3]);
  my(Fs=CV[1][1]+CV[2][1], Wt=CV[1][2]+CV[2][2]);
  my(r=rhos(Fs,Wt,m,m2,0), Rc=cboot(CV,m,m2,10000,9002), ic=pint(Rc,8));
  print("PCSYNTH inj=",inj," curve-level: rho_PS_best point=",fmt(r[8])," 99% interval=[",fmt(ic[1]),",",fmt(ic[2]),"] (nvalid ",ic[3],")   expected true rho=",fmt(1.0/inj));
  write(fn,inj,",curve,",fmt(r[8]),",",fmt(ic[1]),",",fmt(ic[2]),",",ic[3],",",fmt(1.0/inj));
  my(Rt=tboot(FR[1],WL[1],m,m2,2000,9001), it=pint(Rt,8), r1=rhos(CV[1][1],WL[1],m,m2,0));
  print("PCSYNTH inj=",inj," target-level (curve 1): point=",fmt(r1[8])," 99% interval=[",fmt(it[1]),",",fmt(it[2]),"]  PC1 handicapped x2 point=",fmt(r1[9])," boosted x0.5 point=",fmt(r1[10]));
  write(fn,inj,",target,",fmt(r1[8]),",",fmt(it[1]),",",fmt(it[2]),",",it[3],",",fmt(1.0/inj));
  if(inj==2, if(ic[2]>=1 || it[2]>=1, allok=0));
  if(inj==1, if(ic[1]>1 || ic[2]<1 || it[1]>1 || it[2]<1, allok=0));
  if(abs(r1[9]-0.5*r1[8])>1e-9 || abs(r1[10]-2*r1[8])>1e-9, allok=0));
print("PCSYNTH overall: ",if(allok,"PASS (interval reports rho<1 when injected arm is cheaper; null contains 1)","FAIL"));
write(fn,"overall,",if(allok,"PASS","FAIL"),",,,,,");
}
quit;
