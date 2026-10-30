default(parisizemax,2000000000);
setrand(1);
p=101;
E=ellinit([3,5],p); n=ellcard(E); t=p+1-n; nt=p+1+t;
print("p=",p," #E=",n," #E'=",nt," gcd=",gcd(n,nt)," group E'=",ellgroup(ellinit([3*2^2,5*2^3],p)));
\\ F_{p^2} curve check
g=ffgen(p^2,'g); EE=ellinit([3*g^0,5*g^0]); print("#E(Fp2)=",ellcard(EE)," expect ",n*nt);
print(version());
