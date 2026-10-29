\\ e2_run.gp -- one cell/run of E2-twin-target-amortization.  Env: E2_BITS E2_MU E2_CURVE E2_SEED E2_ARMS(all|core)
default(parisizemax,2000000000);
default(realprecision,38);
read("e2_lib.gp");
bits=e2_envnum("E2_BITS",22); mu=e2_envnum("E2_MU",1.5); cidx=e2_envnum("E2_CURVE",1); sd=e2_envnum("E2_SEED",1);
arms=e2_env("E2_ARMS","core");
doall=(arms=="all");
tt=e2_curves(bits,max(cidx,2)); p=tt[1]; cv=tt[2][cidx]; a=cv[1]; b=cv[2]; lE=cv[3]; lP=cv[4];
d=e2_nonres(p);
N=round(mu*p^(1/3)); W=ceil(0.5*N*log(N));
s0=1000*bits+10*cidx+sd; s0b=s0+100000;
if(N>=4096, error("N too large for key packing"));
print("HEADER,gp_version=",version(),",bits=",bits,",mu=",mu,",curve=",cidx,",seed=",sd,",arms=",arms);
print("HEADER,p=",p,",a=",a,",b=",b,",nE=",lE,",nEp=",lP,",lE=",lE,",lEp=",lP,",d=",d,",N=",N,",W=",W);
print("HEADER,curve_search_seed=",20261021+bits,",s0=",s0,",s0_second_stream=",s0b);

Ec=ellinit([a,b],p); Epc=ellinit([a*d^2%p,b*d^3%p],p);
fgen(Ecc,aa,bb,dd,typ)={ my(x=p\2,X); while(1, x++; X=if(typ==0,x,x); if(kronecker((X^3+aa*X+bb)%p,p)==1, return([Mod(X,p),ellordinate(Ecc,X)[1]]))) };
PE=fgen(Ec,a,b,d,0);
PP=fgen(Epc,a*d^2%p,b*d^3%p,d,1);
dinv=Mod(1,p)/d;

\\ ---- scan
t0=getabstime();
fb=e2_scan(a,b,p,N); VE=fb[1]; VP=fb[2]; scanE=fb[3]; scanP=fb[4];
tscan=getabstime()-t0; print("PHASE,scan_ms=",tscan,",scanE=",scanE,",scanEp=",scanP);

