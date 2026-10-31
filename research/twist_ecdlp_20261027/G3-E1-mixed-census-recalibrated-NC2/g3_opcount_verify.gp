\\ g3_opcount_verify.gp -- instrumented check of the operation-count formulas (added after the frozen scripts; see DEVIATIONS.md B1)
\\ Run as: G3_STAGE=none gp -q g3_census.gp g3_opcount_verify.gp  (loads the frozen g3_census.gp), wraps pkeys with a call counter, and for a few small configs counts
\\ S_3-root-set evaluations (pkeys calls) in table build and per target for the production detector A. Output: OPV lines.
PK=0;
pkeys0=pkeys;
pkeys(xi,xj,ci,cj,ri,rj)={PK++;pkeys0(xi,xj,ci,cj,ri,rj)};
{
 my(cfgs=[[12,0,0,3,"random",0.5],[12,0,2,3,"farey",0.65],[12,1,2,2,"interval",0.65],[12,0,1,2,"random",0.5],[16,0,2,3,"random",0.5]],NT=20,W,E,Ed,T,V,vd,CL,RS,Mx,tab,n,xR,p0,pt,codes,
    bits,pidx,cidx,mm,fb,al,cap);
 for(c=1,#cfgs,
  bits=cfgs[c][1];pidx=cfgs[c][2];cidx=cfgs[c][3];mm=cfgs[c][4];fb=cfgs[c][5];al=cfgs[c][6];
  W=world(bits,pidx,cidx);E=W[9];Ed=W[10];T=mktargets(E,W[4],W[13],200);
  cap=if(mm==2,16384,1500);n=min(ceil(Q^al),cap);
  V=mkfb(fb,n,E,Ed,W[13]);vd=vdata(V,CA,CB);CL=vd[1];RS=vd[2];Mx=mkmap(V);
  PK=0;if(mm==3,tab=mktab(V,CL,RS));p0=PK;
  PK=0;
  for(t=1,NT,xR=T[t][1];codes=runA(mm,tab,Mx,V,CL,RS,xR,1,sqrtI(fx(xR))));
  pt=PK;
  print("OPV\tbits=",bits,"\tpidx=",pidx,"\tcidx=",cidx,"\tm=",mm,"\tfb=",fb,"\tn=",#V,"\ttargets=",NT,"\ttable_pkeys_counted=",p0,"\ttable_pkeys_formula=",if(mm==3,#V*(#V+1)/2,0),
        "\ttarget_pkeys_counted=",pt,"\ttarget_pkeys_formula=",#V*NT,"\tOK=",if(p0==if(mm==3,#V*(#V+1)/2,0)&&pt==#V*NT,1,0)));
}
