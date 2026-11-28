\\ FEASIBILITY ONLY, toy size, outputs discarded, no results recorded.
default(parisizemax,1000000000);
setrand(1);
p=1009;
\\ find E/F_p with N1,N2 odd, coprime
{
while(1,
  a=random(p); b=random(p);
  if((4*a^3+27*b^2)%p==0,next);
  E=ellinit([a,b],p); N1=ellcard(E); t=p+1-N1; N2=p+1+t;
  if(N1%2==1 && N2%2==1 && gcd(N1,N2)==1, break));
print("N1=",N1," N2=",N2," N=",N1*N2);
}
d=2; while(kronecker(d,p)!=-1,d++);
\\ F_{p^2} = F_p[t]/(t^2-d) as ffelt
T=ffgen(Mod(1,p)*(w^2-d),'T);
Ek=ellinit([a,b]*T^0,T);
\\ random point of E(F_{p^2})
R=ellrandom(Ek); xR=R[1]; 
u=polcoef(lift(xR),0); v=polcoef(lift(xR),1);
print("xR in Fp? ", v==0);
\\ Weil restriction of S3
S3(x1,x2,x3)=(x1-x2)^2*x3^2-2*((x1+x2)*(x1*x2+a)+2*b)*x3+(x1*x2-a)^2-4*b*(x1+x2);
tt=varlower("tt");
S=S3(x1,x2,u+v*tt);
S=substpol(S,tt^2,d);
U=polcoef(S,0,tt); V=polcoef(S,1,tt);
Us=Mod(1,p)*U; Vs=Mod(1,p)*V;
r=polresultant(Us,Vs,x2);
rts=polrootsmod(r,p);
print("roots of resultant: ", #rts);
