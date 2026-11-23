\\ FEASIBILITY ONLY, toy size, outputs discarded.
default(parisizemax,1000000000);
setrand(2);
p=1009;
{
while(1,
  a=random(p); b=random(p);
  if((4*a^3+27*b^2)%p==0,next);
  E=ellinit([a,b],p); N1=ellcard(E); t=p+1-N1; N2=p+1+t;
  if(N1%2==1 && N2%2==1 && gcd(N1,N2)==1 && N1!=p && N2!=p, break));
print("N1=",N1," N2=",N2);
}
d=2; while(kronecker(d,p)!=-1,d++);
T=ffgen(Mod(1,p)*(w^2-d),'T);
Ek=ellinit([a,b]*T^0,T);
S3(x1,x2,x3)=(x1-x2)^2*x3^2-2*((x1+x2)*(x1*x2+a)+2*b)*x3+(x1*x2-a)^2-4*b*(x1+x2);
tt=varlower("tt");
{
cnt=vector(6); 
for(i=1,200,
  R=random(Ek); xR=R[1];
  \\ coefficient extraction of ffelt: use fffrobenius-free route via Pol(lift)
  \\ fall back: represent via liftall
  L=xR.pol;
  u=polcoef(L,0); v=polcoef(L,1);
  S=S3(x1,x2,u+v*tt); S=substpol(S,tt^2,d);
  U=Mod(1,p)*polcoef(S,0,tt); V=Mod(1,p)*polcoef(S,1,tt);
  r=polresultant(U,V,x2);
  n=#polrootsmod(lift(r),p);
  if(n<=5,cnt[n+1]++,cnt[6]++));
print("histogram of #roots (0..4,5+): ",cnt);
}
