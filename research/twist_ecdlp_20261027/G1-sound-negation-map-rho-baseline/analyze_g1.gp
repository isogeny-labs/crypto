default(parisizemax, 2000000000);
default(realprecision, 28);
\\ analyze_g1.gp -- G1 analysis exactly as PREREGISTRATION.md section 7.  Reads raw/cell_*.txt (G1 N1 arms, extension K/A0 arms)
\\ and e5_inputs/cell_*.txt (E5 raw, read-only copies).  Writes out/solve.csv, out/ratios.csv; prints the summary on stdout.
NBOOT = 2000;
CELLS = Map(); CURHDR = Map(); CELLREC = List();
hdr(key, v) = mapput(CURHDR, key, v);
rec(v) = listput(CELLREC, v);
curve(v) = 0;
startswith(s, pre) = #s >= #pre && Vecsmall(s)[1..#pre] == Vecsmall(pre);
f2(x) = strprintf("%.3f", x);
LB = [1, 1, 2, 2, 3, 3, 4, 4];   \\ ladder iterations per step for SB = {2,3,5,7,11,13,17,19} (bitlength - 1)
RHOS = [3, 10, 30, 100];
\\ metric vector per trial.  1 steps, 2 solved, then per rho (index ri = 1..4) a block of 6 at 3 + 6*(ri-1):
\\   +0 M + 0.8 S + rho I ; +1 same + 0.1 GAE * NATIVE_B ; +2 same + 1 GAE * NATIVE_B ; +3 same + 0.1 GAE * NATIVE_A ;
\\   +4 same + 1 GAE * NATIVE_A ; +5 M + 1.0 S + rho I.   27..29 = raw M, S, I.   GAE = rho + 2 + 0.8 (one affine addition, in M).
\\ NATIVE_A = DP lookups/inserts + trail starts.  NATIVE_B = A + per-step selector hash + DP test + (N1: canonicalisation + 2 cycle compares)
\\ + (K arms: one per ladder iteration).
NMC = 26; NM = 29;
isK(arm) = (arm == "kpure" || arm == "kmix");
isN(arm) = (arm == "n1" || arm == "n1r64");
trialvec(arm, r) =
{
  my(st = r[7], ok = r[6], M = r[13], S = r[14], I = r[15], dps = r[12], tr = r[11], nB, nA, lad = 0, v = vector(NM), rho, g, base);
  nA = dps + tr;
  if(isK(arm), for(j = 1, 8, lad += r[18][j] * LB[j]); nB = 2*st + lad + nA,
     arm == "a0", nB = 2*st + nA,
     isN(arm), nB = 5*st + nA,
     nB = -1);
  v[1] = st; v[2] = ok; v[27] = M; v[28] = S; v[29] = I;
  if(nB >= 0,
    for(ri = 1, 4,
      rho = RHOS[ri]; g = rho + 2.8; base = M + 0.8*S + rho*I;
      v[3 + 6*(ri-1)] = base;
      v[4 + 6*(ri-1)] = base + 0.1*g*nB;
      v[5 + 6*(ri-1)] = base + 1.0*g*nB;
      v[6 + 6*(ri-1)] = base + 0.1*g*nA;
      v[7 + 6*(ri-1)] = base + 1.0*g*nA;
      v[8 + 6*(ri-1)] = M + 1.0*S + rho*I));
  v;
}
mname(m) =
{
  if(m == 1, return("steps")); if(m == 2, return("solved"));
  Str("I/M=", RHOS[(m - 3) \ 6 + 1], [" w0", " w0.1(B)", " w1(B)", " w0.1(A)", " w1(A)", " w0,S=1.0"][(m - 3) % 6 + 1]);
}
\\ ---------------------------------------------------------------- load
{
foreach(["raw", "e5_inputs"], dir,
  my(files = externstr(Str("ls ", dir)));
  foreach(files, f,
    if(startswith(f, "cell_") && #f > 4 && Vecsmall(f)[#f - 3..#f] == Vecsmall(".txt"),
      CURHDR = Map(); CELLREC = List();
      iferr(read(Str(dir, "/", f)), E, print("WARN truncated/failed read of ", dir, "/", f));
      my(c);
      if(mapisdefined(CURHDR, "cell", &c),
        my(arm = c[1], key = [c[1], c[2], c[3]], cur, n = mapget(CURHDR, "n"), cols);
        cols = vector(#CELLREC, i, trialvec(arm, CELLREC[i])~);
        if(mapisdefined(CELLS, key, &cur),
          mapput(CELLS, key, [cur[1], concat(cur[2], cols), concat(cur[3], [Str(dir, "/", f)])]),
          mapput(CELLS, key, [n, cols, [Str(dir, "/", f)]]))))));
}
CL = vecsort(Vec(Mat(CELLS)[, 1]~));
print("=== G1 analysis: ", #CL, " (arm,b,inst) cells loaded; bootstrap B = ", NBOOT, ", setrand(31337), instance-level then trial-level, stratified by b ===");
cell(arm, b, inst) = my(c); if(mapisdefined(CELLS, [arm, b, inst], &c), c, 0);
BS = [24, 28, 32, 36, 40];
HB = [28, 32, 36, 40];
INSTS = [1, 2, 3, 4];
wilson(k, n) = { my(z = 1.96, p = k / n, d = 1 + z^2 / n, c = p + z^2 / (2*n), h = z * sqrt(p*(1-p)/n + z^2 / (4*n^2))); [(c - h) / d, (c + h) / d]; }
CMAT = Map();
cmat(c, key) = my(m); if(mapisdefined(CMAT, key, &m), m, m = matconcat(Vec(c[2])); mapput(CMAT, key, m); m);
ksolved(arm, b, insts) =
{
  my(k = 0, n = 0, worst = 1);
  foreach(insts, i, my(c = cell(arm, b, i));
    if(c, my(ki = vecsum(vector(#c[2], t, c[2][t][2]))); k += ki; n += #c[2]; worst = min(worst, ki / #c[2])));
  [k, n, worst];
}
\\ one pass computes every metric: vector (length NM) of pooled geometric means of per-instance mean-cost ratios X/Y.
\\ boot = 1: within each b stratum the instances are drawn with replacement, then trials of X and of Y are resampled independently.
gmall(X, Y, b_list, insts, boot) =
{
  my(acc = vector(NM), cnt = 0);
  foreach(b_list, bb,
    my(avail = List());
    foreach(insts, i, if(cell(X, bb, i) && cell(Y, bb, i), listput(avail, i)));
    avail = Vec(avail);
    if(#avail == 0, next);
    for(k = 1, #avail,
      my(i = if(boot, avail[random(#avail) + 1], avail[k]), cx = cell(X, bb, i), cy = cell(Y, bb, i), Mx, My, nx, ny, px, py, wx, wy);
      Mx = cmat(cx, [X, bb, i]); My = cmat(cy, [Y, bb, i]); nx = #cx[2]; ny = #cy[2];
      if(boot,
        wx = vector(nx); for(t = 1, nx, wx[random(nx) + 1]++);
        wy = vector(ny); for(t = 1, ny, wy[random(ny) + 1]++);
        px = Mx * wx~ / nx; py = My * wy~ / ny,
        px = Mx * vector(nx, t, 1)~ / nx; py = My * vector(ny, t, 1)~ / ny);
      for(m = 1, NM, if(py[m] > 0 && px[m] > 0, acc[m] += log(px[m] / py[m])));
      cnt++));
  if(cnt == 0, return(0));
  vector(NM, m, exp(acc[m] / cnt));
}
RC = Map();
ratio_all(X, Y, b_list, insts) =
{
  my(key = [X, Y, b_list, insts], pt, mat, res, r);
  if(mapisdefined(RC, key, &r), return(r));
  pt = gmall(X, Y, b_list, insts, 0);
  if(pt == 0, mapput(RC, key, 0); return(0));
  setrand(31337);
  mat = matrix(NBOOT, NM);
  for(i = 1, NBOOT, my(v = gmall(X, Y, b_list, insts, 1)); for(m = 1, NM, mat[i, m] = v[m]));
  res = vector(NM, m, my(col = vecsort(mat[, m]~)); [pt[m], col[floor(0.025 * NBOOT) + 1], col[ceil(0.975 * NBOOT)]]);
  mapput(RC, key, res);
  res;
}
ratio_ci(X, Y, m, b_list, insts) = my(r = ratio_all(X, Y, b_list, insts)); if(r == 0, 0, r[m]);
fmt(c) = if(c == 0, "n/a (missing cells)", strprintf("%.3f [%.3f, %.3f]", c[1], c[2], c[3]));
CSV = List();
csvrow(name, c) = if(c != 0, listput(CSV, Str("\"", name, "\",", c[1], ",", c[2], ",", c[3])));

\\ ---------------------------------------------------------------- 1. solve rates
print("--- 1. solve rates (verified solves / trials) per arm and b, pooled over instances; Wilson 95% on the pooled rate");
SOLVE = List();
{
foreach(["n1", "n1r64", "n1sab", "n1c2", "a0", "kpure", "kmix", "a1c"], arm,
  foreach(BS, b,
    my(r = ksolved(arm, b, INSTS), parts = "");
    foreach(INSTS, i, my(c = cell(arm, b, i)); if(c, parts = Str(parts, " i", i, ":", vecsum(vector(#c[2], t, c[2][t][2])), "/", #c[2])));
    if(r[2] > 0, my(w = wilson(r[1], r[2]));
      print(arm, " b", b, ": ", r[1], "/", r[2], " = ", f2(r[1] / r[2]), "  Wilson [", f2(w[1]), ", ", f2(w[2]), "]  (", parts, " )");
      listput(SOLVE, Str(arm, ",", b, ",", r[1], ",", r[2], ",", w[1], ",", w[2])))));
}
write("out/solve.csv", "arm,b,solved,trials,wilson_lo,wilson_hi");
foreach(Vec(SOLVE), s, write("out/solve.csv", s));

\\ ---------------------------------------------------------------- 2. positive control
print("--- 2. positive control PC1 (N1 solves >= 95% pooled at every b = 24..40 and >= 90% on every instance) and PC2 (steps N1/A0 pooled b = 28..40 in [0.71, 0.80])");
{
my(pass = 1);
foreach(BS, b,
  my(r = ksolved("n1", b, INSTS), okb);
  if(r[2] == 0, print("PC1 b", b, ": MISSING"); pass = 0,
    okb = (r[1] / r[2] >= 0.95 && r[3] >= 0.90);
    print("PC1 b", b, ": pooled rate ", f2(r[1] / r[2]), " (worst instance ", f2(r[3]), ")  ", if(okb, "pass", "FAIL"));
    if(!okb, pass = 0)));
print("PC1 overall: ", if(pass, "PASS", "FAIL"));
}
PCR_ALL = ratio_ci("n1", "a0", 1, HB, INSTS);
PCR_E5 = ratio_ci("n1", "a0", 1, HB, [1, 2]);
print("PC2 steps N1/A0 pooled b28-40, all instances (E5+ext): ", fmt(PCR_ALL));
print("PC2 steps N1/A0 pooled b28-40, E5 instances only     : ", fmt(PCR_E5));
if(PCR_ALL != 0, print("PC2 verdict: ", if(PCR_ALL[1] >= 0.71 && PCR_ALL[1] <= 0.80, "PASS (point in [0.71, 0.80])", if(PCR_ALL[1] >= 0.68 && PCR_ALL[1] <= 0.85, "MARGINAL (point in [0.68, 0.85], outside [0.71, 0.80])", "FAIL"))));
csvrow("steps N1/A0 b28-40 all", PCR_ALL); csvrow("steps N1/A0 b28-40 E5inst", PCR_E5);
{
foreach(BS, b,
  my(c = ratio_ci("n1", "a0", 1, [b], INSTS));
  print("steps N1/A0 b", b, ": ", fmt(c)); csvrow(Str("steps N1/A0 b", b), c));
}

\\ ---------------------------------------------------------------- 3. negative controls
print("--- 3. negative controls");
{
foreach(["n1sab", "n1c2"], arm,
  foreach(BS, b,
    my(r = ksolved(arm, b, INSTS), insts2 = List(), r1);
    foreach(INSTS, i, if(cell(arm, b, i), listput(insts2, i)));
    r1 = ksolved("n1", b, Vec(insts2));
    if(r[2] > 0 && r1[2] > 0,
      my(w = wilson(r[1], r[2]));
      print("NC1 ", arm, " b", b, ": rate ", f2(r[1] / r[2]), " (Wilson upper ", f2(w[2]), ") vs N1 ", f2(r1[1] / r1[2]), " on the same instances; detector (upper < 0.95 and drop >= 0.20): ",
        if(w[2] < 0.95 && r1[1] / r1[2] - r[1] / r[2] >= 0.20, "SEES DROP", "no drop seen")))));
}
{
foreach(BS, b,
  my(lo = List(), hi = List());
  foreach(INSTS, i,
    my(c = cell("n1", b, i));
    if(c, my(nt = #c[2]); if(nt >= 4, listput(lo, vecsum(vector(nt \ 2, t, c[2][2*t - 1][1])) / (nt \ 2)); listput(hi, vecsum(vector(nt \ 2, t, c[2][2*t][1])) / (nt \ 2)))));
  if(#lo > 0,
    my(acc = 0);
    for(k = 1, #lo, acc += log(lo[k] / hi[k]));
    print("NC2 split-half null, N1 b", b, ": geometric mean over ", #lo, " instances of (odd-trial mean steps)/(even-trial mean steps) = ", f2(exp(acc / #lo)), " (must be near 1)")));
}

\\ ---------------------------------------------------------------- 4. headline: counted work of the K arms against N1
print("--- 4. ratios X/N1 pooled b = 28..40 (geometric mean over instances of per-instance mean-cost ratios; failures censored at the budget).");
print("     all = E5 instances 1,2 + extension 3,4 (headline); e5 = instances 1,2 only (co-primary).  95% percentile CI, instance-level then trial-level resampling.");
{
foreach(["kmix", "kpure"], X,
  my(ra = ratio_all(X, "n1", HB, INSTS), re = ratio_all(X, "n1", HB, [1, 2]));
  for(m = 1, NMC,
    if(m == 2, next);
    print(X, "/n1 ", mname(m), "  all: ", fmt(if(ra == 0, 0, ra[m])), "   e5: ", fmt(if(re == 0, 0, re[m])));
    csvrow(Str(X, "/n1 ", mname(m), " all"), if(ra == 0, 0, ra[m])); csvrow(Str(X, "/n1 ", mname(m), " e5"), if(re == 0, 0, re[m]))));
}
print("--- 4b. reference: steps X/A0 (the unmatched E5 baseline) and K-mix/N1 per b");
{
foreach(["kmix", "kpure"], X,
  my(c = ratio_ci(X, "a0", 1, HB, INSTS)); print(X, "/a0 steps all: ", fmt(c)); csvrow(Str(X, "/a0 steps all"), c));
foreach(BS, b,
  my(r = ratio_all("kmix", "n1", [b], INSTS));
  for(ri = 1, 4,
    my(m = 3 + 6*(ri - 1), c = if(r == 0, 0, r[m]));
    print("kmix/n1 b", b, " ", mname(m), ": ", fmt(c)); csvrow(Str("kmix/n1 b", b, " ", mname(m)), c)));
}
print("--- 4c. counted M, S, I per walk step (sum over all trials with b >= 28 divided by total steps; escapes and conversions included)");
{
foreach(["n1", "kmix", "kpure", "a0"], arm,
  my(M = 0, S = 0, I = 0, st = 0);
  foreach(HB, b, foreach(INSTS, i, my(c = cell(arm, b, i));
    if(c, foreach(c[2], v, st += v[1]; M += v[27]; S += v[28]; I += v[29]))));
  if(st > 0, print(arm, ": M/step ", f2(M/st), "  S/step ", f2(S/st), "  I/step ", f2(I/st), "  (total steps ", st, ")")));
}
write("out/ratios.csv", "name,point,lo95,hi95");
foreach(Vec(CSV), s, write("out/ratios.csv", s));
print("=== done ===");
