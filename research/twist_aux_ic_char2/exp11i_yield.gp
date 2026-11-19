\\ EXP11i: exact mean decompositions per random target R in 2E(K), n = m = 4, via 2-descent classes.
\\ Each FB x-value has class c(x) = ([x-e1 nonsquare], [x-e2 nonsquare]) in (Z/2)^2 = E/2E. A 4-multiset with class sum 0
\\ yields 8 sums up to sign, all in 2E, so E[decomps/target] ~ 8*N0 / (#2E/2) = 64*N0/#E (#E/2E = 4), up to rare coincidences.
read("exp11d_gen.gp");
lineclasses(e, c0, c1, q, one) = { my(n=vector(4)); for(w=0,q-1, my(x=c0+c1*w*one, d=[x-e[1],x-e[2],x-e[3]]);
    if(d[1]*d[2]*d[3]==0 || !issquare(d[1]*d[2]*d[3]), next); n[1+(!issquare(d[1]))+2*(!issquare(d[2]))]++); n; }
N0(n) = { my(s=0); forvec(k=vector(4,i,[0,4]), if(vecsum(k)!=4, next); my(x=0); for(c=1,4, if(k[c]%2, x=bitxor(x,c-1)));
    if(x==0, s+=prod(c=1,4, binomial(n[c]+k[c]-1,k[c])))); s; }
predict(e, c0, c1, q, one, N) = { my(n=lineclasses(e,c0,c1,q,one)); [vecsum(n), 64.*N0(n)/N]; }
\\ find case-P line for curve with roots e over K=F_{q^4}
caseP(e, q, t) = { my(n=4, bas=vector(n,i,t^(i-1)), gam=ffprimroot(t)^((q^4-1)/(q^2-1)));
  for(i=1,3, for(j=1,3, if(i==j, next); my(k=6-i-j, A=e[i]-e[k], B=e[j]-e[k], M=matrix(n,n), sol, z, c1);
    for(b=1,n, M[,b] = tovec(B*bas[b] - A^q*bas[b]^q, n)~);
    sol = iferr(matsolve(M, tovec(gam^q-gam,n)~), err, 0); if(sol==0, next);
    z = sum(b=1,n, lift(sol[b])*bas[b]); if(z==0, next); c1=1/z; if(issquare(c1), return([e[k]-c1*gam, c1]))));
  0; }
