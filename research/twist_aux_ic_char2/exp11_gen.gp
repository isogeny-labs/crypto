\\ EXP11 generator: per curve (q=11, n=4, full 2-torsion), one all-lift case-P line and one random line;
\\ per line, NT targets R = 2*random point. Writes symmetrised descended S5 systems for msolve, plus a manifest
\\ with the brute-force count of decompositions (unordered 4-multisets of x on the line whose lifts sum to +-R).
read("exp11_descent.gp");
tovec(x, n) = my(v=Vecrev(x.pol, n)); vector(n, i, Mod(lift(v[i]), x.p));
key(P) = if(P==[0], [-1], concat(apply(z->Vecrev(lift(z.pol),4), P)));
\\ brute force: count multisets {w1<=..<=w4} with signs giving R
bfcount(E, c0, c1, q, one, R) = {
  my(pts=vector(q, w, my(x=c0+c1*(w-1)*one, Y=ellordinate(E,x)); vector(#Y, i, [x,Y[i]])), cnt=0);
  forvec(W=vector(4,i,[1,q]), my(ok=0, P=vector(4,i,pts[W[i]])); if(vecmin(apply(length,P))==0, next);
    forvec(sg=vector(4,i,[1,#P[i]]), my(S=[0]); for(i=1,4, S=elladd(E,S,P[i][sg[i]])); if(S==R || S==ellneg(E,R), ok=1; break));
    cnt+=ok, 1);
  cnt;
}
gen(q, ncurves, NT, seed) = {
  setrand(seed); p_glob=q; my(n=4, t=ffgen(q^4,'t), one=t^0, bas=vector(n,i,t^(i-1)), gam=ffprimroot(t)^((q^4-1)/(q^2-1)), done=0, man="");
  while(done < ncurves,
    my(e=vector(3,i,random(t)), cf, E, N, got=0, line=0);
    if(#Set(e)<3, next);
    cf=[-(e[1]+e[2]+e[3]), e[1]*e[2]+e[1]*e[3]+e[2]*e[3], -e[1]*e[2]*e[3]];
    E=ellinit([0,cf[1],0,cf[2],cf[3]],t); N=ellcard(E);
    for(i=1,3, if(got,break); for(j=1,3, if(i==j||got, next); my(k=6-i-j, A=e[i]-e[k], B=e[j]-e[k], M=matrix(n,n), sol, z, c1, c0);
      for(b=1,n, M[,b] = tovec(B*bas[b] - A^q*bas[b]^q, n)~);
      sol = iferr(matsolve(M, tovec(gam^q-gam,n)~), err, 0); if(sol==0, next);
      z = sum(b=1,n, lift(sol[b])*bas[b]); if(z==0, next);
      c1=1/z; c0=e[k]-c1*gam; if(!issquare(c1), next); got=1; line=[c0,c1]));
    if(!got, next); done++;
    my(lines=[["P", line[1], line[2]], ["R", random(t), random(t)]]);
    for(L=1,2, for(r=1,NT, my(R=ellmul(E,random(E),2)); if(R==[0], next);
      my(c0=lines[L][2], c1=lines[L][3], S=s5line(cf,c0,c1,R[1]), eqs=descend(S,4), G=vector(4,i,symred(eqs[i],q)), nm=Str("msolve_runs/c", done, "_", lines[L][1], "_", r, ".ms"));
      writemsS(nm, G, q);
      man = Str(man, nm, " ", lines[L][1], " ", done, " ", bfcount(E,c0,c1,q,one,R), "\n"))));
  write1("msolve_runs/manifest.txt", man);
}
gen(11, 3, 4, 501);
