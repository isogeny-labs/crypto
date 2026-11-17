\\ e3_lemma.gp -- PC1 (group-law coherence), NC1 / H4 / P6 group-law audit (odd curves and one even curve per p).
\\ Writes out/lemma.csv (one row per audit) ; all stdout goes to out/lemma.out
default(parisizemax,2000000000);
read("e3_grid.gp");
read("e3_lib.gp");
N0=28;      \\ audit factor base size (4-subsets of 0..N0-1)
NPC1=50;
fn=Str(OUTDIR,"/lemma.csv"); system(Str("rm -f ",fn));
write(fn,"kind,p,c,l,lp,v1,v2,v3,v4,v5,v6,v7");
\\ ---------------------------------------------------------------- PC1 (positive control): logs agree with the F_{p^2} group law
\\ columns: v1=#E(Fp2)==l*lp (1/0), v2=generator order mismatches, v3=pi mismatches over x in 0..N0-1,
\\ v4=pair-sum full-point mismatches (of NPC1), v5=pair-sum x-coordinate mismatches, v6=NPC1, v7=n pairs with sum != O
pc1(p,c) = {
  my(cr=curve_row(p,c));
  ctx(p,cr[3],cr[4]);
  my(card2=ellcard(CE2), ok1=(card2==Cl*Clp));
  my(om=0, pim=0, pmm=0, pxm=0, nonO=0);
  if(ellorder(CE2,CGE2)!=Cl, om++);
  if(ellorder(CE2,CGT2)!=Clp, om++);
  \\ pi on generators: +1 on E, -1 on twist
  my(piE=[CGE2[1]^p,CGE2[2]^p], piT=[CGT2[1]^p,CGT2[2]^p]);
  if(piE!=CGE2, pim++);
  if(piT!=ellneg(CE2,CGT2), pim++);
  for(x=0,N0-1,
    my(Q=ptx2(x), piQ=[Q[1]^p,Q[2]^p]);
    if(Ctype[x+1]==1, if(piQ!=Q, pim++), if(piQ!=ellneg(CE2,Q), pim++)));
  setrand(4242);
  for(i=1,NPC1,
    my(x1=random(N0), x2=random(N0), s1=2*random(2)-1, s2=2*random(2)-1);
    my(P1=ptx2(x1), P2=ptx2(x2));
    if(s1<0, P1=ellneg(CE2,P1)); if(s2<0, P2=ellneg(CE2,P2));
    my(S=elladd(CE2,P1,P2));
    my(uu=0, ww=0);
    if(Ctype[x1+1]==1, uu+=s1*Cu[x1+1], ww+=s1*Cw[x1+1]);
    if(Ctype[x2+1]==1, uu+=s2*Cu[x2+1], ww+=s2*Cw[x2+1]);
    my(pred=elladd(CE2, ellmul(CE2,CGE2,uu%Cl), ellmul(CE2,CGT2,ww%Clp)));
    if(S!=pred, pmm++);
    if(S==[0] || pred==[0], if(S!=pred, pxm++), if(S[1]!=pred[1], pxm++));
    if(S!=[0], nonO++));
  print("PC1 p=",p," c=",c," #E(Fp2)==#E*#E': ",ok1," (",card2," vs ",Cl*Clp,") generator-order mismatches=",om," pi mismatches=",pim," pair-sum full-point mismatches=",pmm,"/",NPC1," x-coordinate mismatches=",pxm," nonzero sums=",nonO);
  write(fn,"pc1,",p,",",c,",",Cl,",",Clp,",",ok1,",",om,",",pim,",",pmm,",",pxm,",",NPC1,",",nonO);
}
\\ ---------------------------------------------------------------- NC1 / H4 / P6 audit, group law only (no logs)
\\ every 4-subset of V=0..N0-1, every sign vector (eps_1=+1, the other 8 -> 8 vectors; S -> -S symmetric, counted x2)
\\ E part A = signed sum of the terms in E(F_p) (f(x) square or zero), twist part B = signed sum of terms with f(x) non-square
\\ columns v1=#decompositions (all 16 sign vectors), v2=#with S in E(F_p), v3=#of those mixed (A-part and B-part both non-empty term sets),
\\ v4=violations 2B!=O, v5=violations B!=O, v6=violations A!=S, v7=#T-type x in V
audit(p,a,b,cid) = {
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
  my(par=if(cid==0,"even","odd"));
  print("AUDIT ",par," p=",p," c=",cid," a=",a," b=",b," N0=",N0," #decompositions=",ndec," #with R in E(Fp)=",nS," mixed(E and T terms)=",nmix," violations: 2B!=O -> ",v2B,"  B!=O -> ",vB,"  A!=R -> ",vA," (#T-type x in V=",nT,")");
  write(fn,"audit_",par,",",p,",",cid,",",ellcard(ellinit([a,b],p)),",",0,",",ndec,",",nS,",",nmix,",",v2B,",",vB,",",vA,",",nT);
}
{
foreach(GRID_A_P,p, for(c=1,2, pc1(p,c)));
foreach(GRID_A_P,p, for(c=1,2, my(cr=curve_row(p,c)); audit(p,cr[3],cr[4],c)));
my(RE=read_csv(Str(OUTDIR,"/curves_even.csv")));
for(i=1,#RE, audit(RE[i][1],RE[i][3],RE[i][4],0));
}
quit;
