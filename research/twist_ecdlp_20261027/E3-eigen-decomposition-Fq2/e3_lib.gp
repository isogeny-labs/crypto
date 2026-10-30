\\ e3_lib.gp -- shared library for E3-eigen-decomposition-Fq2 (all numbers come from running these functions)
\\ Conventions: p prime, E: y^2=x^3+ax+b over F_p, #E = Cl (prime), #E' = Clp (odd, cyclic group), d = smallest non-residue,
\\ F_ p^2  = F_p[t]/(t^2-d).  Factor base V =  0..N-1 .  E-type x: f(x) nonzero square -> (x,y) in E(F_p).
\\ T-type x: f(x) non-square, f(x)=d*s^2 -> (x,s*t) in E(F_ p^2 ), pi acts as -1.  Twist E_T: Y^2=X^3+a d^2 X+b d^3, (X,Y)=(d x, d^2 s).
\\ (callers must run default(parisizemax,2000000000) BEFORE read of this file)
\\ polynomial variables are created BEFORE the finite-field generator 't so that polrootsmod/factor accept ffelt coefficients
w='w; u='u; y1='y1; y2='y2; y3='y3; y4='y4;

nonres(p) = {my(d=2); while(kronecker(d,p)!=-1, d++); d}

\\ ---------------------------------------------------------------- context
ctx(p,a,b) = {
  Cp=p; Ca=a; Cb=b; Cd=nonres(p); Cinvd=lift(1/Mod(Cd,p));
  Ct=ffgen(Mod(1,p)*('t^2-Cd),'t);
  CE=ellinit([a,b],p);
  CET=ellinit([a*Cd^2%p, b*Cd^3%p],p);
  CE2=ellinit([a*Ct^0, b*Ct^0]);
  Cl=ellcard(CE); Clp=ellcard(CET);
  mklogs();
}

mklogs() = {
  my(xg,M,Pk,xi,rec,Pq,Mt);
  Ctype=vectorsmall(Cp); Cyv=vector(Cp); Cu=vectorsmall(Cp); Cw=vectorsmall(Cp);
  for(x=0,Cp-1,
    my(f=(x^3+Ca*x+Cb)%Cp);
    if(f==0, Ctype[x+1]=0,
      if(kronecker(f,Cp)==1,
        Ctype[x+1]=1; Cyv[x+1]=lift(sqrt(Mod(f,Cp))),
        Ctype[x+1]=-1; Cyv[x+1]=lift(sqrt(Mod(f*Cinvd,Cp))))));
  \\ E side: Cl is prime, any non-O point generates
  xg=0; while(Ctype[xg+1]!=1, xg++);
  CGE=[Mod(xg,Cp),Mod(Cyv[xg+1],Cp)];
  M=Map(); Pk=[0];
  for(k=1,Cl-1, Pk=elladd(CE,Pk,CGE); if(!mapisdefined(M,lift(Pk[1])), mapput(M,lift(Pk[1]),[k,lift(Pk[2])])));
  for(x=0,Cp-1, if(Ctype[x+1]==1,
     rec=mapget(M,x);
     Cu[x+1]=if(rec[2]==Cyv[x+1], rec[1], Cl-rec[1])));
  \\ twist side: generator of E_T(F_p) (cyclic of order Clp)
  CGT=0;
  for(x=0,Cp-1, if(Ctype[x+1]==-1,
     Pq=[Mod(Cd*x,Cp),Mod(Cd^2*Cyv[x+1],Cp)];
     if(ellorder(CET,Pq)==Clp, CGT=Pq; break)));
  Mt=Map(); Pk=[0];
  for(k=1,Clp-1, Pk=elladd(CET,Pk,CGT); if(Pk!=[0] && !mapisdefined(Mt,lift(Pk[1])), mapput(Mt,lift(Pk[1]),[k,lift(Pk[2])])));
  for(x=0,Cp-1, if(Ctype[x+1]==-1,
     rec=mapget(Mt,(Cd*x)%Cp);
     Cw[x+1]=if(rec[2]==(Cd^2*Cyv[x+1])%Cp, rec[1], Clp-rec[1])));
  \\ generators as points of E(F_p2): E-gen and the image of the twist generator under (X,Y)->(X/d, Y/(d t))
  CGE2=[lift(CGE[1])*Ct^0, lift(CGE[2])*Ct^0];
  CGT2=[(lift(CGT[1])*Cinvd%Cp)*Ct^0, (lift(CGT[2])*Cinvd%Cp)*Ct^(-1)];
}

\\ points of E(F_ p^2 ) for x in V
ptx2(x) = if(Ctype[x+1]==1, [x*Ct^0, Cyv[x+1]*Ct^0], [x*Ct^0, Cyv[x+1]*Ct])

\\ ---------------------------------------------------------------- sign-sum sets
sums_of(vals, md) = {
  my(S=[0]);
  for(i=1,#vals, my(v=vals[i]); S=concat(vector(#S,k,(S[k]+v)%md), vector(#S,k,(S[k]-v)%md)));
  S
}

\\ all k-subsets of idx (positions) with: bitmask over x, Set of signed sums mod md, raw list of 2^k signed sums
subtab(idx, vals, k, md) = {
  my(n=#idx, out=List());
  if(k>n, return([]));
  if(k==0, return([[0,[0],[0]]]));
  forsubset([n,k], s,
    my(mk=0, vv=vector(k,i,vals[s[i]]));
    for(i=1,k, mk+=2^idx[s[i]]);
    my(raw=sums_of(vv,md));
    listput(out,[mk,Vec(Set(raw)),raw]));
  Vec(out)
}

cntv(v,val) = {my(c=0); for(i=1,#v, if(v[i]==val,c++)); c}

\\ work counter in the Semaev unit: sum over r-subsets of an n-set of 2^r (counted by loop, not by formula)
workloop(n, r) = {my(c=0); if(r>n, return(0)); forsubset([n,r], s, c+=2^r); c}

\\ ---------------------------------------------------------------- the table pass
\\ Av[ti] in 1..l-1 (A part), Bv[ti] in 1..lp-1 (B part), B2v[ti] independent redraw (NC2).
\\ Returns vector:
\\  1 NE  2 NT  3 Dc(T x msz)  4 Tc(T x msz)  5 Xc(T x msz)  6 Xc2  7 X0c  8 mult  9 nhits  10 p5viol  11 nc1viol
\\  12 factor mismatches  13 WD (code, (msz-1)-subsets of E-type)  14 WX (code, (msz-1)-subsets of V)
\\  15 nc3 main hits (E-only base, B!=0)  16 nc3 B=0 hits  17 hits list [target,mask,j,Emask]  18 D list [target,j,mask]
cellrun(N, msz, Av, Bv, B2v) = {
  my(T=#Av, Eidx=List(), Tidx=List(), NE, NT, Ev, Tv, subE, subT);
  for(x=0,N-1, if(Ctype[x+1]==1, listput(Eidx,x), if(Ctype[x+1]==-1, listput(Tidx,x))));
  Eidx=Vec(Eidx); Tidx=Vec(Tidx); NE=#Eidx; NT=#Tidx;
  Ev=vector(NE,i,Cu[Eidx[i]+1]); Tv=vector(NT,i,Cw[Tidx[i]+1]);
  subE=vector(msz+1,j,subtab(Eidx,Ev,j-1,Cl));
  subT=vector(msz+1,k,subtab(Tidx,Tv,k-1,Clp));
  my(dhead=vectorsmall(Cl), dnxt=vectorsmall(T), thead=vectorsmall(Clp), tnxt=vectorsmall(T));
  for(ti=1,T, dnxt[ti]=dhead[Av[ti]+1]; dhead[Av[ti]+1]=ti; tnxt[ti]=thead[Bv[ti]+1]; thead[Bv[ti]+1]=ti);
  my(sz=Cl*Clp, h1=vectorsmall(sz), n1=vectorsmall(T), h2=vectorsmall(sz), n2=vectorsmall(T), h0=vectorsmall(sz), n0=vectorsmall(T));
  for(ti=1,T,
    my(k1=Av[ti]*Clp+Bv[ti]+1, k2=Av[ti]*Clp+B2v[ti]+1, k0=Av[ti]*Clp+1);
    n1[ti]=h1[k1]; h1[k1]=ti; n2[ti]=h2[k2]; h2[k2]=ti; n0[ti]=h0[k0]; h0[k0]=ti);
  \\ D pass: E-type j-subsets, key A
  my(Dc=matrix(T,msz), Dmap=Map(), Dl=List(), Tc=matrix(T,msz));
  for(j=1,msz, my(L=subE[j+1]);
    for(r=1,#L, my(rc=L[r]);
      for(q=1,#rc[2], my(tt=dhead[rc[2][q]+1]);
        while(tt, Dc[tt,j]++; mapput(Dmap,(rc[1]*(msz+1)+j)*(T+1)+tt,1); listput(Dl,[tt,j,rc[1]]); tt=dnxt[tt]))));
  \\ T pass: T-type k-subsets, key B
  for(k=1,msz, my(L=subT[k+1]);
    for(r=1,#L, my(rc=L[r]);
      for(q=1,#rc[2], my(tt=thead[rc[2][q]+1]);
        while(tt, Tc[tt,k]++; tt=tnxt[tt]))));
  \\ X pass over all x-sets of size msz with j E-type and k=msz-j T-type terms (j>=1,k>=1; j=0 or k=0 cannot hit A,B nonzero)
  my(Xc=matrix(T,msz), Xc2=matrix(T,msz), X0c=matrix(T,msz), hits=List(), mult=0, nh=0, p5=0, nc1=0);
  for(j=1,msz-1, my(k=msz-j, LE=subE[j+1], LT=subT[k+1]);
    for(r=1,#LE, my(re=LE[r]);
      for(s=1,#LT, my(ru=LT[s]);
        for(q1=1,#re[2], my(a=re[2][q1]);
          for(q2=1,#ru[2], my(b=ru[2][q2], key=a*Clp+b+1, tt=h1[key]);
            while(tt,
              Xc[tt,j]++; nh++;
              listput(hits,[tt,re[1]+ru[1],j,re[1]]);
              mult+=cntv(re[3],a)*cntv(ru[3],b)-1;
              if(!mapisdefined(Dmap,(re[1]*(msz+1)+j)*(T+1)+tt), p5++);
              tt=n1[tt]);
            tt=h2[key];
            while(tt, Xc2[tt,j]++; tt=n2[tt]);
            if(b==0,
              tt=h0[key];
              while(tt, X0c[tt,j]++; if(!mapisdefined(Dmap,(re[1]*(msz+1)+j)*(T+1)+tt), nc1++); tt=n0[tt]))
            )))));
  \\ factorisation cross-check: X count = D_j(A) * T_k(B)
  my(fm=0);
  for(ti=1,T, for(j=1,msz-1, if(Xc[ti,j]!=Dc[ti,j]*Tc[ti,msz-j], fm++)));
  \\ NC3: E-only factor base, all-E x-sets of size msz, key (a,0)
  my(nc3m=0, nc3z=0, LE=subE[msz+1]);
  for(r=1,#LE, my(re=LE[r]);
    for(q=1,#re[2], my(key=re[2][q]*Clp+1, tt=h1[key]);
      while(tt, nc3m++; tt=n1[tt]);
      tt=h0[key];
      while(tt, nc3z++; tt=n0[tt])));
  \\ work counters
  my(WD=workloop(NE,msz-1), WX=workloop(N,msz-1));
  [NE,NT,Dc,Tc,Xc,Xc2,X0c,mult,nh,p5,nc1,fm,WD,WX,nc3m,nc3z,Vec(hits),Vec(Dl)]
}

\\ ---------------------------------------------------------------- target construction
\\ draw T pairs (A,B) and T redraws B2 from the stream seeded by setrand(seed)
draw_targets(seed, T) = {
  setrand(seed);
  my(Av=vectorsmall(T), Bv=vectorsmall(T), B2v=vectorsmall(T));
  for(i=1,T, Av[i]=1+random(Cl-1); Bv[i]=1+random(Clp-1));
  for(i=1,T, B2v[i]=1+random(Clp-1));
  [Av,Bv,B2v]
}

\\ point of E(F_ p^2 ) for target (A,B): A*GE + B*phi(GT)
target_pt(A, B) = elladd(CE2, ellmul(CE2,CGE2,A), ellmul(CE2,CGT2,B))

\\ ---------------------------------------------------------------- Semaev
S3(x1,x2,x3) = (x1-x2)^2*x3^2 - 2*((x1+x2)*(x1*x2+Ca)+2*Cb)*x3 + ((x1*x2-Ca)^2 - 4*Cb*(x1+x2))

semaev_init() = {
  CS4=polresultant(S3(y1,y2,u)*Mod(1,Cp), S3(y3,y4,u)*Mod(1,Cp), u);
  CS4w=substvec(CS4,[y1,y4],[u,w]);   \\ S4(u,y2,y3,w)
}

\\ D_m for m=2: all x-sets  x1,x2  subset of V (E-type x1 enumerated) with S3(x1,x2,xA)=0; returns Vec of masks (sorted, distinct)
semD2(xA, N) = {
  my(out=List(), q, rts, x2);
  for(x1=0,N-1,
    if(Ctype[x1+1]!=1, next);
    q=S3(x1,u,xA)*Mod(1,Cp);
    if(q==0, error("semD2: identically zero polynomial"));
    if(poldegree(q)<1, next);
    rts=polrootsmod(q,Cp);
    for(i=1,#rts, x2=lift(rts[i]); if(x2>=0 && x2<N && x2!=x1, listput(out,2^x1+2^x2))));
  Vec(Set(Vec(out)))
}

\\ X_4: all 4-sets of V with S5(x1,x2,x3,x4,XR)=0, XR in F_ p^2 ; one resultant + one root-solve per 3-subset
semX4(XR, N) = {
  my(out=List(), q, g, R, rts, v, x4);
  forsubset([N,3], s,
    my(x1=s[1]-1, x2=s[2]-1, x3=s[3]-1);
    q=S3(x1,x2,u)*Mod(1,Cp)*Ct^0;
    g=substvec(CS4w,[y2,y3],[x3,XR]);
    R=polresultant(q,g,u);
    if(R==0, error("semX4: identically zero polynomial"));
    if(poldegree(R,w)<1, next);
    rts=polrootsmod(R,[Ct.mod,Cp]);
    for(i=1,#rts,
      v=lift(rts[i]);
      if(type(v)=="t_POL", if(poldegree(v)>0, next); v=polcoeff(v,0));
      x4=lift(v);
      if(x4>=0 && x4<N && x4!=x1 && x4!=x2 && x4!=x3, listput(out,2^x1+2^x2+2^x3+2^x4))));
  Vec(Set(Vec(out)))
}
