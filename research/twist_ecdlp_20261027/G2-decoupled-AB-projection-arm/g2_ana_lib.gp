\\ g2_ana_lib.gp -- analysis library: arm costs from pooled sums, bootstrap code. Pure functions of recorded counts.
\\ Feature vector per target (31): 1..6 D_j, 7..12 T_k, 13..17 X_j (coupled, j E-terms, m2-j T-terms), 18 Xe (distinct E-sets in coupled hits),
\\   19..24 S_j (P arm class S counts), 25..29 PW_j = D_j*T_{m2-j} (j=1..5, pair counts), 30 = D_m*T_1, 31 = D_m*T_m.
\\ Work vector Wt (13, already multiplied by T and pooled): 1..6 W_D(j), 7..12 W_T(k), 13 W_X(m2) (0 when the coupled arm was not run).
INF=10^18;
isinf(x)=(x>=INF/10);
NF=31; NW=13; NR=12;
zdiv(w,y,zf) = if(y<=0, if(zf>0 && w>0, w/zf, INF), w/y)
\\ cost vector from pooled sums; zf>0 floors a zero yield at zf (rule of three), zf=0 gives INF
armcost(Fs,Wt,m,m2,zf) = {
  my(z=vector(24));
  for(j=1,m2, z[j]=zdiv(Wt[j],Fs[j],zf); z[16+j]=zdiv(Wt[j],Fs[18+j],zf));
  my(zs=INF, js=0, zps=INF);
  for(j=m,m2, if(z[j]<zs, zs=z[j]; js=j); if(z[16+j]<zps, zps=z[16+j]));
  z[7]=zs; z[8]=js; z[23]=zps;
  z[9]=zdiv(Wt[m]+Wt[6+m],Fs[m],zf);
  z[10]=zdiv(Wt[m]+Wt[7],Fs[m],zf);
  my(zb=min(z[9],z[10]), code=if(z[9]<=z[10], m*10+m, m*10+1));
  for(j=1,m2-1, my(zz=zdiv(Wt[j]+Wt[6+m2-j],Fs[j],zf)); if(zz<zb, zb=zz; code=j*10+m2-j));
  z[11]=zb; z[12]=code;
  if(Wt[13]>0,
    z[13]=zdiv(Wt[13],Fs[18],zf);
    z[14]=zdiv(Wt[13],sum(j=1,m,Fs[12+j]),zf);
    z[15]=zdiv(Wt[13],sum(j=1,m2-1,Fs[12+j]),zf),
    z[13]=INF; z[14]=INF; z[15]=INF);
  z[16]=zdiv(Wt[m]+Wt[6+m],Fs[31],zf);
  z
}
rr(a,b) = if(b>=INF || b<=0, -1, if(a>=INF, INF, a/b))
\\ ratio vector (12): 1 PFmm, 2 PFm1, 3 PFbest, 4 XE, 5 Xgen(E3 definition), 6 Xall, 7 pair(m,m), 8 PSbest, 9 PS vs 2*baseline (PC1 handicapped), 10 PS vs 0.5*baseline (PC1 boosted), 11 Z_D(m2)/Z_D(m), 12 PFmm/(Z_D(m2)/Z_D(m))
rhos(Fs,Wt,m,m2,zf) = {
  my(z=armcost(Fs,Wt,m,m2,zf), zs=z[7], r=vector(NR));
  r[1]=rr(z[9],zs); r[2]=rr(z[10],zs); r[3]=rr(z[11],zs); r[4]=rr(z[13],zs); r[5]=rr(z[14],zs); r[6]=rr(z[15],zs); r[7]=rr(z[16],zs);
  r[8]=rr(z[23],zs); r[9]=rr(z[23],2*zs); r[10]=rr(z[23],zs/2);
  r[11]=rr(z[m2],z[m]);
  r[12]=if(r[1]>0 && r[11]>0 && r[1]<INF && r[11]<INF, r[1]/r[11], if(r[1]>=INF, INF, -1));
  r
}
RHONAME=["PF(j=m;k=m)","PF(j=m;k=1)","PF best (sweep)","X coupled E-sets","X coupled E3 gen","X coupled all-j","pair(m;m) accounting","P-S best","PC1 handicapped x2","PC1 boosted x0.5","Z_D(m2)/Z_D(m)","PF(m;m)/(Z_D(m2)/Z_D(m))"];

\\ 99% percentile interval of column col of matrix R (valid = finite and >=0 entries); returns [lo,hi,nvalid]
pint(R,col) = {
  my(v=List());
  for(b=1,matsize(R)[1], my(x=R[b,col]); if(x>=0, listput(v,x)));
  v=vecsort(Vec(v)); my(n=#v);
  if(n<100, return([-1,-1,n]));
  [v[max(1,round(0.005*n))], v[min(n,round(0.995*n))], n]
}
\\ curve-level bootstrap: CV = vector of [Fsum, Wt] per curve; returns matrix B x NR
cboot(CV,m,m2,B,seed) = {
  my(nc=#CV, R=matrix(B,NR));
  setrand(seed);
  for(b=1,B,
    my(Fs=vector(NF), Wt=vector(NW));
    for(i=1,nc, my(k=1+random(nc)); Fs+=CV[k][1]; Wt+=CV[k][2]);
    R[b,]=rhos(Fs,Wt,m,m2,0));
  R
}
\\ target-level bootstrap within one curve: Frows = vector of T feature vectors; Wt for that curve (already x T)
tboot(Frows,Wt,m,m2,B,seed) = {
  my(T=#Frows, R=matrix(B,NR));
  setrand(seed);
  for(b=1,B,
    my(Fs=vector(NF));
    for(i=1,T, Fs+=Frows[1+random(T)]);
    R[b,]=rhos(Fs,Wt,m,m2,0));
  R
}