\\ ---- tables
t0=getabstime(); hE=e2_half(VE,a,b,p,0,N); tbE=getabstime()-t0; print("PHASE,buildE_ms=",tbE,",K=",hE[2],",SQ=",hE[3]);
t0=getabstime(); hP=e2_half(VP,a,b,p,1,N); tbP=getabstime()-t0; print("PHASE,buildEp_ms=",tbP,",K=",hP[2],",SQ=",hP[3]);
\\ NOTE: for the E' class the abscissa x (E'-type) is used with the same x-only formulas for f(x)=x^3+ax+b (shared by E and E').
KE=hE[2]; SQE=hE[3]; KP=hP[2]; SQP=hP[3];
t0=getabstime(); SE=vecsort(hE[1]); tsE=getabstime()-t0; nE=#SE;
t0=getabstime(); SP=vecsort(hP[1]); tsP=getabstime()-t0; nP=#SP;
t0=getabstime(); SJ=vecsort(concat(hE[1],hP[1])); tsJ=getabstime()-t0; nJ=#SJ;
hE=0; hP=0;
print("PHASE,sortE_ms=",tsE,",sortEp_ms=",tsP,",sortJ_ms=",tsJ,",nE=",nE,",nEp=",nP,",nJ=",nJ);
cmpE=nE*e2_clog2(nE); cmpP=nP*e2_clog2(nP); cmpJ=nJ*e2_clog2(nJ);

\\ ---- lifts (verification only)
ptsE=e2_lift(Ec,VE,0,d,p); ptsP=e2_lift(Epc,VP,1,d,p);

\\ ---- distinct keys, caps
dkE=e2_distinct(SE); dkP=e2_distinct(SP);
yE=dkE/((lE-1)/2); yP=dkP/((lP-1)/2);
TmaxE=ceil(6*W/yE); TmaxP=ceil(6*W/yP);
print("PHASE,distinctE=",dkE,",distinctEp=",dkP,",yhatE=",yE*1.,",yhatEp=",yP*1.,",TmaxE=",TmaxE,",TmaxEp=",TmaxP);

\\ ---- collections (E, E' ; E-only array and joint array probed on the same target stream)
t0=getabstime(); cE=e2_collect(Ec,PE,0,0,SE,SJ,Ec,ptsE,s0,W,TmaxE,0,1); tcE=getabstime()-t0;
print("PHASE,collectE_ms=",tcE,",T=",cE[1],",probesA=",cE[2],",probesJ=",cE[3],",relA=",cE[4],",relJ=",cE[5],",hitT=",cE[6],",unver=",cE[7],",cross=",cE[8],",cap=",cE[9]);
t0=getabstime(); cP=e2_collect(Epc,PP,1,dinv,SP,SJ,Epc,ptsP,s0,W,TmaxP,0,1); tcP=getabstime()-t0;
print("PHASE,collectEp_ms=",tcP,",T=",cP[1],",probesA=",cP[2],",probesJ=",cP[3],",relA=",cP[4],",relJ=",cP[5],",hitT=",cP[6],",unver=",cP[7],",cross=",cP[8],",cap=",cP[9]);
eqE=(Vec(cE[10])==Vec(cE[11])); eqP=(Vec(cP[10])==Vec(cP[11]));
gE=e2_graph(cE[10],N); gP=e2_graph(cP[10],N);
rkE=-1; if(N<=700, rkE=e2_rank(cE[10],N,lE));
print("PHASE,graphE_cols=",gE[1],",comps=",gE[2],",selfloops=",gE[3],",rankE=",rkE,",graphEp_cols=",gP[1],",comps=",gP[2]);

\\ ---- PC1, NC1, J0 (arms=all)
cE2=[-1,-1,-1,-1,-1,-1,-1,-1,-1]; tpc=0;
nc=[-1,-1,-1,-1,-1,-1,-1,-1,-1]; tnc=0; hasnc=0;
J0=[-1,-1,-1,-1,-1,-1]; tj0=0;
{
if(doall,
  t0=getabstime(); cE2=e2_collect(Ec,PE,0,0,SE,0,Ec,ptsE,s0b,W,TmaxE,0,1); tpc=getabstime()-t0;
  print("PHASE,PC1_ms=",tpc,",T2=",cE2[1],",probes2=",cE2[2],",rel2=",cE2[4],",cap=",cE2[9]);
  if(cidx==1,
    hasnc=1; cv2=tt[2][2]; a2=cv2[1]; b2=cv2[2]; Ec2=ellinit([a2,b2],p); P2=fgen(Ec2,a2,b2,d,0);
    t0=getabstime(); nc=e2_collect(Ec2,P2,0,0,SE,0,Ec,ptsE,s0,0,0,cE[1],0); tnc=getabstime()-t0;
    print("PHASE,NC1_ms=",tnc,",curve2=(",a2,",",b2,"),T=",nc[1],",probes=",nc[2],",hits_targets=",nc[6],",verified=",nc[4],",unverified=",nc[7]);
  );
  if(cidx==1 && bits<=26,
    t0=getabstime(); K0=0; L0=0; acc=0; zer=0; rej=0;
    for(i=1,N, my(xi=VE[i]);
      for(j=1,N, my(xj=VP[j], iv, Dv, sx, pm, s, pr, ds, kr);
        iv=Mod(xi-xj,p)^-1; Dv=iv^2; K0++;
        sx=xi+xj; pm=xi*xj;
        s=2*((pm+a)*sx+2*b)*Dv; pr=((pm-a)^2-4*b*sx)*Dv;
        ds=lift(s^2-4*pr); kr=kronecker(ds,p); L0++;
        if(kr==1,acc++, if(kr==0,zer++,rej++))));
    tj0=getabstime()-t0;
    \\ same-type pairs (i<j) would each also need a Legendre test in the unpartitioned arm: N(N-1) tests (analytic count)
    J0=[K0,L0+N*(N-1),acc,zer,rej,N*(N-1)];
    print("PHASE,J0_ms=",tj0,",mixedK=",K0,",mixedLegendre=",L0,",mixed_with_Fp_root=",acc,",mixed_zero_disc=",zer,",mixed_rejected=",rej);
  );
);
}

\\ ---- RESULT line
R=List();
add(nm,v)={ listput(R,[nm,v]) };
add("bits",bits); add("mu",mu*1.); add("curve",cidx); add("seed",sd); add("p",p); add("N",N); add("W",W);
add("lE",lE); add("lEp",lP); add("d",d); add("s0",s0);
add("scanE",scanE); add("scanEp",scanP); add("scanJ",max(scanE,scanP));
add("KE",KE); add("SQE",SQE); add("KEp",KP); add("SQEp",SQP);
add("nkeysE",nE); add("nkeysEp",nP); add("nkeysJ",nJ); add("cmpE",cmpE); add("cmpEp",cmpP); add("cmpJ",cmpJ);
add("startE",e2_startcost(s0)); add("startEb",e2_startcost(s0b));
add("TE",cE[1]); add("prE_A",cE[2]); add("prE_J",cE[3]); add("relE",cE[4]); add("relE_J",cE[5]); add("hitTE",cE[6]); add("unverE",cE[7]); add("crossE",cE[8]); add("capE",cE[9]);
add("TEp",cP[1]); add("prEp_A",cP[2]); add("prEp_J",cP[3]); add("relEp",cP[4]); add("relEp_J",cP[5]); add("hitTEp",cP[6]); add("unverEp",cP[7]); add("crossEp",cP[8]); add("capEp",cP[9]);
add("eqE",eqE); add("eqEp",eqP);
add("distE",dkE); add("distEp",dkP);
add("colsE",gE[1]); add("compsE",gE[2]); add("selfE",gE[3]); add("rankE",rkE); add("colsEp",gP[1]); add("compsEp",gP[2]);
add("T2",cE2[1]); add("pr2",cE2[2]); add("rel2",cE2[4]); add("cap2",cE2[9]);
add("hasNC1",hasnc); add("ncT",nc[1]); add("ncPr",nc[2]); add("ncHitT",nc[6]); add("ncVer",nc[4]); add("ncUnver",nc[7]);
add("j0K",J0[1]); add("j0Leg",J0[2]); add("j0Facc",J0[3]); add("j0Zero",J0[4]); add("j0Rej",J0[5]);
add("ms_scan",tscan); add("ms_buildE",tbE); add("ms_buildEp",tbP); add("ms_sort",tsE+tsP+tsJ); add("ms_collE",tcE); add("ms_collEp",tcP); add("ms_pc1",tpc); add("ms_nc1",tnc); add("ms_j0",tj0);
hdr="RESULT_HEADER"; vals="RESULT";
for(u=1,#R, hdr=Str(hdr,",",R[u][1]); vals=Str(vals,",",R[u][2]));
print(hdr); print(vals);
quit;
