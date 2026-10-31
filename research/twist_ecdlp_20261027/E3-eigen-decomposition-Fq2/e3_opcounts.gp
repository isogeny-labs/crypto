\\ e3_opcounts.gp -- operation-count metrics derived ONLY from recorded out/results.csv and out/semaev.csv (no new experiment).
\\ Not part of the frozen hash set (added after the run, DEVIATIONS.md item 7). Reads: OUTDIR/results.csv, OUTDIR/semaev.csv. Writes: OUTDIR/opcounts.csv.
\\ Definitions (per cell, T targets):
\\  solves_D   = T*binomial(NE,m-1)        univariate root-solves, direct Semaev (degree 2^(m-1) over F_p, E-type x only)
\\  solves_X   = T*binomial(N,m2-1)        univariate root-solves, F_{p^2} route (degree 2^(m2-1), all of V)
\\  tuples_D   = binomial(NE,m), tuples_X = binomial(N,m2)   candidate x-sets in the table arm (shared by all T targets)
\\  signsum_D  = tuples_D*2^m, signsum_X = tuples_X*2^m2      group-sum (sign-vector) evaluations in the table arm
\\  work_D/work_X = solves * degree (the preregistered Semaev unit) -- must equal T*WD, T*WX from results.csv
\\  per-relation: work / relations (= Z), solves / relations, signsum / relations.
OUTDIR=getenv("E3OUT"); if(OUTDIR==0 || OUTDIR=="", OUTDIR="out");
rows(fn)={ my(v=List(), s); for(i=1,10^6, s=externstr(Str("sed -n ",i,"p ",fn)); if(#s==0||#s[1]=="", break); listput(v,s[1])); Vec(v) };
fld(s)=strsplit(s,",");
R=rows(Str(OUTDIR,"/results.csv"));
out=Str(OUTDIR,"/opcounts.csv"); system(Str("rm -f ",out));
write(out,"fam,m,p,c,N,NE,T,Dtot,Xgen_tot,solves_D,solves_X,work_D,work_X,work_D_matches_WD,work_X_matches_WX,tuples_D,tuples_X,signsum_D,signsum_X,solves_per_Erel_D,solves_per_Erel_Xgen,signsum_per_Erel_D,signsum_per_Erel_Xgen,solves_ratio_gen,signsum_ratio_gen,work_ratio_gen");
h=fld(R[1]); ix(nm)={for(i=1,#h,if(h[i]==nm,return(i))); error("col ",nm)};
nR=#R; bad=0;
{for(r=2,nR,
  my(f=fld(R[r])); my(g(nm)=eval(f[ix(nm)]));
  my(fam=f[1], m=g("m"), p=g("p"), c=g("c"), N=g("N"), NE=g("NE"), T=g("T"), Dt=g("Dtot"), Xt=g("Xgen_tot"), WD=g("WD"), WX=g("WX"));
  my(m2=2*m, sD=T*binomial(NE,m-1), sX=T*binomial(N,m2-1), wD=sD*2^(m-1), wX=sX*2^(m2-1));
  my(tD=binomial(NE,m), tX=binomial(N,m2), gD=tD*2^m, gX=tX*2^m2);
  my(okD=(wD==T*WD), okX=(wX==T*WX)); if(!okD||!okX, bad++);
  my(a1=if(Dt>0, sD/Dt, -1), a2=if(Xt>0, sX/Xt, -1), b1=if(Dt>0, gD/Dt, -1), b2=if(Xt>0, gX/Xt, -1));
  \\ per-relation costs are per-relation of the whole T-target batch (as in Z); table-arm sums are shared by the T targets
  my(r1=if(Dt>0&&Xt>0, a2/a1, -1), r2=if(Dt>0&&Xt>0, b2/b1, -1), r3=if(Dt>0&&Xt>0, (wX/Xt)/(wD/Dt), -1));
  write(out,fam,",",m,",",p,",",c,",",N,",",NE,",",T,",",Dt,",",Xt,",",sD,",",sX,",",wD,",",wX,",",okD,",",okX,",",tD,",",tX,",",gD,",",gX,",",
    if(a1<0,-1,a1*1.0),",",if(a2<0,-1,a2*1.0),",",if(b1<0,-1,b1*1.0),",",if(b2<0,-1,b2*1.0),",",if(r1<0,-1,r1*1.0),",",if(r2<0,-1,r2*1.0),",",if(r3<0,-1,r3*1.0)));
}
print("opcounts rows=",nR-1," work-counter mismatches vs results.csv=",bad);
quit;
