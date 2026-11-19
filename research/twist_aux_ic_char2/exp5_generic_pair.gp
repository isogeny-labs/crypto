\\ EXP5: generic curves over K=F_{q^4} with full 2-torsion (random e1,e2,e3 in K).
\\ Prediction: for each ordered pair (i,j) and the third root k, the line L = c0 + c1 F_q with
\\   c1 = 1/z,  c0 = e_k - c1*gamma,  gamma in F_{q^2}\F_q (one class, take gamma = g2),
\\   where z solves  B z - A^q z^q = gamma^q - gamma,  A = e_i - e_k, B = e_j - e_k,
\\ has (e_j - c0)/c1 = ((e_i - c0)/c1)^q, so g_L is const*square: all-lift if c1 square in K, all-twist otherwise.
\\ Also: no descent to F_{q^2}, and (for all-lift lines) dead count ~ 0.
read("common.gp");
lproj(E, N, l, x) = my(P=ellordinate(E,x)); if(#P==0, -1, ellmul(E,[x,P[1]], N/l)==[0]);
bigl(N) = my(f=factor(N)[,1]); f[#f];
tovec(x, n) = my(v=Vecrev(x.pol, n)); vector(n, i, Mod(lift(v[i]), x.p));
run(q, ncurves, seed) = {
  setrand(seed); my(n=4, t=ffgen(q^4,'t), one=t^0, bas=vector(n,i,t^(i-1)), g2=ffprimroot(t)^((q^4-1)/(q^2-1)), gam,
     T=[0,0,0,0,0,0,0,0], done=0);
  gam = g2; if(gam^q==gam, error("g2 in Fq"));
  while(done < ncurves,
    my(e=vector(3,i,random(t)), c, E, N, l);
    if(#Set(e)<3, next);
    c=[-(e[1]+e[2]+e[3]), e[1]*e[2]+e[1]*e[3]+e[2]*e[3], -e[1]*e[2]*e[3]];
    E=ellinit([0,c[1],0,c[2],c[3]],t); N=ellcard(E); l=bigl(N); if(l<100, next);
    done++;
    for(i=1,3, for(j=1,3, if(i==j, next); my(k=6-i-j, A=e[i]-e[k], B=e[j]-e[k]);
      \\ linear map z -> B z - A^q z^q over F_q
      my(M=matrix(n,n), rhs, sol, z, c1, c0, nl=0, dead=0, good=0);
      for(b=1,n, my(img = B*bas[b] - A^q*bas[b]^q); M[,b] = tovec(img,n)~);
      rhs = tovec(gam^q - gam, n)~;
      sol = iferr(matsolve(M, rhs), err, 0); if(sol==0, T[8]++; next);
      z = sum(b=1,n, lift(sol[b])*bas[b]); if(z==0, T[8]++; next);
      c1 = 1/z; c0 = e[k] - c1*gam;
      my(al=(e[i]-c0)/c1, be=(e[j]-c0)/c1, check = (be == al^q));
      for(w=0,q-1, my(x=c0+c1*w*one); if(fval(c,x)==0,next); good++; my(r=lproj(E,N,l,x)); if(r==-1,next); nl++; if(r,dead++));
      my(sq=issquare(c1), dsc=desc(c,c0,c1,q,2));
      T += [1, check, sq, (sq && nl==good), (!sq && nl==0), dsc, dead, 0];
      ));
  );
  printf("q=%d: %d curves, %d constructed lines (6/curve), pair relation holds %d, c1 square %d -> all-lift %d ; c1 nonsquare -> all-twist %d ; descend to F_q^2: %d ; dead points on all-lift lines: %d ; solve failures %d\n",
     q, ncurves, T[1], T[2], T[3], T[4], T[5], T[6], T[7], T[8]);
}
run(11, 8, 21); run(13, 8, 22); run(17, 6, 23); run(19, 5, 24);
