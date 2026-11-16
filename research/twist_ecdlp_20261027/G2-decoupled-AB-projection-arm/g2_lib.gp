\\ g2_lib.gp -- shared library for G2-decoupled-AB-projection-arm. Re-uses the E3 library UNMODIFIED (read first).
\\ Callers: default(parisizemax,2000000000); then read("g2_lib.gp") from the G2 directory.
\\ Conventions as E3: odd #E=Cl (prime), odd cyclic #E'=Clp, V={0..N-1}, E-type x: f(x) nonzero square, T-type x: non-square.
E3DIR="../E3-eigen-decomposition-Fq2/";
read(Str(E3DIR,"e3_grid.gp"));
read(Str(E3DIR,"e3_lib.gp"));
if(OUTDIR=="out" && getenv("G2OUT")!=0 && getenv("G2OUT")!="", OUTDIR=getenv("G2OUT"));

\\ ---- grid (PREREGISTRATION section 7)
g2N(fam,p) = if(fam=="A", if(p<401,[16,24,32,40,48,64],[16,24,32,40]), [12,16,20])
g2m(fam) = if(fam=="A",2,3)
g2dev() = (genv("G2DEV","0")=="1")
\\ coupled arm X only for N<=40 (as E3)
XMAXN=40;

\\ ---- ffelt in F_p -> integer
ffint(z) = {my(P=z.pol); if(P==0, return(0)); if(poldegree(P)>0, error("ffint: element not in F_p")); polcoeff(P,0)}

\\ ---------------------------------------------------------------- projection (actual F_{p^2} arithmetic)
\\ pi = Frobenius (x,y)->(x^p,y^p); A=(R+piR)/2, B=(R-piR)/2 where /2 = multiplication by 1/2 mod the (odd) group order
proj_init() = { Ci2=lift(1/Mod(2,Cl*Clp)); }
projpts(R2) = {
  my(piR, S, D);
  if(R2==[0], return([[0],[0]]));
  piR=[R2[1]^Cp,R2[2]^Cp];
  S=elladd(CE2,R2,piR); D=elladd(CE2,R2,ellneg(CE2,piR));
  [ellmul(CE2,S,Ci2), ellmul(CE2,D,Ci2)]
}
\\ signed discrete log of an E(F_p)-point (given in E(F_{p^2}) coordinates) w.r.t. CGE, via the x-keyed tables
elog(Q) = {
  my(x, y);
  if(Q==[0], return(0));
  x=ffint(Q[1]); y=ffint(Q[2]);
  if(y==Cyv[x+1], Cu[x+1], (Cl-Cu[x+1])%Cl)
}
\\ signed discrete log of a point of E'(F_p) given as (x, c*t) in E(F_{p^2}) w.r.t. CGT
tlog(Q) = {
  my(x, c);
  if(Q==[0], return(0));
  x=ffint(Q[1]); c=ffint(Q[2]/Ct);
  if(c==Cyv[x+1], Cw[x+1], (Clp-Cw[x+1])%Clp)
}
projlogs(R2) = { my(P=projpts(R2)); [elog(P[1]), tlog(P[2])] }

