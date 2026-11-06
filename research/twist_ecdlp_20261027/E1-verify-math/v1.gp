default(parisizemax,2000000000);
\\ independent recount of per-target E1 counts via full-point group-law join over F_{q^2}
q=2609; a=2357; b=32; n=167; Wc=21461061; NT=100;
if(getenv("VT"),NT=eval(getenv("VT")));
if(getenv("VQ"),q=eval(getenv("VQ"));a=eval(getenv("VA"));b=eval(getenv("VB"));n=eval(getenv("VN"));Wc=eval(getenv("VW")));
g=ffgen(q^2,'w); Eg=ellinit([a*g^0,b*g^0]);
Eq=ellinit([a,b],q);
cardE=ellcard(Eq); print("card E=",cardE," card twist=",2*q+2-cardE," roots f=",#polrootsmod('x^3+a*'x+b,q));
f(x)=(x^3+a*x+b)%q;
V=vector(n,i,i-1);
CL=vector(n,i,kronecker(f(V[i]),q));
pts=vector(n,i,[V[i]*g^0,sqrt(f(V[i])*g^0)]);
key(P)=if(#P==1,"O",Str(P));
M=Map();
addpair(i,j)={my(S1,S2,P,k,L);
  S1=elladd(Eg,pts[i],pts[j]); S2=elladd(Eg,pts[i],ellneg(Eg,pts[j]));
  for(u=1,2,for(s=1,2,P=if(u==1,S1,S2);if(s==2,P=ellneg(Eg,P));k=key(P);
    if(!mapisdefined(M,k,&L),L=List());listput(L,[i,j]);mapput(M,k,L)))};
{for(i=1,n,for(j=i,n,addpair(i,j)));}
setrand(Wc+7);
T=vector(NT);
{for(tt=1,NT,while(1,P1=random(Eq);Q1=random(Eq);R=elladd(Eq,ellmul(Eq,P1,1+random(cardE)),ellmul(Eq,Q1,1+random(cardE)));if(R==[0],next);if(ellmul(Eq,R,2)==[0],next);break);T[tt]=[lift(R[1]),lift(R[2])]);}
totE=0;totED=0;totM=0;totMD=0;
{for(tt=1,NT,
  xR=T[tt][1];Rp=[xR*g^0,T[tt][2]*g^0];
  SL=List();
  for(k=1,n,for(s=1,2,
    U=elladd(Eg,Rp,if(s==1,pts[k],ellneg(Eg,pts[k])));
    kk=key(ellneg(Eg,U));
    if(mapisdefined(M,kk,&L),L=Vec(L);for(z=1,#L,listput(SL,vecsort([L[z][1],L[z][2],k])))))); 
  seen=Set(Vec(SL));e1=0;e2=0;m1=0;m2=0;
  for(z=1,#seen,tr=seen[z];deg=0;mix=0;
    for(u=1,3,if(V[tr[u]]==xR,deg=1);if(CL[tr[u]]==-1,mix=1);if(u>1&&tr[u]==tr[u-1],deg=1));
    if(mix,if(deg,m2++,m1++),if(deg,e2++,e1++)));
  print("MYTGT\t",tt,"\t",e1,"\t",e2,"\t",m1,"\t",m2);
  totE+=e1;totED+=e2;totM+=m1;totMD+=m2);}
print("MYTOT\t",totE,"\t",totED,"\t",totM,"\t",totMD);
