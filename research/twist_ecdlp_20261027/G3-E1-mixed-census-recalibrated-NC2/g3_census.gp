\\ g3_census.gp  -- G3 re-registered mixed-solution census (PREREGISTRATION.md sections 1-9)
\\ stages (env G3_STAGE): oracle | controls | grid   ; seed env G3_SEED (default 20261026)
default(parisizemax,2000000000);
default(realprecision,19);
genv(name,def)={my(s=getenv(name)); if(type(s)=="t_STR" && s!="", s, def)};
geni(name,def)={my(s=genv(name,def)); if(type(s)=="t_STR", eval(s), s)};
S0=geni("G3_SEED",20261026);
setrand(S0);
STAGE=genv("G3_STAGE","none");

\\ ---------------------------------------------------------------- basic field / curve
\\ globals: Q (prime), CA, CB (curve), DD (least non-residue), DINV; SEMA,SEMB (curve used inside Semaev polys)
setcurve(q,a,b)={Q=q;CA=a;CB=b;DD=2;while(kronecker(DD,Q)!=-1,DD++);DINV=lift(1/Mod(DD,Q));SEMA=a;SEMB=b;};
fAB(xv,a,b)=(((xv*xv)%Q+a)*xv+b)%Q;
fx(xv)=fAB(xv,CA,CB);
tauT(e,xv)=(e+((3*e*e+CA)%Q)*lift(1/Mod(xv-e,Q)))%Q;   \\ x(P+T), T=(e,0)
oddp(n)=n>>valuation(n,2);
sqrtI(f)=lift(sqrt(Mod(f,Q)));

\\ Semaev S_3 coefficient triple in the last variable: S_3(u,v,z)=cA z^2+cB z+cC
cA(u,v)=(u-v)^2%Q;
cB(u,v)=(-2*((u+v)*(u*v+SEMA)+2*SEMB))%Q;
cC(u,v)=((u*v-SEMA)^2-4*SEMB*(u+v))%Q;

