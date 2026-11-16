\\ e2_lemma.gp -- controls L1 (x-level separation) and L2 (F_{p^2} separation lemma).  Env: E2_BITS (L1 size; L2 uses p~2^16 and 2^20 always)
default(parisizemax,2000000000);
default(realprecision,38);
read("e2_lib.gp");
bits=e2_envnum("E2_BITS",22); mu=1.5;
print("HEADER,gp_version=",version(),",control=L1+L2,bits=",bits,",mu=",mu,",L2_seed=777");

\\ ---------------- L1 ----------------
tt=e2_curves(bits,1); p=tt[1]; cv=tt[2][1]; a=cv[1]; b=cv[2]; lE=cv[3]; lP=cv[4]; d=e2_nonres(p);
N=round(mu*p^(1/3));
print("HEADER,L1,p=",p,",a=",a,",b=",b,",nE=",lE,",nEp=",lP,",d=",d,",N=",N);
fb=e2_scan(a,b,p,N); VE=fb[1]; VP=fb[2];
hE=e2_half(VE,a,b,p,0,N); hP=e2_half(VP,a,b,p,1,N);
SE=vecsort(hE[1]); SP=vecsort(hP[1]); SJ=vecsort(concat(hE[1],hP[1]));
\\ mixed pairs
mixed=0; rat=0; zer=0; nonres=0;
mixstep(xi,xj)={ my(iv=Mod(xi-xj,p)^-1, Dv=iv^2, sx=xi+xj, pm=xi*xj, s, pr, kr);
  s=2*((pm+a)*sx+2*b)*Dv; pr=((pm-a)^2-4*b*sx)*Dv; kr=kronecker(lift(s^2-4*pr),p);
  mixed++; if(kr==1,rat++, if(kr==0,zer++,nonres++)) };
{
if(N<=400, mode="full";
  for(i=1,N, for(j=1,N, mixstep(VE[i],VP[j]))),
  mode="sampled"; setrand(777);
  for(r=1,10^6, mixstep(VE[random(N)+1],VP[random(N)+1])));
}
\\ key-set disjointness : x values present with both type codes in the sorted joint array
inter=0; i=1; n=#SJ;
{
while(i<=n, xx=SJ[i]\KB; hasE=0; hasP=0;
  while(i<=n && SJ[i]\KB==xx, if(SJ[i]%KB>=TYPE_EP, hasP=1, hasE=1); i++);
  if(hasE&&hasP, inter++));
}
badE=0; for(i=1,#SE, if(kronecker(((SE[i]\KB)^3+a*(SE[i]\KB)+b)%p,p)!=1, badE++));
badP=0; for(i=1,#SP, if(kronecker(((SP[i]\KB)^3+a*(SP[i]\KB)+b)%p,p)!=-1, badP++));
print("L1,bits=",bits,",mode=",mode,",N=",N,",mixed_pairs=",mixed,",mixed_rational=",rat,",mixed_zero_disc=",zer,",mixed_nonsquare=",nonres,",key_intersection=",inter,",Ekeys=",#SE,",Epkeys=",#SP,",E_keys_nonsquare_f=",badE,",Ep_keys_square_f=",badP);

\\ ---------------- L2 ----------------
L2(pb,npairs)={
  my(tq=e2_curves(pb,1), p=tq[1], cv=tq[2][1], a=cv[1], b=cv[2], dd=e2_nonres(p), g, EF, Epc, nrat=0, nT2=0, nT3=0, nT4=0, nQneg=0, nPfix=0, nQnz=0, ntot=0, n4=0);
  g=ffgen(Mod(1,p)*(x^2-dd),'t); EF=ellinit([a*g^0,b*g^0]); Epc=ellinit([a*dd^2%p,b*dd^3%p],p);
  my(fr=(Z)->if(#Z<2,Z,[Z[1]^p,Z[2]^p]));
  my(randE=()->my(x,fx); while(1,x=random(p); fx=(x^3+a*x+b)%p; if(kronecker(fx,p)==1, my(y=lift(sqrt(Mod(fx,p)))); if(random(2),y=p-y); return([x*g^0,y*g^0]))));
  my(randEp=()->my(X,fx); while(1,X=random(p); fx=(X^3+a*dd^2*X+b*dd^3)%p; if(kronecker(fx,p)==1, my(Y=lift(sqrt(Mod(fx,p)))); if(random(2),Y=p-Y); return([lift(Mod(X,p)/dd)*g^0, g*lift(Mod(Y,p)/dd^2)]))));
  setrand(777);
  for(r=1,npairs,
    my(P=randE(), Q=randEp(), Q2=randEp(), S, S2, S3, SQ);
    if(!ellisoncurve(EF,P)||!ellisoncurve(EF,Q), error("lift failed"));
    ntot++;
    if(fr(P)==P, nPfix++);
    if(fr(Q)==ellneg(EF,Q), nQneg++);
    S=elladd(EF,P,Q);
    if(#S>=2 && fr(S)==S, nrat++);                    \\ T1: P+Q rational?
    S2=elladd(EF,S,ellneg(EF,Q));                      \\ T2: P+Q+(-Q) = P ; B=O
    if(#S2>=2 && fr(S2)==S2 && S2==P, nT2++);
    if(fr(S)==elladd(EF,P,ellneg(EF,Q)), nT3++);       \\ T3: pi(P+Q) = P - Q
    S3=elladd(EF,Q,Q2);                                \\ T4: pi(Q+Q2) = -(Q+Q2)
    if(#S3<2, ntot--; next);
    n4++; if(fr(S3)==ellneg(EF,S3), nT4++));
  print("L2,pbits=",pb,",p=",p,",nE=",cv[3],",nEp=",cv[4],",d=",dd,",pairs=",ntot,",T1_PplusQ_rational=",nrat,",T2_BoZero_rational=",nT2,",T3_frob_is_P_minus_Q=",nT3,",T4_pairs=",n4,",T4_frob_neg=",nT4,",Q_frob_eq_negQ=",nQneg,",P_frob_fixed=",nPfix);
}
L2(16,2000); L2(20,2000);
quit;
