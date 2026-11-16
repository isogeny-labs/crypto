\\ FEASIBILITY ONLY (arm C solver cost on toy p). Outputs discarded.
default(parisizemax,1000000000);
setrand(3);
p=101; 
c=2; while(polisirreducible(Mod(1,p)*(w^3-c))==0,c++);
\\ generic curve over F_{p^3}
g=ffgen(Mod(1,p)*(w^3-c),'T);
a=g^2+3*g+5; b=g+7;
Ek=ellinit([a,b],g);
S3g(x1,x2,x3,A,B)=(x1-x2)^2*x3^2-2*((x1+x2)*(x1*x2+A)+2*B)*x3+(x1*x2-A)^2-4*B*(x1+x2);
\\ S4 via resultant in z with coefficients in F_{p^3}: use symbolic a,b as polynomials in tt, reduce later
tt=varlower("tt");
ap=a.pol; bp=b.pol; ap=subst(ap,variable(ap),tt); bp=subst(bp,variable(bp),tt);
z=varlower("z");
S4=polresultant(S3g(x1,x2,z,ap,bp),S3g(x3,x4,z,ap,bp),z);
print("S4 built");
R=random(Ek); xR=R[1]; L=xR.pol; 
u=polcoef(L,0); v=polcoef(L,1); w2=polcoef(L,2);
E4=subst(S4,x4,u+v*tt+w2*tt^2);
E4=substpol(E4,tt^3,c);
\\ reduce tt^k for k>=3 repeatedly
E4=substpol(E4,tt^3,c);
e0=polcoef(E4,0,tt); e1=polcoef(E4,1,tt); e2=polcoef(E4,2,tt);
print("degrees ",[poldegree(e0,x1),poldegree(e0,x2),poldegree(e0,x3)]);
e0=Mod(1,p)*e0; e1=Mod(1,p)*e1; e2=Mod(1,p)*e2;
t0=getabstime();
r1=polresultant(e0,e1,x3); r2=polresultant(e0,e2,x3);
r=polresultant(r1,r2,x2);
print("univariate degree ",poldegree(r));
n=#polrootsmod(lift(r),p);
print("ms: ",getabstime()-t0," roots ",n);
