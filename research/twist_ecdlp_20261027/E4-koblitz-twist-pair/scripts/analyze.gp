default(parisizemax,2000000000);
\\ analyze.gp: aggregates out/*.csv and out/stage0*.txt, bootstrap (setrand(20261021), 2000 resamples),
\\ control verdicts; writes out/verdict.txt (stdout) and results.json. Reads recorded files only.
setrand(20261021);
MS=[11,13,17,19];
LR(m)=if(m==11,[4,9],m==13,[5,10],m==17,[6,10],[7,10]);
KD(l,pop)=if(pop==2,1,if(l<=8,20,if(l==9,8,4)));
\\ ---- helpers ----
loadcsv(f) = { my(L,names=[],rows=List(),s);
  iferr(L=readstr(f), e, return([[],[]]));
  for(i=1,#L, s=L[i]; if(#s==0,next);
    if(Vec(s)[1]=="#", if(#s>10 && substr(s,1,10)=="# fields: " && #names==0, names=strsplit(substr(s,11),",")); next);
    listput(rows,eval(Str("[",s,"]"))));
  [names,Vec(rows)] }
substr(s,a,b=0) = my(v=Vec(s)); if(b<=0||b>#v,b=#v); concat(vector(b-a+1,i,v[a+i-1]));
ci(names,nm) = { for(i=1,#names, if(names[i]==nm, return(i))); error("no column ",nm) }
sel(rows,l,pop) = { my(L=List()); for(i=1,#rows, if(rows[i][2]==l && rows[i][3]==pop, listput(L,rows[i]))); Vec(L) }
colv(rows,j) = vector(#rows,i,rows[i][j]);
vsum(v) = my(s=0); for(i=1,#v,s+=v[i]); s;
med(v) = my(w=vecsort(v),n=#w); if(n==0,return(0)); if(n%2,w[(n+1)/2],(w[n/2]+w[n/2+1])/2);
bootci(v) = { my(n=#v,B); if(n==0,return([0.,0.,0.])); if(n==1,return([v[1]*1.,v[1]*1.,v[1]*1.])); B=vector(2000,b,med(vector(n,i,v[random(n)+1]))); B=vecsort(B); [med(v)*1.,B[50]*1.,B[1951]*1.] }
fm(x) = Strprintf("%.4f",x);
jn(x) = if(type(x)=="t_INT",Str(x),Strprintf("%.6f",x));
jci(c) = Str("{\"median\":",jn(c[1]),",\"ci95_lo\":",jn(c[2]),",\"ci95_hi\":",jn(c[3]),"}");
VER=List(); say(s) = { print(s); listput(VER,s) }
JS=List();

\\ ---- Stage 0 / 0b gates ----
GATEFAIL=0; INVALID=List();
{ for(i=1,#MS, my(m=MS[i],L,np=0,nf=0,done=0);
  iferr(L=readstr(Str("out/stage0_m",m,".txt")), e, L=[]);
  for(k=1,#L, my(w=strsplit(L[k]," ")); if(w[1]=="CHECK", if(w[4]=="PASS",np++, nf++)); if(#L[k]>=14 && substr(L[k],1,14)=="# stage0 done ", done=1));
  say(Str("STAGE0 m=",m," checks_pass=",np," checks_fail=",nf," complete=",done));
  if(nf>0 || done==0 || np!=16, GATEFAIL++; listput(INVALID,Str("stage0 m=",m)));
  listput(JS,Str("\"stage0_m",m,"\":{\"pass\":",np,",\"fail\":",nf,",\"complete\":",done,"}")));
}
PC3FAIL=0; F2FAIL=0;
{ for(i=1,3, my(m=MS[i],L,np=0,nf=0,done=0,pc3f=0,f2f=0);
  iferr(L=readstr(Str("out/stage0b_m",m,".txt")), e, L=[]);
  for(k=1,#L, my(w=strsplit(L[k]," ")); if(w[1]=="CHECK", if(w[4]=="PASS",np++, nf++; if(Vec(w[2])[1]=="P",pc3f++); if(Vec(w[2])[1]=="F",f2f++))); if(#L[k]>=14 && substr(L[k],1,14)=="# stage0b done",done=1));
  say(Str("STAGE0B m=",m," checks_pass=",np," checks_fail=",nf," complete=",done," PC3_fail=",pc3f," F2_fail=",f2f));
  PC3FAIL+=pc3f; F2FAIL+=f2f; if(nf>0 && pc3f+f2f==0, GATEFAIL++; listput(INVALID,Str("stage0b gate m=",m)));
  if(done==0 || np+nf!=7, GATEFAIL++; listput(INVALID,Str("stage0b incomplete m=",m)));
  listput(JS,Str("\"stage0b_m",m,"\":{\"pass\":",np,",\"fail\":",nf,",\"PC3_fail\":",pc3f,",\"F2_fail\":",f2f,",\"complete\":",done,"}")));
}

\\ ---- Stage 1 / Stage 2 ----
T1COLS=0; T2COLS=0;
NC1R=List(); NC1Rbig=List();
F1=0; F3cost=0; F3wall=0; F4=0; PC1FAIL=0; PC2FAIL=0; MISSING=List(); CROSS1=0; CROSS2=0; NC3BAD=0; NC1BAD=0; ROOTFAIL=0; TGTBAD=0; ROWSBAD=0; T1USED=0; VIOL=0;
CELLS=List(); P3COST_OUT=0; P3WALL_OUT=0; P3N=0; PC1CELLS=List();
S2all=List(); S1all=List();
{ for(i=1,#MS, my(m=MS[i],r1,r2,n1,n2,lr=LR(m));
  r1=loadcsv(Str("out/stage1_m",m,".csv")); r2=loadcsv(Str("out/stage2_m",m,".csv")); n1=r1[1]; n2=r2[1];
  if(#n1==0||#n2==0, listput(MISSING,Str("m=",m," csv missing")); next);
  my(pc2r=0,pc2d=0);
  for(l=lr[1],lr[2], for(pop=1,2,
    my(s1=sel(r1[2],l,pop),s2=sel(r2[2],l,pop),kd=KD(l,pop),nd1=#s1,nd2=#s2,tag=if(pop==1,"rand","poly"),js);
    if(nd1<kd, listput(MISSING,Str("stage1 m=",m," l=",l," ",tag," draws ",nd1,"/",kd)));
    if(nd2<kd, listput(MISSING,Str("stage2 m=",m," l=",l," ",tag," draws ",nd2,"/",kd)));
    my(rc=vsum(colv(s1,ci(n1,"rat_cross"))),vi=vsum(colv(s1,ci(n1,"viol"))),f4=#s1-vsum(colv(s1,ci(n1,"f4_ok"))),rf=vsum(colv(s1,ci(n1,"rootfail"))),
       p2r=vsum(colv(s1,ci(n1,"pc2_rat"))),p2d=vsum(colv(s1,ci(n1,"pc2_diff"))),trip=vsum(colv(s1,ci(n1,"triples"))));
    CROSS1+=rc; VIOL+=vi; F4+=f4; ROOTFAIL+=rf; if(pop==1, pc2r+=p2r; pc2d+=p2d);
    say(Str("STAGE1 m=",m," l=",l," pop=",tag," draws=",nd1," cross_rational_pairs=",rc," violations=",vi," F4_formula_fail_draws=",f4," rootfail=",rf," triples_total=",trip," pc2_diff/rat=",p2d,"/",p2r));
    if(nd2==0, next);
    my(Er=vsum(colv(s2,ci(n2,"E_rel0"))),Sk=vsum(colv(s2,ci(n2,"S_k0pure0"))),Sc=vsum(colv(s2,ci(n2,"S_cross0"))),S1p=vsum(colv(s2,ci(n2,"S_k1pure0"))),
       mi=vsum(colv(s2,ci(n2,"mism"))),mis=vsum(colv(s2,ci(n2,"mism_sh"))),E1=vsum(colv(s2,ci(n2,"E_rel1"))),M5=vsum(colv(s2,ci(n2,"S_k1pure1"))),
       x1c=vsum(colv(s2,ci(n2,"S_cross1")))+vsum(colv(s2,ci(n2,"S_k0pure1"))),rq=#s2-vsum(colv(s2,ci(n2,"rows_eq"))),t1u=vsum(colv(s2,ci(n2,"cls_S_t1_used"))),
       tb=vsum(colv(s2,ci(n2,"tgt_bad"))),Esol=vsum(colv(s2,ci(n2,"E_solves0"))),Ssol=vsum(colv(s2,ci(n2,"S_solves0"))));
    my(dr=vector(nd2,i,s2[i][ci(n2,"S_k0pure0")]-s2[i][ci(n2,"E_rel0")]),ndr=0); for(i=1,nd2,if(dr[i]!=0,ndr++));
    F1+=(mi>0 || Sk!=Er || ndr>0 || mis>0); CROSS2+=Sc; NC3BAD+=E1; NC1BAD+=mis; TGTBAD+=tb; ROWSBAD+=rq; T1USED+=t1u;
    if(mi>0 || Sk!=Er || ndr>0, say(Str("F1_TRIGGER m=",m," l=",l," ",tag," mism=",mi," Delta=",Sk-Er)));
    \\ per-draw ratios
    my(cr=List(),cpr=List(),wr=List(),nc1=List(),zero=0,lowcost=0);
    for(d=1,nd2, my(row=s2[d],es=row[ci(n2,"E_solves0")],ss=row[ci(n2,"S_solves0")],er=row[ci(n2,"E_rel0")],sr=row[ci(n2,"S_k0pure0")]);
      listput(cr,ss/es);
      if(er>0 && sr>0, listput(cpr,(ss/sr)/(es/er)); if((ss/sr)<(es/er), lowcost++), zero++);
      my(rr=List(),nn=List());
      for(rep=1,3, my(tE=row[ci(n2,"tE1")+4*(rep-1)],tS=row[ci(n2,"tS1")+4*(rep-1)],ta=row[ci(n2,"tEa1")+4*(rep-1)],ts=row[ci(n2,"tEs1")+4*(rep-1)]);
        if(tE>0, listput(rr,tS/tE));
        if(ta>0, listput(NC1R,ts/ta); listput(nn,ts/ta); if(ta>=100, listput(NC1Rbig,ts/ta))));
      if(#rr>0, listput(wr,med(Vec(rr)))); if(#nn>0, listput(nc1,med(Vec(nn)))));
    F3cost+=lowcost;
    my(cc=bootci(Vec(cr)),cpc=bootci(Vec(cpr)),wc=bootci(Vec(wr)));
    my(pc1s="NA",pc1jr="null",pc1jc="null");
    if(l<=9,
      my(P_rel=vsum(colv(s2,ci(n2,"P_rel0"))),P_sol=vsum(colv(s2,ci(n2,"P_solves0"))),rr1=P_rel/Er,cc1=(P_sol/P_rel)/(Esol/Er));
      my(pass=(P_rel>0 && Er>0 && rr1>=3.0 && rr1<=5.0 && cc1>=0.35 && cc1<=0.70));
      if(pop==1 && !pass, PC1FAIL++);
      pc1s=Str("rel_ratio=",fm(rr1)," cost_per_rel_ratio=",fm(cc1)," pass=",pass," [rand-gate=",pop==1,"] P_rel=",P_rel," E_rel=",Er);
      pc1jr=jn(rr1*1.); pc1jc=jn(cc1*1.);
      listput(PC1CELLS,[m,l,pop,rr1,cc1,pass]));
    say(Str("STAGE2 m=",m," l=",l," pop=",tag," draws=",nd2," N0V_mean=",fm(vsum(colv(s2,ci(n2,"N0V")))/nd2)," E_rel=",Er," S_k0pure=",Sk," Delta_rel=",Sk-Er," draws_Delta!=0=",ndr," mismatching_targets=",mi," S_cross=",Sc," S_k1pure_on_K0=",S1p," NC1_mism=",mis," NC3_E_rel_on_K1targets=",E1," M5_S_k1pure_on_K1targets=",M5," rows_neq_draws=",rq));
    say(Str("   COST rho_C median=",fm(cc[1])," [",fm(cc[2]),",",fm(cc[3]),"]  cost/rel ratio median=",fm(cpc[1])," [",fm(cpc[2]),",",fm(cpc[3]),"] (draws with 0 relations skipped=",zero,")  WALL S/E median=",fm(wc[1])," [",fm(wc[2]),",",fm(wc[3]),"]  draws S cheaper per relation=",lowcost));
    say(Str("   PC1 ",pc1s));
    if(l>=6 && pop==1, P3N++; if(cc[1]<1.6||cc[1]>2.5,P3COST_OUT++); if(wc[1]<1.3||wc[1]>2.3,P3WALL_OUT++));
    listput(CELLS,[m,l,pop,wc]);
    listput(JS,Str("\"cell_m",m,"_l",l,"_",tag,"\":{\"draws\":",nd2,",\"draws_expected\":",kd,",\"Delta_rel_total\":",Sk-Er,",\"E_rel_total\":",Er,",\"S_k0pure_total\":",Sk,",\"mismatching_targets\":",mi,",\"cross_rel_stage2\":",Sc,",\"cross_rational_pairs_stage1\":",rc,",\"cost_ratio_solves\":",jci(cc),",\"cost_per_rel_ratio\":",jci(cpc),",\"wall_ratio_S_over_E\":",jci(wc),",\"pc1_rel_ratio\":",pc1jr,",\"pc1_cost_per_rel_ratio\":",pc1jc,",\"M5_k1pure_rel_on_K1_targets\":",M5,",\"E_rel_on_K1_targets\":",E1,"}"));
  ));
  my(f=if(pc2r>0,pc2d/pc2r,-1)*1.); my(ok=(f>=0.35&&f<=0.65));
  if(!ok, PC2FAIL++); say(Str("PC2 m=",m," scrambled-label cross fraction=",fm(f)," (",pc2d,"/",pc2r,") pass=",ok));
  listput(JS,Str("\"pc2_m",m,"\":{\"fraction\":",jn(f),",\"diff\":",pc2d,",\"rational_pairs\":",pc2r,",\"pass\":",ok,"}")));
}
\\ NC1 noise band
{ my(v=vecsort(Vec(NC1R)),n=#v,vb=vecsort(Vec(NC1Rbig)),nb=#vb);
  if(n>0, BLO=v[max(1,round(0.025*n))]*1.; BHI=v[min(n,ceil(0.975*n))]*1.; BMED=med(v)*1., BLO=BHI=BMED=1.);
  say(Str("NC1 wall-ratio (shuffled-E / E) noise band over ",n," repeat-level ratios: median=",fm(BMED)," 95% band=[",fm(BLO),",",fm(BHI),"]"));
  if(nb>0, say(Str("NC1 restricted to runs with E time >= 100 ms: n=",nb," median=",fm(med(vb))," 95% band=[",fm(vb[max(1,round(0.025*nb))]),",",fm(vb[min(nb,ceil(0.975*nb))]),"]")));
}
\\ F3 wall test
for(i=1,#CELLS, my(c=CELLS[i],wc=c[4]); if(c[3]==1 && wc[3]<1 && wc[3]<BLO, F3wall++; say(Str("F3_WALL_TRIGGER m=",c[1]," l=",c[2]," hi=",fm(wc[3])))));
for(i=1,#CELLS, my(c=CELLS[i],wc=c[4],cls); cls=if(wc[2]>BHI,"slower-than-E (above NC1 band)",if(wc[3]<BLO&&wc[3]<1,"FASTER","indistinguishable from NC1 band")); say(Str("WALLCLASS m=",c[1]," l=",c[2]," pop=",if(c[3]==1,"rand","poly")," S/E 95% interval [",fm(wc[2]),",",fm(wc[3]),"]: ",cls)));
\\ ---- verdict ----
F1F=(F1>0); F2F=(CROSS1>0||CROSS2>0||F2FAIL>0); F3F=(F3cost>0||F3wall>0); F4F=(F4>0||VIOL>0);
CTRLFAIL=(PC1FAIL>0||PC2FAIL>0||PC3FAIL>0||NC1BAD>0||NC3BAD>0);
say(Str("FALSIFIERS F1(relation-set difference)=",F1F," F2(cross-type rational pair or mixed F_q^2 sum)=",F2F," F3(cost per relation lower or wall interval below 1 and band)=",F3F," F4(triple formula/type-iff violated)=",F4F));
say(Str("CONTROLS PC1_fail_cells=",PC1FAIL," PC2_fail_m=",PC2FAIL," PC3_fail=",PC3FAIL," NC1_relation_mismatch=",NC1BAD," NC3_E_rel_on_K1_targets=",NC3BAD," tgt_type_mismatch=",TGTBAD," rootfail=",ROOTFAIL," rows_neq=",ROWSBAD," type1_classes_used_for_K0=",T1USED));
say(Str("P3 numeric windows (rand, l>=6): cells=",P3N," cost-ratio median outside [1.6,2.5]=",P3COST_OUT," wall-ratio median outside [1.3,2.3]=",P3WALL_OUT));
say(Str("MISSING cells/draws: ",#MISSING," ",Vec(MISSING)));
say(Str("INVALID reasons: ",Vec(INVALID)));
OUTCOME=if(#INVALID>0||CTRLFAIL||#MISSING>0, "inconclusive", if(F1F||F2F||F3F||F4F,"prediction_refuted","prediction_confirmed"));
say(Str("OUTCOME ",OUTCOME));
{ my(s=Str("{\"experiment\":\"E4-koblitz-twist-pair\",\"outcome\":\"",OUTCOME,"\","));
  s=Str(s,"\"falsifiers\":{\"F1\":",F1F,",\"F2\":",F2F,",\"F3\":",F3F,",\"F4\":",F4F,"},");
  s=Str(s,"\"controls_failed\":{\"PC1\":",PC1FAIL,",\"PC2\":",PC2FAIL,",\"PC3\":",PC3FAIL,",\"NC1_mismatch\":",NC1BAD,",\"NC3_nonzero\":",NC3BAD,"},");
  s=Str(s,"\"nc1_noise_band\":{\"n\":",#NC1R,",\"median\":",jn(BMED),",\"lo\":",jn(BLO),",\"hi\":",jn(BHI),"},");
  s=Str(s,"\"missing\":",#MISSING,",\"p3_windows\":{\"cells\":",P3N,",\"cost_out\":",P3COST_OUT,",\"wall_out\":",P3WALL_OUT,"},");
  s=Str(s,"\"cross_rational_pairs_stage1_total\":",CROSS1,",\"cross_relations_stage2_total\":",CROSS2,",\"stage1_violations\":",VIOL,",");
  my(v=Vec(JS)); for(i=1,#v, s=Str(s,v[i],if(i<#v,",","")));
  s=Str(s,"}");
  write("results.json",s); }
quit
