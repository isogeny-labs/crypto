\\ EXP8c: (a) is the ~25% shortfall vs random symmetric sets specific to case-P lines, or common to all
\\ x-line factor bases?  Compare random lines R and subfield FB against random symmetric sets of the SAME F.
\\ (b) 2-torsion translation closure: for each FB, count P in FB with P+T in FB, T in E[2].
read("exp8_yield.gp");
randsym(E, t, F) = { my(L=List(), X=Map()); while(#L < F, my(x=random(t), Y=ellordinate(E,x)); if(#Y==2 && !mapisdefined(X,x), mapput(X,x,1); listput(L,[x,Y[1]]); listput(L,[x,Y[2]]))); Vec(L); }
tclose(E, FB, e) = { my(K=Map(), r=vector(3)); for(i=1,#FB, mapput(K,key(FB[i]),1)); for(s=1,3, my(T=[e[s],0*e[s]]); for(i=1,#FB, if(mapisdefined(K,key(elladd(E,FB[i],T))), r[s]++))); r; }
diag(q, ncurves, nr, seed) = {
  setrand(seed); my(n=4, t=ffgen(q^4,'t), one=t^0, bas=vector(n,i,t^(i-1)), gam=ffprimroot(t)^((q^4-1)/(q^2-1)), done=0, A=vector(6));
  while(done < ncurves,
    my(e=vector(3,i,random(t)), c, E, N);
    if(#Set(e)<3, next);
    c=[-(e[1]+e[2]+e[3]), e[1]*e[2]+e[1]*e[3]+e[2]*e[3], -e[1]*e[2]*e[3]];
    E=ellinit([0,c[1],0,c[2],c[3]],t); N=ellcard(E); done++;
    \\ all case-P all-lift lines on this curve (up to 6), with T-closure
    for(i=1,3, for(j=1,3, if(i==j, next); my(k=6-i-j, Aa=e[i]-e[k], B=e[j]-e[k], M=matrix(n,n), sol, z, c1, c0);
      for(b=1,n, M[,b] = tovec(B*bas[b] - Aa^q*bas[b]^q, n)~);
      sol = iferr(matsolve(M, tovec(gam^q-gam,n)~), err, 0); if(sol==0, next);
      z = sum(b=1,n, lift(sol[b])*bas[b]); if(z==0, next);
      c1=1/z; c0=e[k]-c1*gam; if(!issquare(c1), next);
      my(FB=fbase(E,c,c0,c1,q,one), r=cover(E,FB,N), s=cover(E,randsym(E,t,#FB),N));
      printf("q=%d P(i=%d,j=%d) F=%d cov=%.4f randsym=%.4f ratio=%.3f Tclose=%Ps\n", q,i,j,#FB, r[3]/N, s[3]/N, r[3]/s[3], tclose(E,FB,e));
      A[1]++; A[2]+=r[3]/s[3]));
    for(m=1,nr, my(c0=random(t), c1=random(t)); if(c1==0,next);
      my(FB=fbase(E,c,c0,c1,q,one)); if(#FB<8, next);
      my(r=cover(E,FB,N), s=cover(E,randsym(E,t,#FB),N));
      printf("q=%d R F=%d cov=%.4f randsym=%.4f ratio=%.3f Tclose=%Ps\n", q,#FB, r[3]/N, s[3]/N, r[3]/s[3], tclose(E,FB,e));
      A[3]++; A[4]+=r[3]/s[3]);
  );
  printf("SUMMARY q=%d: P-lines %d mean cov/randsym %.3f | R-lines %d mean cov/randsym %.3f\n", q, A[1], A[2]/A[1], A[3], A[4]/A[3]);
}
diag(11, 4, 5, 51); diag(13, 3, 5, 52);
