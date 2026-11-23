\\ EXP7: distinct l-columns on all-lift lines.  key(w) = x([N/l] P_w)  (identifies +-Q).
\\ A line collapsed by 2-torsion translation would show ~half as many distinct keys as lifting abscissae.
read("common.gp");
bigl(N) = my(f=factor(N)[,1]); f[#f];
keys(E,N,l,xs) = { my(K=List(), z=0, nl=0); foreach(xs, x, my(P=ellordinate(E,x)); if(#P==0, next); nl++;
   my(Q=ellmul(E,[x,P[1]],N/l)); if(Q==[0], z++, listput(K, Q[1]))); [nl, #Set(Vec(K)), z, (#K)*(#K-1)/l*1.]; }
tovec(x, n) = my(v=Vecrev(x.pol, n)); vector(n, i, Mod(lift(v[i]), x.p));
\\ --- odd char, generic case P (Prop 5), n=4
oddP(q, ncurves, seed) = {
  setrand(seed); my(n=4, t=ffgen(q^4,'t), one=t^0, bas=vector(n,i,t^(i-1)), gam=ffprimroot(t)^((q^4-1)/(q^2-1)), S=[0,0,0,0,0.], done=0, jF=0);
  while(done<ncurves, my(e=vector(3,i,random(t)), c, E, N, l);
    if(#Set(e)<3, next); c=[-(e[1]+e[2]+e[3]), e[1]*e[2]+e[1]*e[3]+e[2]*e[3], -e[1]*e[2]*e[3]];
    E=ellinit([0,c[1],0,c[2],c[3]],t); N=ellcard(E); l=bigl(N); if(l<100, next); done++; if(E.j^(q^2)==E.j, jF++);
    for(i=1,3, for(j=1,3, if(i==j,next); my(k=6-i-j, A=e[i]-e[k], B=e[j]-e[k], M=matrix(n,n));
      for(b=1,n, M[,b]=tovec(B*bas[b]-A^q*bas[b]^q,n)~); if(matdet(M)==0, next);
      my(sol=matsolve(M, tovec(gam^q-gam,n)~), z=sum(b=1,n,lift(sol[b])*bas[b]), c1=1/z, c0=e[k]-c1*gam);
      if(!issquare(c1), next);
      my(r=keys(E,N,l, vector(q,w,c0+c1*(w-1)*one))); S += [1, r[1], r[2], r[3], r[4]])));
  printf("odd case P q=%d: %d all-lift lines, %d lifting abscissae, %d distinct l-columns (expected distinct if no collapse: %.1f), %d dead; curves with j in F_q^2: %d/%d\n", q, S[1],S[2],S[3],S[2]-S[4]-S[5],S[4], jF, ncurves);
}
\\ --- char 2, N=15: random valid u (repo construction) vs T-invariant u in b^(1/4) F_Q
bin(trials, seed, tinv) = {
  setrand(seed); my(u=ffgen(2^15,'u), one=u^0, h=ffprimroot(u)^((2^15-1)/31), FQ=vector(31,i,h^i), T=(z->z+z^32+z^1024), S=[0,0,0,0,0.], done=0);
  while(done<trials, my(b, E, N, l, c, v=0, ok=0);
    if(tinv, my(r=0); while(r==0, my(z=random(u)); r=z^32+z); b=r^4; v=r*h^random(31); ok=1,
             b=random(u); if(b==0,next); c=b^(2^14); for(k=1,4000, my(z=random(u)); v=z^32+z; if(v!=0 && T(c/v)==0, ok=1; break)));
    if(!ok, next); E=ellinit([1,0,0,0,b],u); N=ellcard(E); l=bigl(N); if(l<500, next);
    c=b^(2^14); if(T(v)!=0 || T(c/v)!=0, error("invalid u"));
    done++; my(r=keys(E,N,l, vector(31,i,v*FQ[i]))); S += [1, r[1], r[2], r[3], r[4]]);
  printf("char2 %s: %d lines, %d lifting abscissae, %d distinct l-columns (expected distinct if no collapse: %.1f), %d dead\n", if(tinv,"T-INVARIANT u in b^(1/4)F_Q","random valid u"), S[1],S[2],S[3],S[2]-S[4]-S[5],S[4]);
}
\\ sanity: x-action of translation by T=(0,sqrt b) in char 2 is x -> sqrt(b)/x
sanity() = { my(u=ffgen(2^15,'u), b=random(u), E=ellinit([1,0,0,0,b],u), c=b^(2^14), Tp=[0,c], bad=0);
  if(!ellisoncurve(E,Tp), error("T not on curve"));
  for(i=1,200, my(P=random(E)); if(P==[0], next); my(R=elladd(E,P,Tp)); if(R!=[0] && R[1] != c/P[1], bad++));
  printf("sanity char2: x(P+T) == sqrt(b)/x(P) failures: %d/200\n", bad); }
sanity(); oddP(11,6,31); oddP(13,6,32); oddP(17,4,33); bin(6,41,0); bin(6,42,1);
