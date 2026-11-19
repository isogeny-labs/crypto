\\ EXP8e: 2-descent explanation of the P-line coverage shortfall, and target-relevant yield.
\\ Claim: on a case-P line (pair i,j conjugate, third root k), (x-e_i)(x-e_j) = c1^2 (w-al)^(1+q) is a square,
\\ so x-e_k is a square for every FB point: FB lies in the index-2 subgroup H = {class (s,s,1)} of E(K)/2E(K).
\\ Hence all FB sums lie in H, and the odd-order target subgroup (inside 2E(K)) gets ~2x the sum density.
\\ We measure coverage of 2E(K) (contains every odd-order target) for P-lines, random lines, random symmetric sets.
read("exp8_yield.gp");
randsym(E, t, F) = { my(L=List(), X=Map()); while(#L < F, my(x=random(t), Y=ellordinate(E,x)); if(#Y==2 && !mapisdefined(X,x), mapput(X,x,1); listput(L,[x,Y[1]]); listput(L,[x,Y[2]]))); Vec(L); }
in2E(P, e) = if(P==[0], 1, my(x=P[1]); if(P[2]==0, 0, issquare(x-e[1]) && issquare(x-e[2]) && issquare(x-e[3])));
cov2E(E, FB, e, l, N) = {
  my(S2=Map(), L2=List(), S4=Map(), c2=0, cl=0, h=N/l);
  for(i=1,#FB, for(j=i,#FB, my(R=elladd(E,FB[i],FB[j]), k=key(R)); if(!mapisdefined(S2,k), mapput(S2,k,1); listput(L2,R))));
  for(i=1,#L2, for(j=i,#L2, my(R=elladd(E,L2[i],L2[j]), k=key(R)); if(!mapisdefined(S4,k), mapput(S4,k,1); if(in2E(R,e), c2++); if(ellmul(E,R,l)==[0], cl++))));
  [#S4/N, c2/(N/4), cl/l];
}
bigl(N) = my(f=factor(N)[,1]); f[#f];
go(q, ncurves, nr, seed) = {
  setrand(seed); my(n=4, t=ffgen(q^4,'t), one=t^0, bas=vector(n,i,t^(i-1)), gam=ffprimroot(t)^((q^4-1)/(q^2-1)), done=0, A=vector(9), bad=0);
  while(done < ncurves,
    my(e=vector(3,i,random(t)), c, E, N, l, got=0);
    if(#Set(e)<3, next);
    c=[-(e[1]+e[2]+e[3]), e[1]*e[2]+e[1]*e[3]+e[2]*e[3], -e[1]*e[2]*e[3]];
    E=ellinit([0,c[1],0,c[2],c[3]],t); N=ellcard(E); l=bigl(N); if(l<60, next);
    for(i=1,3, if(got,break); for(j=1,3, if(i==j||got, next); my(k=6-i-j, Aa=e[i]-e[k], B=e[j]-e[k], M=matrix(n,n), sol, z, c1, c0);
      for(b=1,n, M[,b] = tovec(B*bas[b] - Aa^q*bas[b]^q, n)~);
      sol = iferr(matsolve(M, tovec(gam^q-gam,n)~), err, 0); if(sol==0, next);
      z = sum(b=1,n, lift(sol[b])*bas[b]); if(z==0, next);
      c1=1/z; c0=e[k]-c1*gam; if(!issquare(c1), next); got=1;
      my(FB=fbase(E,c,c0,c1,q,one));
      for(m=1,#FB, if(!issquare(FB[m][1]-e[k]), bad++));
      my(a=cov2E(E,FB,e,l,N), b=cov2E(E,randsym(E,t,#FB),e,l,N));
      printf("q=%d N=%d l=%d F=%d | P: all=%.3f 2E=%.3f lsub=%.3f | randsym: all=%.3f 2E=%.3f lsub=%.3f\n", q,N,l,#FB,a[1],a[2],a[3],b[1],b[2],b[3]);
      A[1]++; A[2]+=a[2]; A[3]+=b[2]; A[7]+=a[3]; A[8]+=b[3]));
    if(!got, next); done++;
    for(m=1,nr, my(c0=random(t), c1=random(t)); if(c1==0,next);
      my(FB=fbase(E,c,c0,c1,q,one)); if(#FB<6,next); my(a=cov2E(E,FB,e,l,N));
      printf("q=%d R F=%d | all=%.4f 2E=%.4f lsub=%.4f\n", q, #FB, a[1], a[2], a[3]);
      A[4]++; A[5]+=a[2]; A[6]+=#FB; A[9]+=a[3]);
  );
  printf("SUMMARY q=%d: x-e_k nonsquare on P-lines: %d | P cov(2E)=%.3f  randsym-same-F cov(2E)=%.3f | P cov(l-sub)=%.3f randsym=%.3f | R-lines meanF=%.1f cov(2E)=%.4f cov(l-sub)=%.4f | P/R on 2E = %.1f\n",
    q, bad, A[2]/A[1], A[3]/A[1], A[7]/A[1], A[8]/A[1], A[6]/A[4], A[5]/A[4], A[9]/A[4], (A[2]/A[1])/(A[5]/A[4]));
}
if(!defined_8f && !defined_8h, go(11, 5, 4, 71); go(13, 4, 4, 72); go(17, 3, 3, 73));
