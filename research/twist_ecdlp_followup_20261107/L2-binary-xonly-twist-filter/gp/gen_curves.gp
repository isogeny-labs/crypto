default(parisizemax,1000000000);
read("gp/common.gp");
\\ output: results/curves.txt
fd=fileopen("results/curves.txt","w");
{foreach([17,19,23,29,31],k,
  v=polyint(k); f=polfromint(v); g=ffgen(f,'t);
  for(idx=0,19,
    setrand(gpseed("curve",k,0,idx,0));
    while(1, b=random(g); if(b==0,next); E=ellinit([1,1,0,0,b]); N=ellcard(E);
      if(N%2==0 && isprime(N/2), filewrite(fd,Str("B ",k," ",idx," ",v," ",ff2int(b)," ",N," ",N/2)); break))));}
{foreach([17,19,23,29,31],k,
  p=precprime(2^k); while(p%4!=3,p=precprime(p-1));
  for(idx=0,9,
    setrand(gpseed("pcurve",k,0,idx,0));
    while(1, A=random(p); B=random(p); if((4*A^3+27*B^2)%p==0,next); E=ellinit([A,B],p); N=ellcard(E);
      if(isprime(N), filewrite(fd,Str("P ",k," ",idx," ",p," ",A," ",B," ",N," ",N)); break))));}
fileclose(fd);
quit;
