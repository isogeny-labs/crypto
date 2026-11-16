default(parisizemax, 2000000000);
\\ ops_table.gp -- OPERATION COUNTS straight from the raw trial records (no bootstrap, no statistics): per (arm,b,inst) cell and pooled over b>=28.
\\ rec fields (run_cell.gp): [arm,b,inst,t,a, ok(6), T(7), ..., M(13), S(14), I(15), ..., cand(17)]; counted cost = M + 0.8 S + (I/M) I.
RAW = getenv("RAWDIR"); if(RAW == 0, RAW = "raw");
OUT = getenv("OUTDIR"); if(OUT == 0, OUT = "out");
CELLREC = List(); HDRM = Map();
rec(v) = listput(CELLREC, v);
hdr(k, v) = mapput(HDRM, k, v);
curve(v) = 0; chk(n, v) = 0;
files = vecsort(externstr(Str("ls ", RAW)));
F = Str(OUT, "/opcounts.csv"); system(Str("rm -f ", F));
write(F, "arm,b,inst,trials,solved,total_steps,total_M,total_S,total_I,total_cand_tuples,mean_steps_per_trial,mean_M_per_trial,mean_S_per_trial,mean_I_per_trial,cost_Mequiv_IM3_per_trial,cost_Mequiv_IM10_per_trial,cost_Mequiv_IM30_per_trial,cost_Mequiv_IM100_per_trial");
POOL = Map();
{
foreach(files, f,
  if(#f > 9 && Vecsmall(f)[1..5] == Vecsmall("cell_") && Vecsmall(f)[#f-3..#f] == Vecsmall(".txt"),
    CELLREC = List(); HDRM = Map();
    iferr(read(Str(RAW, "/", f)), E, next);
    my(c, R = Vec(CELLREC), nt = #R, ok = 0, T = 0, M = 0, S = 0, I = 0, C = 0, x);
    if(!mapisdefined(HDRM, "cell", &c) || nt == 0, next);
    for(t = 1, nt, ok += R[t][6]; T += R[t][7]; M += R[t][13]; S += R[t][14]; I += R[t][15]; C += R[t][17]);
    write(F, Str(c[1], ",", c[2], ",", c[3], ",", nt, ",", ok, ",", T, ",", M, ",", S, ",", I, ",", C, ",", strprintf("%.1f,%.1f,%.1f,%.1f", T/nt, M/nt, S/nt, I/nt), ",",
       strprintf("%.1f,%.1f,%.1f,%.1f", (M+0.8*S+3*I)/nt, (M+0.8*S+10*I)/nt, (M+0.8*S+30*I)/nt, (M+0.8*S+100*I)/nt)));
    if(c[2] >= 28 && (c[3] <= 2) && (c[1] == "kpure" || c[1] == "kmix" || c[1] == "a0" || c[1] == "kpureT"),
      x = [0,0,0,0,0,0]; mapisdefined(POOL, c[1], &x);
      mapput(POOL, c[1], x + [nt, T, M, S, I, C]))));
}
print("pooled over b>=28, instances 1,2, headline arms (raw record sums; NOT normalised by sqrt(n), so b-mix weighted by trial counts 60/40/24/12):");
print("arm trials total_steps total_M total_S total_I total_cand mean_steps mean_M mean_S mean_I");
{
foreach(["a0", "kpure", "kmix", "kpureT"], a,
  my(x);
  if(mapisdefined(POOL, a, &x),
    print(a, " ", x[1], " ", x[2], " ", x[3], " ", x[4], " ", x[5], " ", x[6], " ", strprintf("%.1f %.1f %.1f %.1f", x[2]/x[1], x[3]/x[1], x[4]/x[1], x[5]/x[1]))));
}
