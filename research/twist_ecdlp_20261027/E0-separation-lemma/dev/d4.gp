p=13; E=ellinit([0,1],p);
iso=ellisogeny(E,x+1,0);
E2=ellinit(iso[1]);
P=[Mod(2,13),Mod(3,13)]; print(ellisoncurve(E,P));
print(iferr(ellisogenyapply(iso[2],P),e,"errA"));
print(iferr(ellisogenyapply(iso[2],[0]),e,"errB"));
f=iso[2]; print(f);
print(iferr(ellisogenyapply(iso,P),e,"errC"));
\\ manual apply: (f/h^2, g/h^3)
{xx=P[1]; yy=P[2]; h=subst(f[3],x,xx); X=subst(f[1],x,xx)/h^2; Y=subst(subst(f[2],x,xx),y,yy)/h^3; print([X,Y]," ",ellisoncurve(E2,[X,Y]));}
print(ellisogeny(E,x+1,1));
