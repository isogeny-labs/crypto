default(parisizemax,2000000000);
setrand(1);
p=nextprime(2^20); a=17; b=29;
fx(u)=(u^3+a*u+b)%p;
bad=0;chk=0;mixedroots=0;mixed=0;
{
for(k=1,3000, my(x1=random(p),x2=random(p),Dv,s,pr,dc,t1,t2);
  if(x1==x2||fx(x1)==0||fx(x2)==0,next);
  Dv=Mod(x1-x2,p)^-2; s=2*((x1*x2+a)*(x1+x2)+2*b)*Dv; pr=((x1*x2-a)^2-4*b*(x1+x2))*Dv; dc=s^2-4*pr;
  t1=kronecker(fx(x1),p); t2=kronecker(fx(x2),p);
  if(t1==t2, if(!issquare(dc),bad++), mixed++; if(issquare(dc),mixedroots++));
  chk++);
}
print("pairs=",chk," same-type nonsquare disc=",bad," mixed=",mixed," mixed with square disc=",mixedroots);
d=1; while(kronecker(d,p)!=-1,d++);
Et=ellinit([a*d^2%p,b*d^3%p],p);
ok=0;tot=0;
{
for(k=1,200, my(x1=random(p),x2=random(p),X1,X2,P,Q,S,Dv,s,pr,dc,r1,r2);
  if(x1==x2||kronecker(fx(x1),p)!=-1||kronecker(fx(x2),p)!=-1,next);
  X1=d*x1%p; X2=d*x2%p; P=[Mod(X1,p),ellordinate(Et,X1)[1]]; Q=[Mod(X2,p),ellordinate(Et,X2)[1]];
  S=elladd(Et,P,Q); if(#S<2,next);
  Dv=Mod(x1-x2,p)^-2; s=2*((x1*x2+a)*(x1+x2)+2*b)*Dv; pr=((x1*x2-a)^2-4*b*(x1+x2))*Dv; dc=s^2-4*pr;
  r1=(s+sqrt(dc))/2; r2=(s-sqrt(dc))/2; tot++;
  if(S[1]/d==r1||S[1]/d==r2,ok++));
}
print("twist x-line check: ",ok,"/",tot);
print(version());
