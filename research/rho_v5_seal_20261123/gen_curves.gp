\\ prime-order (cofactor 1) curves y^2=x^3+ax+b over F_p, p of exactly `bits` bits; seeded.
gen(bits,n,seed)={ my(res=List()); setrand(seed);
 while(#res<n,
  my(p=nextprime(2^(bits-1)+random(2^(bits-1)-1)));
  if(p>=2^bits,next);
  my(a=1+random(p-1),b=1+random(p-1));
  if((4*a^3+27*b^2)%p==0,next);
  my(E=ellinit([a,b],p)); my(N=ellcard(E));
  if(!isprime(N),next);
  my(P=random(E)); while(P==[0],P=random(E));
  listput(res,[p,a,b,N,lift(P[1]),lift(P[2])]));
 Vec(res)}
for(i=1,#BITS,my(v=gen(BITS[i],16,880000+BITS[i])); write(OUT,"["); for(j=1,#v,write(OUT,Str("[",v[j][1],",",v[j][2],",",v[j][3],",",v[j][4],",",v[j][5],",",v[j][6],"]",if(j<#v,",","")))); write(OUT,"]"));
