\\ e2_analyze.gp -- reads raw/run_*.out, raw/rho_*.out, raw/L1_L2_*.out ; writes results.csv, summary.csv, results.json ; prints verdict flags
default(parisizemax,2000000000);
default(realprecision,38);
WS=[0,1/10,1]; WN=["w0","w0.1","w1"]; WL=["0","0.1","1"];
NBOOT=10000;

fmt(x)=if(x==0,"0",strprintf("%.6g",x*1.));
med(v)={ my(n=#v,s); if(n==0,return(-1)); s=vecsort(v); if(n%2, s[(n+1)/2], (s[n/2]+s[n/2+1])/2) };
bootmed(v)={ my(n=#v,res=vector(NBOOT),s); if(n<3,return([-1,-1])); for(r=1,NBOOT, s=vector(n,i,v[random(n)+1]); res[r]=med(s)); res=vecsort(res); [res[250],res[9750]] };
isnumstr(s)={ my(c=Vec(s)[1]); (c=="-")||(c>="0"&&c<="9") };
kvline(line)={ my(t=strsplit(line,","),m=Map(),kv); for(i=2,#t, kv=strsplit(t[i],"="); if(#kv==2, mapput(m,kv[1],if(isnumstr(kv[2]),eval(kv[2]),kv[2])))); m };

\\ ---------- load runs ----------
files=externstr("ls raw/run_*.out 2>/dev/null");
rows=List(); names=0; idx=Map(); nmiss=0; missing=List();
{
for(f=1,#files,
  my(L=readstr(files[f]),hi=0,vals);
  for(u=1,#L, if(#L[u]>=14 && Vec(L[u])[1..13]==Vec("RESULT_HEADER"), hi=u));
  if(hi==0 || hi==#L, listput(missing,files[f]); next);
  if(names===0, names=strsplit(L[hi],",")[2..-1]; for(i=1,#names,mapput(idx,names[i],i)));
  vals=vector(#names,i,eval(strsplit(L[hi+1],",")[i+1]));
  listput(rows,vals));
}
rows=Vec(rows); nrun=#rows;
{ for(i=1,nrun, rows[i][mapget(idx,"mu")]=round(rows[i][mapget(idx,"mu")]*4)/4); }
g(r,nm)=r[mapget(idx,nm)];

\\ ---------- load rho ----------
rhofiles=externstr("ls raw/rho_*.out 2>/dev/null");
rhoM=Map();   \\ key [bits,curve] -> vector of [steps+init+setup, cmp, success]
{
for(f=1,#rhofiles,
  my(L=readstr(rhofiles[f]));
  for(u=1,#L, if(#L[u]>=4 && Vec(L[u])[1..4]==Vec("RHO,"),
    my(m=kvline(L[u]), key=[mapget(m,"bits"),mapget(m,"curve")], cur=if(mapisdefined(rhoM,key),mapget(rhoM,key),[]));
    mapput(rhoM,key,concat(cur,[[mapget(m,"steps")+mapget(m,"init")+mapget(m,"setup"),mapget(m,"cmp"),mapget(m,"success"),mapget(m,"l")]])))));
}
rhocost(bits,c,w)={ my(v,s=0); if(!mapisdefined(rhoM,[bits,c]),return(-1)); v=mapget(rhoM,[bits,c]); for(i=1,#v, s+=v[i][1]+w*v[i][2]); s/#v };
rhofail(bits,c)={ my(v,s=0); if(!mapisdefined(rhoM,[bits,c]),return(-1)); v=mapget(rhoM,[bits,c]); for(i=1,#v, if(v[i][3]!=1,s++)); s };

\\ ---------- per-run derived quantities ----------
\\ returns vector indexed by w : [CE,CEp,Cb,CA4,eps,sav,single,Rpc,Rj0,rho,dist]
derive(r,wi)={ my(Cb2, w=WS[wi], sE=g(r,"startE"), sEb=g(r,"startEb"), CE,CEp,Cb,CA4,eps,sav,single,Rpc=-1,Rj0=-1,rho,dist,Csh,Csep,Cj0);
  CE =sE+g(r,"TE")+g(r,"KE") + w*(g(r,"scanE")+g(r,"SQE")+g(r,"cmpE")+g(r,"prE_A"));
  CEp=sE+g(r,"TEp")+g(r,"KEp") + w*(g(r,"scanEp")+g(r,"SQEp")+g(r,"cmpEp")+g(r,"prEp_A")+g(r,"TEp"));
  Cb =2*sE+g(r,"TE")+g(r,"TEp")+g(r,"KE")+g(r,"KEp") + w*(g(r,"scanJ")+g(r,"SQE")+g(r,"SQEp")+g(r,"cmpJ")+g(r,"prE_J")+g(r,"prEp_J")+g(r,"TEp"));
  CA4=sE+g(r,"TE")+g(r,"KE")+g(r,"KEp") + w*(g(r,"scanJ")+g(r,"SQE")+g(r,"SQEp")+g(r,"cmpJ")+g(r,"prE_J"));
  eps=Cb/CE-1; sav=1-Cb/(CE+CEp); single=CA4/CE-1;
  if(g(r,"T2")>=0,
    Csh=sE+g(r,"TE")+sEb+g(r,"T2")+g(r,"KE") + w*(g(r,"scanE")+g(r,"SQE")+g(r,"cmpE")+g(r,"prE_A")+g(r,"pr2"));
    Csep=CE + (sEb+g(r,"T2")+g(r,"KE") + w*(g(r,"scanE")+g(r,"SQE")+g(r,"cmpE")+g(r,"pr2")));
    Rpc=Csh/Csep);
  if(g(r,"j0K")>=0,
    Cj0=Cb+g(r,"j0K")+w*g(r,"j0Leg"); Rj0=Cj0/(CE+CEp));
  rho=rhocost(g(r,"bits"),g(r,"curve"),w); dist=if(rho>0,CE/rho,-1);
  Cb2=2*sE+g(r,"TE")+g(r,"TEp")+g(r,"KE")+g(r,"KEp") + w*(g(r,"scanJ")+g(r,"SQE")+g(r,"SQEp")+g(r,"cmpE")+g(r,"cmpEp")+g(r,"prE_A")+g(r,"prEp_A")+g(r,"TEp"));
  [CE*1.,CEp*1.,Cb*1.,CA4*1.,eps*1.,sav*1.,single*1.,Rpc*1.,Rj0*1.,rho*1.,dist*1.,(Cb2/CE-1)*1.,(1-Cb2/(CE+CEp))*1.] };
D=vector(nrun,i,vector(3,wi,derive(rows[i],wi)));

\\ ---------- results.csv ----------
system("rm -f results.csv summary.csv results.json");
write("results.csv",Str("bits,mu,curve,seed,p,N,W,TE,TEp,T2,relE,relEp,eqE,eqEp,unverE,unverEp,crossE,crossEp,capE,capEp,hitrateE_over_yhat,hitrateEp_over_yhat,zE,zEp,colsE,compsE,rankE,j0Facc,ncT,ncHitT,ncVer,",strjoin(vector(3,wi,Str("CE_",WN[wi],",CEp_",WN[wi],",Cboth_",WN[wi],",CA4_",WN[wi],",eps_",WN[wi],",sav_",WN[wi],",single_",WN[wi],",Rpc_",WN[wi],",Rj0_",WN[wi],",rho_",WN[wi],",dist_",WN[wi],",epsBest_",WN[wi],",savBest_",WN[wi])),",")));
nc2z(r,typ)={ my(T=g(r,if(typ,"TEp","TE")), h=g(r,if(typ,"hitTEp","hitTE")), l=g(r,if(typ,"lEp","lE")), dk=g(r,if(typ,"distEp","distE")), y=dk/((l-1)/2.), ratio=(h/T)/y, z=(h-T*y)/sqrt(T*y*(1-y)));  [ratio*1.,z*1.] };
{
for(i=1,nrun, my(r=rows[i], zE=nc2z(r,0), zP=nc2z(r,1), s);
  s=Str(g(r,"bits"),",",fmt(g(r,"mu")),",",g(r,"curve"),",",g(r,"seed"),",",g(r,"p"),",",g(r,"N"),",",g(r,"W"),",",g(r,"TE"),",",g(r,"TEp"),",",g(r,"T2"),",",g(r,"relE"),",",g(r,"relEp"),",",g(r,"eqE"),",",g(r,"eqEp"),",",g(r,"unverE"),",",g(r,"unverEp"),",",g(r,"crossE"),",",g(r,"crossEp"),",",g(r,"capE"),",",g(r,"capEp"),",",fmt(zE[1]),",",fmt(zP[1]),",",fmt(zE[2]),",",fmt(zP[2]),",",g(r,"colsE"),",",g(r,"compsE"),",",g(r,"rankE"),",",g(r,"j0Facc"),",",g(r,"ncT"),",",g(r,"ncHitT"),",",g(r,"ncVer"));
  for(wi=1,3, for(k=1,13, s=Str(s,",",fmt(D[i][wi][k]))));
  write("results.csv",s));
}

\\ ---------- cells ----------
cells=List();
{ for(i=1,nrun, my(c=[g(rows[i],"bits"),g(rows[i],"mu")]); if(!setsearch(Set(cells),c) ,listput(cells,c))); }
cells=vecsort(Vec(cells));
cellruns(c)={ my(L=List()); for(i=1,nrun, if(g(rows[i],"bits")==c[1] && g(rows[i],"mu")==c[2], listput(L,i))); Vec(L) };
colvals(c,wi,k)={ my(L=List(),ids=cellruns(c)); for(u=1,#ids, my(v=D[ids[u]][wi][k]); if(v>=-0.5 || k==5 || k==6 || k==7, listput(L,v))); Vec(L) };
rng(v)={ if(#v==0,[-1,-1],[vecmin(v),vecmax(v)]) };

setrand(424242);
write("summary.csv","bits,mu,w,n,eps_med,eps_min,eps_max,eps_lo,eps_hi,sav_med,sav_min,sav_max,sav_lo,sav_hi,single_med,single_min,single_max,Rpc_n,Rpc_med,Rpc_min,Rpc_max,Rpc_lo,Rpc_hi,Rj0_n,Rj0_med,Rj0_min,Rj0_max,dist_n,dist_med,dist_min,dist_max,eq_frac,epsBest_med,savBest_med");
SUM=Map();   \\ key [bits,mu,wi] -> Map of stats
{
for(ci=1,#cells, my(c=cells[ci], ids=cellruns(c));
  my(eqn=0); for(u=1,#ids, eqn+=rows[ids[u]][mapget(idx,"eqE")]+rows[ids[u]][mapget(idx,"eqEp")]);
  for(wi=1,3,
    my(ve=colvals(c,wi,5), vs=colvals(c,wi,6), v1=colvals(c,wi,7), vp=colvals(c,wi,8), vj=colvals(c,wi,9), vd=colvals(c,wi,11), veb=colvals(c,wi,12), vsb=colvals(c,wi,13), be,bs,bp,m=Map(), re, rs, r1, rp, rj, rd);
    be=bootmed(ve); bs=bootmed(vs); bp=bootmed(vp);
    re=rng(ve); rs=rng(vs); r1=rng(v1); rp=rng(vp); rj=rng(vj); rd=rng(vd);
    mapput(m,"n",#ve); mapput(m,"eps_med",med(ve)); mapput(m,"eps_lo",be[1]); mapput(m,"eps_hi",be[2]); mapput(m,"eps_min",re[1]); mapput(m,"eps_max",re[2]);
    mapput(m,"sav_med",med(vs)); mapput(m,"sav_min",rs[1]); mapput(m,"sav_max",rs[2]); mapput(m,"sav_lo",bs[1]); mapput(m,"sav_hi",bs[2]);
    mapput(m,"single_med",med(v1)); mapput(m,"single_min",r1[1]); mapput(m,"single_max",r1[2]);
    mapput(m,"Rpc_n",#vp); mapput(m,"Rpc_med",med(vp)); mapput(m,"Rpc_min",rp[1]); mapput(m,"Rpc_max",rp[2]); mapput(m,"Rpc_lo",bp[1]); mapput(m,"Rpc_hi",bp[2]);
    mapput(m,"Rj0_n",#vj); mapput(m,"Rj0_med",med(vj)); mapput(m,"Rj0_min",rj[1]); mapput(m,"Rj0_max",rj[2]);
    mapput(m,"dist_n",#vd); mapput(m,"dist_med",med(vd)); mapput(m,"dist_min",rd[1]); mapput(m,"dist_max",rd[2]);
    mapput(m,"eq_frac",eqn/(2.*#ids)); mapput(m,"epsBest_med",med(veb)); mapput(m,"savBest_med",med(vsb));
    mapput(SUM,[c[1],c[2],wi],m);
    write("summary.csv",Str(c[1],",",fmt(c[2]),",",WL[wi],",",#ve,",",fmt(med(ve)),",",fmt(re[1]),",",fmt(re[2]),",",fmt(be[1]),",",fmt(be[2]),",",fmt(med(vs)),",",fmt(rs[1]),",",fmt(rs[2]),",",fmt(bs[1]),",",fmt(bs[2]),",",fmt(med(v1)),",",fmt(r1[1]),",",fmt(r1[2]),",",#vp,",",fmt(med(vp)),",",fmt(rp[1]),",",fmt(rp[2]),",",fmt(bp[1]),",",fmt(bp[2]),",",#vj,",",fmt(med(vj)),",",fmt(rj[1]),",",fmt(rj[2]),",",#vd,",",fmt(med(vd)),",",fmt(rd[1]),",",fmt(rd[2]),",",fmt(eqn/(2.*#ids)),",",fmt(med(veb)),",",fmt(med(vsb))))));
}

\\ ---------- control files (L1/L2) ----------
lfiles=externstr("ls raw/L1_L2_*.out 2>/dev/null");
L1ok=1; L2ok=1; nL1=0; nL2=0;
{
for(f=1,#lfiles, my(L=readstr(lfiles[f]));
  for(u=1,#L,
    if(#L[u]>=3 && Vec(L[u])[1..3]==Vec("L1,"), my(m=kvline(L[u])); nL1++;
      print("L1 ",lfiles[f],": ",L[u]);
      if(mapget(m,"mixed_rational")!=0||mapget(m,"mixed_zero_disc")!=0||mapget(m,"key_intersection")!=0||mapget(m,"E_keys_nonsquare_f")!=0||mapget(m,"Ep_keys_square_f")!=0, L1ok=0));
    if(#L[u]>=3 && Vec(L[u])[1..3]==Vec("L2,"), my(m=kvline(L[u])); nL2++;
      print("L2 ",lfiles[f],": ",L[u]);
      my(n=mapget(m,"pairs"));
      if(mapget(m,"T1_PplusQ_rational")!=0||mapget(m,"T2_BoZero_rational")!=n||mapget(m,"T3_frob_is_P_minus_Q")!=n||mapget(m,"T4_frob_neg")!=mapget(m,"T4_pairs")||mapget(m,"Q_frob_eq_negQ")!=n||mapget(m,"P_frob_fixed")!=n, L2ok=0))));
}
if(nL1==0, L1ok=-1); if(nL2==0, L2ok=-1);

\\ ---------- print the summary ----------
print("RUNS_LOADED=",nrun," missing/failed files=",#missing);
{ for(u=1,#missing, print("MISSING_OR_FAILED ",missing[u])); }
print("CELLS=",cells);
{
for(wi=1,3, print("--- w=",WL[wi]);
  for(ci=1,#cells, my(c=cells[ci], m=mapget(SUM,[c[1],c[2],wi]));
    print("bits=",c[1]," mu=",fmt(c[2])," n=",mapget(m,"n"),
      " eps med=",fmt(mapget(m,"eps_med"))," [",fmt(mapget(m,"eps_min")),",",fmt(mapget(m,"eps_max")),"] boot95=[",fmt(mapget(m,"eps_lo")),",",fmt(mapget(m,"eps_hi")),"]",
      " sav med=",fmt(mapget(m,"sav_med"))," [",fmt(mapget(m,"sav_min")),",",fmt(mapget(m,"sav_max")),"]",
      " single-1 med=",fmt(mapget(m,"single_med")),
      " Rpc n=",mapget(m,"Rpc_n")," med=",fmt(mapget(m,"Rpc_med"))," boot95=[",fmt(mapget(m,"Rpc_lo")),",",fmt(mapget(m,"Rpc_hi")),"]",
      " Rj0 n=",mapget(m,"Rj0_n")," med=",fmt(mapget(m,"Rj0_med")),
      " dist_rho n=",mapget(m,"dist_n")," med=",fmt(mapget(m,"dist_med")),
      " eq=",fmt(mapget(m,"eq_frac"))," epsBest=",fmt(mapget(m,"epsBest_med"))," savBest=",fmt(mapget(m,"savBest_med")))));
}

\\ ---------- verdict logic ----------
\\ instrument: PC1 (bits 22 and 26) at each w
pcpass=1; pcdetail=List();
{
foreach([22,26], bb, for(wi=1,3,
  if(mapisdefined(SUM,[bb,3,wi]) && mapisdefined(SUM,[bb,3/4,wi]),
    my(r3=mapget(mapget(SUM,[bb,3,wi]),"Rpc_med"), r07=mapget(mapget(SUM,[bb,3/4,wi]),"Rpc_med"), ok=(r3<=0.65 && r07>=0.85 && r07-r3>=0.20));
    listput(pcdetail,[bb,WL[wi],fmt(r3),fmt(r07),ok]); if(!ok,pcpass=0),
    pcpass=-1; listput(pcdetail,[bb,WL[wi],"missing"]))));
}
print("PC1 gate (R(mu=3)<=0.65, R(mu=0.75)>=0.85, diff>=0.20): ",pcdetail," -> pass=",pcpass);
print("L1 pass=",L1ok," (",nL1," files)  L2 pass=",L2ok," (",nL2," lines)");
instr=(pcpass==1 && L1ok==1 && L2ok==1);
print("instrument_valid=",instr);
\\ falsifier: every cell median eps >= 0.75 at every w
allge=1; cellsupp=List(); cellcontra=List();
{
for(ci=1,#cells, my(c=cells[ci]); my(okw=1,okc=1);
  for(wi=1,3, my(m=mapget(SUM,[c[1],c[2],wi]));
    if(mapget(m,"eps_med")<0.75, allge=0);
    if(!(mapget(m,"n")>=3 && mapget(m,"eps_hi")<=0.25), okw=0);
    if(!(mapget(m,"eps_med")<=0.25 && mapget(m,"n")>=3 && mapget(m,"eps_hi")<=0.25), okc=0));
  if(okw, listput(cellsupp,c)); if(okc, listput(cellcontra,c)));
}
bitsupp=#Set(vector(#cellsupp,i,cellsupp[i][1]));
supported=(#cellsupp>=3 && bitsupp>=2);
print("all cells median eps>=0.75 at all w: ",allge);
print("cells with boot95 upper bound eps<=0.25 at all w: ",Vec(cellsupp)," (bit sizes spanned: ",bitsupp,")");
print("H_idea SUPPORTED: ",supported,"   H_mech contradicted (cell with median<=0.25 and hi<=0.25): ",#cellcontra>0);
verdict=if(!instr,"INCONCLUSIVE (instrument gate failed or incomplete)", supported,"H_idea SUPPORTED", allge,"H_idea FALSIFIED (prediction held)", "INCONCLUSIVE (medians between 0.25 and 0.75 or mixed)");
print("VERDICT: ",verdict);

\\ ---------- prediction checks ----------
nrunsE=0; nclean=0; p1in=0; p2neg=0; p2big=0; crossany=0; unvany=0; capany=0; eqall=0; mixroot=0; p4n=0; p4in=0;
{
for(i=1,nrun, my(r=rows[i]);
  if(g(r,"unverE")>0||g(r,"unverEp")>0, unvany++);
  if(g(r,"crossE")>0||g(r,"crossEp")>0, crossany++);
  if(g(r,"capE")>0||g(r,"capEp")>0||g(r,"cap2")>0, capany++);
  if(g(r,"eqE")==1&&g(r,"eqEp")==1, eqall++);
  if(g(r,"j0Facc")>0, mixroot++));
}
print("P3/P4 runs: n=",nrun," runs with unverified main hits=",unvany," runs with cross-type joint hits=",crossany," runs hitting the cap=",capany," runs with equal relation sets (E and E')=",eqall," runs with mixed F_p-rational pair (J0)=",mixroot);
\\ P1: individual runs in [0.80,1.20]; P2: sav range; P4 ratio; P6; P7
{
for(wi=1,3, my(n1=0,nt=0,sv=0,mx=-1,mn=1,smax=-1);
  for(i=1,nrun, my(e=D[i][wi][5], s=D[i][wi][6]); nt++; if(e>=0.80&&e<=1.20,n1++); smax=max(smax,s); mn=min(mn,s));
  print("w=",WL[wi]," P1 runs with eps in [0.80,1.20]: ",n1,"/",nt,"  P2 sav range over all runs: [",fmt(mn),",",fmt(smax),"]"));
}
print("P1 cell medians in [0.95,1.05] (per w): ");
{
for(wi=1,3, my(ok=0,tot=0); for(ci=1,#cells, my(m=mapget(SUM,[cells[ci][1],cells[ci][2],wi]),e=mapget(m,"eps_med")); tot++; if(e>=0.95&&e<=1.05,ok++)); print("  w=",WL[wi],": ",ok,"/",tot));
}
print("P2 cell median sav within range (N>=300:[0,0.02]; N<300:[0,0.05]) (per w): ");
{
for(wi=1,3, my(ok=0,tot=0); for(ci=1,#cells, my(c=cells[ci], m=mapget(SUM,[c[1],c[2],wi]),s=mapget(m,"sav_med"), Nn=g(rows[cellruns(c)[1]],"N")); tot++; if(s>=0 && s<=if(Nn>=300,0.02,0.05),ok++)); print("  w=",WL[wi],": ",ok,"/",tot));
}
print("P4 single-curve: median(C_A4/C_E-1) per cell in [0.9,1.1] (per w): ");
{
for(wi=1,3, my(ok=0,tot=0); for(ci=1,#cells, my(m=mapget(SUM,[cells[ci][1],cells[ci][2],wi]),s=mapget(m,"single_med")); tot++; if(s>=0.9&&s<=1.1,ok++)); print("  w=",WL[wi],": ",ok,"/",tot));
}
print("P6 Rj0 median in [1.3,2.0] at mu>=1.5 (per w): ");
{
for(wi=1,3, my(ok=0,tot=0); for(ci=1,#cells, my(c=cells[ci],m=mapget(SUM,[c[1],c[2],wi]),s=mapget(m,"Rj0_med")); if(c[2]>=3/2 && mapget(m,"Rj0_n")>0, tot++; if(s>=1.3&&s<=2.0,ok++))); print("  w=",WL[wi],": ",ok,"/",tot));
}
print("P7 distance to rho: C_E/rho (median per cell) between 4 and 40 (per w): ");
{
for(wi=1,3, my(ok=0,tot=0); for(ci=1,#cells, my(c=cells[ci],m=mapget(SUM,[c[1],c[2],wi]),s=mapget(m,"dist_med")); if(mapget(m,"dist_n")>0, tot++; if(s>=4&&s<=40,ok++))); print("  w=",WL[wi],": ",ok,"/",tot));
}
\\ NC2 summary
{
my(zs=List(),rs=List());
for(i=1,nrun, my(z0=nc2z(rows[i],0),z1=nc2z(rows[i],1)); listput(zs,z0[2]); listput(zs,z1[2]); listput(rs,z0[1]); listput(rs,z1[1]));
my(zv=Vec(zs),rv=Vec(rs),n3=0); for(i=1,#zv, if(abs(zv[i])>3,n3++));
print("NC2: hit-rate/y_hat over ",#rv," streams: median=",fmt(med(rv))," min=",fmt(vecmin(rv))," max=",fmt(vecmax(rv))," ; |z|>3 in ",n3,"/",#zv);
}
\\ NC1 summary
{
my(nn=0,nv=0,nh=0,nu=0);
for(i=1,nrun, if(g(rows[i],"hasNC1")==1, nn++; nv+=g(rows[i],"ncVer"); nh+=g(rows[i],"ncHitT")); );
print("NC1: runs=",nn," total verified relations from wrong-curve targets=",nv," total wrong-curve hit targets=",nh);
}

\\ ---------- results.json ----------
{
my(js=Str("{\n  \"runs_loaded\": ",nrun,",\n  \"missing_or_failed_files\": ",#missing,",\n  \"instrument_valid\": ",instr,",\n  \"pc1_gate_pass\": ",pcpass,",\n  \"L1_pass\": ",L1ok,",\n  \"L2_pass\": ",L2ok,",\n  \"all_cells_median_eps_ge_0.75\": ",allge,",\n  \"h_idea_supported\": ",supported,",\n  \"verdict\": \"",verdict,"\",\n  \"cells\": [\n"));
for(ci=1,#cells, for(wi=1,3, my(c=cells[ci], m=mapget(SUM,[c[1],c[2],wi]), ks=["n","eps_med","eps_min","eps_max","eps_lo","eps_hi","sav_med","sav_min","sav_max","sav_lo","sav_hi","single_med","Rpc_n","Rpc_med","Rpc_lo","Rpc_hi","Rj0_n","Rj0_med","dist_n","dist_med","eq_frac","epsBest_med","savBest_med"], s);
  s=Str("    {\"bits\": ",c[1],", \"mu\": ",fmt(c[2]),", \"w\": ",fmt(WS[wi]));
  for(k=1,#ks, s=Str(s,", \"",ks[k],"\": ",fmt(mapget(m,ks[k]))));
  js=Str(js,s,"}",if(ci==#cells&&wi==3,"\n",",\n"))));
js=Str(js,"  ]\n}\n");
write("results.json",js);
}
print("WROTE results.csv summary.csv results.json");
quit;
