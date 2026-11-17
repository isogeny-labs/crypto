\\ e3_model.gp -- registered model (PREREGISTRATION section 2) per cell, from curve parameters only. Writes out/model.csv
default(parisizemax,2000000000);
read("e3_grid.gp");
fn=Str(OUTDIR,"/model.csv"); system(Str("rm -f ",fn));
write(fn,"p,c,fam,m,N,NE,NT,l,lp,YD,YXgen,YXex,WD,WX,ZD,ZXgen,ZXex,rho_model_gen,rho_model_ex");
nonres(p) = {my(d=2); while(kronecker(d,p)!=-1, d++); d}
zz(w,y) = if(y==0,-1,w/y)
countsEN(p,a,b,N) = {my(ne=0,nt=0); for(x=0,N-1, my(f=(x^3+a*x+b)%p, k=kronecker(f,p)); if(k==1,ne++, if(k==-1,nt++))); [ne,nt]}
{
for(fi=1,2,
  my(fam=if(fi==1,"A","B"), m=famM(fam), m2=2*m);
  foreach(famP(fam),p, for(c=1,2,
    my(r=curve_row(p,c), a=r[3], b=r[4], l=r[5], lp=r[6]);
    foreach(famN(fam),N,
      my(en=countsEN(p,a,b,N), NE=en[1], NT=en[2]);
      my(YD=binomial(NE,m)*2^m/l);
      my(YXgen=sum(j=1,m,binomial(NE,j)*binomial(NT,m2-j)*2^m2/(l*lp)));
      my(YXex=binomial(NE,m)*binomial(NT,m)*2^m2/(l*lp));
      my(WD=binomial(NE,m-1)*2^(m-1), WX=binomial(N,m2-1)*2^(m2-1));
      my(ZD=zz(WD,YD), ZXgen=zz(WX,YXgen), ZXex=zz(WX,YXex));
      write(fn,p,",",c,",",fi,",",m,",",N,",",NE,",",NT,",",l,",",lp,",",fmt(YD),",",fmt(YXgen),",",fmt(YXex),",",WD,",",WX,",",fmt(ZD),",",fmt(ZXgen),",",fmt(ZXex),",",fmt(zz(ZXgen,ZD)),",",fmt(zz(ZXex,ZD)))))));
}
quit;
