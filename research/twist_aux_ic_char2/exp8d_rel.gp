\\ EXP8d: where does the P-line sum collapse come from? Count, per FB (up to sign, x-distinct points):
\\ distinct 3-sums, number of 3-term zero relations (x1,x2,x3 on L with S3=0), 4-term zero relations (S4=0 among x's).
read("exp8_yield.gp");
randsym(E, t, F) = { my(L=List(), X=Map()); while(#L < F, my(x=random(t), Y=ellordinate(E,x)); if(#Y==2 && !mapisdefined(X,x), mapput(X,x,1); listput(L,[x,Y[1]]); listput(L,[x,Y[2]]))); Vec(L); }
stats(E, FB) = {
  my(F=#FB, S3=Map(), z3=0, z4=0, S2=Map(), L2=List());
  for(i=1,F, for(j=i,F, my(R=elladd(E,FB[i],FB[j])); if(R==[0], next); my(k=key(R)); if(!mapisdefined(S2,k), mapput(S2,k,List())); my(v=mapget(S2,k)); listput(v,[i,j]); mapput(S2,k,v)));
  \\ 4-term zero relations: P_i+P_j = -(P_k+P_l) with {i,j}!={k,l}; count unordered pairs of pairs
  for(i=1,F, for(j=i,F, my(R=elladd(E,FB[i],FB[j])); if(R==[0],next); my(k=key(ellneg(E,R))); if(mapisdefined(S2,k), z4 += #mapget(S2,k))));
  for(i=1,F, for(j=i,F, for(l=j,F, my(R=elladd(E,elladd(E,FB[i],FB[j]),FB[l])); if(R==[0], z3++, my(k=key(R)); if(!mapisdefined(S3,k), mapput(S3,k,1))))));
  [F, #S3, binomial(F+2,3), z3, z4/2];
}
go(q, ncurves, seed) = {
  setrand(seed); my(n=4, t=ffgen(q^4,'t), one=t^0, bas=vector(n,i,t^(i-1)), gam=ffprimroot(t)^((q^4-1)/(q^2-1)), done=0);
  while(done < ncurves,
    my(e=vector(3,i,random(t)), c, E, N, got=0);
    if(#Set(e)<3, next);
    c=[-(e[1]+e[2]+e[3]), e[1]*e[2]+e[1]*e[3]+e[2]*e[3], -e[1]*e[2]*e[3]];
    E=ellinit([0,c[1],0,c[2],c[3]],t); N=ellcard(E);
    for(i=1,3, if(got,break); for(j=1,3, if(i==j||got, next); my(k=6-i-j, Aa=e[i]-e[k], B=e[j]-e[k], M=matrix(n,n), sol, z, c1, c0);
      for(b=1,n, M[,b] = tovec(B*bas[b] - Aa^q*bas[b]^q, n)~);
      sol = iferr(matsolve(M, tovec(gam^q-gam,n)~), err, 0); if(sol==0, next);
      z = sum(b=1,n, lift(sol[b])*bas[b]); if(z==0, next);
      c1=1/z; c0=e[k]-c1*gam; if(!issquare(c1), next); got=1;
      my(FB=fbase(E,c,c0,c1,q,one), a=stats(E,FB), b=stats(E,randsym(E,t,#FB)));
      printf("q=%d F=%d | P-line: 3sums %d/%d  zero3 %d  zero4 %d | randsym: 3sums %d/%d zero3 %d zero4 %d\n", q, a[1], a[2],a[3],a[4],a[5], b[2],b[3],b[4],b[5])));
    if(got, done++));
}
go(11, 4, 61); go(13, 3, 62);
