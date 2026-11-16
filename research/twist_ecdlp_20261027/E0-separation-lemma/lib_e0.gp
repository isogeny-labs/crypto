\\ lib_e0.gp -- E0 separation lemma: library (field, twist, phi, classes, oracles, pair loop, Hom/isogeny helpers)
\\ Protocol: PREREGISTRATION.md (sha256 in PREREGISTRATION.sha256). PARI/GP only.
default(parisizemax,2000000000);
setrand(20261021);

ENVI(nm,def)={my(s=getenv(nm)); if(type(s)!="t_STR"||s=="",def,eval(s))};
ENVS(nm,def)={my(s=getenv(nm)); if(type(s)!="t_STR"||s=="",def,s)};
emit(v)={print(strjoin(apply(x->Str(x),v),"|"))};

\\ ---------------- globals ----------------
CHR_=0; Q_=0; M_=0; BIN_=0; ONE_=0; GEN_=0; SUBF_=[]; D_=0; DEL_=0; WIT_=0;
TRIV_=ENVI("E0_TRIV",0);

\\ ---------------- encoding / basic predicates ----------------
enc(z)={my(P=z.pol); if(type(P)!="t_POL",P,fromdigits(Vec(P),CHR_))};
ptenc(P)=if(#P==1,0,1+enc(P[1])*Q_^2+enc(P[2]));
ptstr(P)=if(#P==1,"O",Str("(",enc(P[1]),",",enc(P[2]),")"));
cstr(c)=strjoin(apply(x->Str(if(type(x)=="t_INT",x,enc(x))),c),",");
israt(P)=(#P==1)||(P[1]^Q_==P[1]&&P[2]^Q_==P[2]);
isneg(E,P)=(#P==1)||(P[1]^Q_==P[1]&&P[2]^Q_==ellneg(E,P)[2]);
oddpart(n)=n/2^valuation(n,2);
abstr(z)={my(s=0*ONE_,t=z); for(i=1,M_,s+=t; t=t^2); s};

\\ ---------------- field set-up ----------------
\\ odd p: F_{p^2}=ffgen(p^2), F_p = prime subfield (constants), d = smallest nonsquare, delta^2=d.
\\ binary: F_{q^2}=ffgen(2^(2m)), F_q = {z: z^q=z} = {0} U <g^(q+1)>, gamma = first abs-trace-1 element, s^2+s=gamma.
\\ TRIV_=1 (positive control PC1): d=1,delta=1 resp. gamma=0,s=0, so E'=E and phi=identity.
setfield(p,m)={
  CHR_=p; BIN_=(p==2); M_=if(BIN_,m,1); WIT_=0;
  if(BIN_, Q_=2^m; GEN_=ffprimroot(ffgen(2^(2*m),'z)), Q_=p; GEN_=ffprimroot(ffgen(p^2,'w)));
  ONE_=GEN_^0;
  if(BIN_,
    my(h=GEN_^(Q_+1));
    SUBF_=concat([0*ONE_],vector(Q_-1,i,h^(i-1)));
    if(TRIV_,
      D_=0*ONE_; DEL_=0*ONE_,
      my(gm=0*ONE_,ok=0);
      for(i=1,#SUBF_, if(abstr(SUBF_[i])==ONE_, gm=SUBF_[i]; ok=1; break));
      if(!ok,error("no trace-1 gamma"));
      D_=gm;
      my(f=factor('x^2+'x+gm));
      DEL_=polcoeff(f[1,1],0);
      if(DEL_^2+DEL_!=D_,error("s^2+s != gamma"));
      if(DEL_^Q_==DEL_,error("s in F_q"))),
    SUBF_=vector(p,i,(i-1)*ONE_);
    if(TRIV_,
      D_=1; DEL_=ONE_,
      my(d=2); while(kronecker(d,p)!=-1,d++);
      D_=d; DEL_=sqrt(d*ONE_);
      if(DEL_^2!=d*ONE_||DEL_^Q_==DEL_,error("delta")))
  );
};

\\ ---------------- curves, twist, phi ----------------
\\ kind 1: odd p, c=[a2,a4,a6] ints; kind 2: binary ordinary y^2+xy=x^3+a x^2+b, c=[a,b];
\\ kind 3: binary supersingular y^2+a3 y=x^3+a4 x+a6, c=[a3,a4,a6].
\\ returns [E, Etwist, phi (E'->E), phiinv (E->E')], all over F_{q^2}.
mkcurves(kind,c)={
  my(E,Et,ph,pinv);
  if(kind==1,
    E =ellinit([0,c[1]*ONE_,0,c[2]*ONE_,c[3]*ONE_]);
    Et=ellinit([0,D_*c[1]*ONE_,0,D_^2*c[2]*ONE_,D_^3*c[3]*ONE_]);
    ph=((P)->if(#P==1,P,[P[1]/D_,P[2]/(D_*DEL_)]));
    pinv=((P)->if(#P==1,P,[D_*P[1],D_*DEL_*P[2]])),
  kind==2,
    E =ellinit([1,c[1],0,0,c[2]]);
    Et=ellinit([1,c[1]+D_,0,0,c[2]]);
    ph=((P)->if(#P==1,P,[P[1],P[2]+DEL_*P[1]]));
    pinv=ph,
    E =ellinit([0,0,c[1],c[2],c[3]]);
    Et=ellinit([0,0,c[1],c[2],c[3]+c[1]^2*D_]);
    ph=((P)->if(#P==1,P,[P[1],P[2]+c[1]*DEL_]));
    pinv=ph);
  if(#E==0||#Et==0,error("singular curve"));
  [E,Et,ph,pinv]
};

\\ ---------------- class enumeration ----------------
\\ odd p>=5: orbit representatives (lexicographically smallest) of (a4,a6) under (u^4 a4,u^6 a6), weight=orbit size;
\\ p=3: all 27 triples with nonzero discriminant, weight 1. Returns vector of [kind, c, weight].
classes_odd(p)={
  my(v=List());
  if(p==3,
    for(a2=0,2,for(a4=0,2,for(a6=0,2, if(#ellinit([0,a2,0,a4,a6]*Mod(1,3))>0, listput(v,[1,[a2,a4,a6],1]))))); return(Vec(v)));
  my(seen=Map(),tot=0);
  for(a4=0,p-1, for(a6=0,p-1,
    if((4*a4^3+27*a6^2)%p==0,next);
    if(mapisdefined(seen,a4*p+a6),next);
    my(orb=Map(),n=0);
    for(u=1,p-1, my(k=((u^4*a4)%p)*p+((u^6*a6)%p)); if(!mapisdefined(orb,k),mapput(orb,k,1);mapput(seen,k,1);n++));
    tot+=n; listput(v,[1,[0,a4,a6],n])));
  if(tot!=p^2-p,error("orbit weights do not sum to p^2-p"));
  Vec(v)
};
\\ all (a4,a6) with nonzero discriminant (p>=5), weight 1 (T3)
curves_odd_all(p)={
  my(v=List());
  for(a4=0,p-1,for(a6=0,p-1, if((4*a4^3+27*a6^2)%p!=0, listput(v,[1,[0,a4,a6],1]))));
  Vec(v)
};
\\ binary ordinary: a in {0,gamma}, b in F_q^*  (setfield must be done)
classes_ord()={
  my(v=List());
  for(ai=1,2, my(a=if(ai==1,0*ONE_,D_)); for(i=2,#SUBF_, listput(v,[2,[a,SUBF_[i]],1])));
  Vec(v)
};
\\ binary supersingular: a3 in F_q^* (all) if allA3, else a3=1; a4,a6 in F_q
classes_ss(allA3)={
  my(v=List());
  for(i=if(allA3,2,2),if(allA3,#SUBF_,2), for(j=1,#SUBF_,for(k=1,#SUBF_, listput(v,[3,[SUBF_[i],SUBF_[j],SUBF_[k]],1]))));
  Vec(v)
};

\\ ---------------- point enumeration ----------------
\\ E(F_q) (and E'(F_q)) by scanning x in F_q and keeping y in F_q; O first.
ptsFq(E)={
  my(v=List([[0]]));
  for(i=1,#SUBF_, my(x=SUBF_[i],ys=ellordinate(E,x));
    for(j=1,#ys, my(y=ys[j]); if(y^Q_==y,listput(v,[x,y]))));
  Vec(v)
};
\\ full E(F_{q^2}) scan: returns [#E(F_{q^2}), #ker(pi-1), #ker(pi+1)]
fullenum(E)={
  my(cnt=1,ep=1,em=1,x=0*ONE_);
  for(k=-1,Q_^2-2,
    x=if(k<0,0*ONE_,if(k==0,ONE_,x*GEN_));
    my(ys=ellordinate(E,x));
    for(j=1,#ys, my(y=ys[j]); cnt++;
      if(x^Q_==x,
        if(y^Q_==y,ep++);
        if(y^Q_==ellneg(E,[x,y])[2],em++))));
  [cnt,ep,em]
};
tors2(E,A)=select(P->(#P==1)||(elladd(E,P,P)==[0]),A);

\\ ---------------- pair loop ----------------
\\ all pairs (A in E(F_q), B in phi(E'(F_q))): explicit sums.
\\ returns [nrat, ker, img, Rcnt(Map R->#decompositions), bseen(0/1 per B index), nsums]
pairstats(E,A,Bphi)={
  my(n=#A,m=#Bphi,nrat=0,ker=0,Rcnt=Map(),sums=vector(n*m),idx=0,bseen=vector(m));
  for(j=1,m, my(B=Bphi[j]);
    for(i=1,n, my(S=elladd(E,A[i],B),k=ptenc(S)); idx++; sums[idx]=k;
      if(#S==1,ker++);
      if(israt(S), nrat++; bseen[j]=1; my(c);
        if(mapisdefined(Rcnt,k,&c),mapput(Rcnt,k,c+1),mapput(Rcnt,k,1)))));
  [nrat,ker,#Set(sums),Rcnt,bseen,idx]
};

\\ ---------------- oracles for g2 = |E(F_q)[2]| ----------------
\\ (ii) odd p: 1+#roots of the cubic in F_p; binary ordinary: 2 (point (0,sqrt b)); binary SS: 1
g2_oracle2(kind,c)=if(kind==1,1+#polrootsmod('x^3+c[1]*'x^2+c[2]*'x+c[3],CHR_),if(kind==2,2,1));
\\ native ellcard / ellgroup for odd p
natcurve(kind,c,tw)={
  my(d=if(tw,D_,1));
  ellinit([0,d*c[1],0,d^2*c[2],d^3*c[3]],CHR_)
};

\\ ---------------- T1 class ----------------
\\ returns row (vector of fields) and sets violation flags
t1class(kind,c,w)={
  my(cv=mkcurves(kind,c),E=cv[1],Et=cv[2],ph=cv[3],pinv=cv[4],flags=List());
  my(A=ptsFq(E),Bp=ptsFq(Et),nE=#A,nEt=#Bp,t=Q_+1-nE);
  my(Bphi=vector(nEt,j,ph(Bp[j])));
  for(j=1,nEt, if(!ellisoncurve(E,Bphi[j]),error("phi image not on E")); if(!isneg(E,Bphi[j]),listput(flags,"NEG")));
  \\ g2 and oracles
  my(T=tors2(E,A),g2=#T,g2b=g2_oracle2(kind,c),par=(nE%2==0),g2d,odd1,rk2);
  my(Aenc=Set(vector(nE,i,ptenc(A[i]))),Benc=Set(vector(nEt,j,ptenc(Bphi[j]))),Tenc=Set(vector(g2,i,ptenc(T[i]))));
  my(Dset=setintersect(Aenc,Benc),nD=#Dset);
  my(cardq2=ellcard(E),oraOK=1);
  if(g2!=g2b,oraOK=0);
  if(kind==1,
    my(nc=ellcard(natcurve(kind,c,0)),nct=ellcard(natcurve(kind,c,1)),G=ellgroup(natcurve(kind,c,0)));
    rk2=sum(i=1,#G,(G[i]%2==0));
    if(nc!=nE||nct!=nEt||2^rk2!=g2,oraOK=0);
    if((g2>1)!=par,oraOK=0),
    if(kind==2,if(!par,oraOK=0),if(par,oraOK=0)));
  if(!oraOK,listput(flags,"ORA"));
  \\ full F_{q^2} scan
  my(fe=fullenum(E),n2=fe[1],eigp=fe[2],eigm=fe[3]);
  if(eigp!=nE||eigm!=nEt,listput(flags,"EIG"));
  if(n2!=nE*nEt||cardq2!=n2,listput(flags,"CARD"));
  \\ pair loop
  my(ps=pairstats(E,A,Bphi),nrat=ps[1],ker=ps[2],img=ps[3],Rcnt=ps[4],bseen=ps[5]);
  my(M=Mat(Rcnt),nkeys=if(#Rcnt==0,0,#M[,1]),cmin=0,cmax=0);
  if(nkeys>0,cmin=vecmin(M[,2]);cmax=vecmax(M[,2]));
  \\ S3: every R has exactly g2 decompositions, and the keys of R are exactly E(F_q)
  my(s3ok=(nkeys==nE&&cmin==g2&&cmax==g2&&Set(M[,1]~)==Aenc));
  if(!s3ok,listput(flags,"S3"));
  \\ NC3 / pair-count prediction #E*g2
  if(nrat!=nE*g2,listput(flags,"NC3"));
  \\ S1: B-set equals E(F_q)[2] (as subset of E(F_{q^2})) and 2B=O for all B seen
  my(Bsen=List(),nb2=0,nodd=0);
  for(j=1,nEt, if(bseen[j], listput(Bsen,ptenc(Bphi[j])); if(!(#Bphi[j]==1||elladd(E,Bphi[j],Bphi[j])==[0]),nb2++)));
  if(Set(Vec(Bsen))!=Tenc||nb2>0,listput(flags,"S1"));
  \\ S5: no B of odd order > 1 occurs
  for(j=2,nEt, if(bseen[j], my(o=ellorder(E,Bphi[j],n2)); if(o%2==1&&o>1,nodd++)));
  if(nodd>0,listput(flags,"S5"));
  \\ S2: |D|=g2, D=E[2]; kernel size g2; image size nE*nEt/g2; bijective iff g2==1
  my(bij=(ker==1&&img==n2));
  if(nD!=g2||Dset!=Tenc||ker!=g2||img*g2!=nE*nEt||(bij!=(g2==1)),listput(flags,"S2"));
  \\ S9: |phi^{-1}(E(F_q)) cap E'(F_q)| = g2
  my(s9=0); for(i=1,nE, my(Pm=pinv(A[i])); if(!ellisoncurve(Et,Pm),error("phiinv image not on E'")); if(israt(Pm),s9++));
  if(s9!=g2,listput(flags,"S9"));
  \\ literal lemma (b): every decomposition has B=O
  my(litb=(sum(j=2,nEt,bseen[j])==0));
  my(cop=(gcd(oddpart(nE),oddpart(nEt))==1));
  if(litb!=(g2==1),listput(flags,"M2"));
  \\ witness (first coprime & g2>1 class per q)
  if(cop&&g2>1&&WIT_<3,
    my(jb=0); for(j=2,nEt,if(bseen[j],jb=j;break));
    my(R=if(nE>1,A[2],A[1]),B=Bphi[jb],Aa=elladd(E,R,ellneg(E,B)));
    if(!(israt(Aa)&&elladd(E,Aa,B)==R),error("witness check failed"));
    WIT_++;
    emit(["WITNESS",Q_,kind,cstr(c),Str("t=",t),Str("nE=",nE),Str("nEt=",nEt),Str("g2=",g2),Str("R=",ptstr(R)),Str("A=R-B=",ptstr(Aa)),Str("B=",ptstr(B)),Str("B!=O:",(#B>1)),Str("A!=R:",(Aa!=R))]));
  my(fl=if(#flags==0,"OK",strjoin(Vec(flags),",")));
  [ "T1",Q_,kind,cstr(c),w,t,nE,nEt,g2,g2b,cop,n2,eigp,eigm,ker,img,nrat,cmin,cmax,nD,s9,litb,bij,fl ]
};

\\ ---------------- T2 class (B-only linear check) ----------------
t2class(kind,c,w)={
  my(cv=mkcurves(kind,c),E=cv[1],Et=cv[2],ph=cv[3],pinv=cv[4],flags=List());
  my(A=ptsFq(E),Bp=ptsFq(Et),nE=#A,nEt=#Bp,t=Q_+1-nE,q=Q_);
  my(g2=#tors2(E,A),g2b=g2_oracle2(kind,c),par=(nE%2==0),oraOK=1);
  if(g2!=g2b,oraOK=0);
  if(kind==1,
    my(nc=ellcard(natcurve(kind,c,0)),nct=ellcard(natcurve(kind,c,1)));
    if(nc!=nE||nct!=nEt,oraOK=0);
    if((g2>1)!=par,oraOK=0),
    if(kind==2,if(!par,oraOK=0),if(par,oraOK=0)));
  \\ (iii) Hasse/sanity: nE*nEt=(q+1)^2-t^2 and ellcard over F_{q^2}
  if(nE*nEt!=(q+1)^2-t^2||abs(t)>2*sqrt(q)+1e-9,oraOK=0);
  if(q<=256&&ellcard(E)!=nE*nEt,oraOK=0);
  if(!oraOK,listput(flags,"ORA"));
  my(Dlist=List(),lin=0,negv=0);
  my(Aenc=Set(vector(nE,i,ptenc(A[i]))),Benc=List());
  for(j=1,nEt, my(P=ph(Bp[j]));
    if(!ellisoncurve(E,P),error("phi image not on E"));
    listput(Benc,ptenc(P));
    my(r=israt(P),tw=(#P==1||elladd(E,P,P)==[0]));
    if(!isneg(E,P),negv++);
    if(r!=tw,lin++);
    if(r,listput(Dlist,ptenc(P))));
  if(negv>0,listput(flags,"NEG"));
  if(lin>0,listput(flags,"LIN"));
  my(T=tors2(E,A),Tenc=Set(vector(g2,i,ptenc(T[i]))),Dset=setintersect(Aenc,Set(Vec(Benc))),nD=#Dset);
  if(nD!=g2||Dset!=Tenc||Set(Vec(Dlist))!=Dset,listput(flags,"S2"));
  \\ S5: odd-order B never lies in D
  my(nodd=0); for(j=2,nEt, my(P=ph(Bp[j])); if(israt(P), my(o=ellorder(E,P,(q+1)^2-t^2)); if(o%2==1&&o>1,nodd++)));
  if(nodd>0,listput(flags,"S5"));
  my(s9=0); for(i=1,nE, my(Pm=pinv(A[i])); if(!ellisoncurve(Et,Pm),error("phiinv image not on E'")); if(israt(Pm),s9++));
  if(s9!=g2,listput(flags,"S9"));
  my(gg=gcd(nE,nEt),cop=(gcd(oddpart(nE),oddpart(nEt))==1));
  my(gok=(( 2*gcd(q+1,t) )%gg==0));
  my(pm=if(gg==1,1,vecmax(factor(gg)[,1])));
  my(pok=(t==0||pm<=4*sqrt(q)+1e-9));
  if(!gok,listput(flags,"GCDDIV")); if(!pok,listput(flags,"GCDPRIME"));
  my(fl=if(#flags==0,"OK",strjoin(Vec(flags),",")));
  [ "T2",q,kind,cstr(c),w,t,nE,nEt,g2,g2b,cop,gg,nD,s9,(nD>1),fl ]
};

\\ ---------------- T4 helpers ----------------
\\ group structure of the point set A on curve E (rank<=2): [exponent, n/exponent] (1s dropped)
grpstruct(E,A,N)={
  my(n=#A,e=1);
  for(i=1,n, e=lcm(e,ellorder(E,A[i],N)));
  if(n/e==1,if(e==1,[],[e]),[e,n/e])
};
snfcat(v1,v2)={
  my(v=concat(v1,v2));
  if(#v==0,return([]));
  my(s=matsnf(matdiagonal(v),4));
  Vec(s)
};
t4class(kind,c,w)={
  my(cv=mkcurves(kind,c),E=cv[1],Et=cv[2],ph=cv[3]);
  my(A=ptsFq(E),Bp=ptsFq(Et),nE=#A,nEt=#Bp,t=Q_+1-nE,N=nE*nEt);
  my(g2=#tors2(E,A));
  my(gE=grpstruct(E,A,N),gEt=grpstruct(Et,Bp,N));
  my(gq2=ellgroup(E),gq2s=vecsort(select(x->x>1,Vec(gq2)),,4),prod=snfcat(gE,gEt),prods=vecsort(select(x->x>1,prod),,4));
  my(natE="NA");
  if(kind==1,
    my(G=ellgroup(natcurve(kind,c,0)),Gt=ellgroup(natcurve(kind,c,1)));
    natE=if(vecsort(select(x->x>1,Vec(G)),,4)==gE&&vecsort(select(x->x>1,Vec(Gt)),,4)==gEt,"match","MISMATCH"));
  [ "T4",Q_,kind,cstr(c),w,t,nE,nEt,g2,gcd(oddpart(nE),oddpart(nEt))==1,Str(gE),Str(gEt),Str(gq2s),Str(prods),(gq2s==prods),natE ]
};

\\ ---------------- T5 helpers (odd p>=5, short Weierstrass) ----------------
\\ group Hom count: product over invariant factors n_i of G1, m_j of G2 of gcd(n_i,m_j)
homformula(G1,G2)={my(r=1); for(i=1,#G1,for(j=1,#G2,r*=gcd(G1[i],G2[j]))); r};
\\ brute-force: |Hom(G1,G2)| = prod_i #{h in G2 : n_i h = O}; points of G2 enumerated natively on curve Ec/F_p
homcount(G1,Ec,p)={
  my(pts=List([[0]]),r=1);
  for(x=0,p-1, my(ys=ellordinate(Ec,Mod(x,p))); for(j=1,#ys, listput(pts,[Mod(x,p),ys[j]])));
  my(ords=vector(#pts,i,if(#pts[i]==1,1,ellorder(Ec,pts[i]))));
  for(i=1,#G1, r*=sum(k=1,#ords,(G1[i]%ords[k]==0)));
  r
};
\\ canonical F_p-isomorphism class of y^2=x^3+a4x+a6 : lexicographic min over u
canon(a4,a6,p)={
  my(best=[p,p]);
  for(u=1,(p-1)/2+1, my(k=[(u^4*a4)%p,(u^6*a6)%p]); if(lex(k,best)<0,best=k));
  best
};
shortof(E2,p)=canon(lift(-27*E2.c4),lift(-54*E2.c6),p);
subsets(fs,k,start,deg,prod)={
  if(deg==k,return([prod]));
  my(r=[]);
  for(i=start,#fs, my(d=poldegree(fs[i])); if(deg+d<=k, r=concat(r,subsets(fs,k,i+1,deg+d,prod*fs[i]))));
  r
};
\\ F_p-rational l-isogenies from y^2=x^3+a4x+a6 (p>=5, l!=p, l in {2,3,5,7}): returns vector of [canon target, card, j]
\\ candidates: kernel polynomials = monic divisors of degree (l-1)/2 of the l-division polynomial (products of its irreducible factors);
\\ validated by ellisogeny + equal ellcard + image of a random point lies on target; j checked against Phi_l roots.
lisos(a4,a6,p,l)={
  my(E=ellinit([a4,a6],p),k=if(l==2,1,(l-1)/2),dv=elldivpol(E,l),fa=factor(dv),fs=List(),res=List(),jE=E.j,bad=0);
  for(i=1,#fa~, listput(fs,fa[i,1]));
  fs=Vec(fs);
  my(cands=subsets(fs,k,1,0,Mod(1,p)*1));
  my(Phi=polmodular(l),fy=lift(subst(subst(Phi,'x,jE),'y,'x)),rts=Set(apply(r->lift(r),Vec(polrootsmod(fy,p)))));
  my(pts=vector(5,i,random(E)));
  for(i=1,#cands,
    my(G=cands[i],iso=iferr(ellisogeny(E,G),e,"ERR"));
    if(iso==="ERR",bad++;next);
    my(E2=ellinit(iso[1]),ok=1); if(#E2==0,bad++;next);
    for(u=1,#pts, my(Q=iferr(ellisogenyapply(iso[2],pts[u]),e,"ERR")); if(Q==="ERR"||!ellisoncurve(E2,Q),ok=0;break));
    if(!ok,bad++;next);
    my(j2=lift(E2.j));
    if(!setsearch(rts,j2),bad++;next);
    listput(res,[shortof(E2,p),ellcard(E2),j2]));
  [Vec(res),bad]
};
\\ BFS over the F_p-isogeny graph from the class of (a4,a6); ls = primes used, depth = max depth.
\\ returns [visited keys list (canonical), n edges examined, #bad candidates, #card violations]
bfs(a4,a6,p,ls,depth)={
  my(start=canon(a4,a6,p),vis=Map(),frontier=[start],nbad=0,ncardbad=0,card0=ellcard(ellinit([a4,a6],p)),ned=0);
  mapput(vis,start,0);
  for(dd=1,depth,
    my(nf=List());
    for(f=1,#frontier, my(cur=frontier[f]);
      for(li=1,#ls, my(l=ls[li]); if(l==p,next);
        my(r=lisos(cur[1],cur[2],p,l)); nbad+=r[2];
        for(k=1,#r[1], ned++; my(tg=r[1][k]); if(r[1][k][2]!=card0,ncardbad++);
          if(!mapisdefined(vis,tg[1]),mapput(vis,tg[1],dd);listput(nf,tg[1])))));
    frontier=Vec(nf));
  [Vec(Mat(vis)[,1]~),ned,nbad,ncardbad]
};
