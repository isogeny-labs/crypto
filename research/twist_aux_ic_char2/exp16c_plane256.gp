\\ EXP16c: Q = 256 (and 512) check of the predicted lift-dense affine lines, target direction only (the full plane
\\ enumeration of exp16c_plane.gp is too slow in gp at Q = 256). Direction u0 = sqrt(b)/tau, tau = sqrt b + sqrt b^(Q^2);
\\ all Q cosets c0 = c*v0 (v0 = sqrt(b)*h2 not in u0*F_Q), actual points: lift count and class (#E/2)P.
cls2(E, NE, P) = ellmul(E, P, NE/2) != [0];
run(d, seed, ta) = {
  setrand(seed); my(N=4*d, Q=2^d, t=ffgen(2^N,'t), one=t^0, g=ffprimroot(t), h=g^((2^N-1)/(Q-1)),
    h2=g^((2^N-1)/(Q^2-1)), FQ=concat([0*one], vector(Q-1,i,h^i)), T=(z->sum(i=0,3,z^(Q^i))), Tr=(z->lift(trace(z))));
  my(b, a, sb, tau, E, NE, u0, v0, res=List());
  until(b!=0 && tau!=0, b=random(t); sb=sqrt(b); tau=sb+sb^(Q^2));
  if(ta==0, a=0*one, until(Tr(a)==1, a=random(t)));
  E=ellinit([1,a,0,0,b],t); NE=ellcard(E); u0=sb/tau; v0=sb*h2; if(T(u0)!=0, error("T(u0)"));
  if(((v0/u0)^Q)==v0/u0, v0=sb);
  my(nfull=0, ok=1, other=Map());
  for(ci=2,Q, my(c0=FQ[ci]*v0, n=[0,0]);
    for(wi=1,Q, my(x=c0+u0*FQ[wi], Y=ellordinate(E,x)); if(#Y, n[cls2(E,NE,[x,Y[1]])+1]++));
    if(n==[Q,0], nfull++; if(Tr(c0)!=Tr(a), ok=0), if(Tr(c0)==Tr(a), ok=0); if(vecsum(n)!=0, ok=0)));
  printf("d=%d Q=%d Tr(a)=%d #E=%d: affine cosets of u0*F_Q in sqrt(b)F_{Q^2}: %d with all Q points lifting in 2E (predicted %d), others have 0 lifts: %d\n",
    d, Q, Tr(a), NE, nfull, if(Tr(a), Q/2, Q/2-1), ok);
}
run(8, 1701, 0); run(8, 1702, 1); run(9, 1703, 0); run(9, 1704, 1);
