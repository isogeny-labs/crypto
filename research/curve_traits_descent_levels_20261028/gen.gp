read("bfs.gp");
{setrand(20261028); n=0;
while(n<40,
  p=randomprime([10^4,10^6]); a=random(p); b=random(p);
  if((4*a^3+27*b^2)%p==0, next);
  E=ellinit([a,b],p); t=ellap(E); D=t^2-4*p;
  j=lift(E.j); if(j==0||j==1728, next);
  dK=coredisc(D); v=sqrtint(D/dK);
  forprime(l=2,5, dep=valuation(v,l);
    if(dep>=2, h=height(j,l,p); print(p," ",a," ",b," ",l," ",dep," ",h); n++)))}
