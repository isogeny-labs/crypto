\\ opcount_verify.gp -- instrumented re-run of Detector A on ONE config to verify the analytic operation counts in opcounts.sh
\\ env: E1_STAGE=none (loads e1_census.gp definitions only), E1_BITS,E1_PIDX,E1_CURVE,E1_M,E1_FB,E1_ALPHA_IDX(1..3),E1_TARGETS
default(parisizemax,2000000000);
\\ run as: gp -q e1_census.gp opcount_verify.gp (read() is not used)
NPK=0;NKEYS=0;NFIND=0;NHITS=0;NMAPL=0;
pkeys0=pkeys; tabfind0=tabfind;
pkeys(xi,xj,ci,cj,ri,rj)={my(r=pkeys0(xi,xj,ci,cj,ri,rj));NPK++;NKEYS+=#r;r};
tabfind(tab,key)={my(r=tabfind0(tab,key));NFIND++;NHITS+=#r;r};
opc()={
my(bits=geni("E1_BITS",12),pidx=geni("E1_PIDX",0),cidx=geni("E1_CURVE",0),mm=geni("E1_M",3),fb=genv("E1_FB","random"),
   ai=geni("E1_ALPHA_IDX",1),NT=geni("E1_TARGETS",100),als=[1/2,13/20,4/5],cap=if(mm==2,16384,1500),
   W,E,Ed,T,V,vd,CL,RS,Mx,tab,n,xR,codes,cA_,tot=[0,0,0,0],pk_build,pk_tgt,find_tgt,hits_tgt,nsol=0);
W=world(bits,pidx,cidx);E=W[9];Ed=W[10];T=mktargets(E,W[4],W[13],200);
n=alphaN(als[ai],cap);
V=mkfb(fb,n,E,Ed,W[13]);vd=vdata(V,CA,CB);CL=vd[1];RS=vd[2];Mx=mkmap(V);
NPK=0;NKEYS=0;
if(mm==3,tab=mktab(V,CL,RS));
pk_build=NPK;
print("OPC_BUILD\tn=",#V,"\tpkeys_calls_table_build=",pk_build,"\ttable_entries=",if(mm==3,#TK,0));
NPK=0;NKEYS=0;NFIND=0;NHITS=0;
for(t=1,NT,xR=T[t][1];
  codes=runA(mm,tab,Mx,V,CL,RS,xR,1,sqrtI(fx(xR)));
  cA_=classify(codes,mm,#V,V,CL,1,xR);for(u=1,4,tot[u]+=cA_[u]));
print("OPC_TARGETS\tm=",mm,"\ttargets=",NT,"\tpkeys_calls=",NPK,"\tkeys_returned=",NKEYS,"\ttable_lookups=",NFIND,"\traw_join_hits=",NHITS,"\tdistinct_solutions=",vecsum(tot),"\tE_nd,E_deg,M_nd,M_deg=",tot);
}
opc();
