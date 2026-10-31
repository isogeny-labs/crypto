default(parisizemax, "6G");
\\ e-form support of S_{m+1}(x_1..x_m; z) over F_P, m = 2,3,4 (PARI/GP, tooling).
\\ S3 is Semaev's; S4 and S5 by the resultant recursion.  The e-form polynomial T(e_1..e_m)
\\ is fitted by solving a linear system at random points, so every monomial found with a
\\ nonzero coefficient is a certificate that it is present for generic (a, b, z).
P = 2147483647; a = Mod(-3, P); b = Mod(1234567, P);
S3(x,y,z) = (x-y)^2*z^2 - 2*((x+y)*(x*y+a)+2*b)*z + ((x*y-a)^2 - 4*b*(x+y));
S4num(x1,x2,x3,z) = polresultant(S3(x1,x2,'X), S3(x3,z,'X), 'X);
S4X(x3,x4,z) = polresultant(S3(x3,x4,'Y), S3(z,'X,'Y), 'Y);
S5num(x1,x2,x3,x4,z) = polresultant(S3(x1,x2,'X), S4X(x3,x4,z), 'X);
{
mons(m,D) = my(L=List());
  forvec(v=vector(m,i,[0,D]), if(vecsum(v)<=D, listput(L,v)));
  Vec(L);
}
{
esym(xs) = my(m=#xs, p=prod(i=1,m,1+xs[i]*'t));
  vector(m,k,polcoef(p,k,'t));
}
{
fit(m, D, f) = my(Ms=mons(m,D), n=#Ms, A=matrix(n+8,n), B=vector(n+8), xs, e);
  setrand(7);
  for(r=1,n+8, xs=vector(m,i,Mod(random(P),P)); e=esym(xs);
      for(c=1,n, A[r,c]=prod(i=1,m,e[i]^Ms[c][i])); B[r]=f(xs));
  my(sol=matsolve(A[1..n,],B[1..n]~));
  for(r=n+1,n+8, if(sum(c=1,n,A[r,c]*sol[c])!=B[r], error("fit failed: not a polynomial of this support")));
  [Ms,sol];
}
{
report(m, D, f) = my(R=fit(m,D,f), Ms=R[1], sol=R[2], sup=List());
  for(c=1,#Ms, if(sol[c]!=0, listput(sup,Ms[c])));
  my(W=vector(m,j,vecmax(vector(#sup,k,sup[k][j]))), Wmin=vector(m,j,vecmin(vector(#sup,k,sup[k][j]))));
  print("m=",m," D=",D," monomials fitted: ",#Ms," nonzero: ",#sup);
  print("  max exponent per e_j = ",W," ; min exponent per e_j = ",Wmin);
  print("  e_j^D present (j=1..m): ", vector(m,j,sum(k=1,#sup, sup[k]==vector(m,i,if(i==j,D,0)))));
}
zz = Mod(987654321, P);
report(2,2, xs->S3(xs[1],xs[2],zz));
report(3,4, xs->S4num(xs[1],xs[2],xs[3],zz));
report(4,8, xs->S5num(xs[1],xs[2],xs[3],xs[4],zz));