\\ ---------------------------------------------------------------- one cell: subset tables, then counting passes
cellsub(N, msz) = {
  my(Eidx=List(), Tidx=List(), NE, NT, Ev, Tv);
  for(x=0,N-1, if(Ctype[x+1]==1, listput(Eidx,x), if(Ctype[x+1]==-1, listput(Tidx,x))));
  Eidx=Vec(Eidx); Tidx=Vec(Tidx); NE=#Eidx; NT=#Tidx;
  Ev=vector(NE,i,Cu[Eidx[i]+1]); Tv=vector(NT,i,Cw[Tidx[i]+1]);
  [NE,NT,vector(msz+1,j,subtab(Eidx,Ev,j-1,Cl)), vector(msz+1,k,subtab(Tidx,Tv,k-1,Clp)), N, msz]
}
\\ counting pass: Ak[ti] in 1..Cl-1, Bk[ti] in 1..Clp-1 (keys). Returns [Dc (T x msz), Tc (T x msz), Dsets (vector of lists), Tsets]
\\ set code: mask*8+j  (masks are bit sets over x; j = number of terms)
cellcount(S, Ak, Bk) = {
  my(T=#Ak, msz=S[6], subE=S[3], subT=S[4]);
  my(dhead=vectorsmall(Cl), dnxt=vectorsmall(T), thead=vectorsmall(Clp), tnxt=vectorsmall(T));
  for(ti=1,T, dnxt[ti]=dhead[Ak[ti]+1]; dhead[Ak[ti]+1]=ti; tnxt[ti]=thead[Bk[ti]+1]; thead[Bk[ti]+1]=ti);
  my(Dc=matrix(T,msz), Tc=matrix(T,msz), Ds=vector(T,i,List()), Ts=vector(T,i,List()));
  for(j=1,msz, my(L=subE[j+1]);
    for(r=1,#L, my(rc=L[r]);
      for(q=1,#rc[2], my(tt=dhead[rc[2][q]+1]);
        while(tt, Dc[tt,j]++; listput(Ds[tt],rc[1]*8+j); tt=dnxt[tt]))));
  for(k=1,msz, my(L=subT[k+1]);
    for(r=1,#L, my(rc=L[r]);
      for(q=1,#rc[2], my(tt=thead[rc[2][q]+1]);
        while(tt, Tc[tt,k]++; listput(Ts[tt],rc[1]*8+k); tt=tnxt[tt]))));
  [Dc,Tc,Ds,Ts]
}

bitsrow(mask,N) = vector(N,i,bittest(mask,i-1))
\\ rank over Q of the 0/1 incidence matrix whose rows are the given masks (columns = x in V)
maskrank(masks,N) = {
  if(#masks==0, return(0));
  my(M=matrix(#masks,N,r,c,bittest(masks[r],c-1)));
  matrank(M)
}

\\ two-sample permutation test on the difference of means (NPERM label shuffles), two-sided add-one p-value
permtest(x,y,nperm) = {
  my(n1=#x, n2=#y, pool=concat(x,y), n=#pool, tot=vecsum(pool), s0=vecsum(x), obs, cnt=0);
  obs=abs(s0/n1 - (tot-s0)/n2);
  if(vecmax(pool)==vecmin(pool), return(1.0));
  for(r=1,nperm,
    my(pp=pool, s1=0);
    for(i=1,n1, my(k=i+random(n-i+1), tmp=pp[i]); pp[i]=pp[k]; pp[k]=tmp; s1+=pp[i]);
    my(st=abs(s1/n1-(tot-s1)/n2));
    if(st>=obs-1e-12, cnt++));
  (cnt+1)/(nperm+1)
}

\\ registered work units (formulas; the table script also counts them by loop)
wformula(n,r) = if(r>n || r<0, 0, binomial(n,r)*2^r)

\\ ---------------------------------------------------------------- Semaev solvers (all return Vec of masks over x in V)
S3g(x1,x2,x3,a,b) = (x1-x2)^2*x3^2 - 2*((x1+x2)*(x1*x2+a)+2*b)*x3 + ((x1*x2-a)^2 - 4*b*(x1+x2))
semaev_init2() = {
  semaev_init();     \\ E3 library: CS4, CS4w (uses Ca,Cb of the current context)
}
\\ D_2 on E (target x-coordinate xA in F_p): x1 E-type in V
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
\\ T_2 on the twist E_T: Y^2=X^3+a d^2 X+b d^3 with X = d x; target X-coordinate XB in F_p; x1 T-type in V
semT2(XB, N) = {
  my(out=List(), q, rts, x2, aT=(Ca*Cd^2)%Cp, bT=(Cb*Cd^3)%Cp);
  for(x1=0,N-1,
    if(Ctype[x1+1]!=-1, next);
    q=S3g((Cd*x1)%Cp,u,XB,aT,bT)*Mod(1,Cp);
    if(q==0, error("semT2: identically zero polynomial"));
    if(poldegree(q)<1, next);
    rts=polrootsmod(q,Cp);
    for(i=1,#rts, x2=lift(rts[i]*Mod(Cinvd,Cp)); if(x2>=0 && x2<N && x2!=x1, listput(out,2^x1+2^x2))));
  Vec(Set(Vec(out)))
}
\\ D_4 on E: S5(x1,x2,x3,x4,xA)=0 with x1<x2<x3 E-type in V, x4 root in F_p (numeric S5 via resultant with S4w)
semD4(xA, N) = {
  my(out=List(), q, g, R, rts, x4, Ei=List());
  for(x=0,N-1, if(Ctype[x+1]==1, listput(Ei,x)));
  Ei=Vec(Ei);
  forsubset([#Ei,3], s,
    my(x1=Ei[s[1]], x2=Ei[s[2]], x3=Ei[s[3]]);
    q=S3(x1,x2,u)*Mod(1,Cp);
    g=substvec(CS4w,[y2,y3],[x3,xA]);
    R=polresultant(q,g,u);
    if(R==0, error("semD4: identically zero polynomial"));
    if(poldegree(R,w)<1, next);
    rts=polrootsmod(R,Cp);
    for(i=1,#rts, x4=lift(rts[i]); if(x4>=0 && x4<N && x4!=x1 && x4!=x2 && x4!=x3, listput(out,2^x1+2^x2+2^x3+2^x4))));
  Vec(Set(Vec(out)))
}
\\ semX4: from the E3 library (unchanged).

csvrow(v) = strjoin(apply(z->Str(z),v),",")
