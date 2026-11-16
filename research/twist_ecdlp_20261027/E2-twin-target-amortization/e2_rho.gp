\\ e2_rho.gp -- measured Pollard rho reference on E (r-adding walk, 16 branches, Brent cycle detection, 10 seeds).  Env: E2_BITS E2_CURVE
default(parisizemax,2000000000);
default(realprecision,38);
read("e2_lib.gp");
bits=e2_envnum("E2_BITS",26); cidx=e2_envnum("E2_CURVE",1);
tt=e2_curves(bits,max(cidx,2)); p=tt[1]; cv=tt[2][cidx]; a=cv[1]; b=cv[2]; l=cv[3]; lP=cv[4];
Ec=ellinit([a,b],p);
print("HEADER,gp_version=",version(),",bits=",bits,",curve=",cidx,",p=",p,",a=",a,",b=",b,",l=",l,",seeds=5000+100*c+k(k=1..10),nominal_0.886_sqrt_l=",0.886*sqrt(l)*1.);
x0=p\2; while(kronecker((x0^3+a*x0+b)%p,p)!=1, x0++);
P=[Mod(x0,p),ellordinate(Ec,x0)[1]];
RB=16;
\\ one attempt: returns [success,k,steps,initGAE]
rho_attempt(Q,Mu,Mv,Mp)={
  my(a0=random(l), b0=random(l), X, at, bt, ah, bh, T, H, steps=0, power=1, lam=1, j, cap=ceil(60*sqrt(l))+1000, kk, init);
  init=3*bits+1;                       \\ nominal GAE for the starting point a0 P + b0 Q (double-and-add)
  H=elladd(Ec,ellmul(Ec,P,a0),ellmul(Ec,Q,b0)); ah=a0; bh=b0; T=H; at=ah; bt=bh;
  while(steps<cap,
    \\ hare step
    j=if(#H<2,1,lift(H[1])%RB+1); H=elladd(Ec,H,Mp[j]); ah=(ah+Mu[j])%l; bh=(bh+Mv[j])%l; steps++;
    if(H==T,
      if((bh-bt)%l==0, return([0,0,steps,init]));
      kk=Mod(at-ah,l)/Mod(bh-bt,l); kk=lift(kk);
      if(ellmul(Ec,P,kk)==Q, return([1,kk,steps,init]), return([0,0,steps,init])));
    if(power==lam, T=H; at=ah; bt=bh; power*=2; lam=0);
    lam++);
  [0,0,steps,init]
}
{
for(k=1,10,
  setrand(5000+100*cidx+k);
  my(kk=1+random(l-1), Q=ellmul(Ec,P,kk), Mu=vector(RB,i,random(l)), Mv=vector(RB,i,random(l)), Mp, tot=0, att=0, ok=0, r, setup, initall=0);
  Mp=vector(RB,j,elladd(Ec,ellmul(Ec,P,Mu[j]),ellmul(Ec,Q,Mv[j])));
  setup=RB*(3*bits+1);
  while(!ok && att<50, att++; r=rho_attempt(Q,Mu,Mv,Mp); tot+=r[3]; initall+=r[4]; ok=r[1]; if(ok && r[2]!=kk, ok=-1));
  print("RHO,bits=",bits,",curve=",cidx,",k=",k,",l=",l,",attempts=",att,",steps=",tot,",cmp=",tot,",init=",initall,",setup=",setup,",success=",ok));
}
quit;
