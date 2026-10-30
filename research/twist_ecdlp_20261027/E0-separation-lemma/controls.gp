\\ controls.gp -- positive controls PC1-PC3 first, then negative controls NC1-NC3 (instrument validity).
FAILS=0; NROWS=0;
ctl(tag,ok,v)={ NROWS++; if(!ok,FAILS++); emit(concat([tag],concat(v,[if(ok,"PASS","FAIL")]))) };

\\ ---- PC1: trivial twist (E'=E, phi=identity): detector must report #E decompositions per R, |D|=#E
pc1run(cl)={
  for(i=1,#cl,
    my(kind=cl[i][1],c=cl[i][2],cv=mkcurves(kind,c),E=cv[1],Et=cv[2],ph=cv[3]);
    my(A=ptsFq(E),Bp=ptsFq(Et),nE=#A,Bphi=vector(#Bp,j,ph(Bp[j])));
    my(ps=pairstats(E,A,Bphi),M=Mat(ps[4]),cmin=vecmin(M[,2]),cmax=vecmax(M[,2]));
    my(Dset=setintersect(Set(vector(nE,k,ptenc(A[k]))),Set(vector(#Bphi,k,ptenc(Bphi[k])))));
    my(ok=(#Bp==nE&&ps[1]==nE^2&&cmin==nE&&cmax==nE&&#Dset==nE&&ps[2]==nE&&ps[3]==nE&&#M[,1]==nE));
    ctl("PC1",ok,[Q_,kind,cstr(c),nE,ps[1],cmin,cmax,#Dset,ps[2],ps[3]]));
};
\\ ---- PC2: planted witness (A,B)=(R-T,T), T an independently found rational 2-torsion point
pc2run(cl)={
  for(i=1,#cl,
    my(kind=cl[i][1],c=cl[i][2]);
    if(g2_oracle2(kind,c)<=1,next);
    my(cv=mkcurves(kind,c),E=cv[1],Et=cv[2],ph=cv[3],pinv=cv[4],A=ptsFq(E),T);
    if(kind==1,
      my(rt=polrootsmod('x^3+c[1]*'x^2+c[2]*'x+c[3],CHR_)); T=[lift(rt[1])*ONE_,0*ONE_],
      T=[0*ONE_,c[2]^(Q_/2)]);
    my(ok=ellisoncurve(E,T),np=0,Bq=pinv(T));
    if(!(ellisoncurve(Et,Bq)&&israt(Bq)),ok=0);
    my(B=ph(Bq));
    if(B!=T||#B==1||!isneg(E,B),ok=0);
    for(r=1,#A, my(R=A[r],Aa=elladd(E,R,ellneg(E,T)));
      if(israt(Aa)&&elladd(E,Aa,B)==R&&Aa!=R&&ellisoncurve(E,Aa),np++));
    if(np!=#A,ok=0);
    ctl("PC2",ok,[Q_,kind,cstr(c),#A,np]));
};
\\ ---- PC3 / NC2: group-level Hom detector and isogeny BFS (odd p>=5)
pc3run(p)={
  my(cl=classes_odd(p),nc=#cl,nE=vector(nc),Gs=vector(nc));
  for(i=1,nc, my(c=cl[i][2],En=natcurve(1,c,0)); nE[i]=ellcard(En); Gs[i]=ellgroup(En));
  for(i=1,nc,
    my(c=cl[i][2],En=natcurve(1,c,0),Et=natcurve(1,c,1),Gt=ellgroup(Et),t=p+1-nE[i]);
    my(hff=homformula(Gs[i],Gs[i]),hfb=homcount(Gs[i],En,p));
    my(b=bfs(c[2],c[3],p,[2,3],1),key=canon(c[2],c[3],p),reach=0);
    for(k=1,#b[1],if(b[1][k]==key,reach=1));
    my(keyEt=canon((D_^2*c[2])%p,(D_^3*c[3])%p,p),twiso=(keyEt==key));
    my(ok=(hff==hfb&&reach==1&&(nE[i]==1||hff>1)));
    if(twiso, my(hft=homformula(Gs[i],Gt)); if(!(nE[i]==1||hft>1),ok=0));
    ctl("PC3",ok,[p,Str(c[2],",",c[3]),cl[i][3],t,nE[i],hff,hfb,reach,twiso]));
  setrand(20261021+p);
  for(i=1,nc,
    my(c=cl[i][2],En=natcurve(1,c,0),cand=List());
    for(j=1,nc, if(gcd(nE[i],nE[j])==1,listput(cand,j)));
    if(#cand==0, emit(["NC2",p,Str(c[2],",",c[3]),"no_coprime_partner","NA"]); next);
    my(j=cand[1+random(#cand)],c2=cl[j][2],E2=natcurve(1,c2,0));
    my(hf=homformula(Gs[i],Gs[j]),hb=homcount(Gs[i],E2,p));
    my(b=bfs(c[2],c[3],p,[2,3,5,7],1),key2=canon(c2[2],c2[3],p),reach=0);
    for(k=1,#b[1],if(b[1][k]==key2,reach=1));
    ctl("NC2",(hf==1&&hb==1&&reach==0),[p,Str(c[2],",",c[3]),Str(c2[2],",",c2[3]),nE[i],nE[j],gcd(nE[i],nE[j]),hf,hb,reach]));
};
\\ ---- NC1: classes where the lemma is predicted TRUE (#E odd odd p; binary supersingular): no nonzero B with rational sum
nc1run(cl)={
  for(i=1,#cl,
    my(kind=cl[i][1],c=cl[i][2],cv=mkcurves(kind,c),E=cv[1],Et=cv[2],ph=cv[3]);
    my(A=ptsFq(E),Bp=ptsFq(Et),nE=#A);
    if(kind==1&&nE%2==0,next);
    my(Bphi=vector(#Bp,j,ph(Bp[j])),ps=pairstats(E,A,Bphi),nb=sum(j=2,#Bp,ps[5][j]));
    ctl("NC1",(nb==0&&ps[1]==nE),[Q_,kind,cstr(c),nE,#Bp,nb]));
};
\\ ---- NC3: pair-count prediction #E*g2 equals explicit pair count
nc3run(cl)={
  for(i=1,#cl,
    my(kind=cl[i][1],c=cl[i][2],cv=mkcurves(kind,c),E=cv[1],Et=cv[2],ph=cv[3]);
    my(A=ptsFq(E),Bp=ptsFq(Et),nE=#A,g2=#tors2(E,A),Bphi=vector(#Bp,j,ph(Bp[j])),ps=pairstats(E,A,Bphi));
    ctl("NC3",(ps[1]==nE*g2),[Q_,kind,cstr(c),nE,g2,ps[1],nE*g2]));
};

\\ ================= run =================
{
TRIV_=1;
forprime(p=3,31, setfield(p,1); pc1run(classes_odd(p)));
for(m=2,4, setfield(2,m); pc1run(classes_ord()); pc1run(classes_ss(0)));
emit(["PC1_SUMMARY",Str("rows=",NROWS),Str("fails=",FAILS)]);
TRIV_=0;
if(FAILS>0, emit(["ABORT","PC1 failed: detector blind or buggy"]); quit(1));
my(f0=FAILS,r0=NROWS);
forprime(p=3,31, setfield(p,1); pc2run(classes_odd(p)));
for(m=2,5, setfield(2,m); pc2run(classes_ord()));
emit(["PC2_SUMMARY",Str("rows=",NROWS-r0),Str("fails=",FAILS-f0)]);
f0=FAILS; r0=NROWS;
forprime(p=5,31, setfield(p,1); pc3run(p));
emit(["PC3_NC2_SUMMARY",Str("rows=",NROWS-r0),Str("fails=",FAILS-f0)]);
if(FAILS>0, emit(["ABORT","PC2/PC3/NC2 failed"]); quit(1));
f0=FAILS; r0=NROWS;
forprime(p=3,31, setfield(p,1); nc1run(classes_odd(p)));
for(m=2,4, setfield(2,m); nc1run(classes_ss(0)));
emit(["NC1_SUMMARY",Str("rows=",NROWS-r0),Str("fails=",FAILS-f0)]);
f0=FAILS; r0=NROWS;
forprime(p=3,31, setfield(p,1); nc3run(classes_odd(p)));
for(m=2,4, setfield(2,m); nc3run(classes_ord()); nc3run(classes_ss(0)));
emit(["NC3_SUMMARY",Str("rows=",NROWS-r0),Str("fails=",FAILS-f0)]);
emit(["CONTROLS_SUMMARY",Str("rows=",NROWS),Str("fails=",FAILS),if(FAILS==0,"CONTROLS_PASS","CONTROLS_FAIL")]);
}
print("DONE|controls");
