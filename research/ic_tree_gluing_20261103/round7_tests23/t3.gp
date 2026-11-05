\\ Test 3: exact count N(S,R) = # unordered triples of distinct E(F_p)-points with x in S and Q1+Q2+Q3=R (m=3, L(4O) through -R).
\\ wrapper sets: TT (list of t), NCTL, NT (targets), SEED
default(parisizemax, 300000000);
setrand(SEED);
p=786433; a=779083; b=163026; g=780070;
E=ellinit([a,b],p); NE=ellcard(E);
one=Mod(1,p);
key(P) = if(#P==1, 0, lift(P[1])*p+lift(P[2])+1);
ordsq(x) = { my(v=(x^3+a*x+b)%p); if(v==0, return([0])); if(kronecker(v,p)==1, my(r=lift(sqrt(Mod(v,p)))); return([r,p-r])); [] };
ptsof(S) = { my(L=List()); for(i=1,#S, my(ys=ordsq(S[i])); for(j=1,#ys, listput(L,[Mod(S[i],p),Mod(ys[j],p)]))); Vec(L) };
randpt() = { my(x,ys); while(1, x=random(p); ys=ordsq(x); if(#ys==2, return([Mod(x,p),Mod(ys[random(2)+1],p)]))) };
\\ N(S,R) summed over the target list TG
countset(PS, TG) = {
  my(n=#PS,M=Map(),Mp=Map(),c,tot=0,K,R2,R3,k,nk);
  for(i=1,n, mapput(Mp,key(PS[i]),1));
  for(i=1,n-1, for(j=i+1,n, K=key(elladd(E,PS[i],PS[j])); if(mapisdefined(M,K,&c), mapput(M,K,c+1), mapput(M,K,1))));
  my(res=vector(#TG));
  for(r=1,#TG, tot=0;
    for(k=1,n, R2=elladd(E,TG[r],ellneg(E,PS[k])); K=key(R2);
       if(mapisdefined(M,K,&c), nk=c, nk=0);
       \\ remove pairs containing k itself: partner Q_i = R-2Q_k, i != k
       my(R3=elladd(E,R2,ellneg(E,PS[k])), K3=key(R3));
       if(mapisdefined(Mp,K3), if(R3!=PS[k], nk--));
       tot+=nk);
    res[r]=tot/3);
  res };
pois_ci(lam) = { my(lo=0,hi=0,cdf=0,k=0,pk); while(1, pk=exp(-lam+k*log(lam)-lngamma(k+1)); cdf+=pk; if(lo==0 && cdf>0.025, lo=k); if(cdf>=0.975, hi=k; break); k++); [lo,hi] };
stat(v) = { my(m=vecsum(v)/#v, s=sqrt(sum(i=1,#v,(v[i]-m)^2)/(#v-1))); [m*1.,s*1.,vecmin(v),vecmax(v)] };
{
print("p=",p," #E=",NE," seed=",SEED," T=",NT," targets, ",NCTL," random controls");
my(TG=vector(NT,i,randpt()));
foreach(TT, t,
  my(d=2^t,c=lift(Mod(g,p)^d),V=List(),x0,t0=getabstime());
  \\ V = {x : x^d = c} = g * mu_d
  my(z=lift(sqrtn(Mod(1,p),d,&zz)),w=zz); \\ zz primitive d-th root of unity
  for(i=0,d-1, listput(V,lift(Mod(g,p)*zz^i)));
  V=Vec(V); if(#Set(V)!=d, error("V size"));
  my(PS=ptsof(V),n=#PS,lam=NT*binomial(n,3)/NE,ci=pois_ci(lam),kum=vecsum(countset(PS,TG)));
  my(cnt=vector(NCTL),ratio=vector(NCTL),nn=vector(NCTL));
  for(q=1,NCTL, my(S=Set(),Sv,PSq,nq); while(#S<d, S=setunion(S,Set([1+random(p-1)]))); Sv=Vec(S); PSq=ptsof(Sv); nq=#PSq;
     cnt[q]=vecsum(countset(PSq,TG)); nn[q]=nq; ratio[q]=cnt[q]/(NT*binomial(nq,3)/NE));
  \\ positive control: AP of multiples of G, x-set {x(iG), i=1..d/2}; targets sG with |s|<=d/2
  my(G=randpt(),Sa=List(),Pa,TA,posc,lamA,na);
  for(i=1,d/2, listput(Sa,lift(ellmul(E,G,i)[1])));
  Pa=ptsof(Vec(Sa)); na=#Pa; TA=vector(NT,i,ellmul(E,G,random(d)-d\2+1)); 
  posc=vecsum(countset(Pa,TA)); lamA=NT*binomial(na,3)/NE;
  my(kr=kum/lam, rs=stat(ratio), rank=sum(q=1,NCTL,ratio[q]<kr));
  print("t=",t," d=",d," n_V=",n," E_tot=",lam*1.," Poisson95=",ci," KUMMER=",kum," ratio=",kr*1.);
  print("   controls: counts ",stat(cnt)," ratio mean,sd,min,max ",rs," n range ",[vecmin(nn),vecmax(nn)]," rank of Kummer ratio among ",NCTL+1,": ",rank," ; controls outside own-Poisson95: ",sum(q=1,NCTL,my(l=NT*binomial(nn[q],3)/NE,c2=pois_ci(l)); cnt[q]<c2[1]||cnt[q]>c2[2]));
  print("   positive control (AP, planted-range targets): n=",na," count=",posc," random-expectation=",lamA*1.);
  print("   Johnson: agreement needed m=3, sqrt(d*(2h-1))=sqrt(3d)=",sqrt(3.*d)," ; time ms ",getabstime()-t0);
);
}
quit
