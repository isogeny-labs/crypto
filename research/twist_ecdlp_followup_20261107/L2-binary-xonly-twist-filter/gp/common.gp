SEEDENV=getenv("L2SEED_XOR");
MSEED=bitxor(20261107,if(SEEDENV==0,0,eval(SEEDENV)));
fnv(s)={my(h=14695981039346656037,v=Vecsmall(s));for(i=1,#v,h=bitxor(h,v[i]);h=(h*1099511628211)%2^64);h};
seedof(role,k,m,c,t)=bitxor(MSEED,fnv(Str(role,"/",k,"/",m,"/",c,"/",t)));
gpseed(role,k,m,c,t)=seedof(role,k,m,c,t)%2^32+1;
\\ lexicographically first minimal-weight irreducible: smallest integer value among weight 3, else weight 5
polyint(k)={my(best=0,val=0);
  for(j=1,k-1, if(polisirreducible(Mod(1,2)*(x^k+x^j+1)), return(2^k+2^j+1)));
  for(a=1,k-1,for(b=1,a-1,for(c=1,b-1,
    if(polisirreducible(Mod(1,2)*(x^k+x^a+x^b+x^c+1)),
      val=2^k+2^a+2^b+2^c+1; if(best==0||val<best,best=val)))));
  best};
polfromint(v)=Mod(1,2)*Pol(binary(v));
ff2int(e)={my(v=Vecrev(e.pol),s=0);for(i=1,#v,s+=lift(v[i])*2^(i-1));s};
