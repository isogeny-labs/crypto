\\ lib.gp -- implementation of PREREGISTRATION.md (L1-descent-parity), PARI/GP 2.15
\\ Operation counters are charged at call sites; NAT formulas are the frozen ones of section 5.

cl2(x)=if(x<=1,0,#binary(x-1));
smul(k)=if(k<=1,0,#binary(k)-1+hammingweight(k)-1);
OUTDIR=getenv("OUTDIR");
RUNTAG=getenv("RUNTAG");

\\ ------------------------------------------------------------ counters
cnt_reset()={GAE=0;NAT=0;LANAT=0;NSOLV=0;NRES=0;NGCD=0;NROOT=0;SROOT=0;NSQ=0;LAROW=0;LAENT=0;ANOM=0;NREL=0;NDEGC=0;}
cnt_reset();

\\ ------------------------------------------------------------ field and curve
fq_def(p,kdeg,fpar)=if(kdeg==2,Mod(1,p)*(w^2-fpar),Mod(1,p)*(w^3-w-fpar));
fq_par(p,kdeg)={my(c);if(kdeg==2,c=2;while(kronecker(c,p)!=-1,c++),c=1;while(!polisirreducible(Mod(1,p)*(w^3-w-c)),c++));c}
fq_setup(p,kdeg,fpar)={PP=p;KDEG=kdeg;FPAR=fpar;GT=ffgen(fq_def(p,kdeg,fpar),'tt);HP=(p-1)\2;LGP=cl2(p);LGQ=cl2(p^kdeg);}
xkey(x)=subst(x.pol,'tt,PP);
ycan(y)={my(pl=y.pol);polcoef(pl,poldegree(pl,'tt),'tt)<=HP}
cpt(P)=if(ycan(P[2]),P,ellneg(EC,P));
infp(x)=(poldegree(x.pol,'tt)<=0);
\\ frozen cost formulas
sqrtcost()=ceil(3*LGQ/2)*KDEG^2;
legcost()=ceil(3*LGP/2)+2;
rootcost(d)=3*LGP*d^2;
RESCOST=104;   \\ explicit Sylvester form of two quadratics with quadratic coefficients: 6 products x 9 + 25 + 25
EVALCOST=12;   \\ evaluation of the 6 quadratics at a root (2 mults each)
GCDCOST=4;     \\ GCD(2,2)
CALLC=299;     \\ C-fix per solver call: expansion 27 + 2 resultants 208 + gcd 64

\\ ------------------------------------------------------------ instance
\\ rec = [arm,s,idx,p,kdeg,fpar,a0,a1,a2,b0,b1,b2,N,N1,N2,lstar,h,k,gseed,cstar,ME,MW]
inst_setup(rec)={
  my(fN,ok);
  fq_setup(rec[4],rec[5],rec[6]);
  AA=rec[7]*GT^0+rec[8]*GT+rec[9]*GT^2; BB=rec[10]*GT^0+rec[11]*GT+rec[12]*GT^2;
  AB_MODE=(rec[1]==1);AA_INT=rec[7];BB_INT=rec[10];
  EC=ellinit([AA,BB]);
  NN=rec[13];N1=rec[14];N2=rec[15];LS=rec[16];HH=rec[17];KK=rec[18];CSTAR=rec[20];
  ARMC=rec[1];
  setrand(rec[19]);
  fN=factor(NN)[,1]; 
  while(1,GG=random(EC);ok=(ellmul(EC,GG,NN)==[0]);for(i=1,#fN,if(ellmul(EC,GG,NN/fN[i])==[0],ok=0));if(ok,break));
  QQ=ellmul(EC,GG,KK);
  CC=NN/LS;
  GS=ellmul(EC,GG,CC);QS=ellmul(EC,QQ,CC);
  AA2=AA^2;
  WW=ceil(#binary(NN)/8);
  PRECOMP=2*WW*262;
}

\\ tables: CLS (class of f(x) in F_p, arms A/B), YPT (canonical point with abscissa x or 0)
build_tables()={
  my(fv,y,fi);
  CLS=vector(PP);YPT=vector(PP);
  for(x=0,PP-1,
    fv=x^3+AA*x+BB;
    if(fv!=0 && issquare(fv,&y), YPT[x+1]=cpt([GT^0*x,y]));
    if(AB_MODE, fi=(x^3+AA_INT*x+BB_INT)%PP; CLS[x+1]=kronecker(fi,PP));
  );
}

\\ comb tables for target generation (precomputation counted in PRECOMP)
comb_build()={
  my(base);
  TG=vector(WW);TQ=vector(WW);
  base=GG;
  for(j=1,WW, TG[j]=vector(256);TG[j][1]=[0];TG[j][2]=base;for(d=2,255,TG[j][d+1]=elladd(EC,TG[j][d],base)); base=ellmul(EC,base,256));
  base=QQ;
  for(j=1,WW, TQ[j]=vector(256);TQ[j][1]=[0];TQ[j][2]=base;for(d=2,255,TQ[j][d+1]=elladd(EC,TQ[j][d],base)); base=ellmul(EC,base,256));
}
gen_target()={
  my(a=random(NN),b=random(NN),R=[0]);
  for(j=1,WW, R=elladd(EC,R,TG[j][bitand(shift(a,-8*(j-1)),255)+1]); R=elladd(EC,R,TQ[j][bitand(shift(b,-8*(j-1)),255)+1]));
  GAE+=2*WW-1;
  [R,a,b];
}

\\ ------------------------------------------------------------ Weil-restricted S3 solver for xR in F_{p^2}, x1,x2 in F_p
\\ returns [] of unordered pairs, or -1 if x(R) in F_p (degenerate, no solver call)
solve_pairs(X)={
  my(X2,aX,bX,c11,c10,c00,M2,M1,M0,ar,bi,ae,res,d,rts,pl,r,Ur,Vr,g,out,pr,y,cs,t0,t1);
  if(infp(X),NAT+=1;return(-1));
  NSOLV++;
  X2=X^2;aX=AA*X;bX=BB*X;
  c11=-2*X2-2*AA;c10=-2*aX-4*BB;c00=AA2-4*bX;
  NAT+=KDEG^2+2*(if(ARMC==1,KDEG,KDEG^2));  \\ expansion: X^2, aX, bX
  M2=[GT^0,-2*X,X2];M1=[-2*X,c11,c10];M0=[X2,c10,c00];
  \\ real and imaginary parts in the basis 1,tt
  ar=vector(3);bi=vector(3);
  my(MM=[M2,M1,M0]);
  for(i=1,3, ar[i]=Pol(vector(3,j,polcoef(MM[i][j].pol,0,'tt)),'xx)*Mod(1,PP); bi[i]=Pol(vector(3,j,polcoef(MM[i][j].pol,1,'tt)),'xx)*Mod(1,PP));
  \\ ar[1],ar[2],ar[3]: coefficients of x2^2, x2^1, x2^0 (each a polynomial in xx=x1)
  res=(ar[1]*bi[3]-ar[3]*bi[1])^2-(ar[1]*bi[2]-ar[2]*bi[1])*(ar[2]*bi[3]-ar[3]*bi[2]);
  NRES++;NAT+=RESCOST;
  out=Set();
  if(res==0,ANOM++;return(out));
  d=poldegree(res);
  if(d<1,return(out));
  NROOT++;SROOT+=d;NAT+=rootcost(d);
  rts=polrootsmod(res,PP);
  for(i=1,#rts,
    r=rts[i];
    NAT+=EVALCOST;NGCD++;NAT+=GCDCOST;
    Ur=Pol([subst(ar[1],'xx,r),subst(ar[2],'xx,r),subst(ar[3],'xx,r)],'yy);
    Vr=Pol([subst(bi[1],'xx,r),subst(bi[2],'xx,r),subst(bi[3],'xx,r)],'yy);
    g=gcd(Ur,Vr);
    if(g==0,ANOM++;next);
    pl=poldegree(g,'yy);
    if(pl==1,
      y=-polcoef(g,0,'yy)/polcoef(g,1,'yy);
      out=setunion(out,Set([vecsort([lift(r),lift(y)])])),
      if(pl==2,
        NAT+=rootcost(2);
        cs=polrootsmod(g,PP);
        for(j=1,#cs,out=setunion(out,Set([vecsort([lift(r),lift(cs[j])])])))));
  );
  out;
}

\\ ------------------------------------------------------------ sign test (2 GAE)
sign2(R,P1,P2)={
  my(S);
  GAE+=2;
  S=elladd(EC,P1,P2);
  if(#S>1 && S[1]==R[1], return(if(S[2]==R[2],[1,1],[-1,-1])));
  S=elladd(EC,P1,ellneg(EC,P2));
  if(#S>1 && S[1]==R[1], return(if(S[2]==R[2],[1,-1],[-1,1])));
  0;
}

\\ ------------------------------------------------------------ linear algebra (incremental structured elimination mod LS)
la_reset()={PIV=Map();}
\\ row: sum coefs[i]*u_cols[i] + beta*q = alpha (mod LS); returns qhat (>=0) when solved, else -1
la_add(cols,coefs,beta,alpha)={
  my(row=Map(),cur,K,found,pr,f,Vp,nv,inv,kk,L=LS,pv);
  for(i=1,#cols,
    if(mapisdefined(row,cols[i],&cur),nv=(cur+coefs[i])%L,nv=coefs[i]%L);
    if(nv==0,mapdelete(row,cols[i]),mapput(row,cols[i],nv)));
  beta%=L;alpha%=L;
  while(1,
    K=Vec(row);found=0;
    for(i=1,#K, if(mapisdefined(PIV,K[i],&pr), found=K[i];break));
    if(!found,break);
    f=mapget(row,found);
    Vp=Vec(pr[1]);
    for(i=1,#Vp,
      kk=Vp[i];pv=mapget(pr[1],kk);
      if(mapisdefined(row,kk,&cur),nv=(cur-f*pv)%L,nv=(-f*pv)%L);
      LAENT++;LANAT++;NAT++;
      if(nv==0,mapdelete(row,kk),mapput(row,kk,nv)));
    beta=(beta-f*pr[2])%L;alpha=(alpha-f*pr[3])%L;LANAT+=2;NAT+=2;LAROW++;
  );
  K=Vec(row);
  if(#K==0,
    if(beta!=0 && gcd(beta,L)==1, LANAT+=10;NAT+=10;return(lift(Mod(alpha,L)/Mod(beta,L))));
    return(-1));
  kk=K[1];
  inv=lift(Mod(mapget(row,kk),L)^-1);LANAT+=10;NAT+=10;
  Vp=Vec(row);
  for(i=1,#Vp, mapput(row,Vp[i],(mapget(row,Vp[i])*inv)%L);LAENT++;LANAT++;NAT++);
  beta=(beta*inv)%L;alpha=(alpha*inv)%L;LANAT+=2;NAT+=2;LAROW++;
  mapput(PIV,kk,[row,beta,alpha]);
  -1;
}

\\ ------------------------------------------------------------ one IC solve (arms A,B,D); FB is the 0/1 factor-base vector
\\ MODE: 1 = A/B (projection to the eigenspace of l*), 2 = D
ic_run_pairs(mode,cap)={
  my(att=0,ndeg=0,tg,R,P,pr,x1,x2,P1,P2,sg,q=-1,cols,coefs,ncols=0,rel_at=0,seen=Map(),t0=getabstime(),X,c1,c2,col,e,pairs,usedcols=Map());
  cnt_reset();la_reset();GAE=PRECOMP;
  while(att<cap && q<0,
    tg=gen_target();R=tg[1];
    if(R==[0],ndeg++;next);
    pairs=solve_pairs(R[1]);
    if(pairs==-1,ndeg++;next);
    att++;
    for(i=1,#pairs,
      x1=pairs[i][1];x2=pairs[i][2];
      NAT+=2;
      if(!(FB[x1+1] && FB[x2+1]),next);
      NAT+=2*sqrtcost();NSQ+=2;
      P1=YPT[x1+1];P2=YPT[x2+1];
      sg=sign2(R,P1,P2);
      if(sg==0,ANOM++;next);
      if(mode==1,
        NAT+=2*legcost();
        c1=CLS[x1+1];c2=CLS[x2+1];
        if(c1==CSTAR && c2!=CSTAR, col=x1;e=sg[1],
          if(c2==CSTAR && c1!=CSTAR, col=x2;e=sg[2], ANOM++;next));
        cols=[col];coefs=[e],
        cols=[x1,x2];coefs=sg);
      NREL++;
      for(j=1,#cols,mapput(usedcols,cols[j],1));
      if(q<0, q=la_add(cols,coefs,-tg[3],tg[2]); if(q>=0,rel_at=NREL));
    );
  );
  [if(q>=0 && ellmul(EC,GS,q)==QS,1,0),att,ndeg,rel_at,#Vec(usedcols),NSOLV,NRES,NGCD,NROOT,SROOT,NSQ,GAE,NAT,LANAT,LAROW,LAENT,ANOM,getabstime()-t0,NREL];
}

\\ ------------------------------------------------------------ census: D(R) distribution and valid-relation rates (no relation processing)
\\ FBS is a vector of factor-base vectors; returns [ntargets,ndeg,hist (D(R)=0..4+),valid counts per FB]
census_inst(n,FBS)={
  my(tg,pairs,hist=vector(6),valid=vector(#FBS),nt=0,nd=0,okp,x1,x2,tmp);
  for(t=1,n,
    tg=gen_target();
    if(tg[1]==[0],nd++;next);
    pairs=solve_pairs(tg[1][1]);
    if(pairs==-1,nd++;next);
    nt++;
    hist[min(#pairs,5)+1]++;
    for(f=1,#FBS,
      okp=0;
      for(i=1,#pairs,x1=pairs[i][1];x2=pairs[i][2];if(FBS[f][x1+1]&&FBS[f][x2+1],okp++));
      valid[f]+=okp);
  );
  [nt,nd,hist,valid];
}

\\ ------------------------------------------------------------ arm C: C-fix with exact table oracle (see DEVIATIONS.md)
c_build()={
  my(S,key,v,Pi,Pj,sig);
  VL=List();for(x=0,PP-1,if(YPT[x+1]!=0,listput(VL,x)));VL=Vec(VL);MV=#VL;
  CT=Map();
  for(i=1,MV,Pi=YPT[VL[i]+1];
    for(j=i,MV,Pj=YPT[VL[j]+1];
      for(sg=0,if(i==j,0,1),
        S=if(sg==0,elladd(EC,Pi,Pj),elladd(EC,Pi,ellneg(EC,Pj)));
        if(#S>1 && !infp(S[1]),
          key=xkey(S[1]);
          if(mapisdefined(CT,key,&v),v=concat(v,[[i,j,1-2*sg,S[2]]]),v=[[i,j,1-2*sg,S[2]]]);
          mapput(CT,key,v)))));
  NEGV=vector(MV,i,ellneg(EC,YPT[VL[i]+1]));
}
\\ one attempt: returns list of relations [cols,coefs]
c_attempt(R)={
  my(rels=List(),seen=Map(),T,key,v,P1,e1,E,ent,i2,j2,sT,cols,coefs,tk,xs,roots,ysol,dd);
  for(l=1,MV,
    P1=YPT[VL[l]+1];
    for(sgn=0,1,
      T=if(sgn==0,elladd(EC,R,NEGV[l]),elladd(EC,R,P1));  \\ sgn=0: R-P1 (e1=+1); sgn=1: R+P1 (e1=-1)
      e1=1-2*sgn;
      GAE+=1;NSOLV++;NAT+=CALLC;NRES+=2;NGCD++;
      if(#T==1,next);
      if(infp(T[1]),NDEGC++;next);
      key=xkey(T[1]);
      if(mapisdefined(CT,key,&v),
        \\ solver found roots: charge root-finding on the gcd of degree = number of distinct abscissas, and y recovery for each solution
        xs=Set();
        for(z=1,#v,xs=setunion(xs,Set([v[z][1],v[z][2]])));
        dd=#xs;NROOT++;SROOT+=dd;NAT+=rootcost(dd);
        for(z=1,#v,
          ent=v[z];NAT+=2*sqrtcost();NSQ+=2;
          sT=if(T[2]==ent[4],1,-1);
          cols=[VL[l],VL[ent[1]],VL[ent[2]]];coefs=[e1,sT,sT*ent[3]];
          tk=vecsort(cols);
          if(!mapisdefined(seen,tk),mapput(seen,tk,1);GAE+=2;listput(rels,[cols,coefs])))));
  );
  Vec(rels);
}
ic_run_c(cap)={
  my(att=0,ndeg=0,tg,R,rels,q=-1,rel_at=0,t0=getabstime(),usedcols=Map());
  cnt_reset();la_reset();GAE=PRECOMP;
  while(att<cap && q<0,
    tg=gen_target();R=tg[1];
    if(R==[0] || infp(R[1]),ndeg++;next);
    att++;
    rels=c_attempt(R);
    for(i=1,#rels,
      NREL++;
      for(j=1,#rels[i][1],mapput(usedcols,rels[i][1][j],1));
      if(q<0, q=la_add(rels[i][1],rels[i][2],-tg[3],tg[2]); if(q>=0,rel_at=NREL)));
  );
  [if(q>=0 && ellmul(EC,GS,q)==QS,1,0),att,ndeg,rel_at,#Vec(usedcols),NSOLV,NRES,NGCD,NROOT,SROOT,NSQ,GAE,NAT,LANAT,LAROW,LAENT,NDEGC,getabstime()-t0,NREL];
}

\\ ------------------------------------------------------------ Pollard rho with negation map (section 4.R)
\\ Gm,Qm points of order L; returns [success,steps,escapes,restarts,GAE,NAT,wall_ms]
rho_run(Gm,Qm,L,cap)={
  my(dd,gg,q0,qc,al=vector(16),be=vector(16),Tj=vector(16),steps=0,esc=0,rst=0,g=0,n5=0,P,a,b,tbl,key,k1=0,k2=0,j,e,ok=0,q=-1,t0=getabstime(),xk,Pn,done);
  for(j=1,16,al[j]=random(L);be[j]=random(L);Tj[j]=elladd(EC,ellmul(EC,Gm,al[j]),ellmul(EC,Qm,be[j]));g+=smul(al[j])+smul(be[j])+1);
  tbl=Map();
  while(q<0 && steps<cap,
    rst++;
    a=random(L);b=random(L);P=elladd(EC,ellmul(EC,Gm,a),ellmul(EC,Qm,b));g+=smul(a)+smul(b)+1;
    if(P==[0],next);
    if(!ycan(P[2]),P=ellneg(EC,P);a=(-a)%L;b=(-b)%L);
    k1=0;k2=0;done=0;
    while(!done && steps<cap,
      key=xkey(P[1]);
      if(mapisdefined(tbl,key,&e),
        dd=(b-e[2])%L;
        if(dd!=0,
          gg=gcd(dd,L);
          if(gg==1, q=lift(Mod(e[1]-a,L)/Mod(b-e[2],L)); if(ellmul(EC,Gm,q)!=Qm,q=-1),
            \\ composite order only: solve dd*q = e1-a mod L, try the gg candidates mod L (each test = one scalar multiplication, counted)
            if(gg<=64 && (e[1]-a)%gg==0,
              q0=lift(Mod((e[1]-a)/gg,L/gg)/Mod(dd/gg,L/gg));
              for(jj=0,gg-1, qc=q0+jj*(L/gg);g+=smul(qc);if(ellmul(EC,Gm,qc)==Qm,q=qc;break)))));
        done=1;break);
      mapput(tbl,key,[a,b]);
      j=shift((key*2654435761)%4294967296,-28)+1;
      Pn=elladd(EC,P,Tj[j]);g+=1;steps++;n5+=5;
      a=(a+al[j])%L;b=(b+be[j])%L;
      if(Pn==[0],done=1;break);
      if(!ycan(Pn[2]),Pn=ellneg(EC,Pn);a=(-a)%L;b=(-b)%L);
      if(xkey(Pn[1])==k1,
        \\ fruitless 2-cycle: escape by a doubling
        Pn=elladd(EC,Pn,Pn);g+=1;esc++;a=(2*a)%L;b=(2*b)%L;
        if(Pn==[0],done=1;break);
        if(!ycan(Pn[2]),Pn=ellneg(EC,Pn);a=(-a)%L;b=(-b)%L));
      k2=k1;k1=key;P=Pn;
    );
  );
  [if(q>=0,1,0),steps,esc,rst,steps+esc,n5,getabstime()-t0,g-steps-esc];
}

\\ ------------------------------------------------------------ io helpers and seeds
load_recs(s)={my(L=readstr(Str(OUTDIR,"/curves_",s,".csv")));vector(#L-1,i,eval(Str("[",L[i+1],"]")))}
recs_arm(R,arm)=select(r->r[1]==arm,R);
wl(file,v)=write(file,strjoin(vector(#v,i,Str(v[i])),","));
sbase(s,a,u)=7000000+10000*s+100*a+u;
\\ predicted attempts (section 3) used only for caps
pred_att(id,p)={
  if(id==1,0.886*sqrt(p),if(id==2||id==12,2.51*sqrt(p),if(id==4,0.49*p,if(id==14,0.98*p,if(id==24,1.96*p,2.75*p)))));
}
capatt(id,p)=40*ceil(max(pred_att(id,p),2000));
\\ random subsets without replacement (partial Fisher-Yates); returns 0/1 vector indexed x+1
rand_subset(pool,nkeep,p)={
  my(v=pool,n=#pool,t,j,fb=vector(p));
  for(i=1,nkeep,j=i+random(n-i+1);t=v[i];v[i]=v[j];v[j]=t;fb[v[i]+1]=1);
  fb;
}
\\ one factor-base half/quarter per distinct curve (first appearance order), shared by its instances
\\ kind: "B" half of F_p ; "D1" half of V ; "D2" quarter of V. returns vector indexed by instance position
fb_precompute(rs,kind,seed)={
  my(out=vector(#rs),cache=Map(),ck,fbv,pool,rec,key);
  setrand(seed);
  for(i=1,#rs,
    rec=rs[i];key=[rec[4],rec[7],rec[8],rec[9],rec[10],rec[11],rec[12]];
    if(mapisdefined(cache,key,&fbv),out[i]=fbv;next);
    if(kind=="B",
      pool=vector(rec[4],x,x-1);fbv=rand_subset(pool,rec[4]\2,rec[4]),
      my(Vs,pl);
      fq_setup(rec[4],rec[5],rec[6]);
      my(a=rec[7]*GT^0+rec[8]*GT,b=rec[10]*GT^0+rec[11]*GT,L=List());
      for(x=0,rec[4]-1,if(issquare(x^3+a*x+b),listput(L,x)));
      pl=Vec(L);
      fbv=rand_subset(pl,if(kind=="D1",#pl\2,#pl\4),rec[4]));
    mapput(cache,key,fbv);out[i]=fbv);
  out;
}
