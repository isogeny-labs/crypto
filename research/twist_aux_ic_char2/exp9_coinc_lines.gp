\\ EXP9: is the excess of 4-term coincidences P1-P2 = P3-P4 (Exp 8h) specific to case-P lines, or common to
\\ every F_q-line factor base?  For each factor base type, compare its excess to a null: random symmetric set of
\\ the same size with the same split over the four 2-descent classes (chi(x-e1), chi(x-e2)).
\\ Types: P = all-lift case-P line; R = random line; U = union of two random lines (size ~ 2q, like P);
\\        Q = random "quadratic curve" x = c0 + c1 w + c2 w^2, w in F_q (non-line control of the same size type).
read("exp8_yield.gp");
excess(E, FB) = { my(D=Map(), ex=0); for(i=1,#FB, for(k=1,#FB, if(i==k,next); my(R=elladd(E,FB[i],ellneg(E,FB[k]))); if(R==[0],next); my(kk=key(R)); mapput(D,kk,if(mapisdefined(D,kk),mapget(D,kk)+1,1))));
  foreach(D, z, my(m=z[1][2]); if(m>2, ex+=m-2)); ex; }
cls(x, e) = 1 + issquare(x-e[1]) + 2*issquare(x-e[2]);
clcount(FB, e) = { my(v=vector(4)); for(i=1,#FB, v[cls(FB[i][1],e)]++); v; }
nullset(E, t, e, v) = { my(L=List(), X=Map(), have=vector(4)); while(have!=v, my(x=random(t), Y=ellordinate(E,x)); if(#Y!=2 || mapisdefined(X,x), next); my(c=cls(x,e)); if(have[c]>=v[c], next);
   have[c]+=2; mapput(X,x,1); listput(L,[x,Y[1]]); listput(L,[x,Y[2]])); Vec(L); }
fbx(E, xs) = { my(L=List(), X=Map()); for(i=1,#xs, my(x=xs[i]); if(mapisdefined(X,x), next); mapput(X,x,1); my(Y=ellordinate(E,x)); if(#Y==2, listput(L,[x,Y[1]]); listput(L,[x,Y[2]]))); Vec(L); }
addr(t, FB, E, e) = { if(#FB<8, return([0,0,0,0,0])); my(v=clcount(FB,e), a=excess(E,FB), b=excess(E,nullset(E,t,e,v))); [1, #FB, a, b, #FB^4]; }
\\ only keep points with two distinct y (drop 2-torsion), on both sides
run9(q, ncurves, nR, nU, nQ, seed) = {
  setrand(seed); my(n=4, t=ffgen(q^4,'t), one=t^0, bas=vector(n,i,t^(i-1)), gam=ffprimroot(t)^((q^4-1)/(q^2-1)), done=0,
    S=vector(4,i,[0,0,0,0,0]), names=["P","R","U","Q"]); \\ type -> [lines, sumF, sumexcess, sumnullexcess, sum F^4]
  while(done < ncurves,
    my(e=vector(3,i,random(t)), c, E, N);
    if(#Set(e)<3, next);
    c=[-(e[1]+e[2]+e[3]), e[1]*e[2]+e[1]*e[3]+e[2]*e[3], -e[1]*e[2]*e[3]];
    E=ellinit([0,c[1],0,c[2],c[3]],t); N=ellcard(E); done++;
    for(i=1,3, for(j=1,3, if(i==j, next); my(k=6-i-j, Aa=e[i]-e[k], B=e[j]-e[k], M=matrix(n,n), sol, z, c1, c0);
      for(b=1,n, M[,b] = tovec(B*bas[b] - Aa^q*bas[b]^q, n)~);
      sol = iferr(matsolve(M, tovec(gam^q-gam,n)~), err, 0); if(sol==0, next);
      z = sum(b=1,n, lift(sol[b])*bas[b]); if(z==0, next);
      c1=1/z; c0=e[k]-c1*gam; if(!issquare(c1), next);
      S[1] += addr(t, fbx(E, vector(q,w,c0+c1*(w-1)*one)), E, e)));
    for(r=1,nR, my(c0=random(t), c1=random(t)); if(c1==0,next); S[2] += addr(t, fbx(E, vector(q,w,c0+c1*(w-1)*one)), E, e));
    for(r=1,nU, my(a0=random(t), a1=random(t), b0=random(t), b1=random(t)); if(a1==0||b1==0,next);
      S[3] += addr(t, fbx(E, concat(vector(q,w,a0+a1*(w-1)*one), vector(q,w,b0+b1*(w-1)*one))), E, e));
    for(r=1,nQ, my(c0=random(t), c1=random(t), c2=random(t)); if(c2==0,next); S[4] += addr(t, fbx(E, vector(q,w,c0+c1*(w-1)*one+c2*(w-1)^2*one)), E, e));
  );
  for(ii=1,4, my(ty=names[ii], v=S[ii]); if(v[1]==0, next);
    printf("q=%d type %s: lines %d  mean F %.1f  excess %d  null excess %d  ratio %.2f  (excess per 1e4 F^4: %.3f vs null %.3f)\n",
      q, ty, v[1], v[2]/v[1], v[3], v[4], if(v[4], v[3]/v[4], -1), 1e4*v[3]/v[5], 1e4*v[4]/v[5]));
}
run9(11, 40, 6, 3, 3, 101);
run9(13, 25, 6, 3, 3, 102);
