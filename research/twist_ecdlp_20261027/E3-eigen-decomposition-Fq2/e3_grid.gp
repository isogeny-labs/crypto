\\ e3_grid.gp -- grid (PREREGISTRATION section 7) and csv helpers shared by model/tables/analyze
genv(n,def)={my(v=getenv(n)); if(type(v)!="t_STR" || v=="", def, v)}
OUTDIR=genv("E3OUT","out");
GRID_A_P=[101,211,401]; GRID_A_N=[16,24,32,40]; GRID_A_M=2;
GRID_B_P=[101,211];     GRID_B_N=[12,16,20];    GRID_B_M=3;
TNUM=200;
\\ E3DEV=1 switches to a tiny toolchain-test grid (p=61, not part of the experiment)
if(genv("E3DEV","0")=="1", GRID_A_P=[61]; GRID_A_N=[12,16]; GRID_B_P=[61]; GRID_B_N=[12]);
read_csv(fn) = {
  my(L=readstr(fn), out=List());
  for(i=2,#L, if(L[i]=="", next); my(s=strsplit(L[i],",")); listput(out, vector(#s,k,eval(s[k]))));
  Vec(out)
}
famP(fam) = if(fam=="A", GRID_A_P, GRID_B_P)
famN(fam) = if(fam=="A", GRID_A_N, GRID_B_N)
famM(fam) = if(fam=="A", GRID_A_M, GRID_B_M)
curve_row(p,c) = {
  my(R=read_csv(Str(OUTDIR,"/curves.csv")));
  for(i=1,#R, if(R[i][1]==p && R[i][2]==c, return(R[i])));
  error("curve not found")
}
\\ fixed-format float for csv
fmt(x) = Strprintf("%.8g", x)
