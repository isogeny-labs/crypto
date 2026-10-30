default(parisizemax, "6G");
\\ Exceptional specializations of the e-form (PARI/GP, tooling).
\\ W_j = D (full extent in e_j) iff the pure-power coefficient c_j(z) of e_j^D is nonzero, because the
\\ total e-degree is <= D.  c_j(z) is recovered by evaluating T at points with e_i = 0 for i != j and
\\ interpolating in e_j and z.  Points with several zero arguments make leading coefficients vanish, so the
\\ resultants use FORMAL degrees (explicit Sylvester determinants); PARI's polresultant uses actual degrees
\\ and would return a different value there.
P = 3221225473;  \\ 3*2^30+1: has the 2nd, 3rd and 4th roots of unity needed for m = 2, 3, 4
if(!isprime(P), error("P not prime"));
a = Mod(-3, P); b = Mod(1234567, P);
g = znprimroot(P);
S3(x,y,z) = (x-y)^2*z^2 - 2*((x+y)*(x*y+a)+2*b)*z + ((x*y-a)^2 - 4*b*(x+y));
{
resf(f, g1, df, dg, v) = my(n=df+dg, M=matrix(n,n));
  for(i=1,dg, for(k=0,df, M[i,i+k]=polcoef(f,df-k,v)));
  for(i=1,df, for(k=0,dg, M[dg+i,i+k]=polcoef(g1,dg-k,v)));
  matdet(M);
}
S4num(x1,x2,x3,z) = resf(S3(x1,x2,'X), S3(x3,z,'X), 2, 2, 'X);
S4X(x3,x4,z) = resf(S3(x3,x4,'Y), S3(z,'X,'Y), 2, 2, 'Y);
S5num(x1,x2,x3,x4,z) = resf(S3(x1,x2,'X), S4X(x3,x4,z), 2, 4, 'X);
T(m, xs, z) = if(m==2, S3(xs[1],xs[2],z), if(m==3, S4num(xs[1],xs[2],xs[3],z), S5num(xs[1],xs[2],xs[3],xs[4],z)));
\\ sanity: formal-degree and actual-degree resultants agree at a nondegenerate point
{
chk() = my(r=vector(5,i,Mod(random(P),P)));
  if(S4num(r[1],r[2],r[3],r[4]) != polresultant(S3(r[1],r[2],'X), S3(r[3],r[4],'X), 'X), error("S4 formal != actual"));
  if(S5num(r[1],r[2],r[3],r[4],r[5]) != polresultant(S3(r[1],r[2],'X), polresultant(S3(r[3],r[4],'Y), S3(r[5],'X,'Y), 'Y), 'X), error("S5 formal != actual"));
  print("formal-degree resultants agree with polresultant at a random point (m = 3, 4)");
}
setrand(11); chk();
pt(m, j, r) = my(zeta=g^((P-1)/j)); vector(m, k, if(k<=j, r*zeta^(k-1), 0));
{
cj(m, j, zv) = my(D=2^(m-1), vals=vector(D+1), rs=vector(D+1,i,Mod(i,P)));
  for(i=1,D+1, vals[i]=T(m, pt(m,j,rs[i]), zv));
  my(us=vector(D+1,i,rs[i]^j), poly=polinterpolate(us, vals));
  polcoef(poly, D) * (-1)^((j+1)*D);
}
{
cpoly(m, j) = my(D=2^(m-1), zs=vector(D+1,i,Mod(i+10,P)));
  polinterpolate(zs, vector(D+1,i,cj(m,j,zs[i])));
}
\\ consistency with the generic fit: T at a random point of the e-form must equal what the pure-power data predict
{
for(m=2,4, my(D=2^(m-1)); for(j=1,m, my(c=cpoly(m,j));
  print("m=",m," j=",j,"  c_j(z) of degree ",poldegree(c),",  c_j(0) = ",polcoef(c,0),",  roots in F_P: ",polrootsmod(lift(c),P)~, "  (monic part: ", factormod(lift(c),P)[,2]~, ")")));
}