\\ class (kronecker of f), and r with f=r^2 (class 1) or f=DD r^2 (class -1), for curve (a,b)
vdata(V,a,b)={my(n=#V,CL=vector(n),RS=vector(n),f,c);
  for(i=1,n,f=fAB(V[i],a,b);c=kronecker(f,Q);CL[i]=c;
    if(c==1,RS[i]=sqrtI(f),if(c==-1,RS[i]=sqrtI((f*DINV)%Q),RS[i]=0)));
  [CL,RS]};

\\ keys of the root set of S_3(xi,xj,z) in z: rational root z in [0,Q); INF=Q; irreducible quadratic: Q+1+u*Q+v (monic z^2+uz+v)
pkeys(xi,xj,ci,cj,ri,rj)={
  my(A=cA(xi,xj),Bn=cB(xi,xj),Cn=cC(xi,xj),cc=ci*cj,s,iv,z1,z2,u,v);
  if(A==0,
    if(Bn==0,return([Q]));
    return([Q,(-Cn*lift(1/Mod(Bn,Q)))%Q]));
  if(cc<0,iv=lift(1/Mod(A,Q));u=(Bn*iv)%Q;v=(Cn*iv)%Q;return([Q+1+u*Q+v]));
  if(cc==0,s=0,if(ci>0,s=(ri*rj)%Q,s=(DD*ri*rj)%Q));
  iv=lift(1/Mod(2*A,Q));
  z1=((-Bn+4*s)*iv)%Q;
  if(s==0,return([z1]));
  z2=((-Bn-4*s)*iv)%Q;
  [z1,z2]};

\\ ---------------------------------------------------------------- multiset codes (indices into V, sorted)
mkcode2(n,i,j)=if(i<=j,i*(n+1)+j,j*(n+1)+i);
mkcode3(n,i,j,k)={my(s=vecsort([i,j,k]));(s[1]*(n+1)+s[2])*(n+1)+s[3]};
decode(c,m,n)={if(m==2,[c\(n+1),c%(n+1)],[c\((n+1)^2),(c\(n+1))%(n+1),c%(n+1)])};

\\ classify a Set of codes: returns [E_nd,E_deg,M_nd,M_deg]; mixed = some element of class -tc
classify(codes,m,n,V,CLr,tc,xR)={my(r=[0,0,0,0],idx,deg,mix);
  for(t=1,#codes,idx=decode(codes[t],m,n);deg=0;mix=0;
    for(u=1,m,if(V[idx[u]]==xR,deg=1);if(CLr[idx[u]]==-tc,mix=1);if(u>1&&idx[u]==idx[u-1],deg=1));
    r[if(mix,3,1)+if(deg,1,0)]++);
  r};

\\ ---------------------------------------------------------------- Detector A
mktab(V,CL,RS)={my(n=#V,cap=n*(n+1)+2,K=vector(cap),I=vector(cap),c=0,ks,xi,ci,ri,P);
  for(i=1,n,xi=V[i];ci=CL[i];ri=RS[i];
    for(j=i,n,ks=pkeys(xi,V[j],ci,CL[j],ri,RS[j]);
      for(t=1,#ks,c++;K[c]=ks[t];I[c]=(i-1)*n+j)));
  K=K[1..c];I=I[1..c];P=vecsort(K,,1);
  TK=vecextract(K,P);TI=vecextract(I,P);TN=n;0};

tabfind(tab,key)={my(p=vecsearch(TK,key),L=List(),t,nn=#TK);
  if(!p,return([]));t=p;while(t>1&&TK[t-1]==key,t--);
  while(t<=nn&&TK[t]==key,listput(L,TI[t]);t++);Vec(L)};

\\ m=3: set of triple codes S with S_4(.,.,.,xR)=0 ; (cR,rR) class data of xR for the Semaev curve
solve3(tab,V,CL,RS,xR,cR,rR)={my(n=#V,L=List(),ks,ids,i,j);
  for(k=1,n,ks=pkeys(V[k],xR,CL[k],cR,RS[k],rR);
    for(t=1,#ks,ids=tabfind(tab,ks[t]);
      for(u=1,#ids,i=(ids[u]-1)\n+1;j=(ids[u]-1)%n+1;listput(L,mkcode3(n,i,j,k)))));
  Set(L)};

\\ m=2: set of pair codes ; Mx: Map value->index
solve2(Mx,V,CL,RS,xR,cR,rR)={my(n=#V,L=List(),ks,z,j);
  for(i=1,n,ks=pkeys(V[i],xR,CL[i],cR,RS[i],rR);
    for(t=1,#ks,z=ks[t];if(z<Q&&mapisdefined(Mx,z,&j),listput(L,mkcode2(n,i,j)))));
  Set(L)};

mkmap(V)={my(M=Map());for(i=1,#V,mapput(M,V[i],i));M};

\\ ---------------------------------------------------------------- Oracle B (brute force, S_3 formula / closed resultant)
s3eval(x1,x2,x3)=((x1-x2)^2*x3^2-2*((x1+x2)*(x1*x2+SEMA)+2*SEMB)*x3+(x1*x2-SEMA)^2-4*SEMB*(x1+x2))%Q;
s3coef(x1,x2)={my(zz=(x1-x2)^2*'z^2-2*((x1+x2)*(x1*x2+SEMA)+2*SEMB)*'z+(x1*x2-SEMA)^2-4*SEMB*(x1+x2));
  [polcoef(zz,2)%Q,polcoef(zz,1)%Q,polcoef(zz,0)%Q]};
resclosed(p1,p2)=((p1[1]*p2[3]-p2[1]*p1[3])^2-(p1[1]*p2[2]-p2[1]*p1[2])*(p1[2]*p2[3]-p2[2]*p1[3]))%Q;

B2(V,xR)={my(n=#V,L=List());
  for(i=1,n,for(j=i,n,if(s3eval(V[i],V[j],xR)==0,listput(L,mkcode2(n,i,j)))));Set(L)};

\\ returns [Set, symmetry violations]
B3(V,xR)={my(n=#V,L=List(),PC=matrix(n,n),RC=vector(n),r1,r2,r3,viol=0);
  for(i=1,n,for(j=i,n,PC[i,j]=s3coef(V[i],V[j])));
  for(i=1,n,RC[i]=s3coef(V[i],xR));
  for(i=1,n,for(j=i,n,for(k=j,n,
    r1=resclosed(PC[i,j],RC[k]);
    r2=resclosed(PC[i,k],RC[j]);
    r3=resclosed(PC[j,k],RC[i]);
    if((r1==0)!=(r2==0)||(r1==0)!=(r3==0),viol++);
    if(r1==0,listput(L,mkcode3(n,i,j,k))))));
  [Set(L),viol]};

\\ closed resultant versus the formal-degree-2 Sylvester determinant on a sample (formula check)
rescheck(V,xR,ns)={my(bad=0,i,j,k,p1,p2,M);
  for(t=1,ns,i=1+random(#V);j=1+random(#V);k=1+random(#V);
    p1=s3coef(V[i],V[j]);p2=s3coef(V[k],xR);
    M=Mat([p1[1],p1[2],p1[3],0;0,p1[1],p1[2],p1[3];p2[1],p2[2],p2[3],0;0,p2[1],p2[2],p2[3]]*Mod(1,Q));
    if(lift(matdet(M))!=resclosed(p1,p2),bad++));
  bad};

\\ ---------------------------------------------------------------- Oracle G (group law over F_{Q^2}, independent of Semaev)
setupFF()={GG=ffgen(Q^2,'t);EFF=ellinit([CA*GG^0,CB*GG^0]);0};
ptff(xv)={my(f=fx(xv)*GG^0);[xv*GG^0,sqrt(f)]};
ffcls(P)={my(y=P[2]);if(y==0,0,if(y^Q==y,1,-1))};
\\ x-coordinate of a point as an integer if it lies in F_Q, else -1 (point at infinity: -2)
xval(P)={if(#P==1,return(-2));my(xx=P[1]);if(poldegree(xx.pol)>0,return(-1));lift(polcoef(xx.pol,0))};

G2(V,Rp)={my(n=#V,Mx=mkmap(V),pts=vector(n,i,ptff(V[i])),L=List(),Qp,xv,j);
  for(i=1,n,for(s=1,2,Qp=if(s==1,elladd(EFF,Rp,pts[i]),elladd(EFF,Rp,ellneg(EFF,pts[i])));
    xv=xval(Qp);if(xv>=0&&mapisdefined(Mx,xv,&j),listput(L,mkcode2(n,i,j)))));
  [Set(L),vector(n,i,ffcls(pts[i]))]};

G3(V,Rp)={my(n=#V,Mx=mkmap(V),pts=vector(n,i,ptff(V[i])),npts=vector(n,i,ellneg(EFF,pts[i])),L=List(),base,Qp,xv,k);
  for(i=1,n,for(s=1,2,base=if(s==1,elladd(EFF,Rp,pts[i]),elladd(EFF,Rp,npts[i]));
    for(j=i,n,for(u=1,2,Qp=if(u==1,elladd(EFF,base,pts[j]),elladd(EFF,base,npts[j]));
      xv=xval(Qp);if(xv>=0&&mapisdefined(Mx,xv,&k),listput(L,mkcode3(n,i,j,k)))))));
  [Set(L),vector(n,i,ffcls(pts[i]))]};

\\ does R + sum s_i P(xs_i) = O for some signs, P(xs_i) a lift of xs_i to E(F_{Q^2})? xs: vector of x values
Gconf(xs,Rp)={my(mm=#xs,pts=vector(mm,i,ptff(xs[i])),S,ok=0);
  for(sg=0,2^mm-1,S=Rp;for(i=1,mm,S=if(bittest(sg,i-1),elladd(EFF,S,ellneg(EFF,pts[i])),elladd(EFF,S,pts[i])));
    if(#S==1,ok=1;break));
  ok};

\\ ---------------------------------------------------------------- Oracle S (structural predictor, section 2 C1-C3)
\\ returns Set of codes of predicted mixed multisets (m=3); m=2 -> empty
oracleS(V,CLr,Mx,xR,tc,roots,m)={my(n=#V,L=List(),i2,i3,x2,x3,e);
  if(m==2,return(Set([])));
  if(mapisdefined(Mx,xR,&i3),for(i=1,n,if(CLr[i]==-tc,listput(L,mkcode3(n,i,i,i3)))));
  for(r=1,#roots,e=roots[r];x3=tauT(e,xR);
    if(mapisdefined(Mx,x3,&i3),
      for(i=1,n,if(CLr[i]==-tc&&V[i]!=e,x2=tauT(e,V[i]);
        if(mapisdefined(Mx,x2,&i2),listput(L,mkcode3(n,i,i2,i3)))))));
  Set(L)};

\\ ---------------------------------------------------------------- generators of strata: primes, curves, factor bases, targets
\\ G3 prime stream (PREREG section 5): setrand(S0+100000*bits); q_k = nextprime(2^(bits-1)+random(2^(bits-1))), first distinct ones
mkprime(bits,pidx)={my(W=S0+100000*bits,L=List(),q);
  setrand(W);
  while(#L<=pidx,q=nextprime(2^(bits-1)+random(2^(bits-1)));if(#select(x->x==q,Vec(L))==0,listput(L,q)));
  L[pidx+1]};

\\ G3 strata (PREREG section 5): nT = [0,1,3][cidx mod 3]; cidx 0 and 3 seek gcd_odd(#E,#E')>1 within 20000 draws (FALLBACK flag)
\\ returns [a,b,nT,cardE,cardEt,gcdodd,flag,draws]
mkcurve(q,Wp,cidx)={my(Wc=Wp+10*(cidx+1),want=[0,1,3][cidx%3+1],seek=(cidx==0||cidx==3),a,b,nt,E,cE,cEt,go=0,draws=0,flag="ok",done=0);
  setrand(Wc);
  while(!done,
    a=random(q);b=random(q);
    if((4*a^3+27*b^2)%q==0,next);
    nt=#polrootsmod('x^3+a*'x+b,q);draws++;
    if(nt!=want,next);
    if(!seek,done=1;next);
    E=ellinit([a,b],q);cE=ellcard(E);cEt=2*q+2-cE;go=gcd(oddp(cE),oddp(cEt));
    if(go>1,done=1);
    if(!done&&draws>20000,flag="FALLBACK_no_gcdodd_in_20000";done=1));
  E=ellinit([a,b],q);cE=ellcard(E);cEt=2*q+2-cE;go=gcd(oddp(cE),oddp(cEt));
  [a,b,nt,cE,cEt,go,flag,draws]};

twistcurve(a,b)=ellinit([(a*DD^2)%Q,(b*DD^3)%Q],Q);

mkfb(fb,n,E,Ed,Wc)={my(V,Mx,x1,G,P,L,s,w,val,h,k);
  if(fb=="interval",return(vector(n,i,i-1)));
  if(fb=="random",setrand(Wc+31);Mx=Map();L=List();
    while(#L<n,x1=random(Q);if(!mapisdefined(Mx,x1),mapput(Mx,x1,1);listput(L,x1)));return(Vec(L)));
  if(fb=="twistmult",setrand(Wc+41);Mx=Map();L=List();h=n\2;
    G=random(E);while(G==[0],G=random(E));P=G;
    for(k=1,h,if(P==[0],break);x1=lift(P[1]);if(!mapisdefined(Mx,x1),mapput(Mx,x1,1);listput(L,x1));P=elladd(E,P,G));
    G=random(Ed);while(G==[0],G=random(Ed));P=G;
    for(k=1,h,if(P==[0],break);x1=lift(P[1]*DINV);if(!mapisdefined(Mx,x1),mapput(Mx,x1,1);listput(L,x1));P=elladd(Ed,P,G));
    return(Vec(L)));
  if(fb=="farey",Mx=Map();L=List();s=1;
    while(#L<n,for(u=0,s-1,w=s-u;if(gcd(u,w)==1,val=(u*lift(1/Mod(w,Q)))%Q;
        if(!mapisdefined(Mx,val),mapput(Mx,val,1);listput(L,val);if(#L>=n,break))));s++);
    return(Vec(L)));
  error("unknown fb ",fb)};

\\ N targets on curve E (card cE), sequence fixed by seed Wc+7 ; entries [xR,yR]
mktargets(E,cE,Wc,N)={my(T=vector(N),R,P1,Q1);setrand(Wc+7);
  for(t=1,N,while(1,P1=random(E);Q1=random(E);
    R=elladd(E,ellmul(E,P1,1+random(cE)),ellmul(E,Q1,1+random(cE)));
    if(R==[0],next);if(ellmul(E,R,2)==[0],next);break);
    T[t]=[lift(R[1]),lift(R[2])]);T};
\\ twist-side targets: x = X/DD (E-model x), second entry = Y on the twist model
mktargetsW(Ed,cEt,Wc,N)={my(T=vector(N),R,P1,Q1);setrand(Wc+7);
  for(t=1,N,while(1,P1=random(Ed);Q1=random(Ed);
    R=elladd(Ed,ellmul(Ed,P1,1+random(cEt)),ellmul(Ed,Q1,1+random(cEt)));
    if(R==[0],next);if(ellmul(Ed,R,2)==[0],next);break);
    T[t]=[lift(R[1]*DINV),lift(R[2])]);T};

\\ ---------------------------------------------------------------- helpers
tsv(v)={for(i=1,#v,print1(v[i]);if(i<#v,print1("\t")));print("")};
wilson(k,N)={my(z=1.959964,p=k/N,den=1+z^2/N,c=(p+z^2/(2*N))/den,h=z*sqrt(p*(1-p)/N+z^2/(4*N^2))/den);[c-h,c+h]};
alphaN(al,cap)=min(ceil(Q^al),cap);

\\ one fully configured "world": returns [q,a,b,cE,cEt,go,nT,flag,E,Ed,roots,Wp,Wc]
world(bits,pidx,cidx)={my(q,Wp,cv,E,Ed,rts);
  q=mkprime(bits,pidx);Wp=S0+100000*bits+1000*pidx;
  cv=mkcurve(q,Wp,cidx);
  setcurve(q,cv[1],cv[2]);
  E=ellinit([cv[1],cv[2]],q);Ed=twistcurve(cv[1],cv[2]);
  rts=if(cv[3]>0,apply(lift,polrootsmod('x^3+CA*'x+CB,Q)),[]);
  [q,cv[1],cv[2],cv[4],cv[5],cv[6],cv[3],cv[7],E,Ed,rts,Wp,Wp+10*(cidx+1)]};
\\ world for an explicit prime (toy)
worldq(q,Wp,cidx)={my(cv,E,Ed,rts);
  cv=mkcurve(q,Wp,cidx);setcurve(q,cv[1],cv[2]);
  E=ellinit([cv[1],cv[2]],q);Ed=twistcurve(cv[1],cv[2]);
  rts=if(cv[3]>0,apply(lift,polrootsmod('x^3+CA*'x+CB,Q)),[]);
  [q,cv[1],cv[2],cv[4],cv[5],cv[6],cv[3],cv[7],E,Ed,rts,Wp,Wp+10*(cidx+1)]};

\\ full run of Detector A on one target; returns [codes, Set]
runA(m,tab,Mx,V,CL,RS,xR,cR,rR)=if(m==2,solve2(Mx,V,CL,RS,xR,cR,rR),solve3(tab,V,CL,RS,xR,cR,rR));

\\ ================================================================ STAGE 0: oracle agreement V0
stage_oracle()={
  my(bits=12,fbs=["interval","random","twistmult","farey"],W,V,vd,CL,RS,tab,Mx,T,tg,codesA,codesB,codesG,sB,sG,cs,ok,nbad=0,nlines=0,Rp,xR,yR,Sset,cA_,cS_,mm,symv=0,nt0=20,nfail,rcb,gcl,tc,nn=96);
  W=world(bits,0,0);
  print("# STAGE0 oracle q=",W[1]);
  for(cidx=0,3,
    W=world(bits,0,cidx);
    setupFF();
    T=mktargets(W[9],W[4],W[13],200);
    for(fi=1,#fbs,
      V=mkfb(fbs[fi],nn,W[9],W[10],W[13]);
      vd=vdata(V,CA,CB);CL=vd[1];RS=vd[2];Mx=mkmap(V);
      tab=mktab(V,CL,RS);
      rcb=rescheck(V,T[1][1],200);
      print("RESCHECK\t",cidx,"\t",fbs[fi],"\t",rcb);
      if(rcb>0,nbad++);
      for(mm=2,3,
        for(t=1,nt0,
          xR=T[t][1];yR=T[t][2];Rp=[xR*GG^0,yR*GG^0];
          codesA=runA(mm,tab,Mx,V,CL,RS,xR,1,sqrtI(fx(xR)));
          if(mm==2,sB=B2(V,xR);symv=0,cs=B3(V,xR);sB=cs[1];symv=cs[2]);
          cs=if(mm==2,G2(V,Rp),G3(V,Rp));sG=cs[1];gcl=cs[2];
          Sset=oracleS(V,CL,Mx,xR,1,W[11],mm);
          cA_=classify(codesA,mm,#V,V,CL,1,xR);
          cS_=classify(Sset,mm,#V,V,CL,1,xR);
          ok=(codesA==sB)&&(codesA==sG)&&(gcl==CL)&&(symv==0)&&(cA_[3]==cS_[3])&&(cA_[4]==cS_[4]);
          if(!ok,nbad++);
          tsv(["V0",cidx,fbs[fi],mm,t,#V,#codesA,#sB,#sG,cA_[1],cA_[2],cA_[3],cA_[4],cS_[3],cS_[4],symv,ok]);nlines++))));
  print("V0_LINES\t",nlines);
  print("V0_TOY_BEGIN");
  nbad+=stage_toy();
  print("V0_RESULT\t",if(nbad==0,"PASS","FAIL"),"\tbad=",nbad);
  nbad};

\\ supplementary V0-toy: V = all of F_q for two tiny primes (saturated regime; B and G feasible)
stage_toy()={
  my(qs=[101,103],V,vd,CL,RS,tab,Mx,T,W,codesA,sB,sG,cs,Rp,xR,yR,Sset,cA_,cS_,symv,ok,nbad=0,gcl,mm,rcb);
  for(qi=1,#qs,for(cidx=0,3,
    W=worldq(qs[qi],S0+999000+1000*qi,cidx);
    setupFF();
    T=mktargets(W[9],W[4],W[13],200);
    V=vector(Q,i,i-1);vd=vdata(V,CA,CB);CL=vd[1];RS=vd[2];Mx=mkmap(V);tab=mktab(V,CL,RS);
    rcb=rescheck(V,T[1][1],200);if(rcb>0,nbad++);
    for(mm=2,3,for(t=1,20,
      xR=T[t][1];yR=T[t][2];Rp=[xR*GG^0,yR*GG^0];
      codesA=runA(mm,tab,Mx,V,CL,RS,xR,1,sqrtI(fx(xR)));
      if(mm==2,sB=B2(V,xR);symv=0,cs=B3(V,xR);sB=cs[1];symv=cs[2]);
      cs=if(mm==2,G2(V,Rp),G3(V,Rp));sG=cs[1];gcl=cs[2];
      Sset=oracleS(V,CL,Mx,xR,1,W[11],mm);
      cA_=classify(codesA,mm,#V,V,CL,1,xR);cS_=classify(Sset,mm,#V,V,CL,1,xR);
      ok=(codesA==sB)&&(codesA==sG)&&(gcl==CL)&&(symv==0)&&(cA_[3]==cS_[3])&&(cA_[4]==cS_[4]);
      if(!ok,nbad++);
      tsv(["V0TOY",Q,Str("nT=",W[7]),cidx,mm,t,#V,#codesA,#sB,#sG,cA_[1],cA_[2],cA_[3],cA_[4],cS_[3],cS_[4],symv,ok])))));
  nbad};

\\ ================================================================ STAGE 1: controls (PREREG section 6)
\\ planted-tuple presence: tuple of x values, tab etc. on V (already containing the tuple)
findcode3(V,xs)={my(n=#V,Mx=mkmap(V),id=vector(3),j);for(u=1,3,if(!mapisdefined(Mx,xs[u],&j),return(-1));id[u]=j);mkcode3(n,id[1],id[2],id[3])};
findcode2(V,xs)={my(n=#V,Mx=mkmap(V),id=vector(2),j);for(u=1,2,if(!mapisdefined(Mx,xs[u],&j),return(-1));id[u]=j);mkcode2(n,id[1],id[2])};
inV(M,xv)=mapisdefined(M,xv);
hasp(part,s)=#select(x->x==s,strsplit(part,","))>0;

\\ env G3_CPART (default "pc,nc1,nc3"), G3_CBITS (default "12,16,20,24"); positive controls run first inside a process.
\\ returns [nbad_PC, nbad_other]
stage_controls()={
  my(part=genv("G3_CPART","pc,nc1,nc3"),bits_l=eval(Str("[",genv("G3_CBITS","12,16,20,24"),"]")),
     nbadpc=0,nbadoth=0,W,E,Ed,T,xR,yR,Rp,baseV,nb,Mxb,V,vd,CL,RS,tab,Mx,codes,cA_,cS_,Sset,ok,found,cc,nt,
     x1,x2,x3,P1,P2,P3,Pp,e,tries,cnt,sens,mm,Mxt,t0);
  print("# STAGE1 controls part=",part," bits=",bits_l);
  \\ ---------------- PC1/PC2/PC3 (positive controls first)
  if(hasp(part,"pc"),
  for(bi=1,#bits_l,for(cidx=0,2,
    W=world(bits_l[bi],0,cidx);E=W[9];Ed=W[10];setupFF();
    T=mktargets(E,W[4],W[13],200);
    nb=min(ceil(Q^(1/2)),400);
    baseV=mkfb("random",nb,E,Ed,W[13]);Mxb=mkmap(baseV);
    \\ target: T[1] unless x_R is already in baseV (robustness; DEVIATIONS.md D1): first target with x_R not in baseV
    t0=1;while(inV(Mxb,T[t0][1]),t0++);
    xR=T[t0][1];yR=T[t0][2];Rp=[xR*GG^0,yR*GG^0];
    tsv(["PCW",bits_l[bi],cidx,Q,Str("nT=",W[7]),#baseV,t0]);
    \\ PC1 (needs rational 2-torsion): per T
    if(W[7]>0,for(r=1,#W[11],e=W[11][r];
      setrand(W[13]+51+r);cnt=0;sens=0;tries=0;
      while(cnt<50,
        Pp=random(Ed);if(Pp==[0]||Pp[2]==0,next);
        x1=lift(Pp[1]*DINV);x2=tauT(e,x1);x3=tauT(e,xR);
        if(x1==x2||x1==x3||x2==x3||x1==xR||x2==xR||x3==xR,next);
        if(inV(Mxb,x1)||inV(Mxb,x2)||inV(Mxb,x3),next);
        cnt++;
        V=concat(baseV,[x1,x2,x3]);vd=vdata(V,CA,CB);CL=vd[1];RS=vd[2];Mx=mkmap(V);tab=mktab(V,CL,RS);
        codes=solve3(tab,V,CL,RS,xR,1,sqrtI(fx(xR)));
        cc=findcode3(V,[x1,x2,x3]);
        cA_=classify(codes,3,#V,V,CL,1,xR);Sset=oracleS(V,CL,Mx,xR,1,W[11],3);cS_=classify(Sset,3,#V,V,CL,1,xR);
        found=(cc>0&&setsearch(codes,cc)>0);
        nt=if(found,classify(Set([cc]),3,#V,V,CL,1,xR),[0,0,0,0]);
        ok=found&&nt[3]==1&&Gconf([x1,x2,x3],Rp)&&(cA_[3]==cS_[3])&&(cA_[4]==cS_[4])&&(CL[#baseV+1]==-1)&&(CL[#baseV+2]==-1)&&(CL[#baseV+3]==1);
        if(!ok,nbadpc++);
        tsv(["PC1",bits_l[bi],cidx,e,cnt,found,nt[3],Gconf([x1,x2,x3],Rp),cA_[3],cS_[3],cA_[4],cS_[4],ok])));
    );
    \\ PC2 planted degenerate mixed {x',x',xR}
    setrand(W[13]+61);cnt=0;
    while(cnt<50,
      Pp=random(Ed);if(Pp==[0]||Pp[2]==0,next);x1=lift(Pp[1]*DINV);
      if(inV(Mxb,x1)||inV(Mxb,xR),next);
      cnt++;
      V=concat(baseV,[x1,xR]);vd=vdata(V,CA,CB);CL=vd[1];RS=vd[2];Mx=mkmap(V);tab=mktab(V,CL,RS);
      codes=solve3(tab,V,CL,RS,xR,1,sqrtI(fx(xR)));
      cc=findcode3(V,[x1,x1,xR]);
      cA_=classify(codes,3,#V,V,CL,1,xR);Sset=oracleS(V,CL,Mx,xR,1,W[11],3);cS_=classify(Sset,3,#V,V,CL,1,xR);
      found=(cc>0&&setsearch(codes,cc)>0);
      nt=if(found,classify(Set([cc]),3,#V,V,CL,1,xR),[0,0,0,0]);
      ok=found&&nt[4]==1&&Gconf([x1,x1,xR],Rp)&&(cA_[3]==cS_[3])&&(cA_[4]==cS_[4]);
      if(!ok,nbadpc++);
      tsv(["PC2",bits_l[bi],cidx,0,cnt,found,nt[4],Gconf([x1,x1,xR],Rp),cA_[3],cS_[3],cA_[4],cS_[4],ok]));
    \\ PC3 planted E-only m=3 and m=2
    for(mm=2,3,
      setrand(W[13]+70+mm);cnt=0;
      while(cnt<50,
        P1=random(E);P2=random(E);
        if(P1==[0]||P2==[0],next);
        Rpt=[Mod(xR,Q),Mod(yR,Q)];
        if(mm==3,P3=elladd(E,Rpt,ellneg(E,elladd(E,P1,P2))),P3=[0]);
        if(mm==2,P2=elladd(E,Rpt,ellneg(E,P1)));
        if(P2==[0]||(mm==3&&P3==[0]),next);
        x1=lift(P1[1]);x2=lift(P2[1]);
        if(mm==3,x3=lift(P3[1]));
        if(mm==3,if(x1==x2||x1==x3||x2==x3||x1==xR||x2==xR||x3==xR,next),if(x1==x2||x1==xR||x2==xR,next));
        if(inV(Mxb,x1)||inV(Mxb,x2)||(mm==3&&inV(Mxb,x3)),next);
        cnt++;
        V=if(mm==3,concat(baseV,[x1,x2,x3]),concat(baseV,[x1,x2]));
        vd=vdata(V,CA,CB);CL=vd[1];RS=vd[2];Mx=mkmap(V);tab=mktab(V,CL,RS);
        codes=runA(mm,tab,Mx,V,CL,RS,xR,1,sqrtI(fx(xR)));
        cc=if(mm==3,findcode3(V,[x1,x2,x3]),findcode2(V,[x1,x2]));
        found=(cc>0&&setsearch(codes,cc)>0);
        nt=if(found,classify(Set([cc]),mm,#V,V,CL,1,xR),[0,0,0,0]);
        ok=found&&nt[1]==1&&Gconf(if(mm==3,[x1,x2,x3],[x1,x2]),Rp);
        if(!ok,nbadpc++);
        tsv(["PC3",bits_l[bi],cidx,mm,cnt,found,nt[1],Gconf(if(mm==3,[x1,x2,x3],[x1,x2]),Rp),ok])))));
  );
  \\ ---------------- NC1: E-only factor base, x(kG), k=1..n
  if(hasp(part,"nc1"),
  for(bi=1,#bits_l,for(cidx=0,2,
    W=world(bits_l[bi],0,cidx);E=W[9];Ed=W[10];setupFF();
    T=mktargets(E,W[4],W[13],200);
    setrand(W[13]+81);
    my(G=random(E),P,L=List(),Mm=Map(),xv,nn=100);
    while(G==[0],G=random(E));P=G;
    for(k=1,nn,if(P==[0],break);xv=lift(P[1]);if(!mapisdefined(Mm,xv),mapput(Mm,xv,1);listput(L,xv));P=elladd(E,P,G));
    V=Vec(L);vd=vdata(V,CA,CB);CL=vd[1];RS=vd[2];Mx=mkmap(V);tab=mktab(V,CL,RS);
    for(mm=2,3,for(t=1,20,
      xR=T[t][1];yR=T[t][2];Rp=[xR*GG^0,yR*GG^0];
      codes=runA(mm,tab,Mx,V,CL,RS,xR,1,sqrtI(fx(xR)));
      my(sB_,sG_,cs_);
      if(mm==2,sB_=B2(V,xR),cs_=B3(V,xR);sB_=cs_[1]);
      cs_=if(mm==2,G2(V,Rp),G3(V,Rp));sG_=cs_[1];
      cA_=classify(codes,mm,#V,V,CL,1,xR);
      ok=(cA_[3]==0)&&(cA_[4]==0)&&(codes==sB_)&&(codes==sG_)&&(vecmin(CL)>=0);
      if(!ok,nbadoth++);
      tsv(["NC1",bits_l[bi],cidx,mm,t,#V,#codes,#sB_,#sG_,cA_[1],cA_[2],cA_[3],cA_[4],ok])));
    \\ A-only at grid size n=min(ceil(q^0.65),1500), detector only: mixed must be 0
    nn=alphaN(13/20,1500);
    P=G;L=List();Mm=Map();
    for(k=1,nn,if(P==[0],break);xv=lift(P[1]);if(!mapisdefined(Mm,xv),mapput(Mm,xv,1);listput(L,xv));P=elladd(E,P,G));
    V=Vec(L);vd=vdata(V,CA,CB);CL=vd[1];RS=vd[2];Mx=mkmap(V);tab=mktab(V,CL,RS);
    for(mm=2,3,for(t=1,if(mm==2,50,20),
      xR=T[t][1];
      codes=runA(mm,tab,Mx,V,CL,RS,xR,1,sqrtI(fx(xR)));
      cA_=classify(codes,mm,#V,V,CL,1,xR);
      ok=(cA_[3]==0)&&(cA_[4]==0);if(!ok,nbadoth++);
      tsv(["NC1big",bits_l[bi],cidx,mm,t,#V,#codes,cA_[1],cA_[2],cA_[3],cA_[4],ok]))))));
  \\ ---------------- NC3: target on the twist (mirror); bits 12,16,20 ; cidx 0..5 ; FB random,twistmult ; alpha 0.65
  if(hasp(part,"nc3"),
  for(bi=1,3,for(cidx=0,5,
    W=world([12,16,20][bi],0,cidx);E=W[9];Ed=W[10];
    T=mktargetsW(Ed,W[5],W[13],200);
    my(fbl=["random","twistmult"],nn,cR_,rR_);
    for(fi=1,#fbl,
      for(mm=2,3,
        my(nT_=if(mm==2,200,100),tabN,nEnd=0,nEdeg=0,nMnd=0,nMdeg=0,Snd=0,Sdeg=0,mism=0,nWl);
        nn=alphaN(13/20,if(mm==3,1500,16384));
        V=mkfb(fbl[fi],nn,E,Ed,W[13]);vd=vdata(V,CA,CB);CL=vd[1];RS=vd[2];Mx=mkmap(V);
        if(mm==3,tabN=mktab(V,CL,RS));
        for(t=1,nT_,xR=T[t][1];
          cR_=kronecker(fx(xR),Q);rR_=if(cR_==-1,sqrtI((fx(xR)*DINV)%Q),0);
          if(cR_!=-1,error("NC3 target not on twist"));
          codes=if(mm==2,solve2(Mx,V,CL,RS,xR,cR_,rR_),solve3(tabN,V,CL,RS,xR,cR_,rR_));
          cA_=classify(codes,mm,#V,V,CL,-1,xR);
          Sset=oracleS(V,CL,Mx,xR,-1,W[11],mm);cS_=classify(Sset,mm,#V,V,CL,-1,xR);
          nEnd+=cA_[1];nEdeg+=cA_[2];nMnd+=cA_[3];nMdeg+=cA_[4];Snd+=cS_[3];Sdeg+=cS_[4];
          if(cA_[3]!=cS_[3]||cA_[4]!=cS_[4],mism++));
        nWl=#select(c->c==-1,CL);
        if(mism>0||(mm==2&&(nMnd+nMdeg)>0),nbadoth++);
        tsv(["NC3",[12,16,20][bi],cidx,Str("nT=",W[7]),fbl[fi],mm,nT_,#V,nWl,nEnd,nEdeg,nMnd,nMdeg,Snd,Sdeg,mism]))))));
  print("CONTROLS_RESULT\tpart=",part,"\tPCbad=",nbadpc,"\tOTHERbad=",nbadoth);
  [nbadpc,nbadoth]};

\\ ================================================================ STAGE 2: grid job
stage_grid()={
  my(bits=geni("G3_BITS",12),pidx=geni("G3_PIDX",0),cidx=geni("G3_CURVE",0),mm=geni("G3_M",2),fb=genv("G3_FB","random"),
     NT=geni("G3_TARGETS",if(mm==2,200,100)),guard=geni("G3_GUARD_S",120),
     W,E,Ed,T,V,vd,CL,RS,tab,Mx,als=[1/2,13/20,4/5],alnames=["0.5","0.65","0.8"],cap=if(mm==2,16384,1500),
     n,nEl,nWl,nT2l,t0,t1,tabsec,done,trunc,codes,cA_,cS_,Sset,xR,tag,seen=List(),xin,tstart);
  W=world(bits,pidx,cidx);E=W[9];Ed=W[10];
  T=mktargets(E,W[4],W[13],200);
  tstart=getwalltime();
  for(ai=1,3,
    n=alphaN(als[ai],cap);
    if(setsearch(Set(Vec(seen)),n),
      print("DUP\t",bits,"\t",pidx,"\t",cidx,"\t",mm,"\t",fb,"\t",alnames[ai],"\tn=",n,"\tsame_as_earlier_alpha");next);
    listput(seen,n);
    tag=Str(bits,"_",pidx,"_",cidx,"_m",mm,"_",fb,"_",alnames[ai]);
    V=mkfb(fb,n,E,Ed,W[13]);vd=vdata(V,CA,CB);CL=vd[1];RS=vd[2];Mx=mkmap(V);
    nEl=#select(c->c==1,CL);nWl=#select(c->c==-1,CL);nT2l=#select(c->c==0,CL);
    t0=getwalltime();
    if(mm==3,tab=mktab(V,CL,RS));
    tabsec=(getwalltime()-t0)/1000.;
    tsv(["CFG",tag,bits,pidx,cidx,mm,fb,alnames[ai],W[1],W[2],W[3],W[4],W[5],W[6],W[7],W[8],#V,nEl,nWl,nT2l,S0,W[13],NT,guard,cap]);
    done=0;trunc=0;t0=getwalltime();
    for(t=1,NT,
      if((getwalltime()-t0)/1000.>guard,trunc=1;break);
      xR=T[t][1];
      codes=runA(mm,tab,Mx,V,CL,RS,xR,1,sqrtI(fx(xR)));
      cA_=classify(codes,mm,#V,V,CL,1,xR);
      Sset=oracleS(V,CL,Mx,xR,1,W[11],mm);cS_=classify(Sset,mm,#V,V,CL,1,xR);
      xin=if(mapisdefined(Mx,xR),1,0);
      tsv(["TGT",tag,t,cA_[1],cA_[2],cA_[3],cA_[4],cS_[3],cS_[4],xin]);
      done++);
    t1=(getwalltime()-t0)/1000.;
    tsv(["END",tag,done,NT,tabsec,t1,trunc]));
  tsv(["JOBEND",bits,pidx,cidx,mm,fb,(getwalltime()-tstart)/1000.]);
  0};


\\ targets with an explicit seed (resolution blocks): same construction as mktargets
mktargetsS(E,cE,seed,N)={my(T=vector(N),R,P1,Q1);setrand(seed);
  for(t=1,N,while(1,P1=random(E);Q1=random(E);
    R=elladd(E,ellmul(E,P1,1+random(cE)),ellmul(E,Q1,1+random(cE)));
    if(R==[0],next);if(ellmul(E,R,2)==[0],next);break);
    T[t]=[lift(R[1]),lift(R[2])]);T};

\\ ================================================================ STAGE 2b: NC2-calibrated (PREREG section 6)
\\ Semaev polynomials of the curve with b'=b+1 (b+2 if singular); x_i classified by the ORIGINAL f; non-degenerate solutions only
stage_nc2()={
  my(bits=geni("G3_BITS",12),pidx=geni("G3_PIDX",0),cidx=geni("G3_CURVE",0),mm=geni("G3_M",2),fb=genv("G3_FB","random"),
     NT=geni("G3_TARGETS",if(mm==2,200,100)),guard=geni("G3_GUARD_S",120),cap=if(mm==2,16384,1500),
     W,E,Ed,T,V,vd,CL,CLS,RSS,Mx,tab,n,nEl,nWl,nT2l,bs,xR,fS,cR_,rR_,codes,cA_,ntt,done=0,trunc=0,t0,t1,tabsec,tag);
  W=world(bits,pidx,cidx);E=W[9];Ed=W[10];
  T=mktargets(E,W[4],W[13],200);
  n=alphaN(13/20,cap);
  tag=Str(bits,"_",pidx,"_",cidx,"_m",mm,"_",fb,"_0.65");
  V=mkfb(fb,n,E,Ed,W[13]);vd=vdata(V,CA,CB);CL=vd[1];
  nEl=#select(c->c==1,CL);nWl=#select(c->c==-1,CL);nT2l=#select(c->c==0,CL);
  bs=CB+1;while((4*CA^3+27*bs^2)%Q==0,bs++);
  SEMB=bs;
  vd=vdata(V,CA,bs);CLS=vd[1];RSS=vd[2];Mx=mkmap(V);
  t0=getwalltime();
  if(mm==3,tab=mktab(V,CLS,RSS));
  tabsec=(getwalltime()-t0)/1000.;
  tsv(["NCCFG",tag,bits,pidx,cidx,mm,fb,"0.65",W[1],W[2],W[3],bs,W[4],W[5],W[6],W[7],W[8],#V,nEl,nWl,nT2l,S0,W[13],NT,guard,cap]);
  t0=getwalltime();
  for(t=1,NT,
    if((getwalltime()-t0)/1000.>guard,trunc=1;break);
    xR=T[t][1];fS=fAB(xR,CA,bs);cR_=kronecker(fS,Q);
    rR_=if(cR_==1,sqrtI(fS),if(cR_==-1,sqrtI((fS*DINV)%Q),0));
    codes=if(mm==2,solve2(Mx,V,CLS,RSS,xR,cR_,rR_),solve3(tab,V,CLS,RSS,xR,cR_,rR_));
    cA_=classify(codes,mm,#V,V,CL,1,xR);
    ntt=#V-if(mapisdefined(Mx,xR),1,0);
    tsv(["NCT",tag,t,cA_[1],cA_[3],ntt,nWl]);
    done++);
  t1=(getwalltime()-t0)/1000.;
  tsv(["NCEND",tag,done,NT,tabsec,t1,trunc]);
  0};

\\ ================================================================ STAGE 2c: resolution blocks (PREREG section 5), m=3, compact per-target lines
stage_res()={
  my(bits=geni("G3_BITS",20),pidx=geni("G3_PIDX",0),cidx=geni("G3_CURVE",2),fb=genv("G3_FB","random"),blk=geni("G3_BLOCK",0),
     NT=geni("G3_TARGETS",10000),guard=geni("G3_GUARD_S",800),mm=3,
     W,E,Ed,T,V,vd,CL,RS,tab,Mx,n,nEl,nWl,nT2l,al,xR,codes,cA_,cS_,Sset,done=0,trunc=0,t0,t1,tabsec,tag,seed);
  W=world(bits,pidx,cidx);E=W[9];Ed=W[10];
  al=if(bits==20,13/20,1/2);n=alphaN(al,1500);
  tag=Str(bits,"_",pidx,"_",cidx,"_",fb,"_b",blk);
  V=mkfb(fb,n,E,Ed,W[13]);vd=vdata(V,CA,CB);CL=vd[1];RS=vd[2];Mx=mkmap(V);
  nEl=#select(c->c==1,CL);nWl=#select(c->c==-1,CL);nT2l=#select(c->c==0,CL);
  t0=getwalltime();tab=mktab(V,CL,RS);tabsec=(getwalltime()-t0)/1000.;
  seed=W[13]+10^9+1000*blk;
  T=mktargetsS(E,W[4],seed,NT);
  tsv(["RCFG",tag,bits,pidx,cidx,mm,fb,blk,W[1],W[2],W[3],W[4],W[5],W[6],W[7],W[8],#V,nEl,nWl,nT2l,S0,seed,NT,guard,tabsec]);
  t0=getwalltime();
  for(t=1,NT,
    if((getwalltime()-t0)/1000.>guard,trunc=1;break);
    xR=T[t][1];
    codes=solve3(tab,V,CL,RS,xR,1,sqrtI(fx(xR)));
    cA_=classify(codes,3,#V,V,CL,1,xR);
    Sset=oracleS(V,CL,Mx,xR,1,W[11],3);cS_=classify(Sset,3,#V,V,CL,1,xR);
    tsv(["R",t,cA_[1],cA_[3],cS_[3],cA_[4],cS_[4]]);
    done++);
  t1=(getwalltime()-t0)/1000.;
  tsv(["REND",tag,done,NT,tabsec,t1,trunc]);
  0};

\\ ================================================================ dispatch
if(STAGE=="oracle",r=stage_oracle();if(r>0,quit(1)));
if(STAGE=="controls",r=stage_controls();if(r[1]>0,quit(1)));
if(STAGE=="grid",stage_grid());
if(STAGE=="nc2",stage_nc2());
if(STAGE=="res",stage_res());
