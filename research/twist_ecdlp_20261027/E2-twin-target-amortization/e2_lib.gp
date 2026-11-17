\\ e2_lib.gp -- shared library for E2-twin-target-amortization (PARI/GP 2.15.4)
\\ Frozen with PREREGISTRATION.md.  All counters are plain global integers.

KB = 2^26;            \\ key = x*KB + code ; code = (type<<24) | (i-1)<<12 | (j-1)
TYPE_EP = 2^24;       \\ type bit for E'-half codes

\\ ---- environment helpers ---------------------------------------------------
e2_env(name, dflt) = { my(s=getenv(name)); if(s===0 || s=="", dflt, s) };
e2_envnum(name, dflt) = { my(s=getenv(name)); if(s===0 || s=="", dflt, eval(s)) };

\\ ---- curve search ----------------------------------------------------------
\\ returns [p, [[a,b,#E,#E'],...]] : first ncurves curves with #E and #E' = 2(p+1)-#E BOTH prime
e2_curves(bits, ncurves) = {
  my(p=precprime(2^bits-1), res=List(), a, b, E, n, n2);
  setrand(20261021+bits);
  while(#res<ncurves,
    a=random(p); b=random(p);
    if((4*a^3+27*b^2)%p==0, next);
    E=ellinit([a,b],p); n=ellcard(E); n2=2*(p+1)-n;
    if(isprime(n) && isprime(n2), listput(res,[a,b,n,n2])));
  [p, Vec(res)]
}

\\ least positive quadratic non-residue
e2_nonres(p) = { my(d=1); while(kronecker(d,p)!=-1, d++); d };

\\ ceil(log2 n) for n>=2
e2_clog2(n) = if(n<=1, 0, #binary(n-1));

\\ ---- factor base scan ------------------------------------------------------
\\ returns [VE, VEp, scanE, scanEp] ; scan counts = Legendre symbol evaluations needed for each class alone
e2_scan(a,b,p,N) = {
  my(VE=vector(N), VP=vector(N), ne=0, nq=0, x=0, fx, kr, sE=0, sP=0);
  while(ne<N || nq<N,
    x++; fx=(x^3+a*x+b)%p; kr=kronecker(fx,p);
    if(kr==1 && ne<N, ne++; VE[ne]=x; if(ne==N, sE=x));
    if(kr==-1 && nq<N, nq++; VP[nq]=x; if(nq==N, sP=x)));
  [VE,VP,sE,sP]
}

\\ ---- half table ------------------------------------------------------------
\\ builds sorted key array for one class. typ=0 (E), 1 (E'). returns [S_unsorted, K, SQ]
\\ K = inversions (pair steps incl. doublings), SQ = square roots.
e2_half(V,a,b,p,typ,N) = {
  my(S=vectorsmall(N*N), idx=0, K=0, SQ=0, xi,xj,fi,iv,Dv,sx,pm,s,pr,r,hf=Mod(1,p)/2,tb=typ*TYPE_EP,c);
  for(i=1,N, xi=V[i]; fi=(xi^3+a*xi+b)%p;
    c=tb+(i-1)*4096+(i-1);
    idx++; S[idx]=lift(Mod((xi^2-a)^2-8*b*xi,p)/Mod(4*fi,p))*KB+c; K++;
    for(j=i+1,N, xj=V[j];
      iv=Mod(xi-xj,p)^-1; Dv=iv^2; K++;
      sx=xi+xj; pm=xi*xj;
      s=2*((pm+a)*sx+2*b)*Dv; pr=((pm-a)^2-4*b*sx)*Dv;
      r=sqrt(s^2-4*pr); SQ++;
      c=tb+(i-1)*4096+(j-1);
      idx++; S[idx]=lift((s+r)*hf)*KB+c;
      idx++; S[idx]=lift((s-r)*hf)*KB+c));
  [S,K,SQ]
}

\\ ---- lookup ----------------------------------------------------------------
PR = 0;   \\ global probe counter
\\ returns Vec of codes whose key abscissa equals x ; counts every array read as a probe
e2_find(~S,x) = {
  my(n=#S, lo=1, hi=n+1, m, base=x*KB, res=[]);
  while(lo<hi, m=(lo+hi)\2; PR++; if(S[m]<base, lo=m+1, hi=m));
  while(lo<=n, PR++; if(S[lo]\KB!=x, break); res=concat(res,S[lo]%KB); lo++);
  res
}

\\ ---- lifting of the factor base (verification only, excluded from cost) ------
\\ type 0: points of E ; type 1: points of E' in the model Y^2=X^3+a d^2 X+b d^3 with X=d x
e2_lift(Ec,V,typ,d,p) = {
  my(N=#V, pts=vector(N), X);
  for(i=1,N, X=if(typ==0, V[i], V[i]*d%p);
    pts[i]=[Mod(X,p), ellordinate(Ec,X)[1]]);
  pts
}

\\ verification of a relation R = +-P_i +- P_j on curve Ec (explicit group arithmetic)
\\ pts: lifted points of the code's class. Requires R on the curve.
e2_verify(Ec,R,~pts,i,j) = {
  my(Pi=pts[i], Pj=pts[j], S1, S2);
  if(!ellisoncurve(Ec,R), return(0));
  if(i==j, S1=elladd(Ec,Pi,Pi); return(R==S1 || R==ellneg(Ec,S1)));
  S1=elladd(Ec,Pi,Pj); S2=elladd(Ec,Pi,ellneg(Ec,Pj));
  (R==S1 || R==ellneg(Ec,S1) || R==S2 || R==ellneg(Ec,S2))
}

\\ ---- target collection -------------------------------------------------------
\\ Ec/P : the curve and generator the target stream lives on (E' model for typ=1)
\\ Et,pts : the curve on which relations are verified (equal to Ec in the main arms) and lifted FB points of the class
\\ typ : 0 -> E stream (x=R[1]); 1 -> E' stream (x=R[1]*dinv, one scaling multiplication per target)
\\ Sa : class-only sorted array ; Sj : joint sorted array or 0
\\ s0 : R_k=(s0+k)P ; W : stop at W verified relations (fixedT>0: run exactly fixedT targets) ; Tmax cap
\\ chk : 1 -> additionally check R == ellmul(Ec,P,s0+k) for every verified relation (stream sanity)
\\ returns [T, probesA, probesJ, relA, relJ, hitT, unverA, crossJ, capflag, relsA(Vecsmall of k*KB+code), relsJ]
e2_collect(Ec,P,typ,dinv,Sa,Sj,Et,pts,s0,W,Tmax,fixedT,chk) = {
  my(R=ellmul(Ec,P,s0), T=0, x, codes, codesj, same, cj, ra=List(), rj=List(), na0,
     hitT=0, unA=0, cross=0, pa=0, pj=0, ok, ii, jj, lim, nrelA=0, cap=0, k, hasj);
  hasj=(type(Sj)=="t_VECSMALL");
  lim = if(fixedT>0, fixedT, Tmax);
  while(T<lim && (fixedT>0 || nrelA<W),
    R=elladd(Ec,R,P); T++; k=T;
    x=if(typ==0, lift(R[1]), lift(R[1]*dinv));
    PR=0; codes=e2_find(~Sa,x); pa+=PR;
    if(hasj,
      PR=0; codesj=e2_find(~Sj,x); pj+=PR;
      cj=select(c->(c>=TYPE_EP)!=typ, codesj);
      cross+=#cj;
      same=select(c->(c>=TYPE_EP)==typ, codesj));
    if(#codes>0, hitT++);
    na0=#ra;
    for(u=1,#codes,
      ii=((codes[u]%TYPE_EP)\4096)+1; jj=(codes[u]%4096)+1;
      ok=e2_verify(Et,R,~pts,ii,jj);
      if(ok && chk, ok=(R==ellmul(Ec,P,s0+k)));
      if(ok, listput(ra,k*KB+codes[u]); nrelA++, unA++));
    if(hasj,
      if(Vec(same)==Vec(codes),
        for(u=na0+1,#ra, listput(rj,ra[u])),
        for(u=1,#same, ii=((same[u]%TYPE_EP)\4096)+1; jj=(same[u]%4096)+1;
          if(e2_verify(Et,R,~pts,ii,jj), listput(rj,k*KB+same[u])))));
  );
  if(fixedT==0 && nrelA<W, cap=1);
  [T,pa,pj,nrelA,#rj,hitT,unA,cross,cap,Vecsmall(Vec(ra)),Vecsmall(Vec(rj))]
}

\\ number of group additions for the starting multiplication s0*P (double-and-add)
e2_startcost(s0) = #binary(s0)-1+hammingweight(s0)-1;

\\ ---- union-find / graph descriptives ----------------------------------------
UFP = 0;
e2_uf(x) = { my(r=x); while(UFP[r]!=r, r=UFP[r]); r };
\\ rels: Vecsmall of k*KB+code ; returns [columns touched, components among touched columns, selfloops]
e2_graph(rels,N) = {
  my(touched=vectorsmall(N), nt=0, comps=0, sl=0, ii, jj, ri, rj);
  UFP=vector(N,i,i);
  for(u=1,#rels, ii=(((rels[u]%KB)%TYPE_EP)\4096)+1; jj=((rels[u]%KB)%4096)+1;
    if(!touched[ii], touched[ii]=1; nt++); if(!touched[jj], touched[jj]=1; nt++);
    if(ii==jj, sl++; next);
    ri=e2_uf(ii); rj=e2_uf(jj); if(ri!=rj, UFP[ri]=rj));
  for(i=1,N, if(touched[i] && e2_uf(i)==i, comps++));
  [nt,comps,sl]
}
\\ unsigned incidence-type matrix rank over F_l (descriptive)
e2_rank(rels,N,l) = {
  my(M=matrix(#rels,N), ii, jj);
  for(u=1,#rels, ii=(((rels[u]%KB)%TYPE_EP)\4096)+1; jj=((rels[u]%KB)%4096)+1;
    M[u,ii]=1; M[u,jj]=1);
  matrank(M*Mod(1,l))
}

\\ distinct abscissae in a sorted key array
e2_distinct(S) = { my(n=#S, c=0, last=-1, x); for(i=1,n, x=S[i]\KB; if(x!=last, c++; last=x)); c }
