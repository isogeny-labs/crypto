default(parisizemax, 2000000000);
\\ opcounts_g1.gp -- POST-FREEZE ADDENDUM (see DEVIATIONS.md #2).  Not part of the frozen analysis; reads only raw/cell_*.txt, e5_inputs/cell_*.txt,
\\ out/solve.csv, out/ratios.csv, out/RC.txt.  Computes the operation-count tables (primary cost metric) and writes out/opcounts.csv, out/opcounts.txt
\\ and results.json.  No new random numbers, no new walks.
CELLS = Map(); CURHDR = Map(); CELLREC = List();
hdr(key, v) = mapput(CURHDR, key, v);
rec(v) = listput(CELLREC, v);
curve(v) = 0;
startswith(s, pre) = #s >= #pre && Vecsmall(s)[1..#pre] == Vecsmall(pre);
LB = [1, 1, 2, 2, 3, 3, 4, 4];
{
foreach(["raw", "e5_inputs"], dir,
  my(files = externstr(Str("ls ", dir)));
  foreach(files, f,
    if(startswith(f, "cell_") && #f > 4 && Vecsmall(f)[#f - 3..#f] == Vecsmall(".txt"),
      CURHDR = Map(); CELLREC = List();
      iferr(read(Str(dir, "/", f)), E, print("WARN truncated read ", dir, "/", f));
      my(c, mm, ms = 0, cur);
      if(mapisdefined(CURHDR, "cell", &c),
        my(arm = c[1], key = [c[1], c[2], c[3]]);
        mapisdefined(CURHDR, "Marm", &mm); mapisdefined(CURHDR, "done_ms", &ms);
        if(mapisdefined(CELLS, key, &cur),
          mapput(CELLS, key, [mm, concat(cur[2], Vec(CELLREC)), cur[3] + ms]),
          mapput(CELLS, key, [mm, Vec(CELLREC), ms]))))));
}
cell(arm, b, i) = my(c); if(mapisdefined(CELLS, [arm, b, i], &c), c, 0);
INSTS = [1, 2, 3, 4]; BS = [24, 28, 32, 36, 40];
ARMS = ["n1", "n1r64", "n1sab", "n1c2", "a0", "kpure", "kmix"];
\\ pooled per (arm, b): trials solved steps M S I adds dbls esc2 escB dps trails lad ms R
LINES = List(); CSVL = List(); OPC = Map();
system("rm -f out/opcounts.csv");   \\ idempotent: write() appends, so a re-run in a tree holding old output duplicated the table (DEVIATIONS.md #6)
write("out/opcounts.csv", "arm,b,instances,trials,solved,steps,mean_steps_per_trial,R_mean_over_instances,M,S,I,adds_or_ladder_steps,doublings_or_ladder_iters,escapes_2cycle,escapes_brent,dp_found,trails_started,dp_collisions,native_SEL_DPT_per_step,native_CAN,native_CYC,native_LOOK,native_START,native_LAD,cell_wall_ms_contended");
{
foreach(ARMS, arm,
  foreach(BS, b,
    my(nt = 0, ns = 0, st = 0, M = 0, S = 0, I = 0, na = 0, nd = 0, e2 = 0, eb = 0, dps = 0, tr = 0, cand = 0, lad = 0, ms = 0, ni = 0, Rs = 0);
    foreach(INSTS, i, my(c = cell(arm, b, i));
      if(c && #c[2] > 0,
        ni++; ms += c[3]; my(sti = 0);
        foreach(c[2], r,
          nt++; ns += r[6]; st += r[7]; sti += r[7]; M += r[13]; S += r[14]; I += r[15]; dps += r[12]; tr += r[11]; cand += r[17];
          if(arm == "kpure" || arm == "kmix", for(j = 1, 8, lad += r[18][j] * LB[j]); na += r[7],
             if(arm != "a0", na += r[18][1]; nd += r[18][2]; e2 += r[18][3]; eb += r[18][4], na += r[7])));
        Rs += (sti / #c[2]) / sqrt(c[1])));
    if(nt > 0,
      my(isn = (arm == "n1" || arm == "n1r64" || arm == "n1sab" || arm == "n1c2"), canon = if(isn, st, 0), cyc = if(isn && arm != "n1sab", 2*st, 0));
      my(row = [arm, b, ni, nt, ns, st, st / nt, Rs / ni, M, S, I, na, if(lad, lad, nd), e2, eb, dps, tr, cand, st, canon, cyc, dps, tr, lad, ms]);
      mapput(OPC, [arm, b], row);
      write("out/opcounts.csv", Str(strjoin(vector(#row, k, if(type(row[k]) == "t_STR", row[k], if(type(row[k]) == "t_INT", Str(row[k]), strprintf("%.4f", row[k])))), ","))))));
}
\\ ------- human-readable
print("=== OPERATION COUNTS (primary cost metric), pooled over instances, all trials incl. failures (censored at budget) ===");
print("arm b | trials solved | total steps | steps/trial | R=steps/sqrt(n/2) | M S I (totals) | per step M S I | escapes 2cyc/brent per step | DPs trails");
{
foreach(ARMS, arm,
  foreach(BS, b, my(r); if(mapisdefined(OPC, [arm, b], &r),
    print(arm, " b", b, " | ", r[4], " ", r[5], " | ", r[6], " | ", strprintf("%.1f", r[7]), " | ", strprintf("%.3f", r[8]), " | ", r[9], " ", r[10], " ", r[11], " | ",
      strprintf("%.3f %.3f %.3f", r[9]/r[6], r[10]/r[6], r[11]/r[6]), " | ", strprintf("%.5f %.5f", r[14]/r[6], r[15]/r[6]), " | ", r[16], " ", r[17]))));
}
\\ r = 64 sensitivity (instances 1,2, b36, b40): ratio of mean steps per instance, geometric mean
print("--- r=64 vs r=32 steps (instances 1,2), per b: geometric mean over instances of mean-steps(n1r64)/mean-steps(n1)");
{
foreach([36, 40], b,
  my(acc = 0, k = 0);
  foreach([1, 2], i, my(c64 = cell("n1r64", b, i), c32 = cell("n1", b, i));
    if(c64 && c32,
      acc += log((vecsum(vector(#c64[2], t, c64[2][t][7])) / #c64[2]) / (vecsum(vector(#c32[2], t, c32[2][t][7])) / #c32[2])); k++));
  if(k, print("steps n1r64/n1 b", b, ": ", strprintf("%.3f", exp(acc / k)), " (", k, " instances)")));
}
\\ native-operation totals per step for the pricing rows
print("--- native (unpriced) operations per walk step, b>=28 pooled (SEL, DPT, CAN, CYC, LAD per step; LOOK, START per step)");
{
foreach(["n1", "a0", "kmix", "kpure"], arm,
  my(st = 0, lad = 0, dps = 0, tr = 0, canon = 0, cyc = 0);
  foreach([28, 32, 36, 40], b, my(r); if(mapisdefined(OPC, [arm, b], &r), st += r[6]; lad += r[24]; dps += r[22]; tr += r[23]; canon += r[20]; cyc += r[21]));
  print(arm, ": SEL 1, DPT 1, CAN ", strprintf("%.3f", canon / st), ", CYC ", strprintf("%.3f", cyc / st), ", LAD ", strprintf("%.3f", lad / st), ", LOOK ", strprintf("%.5f", dps / st), ", START ", strprintf("%.5f", tr / st), "  (steps ", st, ")"));
}
\\ cell wall time (secondary, contention-affected: 4 cores shared with ~20 agents, 2 cells at a time)
print("--- cell wall time summed per arm (ms; SECONDARY, heavily contention-affected, not a cost metric)");
{
foreach(["n1", "n1r64", "n1sab", "n1c2", "a0", "kmix", "kpure"], arm,
  my(ms = 0); foreach(BS, b, my(r); if(mapisdefined(OPC, [arm, b], &r), ms += r[25]));
  print(arm, ": ", ms, " ms (", strprintf("%.1f", ms / 60000.), " min summed over cells; extension-instance and G1 cells only for K/A0, E5 raw cells include their E5 wall time)"));
}
