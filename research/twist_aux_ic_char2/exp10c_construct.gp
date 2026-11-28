\\ EXP10c: construction checks only, n = 6, both s = 1 and s = 2, all 6 ordered pairs, v2-agnostic curves.
read("common.gp");
tovec(x, n) = my(v=Vecrev(x.pol, n)); vector(n, i, Mod(lift(v[i]), x.p));
go(q, ncurves, seed) = {
  setrand(seed); my(n=6, t=ffgen(q^6,'t), one=t^0, bas=vector(n,i,t^(i-1)), gam=ffprimroot(t)^((q^6-1)/(q^3-1)), T=matrix(2,7));
  for(cv=1,ncurves, my(e=vector(3,i,random(t))); if(#Set(e)<3, next);
    my(c=[-(e[1]+e[2]+e[3]), e[1]*e[2]+e[1]*e[3]+e[2]*e[3], -e[1]*e[2]*e[3]], E=ellinit([0,c[1],0,c[2],c[3]],t), j=E.j);
    for(s=1,2, for(i=1,3, for(jj=1,3, if(i==jj, next); my(k=6-i-jj, A=e[i]-e[k], B=e[jj]-e[k], M=matrix(n,n), sol, z, c1, c0);
      for(b=1,n, M[,b] = tovec(B*bas[b] - A^(q^s)*bas[b]^(q^s), n)~);
      sol = iferr(matsolve(M, tovec(gam^(q^s)-gam,n)~), err, 0); if(sol==0, T[s,7]++; next);
      z = sum(b=1,n, lift(sol[b])*bas[b]); if(z==0, T[s,7]++; next);
      c1=1/z; c0=e[k]-c1*gam;
      my(ll=linelift(c,c0,c1,q,one), sq=issquare(c1), ok=(sq && ll[1]==q-ll[2]) || (!sq && ll[1]==0), l9=0);
      if(sq, for(w=0,q-1, my(x=c0+c1*w*one); if(fval(c,x)!=0 && !issquare(x-e[k]), l9++)));
      T[s,] += [1, ok, sq, desc(c,c0,c1,q,2) || desc(c,c0,c1,q,3), (j^(q^2)==j) || (j^(q^3)==j), l9, 0]))));
  for(s=1,2, printf("q=%d s=%d: lines %d, lift rule holds %d, c1 square %d, f_L descends to F_q2/F_q3 %d, j in F_q2/F_q3 %d, Lemma 9 failures %d, singular solves %d\n", q, s, T[s,1],T[s,2],T[s,3],T[s,4],T[s,5],T[s,6],T[s,7]));
}
go(11, 18, 401); go(13, 10, 402);
