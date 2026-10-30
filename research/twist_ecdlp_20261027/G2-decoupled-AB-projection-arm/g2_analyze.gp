\\ g2_analyze.gp -- reads OUT/*.csv only. Bootstrap (curve-level B=10000 seed 9002; target-level B=2000 seed 9001), controls, falsifier tests.
\\ Writes OUT/results.csv (long format), OUT/cellsums.csv, results.json (top directory); prints the report to stdout (-> out/analysis.out).
default(parisizemax,2000000000);
read("g2_ana_lib.gp");
read("g2_lib.gp");
T=TNUM; BC=10000; BT=2000; ZFLOOR=3;
if(g2dev(), BC=500; BT=200);
CUR=read_csv(Str(OUTDIR,"/curves.csv"));       \\ [p,c,a,b,l,lp,t,kind]
curvesof(p,kinds) = {my(L=List()); for(i=1,#CUR, if(CUR[i][1]==p && setsearch(Set(kinds),CUR[i][8]), listput(L,CUR[i][2]))); Vec(L)}
famlist=["A","B"];
famPs(fam) = if(fam=="A",[101,211,401],[101,211])
E3N(fam) = if(fam=="A",[16,24,32,40],[12,16,20])
fnum(x)=if(isinf(x),"inf",fmt(x));
jn(x)=if(x==0,"0",if(isinf(x),"null",fmt(x)));
JL=List();
jf(x)=if(type(x)=="t_STR", Str("\"",x,"\""), if(type(x)=="t_INT", Str(x), if(x==0,"0",if(isinf(x),"null",fmt(x)))));
jset(k,v)=listput(JL,Str("\"",k,"\":",jf(v)));
mkfeat(r,m,m2) = {
  my(f=vector(NF), D=vector(6,j,r[4+j]), Tt=vector(6,k,r[10+k]));
  for(j=1,6, f[j]=D[j]; f[6+j]=Tt[j]; f[18+j]=r[22+j]);
  for(j=1,5, f[12+j]=r[28+j]);
  f[18]=r[34];
  for(j=1,m2-1, f[24+j]=D[j]*Tt[m2-j]);
  f[30]=D[m]*Tt[1]; f[31]=D[m]*Tt[m];
  f
}
\\ cell record: [N,NE,NT,l,lp,rows,Fs,Wt,hasX,tabrow]
DATA=Map();
loadcurve(fam,p,c) = {
  my(m=g2m(fam), m2=2*m, tab=read_csv(Str(OUTDIR,"/tables_p",p,"_c",c,"_",fam,".csv")), pt=read_csv(Str(OUTDIR,"/pt_p",p,"_c",c,"_",fam,".csv")));
  my(byN=Map(), out=List());
  for(i=1,#pt, my(r=pt[i], N=r[1]); if(!mapisdefined(byN,N), mapput(byN,N,List())); my(L=mapget(byN,N)); listput(L,r); mapput(byN,N,L));
  for(i=1,#tab, my(tr=tab[i], N=tr[5], NE=tr[9], NT=tr[10]);
    my(L=Vec(mapget(byN,N)), rows, Fs=vector(NF), hasX=(tr[17]>=0), Wt=vector(NW));
    if(#L!=T, error("target count mismatch ",p," ",c," ",N));
    rows=vector(#L,k,mkfeat(L[k],m,m2));
    for(k=1,#rows, Fs+=rows[k]);
    for(j=1,6, Wt[j]=T*wformula(NE,j-1); Wt[6+j]=T*wformula(NT,j-1));
    Wt[13]=if(hasX, T*wformula(N,m2-1), 0);
    listput(out,[N,NE,NT,tr[7],tr[8],rows,Fs,Wt,hasX,tr]));
  mapput(DATA,Str(fam,"_",p,"_",c),Vec(out));
}
cellof(fam,p,c,N) = {my(V=mapget(DATA,Str(fam,"_",p,"_",c))); for(i=1,#V, if(V[i][1]==N, return(V[i]))); error("no cell")}
{
foreach(famlist,fam, foreach(famPs(fam),p, foreach(curvesof(p,[0,1,2]),c, loadcurve(fam,p,c))));
}
poolset(fam,p,N,cs) = {
  my(Fs=vector(NF), Wt=vector(NW), CV=vector(#cs));
  for(i=1,#cs, my(cc=cellof(fam,p,cs[i],N)); CV[i]=[cc[7],cc[8]]; Fs+=cc[7]; Wt+=cc[8]);
  [Fs,Wt,CV]
}
fr=Str(OUTDIR,"/results.csv"); system(Str("rm -f ",fr));
write(fr,"fam,m,p,scope,N,ncur,NE_list,powered,Dm_tot,arm_id,arm,point,lo99,hi99,lower_bound,ci_kind,nvalid");
emit(fam,p,scope,N,ncur,nel,pow,dm,a,pt,lb,ci,ck,nx) = if(!nx && a>=4 && a<=6, write(fr,fam,",",g2m(fam),",",p,",",scope,",",N,",",ncur,",",nel,",",pow,",",dm,",",a,",",RHONAME[a],",NA,NA,NA,0,",ck,",0"), write(fr,fam,",",g2m(fam),",",p,",",scope,",",N,",",ncur,",",nel,",",pow,",",dm,",",a,",",RHONAME[a],",",fnum(pt),",",fnum(ci[1]),",",fnum(ci[2]),",",lb,",",ck,",",ci[3]));
pointest(Fs,Wt,m,m2,ncur) = {
  my(rf=rhos(Fs,Wt,m,m2,ZFLOOR*ncur), rw=rhos(Fs,Wt,m,m2,0), lb=vector(NR,a,(isinf(rw[a]) && !isinf(rf[a]) && rf[a]>0)));
  [rf,lb,rw]
}
\\ ---------------------------------------------------------------- cell sums file
fcs=Str(OUTDIR,"/cellsums.csv"); system(Str("rm -f ",fcs));
write(fcs,"fam,p,c,N,NE,NT,l,lp,D1,D2,D3,D4,D5,D6,T1,T2,T3,T4,T5,T6,S1,S2,S3,S4,S5,S6,X1,X2,X3,X4,X5,Xe,hasX,psdiff,pfdiff,xmis,xmis2,nc2_p,nc2_dc_same,rankviol,stackrankA,stackdistinctA");
{
foreach(famlist,fam, foreach(famPs(fam),p, foreach(curvesof(p,[0,1,2]),c,
  my(V=mapget(DATA,Str(fam,"_",p,"_",c)));
  for(i=1,#V, my(cc=V[i], F=cc[7], tr=cc[10]);
    write(fcs,fam,",",p,",",c,",",cc[1],",",cc[2],",",cc[3],",",cc[4],",",cc[5],",",csvrow(F[1..6]),",",csvrow(F[7..12]),",",csvrow(F[19..24]),",",csvrow(F[13..17]),",",F[18],",",cc[9],",",tr[21],",",tr[22],",",tr[23],",",tr[24],",",tr[25],",",tr[26],",",tr[29],",",tr[27],",",tr[28])))));
}
print("=== G2 analysis ===");
\\ ---------------------------------------------------------------- per cell: pooled (primary / ext) and single-curve rows
RES=Map();   \\ key Str(fam,"_",p,"_",scope,"_",N) -> [point, lb, cis, Fs, Wt, ncur, powered, Dm, raw]
{
foreach(famlist,fam,
 my(m=g2m(fam), m2=2*m);
 foreach(famPs(fam),p,
  my(prim=curvesof(p,[0]), ext=curvesof(p,[0,1]), xtr=curvesof(p,[2]));
  foreach(g2N(fam,p),N,
    foreach([["primary",prim],["ext",ext]],ss,
      my(scope=ss[1], cs=ss[2]);
      my(PS=poolset(fam,p,N,cs), Fs=PS[1], Wt=PS[2], CV=PS[3]);
      my(pe=pointest(Fs,Wt,m,m2,#cs), Rc=cboot(CV,m,m2,BC,9002), pow=(Fs[m]>=20), cis=vector(NR,a,pint(Rc,a)));
      my(nel=strjoin(apply(z->Str(z),vector(#cs,i,cellof(fam,p,cs[i],N)[2])),"/"));
      for(a=1,NR, emit(fam,p,scope,N,#cs,nel,pow,Fs[m],a,pe[1][a],pe[2][a],cis[a],"curve",Wt[13]>0));
      mapput(RES,Str(fam,"_",p,"_",scope,"_",N),[pe[1],pe[2],cis,Fs,Wt,#cs,pow,Fs[m],pe[3]]));
    foreach(concat(prim,xtr),c,
      my(cc=cellof(fam,p,c,N), Fs=cc[7], Wt=cc[8], pow=(Fs[m]>=20));
      my(pe=pointest(Fs,Wt,m,m2,1), Rt=tboot(cc[6],Wt,m,m2,BT,9001), cis=vector(NR,a,pint(Rt,a)), sc=Str(if(c==9,"extreme","curve"),c));
      for(a=1,NR, emit(fam,p,sc,N,1,cc[2],pow,Fs[m],a,pe[1][a],pe[2][a],cis[a],"target",Wt[13]>0));
      mapput(RES,Str(fam,"_",p,"_",sc,"_",N),[pe[1],pe[2],cis,Fs,Wt,1,pow,Fs[m],pe[3]])))));
}
\\ ---------------------------------------------------------------- pooled grid (per family, primary curves, E3 N grid), curve-level interval
{
foreach(famlist,fam,
  my(m=g2m(fam), m2=2*m, CV=List());
  foreach(famPs(fam),p, foreach(curvesof(p,[0]),c,
    my(Fs=vector(NF), Wt=vector(NW));
    foreach(E3N(fam),N, my(cc=cellof(fam,p,c,N)); Fs+=cc[7]; Wt+=cc[8]);
    listput(CV,[Fs,Wt])));
  CV=Vec(CV);
  my(Fs=vector(NF), Wt=vector(NW)); for(i=1,#CV, Fs+=CV[i][1]; Wt+=CV[i][2]);
  my(pe=pointest(Fs,Wt,m,m2,#CV), Rc=cboot(CV,m,m2,BC,9002), ac=armcost(Fs,Wt,m,m2,0));
  for(a=1,NR, emit(fam,0,"pooled_grid_primary",0,#CV,"-",1,Fs[m],a,pe[1][a],pe[2][a],pint(Rc,a),"curve",Wt[13]>0));
  print("POOLED GRID fam=",fam," (",#CV," primary curves, all E3-grid N pooled by ratio of totals): D* = D_",ac[8],"; rho PF(m,m)=",fnum(pe[1][1])," PF(m,1)=",fnum(pe[1][2])," PF best=",fnum(pe[1][3])," (j*10+k=",ac[12],") X E-sets=",fnum(pe[1][4])," X gen(E3 form, vs D*)=",fnum(pe[1][5])," pair(m,m) accounting=",fnum(pe[1][7]));
  print("   99% curve-level CI: PF(m,m)=[",fnum(pint(Rc,1)[1]),",",fnum(pint(Rc,1)[2]),"]  PF(m,1)=[",fnum(pint(Rc,2)[1]),",",fnum(pint(Rc,2)[2]),"]  PF best=[",fnum(pint(Rc,3)[1]),",",fnum(pint(Rc,3)[2]),"]  X E-sets=[",fnum(pint(Rc,4)[1]),",",fnum(pint(Rc,4)[2]),"]"));
}
\\ ---------------------------------------------------------------- primary table printout
print("\n--- Primary pooled cells (2 primary curves per p; ratio of totals; 99% curve-level interval, B=",BC," seed 9002). rho = Z_arm/Z_D* in Semaev work units per independent E-relation ---");
print("fam p N | NE/NT | D_m total | D* | rho PF(m,m) [CI] | PF(m,1) [CI] | PF best (j*10+k) [CI] | X E-sets [CI] | pair(m,m) accounting | powered");
{
foreach(famlist,fam, foreach(famPs(fam),p, foreach(g2N(fam,p),N,
  my(R=mapget(RES,Str(fam,"_",p,"_primary_",N)), m=g2m(fam), m2=2*m, ac=armcost(R[4],R[5],m,m2,0));
  my(nel=strjoin(apply(z->Str(z),vector(2,i,cellof(fam,p,i,N)[2])),"/"), nt=strjoin(apply(z->Str(z),vector(2,i,cellof(fam,p,i,N)[3])),"/"));
  print(fam," ",p," ",N," | ",nel," ",nt," | ",R[8]," | D_",ac[8]," | ",fnum(R[1][1])," [",fnum(R[3][1][1]),",",fnum(R[3][1][2]),"] | ",fnum(R[1][2])," [",fnum(R[3][2][1]),",",fnum(R[3][2][2]),"] | ",fnum(R[1][3])," (",ac[12],") [",fnum(R[3][3][1]),",",fnum(R[3][3][2]),"] | ",if(R[5][13]==0,"NA (coupled arm not run, N>40)",Str(fnum(R[1][4]),if(R[2][4]," LB","")," [",fnum(R[3][4][1]),",",fnum(R[3][4][2]),"]"))," | ",fnum(R[1][7])," | ",R[7]))));
}
\\ ================================================================ CONTROLS AND PREDICTIONS
print("\n--- Controls ---");
\\ PCSYNTH
{
my(L=readstr(Str(OUTDIR,"/pcsynth.csv")));
for(i=2,#L, if(L[i]!="", print("PCSYNTH (positive control, synthetic): ",L[i]); if(strsplit(L[i],",")[1]=="overall", jset("PCSYNTH_overall",strsplit(L[i],",")[2]))));
}
\\ PC1 injection on real data (primary pooled powered cells)
npc1=0; pc1ok=0; pc1ci=0; pc1bad=List();
{
foreach(famlist,fam, foreach(famPs(fam),p, foreach(g2N(fam,p),N,
  my(R=mapget(RES,Str(fam,"_",p,"_primary_",N)));
  if(R[7],
    npc1++;
    my(r9=R[1][9], r10=R[1][10], r8=R[1][8]);
    if(abs(r9-0.5)<=1e-9 && abs(r10-2.0)<=1e-9, pc1ok++, listput(pc1bad,[fam,p,N,r9,r10]));
    if(R[3][9][2]<1 && R[3][10][1]>1, pc1ci++)))));
}
print("PC1 injection (real data, primary pooled powered cells): ",npc1," cells; rho_P = 0.500 and 2.000 within 1e-9 in ",pc1ok,"; 99% curve-level interval excludes 1 on the correct side (handicapped hi<1 and boosted lo>1) in ",pc1ci,"; failures ",Vec(pc1bad));
\\ H1 / PC2: P-S counts equal D counts, sets identical, rho exactly 1
h1n=0; h1bad=0; h1tbad=0; h1setbad=0; h1rhobad=0; h1pfbad=0;
{
foreach(famlist,fam, foreach(famPs(fam),p, foreach(curvesof(p,[0,1,2]),c,
  my(V=mapget(DATA,Str(fam,"_",p,"_",c)), m=g2m(fam), m2=2*m);
  for(i=1,#V, my(cc=V[i], F=cc[7], tr=cc[10]); h1n++;
    for(j=1,m2, if(F[18+j]!=F[j], h1bad++));
    for(k=1,#cc[6], my(f=cc[6][k]); for(j=1,m2, if(f[18+j]!=f[j], h1tbad++)));
    h1setbad+=tr[21]; h1pfbad+=tr[22];
    my(r=rhos(cc[7],cc[8],m,m2,0)); if(r[8]>0 && abs(r[8]-1)>1e-9, h1rhobad++)))));
}
print("H1/PC2 (class-S through the F_{p^2} projection path): curve-cells=",h1n," ; sum-count differences P-S vs D=",h1bad," ; per-target count differences=",h1tbad," ; per-target set differences (psdiff)=",h1setbad," ; class-F P counts vs direct (pfdiff)=",h1pfbad," ; cells with rho_E(P,S) != 1 beyond 1e-9: ",h1rhobad);
\\ H2: rho_E(P,F,j,k) / Z_D(j) = 1 + W_T(k)/W_D(j) exactly
h2n=0; h2dev=0; h2le1=0; h2maxdev=0;
{
foreach(famlist,fam, foreach(famPs(fam),p, foreach(curvesof(p,[0,1,2]),c,
  my(V=mapget(DATA,Str(fam,"_",p,"_",c)), m=g2m(fam), m2=2*m);
  for(i=1,#V,
    my(cc=V[i], F=cc[7], W=cc[8]);
    for(j=1,m2,
      for(k=1,m2,
        if(F[j]>0 && W[6+k]>0,
          my(zf, zd, rho, pred, dv);
          zf=(W[j]+W[6+k])/F[j]; zd=W[j]/F[j]; rho=zf/zd; pred=1+W[6+k]/W[j];
          h2n++;
          dv=abs(rho-pred)/pred;
          if(dv>h2maxdev, h2maxdev=dv);
          if(dv>1e-9, h2dev++);
          if(rho<=1, h2le1++))))))));
}
print("H2 (identity rho_E(P,F,j,k)/Z_D(j) = 1+W_T(k)/W_D(j)): arms tested (curve-cell x j x k with D_j>0)=",h2n," ; max relative deviation=",fmt(h2maxdev)," ; deviations >1e-9: ",h2dev," ; rho<=1: ",h2le1);
\\ H2 magnitude: rho(P,F,m,m) vs 1+NT/NE (m=2) and (m,1)
{
foreach(famlist,fam,
  my(m=g2m(fam), lo2=INF, hi2=0, lo1=INF, hi1=0, lod=INF, hid=0, n=0);
  foreach(famPs(fam),p, foreach(curvesof(p,[0]),c, foreach(g2N(fam,p),N,
    my(cc=cellof(fam,p,c,N), F=cc[7], W=cc[8]);
    if(F[m]>=20, n++;
      my(a=1+W[6+m]/W[m], b=1+W[7]/W[m]);
      lo2=min(lo2,a); hi2=max(hi2,a); lo1=min(lo1,b); hi1=max(hi1,b))))); 
  print("H2 magnitude fam=",fam," (primary curve-cells with D_m>=20, n=",n,"): rho_E(P,F,j=k=m)/Z_D(m) = 1+W_T(m)/W_D(m) in [",fmt(lo2),",",fmt(hi2),"] ; rho_E(P,F,j=m,k=1)/Z_D(m) = 1+1/W_D(m) in [",fmt(lo1),",",fmt(hi1),"]"));
}
\\ H3: pair-count accounting, PC3, rank
pc3n=0; pc3hit=List(); pc3all=List(); zxr=0; zxn=0; zxmax=0; xcross=0;
{
foreach(famlist,fam, foreach(famPs(fam),p, foreach(g2N(fam,p),N,
  my(R=mapget(RES,Str(fam,"_",p,"_primary_",N)), m=g2m(fam), m2=2*m, Fs=R[4], Wt=R[5], ac=armcost(Fs,Wt,m,m2,0));
  my(yt=Fs[6+m]/(T*R[6]));
  if(R[7] && ac[16]<INF,
    my(rp=ac[16]/ac[m]);
    listput(pc3all,[fam,p,N,fmt(yt),fmt(rp)]);
    if(yt>2, pc3n++; if(rp<1, listput(pc3hit,[fam,p,N,fmt(yt),fmt(rp)])));
    if(Wt[13]>0 && Fs[12+m]>0, my(zx=Wt[13]/Fs[12+m], q=ac[16]/zx); zxn++; if(q>zxmax, zxmax=q); if(Fs[12+m]!=Fs[31], xcross++))))));
}
print("H3 pair-metric (accounting only, NOT an improvement): primary pooled powered cells; cells with mean Y_T(m)>2: ",pc3n,"; of those with Z_pair(m,m)/Z_D(m)<1: ",#pc3hit," -> ",Vec(pc3hit));
print("H3 all powered cells [fam,p,N,mean Y_T(m),Z_pair(m,m)/Z_D(m)]: ",Vec(pc3all));
print("H3 Z_pair(m,m)/Z_X(exact j=m) over ",zxn," powered cells with the coupled arm: max=",fmt(zxmax)," (registered: < 0.2 in every cell); coupled X_m yield != pair yield in ",xcross," cells");
rkv=0; rkn=0; {foreach(famlist,fam, foreach(famPs(fam),p, foreach(curvesof(p,[0,1,2]),c, my(V=mapget(DATA,Str(fam,"_",p,"_",c))); for(i=1,#V, rkn++; rkv+=V[i][10][29]))))}
print("H3 rank check (per target: rank(A-sets) <= Y_D, rank(B-sets) <= Y_T, rank(pair rows) <= rank A + rank B): violating targets summed over ",rkn," curve-cells = ",rkv);
\\ H4: same-term-count control
h4n=0; h4ok=0; h4min=INF; h4max=0; h4r2min=INF; h4r2max=0; h4r2lt1=0; h4r2n=0;
{
foreach(famlist,fam, foreach(famPs(fam),p, foreach(g2N(fam,p),N,
  my(R=mapget(RES,Str(fam,"_",p,"_primary_",N)));
  if(R[7] && R[1][11]>0 && !isinf(R[1][11]), h4n++; if(R[1][11]>1, h4ok++); h4min=min(h4min,R[1][11]); h4max=max(h4max,R[1][11]);
    if(R[1][12]>0 && !isinf(R[1][12]), h4r2n++; if(R[1][12]<1, h4r2lt1++); h4r2min=min(h4r2min,R[1][12]); h4r2max=max(h4r2max,R[1][12]))))));
}
print("H4: powered primary pooled cells with finite Z_D(m2)/Z_D(m): ",h4n," ; Z_D(m2)>Z_D(m) in ",h4ok," ; ratio range [",fmt(h4min),",",fmt(h4max),"] ; second corrected rho PF(m,m)/(Z_D(m2)/Z_D(m)) = Z_PF(m,m)/Z_D(m2) range [",fmt(h4r2min),",",fmt(h4r2max),"], below 1 in ",h4r2lt1," of ",h4r2n," cells (registered: not predicted to cross 1; it compares an m-E-term arm with a 2m-E-term baseline, so it is not the registered baseline)");
\\ H5 falsifier and distance remaining
{
my(nl=0, nlist=List(), best=INF, bestc=0, bestpt=INF, bestptc=0, n=0, nspan=0, minhi=INF, ptlt1=List(), minlo=INF);
foreach(famlist,fam, foreach(famPs(fam),p, foreach(g2N(fam,p),N,
  my(R=mapget(RES,Str(fam,"_",p,"_primary_",N)));
  if(R[7],
    n++;
    for(a=1,4,
      my(lo=R[3][a][1], hi=R[3][a][2], pt=R[1][a]);
      if(hi>=0 && hi<1, listput(nlist,[fam,p,N,a,hi]));
      if(a<=3 && pt>0 && pt<best, best=pt; bestc=[fam,p,N,a]);
      if(lo>=0 && lo<minlo && a<=4, minlo=lo);
      if(pt>0 && pt<1, listput(ptlt1,[fam,p,N,a,fmt(pt)])))))));
print("H5 falsifier: powered primary pooled cells=",n,"; arms 1-4 (PF(m,m), PF(m,1), PF best, X E-sets) with 99% curve-level interval entirely below 1: ",#nlist," -> ",Vec(nlist));
print("H5: arms 1-4 with a POINT estimate below 1 in any powered primary pooled cell: ",#ptlt1," -> ",Vec(ptlt1));
print("H5/distance remaining: lowest decoupled class-F point rho (PF(m,m), PF(m,1), PF best) over powered primary pooled cells = ",fmt(best)," at [fam,p,N,arm]=",bestc,"  distance rho-1 = ",fmt(best-1)," ; lowest 99% interval end over arms 1-4 = ",fmt(minlo));
jset("H5_primary_pooled_powered_cells",n); jset("H5_arms1to4_ci_below_1",#nlist); jset("H5_arms1to4_point_below_1",#ptlt1); jset("H5_lowest_decoupled_point_rho",best); jset("H5_distance_to_1",best-1); jset("H5_lowest_ci_end_arms1to4",minlo);
}
\\ single-curve (target-level) check of the falsifier, and extension / extreme curves
{
my(n=0, nlist=List(), nptl=List());
foreach(famlist,fam, foreach(famPs(fam),p, foreach(concat(curvesof(p,[0]),[9]),c, foreach(g2N(fam,p),N,
  my(sc=Str(if(c==9,"extreme","curve"),c), R=mapget(RES,Str(fam,"_",p,"_",sc,"_",N)));
  if(R[7], n++;
    for(a=1,4, my(hi=R[3][a][2], pt=R[1][a]); if(hi>=0 && hi<1, listput(nlist,[fam,p,c,N,a,fmt(hi)])); if(pt>0 && pt<1, listput(nptl,[fam,p,c,N,a,fmt(pt)]))))))));
print("H5 single curve cells (primary + extreme, target-level 99% interval), powered curve-cells=",n,"; arms 1-4 with interval entirely below 1: ",#nlist," -> ",Vec(nlist),"; arms 1-4 with point estimate <1: ",#nptl," -> ",Vec(nptl));
jset("H5_single_curve_powered_cells",n); jset("H5_single_curve_ci_below_1",#nlist); jset("H5_single_curve_point_below_1",#nptl);
}
{
my(n=0, nlist=List(), nptl=List(), lowest=INF);
foreach(famlist,fam, foreach(famPs(fam),p, foreach(g2N(fam,p),N,
  my(R=mapget(RES,Str(fam,"_",p,"_ext_",N)));
  if(R[7], n++;
    for(a=1,4, my(hi=R[3][a][2], pt=R[1][a]); if(pt>0, lowest=min(lowest,pt)); if(hi>=0 && hi<1, listput(nlist,[fam,p,N,a,fmt(hi)])); if(pt>0 && pt<1, listput(nptl,[fam,p,N,a,fmt(pt)])))))));
print("EXPLORATORY extension pool (c=1..4, 4 curves per p): powered cells=",n,"; arms 1-4 interval entirely below 1: ",#nlist," -> ",Vec(nlist),"; point<1: ",#nptl," -> ",Vec(nptl),"; lowest point rho=",fmt(lowest));
jset("EXT_pool_powered_cells",n); jset("EXT_pool_ci_below_1",#nlist); jset("EXT_pool_point_below_1",#nptl); jset("EXT_pool_lowest_point_rho",lowest);
}
print("\n--- Extreme-twist curves (PC3; single curve, target-level CI) ---");
{
foreach(famlist,fam, foreach(famPs(fam),p, foreach(g2N(fam,p),N,
  my(R=mapget(RES,Str(fam,"_",p,"_extreme9_",N)), cc=cellof(fam,p,9,N), m=g2m(fam), m2=2*m, ac=armcost(R[4],R[5],m,m2,0));
  print("extreme fam=",fam," p=",p," N=",N," NE/NT=",cc[2],"/",cc[3]," #E=",cc[4]," #E'=",cc[5]," D_m=",R[8]," D*=D_",ac[8]," rho PF(m,m)=",fnum(R[1][1])," [",fnum(R[3][1][1]),",",fnum(R[3][1][2]),"] PF(m,1)=",fnum(R[1][2])," PF best=",fnum(R[1][3])," XE=",if(R[5][13]==0,"NA",fnum(R[1][4]))," pair(m,m)=",fnum(R[1][7])," powered=",R[7]))));
}
\\ H5 reproduction of E3 (instrument check)
{
my(E3=read_csv(Str(E3DIR,"out/results.csv")), n=0, nin=0, nd=0, ne=0, rmin=INF, rmax=0);
for(i=1,#E3, my(r=E3[i], fam=if(r[1]==1,"A","B"), p=r[3], c=r[4], N=r[5]);
  my(cc=cellof(fam,p,c,N), F=cc[7], W=cc[8], m=g2m(fam));
  n++;
  if(F[m]!=r[12], nd++); 
  my(sg=sum(j=1,m,F[12+j])); if(sg!=r[13], ne++);
  if(r[12]>0 && sg>0 && type(r[25])=="t_REAL",
    my(mine=(W[13]/sg)/(W[m]/F[m]), q=mine/r[25]);
    rmin=min(rmin,q); rmax=max(rmax,q); if(q>=0.9 && q<=1.1, nin++)));
print("H5 instrument check vs E3 results.csv (same seeds/code): cells compared=",n," ; D_m total differs in ",nd," ; X(j<=m) total differs in ",ne," ; cells with finite rho_gen compared: ratio mine/E3 in [0.9,1.1] for ",nin," ; ratio range [",fmt(rmin),",",fmt(rmax),"]");
jset("E3_repro_cells",n); jset("E3_repro_Dm_diff",nd); jset("E3_repro_Xgen_diff",ne); jset("E3_repro_ratio_in_0.9_1.1",nin); jset("E3_repro_ratio_min",rmin); jset("E3_repro_ratio_max",rmax);
}
\\ PC4 yield model
{
my(n=0, ok=0, bad=List());
foreach(famlist,fam, foreach(famPs(fam),p, foreach(curvesof(p,[0]),c, foreach(g2N(fam,p),N,
  my(cc=cellof(fam,p,c,N), m=g2m(fam), NE=cc[2], l=cc[4], yd=binomial(NE,m)*2^m/l, ym=cc[7][m]/T);
  if(yd>=0.5, n++; my(q=ym/yd); if(q>=0.8 && q<=1.25, ok++, listput(bad,[fam,p,c,N,fmt(q)])))))));
print("PC4 (D_m measured mean yield within [0.8,1.25] of model, primary curve-cells with model Y_D>=0.5): ",ok," of ",n," = ",fmt(100.0*ok/max(1,n)),"% (registered >=80%); outside: ",Vec(bad));
jset("PC4_cells",n); jset("PC4_ok",ok);
}
\\ NC2 B redraw
{
my(n=0, sig=List(), pmin=1, dsame=0);
foreach(famlist,fam, foreach(famPs(fam),p, foreach(curvesof(p,[0,1,2]),c, my(V=mapget(DATA,Str(fam,"_",p,"_",c)));
  for(i=1,#V, my(tr=V[i][10]); n++; if(tr[25]<0.01, listput(sig,[fam,p,c,V[i][1],tr[25]])); pmin=min(pmin,tr[25]); if(tr[26]==1, dsame++)))));
print("NC2 (B redrawn, seed 8000+10p+c; D counts unchanged in ",dsame," of ",n," curve-cells; pair-count permutation test alpha=0.01): curve-cells with p<0.01: ",#sig," of ",n," (chance expectation ",fmt(0.01*n),"); smallest p=",fmt(pmin),"; list ",Vec(sig));
jset("NC2_curve_cells",n); jset("NC2_p_lt_0.01",#sig); jset("NC2_smallest_p",pmin); jset("NC2_Dc_unchanged",dsame);
}
\\ NC1, NC4, audit from lemma.csv
{
my(L=readstr(Str(OUTDIR,"/lemma.csv")), oddv=0, oddn=0, oddF=0, evB=0, evA=0, evN=0, evEven=0, evbad5=0, nc4F=0, nc4S=0, nc4n=0, aoV=0, aoN=0, aoS=0, aoMix=0, aeB=0, aeN=0, aeS=0, aeMix=0, ae2B=0, aeA=0);
for(i=2,#L, if(L[i]=="", next); my(r=strsplit(L[i],","), k=r[1]);
  if(k=="nc1_odd", oddn+=eval(r[6]); oddv+=eval(r[7]); oddF+=eval(r[9]));
  if(k=="nc1_even", evN+=eval(r[6]); evB+=eval(r[7]); evA+=eval(r[8]); evEven+=eval(r[9]); evbad5+=eval(r[10]));
  if(k=="nc4", nc4n++; nc4F+=eval(r[7]); nc4S+=eval(r[8]));
  if(k=="audit_odd", aoN+=eval(r[6]); aoS+=eval(r[7]); aoMix+=eval(r[8]); aoV+=eval(r[9])+eval(r[10])+eval(r[11]));
  if(k=="audit_even", aeN+=eval(r[6]); aeS+=eval(r[7]); aeMix+=eval(r[8]); ae2B+=eval(r[9]); aeB+=eval(r[10]); aeA+=eval(r[11])));
print("NC1 odd-order curves: class-S targets=",oddn," violations (B'!=O or A'!=R)=",oddv," ; class-F projection mismatches=",oddF);
print("NC1 even-order boundary (p=101,211; class-S targets=",evN,"): B'!=O in ",evB," ; A'!=R in ",evA," (targets with R of even order: ",evEven,"; even order but A'==R: ",evbad5,")");
print("Decomposition audit, odd curves (N0=20 4-subsets): decompositions=",aoN," with R in E(Fp)=",aoS," mixed=",aoMix," violations (2B!=O, B!=O, A!=R summed)=",aoV);
print("Decomposition audit, even-order curves: decompositions=",aeN," with R in E(Fp)=",aeS," mixed=",aeMix," violations 2B!=O: ",ae2B," ; B!=O: ",aeB," ; A!=R: ",aeA);
print("NC4 (E-only factor base, 6 primary curves, N=24, 4-sets, actual F_{p^2} group law): hits vs class-F targets (B!=O)=",nc4F," (expected 0); hits vs class-S targets (detector control)=",nc4S);
jset("NC1_odd_targets",oddn); jset("NC1_odd_violations",oddv); jset("NC1_odd_classF_mismatch",oddF); jset("NC1_even_targets",evN); jset("NC1_even_Bp_ne_O",evB); jset("NC1_even_Ap_ne_R",evA); jset("NC1_even_even_order_R",evEven); jset("audit_odd_decompositions",aoN); jset("audit_odd_violations",aoV); jset("audit_even_decompositions",aeN); jset("audit_even_B_ne_O",aeB); jset("NC4_hits_classF",nc4F); jset("NC4_hits_classS_control",nc4S);
}
\\ Wall time / PC5 / NC3
print("\n--- Semaev PC5 and wall time (CPU, shared 4-core machine) ---");
{
my(L=read_csv(Str(OUTDIR,"/semaev_pc5.csv")), tot=0);
for(i=1,#L, my(r=L[i]); my(mm=r[6]+r[7]+r[8]+r[9]+r[10]+r[11]+r[12]); tot+=mm;
  print("PC5 p=",r[1]," N=",r[2]," NE/NT=",r[3],"/",r[4]," set mismatches D2,T2,X4,D4=",[r[6],r[7],r[8],r[9]]," P-F,P-S,proj-point=",[r[10],r[11],r[12]]," relations/10 targets D2,T2,X4,D4=",[r[13],r[14],r[15],r[16]]));
print("PC5 total mismatches over ",#L," cells x 10 targets = ",tot);
jset("PC5_total_mismatches",tot); jset("PC5_cells",#L);
}
{
my(S=readstr(Str(OUTDIR,"/semaev_summary.csv")));
for(i=2,#S, if(S[i]!="", print("WALLSUM ",S[i])));
my(Q=readstr(Str(OUTDIR,"/semaev_ratios.csv")), nps=0, npsok=0, npf=0, npfok=0, nx=0, nxok=0, nnc=0, nncok=0, hwmax=0);
for(i=2,#Q, if(Q[i]=="", next);
  print("WALLRATIO ",Q[i]);
  my(f=strsplit(Q[i],","), nm=f[3]);
  if(f[5]=="NA", next);
  my(med=eval(f[5]), lo=eval(f[10]), hi=eval(f[11]));
  if(nm=="PS/D2", nps++; if(med>1, npsok++));
  if(nm=="PF/D2", npf++; if(med>1, npfok++));
  if(nm=="PF/X4", nx++; if(med<0.1, nxok++));
  if(nm=="D2/D2c", nnc++; if(lo<=1 && 1<=hi, nncok++); hwmax=max(hwmax,(hi-lo)/2)));
print("H6 orderings (median ratio over 7 interleaved repeats): PS/D2>1 in ",npsok,"/",nps," cells; PF/D2>1 in ",npfok,"/",npf," cells; PF/X4<0.1 in ",nxok,"/",nx," cells with X_4 timed");
print("NC3 timer null (D2 vs identical D2c): 99% t-interval (6 d.o.f.) contains 1 in ",nncok,"/",nnc," cells; largest half-width=",fmt(hwmax));
jset("H6_PS_over_D2_gt1_cells",npsok); jset("H6_PS_cells",nps); jset("H6_PF_over_D2_gt1_cells",npfok); jset("H6_PF_cells",npf); jset("H6_PF_over_X4_lt0.1_cells",nxok); jset("H6_X4_cells",nx); jset("NC3_ci_contains_1_cells",nncok); jset("NC3_cells",nnc); jset("NC3_max_halfwidth",hwmax);
}
{
my(S=readstr(Str(OUTDIR,"/semaev_summary.csv")), M=Map(), cells=List());
for(i=2,#S, if(S[i]=="", next); my(f=strsplit(S[i],","), key=Str(f[1],"_",f[2],"_",f[5])); mapput(M,key,[eval(f[7]),eval(f[10]),eval(f[3]),eval(f[4])]); if(f[5]=="D2", listput(cells,[eval(f[1]),eval(f[2])])));
foreach(Vec(cells),cl,
  my(p=cl[1], N=cl[2], d=mapget(M,Str(p,"_",N,"_D2")), pf=mapget(M,Str(p,"_",N,"_PF")), ps=mapget(M,Str(p,"_",N,"_PS")), t2=mapget(M,Str(p,"_",N,"_T2")));
  my(line=Str("H6 per-target wall p=",p," N=",N," NE/NT=",d[3],"/",d[4],": PS/D2=",fmt(ps[1]/d[1])," PF/D2=",fmt(pf[1]/d[1])," (registered about 1+NT/NE plus projection: 1+NT/NE=",fmt(1+d[4]/d[3]),") ; us per root-solve D2=",fmt(d[2])," T2=",fmt(t2[2])," PF=",fmt(pf[2])));
  if(mapisdefined(M,Str(p,"_",N,"_X4")), my(x=mapget(M,Str(p,"_",N,"_X4"))); line=Str(line," X4=",fmt(x[2])," ; X4/D2 per solve=",fmt(x[2]/d[2])," ; PF/X4 per target=",fmt(pf[1]/x[1])));
  print(line));
}
\\ ================================================================ JSON
fj="results.json"; system(Str("rm -f ",fj));
{
write1(fj,"{\"experiment\":\"G2-decoupled-AB-projection-arm\",\"unit\":\"Semaev work units (root-solves x degree) per independent E-relation; wall in ms/us CPU\",\"targets_per_cell\":",T,",\"bootstrap\":{\"curve_level_B\":",BC,",\"curve_seed\":9002,\"target_level_B\":",BT,",\"target_seed\":9001,\"level\":0.99},\"primary_pooled_cells\":[");
my(first=1);
foreach(famlist,fam, foreach(famPs(fam),p, foreach(g2N(fam,p),N,
  my(R=mapget(RES,Str(fam,"_",p,"_primary_",N)), m=g2m(fam), m2=2*m, ac=armcost(R[4],R[5],m,m2,0));
  write1(fj,if(first,"",","),"{\"family\":\"",fam,"\",\"p\":",p,",\"N\":",N,",\"powered\":",R[7],",\"D_m_total\":",R[8],",\"Dstar_j\":",ac[8],
    ",\"rho_PF_mm\":",jn(R[1][1]),",\"rho_PF_mm_lo99\":",jn(R[3][1][1]),",\"rho_PF_mm_hi99\":",jn(R[3][1][2]),
    ",\"rho_PF_m1\":",jn(R[1][2]),",\"rho_PF_m1_lo99\":",jn(R[3][2][1]),",\"rho_PF_m1_hi99\":",jn(R[3][2][2]),
    ",\"rho_PF_best\":",jn(R[1][3]),",\"rho_PF_best_lo99\":",jn(R[3][3][1]),",\"rho_PF_best_hi99\":",jn(R[3][3][2]),",\"PF_best_jk\":",ac[12],
    ",\"rho_X_Esets\":",jn(R[1][4]),",\"rho_X_Esets_lo99\":",jn(R[3][4][1]),",\"rho_X_Esets_hi99\":",jn(R[3][4][2]),
    ",\"rho_pair_mm_accounting\":",jn(R[1][7]),",\"rho_PS_best\":",jn(R[1][8]),",\"ZDm2_over_ZDm\":",jn(R[1][11]),"}");
  first=0)));
write1(fj,"],\"summary\":{");
jset("PC1_cells",npc1); jset("PC1_ok",pc1ok); jset("PC1_ci_ok",pc1ci); jset("H1_curve_cells",h1n); jset("H1_sum_diff",h1bad); jset("H1_target_diff",h1tbad); jset("H1_set_diff",h1setbad); jset("H1_pf_diff",h1pfbad); jset("H1_rho_ne_1",h1rhobad);
jset("H2_arms_tested",h2n); jset("H2_max_rel_dev",h2maxdev); jset("H2_dev_gt_1e-9",h2dev); jset("H2_rho_le_1",h2le1);
jset("H3_cells_YT_gt_2",pc3n); jset("H3_cells_pair_lt_1",#pc3hit); jset("H3_rank_violations",rkv); jset("H3_Zpair_over_ZX_max",zxmax);
jset("H4_cells",h4n); jset("H4_ok",h4ok); jset("H4_ZDm2_over_ZDm_min",h4min); jset("H4_ZDm2_over_ZDm_max",h4max); jset("H4_second_rho_min",h4r2min); jset("H4_second_rho_max",h4r2max); jset("H4_second_rho_lt1_cells",h4r2lt1); jset("H4_second_rho_cells",h4r2n);
write1(fj,strjoin(Vec(JL),","),"}}\n");
}
quit;
