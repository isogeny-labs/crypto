\\ lib.gp -- shared field layer for experiment E4 (Koblitz K0/K1 twist pair, F_{2^m}, m odd).
\\ Elements of F_q are integers 0..q-1 (bit i = coefficient of a^i, a = ffgen(2^m,'a)).
\\ Arithmetic by exp/log tables (exp table doubled), trace and half-trace by F_2-linear tables.
\\ All tables are validated against native PARI FFELT arithmetic in stage0_identities.gp.
\\ (scripts set default(parisizemax,2000000000) themselves before read(); doing it inside read() aborts the file)
GPVER = version();

fint(x) = subst(x.pol,'a,2);
ffe(n) = { my(v,L,r); if(n==0, return(GEN*0)); v=binary(n); L=#v; r=GEN*0; for(i=1,L, if(v[i], r+=GEN^(L-i))); r };

lib_init(m) = {
  my(e,ok,G0,Tb,Hb,t,s,x,w);
  M=m; Q=2^m; QM1=Q-1;
  GEN=ffgen(2^m,'a); MODINT=subst(lift(GEN.mod),'a,2);
  EXP=vectorsmall(2*QM1); LOGT=vectorsmall(Q);
  e=1; ok=1;
  for(i=0,QM1-1, EXP[i+1]=e; if(i>0 && e==1, ok=0; break); e=e<<1; if(e>>m, e=bitxor(e,MODINT)));
  GENPRIM = ok;
  if(!ok,
    G0=ffprimroot(GEN); e=GEN^0;
    for(i=0,QM1-1, EXP[i+1]=fint(e); e*=G0));
  for(i=0,QM1-1, EXP[QM1+i+1]=EXP[i+1]; LOGT[EXP[i+1]+1]=i);
  Tb=vector(m,j,lift(trace(GEN^(j-1))));
  Hb=vector(m,j, t=GEN^(j-1); s=t; for(i=1,(m-1)/2, t=t^4; s+=t); fint(s));
  TRT=vectorsmall(Q); HTT=vectorsmall(Q);
  for(j=0,m-1, for(x=2^j,2^(j+1)-1, TRT[x+1]=bitxor(TRT[x-2^j+1],Tb[j+1]); HTT[x+1]=bitxor(HTT[x-2^j+1],Hb[j+1])));
  TYT=vectorsmall(Q); TYT[1]=2;
  for(x=1,QM1, TYT[x+1]=bitxor(TRT[x+1], TRT[EXP[(QM1-LOGT[x+1])%QM1+1]+1]));
  CAN=vectorsmall(Q);
}

mul(a,b) = if(a&&b, EXP[LOGT[a+1]+LOGT[b+1]+1], 0);
sq(a) = if(a, EXP[2*LOGT[a+1]+1], 0);
inv(a) = EXP[(QM1-LOGT[a+1])%QM1+1];
S3(x1,x2,x3) = { my(p12=mul(x1,x2)); bitxor(bitxor(sq(bitxor(p12,bitxor(mul(x1,x3),mul(x2,x3)))), mul(p12,x3)), 1) };
\\ rational third abscissas of a pair of nonzero abscissas: roots of S3(x1,x2,X)=0 in F_q
roots3(x1,x2) = {
  my(s=bitxor(x1,x2),B,ls,lB,lt,c,w,k,r);
  B=mul(x1,x2);
  if(s==0, return([bitxor(B,inv(B))]));
  ls=LOGT[s+1]; lB=LOGT[B+1]; lt=(ls-lB)%QM1;
  c=bitxor(EXP[2*ls+1],EXP[2*lt+1]);          \\ c = AC/B^2 = A + (1/x1+1/x2)^2
  if(TRT[c+1], return([]));
  w=HTT[c+1]; k=EXP[(lB-2*ls)%QM1+1];           \\ k = B/A
  r=mul(k,w); [r,bitxor(r,k)]
}
canon(x) = { my(mn,y); if(CAN[x+1], return(CAN[x+1])); mn=x; y=x; for(i=1,M-1, y=sq(y); if(y<mn,mn=y)); y=x; for(i=1,M, CAN[y+1]=mn; y=sq(y)); mn };

\\ deterministic draw of an F_2-subspace V (pop 1 random, pop 2 polynomial basis) and an extension vector w
makeV(l,pop) = {
  my(bas,ech,v,r,el,inV,w);
  if(pop==2,
    bas=vector(l,j,1<<(j-1)),
    bas=[]; ech=[];
    while(#bas<l, v=random(QM1)+1; r=v; for(i=1,#ech, r=min(r,bitxor(r,ech[i]))); if(r, bas=concat(bas,[v]); ech=vecsort(concat(ech,[r]),,4))));
  el=vectorsmall(1);
  for(j=1,#bas, el=concat(el,vectorsmall(#el,i,bitxor(el[i],bas[j]))));
  inV=vectorsmall(Q); for(i=1,#el, inV[el[i]+1]=1);
  w=0; while(w==0 || inV[w+1], w=random(QM1)+1);
  [el,inV,w]
}
mkbase(el,armE) = { my(L=List()); for(i=1,#el, my(x=el[i]); if(x && (!armE || TYT[x+1]==0), listput(L,x))); Vecsmall(L) }

csvline(v) = { my(s=""); for(i=1,#v, s=Str(s,if(i>1,",",""),v[i])); s }

\\ The sieve (Semaev m=2 decomposition). For each target abscissa xR and each base abscissa x1, ONE counted
\\ quadratic solve: A x2^2 + B x2 + C = 0 with A=(x1+xR)^2, B=x1 xR, C=B^2+1 (one inversion via log table, one
\\ trace test, one half-trace extraction). Keeps roots x2 in V\{0} (armE: also of K0 type). Returns
\\ [counted solves, vector of per-target sorted unique relation keys min*Q+max].
sieve(base,inV,tgts,armE) = {
  my(n=#base,nt=#tgts,ns=0,out=vector(nt),xR,lR,x1,s,ls,lB,lt,c,w,k,r,B,acc);
  for(ti=1,nt,
    xR=tgts[ti]; lR=LOGT[xR+1]; acc=List();
    for(i=1,n,
      x1=base[i]; ns++; s=bitxor(x1,xR);
      if(s==0,
        B=EXP[2*LOGT[x1+1]+1]; r=bitxor(B,EXP[(QM1-LOGT[B+1])%QM1+1]);
        if(r && inV[r+1] && (!armE || TYT[r+1]==0), listput(acc,if(x1<r,x1*Q+r,r*Q+x1)))
      ,
        ls=LOGT[s+1]; lB=LOGT[x1+1]+lR; if(lB>=QM1,lB-=QM1);
        lt=ls-lB; if(lt<0,lt+=QM1);
        c=bitxor(EXP[2*ls+1],EXP[2*lt+1]);
        if(!TRT[c+1],
          w=HTT[c+1]; k=EXP[(lB-2*ls)%QM1+1];
          r=if(w,EXP[LOGT[k+1]+LOGT[w+1]+1],0);
          if(r && inV[r+1] && (!armE || TYT[r+1]==0), listput(acc,if(x1<r,x1*Q+r,r*Q+x1)));
          r=bitxor(r,k);
          if(r && inV[r+1] && (!armE || TYT[r+1]==0), listput(acc,if(x1<r,x1*Q+r,r*Q+x1))))));
    out[ti]=vecsort(Vecsmall(acc),,8));
  [ns,out]
}
