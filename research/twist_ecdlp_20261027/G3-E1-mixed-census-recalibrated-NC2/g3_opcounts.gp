\\ g3_opcounts.gp -- operation counts of the production detector A per stage, from recorded CFG/END lines (added after the frozen scripts; DEVIATIONS.md B1).
\\ Primary cost metric = number of S_3 root-set evaluations ("pkeys" calls; each is a constant number of field multiplications, 1 inversion, <=1 sqrt-free combination)
\\ with formulas verified against an instrumented counter (raw/opcount_verify.out):
\\   table build (m=3 only): n(n+1)/2 pkeys calls ; per target: n pkeys calls (m=2 and m=3) ; so  total = [m==3]*n(n+1)/2 + n*targets_done.
\\ Wall seconds (tabsec, loopsec as printed by the jobs) are SECONDARY and contention-affected (4 cores, ~20 agents). Oracle S (comparator) work is not counted.
\\ Output: ops.tsv (stdout); writes nothing else.
default(parisizemax,2000000000);
RAW=getenv("G3_RAW");if(type(RAW)!="t_STR"||RAW=="",RAW="raw");
num(s)=eval(s);
tsvs(v)={my(s="");for(i=1,#v,s=Str(s,v[i],if(i<#v,"\t","")));s};
files(pat)=externstr(Str("ls ",RAW,"/",pat," 2>/dev/null"));
\\ stage spec: [label, glob, cfgkey, endkey, bitsidx, midx(0=fixed 3), nidx]
SPEC=[["grid","grid_*.out","CFG","END"],["nc2","nc2_*.out","NCCFG","NCEND"],["res","res_*.out","RCFG","REND"]];
print(tsvs(["stage","bits","m","configs","targets_done","n_sum","table_pkeys","target_pkeys","total_pkeys","table_wall_s","loop_wall_s"]));
T=Map();
nidx(s)=if(s==2,18,17);
scanfile(s,fn)={my(L=iferr(readstr(fn),E,[]),cfg=[],p,bits,m,n,d,key,v);
  for(k=1,#L,p=strsplit(L[k],"\t");
    if(p[1]==SPEC[s][3],cfg=[num(p[3]),num(p[6]),num(p[nidx(s)])]);
    if(p[1]==SPEC[s][4]&&#cfg>0,
      bits=cfg[1];m=cfg[2];n=cfg[3];d=num(p[3]);key=[SPEC[s][1],bits,m];
      if(!mapisdefined(T,key,&v),v=vector(7));
      v[1]++;v[2]+=d;v[3]+=n;v[4]+=if(m==3,n*(n+1)/2,0);v[5]+=n*d;v[6]+=1.*num(p[5]);v[7]+=1.*num(p[6]);
      mapput(T,key,v);cfg=[]));
  0};
for(s=1,#SPEC,fl=files(SPEC[s][2]);for(i=1,#fl,scanfile(s,fl[i])));
M=Mat(T);
ORD=vecsort(vector(matsize(M)[1],r,[M[r,1][1],M[r,1][2],M[r,1][3],r]));
TOT=vector(7);JS=List();
{for(r=1,#ORD,v=M[ORD[r][4],2];key=M[ORD[r][4],1];TOT+=v;
  print(tsvs([key[1],key[2],key[3],v[1],v[2],v[3],v[4],v[5],v[4]+v[5],strprintf("%.1f",v[6]),strprintf("%.1f",v[7])]));
  listput(JS,Str("{\"stage\":\"",key[1],"\",\"bits\":",key[2],",\"m\":",key[3],",\"configs\":",v[1],",\"targets_done\":",v[2],",\"table_pkeys\":",v[4],",\"target_pkeys\":",v[5],",\"total_pkeys\":",v[4]+v[5],",\"table_wall_s\":",strprintf("%.1f",v[6]),",\"loop_wall_s\":",strprintf("%.1f",v[7]),"}")));}
{print(tsvs(["ALL","-","-",TOT[1],TOT[2],TOT[3],TOT[4],TOT[5],TOT[4]+TOT[5],strprintf("%.1f",TOT[6]),strprintf("%.1f",TOT[7])]));
OJ=getenv("G3_OPS_JSON");
if(type(OJ)=="t_STR"&&OJ!="",
  write(OJ,Str("{\"unit\":\"S3 root-set evaluations (pkeys calls); table=[m==3]*n(n+1)/2, target=n per target; formulas verified in raw/opcount_verify.out\",\"wall_note\":\"wall seconds are contention-affected, secondary\",\"rows\":[",strjoin(Vec(JS),",\n"),"],\"total_pkeys\":",TOT[4]+TOT[5],",\"total_table_pkeys\":",TOT[4],",\"total_target_pkeys\":",TOT[5],",\"total_targets\":",TOT[2],"}")));}
