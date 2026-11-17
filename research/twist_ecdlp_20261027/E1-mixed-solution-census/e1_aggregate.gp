\\ e1_aggregate.gp -- reads raw/*.out of the E1 census, writes summary.tsv on stdout and
\\ pooled.tsv, controls_summary.tsv, verdict.txt, results.json next to it.  (PREREGISTRATION.md sections 7-8)
default(parisizemax,2000000000);
default(realprecision,19);
genv(name,def)={my(s=getenv(name)); if(type(s)=="t_STR" && s!="", s, def)};
S0=eval(genv("E1_SEED","20261021"));
RAW=genv("E1_RAW","raw");
OUT=genv("E1_OUTDIR",".");
BOOT=2000;
setrand(S0+1);

readlines(fn)=iferr(readstr(fn),E,[]);
sp(line)=strsplit(line,"\t");
num(s)=eval(s);
wilson(k,N)={my(z=1.959964,p=k/N,den=1+z^2/N,c=(p+z^2/(2*N))/den,h=z*sqrt(p*(1-p)/N+z^2/(4*N^2))/den);[c-h,c+h]};
\\ [lo,hi,method] for a proportion k/N : exact Clopper-Pearson upper bound 1-0.025^(1/N) when k==0, else Wilson
ciprop(k,N)={if(N==0,return([0.,1.,"NA"]));if(k==0,return([0.,1-0.025^(1/N),"CP0"]));my(w=wilson(k,N));[w[1],w[2],"Wilson"]};
\\ percentile bootstrap of the mean of vector v over its entries, BOOT resamples
boot(v)={my(N=#v,mn=vector(BOOT),s);if(N==0,return([0.,0.]));
  for(b=1,BOOT,s=0;for(i=1,N,s+=v[1+random(N)]);mn[b]=s/N);mn=vecsort(mn);[mn[round(0.025*BOOT)],mn[round(0.975*BOOT)]]};
\\ percentile bootstrap of sum(m)/(sum(m)+sum(e)) over targets (cluster bootstrap)
bootfrac(e,m)={my(N=#e,fr=vector(BOOT),se,sm,j);
  for(b=1,BOOT,se=0;sm=0;for(i=1,N,j=1+random(N);se+=e[j];sm+=m[j]);fr[b]=if(se+sm==0,0.,sm/(se+sm)));fr=vecsort(fr);[fr[round(0.025*BOOT)],fr[round(0.975*BOOT)]]};
r4(x)=round(x*10000)/10000.;
fmt(x)=if(type(x)=="t_REAL",if(x==0,"0",concat(strsplit(strprintf("%.7g",x)," "))),if(type(x)=="t_FRAC",fmt(1.*x),Str(x)));
tsvs(v)={my(s="");for(i=1,#v,s=Str(s,fmt(v[i]),if(i<#v,"\t","")));s};

\\ ================================================================ grid summary
HDR=["tag","bits","pidx","cidx","m","fb","alpha","q","cardE","cardEt","gcdodd","nT","curveflag","n","nE","nW","nT2","targets_done","targets_planned","truncated","E_nd","E_deg","M_nd","M_deg","S_nd","S_deg","target_mismatches","targets_xR_in_V","meanE_nd","B_naive","B_ref","ratio_naive","ratio_naive_lo","ratio_naive_hi","ratio_ref","ratio_ref_lo","ratio_ref_hi","f_mixed","f_lo","f_hi","f_method","f_q_over_nT","resolvable","H2_point","F4","H3_naive","H3_ref","tab_s","loop_s","status"];
print(tsvs(HDR));
POOL=Map();
nrows=0;notrun=List();dups=List();incomplete=List();allrows=List();
jobs=readlines(Str(RAW,"/joblist.txt"));
finalize(cfg,eNd,eDeg,mNd,mDeg,sNd,sDeg,mis,xin,ev,done,endf)={
  my(tag=cfg[2],bits=num(cfg[3]),pidx=num(cfg[4]),cidx=num(cfg[5]),m=num(cfg[6]),fb=cfg[7],alpha=cfg[8],q=num(cfg[9]),cE=num(cfg[12]),cEt=num(cfg[13]),go=num(cfg[14]),nT=num(cfg[15]),flag=cfg[16],
     n=num(cfg[17]),nE=num(cfg[18]),nW=num(cfg[19]),nT2=num(cfg[20]),NT=num(cfg[23]),Bn,Br,mean,bt,rn,rr,fci,f,res,h2,f4,h3n,h3r,fq,trunc,tabs,loops,status);
  if(endf==0,trunc=-1;tabs=-1;loops=-1;status="INCOMPLETE",trunc=num(endf[7]);tabs=endf[5];loops=endf[6];status=if(trunc,"TRUNCATED","ok"));
  Bn=1.*n^m/(m!*q);Br=1.*2^m*nE^m/(m!*cE);
  mean=if(done>0,1.*eNd/done,0.);
  bt=if(done>0,boot(ev),[0.,0.]);
  rn=[mean/Bn,bt[1]/Bn,bt[2]/Bn];rr=[mean/Br,bt[1]/Br,bt[2]/Br];
  fci=ciprop(mNd,mNd+eNd);f=if(mNd+eNd>0,mNd/(mNd+eNd),0.);
  res=(eNd>=4*q);
  h2=if(!res,"UNRESOLVED",if(f<=5./q,"PASS","FAIL"));
  f4=if(res&&fci[1]>5./q,1,0);
  fq=if(nT>0&&m==3&&mNd+eNd>0,f*q/nT,"NA");
  h3n=if(fb=="random",if(rn[1]>=0.7&&rn[1]<=1.4,"PASS","FAIL"),"NA");
  h3r=if(fb=="random",if(rr[1]>=0.7&&rr[1]<=1.4,"PASS","FAIL"),"NA");
  listput(allrows,[tag,bits,pidx,cidx,m,fb,alpha,q,cE,cEt,go,nT,flag,n,nE,nW,nT2,done,NT,trunc,eNd,eDeg,mNd,mDeg,sNd,sDeg,mis,xin,mean,Bn,Br,rn[1],rn[2],rn[3],rr[1],rr[2],rr[3],f,fci[1],fci[2],fci[3],fq,res,h2,f4,h3n,h3r,tabs,loops,status]);
  print(tsvs(allrows[#allrows]));
  \\ pooled by stratum (m,nT,fb)
  my(key=Str(m,"|",nT,"|",fb),cur);
  if(!mapisdefined(POOL,key,&cur),cur=vector(12));
  cur+=[1,eNd,eDeg,mNd,mDeg,sNd,sDeg,mis,done,if(res,1,0),f4,if(status=="ok",0,1)];
  mapput(POOL,key,cur);
  0};

{for(ji=1,#jobs,
  my(jf=strsplit(jobs[ji]," "),fn,lines,cfg=0,eNd,eDeg,mNd,mDeg,sNd,sDeg,mis,xin,ev,done,v);
  fn=Str(RAW,"/grid_",jf[1],"_",jf[2],"_",jf[3],"_m",jf[4],"_",jf[5],".out");
  lines=readlines(fn);
  if(#lines==0,listput(notrun,jobs[ji]);next);
  for(li=1,#lines,
    v=sp(lines[li]);
    if(v[1]=="CFG",
      if(cfg!=0,finalize(cfg,eNd,eDeg,mNd,mDeg,sNd,sDeg,mis,xin,ev,done,0);listput(incomplete,cfg[2]));
      cfg=v;eNd=0;eDeg=0;mNd=0;mDeg=0;sNd=0;sDeg=0;mis=0;xin=0;ev=List();done=0);
    if(v[1]=="TGT"&&cfg!=0,
      my(a=num(v[4]),b=num(v[5]),c=num(v[6]),d=num(v[7]),e=num(v[8]),g=num(v[9]));
      eNd+=a;eDeg+=b;mNd+=c;mDeg+=d;sNd+=e;sDeg+=g;if(c!=e||d!=g,mis++);xin+=num(v[10]);listput(ev,a);done++);
    if(v[1]=="END"&&cfg!=0,finalize(cfg,eNd,eDeg,mNd,mDeg,sNd,sDeg,mis,xin,Vec(ev),done,v);cfg=0);
    if(v[1]=="DUP",listput(dups,lines[li])));
  if(cfg!=0,finalize(cfg,eNd,eDeg,mNd,mDeg,sNd,sDeg,mis,xin,Vec(ev),done,0);listput(incomplete,cfg[2])));};

\\ ================================================================ pooled strata
PF=Str(OUT,"/pooled.tsv");
write(PF,tsvs(["m","nT","fb","configs","E_nd","E_deg","M_nd","M_deg","S_nd","S_deg","target_mismatches","targets","resolvable_configs","F4_configs","nonok_configs","f_mixed","f_lo","f_hi","f_method","contains_1-2^-m","F5"]));
PM=Mat(POOL);
F5any=0;
{for(r=1,matsize(PM)[1],
  my(key=PM[r,1],c=PM[r,2],kf=strsplit(key,"|"),mm=num(kf[1]),ci,pr,cont,f5);
  ci=ciprop(c[4],c[4]+c[2]);pr=1-2^(-mm);
  cont=if(c[4]+c[2]>0&&ci[1]<=pr&&pr<=ci[2],1,0);
  f5=if(cont&&c[4]>0,1,0);F5any+=f5;
  write(PF,tsvs([mm,num(kf[2]),kf[3],c[1],c[2],c[3],c[4],c[5],c[6],c[7],c[8],c[9],c[10],c[11],c[12],if(c[4]+c[2]>0,1.*c[4]/(c[4]+c[2]),0.),ci[1],ci[2],ci[3],cont,f5])));};

\\ ================================================================ verdict over the grid
tot=vector(8); \\ configs, targets, mismatchTargets, F1 count, F2 count, F4 count, truncated, nonok
F1l=List();F2l=List();F4l=List();H2res=0;H2pass=0;H2fail=0;H3nP=0;H3nF=0;H3rP=0;H3rF=0;
nM3nT=0;
{for(i=1,#allrows,
  my(r=allrows[i]);
  tot[1]++;tot[2]+=r[18];tot[3]+=r[27];
  if(r[5]==2&&r[23]+r[24]>0,tot[4]++;listput(F1l,r[1]));
  if(r[5]==3&&r[12]==0&&r[23]>0,tot[5]++;listput(F2l,r[1]));
  if(r[45]==1,tot[6]++;listput(F4l,r[1]));
  if(r[50]!="ok",tot[7]++);
  if(r[44]=="PASS",H2pass++;H2res++);
  if(r[44]=="FAIL",H2fail++;H2res++);
  if(r[46]=="PASS",H3nP++);if(r[46]=="FAIL",H3nF++);
  if(r[47]=="PASS",H3rP++);if(r[47]=="FAIL",H3rF++));};

\\ ================================================================ stage 0 / controls summary
CF=Str(OUT,"/controls_summary.tsv");
s0=readlines(Str(RAW,"/stage0_oracle.out"));
s1=readlines(Str(RAW,"/stage1_controls.out"));
v0n=0;v0bad=0;v0solutions=0;v0mixnd=0;v0mixdeg=0;toyn=0;toybad=0;toyMnd=0;toyMdeg=0;rcbad=0;v0res="MISSING";
{for(i=1,#s0,my(v=sp(s0[i]));
  if(v[1]=="V0",v0n++;if(num(v[#v])!=1,v0bad++);v0solutions+=num(v[7]);v0mixnd+=num(v[13]);v0mixdeg+=num(v[14]));
  if(v[1]=="V0TOY",toyn++;if(num(v[#v])!=1,toybad++);toyMnd+=num(v[13]);toyMdeg+=num(v[14]));
  if(v[1]=="RESCHECK",rcbad+=num(v[4]));
  if(v[1]=="V0_RESULT",v0res=v[2]));};
write(CF,tsvs(["#stage0","V0_rows",v0n,"V0_bad",v0bad,"V0_solutions_A",v0solutions,"V0_mixed_nd_A",v0mixnd,"V0_mixed_deg_A",v0mixdeg,"TOY_rows",toyn,"TOY_bad",toybad,"TOY_mixed_nd_A",toyMnd,"TOY_mixed_deg_A",toyMdeg,"rescheck_bad",rcbad,"V0_RESULT",v0res]));
G=Map();
gadd(key,vals)={my(cur);if(!mapisdefined(G,key,&cur),cur=vector(#vals));cur+=vals;mapput(G,key,cur);0};
n2e=Map();n2m=Map();n2line=List();
c1res="MISSING";
{for(i=1,#s1,my(v=sp(s1[i]),t=v[1]);
  if(t=="PC1"||t=="PC2",gadd(Str(t,"|bits=",v[2],"|curve=",v[3]),[1,num(v[6]),num(v[#v])]));
  if(t=="PC3",gadd(Str(t,"|bits=",v[2],"|curve=",v[3],"|m=",v[4]),[1,num(v[6]),num(v[#v])]));
  if(t=="NC1",gadd(Str(t,"|bits=",v[2],"|curve=",v[3],"|m=",v[4]),[1,num(v[#v]),num(v[12])+num(v[13]),num(v[6])]));
  if(t=="NC1big",gadd(Str(t,"|bits=",v[2],"|curve=",v[3],"|m=",v[4]),[1,num(v[#v]),num(v[10])+num(v[11]),num(v[7])]));
  if(t=="NC3",gadd(Str(t,"|bits=",v[2],"|curve=",v[3],"|",v[5],"|m=",v[6]),[1,num(v[10]),num(v[11]),num(v[12]),num(v[13]),num(v[14]),num(v[15]),num(v[16]),num(v[17])]));
  if(t=="NC2",listput(n2line,v));
  if(t=="NC2T",my(key=Str(v[2],"|",v[3],"|",v[4]),cl);
    if(!mapisdefined(n2e,key,&cl),cl=List());listput(cl,num(v[6]));mapput(n2e,key,cl);
    if(!mapisdefined(n2m,key,&cl),cl=List());listput(cl,num(v[7]));mapput(n2m,key,cl));
  if(t=="CONTROLS_RESULT",c1res=v[2]));};
GM=Mat(G);
pcbad=0;nc1bad=0;nc3bad=0;
write(CF,"#group\tcount\tfound_or_ok\t...");
{for(r=1,matsize(GM)[1],my(key=GM[r,1],c=GM[r,2]);
  write(CF,Str(key,"\t",tsvs(c)));
  if(strsplit(key,"|")[1]=="PC1"||strsplit(key,"|")[1]=="PC2"||strsplit(key,"|")[1]=="PC3",if(c[2]!=c[1]||c[3]!=c[1],pcbad++));
  if(strsplit(key,"|")[1]=="NC1"||strsplit(key,"|")[1]=="NC1big",if(c[2]!=c[1]||c[3]!=0,nc1bad++));
  if(strsplit(key,"|")[1]=="NC3",if(c[8]!=0||(strsplit(key,"|")[5]=="m=2"&&c[4]+c[5]>0),nc3bad++)));};
\\ NC2: literal Wilson vs 1-2^-m (as preregistered) and cluster bootstrap over targets vs conditional null 1-(nE/n)^m
write(CF,"#NC2\tbits\tfb\tm\ttargets\tn\tnE\tM_nd\tE_nd\tfrac\twilson_lo\twilson_hi\tnull_1-2^-m\tnull_cond\tliteral_contains_1-2^-m\tcluster_lo\tcluster_hi\tcluster_contains_cond\tcluster_contains_1-2^-m");
nc2lit=0;nc2cl=0;nc2n=0;nc2maxdev=0.;nc2minfrac=1.;
{for(i=1,#n2line,my(v=n2line[i],key=Str(v[2],"|",v[3],"|",v[4]),e,mm,bf,pr,pc,cc,cl2,lit);
  e=Vec(mapget(n2e,key));mm=Vec(mapget(n2m,key));
  bf=bootfrac(e,mm);pr=num(v[13]);pc=num(v[14]);lit=num(v[15]);
  cc=if(bf[1]<=pc&&pc<=bf[2],1,0);cl2=if(bf[1]<=pr&&pr<=bf[2],1,0);
  nc2n++;nc2lit+=lit;nc2cl+=cc;nc2maxdev=max(nc2maxdev,abs(num(v[10])-pc));nc2minfrac=min(nc2minfrac,num(v[10]));
  write(CF,Str("NC2\t",v[2],"\t",v[3],"\t",v[4],"\t",tsvs(vector(#v-4,j,v[j+4])),"\t",tsvs([bf[1],bf[2],cc,cl2]))));};
write(CF,tsvs(["#summary","PC_groups_bad",pcbad,"NC1_groups_bad",nc1bad,"NC3_groups_bad",nc3bad,"NC2_cells",nc2n,"NC2_literal_pass",nc2lit,"NC2_cluster_cond_pass",nc2cl,"NC2_max_abs_dev_from_cond_null",nc2maxdev,"NC2_min_mixed_fraction",nc2minfrac,"CONTROLS_RESULT_raw",c1res]));

\\ ================================================================ verdict.txt and results.json
VF=Str(OUT,"/verdict.txt");
write(VF,Str("configs_in_summary\t",tot[1]));
write(VF,Str("jobs_not_run\t",#notrun,"\t",Vec(notrun)));
write(VF,Str("configs_incomplete\t",#incomplete));
write(VF,Str("configs_truncated_or_nonok\t",tot[7]));
write(VF,Str("targets_total\t",tot[2]));
write(VF,Str("F3_target_mismatches_total\t",tot[3]));
write(VF,Str("F1_configs_with_mixed_m2\t",tot[4],"\t",Vec(F1l)));
write(VF,Str("F2_configs_with_nd_mixed_m3_nT0\t",tot[5],"\t",Vec(F2l)));
write(VF,Str("F4_configs\t",tot[6],"\t",Vec(F4l)));
write(VF,Str("F5_strata\t",F5any));
write(VF,Str("H2_resolvable_configs\t",H2res,"\tpass\t",H2pass,"\tfail\t",H2fail));
write(VF,Str("H3_random_naive_pass\t",H3nP,"\tfail\t",H3nF));
write(VF,Str("H3_random_ref_pass\t",H3rP,"\tfail\t",H3rF));
write(VF,Str("stage0\t",v0res,"\tV0_bad\t",v0bad,"\tTOY_bad\t",toybad,"\trescheck_bad\t",rcbad));
f6lit=if(v0res!="PASS"||c1res!="PASS"||pcbad>0||nc1bad>0||nc3bad>0||nc2lit<nc2n||rcbad>0||toybad>0,1,0);
write(VF,Str("F6_literal_gate_failed\t",f6lit,"\t(stage0=",v0res,",controls=",c1res,",NC2_literal=",nc2lit,"/",nc2n,")"));
write(VF,Str("NC2_max_abs_dev_from_cond_null\t",fmt(nc2maxdev),"\tmin_mixed_fraction\t",fmt(nc2minfrac)));
write(VF,Str("controls\t",c1res,"\tPC_bad_groups\t",pcbad,"\tNC1_bad\t",nc1bad,"\tNC3_bad\t",nc3bad,"\tNC2_literal_pass\t",nc2lit,"/",nc2n,"\tNC2_cluster_pass\t",nc2cl,"/",nc2n));

JF=Str(OUT,"/results.json");
js(s)=Str("\"",s,"\"");
jv(v)=Str("[",concat(vector(#v,i,Str(v[i],if(i<#v,",",""))))," ]");
write(JF,"{");
write(JF,Str("\"configs\":",tot[1],",\"targets_total\":",tot[2],",\"jobs_not_run\":",#notrun,",\"configs_incomplete\":",#incomplete,",\"configs_truncated_or_nonok\":",tot[7],","));
write(JF,Str("\"F3_target_mismatches_total\":",tot[3],",\"F1_configs\":",tot[4],",\"F2_configs\":",tot[5],",\"F4_configs\":",tot[6],",\"F5_strata\":",F5any,","));
write(JF,Str("\"H2_resolvable\":",H2res,",\"H2_pass\":",H2pass,",\"H2_fail\":",H2fail,",\"H3_random_naive_pass\":",H3nP,",\"H3_random_naive_fail\":",H3nF,",\"H3_random_ref_pass\":",H3rP,",\"H3_random_ref_fail\":",H3rF,","));
write(JF,Str("\"stage0\":",js(v0res),",\"V0_rows\":",v0n,",\"V0_bad\":",v0bad,",\"TOY_rows\":",toyn,",\"TOY_bad\":",toybad,",\"rescheck_bad\":",rcbad,",\"V0_mixed_nd_A\":",v0mixnd,",\"V0_mixed_deg_A\":",v0mixdeg,",\"TOY_mixed_nd_A\":",toyMnd,",\"TOY_mixed_deg_A\":",toyMdeg,","));
write(JF,Str("\"controls\":",js(c1res),",\"PC_bad_groups\":",pcbad,",\"NC1_bad_groups\":",nc1bad,",\"NC3_bad_groups\":",nc3bad,",\"NC2_max_abs_dev_from_cond_null\":",fmt(nc2maxdev),",\"NC2_min_mixed_fraction\":",fmt(nc2minfrac),",\"F6_literal_gate_failed\":",f6lit,",\"NC2_cells\":",nc2n,",\"NC2_literal_pass\":",nc2lit,",\"NC2_cluster_cond_pass\":",nc2cl,","));
write(JF,"\"strata\":[");
{for(r=1,matsize(PM)[1],
  my(key=PM[r,1],c=PM[r,2],kf=strsplit(key,"|"),mm=num(kf[1]),ci=ciprop(c[4],c[4]+c[2]));
  write(JF,Str("{\"m\":",mm,",\"nT\":",kf[2],",\"fb\":",js(kf[3]),",\"configs\":",c[1],",\"E_nd\":",c[2],",\"E_deg\":",c[3],",\"M_nd\":",c[4],",\"M_deg\":",c[5],",\"S_nd\":",c[6],",\"S_deg\":",c[7],",\"target_mismatches\":",c[8],",\"targets\":",c[9],",\"f_lo\":",fmt(ci[1]),",\"f_hi\":",fmt(ci[2]),",\"f_method\":",js(ci[3]),"}",if(r<matsize(PM)[1],",","")));
);};
write(JF,"]}");
