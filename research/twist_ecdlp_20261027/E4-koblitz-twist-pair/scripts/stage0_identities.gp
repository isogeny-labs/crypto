default(parisizemax,2000000000);
read("scripts/lib.gp");
m=eval(getenv("M")); lib_init(m);
SEED0=2026102100+1000000*m; setrand(SEED0);
print("# stage0_identities m=",m," seed=",SEED0," modulus=",MODINT," gp=",GPVER," x_primitive=",GENPRIM);
chk(name,ok,detail) = print("CHECK ",name," m=",m," ",if(ok,"PASS","FAIL")," ",detail);
lucas(mu,mm)=my(Va=2,Vb=mu,Vc); for(k=2,mm, Vc=mu*Vb-2*Va; Va=Vb; Vb=Vc); Vb;
E0=ellinit([1,0,0,0,1],GEN);   \\ K0: y^2+xy=x^3+1       (a2=0)
E1=ellinit([1,1,0,0,1],GEN);   \\ K1: y^2+xy=x^3+x^2+1   (a2=1)

\\ ---- T: table layer vs native FFELT (instrument validation) ----
{
  my(bad=0,x,y,a,b,c,w,tr,trl);
  for(i=1,4000,
    a=random(QM1)+1; b=random(QM1)+1; x=ffe(a); y=ffe(b);
    if(fint(x*y)!=mul(a,b), bad++);
    if(fint(x^2)!=sq(a), bad++);
    if(fint(1/x)!=inv(a), bad++);
    tr=lift(trace(x)); trl=0; c=x; for(j=1,m, trl+=c; c=c^2);   \\ trace by explicit loop of m squarings
    if(trl!=tr && trl!=GEN^0*tr, bad++);
    if(tr!=TRT[a+1], bad++);
    c=random(Q); w=HTT[c+1]; \\ half-trace identity w^2+w = c + Tr(c)
    if(bitxor(sq(w),w)!=bitxor(c,TRT[c+1]), bad++));
  chk("T_tables_vs_ffelt", bad==0, Str("n=4000 mismatches=",bad));
}
\\ ---- C1, C2: counts, traces, Lucas recurrence, cofactors ----
{
  my(N0=0,N1=0,c0,c1,t0,t1,V0,V1,ok,P);
  for(x=1,QM1, if(TYT[x+1]==0,N0++,N1++));
  c0=ellcard(E0); c1=ellcard(E1);
  print("VALUE ncensus_K0=",2+2*N0," ncensus_K1=",2+2*N1," ellcard_K0=",c0," ellcard_K1=",c1);
  print("VALUE N0=",N0," N1=",N1);
  chk("C1_sum", c0+c1==2^(m+1)+2 && 2+2*N0+2+2*N1==2^(m+1)+2, Str("ellcard sum=",c0+c1," census sum=",4+2*(N0+N1)," target=",2^(m+1)+2));
  chk("C1_census_eq_ellcard", 2+2*N0==c0 && 2+2*N1==c1, Str("census K0=",2+2*N0," K1=",2+2*N1));
  t0=2^m+1-c0; t1=2^m+1-c1;
  print("VALUE t_K0=",t0," t_K1=",t1);
  chk("C2_trace_negation", t1==-t0, Str("t0=",t0," t1=",t1));
  \\ Lucas: tau^2 - mu*tau + 2 = 0, mu=(-1)^(1-a); V_k = mu V_{k-1} - 2 V_{k-2}; t(F_{2^m}) = V_m

  V0=lucas(-1,m); V1=lucas(1,m);
  chk("C2_lucas", V0==t0 && V1==t1, Str("Lucas t(K0)=",V0," t(K1)=",V1));
  chk("C2_tau_negation", subst(x^2+x+2,x,-x)==x^2-x+2, "tau1=-tau0 (polynomials tau^2+tau+2 -> tau^2-tau+2)");
  chk("C2_cofactors", valuation(c0,2)==2 && valuation(c1,2)==1, Str("v2(#K0)=",valuation(c0,2)," v2(#K1)=",valuation(c1,2)," odd parts ",c0/4," ",c1/2," gcd=",gcd(c0/4,c1/2)));
  chk("C1_N_formulas", N0==(c0-2)/2 && N1==(c1-2)/2 && N0+N1==2^m-1, Str("N0+N1=",N0+N1));
  ok=1; for(i=1,40, P=random(E0); if(ellmul(E0,P,c0)!=[0], ok=0); P=random(E1); if(ellmul(E1,P,c1)!=[0], ok=0));
  chk("C1_point_orders", ok, "random points: [#E]P=O on both curves (40 each)");
}
\\ ---- C3: every x in F_q^* is the abscissa of a point of exactly one curve, agreeing with the table type ----
{
  my(G0,x,xi,e0,e1,bad=0,bad2=0,n0=0,n1=0);
  G0=ffprimroot(GEN); x=GEN^0;
  for(i=0,QM1-1,
    xi=fint(x); e0=#ellordinate(E0,x); e1=#ellordinate(E1,x);
    if(e0+e1!=2 || (e0==2)!=(TYT[xi+1]==0), bad++);
    if(e0==2,n0++,n1++);
    x*=G0);
  chk("C3_exactly_one_type", bad==0 && n0+n1==QM1, Str("x scanned=",n0+n1," ellordinate-type0=",n0," type1=",n1," violations=",bad));
}
\\ ---- C4: Frobenius preserves type (all x), C6 partial via orbit canon ----
{
  my(bad=0);
  for(x=1,QM1, if(TYT[sq(x)+1]!=TYT[x+1], bad++));
  chk("C4_frobenius_type", bad==0, Str("x checked=",QM1," violations=",bad));
}
\\ ---- C5: Semaev S3 shared by K0 and K1; vanishes at x(P1+P2), x(P1-P2); roots3 returns exactly those ----
{
  my(P1,P2,S,D,x1,x2,xs,xd,bad0=0,bad1=0,badr=0,n0=0,n1=0,r,tot);
  for(cv=0,1,
    for(i=1,2000,
      my(E=if(cv,E1,E0));
      P1=random(E); P2=random(E);
      if(P1==[0]||P2==[0], next);
      x1=fint(P1[1]); x2=fint(P2[1]);
      if(x1==0||x2==0||x1==x2, next);
      S=elladd(E,P1,P2); D=elladd(E,P1,ellneg(E,P2));
      if(S==[0]||D==[0], next);
      xs=fint(S[1]); xd=fint(D[1]);
      if(S3(x1,x2,xs)||S3(x1,x2,xd), if(cv,bad1++,bad0++));
      r=roots3(x1,x2); if(Set(r)!=Set([xs,xd]), badr++);
      if(cv,n1++,n0++)));
  chk("C5_S3_K0", bad0==0, Str("pairs=",n0," nonvanishing=",bad0));
  chk("C5_S3_K1_shared", bad1==0, Str("pairs=",n1," nonvanishing=",bad1));
  chk("C5_roots3_equals_pair_sum_diff", badr==0, Str("pairs=",n0+n1," set mismatches=",badr));
  \\ doubling: x(2P) = x^2 + b/x^2, b=1 (equal-abscissa branch of roots3)
  my(bad=0,P,Dd);
  for(cv=0,1, my(E=if(cv,E1,E0)); for(i=1,300, P=random(E); if(P==[0]||fint(P[1])==0, next); Dd=ellmul(E,P,2); if(Dd==[0], next); x1=fint(P[1]); if(roots3(x1,x1)!=[fint(Dd[1])], bad++)));
  chk("C5_doubling_branch", bad==0, Str("pairs=600 mismatches=",bad));
}
\\ ---- solver brute-force cross-check: #roots3 vs exhaustive search over all x3 in F_q ----
{
  my(npairs=[2000,300,0,20,8][if(m<=11,1,m<=13,2,m<=17,4,5)],bad=0,x1,x2,cnt,r,nc=0,ns=0);
  if(m==17, npairs=20);
  for(i=1,npairs,
    x1=random(QM1)+1; x2=random(QM1)+1;
    cnt=0; for(x3=0,QM1, if(S3(x1,x2,x3)==0, cnt++));
    r=roots3(x1,x2);
    if(cnt!=#Set(r), bad++);
    if(TYT[x1+1]==TYT[x2+1], ns++, nc++));
  chk("solver_bruteforce", bad==0, Str("pairs=",npairs," same-type=",ns," cross-type=",nc," root-count mismatches=",bad));
}
print("# stage0 done m=",m);
quit
