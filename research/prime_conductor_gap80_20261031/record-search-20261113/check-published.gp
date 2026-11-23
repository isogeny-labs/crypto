\\ Arithmetic replay of KKM11 Example 4, not an isogeny construction.
\\ K-571: y^2 + xy = x^3 + 1 over F_(2^571).
\\ Frobenius over F_2 has polynomial X^2 + X + 2.
\\ Fixed published instance; no parameter or key search.
{
  my(u0=0,u1=1,t0=2,t1=-1,u,t,c,small=0,ell,cert);
  for(n=2,571,
    u=-u1-2*u0; u0=u1; u1=u;
    t=-t1-2*t0; t0=t1; t1=t;
  );
  c=abs(u1);
  if(t1^2-4*2^571 != -7*c^2,error("discriminant mismatch"));
  forprime(s=2,2^22-1,if(c%s==0,small=s;break));
  if(!small,error("published small factor absent"));
  ell=c/small;
  if(logint(small,2)+1!=22 || logint(ell,2)+1!=263,error("published widths mismatch"));
  if(!isprime(small) || !isprime(ell),error("nonprime factor"));
  cert=primecert(ell);
  if(!primecertisvalid(cert),error("prime certificate rejected"));
  write("prime263.cert",cert);
  print("{\"schema\":\"published-conductor-arithmetic/v1\",\"curve\":\"K-571\",\"field\":\"GF(2^571)\",\"model\":\"y^2+xy=x^3+1\",\"trace\":\"",t1,"\",\"D_K\":-7,\"frobenius_conductor\":\"",c,"\",\"small_prime\":\"",small,"\",\"large_prime\":\"",ell,"\",\"large_prime_bits\":263,\"primality_method\":\"PARI isprime and primecert/primecertisvalid\",\"quotient_equation\":null,\"point_images\":null,\"endpoint_timings\":null,\"scope\":\"published factor arithmetic only\"}");
}
quit
