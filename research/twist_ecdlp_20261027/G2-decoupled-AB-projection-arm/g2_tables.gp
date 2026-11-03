\\ g2_tables.gp -- all table arms for one curve and one family. Env: P, C, FAMILY (A|B), [G2OUT, G2DEV].
\\ Arms: D_j (direct, class S), T_k, P (decoupled projection: class S and class F, through actual F_{p^2} projection),
\\ X (coupled, E3 cellrun, N<=40), NC2 redraw, ranks. Writes OUT/tables_p{P}_c{C}_{FAM}.csv and OUT/pt_p{P}_c{C}_{FAM}.csv
default(parisizemax,2000000000);
read("g2_lib.gp");
P=eval(getenv("P")); C=eval(getenv("C")); FAM=genv("FAMILY","A");
NPERM=2000; T=TNUM; RANKCAP=1500;
cr=curve_row(P,C);
ctx(P,cr[3],cr[4]); proj_init();
if(Cl!=cr[5] || Clp!=cr[6], error("curve table mismatch"));
m=g2m(FAM); m2=2*m; NL=g2N(FAM,P);
if(g2dev(), NL=[16]);
print("tables p=",P," c=",C," fam=",FAM," a=",cr[3]," b=",cr[4]," #E=",Cl," #E'=",Clp," d=",Cd," m=",m," m2=",m2," T=",T," N list=",NL);
tg=draw_targets(7000+10*P+C,T);
Av=tg[1]; Bv=tg[2];
setrand(8000+10*P+C);
B2=vectorsmall(T,i,1+random(Clp-1));
\\ ---- projection stage (actual F_{p^2} arithmetic): class F target R'=A*GE+B*GT, class S target R=A*GE (B=O forced)
Ap=vectorsmall(T); Bp=vectorsmall(T); As=vectorsmall(T); Bs=vectorsmall(T);
pmisF=0; pmisS=0; t0p=getabstime();
{
for(ti=1,T,
  my(R2=target_pt(Av[ti],Bv[ti]), pl=projlogs(R2));
  Ap[ti]=pl[1]; Bp[ti]=pl[2];
  if(pl[1]!=Av[ti] || pl[2]!=Bv[ti], pmisF++);
  my(Rs=ellmul(CE2,CGE2,Av[ti]), ps=projlogs(Rs));
  As[ti]=ps[1]; Bs[ti]=ps[2];
  if(ps[1]!=Av[ti] || ps[2]!=0, pmisS++));
}
print("projection stage: class F recovered (A,B) mismatches=",pmisF,"/",T,"  class S (A,B=O) mismatches=",pmisS,"/",T," ms=",getabstime()-t0p);
if(pmisF>0 || pmisS>0, error("projection mismatch"));
\\ keys for class S P arm: A from the projection, B key is a dummy (never used for S counts)
setrand(9002);
fn=Str(OUTDIR,"/tables_p",P,"_c",C,"_",FAM,".csv"); system(Str("rm -f ",fn));
write(fn,"p,c,fam,m,N,T,l,lp,NE,NT,WDm_code,WDm_form,WDm2_code,WDm2_form,WTm_code,WTm_form,WX_code,WX_form,pmisF,pmisS,psdiff,pfdiff,xmis,xmis2,nc2_p,nc2_dc_same,stackrankA,stackdistinctA,rankviol,ms");
pf=Str(OUTDIR,"/pt_p",P,"_c",C,"_",FAM,".csv"); system(Str("rm -f ",pf));
write(pf,"N,ti,A,B,D1,D2,D3,D4,D5,D6,T1,T2,T3,T4,T5,T6,R1,R2,R3,R4,R5,R6,S1,S2,S3,S4,S5,S6,X1,X2,X3,X4,X5,Xe,rankA,rankB,rankPair,pairsmm");
pad6(M,ti,msz)=vector(6,j,if(j<=msz,M[ti,j],0));
sortcodes(L)=vecsort(Vec(L));
{
foreach(NL,N,
  my(t0=getabstime(), S=cellsub(N,m2), NE=S[1], NT=S[2]);
  my(Rd=cellcount(S,Av,Bv), Rpf=cellcount(S,Ap,Bp), Rps=cellcount(S,As,Bv), Rr=cellcount(S,Av,B2));
  my(Dc=Rd[1], Tc=Rd[2], Ds=Rd[3], Ts=Rd[4], Dps=Rps[3]);
  \\ P-S sets must equal D sets; P-F counts must equal direct counts
  my(psd=0, pfd=0);
  for(ti=1,T, if(sortcodes(Ds[ti])!=sortcodes(Dps[ti]), psd++); if(Rpf[1][ti,]!=Dc[ti,] || Rpf[2][ti,]!=Tc[ti,], pfd++));
  \\ work counters by loop vs formula
  my(WDm=workloop(NE,m-1), WDm2=workloop(NE,m2-1), WTm=workloop(NT,m-1));
  my(WXc=-1, WXf=-1, Xc=0, Xe=vector(T), xmis=0, xmis2=0);
  if(N<=XMAXN,
    my(r=cellrun(N,m2,Av,Bv,B2));
    Xc=r[5]; WXc=r[14]; WXf=wformula(N,m2-1);
    my(hits=r[17], sl=List());
    for(i=1,#hits, listput(sl,hits[i][4]*256+hits[i][1]));
    my(seen=Set(Vec(sl)));
    for(i=1,#seen, Xe[seen[i]%256]++);   \\ distinct (E-mask, target) pairs; target index <256, recovered mod 256
    for(ti=1,T, for(j=1,m2-1, if(Xc[ti,j]!=Dc[ti,j]*Tc[ti,m2-j], xmis++); if(r[6][ti,j]!=Dc[ti,j]*Rr[2][ti,m2-j], xmis2++))));
  \\ ranks (A-sets of size m, B-sets of size m, pair union rows of size m+m)
  my(rA=vectorsmall(T), rB=vectorsmall(T), rP=vector(T), npair=vector(T), viol=0, allA=List(), distA=Set());
  for(ti=1,T,
    my(am=List(), bm=List());
    for(i=1,#Ds[ti], if(Ds[ti][i]%8==m, listput(am,Ds[ti][i]\8)));
    for(i=1,#Ts[ti], if(Ts[ti][i]%8==m, listput(bm,Ts[ti][i]\8)));
    am=Vec(am); bm=Vec(bm);
    rA[ti]=maskrank(am,N); rB[ti]=maskrank(bm,N);
    npair[ti]=#am*#bm;
    if(#am*#bm<=RANKCAP && #am*#bm>0, my(rows=List()); for(i=1,#am, for(k=1,#bm, listput(rows,bitor(am[i],bm[k])))); rP[ti]=maskrank(Vec(rows),N), rP[ti]=if(#am*#bm==0,0,-1));
    if(rA[ti]>Dc[ti,m] || rB[ti]>Tc[ti,m] || (rP[ti]>=0 && rP[ti]>rA[ti]+rB[ti]), viol++);
    for(i=1,#am, listput(allA,am[i])));
  my(stR=maskrank(Vec(Set(Vec(allA))),N), stD=#Set(Vec(allA)));
  \\ NC2: B redrawn: pair count (j=k=m) per target, permutation test; D counts unchanged
  my(pairO=vector(T,ti,Dc[ti,m]*Tc[ti,m]), pairR=vector(T,ti,Dc[ti,m]*Rr[2][ti,m]), dsame=(Rr[1]==Dc));
  my(pv=permtest(pairO,pairR,NPERM));
  write(fn,P,",",C,",",FAM,",",m,",",N,",",T,",",Cl,",",Clp,",",NE,",",NT,",",WDm,",",wformula(NE,m-1),",",WDm2,",",wformula(NE,m2-1),",",WTm,",",wformula(NT,m-1),",",WXc,",",WXf,",",pmisF,",",pmisS,",",psd,",",pfd,",",xmis,",",xmis2,",",fmt(pv),",",dsame,",",stR,",",stD,",",viol,",",getabstime()-t0);
  for(ti=1,T,
    my(xr=if(Xc==0, vector(5), vector(5,j,if(j<=m2-1,Xc[ti,j],0))));
    write(pf,csvrow(concat([[N,ti,Av[ti],Bv[ti]],pad6(Dc,ti,m2),pad6(Tc,ti,m2),pad6(Rr[2],ti,m2),pad6(Rps[1],ti,m2),xr,[Xe[ti],rA[ti],rB[ti],rP[ti],npair[ti]]])));
  );
  print("cell N=",N," NE=",NE," NT=",NT," D_m tot=",sum(ti=1,T,Dc[ti,m])," T_m tot=",sum(ti=1,T,Tc[ti,m])," D_m2 tot=",sum(ti=1,T,Dc[ti,m2])," psdiff=",psd," pfdiff=",pfd," Wcode=",[WDm,WDm2,WTm,WXc]," Wform=",[wformula(NE,m-1),wformula(NE,m2-1),wformula(NT,m-1),WXf]," xmis=",xmis,"/",xmis2," nc2_p=",fmt(pv)," rankviol=",viol," ms=",getabstime()-t0);
);
}
quit;
