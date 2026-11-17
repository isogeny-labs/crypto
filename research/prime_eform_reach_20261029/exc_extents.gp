default(parisizemax, "6G");
\\ e-form extents at exceptional z (PARI/GP, tooling): full fit at the given z, as in eform_support.gp.
P = 3221225473; a = Mod(-3, P); b = Mod(1234567, P);
S3(x,y,z) = (x-y)^2*z^2 - 2*((x+y)*(x*y+a)+2*b)*z + ((x*y-a)^2 - 4*b*(x+y));
S4num(x1,x2,x3,z) = polresultant(S3(x1,x2,'X), S3(x3,z,'X), 'X);
S4X(x3,x4,z) = polresultant(S3(x3,x4,'Y), S3(z,'X,'Y), 'Y);
S5num(x1,x2,x3,x4,z) = polresultant(S3(x1,x2,'X), S4X(x3,x4,z), 'X);
{ mons(m,D) = my(L=List()); forvec(v=vector(m,i,[0,D]), if(vecsum(v)<=D, listput(L,v))); Vec(L); }
{ esym(xs) = my(m=#xs, p=prod(i=1,m,1+xs[i]*'t)); vector(m,k,polcoef(p,k,'t)); }
{
fit(m, D, f) = my(Ms=mons(m,D), n=#Ms, A=matrix(n+8,n), B=vector(n+8), xs, e);
  setrand(7);
  for(r=1,n+8, xs=vector(m,i,Mod(random(P),P)); e=esym(xs);
      for(c=1,n, A[r,c]=prod(i=1,m,e[i]^Ms[c][i])); B[r]=f(xs));
  my(sol=matsolve(A[1..n,],B[1..n]~));
  for(r=n+1,n+8, if(sum(c=1,n,A[r,c]*sol[c])!=B[r], error("fit failed")));
  [Ms,sol];
}
{
report(lab, m, D, f) = my(R=fit(m,D,f), Ms=R[1], sol=R[2], sup=List());
  for(c=1,#Ms, if(sol[c]!=0, listput(sup,Ms[c])));
  my(W=vector(m,j,vecmax(vector(#sup,k,sup[k][j]))-vecmin(vector(#sup,k,sup[k][j]))));
  print(lab, ":  m=",m," nonzero monomials ",#sup," of ",#Ms,",  extents W_j = ",W,"  (full would be ",vector(m,j,D),")");
}
za = Mod(2395354738, P);
report("S3 at z = 0", 2, 2, xs->S3(xs[1],xs[2],Mod(0,P)));
report("S4 at z = 0", 3, 4, xs->S4num(xs[1],xs[2],xs[3],Mod(0,P)));
report("S4 at the root of c_1 (z = 2395354738)", 3, 4, xs->S4num(xs[1],xs[2],xs[3],za));
report("S5 at z = 0", 4, 8, xs->S5num(xs[1],xs[2],xs[3],xs[4],Mod(0,P)));
report("S5 at the root of c_2 (z = 2395354738)", 4, 8, xs->S5num(xs[1],xs[2],xs[3],xs[4],za));
