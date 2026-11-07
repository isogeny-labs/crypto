\\ g2_opcounts.gp -- exact OPERATION COUNTS per arm, derived from the recorded out/tables_*.csv (no new sampling).
\\ Primary cost metric: root-solves (the Semaev work unit; W_D(j)=C(N_E,j-1)*2^(j-1), loop-counted == formula in tables CSV),
\\ candidate (j-1)-tuples enumerated = C(n,j-1), sign vectors per tuple = 2^(j-1), root-solves = tuples*signs.
\\ Totals multiply by the T targets of the cell (every target needs one full enumeration; same as the work accounting in analysis).
\\ Projection (F_{p^2}) group operations per target are a MODEL count (textbook double-and-add on Ci2 = 1/2 mod #E*#E'):
\\   2 elladd + 1 ellneg + 2 scalar mults (each bitlen-1 doublings + popcount-1 additions) + 2 Frobenius powerings (not group ops).
\\ Reads recorded values only. Writes out/opcounts.csv and prints a summary.
default(parisizemax,2000000000); default(realprecision,15);
wformula(n,r) = if(r>n || r<0, 0, binomial(n,r)*2^r);
OUT="out";
fileexists(fn)=iferr(readstr(fn)!=0, e, 0);
tabfiles=List();
{
foreach(["A","B"],f,
  foreach([101,211,401],p,
    foreach([1,2,3,4,9],c,
      my(fn=Str(OUT,"/tables_p",p,"_c",c,"_",f,".csv"));
      if(fileexists(fn), listput(tabfiles,fn))));
);
}
split(s)=strsplit(s,",");
ncol(H,name)={for(i=1,#H,if(H[i]==name,return(i))); error("column ",name)};
cfn=Str(OUT,"/opcounts.csv"); system(Str("rm -f ",cfn));
write(cfn,"fam,p,c,N,m,T,NE,NT,arm,j,k,candidate_tuples_per_target,rootsolves_per_target,rootsolves_total,ratio_to_Dm_rootsolves,proj_group_ops_per_target_model");
curves=readstr(Str(OUT,"/curves.csv"));
{
n_rows=0; n_chk=0; nbad=0;
foreach(Vec(tabfiles),fn,
  my(L=readstr(fn), H=split(L[1]));
  for(r=2,#L,
    my(v=split(L[r]), g(nm)=eval(v[ncol(H,nm)]));
    my(fam=v[3], p=g("p"), c=g("c"), m=g("m"), N=g("N"), T=g("T"), NE=g("NE"), NT=g("NT"));
    my(Cl=g("l"), Clp=g("lp"), Ci2=lift(1/Mod(2,Cl*Clp)));
    my(bl=#binary(Ci2), pc=hammingweight(Ci2));
    my(sm=2*(bl-1+pc-1));          \\ model: two scalar mults by Ci2, (bl-1) dbl + (pc-1) add each
    my(pg=2+1+sm);                 \\ + 2 elladd + 1 ellneg
    my(Wref=wformula(NE,m-1));
    \\ consistency: recorded loop-counted work == closed form
    n_chk++; if(g("WDm_code")!=Wref || g("WDm2_code")!=wformula(NE,2*m-1) || g("WTm_code")!=wformula(NT,m-1), nbad++);
    my(emit(arm,j,k,tup,rs,pgo)=write(cfn,fam,",",p,",",c,",",N,",",m,",",T,",",NE,",",NT,",",arm,",",j,",",k,",",tup,",",rs,",",rs*T,",",strprintf("%.9f",rs/Wref),",",pgo));
    emit("D_m (baseline)",m,0,binomial(NE,m-1),wformula(NE,m-1),0);
    emit("D_2m (same term count)",2*m,0,binomial(NE,2*m-1),wformula(NE,2*m-1),0);
    emit("P class S (j=m)",m,0,binomial(NE,m-1),wformula(NE,m-1),pg);
    emit("PF (j=m;k=1)",m,1,binomial(NE,m-1)+binomial(NT,0),wformula(NE,m-1)+wformula(NT,0),pg);
    emit("PF (j=m;k=m)",m,m,binomial(NE,m-1)+binomial(NT,m-1),wformula(NE,m-1)+wformula(NT,m-1),pg);
    if(g("WX_code")>=0,
      emit("X coupled",2*m,0,binomial(N,2*m-1),g("WX_code"),0));
    n_rows++;
  );
);
print("tables read: ",#tabfiles," curve-cells: ",n_chk," loop-vs-formula mismatches in recorded W: ",nbad);
}

quit;
