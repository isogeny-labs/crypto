\\ EXP10: n = m = 6, where the case-P base is NOT saturated (2^{m+1}/m! = 128/720 ~ 0.18).
\\ K = F_{q^6}. Case-P construction generalised: s in {1,2}, gamma in F_{q^3}\F_q,
\\   B z - A^{q^s} z^{q^s} = gamma^{q^s} - gamma,  c1 = 1/z, c0 = e_k - c1 gamma.
\\ Checks: all-lift iff c1 square; no descent of f_L to F_{q^2} or F_{q^3}; Lemma 9 (x - e_k square).
\\ Yield: distinct 6-sums landing in 2E(K) (curves with v2(N)=2, so 2E(K) = odd part = all targets),
\\ computed as pairs of distinct 3-sums with matching 2-descent class (class map is a homomorphism).
\\ Model: coverage = 1 - exp(-M2E/(N/4)), M2E = number of 6-multisets of FB with class sum 0 (exact count).
read("common.gp");
key(P) = if(P==[0], [-1], concat(apply(z->Vecrev(lift(z.pol),6), P)));
tovec(x, n) = my(v=Vecrev(x.pol, n)); vector(n, i, Mod(lift(v[i]), x.p));
cls(x, e) = issquare(x-e[1]) + 2*issquare(x-e[2]);   \\ 0..3, XOR group (Z/2)^2 ; class 0 = (square, square)
fbx(E, xs) = { my(L=List(), X=Map()); for(i=1,#xs, my(x=xs[i]); if(mapisdefined(X,x), next); mapput(X,x,1); my(Y=ellordinate(E,x)); if(#Y==2, listput(L,[x,Y[1]]); listput(L,[x,Y[2]]))); Vec(L); }
nullset(E, t, e, v) = { my(L=List(), X=Map(), have=vector(4)); while(have!=v, my(x=random(t), Y=ellordinate(E,x)); if(#Y!=2 || mapisdefined(X,x), next); my(c=cls(x,e)+1); if(have[c]>=v[c], next);
   have[c]+=2; mapput(X,x,1); listput(L,[x,Y[1]]); listput(L,[x,Y[2]])); Vec(L); }
\\ exact number of size-6 multisets with class XOR = 0, given class sizes n[1..4] (classes 0..3)
m2e(nv) = { my(s=0); forvec(a=vector(4,i,[0,6]), if(vecsum(a)!=6, next); my(x=bitxor(if(a[2]%2,1,0), if(a[4]%2,3,0)), y=if(a[3]%2,2,0)); if(bitxor(x,y)!=0, next);
   s += prod(c=1,4, binomial(nv[c]+a[c]-1, a[c]))); s; }
cov6(E, FB, e) = {
  my(F=#FB, cl=vector(F,i,cls(FB[i][1],e)), S3=Map(), L3=List(), C3=List(), S6=Map(), n6=0);
  for(i=1,F, my(Pi=FB[i]); for(j=i,F, my(Pij=elladd(E,Pi,FB[j]), cij=bitxor(cl[i],cl[j])); for(k=j,F,
     my(R=elladd(E,Pij,FB[k]), kk=key(R)); if(!mapisdefined(S3,kk), mapput(S3,kk,1); listput(L3,R); listput(C3, bitxor(cij,cl[k]))))));
  my(L=Vec(L3), C=Vec(C3));
  for(a=1,#L, my(ca=C[a], A=L[a]); for(b=a,#L, if(C[b]!=ca, next); my(kk=key(elladd(E,A,L[b]))); if(!mapisdefined(S6,kk), mapput(S6,kk,1); n6++)));
  [F, #L, n6];
}
go10(q, ncurves, nR, seed) = {
  setrand(seed); my(n=6, t=ffgen(q^6,'t), one=t^0, bas=vector(n,i,t^(i-1)), gam=ffprimroot(t)^((q^6-1)/(q^3-1)), done=0,
    chk=[0,0,0,0,0], AP=[0,0,0,0], AR=[0,0,0,0], AN=[0,0]);
  if(gam^q==gam, error("gamma in Fq"));
  while(done < ncurves,
    my(e=vector(3,i,random(t)), c, E, N, got=0);
    if(#Set(e)<3, next);
    c=[-(e[1]+e[2]+e[3]), e[1]*e[2]+e[1]*e[3]+e[2]*e[3], -e[1]*e[2]*e[3]];
    E=ellinit([0,c[1],0,c[2],c[3]],t); N=ellcard(E); if(valuation(N,2)!=2, next);
    for(s=1,2, for(i=1,3, for(j=1,3, if(i==j || got, next); my(k=6-i-j, Aa=e[i]-e[k], B=e[j]-e[k], M=matrix(n,n), sol, z, c1, c0);
      for(b=1,n, M[,b] = tovec(B*bas[b] - Aa^(q^s)*bas[b]^(q^s), n)~);
      sol = iferr(matsolve(M, tovec(gam^(q^s)-gam,n)~), err, 0); if(sol==0, next);
      z = sum(b=1,n, lift(sol[b])*bas[b]); if(z==0, next);
      c1=1/z; c0=e[k]-c1*gam;
      my(ll=linelift(c,c0,c1,q,one), sq=issquare(c1));
      chk += [1, (sq && ll[1]==q-ll[2]) || (!sq && ll[1]==0), desc(c,c0,c1,q,2) || desc(c,c0,c1,q,3), 0, 0];
      if(!sq, next);
      my(FB=fbx(E, vector(q,w,c0+c1*(w-1)*one)));
      for(m=1,#FB, if(!issquare(FB[m][1]-e[k]), chk[4]++));
      got=1;
      my(nv=vector(4)); for(m=1,#FB, nv[cls(FB[m][1],e)+1]++);
      my(r=cov6(E,FB,e), mod=1.0-exp(-m2e(nv)/(N/4.)), nl=cov6(E,nullset(E,t,e,nv),e));
      printf("q=%d N=%d s=%d P-line F=%d classes=%Ps | cov(2E)=%.4f model=%.4f | null-in-H same split cov=%.4f\n", q, N, s, r[1], nv, r[3]/(N/4.), mod, nl[3]/(N/4.));
      AP += [1, r[3]/(N/4.), mod, r[1]/2]; AN += [1, nl[3]/(N/4.)];
    )));
    if(!got, next); done++;
    for(m=1,nR, my(c0=random(t), c1=random(t)); if(c1==0,next);
      my(FB=fbx(E, vector(q,w,c0+c1*(w-1)*one)), nv=vector(4)); for(u=1,#FB, nv[cls(FB[u][1],e)+1]++);
      my(r=cov6(E,FB,e), mod=1.0-exp(-m2e(nv)/(N/4.)));
      printf("q=%d R-line F=%d classes=%Ps | cov(2E)=%.5f model=%.5f\n", q, r[1], nv, r[3]/(N/4.), mod);
      AR += [1, r[3]/(N/4.), mod, r[1]/2]);
  );
  printf("CHECKS q=%d: constructed %d, lift rule (all-lift iff c1 square) holds %d, descend to F_q2 or F_q3: %d, Lemma 9 failures: %d\n", q, chk[1], chk[2], chk[3], chk[4]);
  my(tP=(AP[4]/AP[1])/(AP[2]/AP[1]), tR=(AR[4]/AR[1])/(AR[2]/AR[1]));
  printf("SUMMARY q=%d n=6: P %d lines cov %.4f (model %.4f; null-in-H %.4f, P/null %.3f) | R %d lines mean F %.1f cov %.5f (model %.5f) | trials per relation P %.1f R %.1f -> ratio %.1f\n",
    q, AP[1], AP[2]/AP[1], AP[3]/AP[1], AN[2]/AN[1], (AP[2]/AP[1])/(AN[2]/AN[1]), AR[1], 2*AR[4]/AR[1], AR[2]/AR[1], AR[3]/AR[1], tP, tR, tR/tP);
}
if(!defined_10b, go10(11, 3, 4, 201));
