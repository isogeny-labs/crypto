\\ e3_analyze.gp -- bootstrap, ratios, controls, falsifier tests. Reads out/*.csv only. Writes out/results.csv, out/results_summary.csv and results.json
default(parisizemax,2000000000);
read("e3_grid.gp");
NBOOT=2000; INF=10^18;
fr=Str(OUTDIR,"/results.csv"); system(Str("rm -f ",fr));
write(fr,"fam,m,p,c,N,NE,NT,T,l,lp,powered,Dtot,Xgen_tot,Xex_tot,Xall_tot,Xmc_tot,WD,WX,WXm,ZD,ZXgen,ZXex,ZXall,ZXmc,rho_gen,rho_gen_lo99,rho_gen_hi99,rho_gen_is_lb,rho_ex,rho_ex_lo99,rho_ex_hi99,rho_ex_is_lb,rho_all,rho_Xmc,rho_Xmc_is_lb,boot_valid,boot_Xgen_zero,rho_model_gen,rho_model_ex,gen_over_model,within3,YD_model,YD_meas,pc3_ratio,pc3_in125,meanj_over_m2,NE_over_N,p4_cell_ok,Zpair_accounting_only,nc2_p,p5,nc1,fm,nc3m,wcount_ok,mult");
ratio_(a,b)=if(b==0, INF, a/b);      \\ INF as sentinel for zero denominators
isinf(x)=(x>=INF/10);
fnum(x)=if(isinf(x),"inf",fmt(x));
jn(x)=if(isinf(x),"null",fmt(x));
cells=List(); 
{
for(fi=1,2,
  my(fam=if(fi==1,"A","B"), m=famM(fam));
  foreach(famP(fam),p, for(c=1,2,
    my(tab=read_csv(Str(OUTDIR,"/tables_p",p,"_c",c,"_",fam,".csv")));
    foreach(famN(fam),N,
      my(row=0, mr=0);
      for(i=1,#tab, if(tab[i][4]==N, row=tab[i]));
      my(MD=read_csv(Str(OUTDIR,"/model.csv")));
      for(i=1,#MD, if(MD[i][1]==p && MD[i][2]==c && MD[i][3]==fi && MD[i][5]==N, mr=MD[i]));
      if(row==0 || mr==0, error("missing row ",p," ",c," ",N));
      my(pt=read_csv(Str(OUTDIR,"/pt_p",p,"_c",c,"_",fam,"_N",N,".csv")), T=#pt);
      my(Dv=vector(T,i,pt[i][4]), Gv=vector(T,i,pt[i][5]), Ev=vector(T,i,pt[i][6]), Av=vector(T,i,pt[i][7]), Mv=vector(T,i,pt[i][8]));
      my(NE=row[8], NT=row[9], l=row[6], lp=row[7]);
      my(WD=row[30], WX=row[32], WXm=row[34]);
      my(wok=(row[30]==row[31] && row[32]==row[33] && row[34]==row[35] && WD==mr[13] && WX==mr[14]));
      my(sD=vecsum(Dv), sG=vecsum(Gv), sE=vecsum(Ev), sA=vecsum(Av), sM=vecsum(Mv));
      my(powered=(sD>=20));
      my(ZD=if(sD>0,T*WD/sD,INF), ZG=if(sG>0,T*WX/sG,INF), ZE=if(sE>0,T*WX/sE,INF), ZA=if(sA>0,T*WX/sA,INF), ZM=if(sM>0,T*WXm/sM,INF));
      \\ ratios (infinite arm -> rule-of-three lower bound: total yield <= 3)
      my(rg, rg_lb=0, re, re_lb=0, rm, rm_lb=0, ra);
      if(sD==0, rg=-1; re=-1; ra=-1; rm=-1,
        rg=if(sG>0, ZG/ZD, (T*WX/3)/ZD); rg_lb=(sG==0);
        re=if(sE>0, ZE/ZD, (T*WX/3)/ZD); re_lb=(sE==0);
        ra=if(sA>0, ZA/ZD, (T*WX/3)/ZD);
        rm=if(sM>0, ZM/ZD, (T*WXm/3)/ZD); rm_lb=(sM==0));
      \\ paired bootstrap over targets (seed 9001 per cell), 99% percentile interval
      setrand(9001);
      my(bg=List(), be=List(), nvalid=0, gzero=0);
      for(b=1,NBOOT,
        my(d=0, g=0, e=0);
        for(i=1,T, my(k=1+random(T)); d+=Dv[k]; g+=Gv[k]; e+=Ev[k]);
        if(d==0, next);
        nvalid++;
        if(g==0, gzero++);
        listput(bg, if(g>0, (WX/WD)*d/g, INF));
        listput(be, if(e>0, (WX/WD)*d/e, INF)));
      my(glo=-1, ghi=-1, elo=-1, ehi=-1);
      if(nvalid>=100,
        my(sg=vecsort(Vec(bg)), se=vecsort(Vec(be)), il=max(1,round(0.005*nvalid)), ih=min(nvalid,round(0.995*nvalid)));
        glo=sg[il]; ghi=sg[ih]; elo=se[il]; ehi=se[ih]);
      my(rmod=mr[18], rmodx=mr[19]);
      my(gom=if(rg>0 && rmod>0, rg/rmod, -1), w3=if(gom>0, (gom>=1/3 && gom<=3), -1));
      my(YD=mr[10], YDm=sD/T, pc3r=if(YD>0, YDm/YD, -1), pc3ok=if(YD>=0.5, (pc3r>=0.8 && pc3r<=1.25), -1));
      my(mj=if(sA>0, sum(j=1,5,j*row[15+j])/(2*m*sA), -1), neN=NE/N, p4ok=if(sA>0, (abs(mj-neN)<=0.15), -1));
      my(Zpair=ZA);
      write(fr,fi,",",m,",",p,",",c,",",N,",",NE,",",NT,",",T,",",l,",",lp,",",powered,",",sD,",",sG,",",sE,",",sA,",",sM,",",WD,",",WX,",",WXm,",",fnum(ZD),",",fnum(ZG),",",fnum(ZE),",",fnum(ZA),",",fnum(ZM),",",
        fnum(rg),",",fnum(glo),",",fnum(ghi),",",rg_lb,",",fnum(re),",",fnum(elo),",",fnum(ehi),",",re_lb,",",fnum(ra),",",fnum(rm),",",rm_lb,",",nvalid,",",gzero,",",fmt(rmod),",",fmt(rmodx),",",fnum(gom),",",w3,",",fmt(YD),",",fmt(YDm),",",fmt(pc3r),",",pc3ok,",",fmt(mj),",",fmt(neN),",",p4ok,",",fnum(Zpair),",",fmt(row[36]),",",row[25],",",row[26],",",row[27],",",row[28],",",wok,",",row[24]);
      listput(cells,[fi,m,p,c,N,NE,NT,T,powered,sD,sG,sE,sA,sM,rg,glo,ghi,rg_lb,re,elo,ehi,re_lb,ra,rm,rm_lb,nvalid,gzero,rmod,rmodx,gom,w3,YD,YDm,pc3r,pc3ok,mj,neN,p4ok,row[36],row[25],row[26],row[27],row[28],wok,ZD,ZG,ZE,row[24]]);
    ))));
}
cells=Vec(cells);
\\ indices into the cell record
iFI=1;iM=2;iP=3;iC=4;iN=5;iNE=6;iNT=7;iT=8;iPOW=9;iD=10;iG=11;iE=12;iA=13;iM_=14;iRG=15;iGLO=16;iGHI=17;iGLB=18;iRE=19;iELO=20;iEHI=21;iELB=22;iRA=23;iRM=24;iRMLB=25;iNV=26;iGZ=27;iRMOD=28;iRMODX=29;iGOM=30;iW3=31;iYD=32;iYDM=33;iPC3R=34;iPC3OK=35;iMJ=36;iNEN=37;iP4=38;iNC2P=39;iP5=40;iNC1=41;iFM=42;iNC3=43;iWOK=44;iZD=45;iZG=46;iZE=47;iMULT=48;
nc=#cells;
print("=== E3 analysis ===  cells=",nc);
\\ ---------------------------------------------------------------- primary table
print("\n--- per cell (fam,p,c,N | NE NT | powered | Dtot Xgen Xex Xall | rho_gen [99% CI] (lb?) | rho_exact | rho_model_gen | gen/model | mean j/m2 vs NE/N) ---");
{
for(i=1,nc, my(r=cells[i]);
  print(if(r[iFI]==1,"A","B")," p=",r[iP]," c=",r[iC]," N=",r[iN]," | ",r[iNE]," ",r[iNT]," | pow=",r[iPOW]," | ",r[iD]," ",r[iG]," ",r[iE]," ",r[iA]," | rho_gen=",fnum(r[iRG])," [",fnum(r[iGLO]),", ",fnum(r[iGHI]),"]",if(r[iGLB]," LB",""), " | rho_ex=",fnum(r[iRE])," [",fnum(r[iELO]),", ",fnum(r[iEHI]),"]",if(r[iELB]," LB","")," | rho_model=",fmt(r[iRMOD])," | gen/model=",fnum(r[iGOM])," | j/m2=",fmt(r[iMJ])," NE/N=",fmt(r[iNEN])));
}
\\ ---------------------------------------------------------------- controls
print("\n--- Controls ---");
\\ PC1 and lemma audit from lemma.csv
LM=List(); {my(L=readstr(Str(OUTDIR,"/lemma.csv"))); for(i=2,#L, if(L[i]=="", next); listput(LM,strsplit(L[i],",")))}
LM=Vec(LM);
pc1n=0; pc1bad=0;
{
for(i=1,#LM, my(r=LM[i]);
  if(r[1]=="pc1", pc1n++; my(bad=(eval(r[6])!=1)+eval(r[7])+eval(r[8])+eval(r[9])+eval(r[10])); pc1bad+=bad; print("PC1 p=",r[2]," c=",r[3]," mismatches (card + gen order + pi + pair-sum + x-coord) = ",bad," of ",eval(r[11])," pairs")));
}
print("PC1 total curves=",pc1n," total mismatches=",pc1bad);
oddv=0; oddn=0; oddmix=0; oddS=0; evenv=0; evenn=0; evenS=0; evenmix=0; evenviolB=List();
{
for(i=1,#LM, my(r=LM[i]);
  if(r[1]=="audit_odd", oddn+=eval(r[6]); oddS+=eval(r[7]); oddmix+=eval(r[8]); oddv+=eval(r[9])+eval(r[10])+eval(r[11]));
  if(r[1]=="audit_even", evenn+=eval(r[6]); evenS+=eval(r[7]); evenmix+=eval(r[8]); evenv+=eval(r[10]); listput(evenviolB,[eval(r[2]),eval(r[6]),eval(r[10])])));
}
print("H4 group-law audit, odd curves: #decompositions=",oddn," with R in E(Fp)=",oddS," of which mixed=",oddmix," violations (2B!=O, B!=O, A!=R summed)=",oddv);
print("H4 group-law audit, even-order curves: #decompositions=",evenn," with R in E(Fp)=",evenS," mixed=",evenmix," violations B!=O=",evenv," per curve [p,#dec,#B!=O]=",Vec(evenviolB));
nc1t=sum(i=1,nc,cells[i][iNC1]); p5t=sum(i=1,nc,cells[i][iP5]); fmt_t=sum(i=1,nc,cells[i][iFM]); nc3t=sum(i=1,nc,cells[i][iNC3]); wbad=sum(i=1,nc,1-cells[i][iWOK]); multt=sum(i=1,nc,cells[i][iMULT]);
print("NC1 table version (X hits with B=0 target whose E-subset is not a direct D-relation), total exceptions=",nc1t);
print("P5 (E-parts of X relations that are not direct relations), total exceptions=",p5t);
print("factorisation cross-check X=D_j*T_k mismatches=",fmt_t,"   work-counter vs formula mismatching cells=",wbad,"   sign-vector multiplicities (total)=",multt);
print("NC3 (E-only factor base, B!=0) hits=",nc3t);
nc2sig=List(); {for(i=1,nc, if(cells[i][iNC2P]<0.01, listput(nc2sig,[cells[i][iP],cells[i][iC],cells[i][iN],cells[i][iM],cells[i][iNC2P]])))}
print("NC2 (B-independence, two-sample permutation test per cell, alpha=0.01): cells with p<0.01: ",#nc2sig," of ",nc," (expected by chance at alpha=0.01: ",fmt(nc*0.01),")  list [p,c,N,m,pval]=",Vec(nc2sig));
print("NC2 smallest p-value over cells=",fmt(vecmin(vector(nc,i,cells[i][iNC2P]))));
\\ PC3
pc3n=0; pc3ok=0; {for(i=1,nc, if(cells[i][iYD]>=0.5, pc3n++; if(cells[i][iPC3OK]==1, pc3ok++)))}
print("PC3 (D_m measured mean yield within factor 1.25 of model, cells with model Y_D>=0.5): ",pc3ok," of ",pc3n," = ",fmt(100.0*pc3ok/max(1,pc3n)),"%");
\\ PC2 / P7 Semaev
SM=read_csv(Str(OUTDIR,"/semaev_summary.csv"));
pc2bad=0; p7ok=1;
{
for(i=1,#SM, my(r=SM[i]);
  pc2bad+=r[6]+r[7]+r[8]+r[9];
  if(r[16]<4, p7ok=0);
  print("SEMAEV p=",r[1]," N=",r[2]," count mismatches D/X=",r[6],"/",r[7]," set mismatches D/X=",r[8],"/",r[9]," us per solve D=",r[14]," X=",r[15]," ratio X/D=",r[16]," relations D=",r[19]," X=",r[20]));
}
print("PC2 total mismatches (D counts, X counts, D sets, X sets) over ",#SM," cells x 10 targets = ",pc2bad);
\\ ---------------------------------------------------------------- predictions
print("\n--- Predictions ---");
npow=0; p1pt=0; p1ci=0; h1fals=List(); p1fail=List(); p2n=0; p2ok=0; p3n=0; p3ok=0;
{
for(i=1,nc, my(r=cells[i]);
  if(r[iPOW]==1,
    npow++;
    if(r[iRG]>=2, p1pt++, listput(p1fail,[r[iP],r[iC],r[iN],r[iM],r[iRG]]));
    if(r[iGLO]>=2, p1ci++);
    if(r[iGHI]>=0 && r[iGHI]<1, listput(h1fals,[r[iP],r[iC],r[iN],r[iM],r[iGHI]]));
    if(r[iW3]>=0, p2n++; if(r[iW3]==1, p2ok++));
    if(r[iG]>0 && r[iE]>0, p3n++; if(r[iRE]>=r[iRG], p3ok++))));
}
print("powered cells (D total yield >= 20): ",npow," of ",nc);
print("P1: powered cells with point rho_gen>=2: ",p1pt," of ",npow,"   with 99% CI lower bound >=2: ",p1ci,"   cells with rho_gen<2: ",Vec(p1fail));
print("H1 falsifier (99% CI upper bound <1 in some powered cell): ",#h1fals," cells: ",Vec(h1fals));
nspan=0; {for(i=1,nc, my(r=cells[i]); if(r[iPOW]==1 && r[iGLO]>=0 && r[iGLO]<1 && r[iGHI]>=1, nspan++))}
print("powered cells whose rho_gen CI spans 1 (weak support): ",nspan);
print("P2: powered cells with rho_gen within factor 3 of rho_model: ",p2ok," of ",p2n," = ",fmt(100.0*p2ok/max(1,p2n)),"% (registered threshold 80%)");
print("P3: cells (powered) with both finite and rho_exact>=rho_gen: ",p3ok," of ",p3n);
\\ P4 pooled
num=0; den=0; num_p=0; den_p=0; p4cellok=0; p4celln=0;
{
for(i=1,nc, my(r=cells[i], row=0);
  my(tab=read_csv(Str(OUTDIR,"/tables_p",r[iP],"_c",r[iC],"_",if(r[iFI]==1,"A","B"),".csv")));
  for(k=1,#tab, if(tab[k][4]==r[iN], row=tab[k]));
  my(m2=2*r[iM]);
  my(s1=sum(j=1,5,j*row[15+j]), s0=sum(j=1,5,row[15+j]));
  num+=s1/m2; den+=s0;
  if(r[iPOW]==1, num_p+=s1/m2; den_p+=s0);
  if(r[iP4]>=0, p4celln++; if(r[iP4]==1, p4cellok++)));
}
print("P4/H2: pooled mean j/m2 over all X relations (all cells) = ",fmt(num/den)," (n relations=",den,");  powered cells only = ",fmt(num_p/den_p)," (n=",den_p,")  registered band [0.35,0.65]");
print("P4 per cell: |mean j/m2 - NE/N| <= 0.15 in ",p4cellok," of ",p4celln," cells with relations");
print("P5: exceptions=",p5t,"    P6 odd: violations=",oddv,"; P6 even: violations (B!=O) = ",evenv," of ",evenn," decompositions (",evenS," with R in E(Fp))");
print("P7: PC2 mismatches=",pc2bad,"; X_4/D_2 per-solve time ratio >=4 in all Semaev cells: ",p7ok);
\\ ---------------------------------------------------------------- secondary
print("\n--- Secondary: best-N (min Z over the N grid) per curve and family; joint-pair cost (ACCOUNTING ONLY, relabelling, not an improvement) ---");
{
for(fi=1,2, foreach(famP(if(fi==1,"A","B")),p, for(c=1,2,
  my(bD=INF, bG=INF, bE=INF, nD=0, nG=0);
  for(i=1,nc, my(r=cells[i]); if(r[iFI]==fi && r[iP]==p && r[iC]==c,
    if(r[iZD]<bD, bD=r[iZD]; nD=r[iN]); if(r[iZG]<bG, bG=r[iZG]; nG=r[iN]); if(r[iZE]<bE, bE=r[iZE])));
  print(if(fi==1,"A","B")," p=",p," c=",c," best Z_D=",fnum(bD)," (N=",nD,")  best Z_Xgen=",fnum(bG)," (N=",nG,")  best Z_Xexact=",fnum(bE),"  ratio best-gen/best-D=",if(bD<INF&&bG<INF,fmt(bG/bD),"na")))));
}
print("\nmean sign-vector multiplicities total=",multt);
\\ ---------------------------------------------------------------- JSON
fj=if(OUTDIR=="out","results.json",Str(OUTDIR,"/results.json")); system(Str("rm -f ",fj));
{
write1(fj,"{\"experiment\":\"E3-eigen-decomposition-Fq2\",\"cells\":[");
for(i=1,nc, my(r=cells[i]);
  write1(fj,if(i>1,",",""),"{\"family\":\"",if(r[iFI]==1,"A","B"),"\",\"m\":",r[iM],",\"p\":",r[iP],",\"curve\":",r[iC],",\"N\":",r[iN],",\"NE\":",r[iNE],",\"NT\":",r[iNT],",\"targets\":",r[iT],",\"powered\":",r[iPOW],
    ",\"D_total\":",r[iD],",\"Xgen_total\":",r[iG],",\"Xexact_total\":",r[iE],",\"Xall_total\":",r[iA],",\"Xm_control_total\":",r[iM_],
    ",\"rho_gen\":",jn(r[iRG]),",\"rho_gen_lo99\":",jn(r[iGLO]),",\"rho_gen_hi99\":",jn(r[iGHI]),",\"rho_gen_is_lower_bound\":",r[iGLB],
    ",\"rho_exact\":",jn(r[iRE]),",\"rho_exact_lo99\":",jn(r[iELO]),",\"rho_exact_hi99\":",jn(r[iEHI]),",\"rho_exact_is_lower_bound\":",r[iELB],
    ",\"rho_model_gen\":",jn(r[iRMOD]),",\"rho_model_exact\":",jn(r[iRMODX]),",\"mean_j_over_m2\":",jn(r[iMJ]),",\"NE_over_N\":",fmt(r[iNEN]),",\"nc2_pvalue\":",jn(r[iNC2P]),"}"));
write1(fj,"],\"summary\":{\"cells\":",nc,",\"powered_cells\":",npow,",\"P1_powered_cells_rho_gen_ge_2\":",p1pt,",\"P1_powered_cells_ci_lo_ge_2\":",p1ci,",\"H1_falsifying_cells\":",#h1fals,",\"powered_cells_ci_spans_1\":",nspan,
  ",\"P2_within3\":",p2ok,",\"P2_n\":",p2n,",\"P3_ok\":",p3ok,",\"P3_n\":",p3n,",\"P4_pooled_mean_j_over_m2_all\":",fmt(num/den),",\"P4_relations\":",den,",\"P4_pooled_powered\":",fmt(num_p/den_p),",\"P4_cells_within_0.15\":",p4cellok,",\"P4_cells_n\":",p4celln,
  ",\"P5_exceptions\":",p5t,",\"P6_odd_violations\":",oddv,",\"P6_odd_decompositions\":",oddn,",\"P6_even_violations\":",evenv,",\"P6_even_decompositions\":",evenn,",\"P7_PC2_mismatches\":",pc2bad,",\"P7_ratio_ge_4_all_cells\":",p7ok,
  ",\"PC1_mismatches\":",pc1bad,",\"PC3_ok\":",pc3ok,",\"PC3_n\":",pc3n,",\"NC1_exceptions\":",nc1t,",\"NC3_hits\":",nc3t,",\"NC2_cells_p_lt_0.01\":",#nc2sig,"}}\n");
}
quit;
