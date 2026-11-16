default(parisizemax,2000000000);
\\ Stage 0b: the separation check inside E(F_{q^2}), q=2^m. No table layer is used: pure native FFELT.
m=eval(getenv("M")); q=2^m;
SEED0=2026102100+1000000*m+1; setrand(SEED0);       \\ seed not fixed by the preregistration; chosen here and recorded
G=ffgen(2^(2*m),'b); g2=ffprimroot(G);
print("# stage0b_fq2 m=",m," seed=",SEED0," modulus_poly=",G.mod," gp=",version());
E0=ellinit([1,0,0,0,1],G); E1=ellinit([1,1,0,0,1],G);
s=g2^((q^2-1)/3);                                    \\ primitive cube root of unity, s^2+s=1 (char 2), s in F_4 not in F_q (m odd)
chk(name,ok,detail) = print("CHECK ",name," m=",m," ",if(ok,"PASS","FAIL")," ",detail);
inFq(u) = u^q==u;
embed() = g2^((q+1)*random(q-1));                    \\ uniform element of F_q^* inside F_{q^2}
trq(u) = my(t=G*0,c=u); for(i=1,m, t+=c; c=c^2); t; \\ absolute trace Tr_{F_q/F_2} of u in F_q, by m squarings
phi(P) = [P[1], P[2]+s*P[1]];                         \\ K1 -> K0, isomorphism over F_4 (y -> y + s x)
chk("S_cuberoot", s^2+s==1 && !inFq(s) && ellisoncurve(E0,[G*0,G^0]) && ellisoncurve(E1,[G*0,G^0]), "s^2+s=1, s not in F_q, T=(0,1) on both curves");
\\ random F_q-rational point (x != 0) of curve E, all coordinates in F_q; returns [] if the sampled x is not an abscissa of E(F_q)
rpt(E) = my(x=embed(),ys=ellordinate(E,x),y); if(#ys==0,return([])); y=ys[1]; if(!inFq(y), return([])); [x,y];
getpt(E) = my(P=[]); while(#P==0, P=rpt(E)); P;
NP=200;
\\ type agreement: rational-y existence vs the trace criterion Tr(x+1/x)
{
  my(bad=0,x,tt,r0,r1,n0=0,n1=0);
  for(i=1,NP, x=embed(); tt=trq(x+1/x);
    r0=#ellordinate(E0,x)>0; r1=#ellordinate(E1,x)>0;
    \\ ellordinate over F_{q^2} always finds y; rationality in F_q is tested by y in F_q
    r0=(r0 && inFq(ellordinate(E0,x)[1])); r1=(r1 && inFq(ellordinate(E1,x)[1]));
    if((tt==0)!=r0 || (tt==1)!=r1 || r0==r1, bad++);
    if(r0,n0++,n1++));
  chk("S_type_vs_rationality", bad==0, Str("x sampled=",NP," type0=",n0," type1=",n1," violations=",bad));
}
T=[G*0,G^0];
{
  my(P1,P2,Q2,S,D,ninF,nO,nfr,nsame);
  \\ K0 x K0
  ninF=0;nO=0; for(i=1,NP, P1=getpt(E0); P2=getpt(E0); if(P1==P2||P1==ellneg(E0,P2),next); S=elladd(E0,P1,P2); if(S==[0],nO++,if(inFq(S[1]),ninF++)));
  print("STAGE0B m=",m," class=K0xK0 n=",NP," x_in_Fq=",ninF," isO=",nO);
  chk("PC3_K0xK0", ninF+nO==NP, Str("x(P1+P2) in F_q for ",ninF,"/",NP));
  \\ K1 x K1 (points genuinely on K1 with F_q coordinates; added on K1 over F_{q^2}, and also mapped to K0)
  ninF=0;nO=0;nsame=0; for(i=1,NP, P1=getpt(E1); P2=getpt(E1); if(P1==P2||P1==ellneg(E1,P2),next); S=elladd(E1,P1,P2); if(S==[0],nO++,if(inFq(S[1]),ninF++));
     my(S0=elladd(E0,phi(P1),phi(P2))); if(S!=[0] && S0!=[0] && S0[1]==S[1], nsame++));
  print("STAGE0B m=",m," class=K1xK1 n=",NP," x_in_Fq=",ninF," isO=",nO," x_same_on_K0_via_phi=",nsame);
  chk("PC3_K1xK1", ninF+nO==NP && nsame==ninF, Str("x(P1+P2) in F_q for ",ninF,"/",NP));
  \\ K1 x T and K0 x T
  ninF=0; for(i=1,NP, P1=getpt(E1); S=elladd(E0,phi(P1),T); if(inFq(S[1]),ninF++));
  print("STAGE0B m=",m," class=K1xT n=",NP," x_in_Fq=",ninF);
  chk("PC3_K1xT", ninF==NP, Str("x(P1+T) in F_q for ",ninF,"/",NP));
  ninF=0; for(i=1,NP, P1=getpt(E0); S=elladd(E0,P1,T); if(inFq(S[1]),ninF++));
  print("STAGE0B m=",m," class=K0xT n=",NP," x_in_Fq=",ninF);
  chk("PC3_K0xT", ninF==NP, Str("x(P1+T) in F_q for ",ninF,"/",NP));
  \\ K0 x K1 (mixed; both != T): prediction x(P1+P2) NOT in F_q; and pi(P1+P2) = P1-P2
  ninF=0;nO=0;nfr=0; for(i=1,NP, P1=getpt(E0); P2=phi(getpt(E1)); S=elladd(E0,P1,P2); D=elladd(E0,P1,ellneg(E0,P2));
     if(S==[0],nO++,if(inFq(S[1]),ninF++); if([S[1]^q,S[2]^q]==D,nfr++)));
  print("STAGE0B m=",m," class=K0xK1 n=",NP," x_in_Fq=",ninF," isO=",nO," pi_S_equals_P1_minus_P2=",nfr);
  chk("F2_mixed_not_rational", ninF==0 && nO==0 && nfr==NP, Str("x(P1+P2) in F_q for ",ninF,"/",NP,"; pi(S)=P1-P2 for ",nfr,"/",NP));
}
print("# stage0b done m=",m);
quit
