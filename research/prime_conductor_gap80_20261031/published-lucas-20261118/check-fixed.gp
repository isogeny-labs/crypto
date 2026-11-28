\\ Fixed published arithmetic fixture, not a parameter search or benchmark.
\\ A101087 lists n=7351 among its prime entries, separately from later PRPs.
{
  my(n=7351,u0=0,u1=1,t0=2,t1=-1,u,t,c,cert,m,ep);
  setrand(20261118);
  default(nbthreads,1);
  ep=ellinit([1,0,0,0,1],2);
  if(ellcard(ep)!=4,error("base point count"));
  if(!isfundamental(-7),error("fundamental discriminant"));
  for(k=2,n,
    u=-u1-2*u0; u0=u1; u1=u;
    t=-t1-2*t0; t0=t1; t1=t;
  );
  c=abs(u1);
  m=[0,-2;1,-1]^n;
  if(m[2,1]!=u1 || trace(m)!=t1,error("matrix recurrence mismatch"));
  if(t1^2-4*2^n != -7*c^2,error("discriminant mismatch"));
  if(c<=2^263 || c%2!=1 || t1%2!=1,error("fixture eligibility"));
  write("arithmetic.json",Str("{\"schema\":\"fixed-lucas-conductor/v1\",\"n\":",n,",\"p\":2,\"model\":[1,0,0,0,1],\"D_K\":-7,\"base_count\":4,\"trace\":\"",t1,"\",\"frobenius_conductor\":\"",c,"\",\"conductor_bits\":",logint(c,2)+1,",\"recurrence_matrix_agree\":true,\"discriminant_identity\":true,\"quotient\":null,\"point_images\":null,\"endpoint_timings\":null}"));
  cert=primecert(c);
  if(cert==0 || !primecertisvalid(cert),error("prime certificate rejected"));
  if(type(cert)!="t_VEC" || cert[1][1]!=c,error("certificate subject"));
  write("conductor.cert",cert);
  print("{\"arithmetic_complete\":true,\"certificate_generated\":true,\"certificate_valid\":true,\"scope\":\"fixed published conductor prime only\"}");
}
quit
