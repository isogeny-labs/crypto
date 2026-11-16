default(parisizemax,2000000000);
read("lib.gp");
\\ my own independent counter via group law over F_{q^2}
xkey(P)=if(#P==1,[-1,-1],[polcoef(P[1].pol,0),polcoef(P[1].pol,1)]);
mycount(bits,pidx,cidx,m,fb,al,nt)={
  my(W=world(bits,pidx,cidx),q=W[1],a=W[2],b=W[3],E=W[9],Ed=W[10],T,V,n,g,EF,pts,cls,Mp,xR,R,res=List(),nd,mm);
  g=ffgen(q^2,'t); EF=ellinit([a*g^0,b*g^0]);
  T=mktargets(E,W[4],W[13],200);
  n=min(ceil(q^al),if(m==2,16384,1500));
  V=mkfb(fb,n,E,Ed,W[13]);
  pts=vector(n,i,[V[i]*g^0,sqrt(((V[i]^3+a*V[i]+b)%q)*g^0)]);
  cls=vector(n,i,kronecker((V[i]^3+a*V[i]+b)%q,q));
  if(m==3,Mp=Map();
    for(i=1,n,for(j=i,n,for(s=0,1,my(S=if(s,elladd(EF,pts[i],ellneg(EF,pts[j])),elladd(EF,pts[i],pts[j])),k=xkey(S),L);
      L=if(mapisdefined(Mp,k),mapget(Mp,k),List());listput(L,[i,j]);mapput(Mp,k,L)))));
  for(t=1,nt,xR=T[t][1];R=[xR*g^0,T[t][2]*g^0];
    my(sol=Set(),inV=0);
    for(i=1,n,if(V[i]==xR,inV=1));
    if(m==2,
      for(i=1,n,for(s=0,1,my(S=elladd(EF,R,if(s,ellneg(EF,pts[i]),pts[i])),k=xkey(S));
        \\ need x(S) = x_j for some j
        if(k[1]>=0&&k[2]==0,for(j=1,n,if(V[j]==k[1],sol=setunion(sol,Set([vecsort([i,j])])))))))
    ,
      for(k=1,n,for(s=0,1,my(S=elladd(EF,R,if(s,ellneg(EF,pts[k]),pts[k])),key=xkey(S));
        if(mapisdefined(Mp,key),my(L=mapget(Mp,key));for(u=1,#L,sol=setunion(sol,Set([vecsort([L[u][1],L[u][2],k])])))))));
    my(r=[0,0,0,0]);
    for(u=1,#sol,my(id=sol[u],mix=0,deg=0);
      for(z=1,#id,if(cls[id[z]]==-1,mix=1);if(V[id[z]]==xR,deg=1);if(z>1&&id[z]==id[z-1],deg=1));
      r[if(mix,3,1)+if(deg,1,0)]++);
    listput(res,concat([t],r)));
  Vec(res)};
