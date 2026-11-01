\\ g3_aggregate.gp -- reads raw/*.out of the G3 census; summary.tsv on stdout; writes pooled.tsv, controls_summary.tsv,
\\ nc2_cells.tsv, f4_table.tsv, verdict.txt, results.json into G3_OUTDIR.  (PREREGISTRATION.md sections 6-9)
\\ Every gate and the verdict are computed from the files. Missing files are counted as missing and block the dependent gate.
default(parisizemax,2000000000);
default(realprecision,19);
genv(name,def)={my(s=getenv(name)); if(type(s)=="t_STR" && s!="", s, def)};
S0=eval(genv("G3_SEED","20261026"));
RAW=genv("G3_RAW","raw");
REP=genv("G3_REP","raw_rep");
OUT=genv("G3_OUTDIR",".");
BOOT=2000;

readlines(fn)=iferr(readstr(fn),E,[]);
sp(line)=strsplit(line,"\t");
num(s)=eval(s);
wilson(k,N)={my(z=1.959964,p=k/N,den=1+z^2/N,c=(p+z^2/(2*N))/den,h=z*sqrt(max(0,p*(1-p)/N+z^2/(4*N^2)))/den);[c-h,c+h]};
ciprop(k,N)={if(N==0,return([0.,1.,"NA"]));if(k==0,return([0.,1-0.025^(1/N),"CP0"]));my(w=wilson(k,N));[w[1],w[2],"Wilson"]};
fmt(x)=if(type(x)=="t_REAL",if(x==0,"0",concat(strsplit(strprintf("%.7g",x)," "))),if(type(x)=="t_FRAC",fmt(1.*x),Str(x)));
tsvs(v)={my(s="");for(i=1,#v,s=Str(s,fmt(v[i]),if(i<#v,"\t","")));s};
nTof(c)=[0,1,3][c%3+1];
PRIMES=Map([12,4;16,4;20,3;24,2]);
FBS=["interval","random","twistmult","farey"];
FB3=["random","interval","farey"];

\\ ---------------------------------------------------------------- hierarchical percentile bootstrap (PREREG section 6/7)
\\ U = Vec over primes of Vec over curves of [numvec,denvec] (vectors over targets). Statistic sum(num)/sum(den).
\\ resample primes with replacement, then curves within each drawn prime, then targets within each drawn curve.
\\ returns [point, lo, hi, nvalid]; point/lo/hi are 0 with nvalid=0 when undefined
hboot(U)={my(K=#U,res=vector(BOOT),cnt=0,sn,sd,pk,nc,cu,T,idx,pn=0,pd=0,r);
  if(K==0,return([0.,0.,0.,0]));
  for(i=1,K,for(j=1,#U[i],pn+=vecsum(U[i][j][1]);pd+=vecsum(U[i][j][2])));
  for(b=1,BOOT,sn=0;sd=0;
    for(i=1,K,pk=U[1+random(K)];nc=#pk;
      for(j=1,nc,cu=pk[1+random(nc)];T=#cu[1];
        if(T>0,idx=vector(T,u,1+random(T));sn+=vecsum(vecextract(cu[1],idx));sd+=vecsum(vecextract(cu[2],idx)))));
    if(sd>0,cnt++;res[cnt]=1.*sn/sd));
  if(cnt<10||pd==0,return([0.,0.,0.,cnt]));
  res=vecsort(res[1..cnt]);
  [1.*pn/pd,res[max(1,round(0.025*cnt))],res[min(cnt,round(0.975*cnt))],cnt]};

\\ ================================================================ grid summary
HDR=["tag","bits","pidx","cidx","m","fb","alpha","q","cardE","cardEt","gcdodd","nT","curveflag","n","nE","nW","nT2","targets_done","targets_planned","truncated","E_nd","E_deg","M_nd","M_deg","S_nd","S_deg","target_mismatches","targets_xR_in_V","meanE_nd","B_naive","B_ref","ratio_naive","ratio_ref","f_mixed","ESS_K","ESS_V","f_lo","f_hi","f_method","c_hat","c_ref","chat_over_cref","chat_over_1.5","Sref_per_target","Sref_expected_total","E1_literal_fire","tab_s","loop_s","status"];
print(tsvs(HDR));
POOL=Map();
CURVE=Map();
allrows=List();notrun=List();incomplete=List();dups=List();
jobs=readlines(Str(RAW,"/joblist.txt"));
finalize(cfg,Nv,Mv,eDeg,mDeg,sNd,sDeg,mis,xin,done,endf)={
  my(tag=cfg[2],bits=num(cfg[3]),pidx=num(cfg[4]),cidx=num(cfg[5]),m=num(cfg[6]),fb=cfg[7],alpha=cfg[8],q=num(cfg[9]),cE=num(cfg[12]),cEt=num(cfg[13]),go=num(cfg[14]),nT=num(cfg[15]),flag=cfg[16],
     n=num(cfg[17]),nE=num(cfg[18]),nW=num(cfg[19]),nT2=num(cfg[20]),NT=num(cfg[23]),Bn,Br,mean,f,fr,cr,Sr,ESSK,ESSV,ESS,var,u2,sN,sN2,sM,w,fci,fq,trunc,tabs,loops,status,eNd,mNd,lit,fm,nsz,Nt,Mt,ch,ch15,sexp);
  eNd=vecsum(Nv)-vecsum(Mv);mNd=vecsum(Mv);
  if(endf==0,trunc=-1;tabs=-1;loops=-1;status="INCOMPLETE",trunc=num(endf[7]);tabs=endf[5];loops=endf[6];status=if(trunc,"TRUNCATED","ok"));
  Bn=1.*n^m/(m!*q);Br=if(nE>0,1.*2^m*nE^m/(m!*cE),0.);
  mean=if(done>0,1.*eNd/done,0.);
  f=if(mNd+eNd>0,1.*mNd/(mNd+eNd),0.);
  fr=if(nT>0&&nE>0,1.5*nT*(1.*nW/nE)^2*(1.*cE/q)/q,0.);       \\ f_ref = c_ref nT/q
  cr=if(nE>0,1.5*(1.*nW/nE)^2*(1.*cE/q),0.);
  Sr=if(nT>0&&nE>0&&m==3,2.*nT*nW^2*nE/q^2,0.);                    \\ S_ref per target (PREREG section 4)
  sN=vecsum(Nv);sN2=sum(i=1,#Nv,1.*Nv[i]^2);
  ESSK=if(sN2>0,1.*sN^2/sN2,0.);
  u2=sum(i=1,#Nv,(1.*Mv[i]-f*Nv[i])^2);
  var=if(done>1&&sN>0,(1.*done/(done-1))*u2/sN^2,0.);
  ESSV=if(var>0,f*(1-f)/var,ESSK);
  ESS=min(ESSK,ESSV);if(ESS<=0,ESS=ESSK);
  if(sN==0,fci=[0.,1.,"NA"],
    if(mNd==0,fci=[0.,3./ESSK,"rule3_ESSK"],w=wilson(f*ESS,ESS);fci=[w[1],w[2],"Wilson_ESS"]));
  fq=if(nT>0&&m==3&&sN>0,f*q/nT,"NA");
  ch=if(fr>0&&sN>0,f/fr,"NA");ch15=if(nT>0&&m==3&&sN>0,f*q/nT/1.5,"NA");
  sexp=Sr*done;
  lit=if(eNd>=4*q&&sN>0&&wilson(mNd,mNd+eNd)[1]>5./q,1,0);
  listput(allrows,[tag,bits,pidx,cidx,m,fb,alpha,q,cE,cEt,go,nT,flag,n,nE,nW,nT2,done,NT,trunc,eNd,eDeg,mNd,mDeg,sNd,sDeg,mis,xin,mean,Bn,Br,if(Bn>0,mean/Bn,0.),if(Br>0,mean/Br,0.),f,ESSK,ESSV,fci[1],fci[2],fci[3],fq,cr,ch,ch15,Sr,sexp,lit,tabs,loops,status]);
  print(tsvs(allrows[#allrows]));
  \\ pooled by stratum (m,nT,fb)
  my(key=Str(m,"|",nT,"|",fb),cur);
  if(!mapisdefined(POOL,key,&cur),cur=vector(11));
  cur+=[1,eNd,eDeg,mNd,mDeg,sNd,sDeg,mis,done,lit,if(status=="ok",0,1)];
  mapput(POOL,key,cur);
  \\ per-curve per-target vectors for the hierarchical bootstrap (m=3, nT>0)
  if(m==3&&nT>0&&nE>0&&done>0,
    my(ck=Str(bits,"|",pidx,"|",cidx,"|",fb),obj,Mx_,Dr,Dn);
    if(!mapisdefined(CURVE,ck,&obj),obj=[vector(100),vector(100),vector(100),0.,0,0,0]);
    Mx_=obj[1];Dr=obj[2];Dn=obj[3];
    for(t=1,done,Mx_[t]+=Mv[t];Dr[t]+=Nv[t]*fr;Dn[t]+=Nv[t]*1.5*nT/q);
    obj=[Mx_,Dr,Dn,obj[4]+Sr*done,obj[5]+mNd,obj[6]+sNd,obj[7]+done];
    mapput(CURVE,ck,obj));
  0};

{for(ji=1,#jobs,
  my(jf=strsplit(jobs[ji]," "),fn,lines,cfg=0,Nv,Mv,eDeg,mDeg,sNd,sDeg,mis,xin,done,v,a,b,c,d,e,g);
  fn=Str(RAW,"/grid_",jf[1],"_",jf[2],"_",jf[3],"_m",jf[4],"_",jf[5],".out");
  lines=readlines(fn);
  if(#lines==0,listput(notrun,jobs[ji]);next);
  for(li=1,#lines,
    v=sp(lines[li]);
    if(v[1]=="CFG",
      if(cfg!=0,finalize(cfg,Vec(Nv),Vec(Mv),eDeg,mDeg,sNd,sDeg,mis,xin,done,0);listput(incomplete,cfg[2]));
      cfg=v;Nv=List();Mv=List();eDeg=0;mDeg=0;sNd=0;sDeg=0;mis=0;xin=0;done=0);
    if(v[1]=="TGT"&&cfg!=0&&#v>=10,
      a=num(v[4]);b=num(v[5]);c=num(v[6]);d=num(v[7]);e=num(v[8]);g=num(v[9]);
      eDeg+=b;mDeg+=d;sNd+=e;sDeg+=g;if(c!=e||d!=g,mis++);xin+=num(v[10]);listput(Nv,a+c);listput(Mv,c);done++);
    if(v[1]=="END"&&cfg!=0,finalize(cfg,Vec(Nv),Vec(Mv),eDeg,mDeg,sNd,sDeg,mis,xin,done,v);cfg=0);
    if(v[1]=="DUP",listput(dups,lines[li])));
  if(cfg!=0,finalize(cfg,Vec(Nv),Vec(Mv),eDeg,mDeg,sNd,sDeg,mis,xin,done,0);listput(incomplete,cfg[2])));};

\\ ================================================================ pooled strata (m,nT,fb)
PF=Str(OUT,"/pooled.tsv");
write(PF,tsvs(["m","nT","fb","configs","E_nd","E_deg","M_nd","M_deg","S_nd","S_deg","target_mismatches","targets","E1_literal_fire_configs","nonok_configs","f_mixed","f_lo","f_hi","f_method"]));
PM=Mat(POOL);
{for(r=1,matsize(PM)[1],
  my(key=PM[r,1],c=PM[r,2],kf=strsplit(key,"|"),mm=num(kf[1]),ci);
  ci=ciprop(c[4],c[4]+c[2]);
  write(PF,tsvs([mm,num(kf[2]),kf[3],c[1],c[2],c[3],c[4],c[5],c[6],c[7],c[8],c[9],c[10],c[11],if(c[4]+c[2]>0,1.*c[4]/(c[4]+c[2]),0.),ci[1],ci[2],ci[3]])));};

\\ grid totals / falsifier lists
tot=vector(8);F1l=List();F2l=List();F3l=List();Litl=List();NE_nT0_m3=0;m2mis=0;
{for(i=1,#allrows,
  my(r=allrows[i]);
  tot[1]++;tot[2]+=r[18];
  if(r[5]==3,tot[3]+=r[27];if(r[27]>0,listput(F3l,r[1])));
  if(r[5]==2,m2mis+=r[27]);
  if(r[5]==2&&r[23]+r[24]>0,tot[4]++;listput(F1l,r[1]));
  if(r[5]==3&&r[12]==0&&r[23]>0,tot[5]++;listput(F2l,r[1]));
  if(r[5]==3&&r[12]==0,NE_nT0_m3+=r[21]);
  if(r[46]==1,tot[6]++;listput(Litl,r[1]));
  if(r[20]!=0||r[49]!="ok",tot[7]++));};

\\ ================================================================ stage 0 / controls (G0, G1, G2, G4)
CF=Str(OUT,"/controls_summary.tsv");
s0=readlines(Str(RAW,"/stage0_oracle.out"));
s1=readlines(Str(RAW,"/stage1_controls.out"));
v0n=0;v0bad=0;v0solutions=0;v0mixnd=0;v0mixdeg=0;toyn=0;toybad=0;toyMnd=0;toyMdeg=0;rcbad=0;rcn=0;v0res="MISSING";
{for(i=1,#s0,my(v=sp(s0[i]));
  if(v[1]=="V0",v0n++;if(num(v[#v])!=1,v0bad++);v0solutions+=num(v[7]);v0mixnd+=num(v[13]);v0mixdeg+=num(v[14]));
  if(v[1]=="V0TOY",toyn++;if(num(v[#v])!=1,toybad++);toyMnd+=num(v[13]);toyMdeg+=num(v[14]));
  if(v[1]=="RESCHECK",rcn++;rcbad+=num(v[4]));
  if(v[1]=="V0_RESULT",v0res=v[2]));};
G0=(v0res=="PASS"&&v0bad==0&&toybad==0&&rcbad==0&&v0n==640&&toyn==320);
write(CF,tsvs(["#stage0","V0_rows",v0n,"V0_rows_expected",640,"V0_bad",v0bad,"V0_solutions_A",v0solutions,"V0_mixed_nd_A",v0mixnd,"V0_mixed_deg_A",v0mixdeg,"TOY_rows",toyn,"TOY_rows_expected",320,"TOY_bad",toybad,"TOY_mixed_nd_A",toyMnd,"TOY_mixed_deg_A",toyMdeg,"rescheck_lines",rcn,"rescheck_bad",rcbad,"V0_RESULT",v0res,"G0",if(G0,"PASS","FAIL")]));
G=Map();
gadd(key,vals)={my(cur);if(!mapisdefined(G,key,&cur),cur=vector(#vals));cur+=vals;mapput(G,key,cur);0};
pc1n=0;pc1f=0;pc1ok=0;pc2n=0;pc2f=0;pc2ok=0;pc3n=0;pc3f=0;pc3ok=0;nc1n=0;nc1ok=0;nc1mix=0;nc1bn=0;nc1bok=0;nc1bmix=0;nc3n=0;nc3bad=0;nc3mm=0;nc3m2mixed=0;
c1res=List();
{for(i=1,#s1,my(v=sp(s1[i]),t=v[1]);
  if(t=="PC1",pc1n++;pc1f+=num(v[6]);pc1ok+=num(v[#v]);gadd(Str(t,"|bits=",v[2],"|curve=",v[3]),[1,num(v[6]),num(v[#v])]));
  if(t=="PC2",pc2n++;pc2f+=num(v[6]);pc2ok+=num(v[#v]);gadd(Str(t,"|bits=",v[2],"|curve=",v[3]),[1,num(v[6]),num(v[#v])]));
  if(t=="PC3",pc3n++;pc3f+=num(v[6]);pc3ok+=num(v[#v]);gadd(Str(t,"|bits=",v[2],"|curve=",v[3],"|m=",v[4]),[1,num(v[6]),num(v[#v])]));
  if(t=="NC1",nc1n++;nc1ok+=num(v[#v]);nc1mix+=num(v[12])+num(v[13]);gadd(Str(t,"|bits=",v[2],"|curve=",v[3],"|m=",v[4]),[1,num(v[#v]),num(v[12])+num(v[13]),num(v[6])]));
  if(t=="NC1big",nc1bn++;nc1bok+=num(v[#v]);nc1bmix+=num(v[10])+num(v[11]);gadd(Str(t,"|bits=",v[2],"|curve=",v[3],"|m=",v[4]),[1,num(v[#v]),num(v[10])+num(v[11]),num(v[7])]));
  if(t=="NC3",nc3n++;my(mm_=num(v[6]),bad_=0);if(num(v[16])!=0,bad_=1;nc3mm+=num(v[16]));if(mm_==2&&num(v[12])+num(v[13])>0,bad_=1;nc3m2mixed+=num(v[12])+num(v[13]));nc3bad+=bad_;
    gadd(Str(t,"|bits=",v[2],"|curve=",v[3],"|",v[5],"|m=",v[6]),[1,num(v[10]),num(v[11]),num(v[12]),num(v[13]),num(v[14]),num(v[15]),num(v[16])]));
  if(t=="CONTROLS_RESULT",listput(c1res,Str(v[2]," ",v[3]," ",v[4]))));};
GM=Mat(G);
write(CF,"#group\tcount\tfound_or_ok\t...");
{for(r=1,matsize(GM)[1],write(CF,Str(GM[r,1],"\t",tsvs(GM[r,2]))));};
G1=(pc1n==800&&pc2n==600&&pc3n==1200&&pc1f==pc1n&&pc2f==pc2n&&pc3f==pc3n&&pc1ok==pc1n&&pc2ok==pc2n&&pc3ok==pc3n);
G2=(nc1n==480&&nc1bn==840&&nc1ok==nc1n&&nc1bok==nc1bn&&nc1mix==0&&nc1bmix==0);
G4=(nc3n==72&&nc3bad==0);
write(CF,tsvs(["#PC","PC1_trials",pc1n,"PC1_found",pc1f,"PC1_ok",pc1ok,"PC2_trials",pc2n,"PC2_found",pc2f,"PC2_ok",pc2ok,"PC3_trials",pc3n,"PC3_found",pc3f,"PC3_ok",pc3ok,"expected_PC1_PC2_PC3","800/600/1200","G1",if(G1,"PASS","FAIL")]));
write(CF,tsvs(["#NC1","NC1_lines",nc1n,"NC1_ok",nc1ok,"NC1_mixed",nc1mix,"NC1big_lines",nc1bn,"NC1big_ok",nc1bok,"NC1big_mixed",nc1bmix,"expected_lines","480/840","G2",if(G2,"PASS","FAIL")]));
write(CF,tsvs(["#NC3","NC3_lines",nc3n,"NC3_expected",72,"NC3_bad_lines",nc3bad,"NC3_target_mismatches",nc3mm,"NC3_m2_mixed",nc3m2mixed,"G4",if(G4,"PASS","FAIL")]));
write(CF,Str("#CONTROLS_RESULT_lines\t",Vec(c1res)));

\\ ================================================================ G3: NC2-calibrated (hierarchical bootstrap)
setrand(S0+1);
NCU=Map();NCM=Map();   \\ NCU: key bits|fb|m -> Map pidx|cidx -> [numvec,denvec] ; accumulate cell stats in NCM
nclist=readlines(Str(RAW,"/nc2list.txt"));
nc_missing=List();nc_trunc=List();
PIC=Map();
pival(ntt,nW,m)={my(key=[ntt,nW,m],r);if(mapisdefined(PIC,key,&r),return(r));
  r=1.-1.*binomial(ntt-nW,m)/binomial(ntt,m);mapput(PIC,key,r);r};
{for(li=1,#nclist,
  my(jf=strsplit(nclist[li]," "),fn,lines,v,tag,numv=List(),denv=List(),ckey,cur,ntl,sumpi=0,cnt=0,sw=0,swpi=0,trunc=0,seenend=0,
     bits=num(jf[1]),pidx=num(jf[2]),cidx=num(jf[3]),mm=num(jf[4]),fb=jf[5],e,mt,N,pi,key);
  fn=Str(RAW,"/nc2_",jf[1],"_",jf[2],"_",jf[3],"_m",jf[4],"_",jf[5],".out");
  lines=readlines(fn);
  if(#lines==0,listput(nc_missing,nclist[li]);next);
  for(k=1,#lines,v=sp(lines[k]);
    if(v[1]=="NCT"&&#v>=7,e=num(v[4]);mt=num(v[5]);N=e+mt;pi=pival(num(v[6]),num(v[7]),mm);
      listput(numv,mt-N*pi);listput(denv,N);cnt++;sumpi+=pi;sw+=N;swpi+=N*pi);
    if(v[1]=="NCEND",seenend=1;if(num(v[7])!=0,listput(nc_trunc,nclist[li]))));
  if(!seenend,listput(nc_missing,Str(nclist[li]," (no NCEND)")));
  key=Str(bits,"|",fb,"|",mm);
  if(!mapisdefined(NCU,key,&cur),cur=Map());
  mapput(cur,Str(pidx,"|",cidx),[Vec(numv),Vec(denv)]);
  mapput(NCU,key,cur);
  if(!mapisdefined(NCM,key,&cur),cur=vector(6));
  cur+=[cnt,sumpi,sw,swpi,0,1];mapput(NCM,key,cur));};
NCF=Str(OUT,"/nc2_cells.tsv");
write(NCF,tsvs(["bits","fb","m","units_found","units_expected","targets","N_nd_total","mean_pi_targets","mean_pi_weighted","D","D_lo","D_hi","boot_valid","pass","reason"]));
nc2pass=0;nc2fail=List();nc2cells=0;nc2maxabs=0.;
{for(bi=1,4,my(bits=[12,16,20,24][bi],K=mapget(PRIMES,bits));
  for(fi=1,4,for(mm=2,3,
    my(fb=FBS[fi],key=Str(bits,"|",fb,"|",mm),cur,U=List(),nu=0,exp_=K*6,hb,mpi,mpw,pass,reason="",st,Dpt);
    nc2cells++;
    if(!mapisdefined(NCU,key,&cur),
      write(NCF,tsvs([bits,fb,mm,0,exp_,0,0,0.,0.,0.,0.,0.,0,0,"MISSING"]));listput(nc2fail,key);next);
    for(p=0,K-1,my(P=List());for(c=0,5,my(o);if(mapisdefined(cur,Str(p,"|",c),&o),listput(P,o);nu++));if(#P>0,listput(U,Vec(P))));
    st=mapget(NCM,key);
    hb=hboot(Vec(U));
    mpi=if(st[1]>0,st[2]/st[1],0.);mpw=if(st[3]>0,st[4]/st[3],0.);
    pass=1;
    if(nu<exp_,pass=0;reason=Str(reason,"incomplete_units;"));
    if(hb[4]<10,pass=0;reason=Str(reason,"bootstrap_undefined;"));
    if(hb[2]< -0.05||hb[3]>0.05,pass=0;reason=Str(reason,"interval_outside_margin;"));
    if(mpi<0.3,pass=0;reason=Str(reason,"power_condition;"));
    if(pass,nc2pass++,listput(nc2fail,key));
    nc2maxabs=max(nc2maxabs,max(abs(hb[2]),abs(hb[3])));
    write(NCF,tsvs([bits,fb,mm,nu,exp_,st[1],st[3],mpi,mpw,hb[1],hb[2],hb[3],hb[4],pass,if(reason=="","ok",reason)])))));};
G3=(nc2pass==32&&nc2cells==32&&#nc_missing==0);

\\ ================================================================ resolution stage (R20, R24)
RES=Map();   \\ key bits|pidx|cidx|fb -> [Mvec,Dref,Dnaive,Snd,Mnd,targets,Endtot,Mdeg,Sdeg,mism,trunc_blocks,blocks,nW,nE,cE,q,nT]
reslist=readlines(Str(RAW,"/reslist.txt"));
res_missing=List();res_trunc=List();res_mism=0;res_targets=0;
{for(li=1,#reslist,
  my(jf=strsplit(reslist[li]," "),fn,lines,v,cfg=0,Ml=List(),Dr=List(),Dn=List(),sNd=0,mNd=0,eNd=0,mDeg=0,sDeg=0,mis=0,done=0,seenend=0,tr=0,key,cur,q,nT,nE,nW,cE,fr,fn_,N,bits,pidx,cidx,fb);
  bits=jf[1];pidx=jf[2];cidx=jf[3];fb=jf[4];
  fn=Str(RAW,"/res_",jf[1],"_",jf[2],"_",jf[3],"_",jf[4],"_b",jf[5],".out");
  lines=readlines(fn);
  if(#lines==0,listput(res_missing,reslist[li]);next);
  for(k=1,#lines,v=sp(lines[k]);
    if(v[1]=="RCFG",cfg=v;q=num(v[9]);cE=num(v[12]);nT=num(v[15]);nE=num(v[18]);nW=num(v[19]);
      fr=if(nE>0,1.5*nT*(1.*nW/nE)^2*(1.*cE/q)/q,0.);fn_=1.5*nT/q);
    if(v[1]=="R"&&cfg!=0&&#v>=7,N=num(v[3])+num(v[4]);listput(Ml,num(v[4]));listput(Dr,N*fr);listput(Dn,N*fn_);
      eNd+=num(v[3]);mNd+=num(v[4]);sNd+=num(v[5]);mDeg+=num(v[6]);sDeg+=num(v[7]);
      if(num(v[4])!=num(v[5])||num(v[6])!=num(v[7]),mis++);done++);
    if(v[1]=="REND",seenend=1;if(num(v[7])!=0,tr=1;listput(res_trunc,reslist[li]))));
  if(!seenend||cfg==0,listput(res_missing,Str(reslist[li]," (no REND)"));next);
  res_mism+=mis;res_targets+=done;
  key=Str(bits,"|",pidx,"|",cidx,"|",fb);
  if(!mapisdefined(RES,key,&cur),cur=[[],[],[],0,0,0,0,0,0,0,0,0,nW,nE,cE,q,nT]);
  cur[1]=concat(cur[1],Vec(Ml));cur[2]=concat(cur[2],Vec(Dr));cur[3]=concat(cur[3],Vec(Dn));
  cur[4]+=sNd;cur[5]+=mNd;cur[6]+=done;cur[7]+=eNd;cur[8]+=mDeg;cur[9]+=sDeg;cur[10]+=mis;cur[11]+=tr;cur[12]+=1;
  mapput(RES,key,cur));};
\\ build units for resolution strata: bits, list of fbs, kind (2=ref,3=naive); also returns totals [Snd,Mnd,targets,Mdeg,Sdeg,mism,units]
mkunitsR(bits,fbs,kind)={my(U=List(),tot=vector(7),K=mapget(PRIMES,bits));
  for(p=0,K-1,my(P=List());
    for(c=0,11,my(have=0,Mv=0,Dv=0,o);
      for(f=1,#fbs,if(mapisdefined(RES,Str(bits,"|",p,"|",c,"|",fbs[f]),&o),
        if(!have,Mv=o[1];Dv=o[kind];have=1,Mv+=o[1];Dv+=o[kind]);
        if(kind==2,tot[1]+=o[4];tot[2]+=o[5];tot[3]+=o[6];tot[4]+=o[8];tot[5]+=o[9];tot[6]+=o[10];tot[7]++)));
      if(have,listput(P,[Mv,Dv])));
    if(#P>0,listput(U,Vec(P))));
  [Vec(U),tot]};

\\ build units for main-grid strata: bits, nT list, fb list, kind (2=ref,3=naive); returns [U, Sref_expected, S_nd, M_nd, targets, curves]
mkunitsG(bits,nTs,fbs,kind)={my(U=List(),K=mapget(PRIMES,bits),sref=0.,snd=0,mnd=0,tg=0,cv=0,nTset=Set(nTs));
  for(p=0,K-1,my(P=List());
    for(c=0,11,if(!setsearch(nTset,nTof(c)),next);
      my(have=0,Mv=vector(100),Dv=vector(100),o);
      for(f=1,#fbs,if(mapisdefined(CURVE,Str(bits,"|",p,"|",c,"|",fbs[f]),&o),
        Mv+=o[1];Dv+=o[kind];have=1;if(kind==2,sref+=o[4];snd+=o[6];mnd+=o[5];tg+=o[7])));
      if(have,listput(P,[Mv,Dv]);if(kind==2,cv++)));
    if(#P>0,listput(U,Vec(P))));
  [Vec(U),sref,snd,mnd,tg,cv]};

\\ ================================================================ F4 table
setrand(S0+1);
F4F=Str(OUT,"/f4_table.tsv");
write(F4F,tsvs(["stratum","source","bits","nT","fb","units","targets","M_nd_obs","S_nd_struct","Sref_expected_or_S_nd","resolved","ratio_cref","lo","hi","boot_valid","ratio_naive1.5","lo_n","hi_n","inside_0.8_1.25","excludes_0.8_1.25","above2","below0.5"]));
F4rows=List();
f4row(name,src,bits,nTs,fbs)={my(g2,g3,b2,b3,res,thr,inside,excl,ab,be,exp_,snd,mnd,tg,nu,tgs);
  if(src=="grid",
    g2=mkunitsG(bits,nTs,fbs,2);b2=hboot(g2[1]);g3=mkunitsG(bits,nTs,fbs,3);b3=hboot(g3[1]);
    exp_=g2[2];snd=g2[3];mnd=g2[4];tg=g2[5];nu=g2[6];res=(exp_>=5);thr=exp_,
    g2=mkunitsR(bits,fbs,2);b2=hboot(g2[1]);g3=mkunitsR(bits,fbs,3);b3=hboot(g3[1]);
    snd=g2[2][1];mnd=g2[2][2];tg=g2[2][3];nu=g2[2][7];thr=snd;
    res=if(bits==20,snd>=20,snd>=10));
  inside=if(b2[4]>=10&&b2[2]>=0.8&&b2[3]<=1.25,1,0);
  excl=if(b2[4]>=10&&(b2[3]<0.8||b2[2]>1.25),1,0);
  ab=if(b2[4]>=10&&b2[2]>2,1,0);be=if(b2[4]>=10&&b2[3]<0.5,1,0);
  my(row=[name,src,bits,if(#nTs==1,nTs[1],"1+3"),if(#fbs==1,fbs[1],"pool3"),nu,tg,mnd,snd,thr,if(res,1,0),b2[1],b2[2],b2[3],b2[4],b3[1],b3[2],b3[3],inside,excl,ab,be]);
  write(F4F,tsvs(row));listput(F4rows,row);row};
{for(bi=1,4,my(bits=[12,16,20,24][bi]);
  f4row(Str("G",bits,"_pooled"),"grid",bits,[1,3],FB3);
  for(nti=1,2,my(nT=[1,3][nti]);
    f4row(Str("G",bits,"_nT",nT,"_pool3"),"grid",bits,[nT],FB3);
    for(fi=1,4,f4row(Str("G",bits,"_nT",nT,"_",FBS[fi]),"grid",bits,[nT],[FBS[fi]]))));};
f4row("R20_random","res",20,[3],["random"]);
f4row("R20_farey","res",20,[3],["farey"]);
f4row("R20_random+farey","res",20,[3],["random","farey"]);
f4row("R20_twistmult","res",20,[3],["twistmult"]);
f4row("R24_random","res",24,[3],["random"]);
\\ lookup by name
f4get(name)={for(i=1,#F4rows,if(F4rows[i][1]==name,return(F4rows[i])));0};
\\ category (section 7): bits-level pooled strata at 12,16,20 (20: grid if resolved, else R20 random+farey if resolved)
cat_row(bits)={my(r=f4get(Str("G",bits,"_pooled")));
  if(bits==20&&r[11]==0,my(r2=f4get("R20_random+farey"));if(r2[11]==1,return(concat(r2,["R20_fallback"]))));
  concat(r,["grid"])};
C12=cat_row(12);C16=cat_row(16);C20=cat_row(20);
F4cat="UNDETERMINED";F4catnote="";
{if(C16[11]==0||C20[11]==0||C12[11]==0,
    F4catnote=Str("unresolved: 12:",C12[11]," 16:",C16[11]," 20:",C20[11]);F4cat="UNDETERMINED",
  if(C12[19]==1&&C16[19]==1&&C20[19]==1,F4cat="A",
    if(C16[19]==1&&C20[19]==1,F4cat="B",
      F4cat="C";F4catnote=Str("16-bit inside=",C16[19]," excludes=",C16[20],"; 20-bit inside=",C20[19]," excludes=",C20[20],if(C16[20]==1||C20[20]==1," ; interval excludes the band: H3 falsified"," ; interval overlaps the band: H3 not confirmed, not refuted")))));};
\\ F4 (refutation of Rt): resolved bits-level grid strata at 16 and 20 bits and resolved R20 strata with the interval entirely above 2 or below 0.5
F4fired=List();
{for(i=1,#F4rows,my(r=F4rows[i],nm=r[1]);
  if(r[11]==1&&(r[21]==1||r[22]==1)&&(nm=="G16_pooled"||nm=="G20_pooled"||nm=="R20_random"||nm=="R20_farey"||nm=="R20_random+farey"),listput(F4fired,nm)));};
R20res=(f4get("R20_random")[11]==1&&f4get("R20_farey")[11]==1);
R24res=(f4get("R24_random")[11]==1);

\\ ================================================================ G5
g5lines=readlines(Str(REP,"/g5.txt"));
g5rep=readlines(Str(REP,"/joblist.txt"));
g5ok=0;g5bad=0;
{for(i=1,#g5lines,my(tk=strsplit(g5lines[i]," "));if(#tk>=2&&tk[1]=="G5"&&tk[2]=="OK",g5ok++);if(#tk>=2&&tk[1]=="G5"&&tk[2]=="MISMATCH",g5bad++));};
G5=(#g5rep>0&&g5ok==#g5rep&&g5bad==0);

\\ ================================================================ verdict
F3total=tot[3]+res_mism;
F1=(tot[4]>0);F2=(tot[5]>0);F3=(F3total>0);F4=(#F4fired>0);
admissible=(G0&&G1);
G1ok=G1;
verdict="INCONCLUSIVE";why=List();
if(!G0,listput(why,"G0 FAIL"));if(!G1,listput(why,"G1 FAIL"));if(!G2,listput(why,"G2 FAIL"));if(!G3,listput(why,"G3 FAIL"));if(!G4,listput(why,"G4 FAIL"));if(!G5,listput(why,"G5 FAIL"));
if(!R20res,listput(why,"R20 UNRESOLVED"));
if(NE_nT0_m3<10^7,listput(why,"N_E_nd(nT=0,m=3) < 1e7"));
if(#notrun>0,listput(why,Str(#notrun," grid jobs not run")));
if(#nc_missing>0,listput(why,Str(#nc_missing," nc2 jobs missing")));
if(#res_missing>0,listput(why,Str(#res_missing," res jobs missing")));
if(#incomplete>0,listput(why,Str(#incomplete," configs incomplete")));
{if(F1||F2||F3||F4,if(admissible,verdict="DECISIVE_REFUTATION",verdict="VOID_G0_G1_FAILED"),
  if(G0&&G1&&G2&&G3&&G4&&G5&&R20res&&NE_nT0_m3>=10^7&&#notrun==0&&#nc_missing==0&&#res_missing==0&&#incomplete==0,verdict="DECISIVE_SUPPORT",verdict="INCONCLUSIVE"))};

VF=Str(OUT,"/verdict.txt");
write(VF,Str("verdict\t",verdict));
write(VF,Str("reasons_not_decisive\t",Vec(why)));
write(VF,Str("G0\t",if(G0,"PASS","FAIL"),"\tG1\t",if(G1,"PASS","FAIL"),"\tG2\t",if(G2,"PASS","FAIL"),"\tG3\t",if(G3,"PASS","FAIL"),"\tG4\t",if(G4,"PASS","FAIL"),"\tG5\t",if(G5,"PASS","FAIL")));
write(VF,Str("G3_cells_passed\t",nc2pass,"/",nc2cells,"\tfailing_cells\t",Vec(nc2fail)));
write(VF,Str("G3_max_abs_interval_end_over_cells\t",fmt(nc2maxabs)));
write(VF,Str("G5_reruns_expected\t",#g5rep,"\tok\t",g5ok,"\tmismatch\t",g5bad));
write(VF,Str("F1_configs_with_mixed_m2\t",tot[4],"\t",Vec(F1l)));
write(VF,Str("F2_configs_with_nd_mixed_m3_nT0\t",tot[5],"\t",Vec(F2l)));
write(VF,Str("F3_target_mismatches_grid_m3\t",tot[3],"\tres\t",res_mism,"\ttotal\t",F3total,"\tm2_target_mismatches(=F1 counts)\t",m2mis));
write(VF,Str("F4_fired_strata\t",Vec(F4fired)));
write(VF,Str("F4_category\t",F4cat,"\t",F4catnote));
write(VF,Str("F4_cat_rows\t12:",C12[12]," [",C12[13],",",C12[14],"] res=",C12[11],"\t16:",C16[12]," [",C16[13],",",C16[14],"] res=",C16[11],"\t20:",C20[12]," [",C20[13],",",C20[14],"] res=",C20[11]," src=",C20[23]));
write(VF,Str("R20_resolved\t",R20res,"\tR24_resolved\t",R24res));
write(VF,Str("E1_literal_fire_configs\t",tot[6],"\tof_m3_configs"));
write(VF,Str("N_E_nd_nT0_m3\t",NE_nT0_m3));
write(VF,Str("configs\t",tot[1],"\ttargets\t",tot[2],"\tjobs_not_run\t",#notrun,"\tconfigs_incomplete\t",#incomplete,"\tconfigs_truncated_or_nonok\t",tot[7]));
write(VF,Str("jobs_not_run_list\t",Vec(notrun)));
write(VF,Str("nc2_missing\t",Vec(nc_missing),"\tnc2_truncated\t",Vec(nc_trunc)));
write(VF,Str("res_missing\t",Vec(res_missing),"\tres_truncated\t",Vec(res_trunc),"\tres_targets\t",res_targets));

\\ ================================================================ results.json
JF=Str(OUT,"/results.json");
js(s)=Str("\"",s,"\"");
jb(x)=if(x,"true","false");
write(JF,"{");
write(JF,Str("\"verdict\":",js(verdict),",\"seed\":",S0,",\"bootstrap_B\":",BOOT,","));
write(JF,Str("\"gates\":{\"G0\":",jb(G0),",\"G1\":",jb(G1),",\"G2\":",jb(G2),",\"G3\":",jb(G3),",\"G4\":",jb(G4),",\"G5\":",jb(G5),"},"));
write(JF,Str("\"G3_cells_passed\":",nc2pass,",\"G3_cells\":",nc2cells,",\"G3_max_abs_interval_end\":",fmt(nc2maxabs),","));
write(JF,Str("\"controls\":{\"PC1\":",pc1n,",\"PC1_found\":",pc1f,",\"PC1_ok\":",pc1ok,",\"PC2\":",pc2n,",\"PC2_found\":",pc2f,",\"PC2_ok\":",pc2ok,",\"PC3\":",pc3n,",\"PC3_found\":",pc3f,",\"PC3_ok\":",pc3ok,",\"NC1_lines\":",nc1n,",\"NC1big_lines\":",nc1bn,",\"NC1_mixed\":",nc1mix+nc1bmix,",\"NC3_lines\":",nc3n,",\"NC3_bad\":",nc3bad,",\"V0_rows\":",v0n,",\"V0_bad\":",v0bad,",\"TOY_rows\":",toyn,",\"TOY_bad\":",toybad,"},"));
write(JF,Str("\"grid\":{\"configs\":",tot[1],",\"targets\":",tot[2],",\"jobs_not_run\":",#notrun,",\"incomplete\":",#incomplete,",\"truncated_or_nonok\":",tot[7],",\"F1_configs\":",tot[4],",\"F2_configs\":",tot[5],",\"F3_mismatch_targets\":",tot[3],",\"E1_literal_fire\":",tot[6],",\"N_E_nd_nT0_m3\":",NE_nT0_m3,"},"));
write(JF,Str("\"resolution\":{\"targets\":",res_targets,",\"mismatch_targets\":",res_mism,",\"R20_resolved\":",jb(R20res),",\"R24_resolved\":",jb(R24res),",\"missing\":",#res_missing,",\"truncated\":",#res_trunc,"},"));
write(JF,Str("\"F4\":{\"category\":",js(F4cat),",\"fired\":",#F4fired,",\"note\":",js(F4catnote),"},"));
write(JF,"\"f4_table\":[");
{for(i=1,#F4rows,my(r=F4rows[i]);
  write(JF,Str("{\"stratum\":",js(r[1]),",\"source\":",js(r[2]),",\"bits\":",r[3],",\"units\":",r[6],",\"targets\":",r[7],",\"M_nd\":",r[8],",\"S_nd\":",r[9],",\"expected_or_S\":",fmt(r[10]),",\"resolved\":",r[11],",\"ratio_cref\":",fmt(r[12]),",\"lo\":",fmt(r[13]),",\"hi\":",fmt(r[14]),",\"ratio_naive\":",fmt(r[16]),",\"lo_n\":",fmt(r[17]),",\"hi_n\":",fmt(r[18]),",\"inside\":",r[19],",\"excludes\":",r[20],"}",if(i<#F4rows,",",""))));};
write(JF,"],");
write(JF,"\"strata\":[");
{for(r=1,matsize(PM)[1],
  my(key=PM[r,1],c=PM[r,2],kf=strsplit(key,"|"),mm=num(kf[1]),ci=ciprop(c[4],c[4]+c[2]));
  write(JF,Str("{\"m\":",mm,",\"nT\":",kf[2],",\"fb\":",js(kf[3]),",\"configs\":",c[1],",\"E_nd\":",c[2],",\"E_deg\":",c[3],",\"M_nd\":",c[4],",\"M_deg\":",c[5],",\"S_nd\":",c[6],",\"S_deg\":",c[7],",\"target_mismatches\":",c[8],",\"targets\":",c[9],",\"f_lo\":",fmt(ci[1]),",\"f_hi\":",fmt(ci[2]),",\"f_method\":",js(ci[3]),"}",if(r<matsize(PM)[1],",","")));
);};
write(JF,"]}");
