\\ EXP12f generator: as exp11d_gen.gp gen(), but the second line per curve is a class-pinned line (type Q), not a random one.
read("exp11d_gen.gp");
pinned(e, q, t) = { my(g=ffprimroot(t)^((q^4-1)/(q^2-1)), a, b, c1); until(a!=b, a=random(q)+random(q)*g; b=random(q)+random(q)*g); c1=(e[2]-e[1])/(a-b); [e[1]+c1*a, c1]; }
genQ(q, ncurves, NT, seed, dir, dobf=1) = {
  setrand(seed); my(n=4, t=ffgen(q^4,'t), one=t^0, bas=vector(n,i,t^(i-1)), gam=ffprimroot(t)^((q^4-1)/(q^2-1)), done=0, man="");
  while(done < ncurves,
    my(e=vector(3,i,random(t)), cf, E, got=0, line=0);
    if(#Set(e)<3, next);
    cf=[-(e[1]+e[2]+e[3]), e[1]*e[2]+e[1]*e[3]+e[2]*e[3], -e[1]*e[2]*e[3]];
    E=ellinit([0,cf[1],0,cf[2],cf[3]],t);
    for(i=1,3, if(got,break); for(j=1,3, if(i==j||got, next); my(k=6-i-j, A=e[i]-e[k], B=e[j]-e[k], M=matrix(n,n), sol, z, c1, c0);
      for(b=1,n, M[,b] = tovec(B*bas[b] - A^q*bas[b]^q, n)~);
      sol = iferr(matsolve(M, tovec(gam^q-gam,n)~), err, 0); if(sol==0, next);
      z = sum(b=1,n, lift(sol[b])*bas[b]); if(z==0, next);
      c1=1/z; c0=e[k]-c1*gam; if(!issquare(c1), next); got=1; line=[c0,c1]));
    if(!got, next); done++;
    my(pq=pinned(e,q,t), lines=[["P", line[1], line[2]], ["Q", pq[1], pq[2]]]);
    for(L=1,2, my(c0=lines[L][2], c1=lines[L][3], pts=linepts(E,c0,c1,q,one));
      if(L==1, my(nl=sum(w=1,q,#pts[w]>0)); if(nl<q, error("case-P line not all-lift: ", nl, "/", q)));
      for(r=1,NT, my(R=ellmul(E,random(E),2)); if(R==[0], next);
        my(S=s5line(cf,c0,c1,R[1]), eqs=descend(S,4), G=vector(4,i,symred(eqs[i],q)), nm=Str(dir, "/c", done, "_", lines[L][1], "_", r, ".ms"));
        writemsS(nm, G, q);
        man = Str(man, nm, " ", lines[L][1], " ", done, " ", if(dobf, bf2(E,pts,q,R), -1), "\n"))));
  write1(Str(dir, "/manifest.txt"), man);
}
