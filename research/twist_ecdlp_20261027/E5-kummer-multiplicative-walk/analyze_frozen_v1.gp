default(parisizemax, 2000000000);
default(realprecision, 28);
\\ analyze.gp -- reads raw/*.txt (or $RAWDIR), writes out/summary.csv, out/costs.csv, out/results.json (or $OUTDIR) and prints the summary.
RAW = getenv("RAWDIR"); if(RAW == 0, RAW = "raw");
OUT = getenv("OUTDIR"); if(OUT == 0, OUT = "out");
NBOOT = 2000;
NA = 10^99;
CELLS = Map(); CHK = Map(); CURVES = Map(); CURHDR = Map(); CELLREC = List();
chk(name, v) = mapput(CHK, name, v);
curve(v) = mapput(CURVES, [v[1], v[2]], v);
hdr(key, v) = mapput(CURHDR, key, v);
rec(v) = listput(CELLREC, v);
startswith(s, pre) = #s >= #pre && Vecsmall(s)[1..#pre] == Vecsmall(pre);
f2(x) = strprintf("%.3f", x);
f1(x) = strprintf("%.1f", x);

\\ ------------------------------------------------------------------ load
{
my(files = externstr(Str("ls ", RAW)));
iferr(read(Str(RAW, "/curves.txt")), E, print("WARN curves.txt: ", E));
foreach(["identity.txt", "exhaustive.txt", "chains.txt"], f, iferr(read(Str(RAW, "/", f)), E, print("WARN ", f, ": ", E)));
foreach(files, f,
  if(startswith(f, "cell_") && #f > 4 && Vecsmall(f)[#f - 3..#f] == Vecsmall(".txt"),
    CURHDR = Map(); CELLREC = List();
    iferr(read(Str(RAW, "/", f)), E, print("WARN truncated/failed read of ", f, " (records before the error are kept)"));
    my(c);
    if(mapisdefined(CURHDR, "cell", &c),
      mapput(CELLS, [c[1], c[2], c[3]], [CURHDR, Vec(CELLREC), f]),
      print("WARN no cell header in ", f))));
}
CL = Vec(Mat(CELLS)[, 1]~);
CL = vecsort(CL);
NC = #CL;
CIDX = Map(); for(i = 1, NC, mapput(CIDX, CL[i], i));
NCELLS = vector(NC); MARM = vector(NC); NTRIAL = vector(NC); HDR = vector(NC); RECS = vector(NC);
{
for(i = 1, NC,
  my(c = mapget(CELLS, CL[i]), h = c[1]);
  HDR[i] = h; RECS[i] = c[2]; NTRIAL[i] = #c[2];
  NCELLS[i] = mapget(h, "n");
  if(mapisdefined(h, "Marm"), MARM[i] = mapget(h, "Marm"), MARM[i] = 1));
}
cidx(arm, b, inst) = my(i); if(mapisdefined(CIDX, [arm, b, inst], &i), i, 0);
has(arm, b, inst) = cidx(arm, b, inst) > 0;
isK(arm) = arm != "a0" && arm != "a1";
\\ headline instance good = +-H = F_n^* for the SB set on E (from the kpure cell header, else the kmix header)
GOOD = Map();
{
for(bi = 1, 6, for(inst = 1, 2,
  my(b = 20 + 4*(bi-1), i = cidx("kpure", b, inst), hi);
  if(i == 0, i = cidx("kmix", b, inst));
  if(i, hi = mapget(HDR[i], "Hinfo"); mapput(GOOD, [b, inst], hi[5] == 1), mapput(GOOD, [b, inst], 0))));
}
good(b, inst) = my(g); if(mapisdefined(GOOD, [b, inst], &g), g, 0);

\\ ------------------------------------------------------------------ per-trial quantities
\\ q: 1 T, 2 ok, 3-6 cost(w=0.8, rho=3,10,30,100), 7-10 cost(w=1.0), 11-14 cost with xADD at 3M (w=0.8), 15 ms
RHO = [3, 10, 30, 100];
NQ = 15;
xaddcount(r, h) =
{
  my(hist = r[18], mults, x = 0);
  if(isK(r[1]), mults = mapget(h, "mults"); for(j = 1, #mults, x += hist[j] * (#binary(mults[j]) - 1)));
  x;
}
qvec(r, h) =
{
  my(v = vector(NQ), x = xaddcount(r, h), cM = r[13], cS = r[14], cI = r[15]);
  v[1] = r[7]; v[2] = r[6];
  for(k = 1, 4,
    v[2 + k] = (cM + 4/5 * cS + RHO[k] * cI) * 1.;
    v[6 + k] = (cM + cS + RHO[k] * cI) * 1.;
    v[10 + k] = (cM - x + 4/5 * cS + RHO[k] * cI) * 1.);
  v[15] = r[19];
  v;
}
VM = vector(NC);
{
for(i = 1, NC,
  my(R = RECS[i], nt = #R, M = matrix(NQ, nt));
  for(t = 1, nt, M[, t] = qvec(R[t], HDR[i])~);
  VM[i] = M);
}
cellmean0() = vector(NC, i, if(NTRIAL[i], (VM[i] * vectorv(NTRIAL[i], t, 1))~ / NTRIAL[i], vector(NQ)));
cellmeanBoot() =
{
  vector(NC, i,
    my(nt = NTRIAL[i], cnt);
    if(nt == 0, vector(NQ),
      cnt = vectorv(nt); for(k = 1, nt, cnt[random(nt) + 1]++);
      (VM[i] * cnt)~ / nt));
}

\\ ------------------------------------------------------------------ statistics (functions of cell means)
CMX = 0; SCNT = 0; SN = vector(4000, i, ""); SV = vector(4000); NAMEMODE = 1;
add(name, v) = { SCNT++; if(NAMEMODE, SN[SCNT] = name); SV[SCNT] = v; }
cmq(arm, b, inst, q) = my(i = cidx(arm, b, inst)); if(i, CMX[i][q], NA);
Rc(arm, b, inst) = my(i = cidx(arm, b, inst)); if(i && NTRIAL[i], CMX[i][1] / sqrt(MARM[i]), NA);
Tn(arm, b, inst, q) = my(i = cidx(arm, b, inst)); if(i && NTRIAL[i], CMX[i][q] / sqrt(NCELLS[i]), NA);
BS = [20, 24, 28, 32, 36, 40];
ARMS5 = ["kpure", "kmix", "a0", "a1", "kpureT"];
\\ pooled mean of R over good cells with b in bs
Rpool(arm, bs) =
{
  my(s = 0, c = 0, r);
  foreach(bs, b, for(inst = 1, 2,
    if(good(b, inst) && has(arm, b, inst), r = Rc(arm, b, inst); if(r < NA, s += r; c++))));
  if(c, s / c, NA);
}
\\ ratio of pooled normalised means (common good cells)
rat(aK, aA, bs, q) =
{
  my(sK = 0, sA = 0, c = 0, x, y);
  foreach(bs, b, for(inst = 1, 2,
    if(good(b, inst) && has(aK, b, inst) && has(aA, b, inst),
      x = Tn(aK, b, inst, q); y = Tn(aA, b, inst, q);
      if(x < NA && y < NA, sK += x; sA += y; c++))));
  if(c && sA > 0, sK / sA, NA);
}
slope(arm, usen) =
{
  my(X = [], Y = [], mx, my_, sxx = 0, sxy = 0, k, i, t);
  foreach(BS, b, for(inst = 1, 2,
    i = cidx(arm, b, inst);
    if(i && NTRIAL[i] && good(b, inst),
      t = CMX[i][1];
      X = concat(X, [if(usen, log(NCELLS[i]) / log(2), b)]); Y = concat(Y, [log(t) / log(2)]))));
  k = #X;
  if(k < 3, return(NA));
  mx = vecsum(X) / k; my_ = vecsum(Y) / k;
  for(j = 1, k, sxx += (X[j] - mx)^2; sxy += (X[j] - mx) * (Y[j] - my_));
  sxy / sxx;
}
PAIRS = [["kpure", "a0"], ["kpure", "a1"], ["kmix", "a0"], ["kmix", "a1"]];
stats(CM) =
{
  CMX = CM; SCNT = 0;
  \\ 1. per-cell R
  for(i = 1, NC, add(Str("R|", CL[i][1], "|", CL[i][2], "|", CL[i][3]), if(NTRIAL[i], CM[i][1] / sqrt(MARM[i]), NA)));
  \\ 2. pooled R per (arm,b) over good instances, and b>=24, b>=28
  foreach(ARMS5, arm,
    foreach(BS, b, add(Str("Rb|", arm, "|", b), Rpool(arm, [b])));
    add(Str("Rge24|", arm), Rpool(arm, [24, 28, 32, 36, 40]));
    add(Str("Rge28|", arm), Rpool(arm, [28, 32, 36, 40])));
  \\ 3. slopes
  foreach(ARMS5, arm, add(Str("slope|", arm), slope(arm, 0)); add(Str("slopen|", arm), slope(arm, 1)));
  \\ 4. step ratios (b >= 28), raw-T normalised by sqrt(n)
  foreach([["kpure", "a0"], ["kpure", "a1"], ["kmix", "a0"], ["kmix", "a1"], ["a1", "a0"], ["kpure", "kmix"]], pr,
    add(Str("SR|", pr[1], "/", pr[2], "|ge28"), rat(pr[1], pr[2], [28, 32, 36, 40], 1));
    foreach([32, 36, 40], b, add(Str("SR|", pr[1], "/", pr[2], "|b", b), rat(pr[1], pr[2], [b], 1))));
  \\ 5. cost ratios
  foreach(PAIRS, pr,
    for(k = 1, 4,
      add(Str("CR|", pr[1], "/", pr[2], "|w0.8|rho", RHO[k], "|ge28"), rat(pr[1], pr[2], [28, 32, 36, 40], 2 + k));
      add(Str("CR|", pr[1], "/", pr[2], "|w1.0|rho", RHO[k], "|ge28"), rat(pr[1], pr[2], [28, 32, 36, 40], 6 + k));
      add(Str("CR3|", pr[1], "/", pr[2], "|w0.8|rho", RHO[k], "|ge28"), rat(pr[1], pr[2], [28, 32, 36, 40], 10 + k));
      foreach([32, 36, 40], b,
        add(Str("CR|", pr[1], "/", pr[2], "|w0.8|rho", RHO[k], "|b", b), rat(pr[1], pr[2], [b], 2 + k)))));
  \\ 6. NC3: A0 normalised steps, inst1/inst2
  foreach(BS, b,
    my(x = Tn("a0", b, 1, 1), y = Tn("a0", b, 2, 1));
    add(Str("NC3|", b), if(x < NA && y < NA && y > 0, x / y, NA)));
  \\ 7. PC4 : A0 raw mean steps b+4 over b (instances pooled, equal weight)
  foreach([[24, 28], [28, 32], [32, 36], [36, 40]], pb,
    my(s0 = 0, c0 = 0, s1 = 0, c1 = 0);
    for(inst = 1, 2,
      if(has("a0", pb[1], inst) && NTRIAL[cidx("a0", pb[1], inst)], s0 += cmq("a0", pb[1], inst, 1); c0++);
      if(has("a0", pb[2], inst) && NTRIAL[cidx("a0", pb[2], inst)], s1 += cmq("a0", pb[2], inst, 1); c1++));
    add(Str("PC4|", pb[1], "-", pb[2]), if(c0 && c1, (s1 / c1) / (s0 / c0), NA)));
  \\ 8. F3: twist K-pure vs E K-pure normalised R ratio
  foreach([[28, 32, 36, 40], [28], [32], [36], [40]], bs,
    my(sT = 0, sE = 0, c = 0, x, y);
    foreach(bs, b, for(inst = 1, 2,
      x = Rc("kpureT", b, inst); y = Rc("kpure", b, inst);
      if(x < NA && y < NA && good(b, inst), sT += x; sE += y; c++)));
    add(Str("F3|RT/RE|", if(#bs > 1, "ge28", Str("b", bs[1]))), if(c, sT / sE, NA)));
  \\ 9. per-step ops (point values of the bootstrap-mean ratio) for K arms, pooled
  0;
}
\\ point estimate
NAMEMODE = 1;
CM0 = cellmean0();
stats(CM0);
NST = SCNT;
NAMES = vector(NST, i, SN[i]);
PT = vector(NST, i, SV[i]);
NAMEMODE = 0;
NAMEIDX = Map(); for(i = 1, NST, mapput(NAMEIDX, NAMES[i], i));
\\ bootstrap
setrand(31337);
BOOT = matrix(NBOOT, NST);
{
for(r = 1, NBOOT,
  stats(cellmeanBoot());
  for(i = 1, NST, BOOT[r, i] = SV[i]));
}
LO = vector(NST); HI = vector(NST);
{
for(i = 1, NST,
  my(v = vecsort(vector(NBOOT, r, BOOT[r, i])));
  if(PT[i] >= NA || v[NBOOT \ 2] >= NA, LO[i] = NA; HI[i] = NA,
    LO[i] = v[max(1, floor(0.025 * NBOOT))]; HI[i] = v[min(NBOOT, ceil(0.975 * NBOOT))]));
}
st(name) = my(i = mapget(NAMEIDX, name)); [PT[i], LO[i], HI[i]];
stok(name) = mapisdefined(NAMEIDX, name) && st(name)[1] < NA;
fst(name) = if(!mapisdefined(NAMEIDX, name), "n/a", my(s = st(name)); if(s[1] >= NA, "NA", Str(f2(s[1]), " [", f2(s[2]), ", ", f2(s[3]), "]")));

\\ ------------------------------------------------------------------ cell table
CSV = Str(OUT, "/summary.csv");
write(CSV, "arm,b,inst,trials,successes,n,Marm,Nstates,indexpm,dpbits,budget,meanT,R,R_lo,R_hi,M_per_step,S_per_step,I_per_step,mean_wasted,mean_trails,mean_dps,mean_cand,us_per_step");
{
for(i = 1, NC,
  my(c = CL[i], R = RECS[i], nt = #R, h = HDR[i], hi = mapget(h, "Hinfo"), ok = 0, sT = 0, sM = 0, sS = 0, sI = 0, sW = 0, sTr = 0, sD = 0, sC = 0, sMs = 0, rr, ri);
  if(nt == 0, write(CSV, Str(c[1], ",", c[2], ",", c[3], ",0,0,", NCELLS[i], ",", MARM[i], ",", hi[4], ",", hi[5], ",,,,,,,,,,,,,")); next);
  for(t = 1, nt, ok += R[t][6]; sT += R[t][7]; sM += R[t][13]; sS += R[t][14]; sI += R[t][15]; sW += R[t][10]; sTr += R[t][11]; sD += R[t][12]; sC += R[t][17]; sMs += R[t][19]);
  ri = mapget(NAMEIDX, Str("R|", c[1], "|", c[2], "|", c[3]));
  write(CSV, Str(c[1], ",", c[2], ",", c[3], ",", nt, ",", ok, ",", NCELLS[i], ",", MARM[i], ",", hi[4], ",", hi[5], ",", mapget(h, "dpbits"), ",", mapget(h, "budget"), ",",
     f2(sT / nt), ",", f2(PT[ri]), ",", f2(LO[ri]), ",", f2(HI[ri]), ",", f2(sM / sT), ",", f2(sS / sT), ",", f2(sI / sT), ",", f2(sW / nt), ",", f2(sTr / nt), ",", f2(sD / nt), ",", f2(sC / nt), ",", f2(1000 * sMs / sT))));
}
COSTS = Str(OUT, "/costs.csv");
write(COSTS, "name,point,lo95,hi95");
{
for(i = 1, NST,
  if(startswith(NAMES[i], "R|"), next);
  write(COSTS, Str(NAMES[i], ",", if(PT[i] >= NA, "NA", f2(PT[i])), ",", if(LO[i] >= NA, "NA", f2(LO[i])), ",", if(HI[i] >= NA, "NA", f2(HI[i])))));
}

\\ ------------------------------------------------------------------ report
print("=== E5 analysis: cells loaded ", NC, ", bootstrap resamples ", NBOOT, " (setrand(31337), stratified by cell, percentile 95%) ===");
print("--- per-cell table (arm b inst : trials ok / n / Marm / index / meanT / R [CI] / M,S,I per step / mean wasted / us per step)");
{
for(i = 1, NC,
  my(c = CL[i], R = RECS[i], nt = #R, h = HDR[i], hi = mapget(h, "Hinfo"), ok = 0, sT = 0, sM = 0, sS = 0, sI = 0, sW = 0, sMs = 0, ri, tt);
  tt = mapget(h, "cell")[4];
  if(nt == 0, print(c[1], " b", c[2], " i", c[3], " : NO TRIALS RECORDED (expected ", tt, ")"); next);
  for(t = 1, nt, ok += R[t][6]; sT += R[t][7]; sM += R[t][13]; sS += R[t][14]; sI += R[t][15]; sW += R[t][10]; sMs += R[t][19]);
  ri = mapget(NAMEIDX, Str("R|", c[1], "|", c[2], "|", c[3]));
  print(c[1], " b", c[2], " i", c[3], " : trials ", nt, "/", tt, " ok ", ok, " n=", NCELLS[i], " Marm=", MARM[i], " idx=", hi[5], " meanT=", f1(sT / nt),
        " R=", f2(PT[ri]), " [", f2(LO[ri]), ", ", f2(HI[ri]), "] M/S/I per step=", f2(sM / sT), "/", f2(sS / sT), "/", f2(sI / sT), " wasted=", f2(sW / nt), " us/step=", f2(1000 * sMs / sT)));
}
print("--- pooled R per (arm,b) over good instances");
foreach(ARMS5, arm, foreach(BS, b, if(stok(Str("Rb|", arm, "|", b)), print("Rb ", arm, " b", b, " : ", fst(Str("Rb|", arm, "|", b))))));
foreach(ARMS5, arm, print("Rge24 ", arm, " : ", fst(Str("Rge24|", arm)), "   Rge28 : ", fst(Str("Rge28|", arm))));
print("--- slopes of log2(mean T) on b and on log2 n");
foreach(ARMS5, arm, print("slope ", arm, " : ", fst(Str("slope|", arm)), "   vs log2 n : ", fst(Str("slopen|", arm))));
print("--- step ratios (normalised by sqrt n, good cells)");
{
foreach([["kpure", "a0"], ["kpure", "a1"], ["kmix", "a0"], ["kmix", "a1"], ["a1", "a0"], ["kpure", "kmix"]], pr,
  print("SR ", pr[1], "/", pr[2], " b>=28 : ", fst(Str("SR|", pr[1], "/", pr[2], "|ge28")), "   b32 ", fst(Str("SR|", pr[1], "/", pr[2], "|b32")), " b36 ", fst(Str("SR|", pr[1], "/", pr[2], "|b36")), " b40 ", fst(Str("SR|", pr[1], "/", pr[2], "|b40"))));
}
print("--- cost ratios K/A (b>=28 pooled); counted cost = M + w S + rho I");
{
foreach(PAIRS, pr, for(k = 1, 4,
  print("CR ", pr[1], "/", pr[2], " rho=", RHO[k], " w0.8 : ", fst(Str("CR|", pr[1], "/", pr[2], "|w0.8|rho", RHO[k], "|ge28")),
        "  w1.0 : ", fst(Str("CR|", pr[1], "/", pr[2], "|w1.0|rho", RHO[k], "|ge28")),
        "  [sens. xADD=3M] : ", fst(Str("CR3|", pr[1], "/", pr[2], "|w0.8|rho", RHO[k], "|ge28")))));
}
print("--- cost ratios per b at w=0.8 (F5 uses rho=3, A1)");
{
foreach(PAIRS, pr, foreach([32, 36, 40], b, for(k = 1, 4,
  print("CRb ", pr[1], "/", pr[2], " b", b, " rho=", RHO[k], " : ", fst(Str("CR|", pr[1], "/", pr[2], "|w0.8|rho", RHO[k], "|b", b))))));
}
print("--- NC3 (A0 inst1/inst2 normalised mean T ratio)");
foreach(BS, b, print("NC3 b", b, " : ", fst(Str("NC3|", b)), if(stok(Str("NC3|", b)) && st(Str("NC3|", b))[2] <= 1 && st(Str("NC3|", b))[3] >= 1, "  contains 1", "  EXCLUDES 1")));
print("--- PC4 (A0 raw mean T ratio b+4 / b, expected 4, band [3.0, 5.3])");
foreach([[24, 28], [28, 32], [32, 36], [36, 40]], pb, print("PC4 ", pb[1], "->", pb[2], " : ", fst(Str("PC4|", pb[1], "-", pb[2]))));
print("--- F3 twist/E normalised R ratio");
foreach(["ge28", "b28", "b32", "b36", "b40"], s, print("F3 RT/RE ", s, " : ", fst(Str("F3|RT/RE|", s))));

\\ ------------------------------------------------------------------ checks
print("--- exact checks (raw/identity.txt, raw/exhaustive.txt)");
chkget(name) = my(v); if(mapisdefined(CHK, name, &v), v, 0);
{
my(tot = 0, mis = 0, infs = 0, cE = 0, cT = 0);
for(bi = 1, 3, for(inst = 1, 2,
  my(v = chkget(Str("ladder_b", 20 + 4*(bi-1), "_i", inst)));
  if(v == 0, print("MISSING ladder_b", 20 + 4*(bi-1), "_i", inst); next);
  cE += v[1]; cT += v[2]; tot += v[3] + v[4]; mis += v[5] + v[6]; infs += v[7] + v[8]));
print("LADDER: x on E ", cE, ", x on E' ", cT, ", ladder-vs-ellmul tests ", tot, ", mismatches ", mis, ", infinity cases consistent ", infs);
LAD_TOT = tot; LAD_MIS = mis;
}
{
my(okc = 0, wp = 0, sp = 0, ntest = 0);
for(bi = 1, 6, for(inst = 1, 2, for(tw = 0, 1,
  my(v = chkget(Str("nc4_b", 20 + 4*(bi-1), "_i", inst, "_t", tw)));
  if(v == 0, print("MISSING nc4"); next);
  okc += v[1]; wp += v[2]; sp += v[3]; ntest += 1000)));
print("NC4: tests ", ntest, ", true a verified ", okc, ", wrong a+1 accepted ", wp, " (special a with 2a+1=0 mod n: ", sp, ")");
NC4_WP = wp; NC4_N = ntest;
}
{
my(cr = 0, mx = 0, tt = 0);
for(ci = 1, 3,
  my(v = chkget(Str("classes_", ci)));
  if(v == 0, print("MISSING classes_", ci); next);
  print("CLASSES curve ", ci, ": p=", v[1], " A=", v[2], " x with f!=0 ", v[3], " step evaluations ", v[4], " class crossings ", v[5], " E-only comps ", v[6], " E'-only comps ", v[7], " mixed comps ", v[8]);
  cr += v[5]; mx += v[8]; tt += v[4]);
CLS_CROSS = cr; CLS_MIXED = mx;
}
{
my(v = chkget("pc1_orbit_set1"), w, o);
print("PC1 SB   : n=", v[1], " |H|=", v[2], " N=", v[3], " index=", v[4], " in-set a=", v[5], " orbit-vs-+-H mismatches=", v[6], " closure-vs-znlog mismatches=", v[7]);
w = chkget("pc1_walk_set1"); o = chkget("pc1_out_set1");
print("PC1 SB   walks: ", w[2], "/", w[1], " verified (budget ", w[3], " steps, N=", w[4], ", dpbits ", w[5], "); out-of-set a walked ", o[2], " solved ", o[3]);
PC1A_MIS = v[6] + v[7]; PC1A_WALK = w; PC1A_OUT = o;
v = chkget("pc1_orbit_set2");
print("PC1 SIDX : n=", v[1], " |H|=", v[2], " N=", v[3], " index=", v[4], " in-set a=", v[5], " orbit-vs-+-H mismatches=", v[6], " closure-vs-znlog mismatches=", v[7]);
w = chkget("pc1_walk_set2"); o = chkget("pc1_out_set2");
print("PC1 SIDX walks: ", w[2], "/", w[1], " verified (budget ", w[3], " steps, N=", w[4], ", dpbits ", w[5], ", total trails ", w[6], ", DPs ", w[7], ", steps ", w[8], "); out-of-set a walked ", o[2], " solved ", o[3]);
PC1B_MIS = v[6] + v[7]; PC1B_WALK = w; PC1B_OUT = o;
}
{
my(bad = 0, tot = 0, v);
foreach(["SA", "SB", "SC"], nm, for(pi = 1, 20,
  v = chkget(Str("pc2_", nm, "_", pi));
  if(v == 0, print("MISSING pc2_", nm, "_", pi); next);
  tot++; if(v[1] != v[2] || v[1] != v[3], bad++)));
print("PC2: orbit-size checks ", tot, ", mismatches (N vs ellmul-orbit vs ladder-BFS) ", bad);
PC2_BAD = bad; PC2_TOT = tot;
}

\\ ------------------------------------------------------------------ PC5 counter check, NC1/NC2, PC3
print("--- PC5 counter identity, NC1, NC2, PC3");
{
my(bad = 0, tt = 0, badT = 0);
for(i = 1, NC,
  my(arm = CL[i][1], h = HDR[i], R = RECS[i], mults, ecM, ecS, ecI);
  for(t = 1, #R,
    my(r = R[t], hist = r[18]);
    tt++;
    if(isK(arm),
      mults = mapget(h, "mults"); ecM = 0; ecS = 0; ecI = 0;
      for(j = 1, #mults, my(nb = #binary(mults[j]) - 1); ecM += hist[j] * (3 + 7*nb + 1); ecS += hist[j] * (2 + 4*nb); ecI += hist[j]);
      if(ecM != r[13] || ecS != r[14] || ecI != r[15] || vecsum(hist) != r[7], bad++),
      ecM = 2*hist[1] + 3*(hist[2] + hist[3]); ecS = hist[1] + 2*(hist[2] + hist[3]); ecI = vecsum(hist);
      if(ecM != r[13] || ecS != r[14] || ecI != r[15], bad++);
      if(hist[1] + hist[2] != r[7] && r[6], badT++))));
print("PC5: trials checked ", tt, ", counter/histogram mismatches ", bad, " (A arms: successful trials with T != adds+degenerate doublings: ", badT, ")");
PC5_BAD = bad; PC5_N = tt;
}
{
my(cand = 0, ok = 0, nt = 0, cells = 0);
for(i = 1, NC, if(CL[i][1] == "nc1", cells++; foreach(RECS[i], r, nt++; cand += r[17]; ok += r[6])));
print("NC1: cells ", cells, ", trials ", nt, ", cross-side x matches ", cand, ", verified solves ", ok);
NC1_CELLS = cells; NC1_N = nt; NC1_CAND = cand; NC1_OK = ok;
cand = 0; ok = 0; nt = 0; cells = 0;
for(i = 1, NC, if(CL[i][1] == "cross", cells++; foreach(RECS[i], r, nt++; cand += r[17]; ok += r[6])));
print("NC2: cross cells ", cells, ", trials ", nt, ", E-vs-E' x matches ", cand, ", verified solves ", ok);
NC2_CELLS = cells; NC2_N = nt; NC2_CAND = cand;
for(i = 1, NC, if(CL[i][1] == "pc1", my(ok2 = 0); foreach(RECS[i], r, ok2 += r[6]); print("PC3 pc1 b", CL[i][2], " i", CL[i][3], ": ", ok2, "/", #RECS[i], " verified (N=", mapget(HDR[i], "Hinfo")[4], ", index ", mapget(HDR[i], "Hinfo")[5], ")")));
}
\\ degenerate / skipped cells
{
for(i = 1, NC, if(mapisdefined(HDR[i], "skipped_degenerate_N"), print("SKIPPED degenerate cell ", CL[i][1], " b", CL[i][2], " i", CL[i][3], " : N=", mapget(HDR[i], "skipped_degenerate_N"))));
}

\\ ------------------------------------------------------------------ chains
print("--- ideal-chain step cost (SB multipliers, S = 0.8 M)");
{
my(sOps = 0, sM = 0, sS = 0, sLM = 0, sLS = 0, sLops = 0, sME = 0, sLME = 0);
foreach([2, 3, 5, 7, 11, 13, 17, 19], k,
  my(v = chkget(Str("chain_", k)), nd, na, l);
  if(v == 0, print("MISSING chain_", k); next);
  nd = v[2]; na = v[3]; l = v[4];
  sOps += nd + na; sM += 3*nd + 4*na; sS += 2*nd + 2*na;
  sLops += l + (l - 1); sLM += 3*l + 4*(l - 1); sLS += 2*l + 2*(l - 1));
print("ideal chain: mean differential ops/step ", f2(sOps / 8), ", mean M ", f2(sM / 8), " + 1 (conversion), mean S ", f2(sS / 8), ", M-equiv excl. I (S=0.8) ", f2(sM / 8 + 1 + 0.8 * sS / 8));
print("ladder (analytic) : mean ops/step ", f2(sLops / 8), ", mean M ", f2(sLM / 8), " + 1, mean S ", f2(sLS / 8), ", M-equiv excl. I (S=0.8) ", f2(sLM / 8 + 1 + 0.8 * sLS / 8));
CH_OPS = sOps / 8; CH_ME = sM / 8 + 1 + 0.8 * sS / 8;
}

\\ ------------------------------------------------------------------ pooled per-step cost of the implemented ladder
print("--- measured per-step counters (K arms, SB, E; pooled over all trials and cells)");
{
foreach(["kpure", "kmix", "kpureT"], arm,
  my(sT = 0, sM = 0, sS = 0, sI = 0, lo = [10^9, 10^9, 10^9], hi = [0, 0, 0]);
  for(i = 1, NC, if(CL[i][1] == arm,
    my(cT = 0, cMM = 0, cSS = 0, cII = 0);
    foreach(RECS[i], r, cT += r[7]; cMM += r[13]; cSS += r[14]; cII += r[15]);
    if(cT, sT += cT; sM += cMM; sS += cSS; sI += cII;
      lo = [min(lo[1], cMM / cT), min(lo[2], cSS / cT), min(lo[3], cII / cT)]; hi = [max(hi[1], cMM / cT), max(hi[2], cSS / cT), max(hi[3], cII / cT)])));
  if(sT, print("STEPCOST ", arm, " : M/step ", f2(sM / sT), " (cell range ", f2(lo[1]), "-", f2(hi[1]), "), S/step ", f2(sS / sT), " (", f2(lo[2]), "-", f2(hi[2]), "), I/step ", f2(sI / sT), " ; predicted 21.5 M, 12 S, 1 I ; dev M ", f2(100 * (sM / sT / 21.5 - 1)), "% S ", f2(100 * (sS / sT / 12 - 1)), "%");
    if(arm == "kpure", STEP_M = sM / sT; STEP_S = sS / sT; STEP_I = sI / sT)));
foreach(["a0", "a1"], arm,
  my(sT = 0, sM = 0, sS = 0, sI = 0);
  for(i = 1, NC, if(CL[i][1] == arm, foreach(RECS[i], r, sT += r[7]; sM += r[13]; sS += r[14]; sI += r[15])));
  if(sT, print("STEPCOST ", arm, " : M/step ", f2(sM / sT), ", S/step ", f2(sS / sT), ", I/step ", f2(sI / sT), " ; predicted 2 M, 1 S, 1 I (+ doublings)")));
}

\\ ------------------------------------------------------------------ verdicts
print("=== VERDICTS per PREREGISTRATION section 8 ===");
{
my(f1fail = [], f2cells = 0, f2list = [], slo, v);
\\ F1
if(PC1A_MIS + PC1B_MIS > 0, f1fail = concat(f1fail, ["orbit mismatch in PC1"]));
if(PC2_BAD > 0, f1fail = concat(f1fail, ["PC2 orbit size mismatch"]));
if(PC1A_WALK != 0 && PC1A_WALK[2] < 0.98 * PC1A_WALK[1], f1fail = concat(f1fail, ["PC1 SB walk recovery < 98%"]));
if(PC1B_WALK != 0 && PC1B_WALK[1] > 0 && PC1B_WALK[2] < 0.98 * PC1B_WALK[1], f1fail = concat(f1fail, [Str("PC1 S_idx walk recovery ", PC1B_WALK[2], "/", PC1B_WALK[1], " < 98%")]));
if(PC1A_OUT[3] + PC1B_OUT[3] > 0, f1fail = concat(f1fail, ["out-of-set a recovered"]));
if(NC1_CAND + NC1_OK > 0, f1fail = concat(f1fail, ["NC1 collision"]));
print("H1/F1: ", if(#f1fail, Str("F1 TRIGGERED: ", f1fail), "no F1 trigger"), "   [exact orbit/exhaustive mismatches: ", PC1A_MIS + PC1B_MIS + PC2_BAD, "]");
\\ F2
foreach([24, 28, 32, 36, 40], b, for(inst = 1, 2,
  my(nm = Str("R|kpure|", b, "|", inst));
  if(mapisdefined(NAMEIDX, nm) && good(b, inst), v = st(nm);
    if(v[1] < NA && v[2] < NA && (v[3] < 1.4 || v[2] > 3.0), f2cells++; f2list = concat(f2list, [[b, inst, v]])))));
print("H2/F2: kpure cells (b>=24, good) with R CI entirely outside [1.4,3.0]: ", f2cells, "  ", f2list);
slo = st("slope|kpure");
print("       slope kpure: ", fst("slope|kpure"), " ; inside [0.40,0.60]: ", if(slo[2] < NA && slo[2] >= 0.40 && slo[3] <= 0.60, "YES", "NO"));
my(bands = [["SR|kpure/a0|ge28", 0.8, 1.3], ["SR|kmix/a0|ge28", 0.55, 0.9], ["SR|kmix/a1|ge28", 0.75, 1.15]]);
foreach(bands, bd, v = st(bd[1]); print("       ", bd[1], " = ", fst(bd[1]), " band [", bd[2], ",", bd[3], "] : ", if(v[2] < NA && (v[3] < bd[2] || v[2] > bd[3]), "CI ENTIRELY OUTSIDE (F2 trigger)", "not entirely outside")));
\\ F3
print("H3/F3: ladder mismatches ", LAD_MIS, " of ", LAD_TOT, "; class crossings ", CLS_CROSS, ", mixed components ", CLS_MIXED, "; cross-curve x matches ", NC2_CAND, " in ", NC2_N, " trials; F3 twist ratio b>=28 ", fst("F3|RT/RE|ge28"));
\\ F4
print("H4/F4: measured kpure per-step M ", f2(STEP_M), " (pred 21.5) S ", f2(STEP_S), " (pred 12) I ", f2(STEP_I), "; relative deviation M ", f2(100 * (STEP_M / 21.5 - 1)), "%, S ", f2(100 * (STEP_S / 12 - 1)), "% (F4 if |dev| > 3%)");
foreach(["a0", "a1"], A,
  my(nm = Str("CR|kmix/", A, "|w0.8|rho3|ge28"));
  print("       kmix/", A, " cost ratio at rho=3 (w=0.8, b>=28): ", fst(nm), if(stok(nm) && st(nm)[3] < 1, "  UPPER BOUND < 1 (improvement condition)", "  not an improvement")));
\\ ordering across band
foreach(PAIRS, pr,
  my(sgn = [], tag = "");
  for(k = 1, 4, my(nm = Str("CR|", pr[1], "/", pr[2], "|w0.8|rho", RHO[k], "|ge28")); v = st(nm);
    sgn = concat(sgn, [if(v[1] >= NA, 0, v[3] < 1, -1, v[2] > 1, 1, 0)]));
  print("       ordering ", pr[1], " vs ", pr[2], " over rho in {3,10,30,100} (-1 K cheaper (CI<1), +1 K dearer (CI>1), 0 unresolved): ", sgn,
        if(sgn == [1,1,1,1], "  => K dearer across the whole band", sgn == [-1,-1,-1,-1], "  => K cheaper across the whole band", "  => UNRANKED across the band")));
\\ F5
my(f5 = 1);
foreach(["kmix", "kpure"], K,
  my(all = 1);
  foreach([32, 36, 40], b, my(nm = Str("CR|", K, "/a1|w0.8|rho3|b", b)); if(!stok(nm) || st(nm)[3] >= 1, all = 0));
  print("H5/F5: ", K, "/a1 at rho=3 (w=0.8) per b: b32 ", fst(Str("CR|", K, "/a1|w0.8|rho3|b32")), " b36 ", fst(Str("CR|", K, "/a1|w0.8|rho3|b36")), " b40 ", fst(Str("CR|", K, "/a1|w0.8|rho3|b40")), "  => CI entirely below 1 at all of b32,36,40: ", if(all, "YES (F5 would be triggered)", "NO"));
  if(all, f5 = 0));
print("H5 headline (no improvement over matched A1): ", if(f5, "NOT falsified", "FALSIFIED"));
}
\\ ------------------------------------------------------------------ results.json
JS = Str(OUT, "/results.json");
externstr(Str("rm -f ", JS));
jn(x) = if(x >= NA, "null", strprintf("%.6g", x));
{
write(JS, "{");
write(JS, Str("  \"experiment\": \"E5-kummer-multiplicative-walk\", \"gp_version\": \"", version(), "\", \"bootstrap_resamples\": ", NBOOT, ", \"bootstrap_seed\": 31337,"));
write(JS, "  \"statistics\": [");
for(i = 1, NST, write(JS, Str("    {\"name\": \"", NAMES[i], "\", \"point\": ", jn(PT[i]), ", \"lo95\": ", jn(LO[i]), ", \"hi95\": ", jn(HI[i]), "}", if(i < NST, ",", ""))));
write(JS, "  ],");
write(JS, "  \"cells\": [");
for(i = 1, NC,
  my(R = RECS[i], nt = #R, ok = 0, sT = 0, hi = mapget(HDR[i], "Hinfo"));
  for(t = 1, nt, ok += R[t][6]; sT += R[t][7]);
  write(JS, Str("    {\"arm\": \"", CL[i][1], "\", \"b\": ", CL[i][2], ", \"inst\": ", CL[i][3], ", \"trials\": ", nt, ", \"successes\": ", ok, ", \"n\": ", NCELLS[i], ", \"Marm\": ", MARM[i], ", \"index_pm_H\": ", hi[5], ", \"mean_T\": ", if(nt, jn(sT / nt * 1.), "null"), "}", if(i < NC, ",", ""))));
write(JS, "  ],");
write(JS, "  \"checks\": {");
write(JS, Str("    \"ladder_tests\": ", LAD_TOT, ", \"ladder_mismatches\": ", LAD_MIS, ", \"nc4_tests\": ", NC4_N, ", \"nc4_wrong_accepted\": ", NC4_WP, ","));
write(JS, Str("    \"class_crossings\": ", CLS_CROSS, ", \"class_mixed_components\": ", CLS_MIXED, ", \"pc2_checks\": ", PC2_TOT, ", \"pc2_mismatches\": ", PC2_BAD, ","));
write(JS, Str("    \"pc1_sb_orbit_mismatches\": ", PC1A_MIS, ", \"pc1_sidx_orbit_mismatches\": ", PC1B_MIS, ", \"pc1_sb_walk\": ", PC1A_WALK, ", \"pc1_sidx_walk\": ", PC1B_WALK, ","));
write(JS, Str("    \"pc5_trials\": ", PC5_N, ", \"pc5_mismatches\": ", PC5_BAD, ", \"nc1_trials\": ", NC1_N, ", \"nc1_cross_matches\": ", NC1_CAND, ", \"nc1_solves\": ", NC1_OK, ", \"nc2_trials\": ", NC2_N, ", \"nc2_cross_matches\": ", NC2_CAND, ","));
write(JS, Str("    \"kpure_M_per_step\": ", jn(STEP_M), ", \"kpure_S_per_step\": ", jn(STEP_S), ", \"kpure_I_per_step\": ", jn(STEP_I), ", \"chain_ops_per_step\": ", jn(CH_OPS), ", \"chain_Mequiv_excl_I\": ", jn(CH_ME)));
write(JS, "  }");
write(JS, "}");
}
print("wrote ", CSV, " ", COSTS, " ", JS);
