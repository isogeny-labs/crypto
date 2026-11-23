\\ Test 2b: sampled split rates. Vars set by wrapper: P, SEED, K (6 or 8), NS (samples per block), NB (blocks), VER (verify first VER hits)
default(parisizemax, 300000000);
setrand(SEED);
p=P; one=Mod(1,p);
sqtab(p) = { my(v=vector(p,i,List())); for(y=0,p-1, listput(v[(y^2%p)+1], y)); v };
SQ=sqtab(p);
issq(u) = (u==0) || (#SQ[u%p+1]>0);
randsextic() = { my(g); while(1, g = one*('x^6 + sum(i=0,5, random(p)*'x^i)); if(poldegree(gcd(g,g'))==0, return(g))) };
newcover() = { my(a,b,f,s); while(1, a=random(p); b=random(p); if((4*a^3+27*b^2)%p==0,next); f=one*('x^3+a*'x+b); s=random(p); if(subst(f,'x,Mod(s,p))==0,next); return([a,b,f,s,subst(f,'x,'x^2+s)])) };
rnd(n) = vector(n,i,random(p));
\\ cover/control sample, family (a deg<=3, b = 1) for K=6, (a deg<=3, b deg<=1, b1!=0) for K=8
split(N) = { my(r=polrootsmod(N,p)); #r==poldegree(N) };
\\ verification of K2a for a cover hit: returns 1 if projected zero set equals zero set of A+Cy on E and sums to O
verify(a,b,F,f,s,Ea) = { my(N=a^2-F*b,r,ok=1,pts,am=subst(a,'x,-'x),bm=subst(b,'x,-'x),Ev,Od,A,Cc,Ptot=[0]);
  \\ N here uses b as polynomial, y-coefficient: h=a+b*y, N=a^2-F*b^2 (b passed already as poly)
  N=a^2-F*b^2; r=polrootsmod(N,p);
  Ev=a*am+b*bm*F; Od=a*bm+am*b;       \\ Norm h = Ev + Od*y, both even in t
  A=0;Cc=0; for(j=0,poldegree(Ev)\2, A+=polcoeff(Ev,2*j)*('x-s)^j); for(j=0,poldegree(Od)\2, Cc+=polcoeff(Od,2*j)*('x-s)^j);
  for(i=1,#r, my(t0=r[i],bv=subst(b,'x,t0),av=subst(a,'x,t0),y0,x0); 
     if(bv==0, ok=0; next); y0=-av/bv; x0=t0^2+s;
     if(y0^2!=subst(f,'x,x0), ok=0);
     if(subst(A,'x,x0)+subst(Cc,'x,x0)*y0!=0, ok=0);
     Ptot=elladd(Ea,Ptot,[lift(x0),lift(y0)]));
  if(Ptot!=[0], ok=0); ok };
run() = {
  my(hitsC=0,hitsG=0,hitsEs=0,hitsEa=0,tot=0,ver=0,verok=0,Ea,c,G,F,f,s,a,b,N,A,C,r,allS,n);
  for(blk=1,NB,
    c=newcover(); f=c[3]; s=c[4]; F=c[5]; Ea=ellinit([c[1],c[2]],p); G=randsextic();
    for(i=1,NS,
      tot++;
      if(K==6, a=Pol(rnd(4),'x); b=one, a=Pol(rnd(4),'x); b=Pol(rnd(2),'x); if(poldegree(b)<1,next));
      a=one*a; b=one*b;
      N=a^2-F*b^2; 
      if(split(N), hitsC++; if(ver<VER, ver++; verok+=verify(a,b,F,f,s,Ea)));
      N=a^2-G*b^2; if(split(N), hitsG++);
      if(K==6,
        \\ direct elliptic sample in L(6O): A cubic (lead != 0), C deg<=1: zeros = roots of A^2-f C^2
        A=one*Pol(rnd(4),'x); C=one*Pol(rnd(2),'x); if(poldegree(A)<3,next);
        N=A^2-f*C^2; r=polrootsmod(N,p);
        if(#r==poldegree(N), hitsEa++; allS=1; for(j=1,#r, if(!issq(lift(r[j])-s), allS=0;break)); hitsEs+=allS);
      );
    );
  );
  [tot,hitsC,hitsG,hitsEa,hitsEs,ver,verok]
};
{
print("p=",p," K=",K," seed=",SEED," NS=",NS," NB=",NB);
my(t0=getabstime(),res=run());
print("[samples, cover hits, random-sextic hits, E-direct all-base hits, E-direct B_s hits, verified, verified ok]");
print(res);
my(n=res[1]); print("rates: cover=",res[2]/n*1.," ctl=",res[3]/n*1.," Edirect_all=",res[4]/n*1.," Edirect_Bs=",res[5]/n*1.);
print("formula 1/k! = ",1./(K!), "   1/(2^k k!) = ",1./(2^K*K!),"   time ms ",getabstime()-t0);
}
quit
