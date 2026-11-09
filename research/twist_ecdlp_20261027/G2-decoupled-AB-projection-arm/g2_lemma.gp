\\ g2_lemma.gp -- NC1 (projection of class-S targets in F_{p^2} arithmetic), the decomposition audit, even-order boundary, NC4 (E-only factor base).
\\ Writes OUT/lemma.csv ; stdout -> out/lemma.out
default(parisizemax,2000000000);
read("g2_lib.gp");
N0=20;      \\ audit factor base 0..N0-1 (4-subsets)
NT1=200;    \\ class-S targets per curve (NC1)
NNC4=24;    \\ NC4 factor base size
fn=Str(OUTDIR,"/lemma.csv"); system(Str("rm -f ",fn));
write(fn,"kind,p,c,l,lp,v1,v2,v3,v4,v5,v6,v7,v8");
curves=read_csv(Str(OUTDIR,"/curves.csv"));
\\ ---------------------------------------------------------------- NC1 odd curves (also class-F projection correctness)
\\ v1=#class-S targets, v2=violations (B'!=O or A'!=R as points of E(F_{p^2})), v3=#class-F targets, v4=class-F mismatches
{
foreach(curves,cr,
  my(p=cr[1], c=cr[2]);
  ctx(p,cr[3],cr[4]); proj_init();
  setrand(4242);
  my(viol=0, vF=0);
  for(i=1,NT1,
    my(A=1+random(Cl-1), B=1+random(Clp-1));
    my(Rs=ellmul(CE2,CGE2,A), P1=projpts(Rs));
    if(P1[2]!=[0] || P1[1]!=Rs, viol++);
    my(RA=ellmul(CE2,CGE2,A), RB=ellmul(CE2,CGT2,B), P2=projpts(elladd(CE2,RA,RB)));
    if(P2[1]!=RA || P2[2]!=RB, vF++));
  print("NC1 p=",p," c=",c," kind=",cr[8]," #E=",Cl," #E'=",Clp," class-S violations=",viol,"/",NT1,"  class-F projection mismatches=",vF,"/",NT1);
  write(fn,"nc1_odd,",p,",",c,",",Cl,",",Clp,",",NT1,",",viol,",",NT1,",",vF,",",cr[8],",0,0,0");
);
}
\\ ---------------------------------------------------------------- decomposition audit (group law only), 4-subsets of 0..N0-1
\\ v1=#decompositions (all 16 sign vectors), v2=#with S in E(F_p), v3=#mixed, v4=viol 2B!=O, v5=viol B!=O, v6=viol A!=S, v7=#T-type x in V
audit(p,a,b,cid,par) = {
  my(d=nonres(p), Ct2=ffgen(Mod(1,p)*('t^2-d),'t), E2=ellinit([a*Ct2^0,b*Ct2^0]));
  my(pts=vector(N0), ty=vectorsmall(N0), nT=0);
  for(x=0,N0-1,
    my(f=(x^3+a*x+b)%p, k=kronecker(f,p));
    if(k==0, ty[x+1]=1; pts[x+1]=[x*Ct2^0, 0*Ct2^0],
      if(k==1, ty[x+1]=1; pts[x+1]=[x*Ct2^0, lift(sqrt(Mod(f,p)))*Ct2^0],
        ty[x+1]=-1; nT++; pts[x+1]=[x*Ct2^0, lift(sqrt(Mod(f*lift(1/Mod(d,p)),p)))*Ct2])));
  my(ndec=0, nS=0, nmix=0, v2B=0, vB=0, vA=0);
  forsubset([N0,4], s,
    my(Q=vector(4,i,pts[s[i]]), tt=vector(4,i,ty[s[i]]));
    my(hasE=0, hasT=0); for(i=1,4, if(tt[i]==1, hasE=1, hasT=1));
    for(mask=0,7,
      my(A=[0], B=[0]);
      for(i=1,4,
        my(R=Q[i]);
        if(i>1 && bittest(mask,i-2), R=ellneg(E2,R));
        if(tt[i]==1, A=elladd(E2,A,R), B=elladd(E2,B,R)));
      my(S=elladd(E2,A,B));
      ndec+=2;
      my(inE = (S==[0]) || (S[1]^p==S[1] && S[2]^p==S[2]));
      if(inE,
        nS+=2; if(hasE && hasT, nmix+=2);
        if(ellmul(E2,B,2)!=[0], v2B+=2);
        if(B!=[0], vB+=2);
        if(A!=S, vA+=2))));
  print("AUDIT ",par," p=",p," c=",cid," a=",a," b=",b," N0=",N0," #decompositions=",ndec," #with R in E(Fp)=",nS," mixed(E and T terms)=",nmix," violations: 2B!=O -> ",v2B,"  B!=O -> ",vB,"  A!=R -> ",vA," (#T-type x in V=",nT,")");
  write(fn,"audit_",par,",",p,",",cid,",",ellcard(ellinit([a,b],p)),",0,",ndec,",",nS,",",nmix,",",v2B,",",vB,",",vA,",",nT);
}
{
foreach(curves,cr, audit(cr[1],cr[3],cr[4],cr[2],"odd"));
}
\\ ---------------------------------------------------------------- even-order boundary: projection of class-S targets (random R in E(F_p)) with 1/2 taken modulo the odd part
\\ v1=#targets, v2=#with B'!=O, v3=#with A'!=R, v4=#R of even order, v5=#R with R not of odd order and A'==R (should be 0), v6=#E, v7=#E'
{
my(RE=read_csv(Str(OUTDIR,"/curves_even.csv")));
for(i=1,#RE, my(r=RE[i], p=r[1], a=r[3], b=r[4], n=r[5], np=r[6]);
  my(d=nonres(p), Ct2=ffgen(Mod(1,p)*('t^2-d),'t), E2=ellinit([a*Ct2^0,b*Ct2^0]));
  my(ntot=n*np, nodd=ntot/2^valuation(ntot,2), h=lift(1/Mod(2,nodd)));
  if(ellcard(E2)!=ntot, error("even curve: #E(Fp2) != #E*#E'"));
  setrand(4242);
  my(cnt=0, vB=0, vA=0, ev=0, bad5=0);
  while(cnt<NT1,
    my(x0=random(p), f=(x0^3+a*x0+b)%p);
    if(kronecker(f,p)==-1, next);
    my(y0=lift(sqrt(Mod(f,p))), R=[x0*Ct2^0, y0*Ct2^0]);
    if(random(2), R=ellneg(E2,R));
    cnt++;
    my(piR=[R[1]^p,R[2]^p], S=elladd(E2,R,piR), D=elladd(E2,R,ellneg(E2,piR)));
    my(A1=ellmul(E2,S,h), B1=ellmul(E2,D,h));
    my(isEven=(ellmul(E2,R,nodd)!=[0]));
    if(B1!=[0], vB++); if(A1!=R, vA++); if(isEven, ev++);
    if(isEven && A1==R, bad5++));
  print("NC1 even-order p=",p," #E=",n," #E'=",np," class-S targets=",NT1," B'!=O: ",vB,"  A'!=R: ",vA,"  (R of even order: ",ev,"; even order but A'==R: ",bad5,")");
  write(fn,"nc1_even,",p,",0,",n,",",np,",",NT1,",",vB,",",vA,",",ev,",",bad5,",0,0,0");
  audit(p,a,b,0,"even"));
}
\\ ---------------------------------------------------------------- NC4: E-only factor base (all x-sets of E-type x in V, size 4, 8 sign vectors x2) against 200 class-F and 200 class-S targets, actual F_{p^2} group law
\\ v1=#E-only sums computed, v2=hits vs class F targets (expect 0), v3=hits vs class S targets (positive control for the detector, expect >0)
{
foreach(curves,cr,
  if(cr[8]!=0, next);
  my(p=cr[1], c=cr[2]);
  ctx(p,cr[3],cr[4]); proj_init();
  my(tg=draw_targets(7000+10*p+c,NT1), Av=tg[1], Bv=tg[2]);
  my(MF=Map(), MS=Map());
  for(i=1,NT1,
    my(Rf=target_pt(Av[i],Bv[i]), Rs=ellmul(CE2,CGE2,Av[i]));
    mapput(MF,Str(Rf[1],"|",Rf[2]),1); mapput(MF,Str(ellneg(CE2,Rf)[1],"|",ellneg(CE2,Rf)[2]),1);
    mapput(MS,Str(Rs[1],"|",Rs[2]),1); mapput(MS,Str(ellneg(CE2,Rs)[1],"|",ellneg(CE2,Rs)[2]),1));
  my(Ei=List()); for(x=0,NNC4-1, if(Ctype[x+1]==1, listput(Ei,x))); Ei=Vec(Ei);
  my(nsum=0, hf=0, hs=0);
  forsubset([#Ei,4], s,
    my(Q=vector(4,i,ptx2(Ei[s[i]])));
    for(mask=0,7,
      my(S=[0]);
      for(i=1,4, my(R=Q[i]); if(i>1 && bittest(mask,i-2), R=ellneg(CE2,R)); S=elladd(CE2,S,R));
      nsum++;
      if(S!=[0], my(k=Str(S[1],"|",S[2])); if(mapisdefined(MF,k), hf++); if(mapisdefined(MS,k), hs++))));
  print("NC4 p=",p," c=",c," N=",NNC4," #E-type x=",#Ei," E-only 4-set signed sums=",nsum," hits vs 200 class-F targets (B!=O): ",hf," ; hits vs 200 class-S targets (control): ",hs);
  write(fn,"nc4,",p,",",c,",",Cl,",",Clp,",",nsum,",",hf,",",hs,",0,0,0,0,0"););
}
quit;
