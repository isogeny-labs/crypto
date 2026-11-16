\\ Test 2a: exhaustive degree-2 divisors on C: y^2=f(t^2+s) -> E: y^2=f(x)=x^3+a x+b over F_p.
\\ usage: gp -q -s 64000000 -D "P=1009;SEED=1;EXH=1;NCTL=20" t2a.gp   (P,SEED,EXH,NCTL via default-setting below)
default(parisizemax, 300000000);
setrand(SEED);
sqtab(p) = { my(v=vector(p,i,List())); for(y=0,p-1, listput(v[(y^2%p)+1], y)); v };
\\ affine points of y^2=g(x), g a t_poly with integer coeffs (as Pol of Mod), return vector of [x,y]
pts(g,p,sq) = { my(L=List(),v); for(x=0,p-1, v=sq[(lift(subst(g,'x,Mod(x,p)))%p)+1]; for(i=1,#v, listput(L,[x,v[i]]))); Vec(L) };
nsplit(n,w) = n*(n+1)/2 - (n-w)/2 - w;   \\ unordered pairs {P1,P2}, P2 != iota(P1)
jacord(g,p) = { my(c=hyperellcharpoly(Mod(1,p)*g)); subst(c,'x,1) };
randsextic(p) = { my(g); while(1, g = Mod(1,p)*('x^6 + sum(i=0,5, random(p)*'x^i)); if(poldegree(gcd(g,g'))==0, return(g))) };
\\ ---- cover curves
coverdata(p) = { my(sq=sqtab(p),a,b,f,s,F,disc);
  while(1, a=random(p); b=random(p); disc=(4*a^3+27*b^2)%p; if(disc==0,next);
    f=Mod(1,p)*('x^3+a*'x+b); s=random(p); if(subst(f,'x,Mod(s,p))==0,next); break);
  F = subst(f,'x,'x^2+s); [a,b,s,f,F,sq] };
\\ main exhaustive comparison for one cover
exh(p) = { my(d=coverdata(p),a=d[1],b=d[2],s=d[3],f=d[4],F=d[5],sq=d[6],CP,EP,nC,w,idx,Ebs,nB,mark,cnt,nSC,nSE,E,miss,sNZ,incl,pr,Q1,Q2,i1,i2,t,y);
  E=ellinit([a,b],p);
  CP=pts(F,p,sq); nC=#CP; w=sum(i=1,nC,CP[i][2]==0);
  EP=pts(f,p,sq); 
  \\ B_s: E-points with x-s a square or 0 ; index them
  Ebs=List(); idx=Map(); for(i=1,#EP, my(u=(EP[i][1]-s)%p); if(u==0 || #sq[u+1]>0 || (u==0), listput(Ebs,EP[i]); mapput(idx,EP[i],#Ebs)));
  \\ note: sq index uses (y^2 mod p)+1, so #sq[u+1]>0 iff u is a square (u=0 included)
  nB=#Ebs;
  mark=matrix(nB,nB);
  cnt=0; incl=0;
  for(i=1,nC, for(j=i,nC,
     if(i==j && CP[i][2]==0, next); 
     if(j>i && CP[j][1]==CP[i][1] && CP[j][2]==(-CP[i][2])%p, next);   \\ P2 = iota(P1)
     cnt++;
     Q1=[(CP[i][1]^2+s)%p,CP[i][2]]; Q2=[(CP[j][1]^2+s)%p,CP[j][2]];
     i1=mapget(idx,Q1); i2=mapget(idx,Q2);
     if(i1>i2,[i1,i2]=[i2,i1]); mark[i1,i2]=1));
  nSC=sum(i=1,nB,sum(j=i,nB,mark[i,j]!=0));
  nSE=nB*(nB+1)/2;
  \\ characterise unreached elements of S_E
  miss=0; my(missinv=0,misst0=0);
  for(i=1,nB, for(j=i,nB, if(mark[i,j]==0, miss++;
     if(Ebs[i][1]==Ebs[j][1] && Ebs[i][2]==(-Ebs[j][2])%p, missinv++));));
  my(nJ=jacord(F,p), nE=ellcard(E), g2, ncE);
  \\ E': v^2 = w f(w+s) (quartic, monic) : affine count + 2
  my(g4=Mod(1,p)*('x*subst(f,'x,'x+s))); ncE = #pts(g4,p,sq)+2;
  [p,nC,w,cnt,nsplit(nC,w),nJ,nE*ncE,cnt/nJ*1.,nB,nSE,nSC,miss,missinv,nE,ncE]
};
\\ rate-only (formula) for many covers / controls
coverrate(p) = { my(d=coverdata(p),F=d[5],sq=d[6],CP=pts(F,p,sq),n=#CP,w=sum(i=1,n,CP[i][2]==0),nJ=jacord(F,p)); nsplit(n,w)/nJ*1. };
ctlrate(p) = { my(sq=sqtab(p),g=randsextic(p),CP=pts(g,p,sq),n=#CP,w=sum(i=1,n,CP[i][2]==0),nJ=jacord(g,p)); nsplit(n,w)/nJ*1. };
stat(v) = { my(m=vecsum(v)/#v, s=sqrt(sum(i=1,#v,(v[i]-m)^2)/(#v-1))); [m,s,vecmin(v),vecmax(v)] };
{
print("p=",P," seed=",SEED);
if(EXH, print("EXH [p,nC,w,cnt,formula,#Jac,#E*#E',rate,|B_s|,|S_E|,|S_C|,|S_E\\S_C|,of which Q+(-Q),#E,#E']"); print(exh(P)));
if(NCTL,
  my(vc=vector(NCTL,i,coverrate(P)), vr=vector(NCTL,i,ctlrate(P)));
  print("cover  split rate (mean,sd,min,max) = ", stat(vc));
  print("random split rate (mean,sd,min,max) = ", stat(vr)));
}
quit
