\\ EXP13: char-2 analogue of Exp 12. E: y^2 + xy = x^3 + a x^2 + b over K = F_{2^N}, N = 4d, Q = 2^d, factor-base
\\ lines u*F_Q. E(K)[2] = Z/2, so E/2E = Z/2 and (prediction) class(P) = Tr(x_P) + Tr(a), tested here via (#E/2)P == O.
\\ Line types: D = lift-dense (T(u) = T(sqrt(b)/u) = 0, not T-invariant), P1 = pinned (T(u) = 0 only),
\\ P2 = pinned (T(sqrt(b)/u) = 0 only), R = random u. T = Tr_{K/F_Q}.
cls2(E, N, P) = ellmul(E, P, N/2) != [0];   \\ 1 iff P not in 2E (2-part cyclic since E[2](K) = Z/2)
key2(P) = if(P==[0], [-1], [P[1].pol, P[2].pol]);
\\ multisets {i<=j<=k<=l} of line x-values, some signed choice of lifts summing to +-R
bf2c(E, pts, R) = { my(M=Map(), seen=Map(), cnt=0, q=#pts);
  for(a=1,q, for(b=a,q, foreach(pts[a],P1, foreach(pts[b],P2, my(k=key2(elladd(E,P1,P2)), v);
    if(mapisdefined(M,k,&v), mapput(M,k,concat(v,[[a,b]])), mapput(M,k,[[a,b]]))))));
  for(c=1,q, for(d=c,q, foreach(pts[c],P3, foreach(pts[d],P4, my(S=elladd(E,P3,P4));
    foreach([elladd(E,R,ellneg(E,S)), elladd(E,ellneg(E,R),ellneg(E,S))], T, my(v);
      if(mapisdefined(M,key2(T),&v), foreach(v, ab, my(m=vecsort([ab[1],ab[2],c,d]));
        if(!mapisdefined(seen,m), mapput(seen,m,1); cnt++))))))));
  cnt; }
N0c(n) = sum(k=0,4, if(k%2, 0, binomial(n[1]+3-k,4-k)*binomial(n[2]+k-1,k)));  \\ class-1 count even
run(d, ncurves, NT, seed) = {
  setrand(seed); my(N=4*d, Q=2^d, t=ffgen(2^N,'t), one=t^0, h=ffprimroot(t)^((2^N-1)/(Q-1)),
    FQ=vector(Q-1,i,h^i), T=(z->sum(i=0,3,z^(Q^i))), done=0, tries=0, bad=0);
  while(done < ncurves, my(a=0, b=random(t), E, NE, sb, us=Map(), z, u);
    if(b==0, next); tries++;
    E=ellinit([1,a,0,0,b],t); NE=ellcard(E); sb=sqrt(b);
    my(D=0, P1=0, P2=0, R=0);
    for(k=1,20000, z=random(t); u=z^Q+z; if(u==0, next);
      if(!D && T(sb/u)==0 && (sb/u^2)^Q != sb/u^2, D=u);
      if(!P1 && T(sb/u)!=0, P1=u));
    if(!D, next); done++;
    for(k=1,20000, u=random(t); if(u!=0 && T(u)!=0 && T(sb/u)==0, P2=u; break));
    R=random(t);
    my(L=[["D",D],["P1",P1],["P2",P2],["R",R]]);
    printf("d=%d N=%d curve %d (after %d tries): #E=%d v2=%d\n", d, N, done, tries, NE, valuation(NE,2));
    foreach(L, ln, if(ln[2]==0, printf("   %s: none found\n", ln[1]); next);
      my(pts=vector(Q-1,i, my(x=ln[2]*FQ[i], Y=ellordinate(E,x)); vector(#Y,j,[x,Y[j]])), n=[0,0], s=0, hits=0);
      for(i=1,Q-1, if(#pts[i], my(c=cls2(E,NE,pts[i][1])); n[c+1]++;
        if(c != (trace(pts[i][1][1]) + trace(a*one)) % 2, bad++)));
      if(NT, for(r=1,NT, my(Rt=ellmul(E,random(E),2)); if(Rt==[0], next); s+=bf2c(E,pts,Rt)));
      printf("   %s: lifts %d/%d, classes [in 2E, not] = %s | whole-line yield pred 32*N0/#E = %.4f%s\n", ln[1],
        vecsum(n), Q-1, n, 32.*N0c(n)/NE, if(NT, Strprintf(", measured %.4f (%d targets)", s*1./NT, NT), ""))));
  printf("d=%d: class-criterion mismatches (Tr(x)+Tr(a) vs (#E/2)P) = %d\n", d, bad);
}
run(5, 4, 150, 501);
\\ run(7, 4, 0, 701);  (logged in results/exp13_char2.log)
\\ run(8, 3, 0, 801);
