\\ EXP8: measured 4-term decomposition yield (n = m = 4) for case-P all-lift lines vs random lines.
\\ K=F_{q^4}, curves with full 2-torsion. FB(L) = all points of E(K) with x in L = c0 + c1 F_q (both signs).
\\ Yield = fraction of E(K) expressible as P1+P2+P3+P4 with Pi in FB (repetition allowed).
\\ Model: random sums -> coverage 1-exp(-M/#E), M = number of 4-multisets of FB modulo the
\\ (P1..P4) -> (-P1..-P4) pairing is ignored: we use M = C(F+3,4) and compare P vs R lines under the same model.
read("common.gp");
tovec(x, n) = my(v=Vecrev(x.pol, n)); vector(n, i, Mod(lift(v[i]), x.p));
key(P) = if(P==[0], [-1], concat(apply(z->Vecrev(lift(z.pol),4), P)));
fbase(E, c, c0, c1, q, one) = {
  my(L=List()); for(w=0,q-1, my(x=c0+c1*w*one, Y=ellordinate(E,x)); for(i=1,#Y, listput(L,[x,Y[i]])));
  Vec(L);
}
cover(E, FB, N) = {
  my(S2=Map(), L2=List(), S4=Map(), F=#FB);
  for(i=1,F, for(j=i,F, my(R=elladd(E,FB[i],FB[j]), k=key(R)); if(!mapisdefined(S2,k), mapput(S2,k,1); listput(L2,R))));
  for(i=1,#L2, for(j=i,#L2, my(k=key(elladd(E,L2[i],L2[j]))); if(!mapisdefined(S4,k), mapput(S4,k,1))));
  [F, #L2, #S4];
}
model(F, N) = 1.0 - exp(-binomial(F+3,4)/N);
run(q, ncurves, nrand, seed) = {
  setrand(seed); my(n=4, t=ffgen(q^4,'t), one=t^0, bas=vector(n,i,t^(i-1)), gam=ffprimroot(t)^((q^4-1)/(q^2-1)), done=0,
    AP=[0,0,0,0], AR=[0,0,0,0]);
  while(done < ncurves,
    my(e=vector(3,i,random(t)), c, E, N, got=0);
    if(#Set(e)<3, next);
    c=[-(e[1]+e[2]+e[3]), e[1]*e[2]+e[1]*e[3]+e[2]*e[3], -e[1]*e[2]*e[3]];
    E=ellinit([0,c[1],0,c[2],c[3]],t); N=ellcard(E);
    \\ one all-lift case-P line per curve
    for(i=1,3, if(got,break); for(j=1,3, if(i==j||got, next); my(k=6-i-j, A=e[i]-e[k], B=e[j]-e[k], M=matrix(n,n), sol, z, c1, c0);
      for(b=1,n, M[,b] = tovec(B*bas[b] - A^q*bas[b]^q, n)~);
      sol = iferr(matsolve(M, tovec(gam^q-gam,n)~), err, 0); if(sol==0, next);
      z = sum(b=1,n, lift(sol[b])*bas[b]); if(z==0, next);
      c1=1/z; c0=e[k]-c1*gam; if(!issquare(c1), next);
      got=1; my(FB=fbase(E,c,c0,c1,q,one), r=cover(E,FB,N), mo=model(r[1],N));
      printf("q=%d N=%d  P-line: F=%d  2-sums=%d  4-cov=%.4f  model=%.4f\n", q, N, r[1], r[2], r[3]/N, mo);
      AP += [1, r[1], r[3]/N, mo]));
    if(!got, next); done++;
    for(s=1,nrand, my(c0=random(t), c1=random(t)); if(c1==0, next);
      my(FB=fbase(E,c,c0,c1,q,one), r=cover(E,FB,N), mo=model(r[1],N));
      printf("q=%d N=%d  R-line: F=%d  2-sums=%d  4-cov=%.4f  model=%.4f\n", q, N, r[1], r[2], r[3]/N, mo);
      AR += [1, r[1], r[3]/N, mo]);
  );
  printf("SUMMARY q=%d: P-lines %d: mean F=%.2f mean cov=%.4f mean model=%.4f | R-lines %d: mean F=%.2f mean cov=%.4f mean model=%.4f | yield ratio P/R=%.2f\n",
    q, AP[1], AP[2]/AP[1], AP[3]/AP[1], AP[4]/AP[1], AR[1], AR[2]/AR[1], AR[3]/AR[1], AR[4]/AR[1], (AP[3]/AP[1])/(AR[3]/AR[1]));
}
if(!defined_ctl, run(11, 4, 4, 31));
if(!defined_ctl, run(13, 3, 3, 32));
