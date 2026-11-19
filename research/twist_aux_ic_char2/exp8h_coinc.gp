\\ EXP8h: are the low-coverage outliers chance? Count nontrivial 4-term coincidences P1-P2 = P3-P4 (difference
\\ multiplicity > 2) in every all-lift case-P FB vs a random set of the same size and class split drawn from H.
\\ Also report cov(2E) for the lines with 0 vs >0 excess coincidences.
read("exp8_yield.gp");
read("exp8e_2E.gp");
randH(E, t, e, k, F1, F2) = { my(L=List(), X=Map(), n1=0, n2=0); while(n1<F1 || n2<F2, my(x=random(t), Y=ellordinate(E,x)); if(#Y!=2 || mapisdefined(X,x) || !issquare(x-e[k]), next); my(cl=issquare(x-e[1+(k%3)]));
    if(cl && n1<F1, n1+=2, if(!cl && n2<F2, n2+=2, next)); mapput(X,x,1); listput(L,[x,Y[1]]); listput(L,[x,Y[2]])); Vec(L); }
excess(E, FB) = { my(D=Map(), ex=0); for(i=1,#FB, for(k=1,#FB, if(i==k,next); my(R=elladd(E,FB[i],ellneg(E,FB[k]))); if(R==[0],next); my(kk=key(R)); mapput(D,kk,if(mapisdefined(D,kk),mapget(D,kk)+1,1))));
  foreach(D, z, my(m=z[1][2]); if(m>2, ex+=m-2)); ex; }
go3(q, ncurves, seed) = {
  setrand(seed); my(n=4, t=ffgen(q^4,'t), one=t^0, bas=vector(n,i,t^(i-1)), gam=ffprimroot(t)^((q^4-1)/(q^2-1)), done=0,
    EP=List(), ER=List(), C0=List(), C1=List());
  while(done < ncurves,
    my(e=vector(3,i,random(t)), c, E, N, l);
    if(#Set(e)<3, next);
    c=[-(e[1]+e[2]+e[3]), e[1]*e[2]+e[1]*e[3]+e[2]*e[3], -e[1]*e[2]*e[3]];
    E=ellinit([0,c[1],0,c[2],c[3]],t); N=ellcard(E); if(valuation(N,2)!=2, next); l=bigl(N); done++;
    for(i=1,3, for(j=1,3, if(i==j, next); my(k=6-i-j, Aa=e[i]-e[k], B=e[j]-e[k], M=matrix(n,n), sol, z, c1, c0);
      for(b=1,n, M[,b] = tovec(B*bas[b] - Aa^q*bas[b]^q, n)~);
      sol = iferr(matsolve(M, tovec(gam^q-gam,n)~), err, 0); if(sol==0, next);
      z = sum(b=1,n, lift(sol[b])*bas[b]); if(z==0, next);
      c1=1/z; c0=e[k]-c1*gam; if(!issquare(c1), next);
      my(FB=fbase(E,c,c0,c1,q,one), F1=sum(m=1,#FB, issquare(FB[m][1]-e[1+(k%3)])), R=randH(E,t,e,k,F1,#FB-F1));
      my(xp=excess(E,FB), xr=excess(E,R), cv=cov2E(E,FB,e,l,N)[2]);
      listput(EP,xp); listput(ER,xr); if(xp==0, listput(C0,cv), listput(C1,[xp,cv]))));
  );
  my(mp=vecsum(Vec(EP))/#EP, mr=vecsum(Vec(ER))/#ER);
  printf("q=%d (v2(N)=2 only): %d case-P lines. excess coincidences: P mean %.2f (max %d), null-in-H mean %.2f (max %d)\n", q, #EP, mp, vecmax(Vec(EP)), mr, vecmax(Vec(ER)));
  printf("   cov(2E) on lines with 0 excess: mean %.3f over %d ; lines with excess [excess, cov]: %Ps\n", if(#C0, vecsum(Vec(C0))/#C0, 0), #C0, Vec(C1));
  printf("   P excess histogram %Ps ; null histogram %Ps\n", vecsort(Vec(EP)), vecsort(Vec(ER)));
}
go3(11, 8, 91);
