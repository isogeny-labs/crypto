\\ EXP8f: correct null for P-lines: random symmetric set of the same size drawn from H (x-e_k square),
\\ with the same split between the classes (1,1,1) and (s,s,1). If P matches, the yield gain is fully explained
\\ by (lift density 1) x (index-2 concentration), with no further hidden structure.
read("exp8e_2E.gp");
randH(E, t, e, k, F1, F2) = { my(L=List(), X=Map(), n1=0, n2=0); while(n1<F1 || n2<F2, my(x=random(t), Y=ellordinate(E,x)); if(#Y!=2 || mapisdefined(X,x) || !issquare(x-e[k]), next); my(cl=issquare(x-e[1+(k%3)]));
    if(cl && n1<F1, n1+=2, if(!cl && n2<F2, n2+=2, next)); mapput(X,x,1); listput(L,[x,Y[1]]); listput(L,[x,Y[2]])); Vec(L); }
go2(q, ncurves, seed) = {
  setrand(seed); my(n=4, t=ffgen(q^4,'t), one=t^0, bas=vector(n,i,t^(i-1)), gam=ffprimroot(t)^((q^4-1)/(q^2-1)), done=0, A=vector(3));
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
      my(FB=fbase(E,c,c0,c1,q,one), F1=sum(m=1,#FB, issquare(FB[m][1]-e[1+(k%3)])), a=cov2E(E,FB,e,l,N), b=cov2E(E,randH(E,t,e,k,F1,#FB-F1),e,l,N));
      printf("q=%d F=%d (class split %d/%d) | P cov(2E)=%.3f | randH cov(2E)=%.3f\n", q, #FB, F1, #FB-F1, a[2], b[2]);
      A+=[1,a[2],b[2]]));
    if(got, done++));
  printf("SUMMARY q=%d: P cov(2E)=%.3f  null-in-H cov(2E)=%.3f  ratio=%.3f\n", q, A[2]/A[1], A[3]/A[1], A[2]/A[3]);
}
go2(11, 6, 81); go2(13, 4, 82); go2(17, 3, 83);
