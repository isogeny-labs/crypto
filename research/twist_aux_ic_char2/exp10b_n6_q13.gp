\\ EXP10b: n = 6 at q = 13, plus per-line trial counts (columns / coverage) so the random-line baseline can be
\\ read as mean, median and best-of-sample.
defined_10b=1; read("exp10_n6.gp");
go10b(q, ncurves, nR, seed) = {
  setrand(seed); my(n=6, t=ffgen(q^6,'t), one=t^0, bas=vector(n,i,t^(i-1)), gam=ffprimroot(t)^((q^6-1)/(q^3-1)), done=0, TP=List(), TR=List(), FR=List(), NP=List());
  while(done < ncurves,
    my(e=vector(3,i,random(t)), c, E, N, got=0);
    if(#Set(e)<3, next);
    c=[-(e[1]+e[2]+e[3]), e[1]*e[2]+e[1]*e[3]+e[2]*e[3], -e[1]*e[2]*e[3]];
    E=ellinit([0,c[1],0,c[2],c[3]],t); N=ellcard(E); if(valuation(N,2)!=2, next);
    for(s=1,2, for(i=1,3, for(j=1,3, if(i==j || got, next); my(k=6-i-j, Aa=e[i]-e[k], B=e[j]-e[k], M=matrix(n,n), sol, z, c1, c0);
      for(b=1,n, M[,b] = tovec(B*bas[b] - Aa^(q^s)*bas[b]^(q^s), n)~);
      sol = iferr(matsolve(M, tovec(gam^(q^s)-gam,n)~), err, 0); if(sol==0, next);
      z = sum(b=1,n, lift(sol[b])*bas[b]); if(z==0, next);
      c1=1/z; c0=e[k]-c1*gam; if(!issquare(c1), next);
      my(FB=fbx(E, vector(q,w,c0+c1*(w-1)*one))); if(#FB!=2*q, next); got=1;
      my(nv=vector(4)); for(m=1,#FB, nv[cls(FB[m][1],e)+1]++);
      my(r=cov6(E,FB,e), nl=cov6(E,nullset(E,t,e,nv),e), cv=r[3]/(N/4.));
      printf("q=%d N=%d P-line F=%d cov(2E)=%.4f null-in-H=%.4f trials=%.1f\n", q, N, r[1], cv, nl[3]/(N/4.), (r[1]/2)/cv);
      listput(TP, (r[1]/2)/cv); listput(NP, cv/(nl[3]/(N/4.))))));
    if(!got, next); done++;
    for(m=1,nR, my(c0=random(t), c1=random(t)); if(c1==0,next);
      my(FB=fbx(E, vector(q,w,c0+c1*(w-1)*one))); if(#FB<4, next); my(r=cov6(E,FB,e), cv=r[3]/(N/4.));
      printf("q=%d R-line F=%d cov(2E)=%.5f trials=%.1f\n", q, r[1], cv, (r[1]/2)/cv);
      listput(TR, (r[1]/2)/cv); listput(FR, r[1]));
  );
  my(tp=vecsort(Vec(TP)), tr=vecsort(Vec(TR)), md(v)=v[(#v+1)\2]);
  printf("SUMMARY q=%d n=6: P trials median %.1f (P/null-in-H mean %.3f) | R lines %d, F range [%d,%d], trials median %.1f best %.1f | ratio vs median R %.1f, vs best R %.1f\n",
    q, md(tp), vecsum(Vec(NP))/#NP, #tr, vecmin(Vec(FR)), vecmax(Vec(FR)), md(tr), tr[1], md(tr)/md(tp), tr[1]/md(tp));
}
go10b(13, 3, 8, 301);
