\\ EXP11f: does S5 restricted to a case-P line factor over K (structure that could explain non-generic ideals)?
\\ Restrict to random lines in (w3,w4) with w1,w2 fixed; factor over K; a consistent split (e.g. [8,8]) means S factors.
read("exp11d_gen.gp");
pat(S, q, NS) = { my(M=Map(), sq=0);
  \\ restrict to a random line (w1,w2 fixed; w3=a+b*u, w4=c+d*u): univariate in u, no elliptic-curve root structure
  for(i=1,NS, my(f=substvec(S,[w1,w2,w3,w4],[random(q),random(q),random(q)+random(q)*'u,random(q)+random(q)*'u]), F, k);
    if(f==0 || poldegree(f,'u)<1, next); F=factor(f); if(vecmax(F[,2])>1, sq++);
    k=Str(vecsort(apply(g->poldegree(g,'u), F[,1]~))); mapput(M,k,if(mapisdefined(M,k),mapget(M,k),0)+1));
  [Mat(M), sq]; }
run(q, ncurves, seed) = {
  setrand(seed); my(n=4, t=ffgen(q^4,'t), one=t^0, bas=vector(n,i,t^(i-1)), gam=ffprimroot(t)^((q^4-1)/(q^2-1)), done=0);
  while(done < ncurves,
    my(e=vector(3,i,random(t)), cf, E, got=0, line=0);
    if(#Set(e)<3, next);
    cf=[-(e[1]+e[2]+e[3]), e[1]*e[2]+e[1]*e[3]+e[2]*e[3], -e[1]*e[2]*e[3]]; E=ellinit([0,cf[1],0,cf[2],cf[3]],t);
    for(i=1,3, if(got,break); for(j=1,3, if(i==j||got, next); my(k=6-i-j, A=e[i]-e[k], B=e[j]-e[k], M=matrix(n,n), sol, z, c1, c0);
      for(b=1,n, M[,b] = tovec(B*bas[b] - A^q*bas[b]^q, n)~);
      sol = iferr(matsolve(M, tovec(gam^q-gam,n)~), err, 0); if(sol==0, next);
      z = sum(b=1,n, lift(sol[b])*bas[b]); if(z==0, next);
      c1=1/z; c0=e[k]-c1*gam; if(!issquare(c1), next); got=1; line=[c0,c1]));
    if(!got, next); done++;
    my(R=ellmul(E,random(E),2));
    foreach([["P",line[1],line[2]],["R",random(t),random(t)]], L, my(S=s5line(cf,L[2],L[3],R[1]), r=pat(S,q,40));
      print("q=",q," curve ",done," ",L[1],"  u-line factor degrees (count): ", r[1], "  with repeated factor: ", r[2], "/40")));
}
run(43, 3, 777);
