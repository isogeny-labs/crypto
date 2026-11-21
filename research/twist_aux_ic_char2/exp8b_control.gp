\\ EXP8b: control for EXP8 -- coverage of a RANDOM symmetric set of the same size F (F/2 random x with both y's),
\\ plus the standard subfield factor base x in F_q, on the same curves. Isolates structural sum-collapse.
read("exp8_yield.gp");
randsym(E, t, F) = { my(L=List(), X=Map()); while(#L < F, my(x=random(t), Y=ellordinate(E,x)); if(#Y==2 && !mapisdefined(X,x), mapput(X,x,1); listput(L,[x,Y[1]]); listput(L,[x,Y[2]]))); Vec(L); }
ctl(q, ncurves, seed) = {
  setrand(seed); my(n=4, t=ffgen(q^4,'t), one=t^0, bas=vector(n,i,t^(i-1)), gam=ffprimroot(t)^((q^4-1)/(q^2-1)), done=0, A=[0,0,0,0,0,0,0]);
  while(done < ncurves,
    my(e=vector(3,i,random(t)), c, E, N, got=0);
    if(#Set(e)<3, next);
    c=[-(e[1]+e[2]+e[3]), e[1]*e[2]+e[1]*e[3]+e[2]*e[3], -e[1]*e[2]*e[3]];
    E=ellinit([0,c[1],0,c[2],c[3]],t); N=ellcard(E);
    for(i=1,3, if(got,break); for(j=1,3, if(i==j||got, next); my(k=6-i-j, Aa=e[i]-e[k], B=e[j]-e[k], M=matrix(n,n), sol, z, c1, c0);
      for(b=1,n, M[,b] = tovec(B*bas[b] - Aa^q*bas[b]^q, n)~);
      sol = iferr(matsolve(M, tovec(gam^q-gam,n)~), err, 0); if(sol==0, next);
      z = sum(b=1,n, lift(sol[b])*bas[b]); if(z==0, next);
      c1=1/z; c0=e[k]-c1*gam; if(!issquare(c1), next);
      got=1;
      my(FB=fbase(E,c,c0,c1,q,one), rP=cover(E,FB,N), F=rP[1]);
      my(rS=cover(E,randsym(E,t,F),N), rS2=cover(E,randsym(E,t,F),N));
      my(FBq=fbase(E,c,0*one,one,q,one), rQ=cover(E,FBq,N));
      printf("q=%d N=%d F=%d | P-line 2s=%d cov=%.4f | randsym 2s=%d,%d cov=%.4f,%.4f | subfield x in Fq: F=%d cov=%.4f\n",
        q, N, F, rP[2], rP[3]/N, rS[2], rS2[2], rS[3]/N, rS2[3]/N, rQ[1], rQ[3]/N);
      A += [1, rP[3]/N, (rS[3]+rS2[3])/(2*N), rQ[3]/N, rQ[1], 0, 0]));
    if(got, done++));
  printf("SUMMARY q=%d curves=%d: P cov %.4f | random symmetric same-F cov %.4f (P/randsym=%.3f) | subfield FB mean F=%.1f cov %.4f (P/subfield=%.2f)\n",
    q, A[1], A[2]/A[1], A[3]/A[1], A[2]/A[3], A[5]/A[1], A[4]/A[1], A[2]/A[4]);
}
ctl(11, 6, 41); ctl(13, 5, 42); ctl(17, 3, 43);
