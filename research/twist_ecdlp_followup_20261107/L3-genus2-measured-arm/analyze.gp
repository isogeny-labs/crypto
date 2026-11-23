\\ analyze.gp -- analysis of the raw out/*.txt lines.  Env: SRC, DATA (directory with main_k*_h*.txt and ctrl_*.txt), KLIST (comma list, e.g. 8,9,10),
\\ CKLIST_x (control sizes, default as in the preregistration), RES (path of results.json), SUM (path of summary.txt).
\\ Writes summary.txt (markdown tables) and results.json.  No number is typed in here except the preregistered bands/predictions of sections 2-8.
default(parisizemax, 1000000000);
read(Str(getenv("SRC"), "/lib.gp"));
DATA = getenv("DATA"); RESF = getenv("RES"); SUMF = getenv("SUM");
KS = eval(Str("[", getenv("KLIST"), "]"));
BOOT = 2000;
ARMN = ["R_E", "R_J", "IC-G", "IC-S", "IC-D", "IC-F", "plain-rho"];
WS = [0, 0.1, 1];
setrand(31337);

\\ ------------------------------------------------------------------ helpers
W(s) = write(SUMF, s);
f3(x) = strprintf("%.3f", x);
f2(x) = strprintf("%.2f", x);
numstr(x, dg) =
{
  my(e, m, ax);
  if(type(x) == "t_INT", return(Str(x)));
  x = x * 1.;
  if(x == 0, return("0"));
  ax = abs(x); e = floor(log(ax) / log(10)); m = x / 10.^e;
  if(abs(m) >= 10, m /= 10; e++);
  if(abs(m) < 1, m *= 10; e--);
  if(e >= -3 && e <= 6, return(strprintf(Str("%.", max(0, dg - 1 - e), "f"), x)));
  Str(strprintf(Str("%.", dg - 1, "f"), m), "e", e);
}
fg(x) = numstr(x, 4);
mean(v) = if(#v == 0, 0., vecsum(v) / #v);
gmean(v) = if(#v == 0, 0., exp(vecsum(vector(#v, i, log(v[i]))) / #v));
quant(v, p) = { my(s = vecsort(v), n = #s, pos); if(n == 0, return(0)); pos = 1 + p * (n - 1); my(lo = floor(pos), hi = ceil(pos)); s[lo] + (pos - lo) * (s[hi] - s[lo]); }
wilson(s, n) =
{
  my(z = 1.96, p, d, c, h);
  if(n == 0, return([0., 1.]));
  p = s / n; d = 1 + z^2 / n; c = (p + z^2 / (2 * n)) / d; h = z * sqrt(p * (1 - p) / n + z^2 / (4 * n^2)) / d;
  [max(0., c - h), min(1., c + h)];
}
olsslope(xs, ys) = { my(mx = mean(xs), my_ = mean(ys)); sum(i = 1, #xs, (xs[i] - mx) * (ys[i] - my_)) / sum(i = 1, #xs, (xs[i] - mx)^2); }
\\ JSON: objects are ["@o", keys, vals]
OBJ(keys, vals) = ["@o", keys, vals];
tojson(x) =
{
  my(t = type(x), s, i);
  if(t == "t_INT", return(Str(x)));
  if(t == "t_STR", return(Str("\"", x, "\"")));
  if(t == "t_FRAC", x = x * 1.);
  if(t == "t_REAL" || t == "t_FRAC", return(numstr(x, 9)));
  if(t == "t_VEC",
    if(#x == 3 && type(x[1]) == "t_STR" && x[1] == "@o",
      s = "{"; for(i = 1, #x[2], s = Str(s, if(i > 1, ",", ""), "\"", x[2][i], "\":", tojson(x[3][i]))); return(Str(s, "}")));
    s = "["; for(i = 1, #x, s = Str(s, if(i > 1, ",", ""), tojson(x[i]))); return(Str(s, "]")));
  Str("\"", x, "\"");
}
\\ bootstrap 95% CI of exp(mean log ratio): curves resampled, then instances within curve.  lrs = vector of vectors (one per curve)
bootci(lrs, B) =
{
  my(nc = #lrs, m = vector(B), s, n, cc, ni, b, t, u, lr2);
  lr2 = select(v -> #v > 0, lrs); nc = #lr2;
  if(nc == 0, return([0., 0.]));
  for(b = 1, B, s = 0; n = 0;
    for(t = 1, nc, cc = lr2[1 + random(nc)]; ni = #cc; for(u = 1, ni, s += cc[1 + random(ni)]; n++));
    m[b] = s / n);
  m = vecsort(m);
  [exp(m[max(1, ceil(0.025 * B))]), exp(m[min(B, floor(0.975 * B))])];
}

\\ ------------------------------------------------------------------ loading
loadfile(f) = iferr(readvec(f), e, []);
\\ field indices.  RHO: 5 arm, 6 q, 7 l, 8 solved, 9 ver, 10 p1rej, 11 steps, 12 trails, 13 dps, 14 esc2, 15 escB, 16 cand, 17 waste, 18 core, 19 cM, 20 cS, 21 cI, 22 nfb, 23 wall
\\ IC: 11 trials, 12 jops, 13 cM, 14 cS, 15 cI, 16 nfb, 17 jsetup, 18 nsplit, 19 ndeg1, 20 ndeg0, 21 ndisc0, 22 nFF, 23 nFL, 24 nLL, 25 nrej, 26 ndup, 27 nrestart, 28 nsolve, 29 rowops, 30 rowdels, 31 nnz, 32 nsq, 33 nacc, 34 wall
isrho(r) = (r[1] == "RHO" || (#r == 23));
corecost(r) = if(isrho(r), r[18], r[12]);
native(r) =
{
  if(isrho(r), return(if(r[5] == 7, 2 * r[11] + r[13] + r[12], 5 * r[11] + r[13] + r[12])));
  2 * r[11] + r[32] + 2 * r[33] + if(r[5] == 3, 0, 2 * r[18]) + r[33] + r[29] + r[30] + 2;
}
cost(r, w, kap) = { my(c = corecost(r)); if(r[5] != 1, c = kap * c); c + w * native(r); }
censored(r) = (r[8] == 0 || r[9] == 0);

\\ M[k]: Map [c,i,aid] -> line ; INS[k]: list of [c,i] ; LINES[k]
M = Map(); INS = Map(); LINES = Map();
loadsize(k) =
{
  my(lines = concat(loadfile(Str(DATA, "/main_k", k, "_h1.txt")), loadfile(Str(DATA, "/main_k", k, "_h2.txt"))), m = Map(), insts = List(), r, i);
  for(i = 1, #lines, r = lines[i];
    if(r[1] == "RHO" || r[1] == "IC", mapput(m, [r[3], r[4], r[5]], r));
    if(r[1] == "INST", listput(insts, [r[3], r[4]])));
  mapput(M, k, m); mapput(INS, k, Vec(insts)); mapput(LINES, k, lines);
}
rec(k, c, i, a) = { my(m = mapget(M, k)); if(mapisdefined(m, [c, i, a]), mapget(m, [c, i, a]), 0); }
qof(k) = precprime(2^k);
complete(k) = { my(l = mapget(LINES, k), n = 0); for(i = 1, #l, if(l[i][1] == "DONE", n++)); n == 2; }

\\ paired log-ratio vectors per curve for arms A,B at (w, kappa); excluding censored if excl
lrcurves(k, A, B, w, kap, excl) =
{
  my(ins = mapget(INS, k), cs = Set(vector(#ins, j, ins[j][1])), res = vector(#cs), ra, rb, j, t, lst);
  for(t = 1, #cs, lst = List();
    for(j = 1, #ins, if(ins[j][1] != cs[t], next);
      ra = rec(k, ins[j][1], ins[j][2], A); rb = rec(k, ins[j][1], ins[j][2], B);
      if(ra == 0 || rb == 0, next);
      if(excl && (censored(ra) || censored(rb)), next);
      listput(lst, log(cost(ra, w, kap) / cost(rb, w, kap))));
    res[t] = Vec(lst));
  res;
}
flat(v) = { my(o = List()); for(i = 1, #v, for(j = 1, #v[i], listput(o, v[i][j]))); Vec(o); }
\\ ratio record [gm, lo, hi, n]
ratiorec(k, A, B, w, kap, excl, withci) =
{
  my(lr = lrcurves(k, A, B, w, kap, excl), fl = flat(lr), ci);
  if(#fl == 0, return([0., 0., 0., 0]));
  ci = if(withci, bootci(lr, BOOT), [0., 0.]);
  [exp(mean(fl)), ci[1], ci[2], #fl];
}

PAIRS = [[3, 1], [4, 1], [5, 1], [2, 1], [3, 2], [4, 2], [5, 2]];
PAIRN = ["IC-G/R_E", "IC-S/R_E", "IC-D/R_E", "R_J/R_E", "IC-G/R_J", "IC-S/R_J", "IC-D/R_J"];
RAT = Map();   \\ [k, pairindex, widx] -> [gm,lo,hi,n]
RATX = Map();  \\ excluding censored (gm only)
RATK = Map();  \\ secondary kappa: [k, pairindex, widx, rhoindex] gm only
STAT = Map();  \\ [k, arm] -> [att, ver, wilsonlo, wilsonhi, gm_core, med_core, q1, q3, gm_cost0, gm_cost01, gm_cost1, ncens]
kapJ = Map();  \\ k -> [ [kappa3, kappa10, kappa30], psplit, ... ]

armstats(k) =
{
  my(ins = mapget(INS, k), a, j, r, att, ver, cores, c0, c1, c2, w, cens);
  for(a = 1, 5,
    att = 0; ver = 0; cores = List(); c0 = List(); c1 = List(); c2 = List(); cens = 0;
    for(j = 1, #ins, r = rec(k, ins[j][1], ins[j][2], a); if(r == 0, next);
      att++; if(!censored(r), ver++, cens++);
      listput(cores, corecost(r)); listput(c0, cost(r, 0, 1)); listput(c1, cost(r, 0.1, 1)); listput(c2, cost(r, 1, 1)));
    cores = Vec(cores); w = wilson(ver, att);
    mapput(STAT, [k, a], [att, ver, w[1], w[2], if(att, gmean(cores), 0.), if(att, quant(cores, 0.5), 0.), if(att, quant(cores, 0.25), 0.), if(att, quant(cores, 0.75), 0.),
      if(att, gmean(Vec(c0)), 0.), if(att, gmean(Vec(c1)), 0.), if(att, gmean(Vec(c2)), 0.), cens]));
}
kappaps(k) =
{
  my(lines = mapget(LINES, k), sJM = 0, sJS = 0, sJI = 0, sJn = 0, sEM = 0, sES = 0, sEI = 0, sEn = 0, spl = 0, ptr = 0, r, i, kk);
  for(i = 1, #lines, r = lines[i];
    if(r[1] == "IC",
      sJM += r[13]; sJS += r[14]; sJI += r[15]; sJn += r[12] - r[16];
      spl += r[18]; ptr += r[11] - r[26]);
    if(r[1] == "RHO" && r[5] == 1, sEM += r[19]; sES += r[20]; sEI += r[21]; sEn += r[18]));
  kk = if(sJn > 0 && sEn > 0, vector(3, t, my(rr = [3, 10, 30][t]); ((sJM + 0.8 * sJS + rr * sJI) / sJn) / ((sEM + 0.8 * sES + rr * sEI) / sEn)), [0, 0, 0]);
  mapput(kapJ, k, [kk, if(ptr, spl / ptr, 0.), [sJM / max(1, sJn), sJS / max(1, sJn), sJI / max(1, sJn)], [sEM / max(1, sEn), sES / max(1, sEn), sEI / max(1, sEn)], spl, ptr]);
}

\\ ------------------------------------------------------------------ main
for(i = 1, #KS, loadsize(KS[i]));
KSOK = select(k -> #mapget(INS, k) > 0, KS);
write(SUMF, "");
W("# summary.txt (generated by analyze.gp; every number below is computed from the raw out/*.txt lines)");
W("");
W(Str("sizes with data: ", KSOK, "; complete (both halves DONE): ", select(k -> complete(k), KSOK)));
W("");
\\ 1. arm table
W("## Table 1. Verified solves, core operation counts per solve (geometric mean over instances), cost at w = 0, 0.1, 1 GAE per native op (kappa_J = 1)");
W("");
W("| k | q | arm | verified/attempts | Wilson 95% | core ops geomean | median | q1 | q3 | cost w=0 | cost w=0.1 | cost w=1 | censored |");
W("|---|---|---|---|---|---|---|---|---|---|---|---|---|");
{
for(i = 1, #KSOK, my(k = KSOK[i]); armstats(k); kappaps(k);
  for(a = 1, 5, my(s = mapget(STAT, [k, a]));
    W(Str("| ", k, " | ", qof(k), " | ", ARMN[a], " | ", s[2], "/", s[1], " | [", f3(s[3]), ", ", f3(s[4]), "] | ", fg(s[5]), " | ", fg(s[6]), " | ", fg(s[7]), " | ", fg(s[8]), " | ", fg(s[9]), " | ", fg(s[10]), " | ", fg(s[11]), " | ", s[12], " |"))));
}
W("");
\\ 2. paired ratios with CIs
W("## Table 2. Paired cost ratios (geometric mean, 95% bootstrap CI over curves then instances, B = 2000); primary kappa_J = 1; ranking by the 5a rule");
W("");
W("| k | pair | w=0 | w=0.1 | w=1 | ranking across the band | gm w=0 excl. censored |");
W("|---|---|---|---|---|---|---|");
{
for(i = 1, #KSOK, my(k = KSOK[i]);
  for(p = 1, #PAIRS,
    my(rr = vector(3), below = 1, above = 1, rk, rx);
    for(t = 1, 3, rr[t] = ratiorec(k, PAIRS[p][1], PAIRS[p][2], WS[t], 1, 0, 1); mapput(RAT, [k, p, t], rr[t]);
      if(!(rr[t][3] < 1), below = 0); if(!(rr[t][2] > 1), above = 0));
    rx = ratiorec(k, PAIRS[p][1], PAIRS[p][2], 0, 1, 1, 0); mapput(RATX, [k, p], rx);
    rk = if(below, "first arm below", if(above, "first arm above", "unresolved"));
    mapput(RAT, [k, p, 4], rk);
    W(Str("| ", k, " | ", PAIRN[p], " | ", f3(rr[1][1]), " [", f3(rr[1][2]), ", ", f3(rr[1][3]), "] | ", f3(rr[2][1]), " [", f3(rr[2][2]), ", ", f3(rr[2][3]), "] | ", f3(rr[3][1]), " [", f3(rr[3][2]), ", ", f3(rr[3][3]), "] | ", rk, " | ", f3(rx[1]), " (n=", rx[4], ") |"))));
}
W("");
\\ secondary kappa_J = measured ratio of counted field costs
W("## Table 3. Jacobian group operation constant kappa_J (counted field operations of the explicit formulas over those of E additions, S = 0.8 M) and p_split");
W("");
W("| k | kappa_J (I/M=3) | kappa_J (I/M=10) | kappa_J (I/M=30) | J op (M,S,I per op) | E op (M,S,I per op) | p_split pooled | splits | classified trials |");
W("|---|---|---|---|---|---|---|---|---|");
{
for(i = 1, #KSOK, my(k = KSOK[i], kp = mapget(kapJ, k));
  W(Str("| ", k, " | ", f3(kp[1][1]), " | ", f3(kp[1][2]), " | ", f3(kp[1][3]), " | ", f2(kp[3][1]), ", ", f2(kp[3][2]), ", ", f2(kp[3][3]), " | ", f2(kp[4][1]), ", ", f2(kp[4][2]), ", ", f2(kp[4][3]), " | ", f3(kp[2]), " | ", kp[5], " | ", kp[6], " |")));
}
W("");
\\ 3. predictions vs measurements
W("## Table 4. Preregistered band predictions (section 4) against measurements, per size");
W("");
W("| k | q | IC-D T_D/q median [min,max] | frac in [0.40,0.95] | IC-G T_G/q median | IC-S T_S meas/formula (gm) | R_E steps/sqrt(l/2) mean (gm) | R_J mean (gm) | IC-D solve rate |");
W("|---|---|---|---|---|---|---|---|---|");
PRED = Map();
{
for(i = 1, #KSOK, my(k = KSOK[i], q = qof(k), ins = mapget(INS, k), td = List(), tg = List(), ts = List(), rE = List(), rJ = List(), r, j, fin, s5);
  for(j = 1, #ins,
    r = rec(k, ins[j][1], ins[j][2], 5); if(r, listput(td, r[11] / q));
    r = rec(k, ins[j][1], ins[j][2], 3); if(r, listput(tg, r[11] / q));
    r = rec(k, ins[j][1], ins[j][2], 4); if(r, listput(ts, r[11] / tsformula(q)));
    r = rec(k, ins[j][1], ins[j][2], 1); if(r, listput(rE, r[11] / sqrt(r[7] / 2.)));
    r = rec(k, ins[j][1], ins[j][2], 2); if(r, listput(rJ, r[11] / sqrt(r[7] / 2.))));
  td = Vec(td); tg = Vec(tg); ts = Vec(ts); rE = Vec(rE); rJ = Vec(rJ);
  fin = if(#td, #select(x -> x >= 0.40 && x <= 0.95, td) / #td, 0.);
  s5 = mapget(STAT, [k, 5]);
  mapput(PRED, k, [quant(td, 0.5), vecmin(td), vecmax(td), fin, quant(tg, 0.5), gmean(ts), mean(rE), gmean(rE), mean(rJ), gmean(rJ), s5[2] / max(1, s5[1])]);
  W(Str("| ", k, " | ", q, " | ", f3(quant(td, 0.5)), " [", f3(vecmin(td)), ", ", f3(vecmax(td)), "] | ", f3(fin), " | ", f3(quant(tg, 0.5)), " | ", f3(gmean(ts)), " | ", f3(mean(rE)), " (", f3(gmean(rE)), ") | ", f3(mean(rJ)), " (", f3(gmean(rJ)), ") | ", f3(s5[2] / max(1, s5[1])), " |")));
}
W("");
\\ 4. exponent fits (OLS on log q of the geometric mean per size) with bootstrap CI (instances resampled within size)
logvec(k, a, which) =
{
  my(ins = mapget(INS, k), o = List(), r, j);
  for(j = 1, #ins, r = rec(k, ins[j][1], ins[j][2], a); if(r == 0, next);
    listput(o, log(if(which == "trials", r[11], if(which == "steps", r[11], corecost(r))))));
  Vec(o);
}
\\ per-instance log ratios IC-D/R_E at w=0 (flat)
logratio(k, A, B, w) = flat(lrcurves(k, A, B, w, 1, 0));
fitslope(ks, getv, B) =
{
  my(vs = vector(#ks, i, getv(ks[i])), xs = vector(#ks, i, log(qof(ks[i]))), ys, sl, bs = vector(B), b, t, n, s, hold);
  ys = vector(#ks, i, mean(vs[i])); sl = olsslope(xs, ys);
  for(b = 1, B,
    ys = vector(#ks, i, n = #vs[i]; s = 0; for(t = 1, n, s += vs[i][1 + random(n)]); s / n);
    bs[b] = olsslope(xs, ys));
  bs = vecsort(bs);
  [sl, bs[max(1, ceil(0.025 * B))], bs[min(B, floor(0.975 * B))]];
}
FITS = Map();
W("## Table 5. Fitted exponents (OLS of log geometric mean on log q; 95% bootstrap CI resampling instances within size)");
W("");
W("| quantity | k range | slope | 95% CI | preregistered band |");
W("|---|---|---|---|---|");
dofit(name, fn, ks, band) =
{
  my(f, ok);
  if(#ks < 3, return(0));
  f = fitslope(ks, fn, BOOT);
  mapput(FITS, [name, ks[1]], f);
  W(Str("| ", name, " | ", ks[1], "..", ks[#ks], " | ", f3(f[1]), " | [", f3(f[2]), ", ", f3(f[3]), "] | ", band, " |"));
  f;
}
KS10 = select(k -> k >= 10, KSOK);
dofit("IC-D trials (T_D)", k -> logvec(k, 5, "trials"), KSOK, "F2: [0.85,1.15]; H2: [0.9,1.1]");
dofit("IC-D trials (T_D)", k -> logvec(k, 5, "trials"), KS10, "H2: [0.9,1.1]");
dofit("IC-G trials (T_G)", k -> logvec(k, 3, "trials"), KSOK, "~1");
dofit("IC-S trials (T_S)", k -> logvec(k, 4, "trials"), KSOK, "6/5 asymptotically (not banded)");
dofit("R_E steps", k -> logvec(k, 1, "steps"), KSOK, "F2: [0.4,0.6]");
dofit("R_E steps", k -> logvec(k, 1, "steps"), KS10, "F2: [0.4,0.6]");
dofit("R_J steps", k -> logvec(k, 2, "steps"), KSOK, "[0.4,0.6]");
dofit("IC-D core Jops", k -> logvec(k, 5, "core"), KSOK, "not banded");
dofit("R_E core Eops", k -> logvec(k, 1, "core"), KSOK, "not banded");
dofit("ratio IC-D/R_E (w=0, kappa_J=1)", k -> logratio(k, 5, 1, 0), KS10, "H4: [0.35,0.60]");
dofit("ratio IC-D/R_E (w=0, kappa_J=1)", k -> logratio(k, 5, 1, 0), KSOK, "not banded");
dofit("ratio IC-D/R_E (w=1, kappa_J=1)", k -> logratio(k, 5, 1, 1), KS10, "not banded");
dofit("ratio IC-G/R_E (w=0, kappa_J=1)", k -> logratio(k, 3, 1, 0), KS10, "not banded");
dofit("ratio IC-S/R_E (w=0, kappa_J=1)", k -> logratio(k, 4, 1, 0), KS10, "not banded");
W("");
\\ 5. distance remaining
W("## Table 6. Distance remaining: geometric-mean ratio to the matched rho R_E (kappa_J = 1)");
W("");
W("| k | IC-D/R_E w=0 | w=0.1 | w=1 | R_J/R_E w=0 | w=0.1 | w=1 |");
W("|---|---|---|---|---|---|---|");
{
for(i = 1, #KSOK, my(k = KSOK[i]);
  W(Str("| ", k, " | ", f3(mapget(RAT, [k, 3, 1])[1]), " | ", f3(mapget(RAT, [k, 3, 2])[1]), " | ", f3(mapget(RAT, [k, 3, 3])[1]), " | ", f3(mapget(RAT, [k, 4, 1])[1]), " | ", f3(mapget(RAT, [k, 4, 2])[1]), " | ", f3(mapget(RAT, [k, 4, 3])[1]), " |")));
}
W("");
\\ secondary kappa_J (measured, rho = 10): IC-D / R_E point ratio
W("## Table 7. Secondary pricing: Jop priced at the measured kappa_J (I/M = 3, 10, 30), IC-D/R_E geometric-mean ratio at w = 0 (point estimates)");
W("");
W("| k | kappa=1 | kappa(I/M=3) | kappa(I/M=10) | kappa(I/M=30) |");
W("|---|---|---|---|---|");
{
for(i = 1, #KSOK, my(k = KSOK[i], kp = mapget(kapJ, k)[1]);
  W(Str("| ", k, " | ", f3(ratiorec(k, 5, 1, 0, 1, 0, 0)[1]), " | ", f3(ratiorec(k, 5, 1, 0, kp[1], 0, 0)[1]), " | ", f3(ratiorec(k, 5, 1, 0, kp[2], 0, 0)[1]), " | ", f3(ratiorec(k, 5, 1, 0, kp[3], 0, 0)[1]), " |")));
}
W("");
\\ ------------------------------------------------------------------ transfer / consistency checks (CHK lines) and GEN lines
W("## Table 8. Per-size construction and checks (all curves)");
W("");
W("| k | curves | candidate E sampled | rejections by filter step (pattern3, cofactor, l|E', g2=0, singular, noWpt, #C'!=q+1, other) | #C'=q+1 failures | N*D!=0 | ref-Cantor mismatches / pairs | assoc fails | transfer hom fails / pairs | T(G),T(H) checks failed / total |");
W("|---|---|---|---|---|---|---|---|---|---|");
CHKTOT = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0];  \\ okd failures, nbadN, nN, nref, nbadref, nassoc, nbadassoc, nhom, nbadhom, nbadT (+ nT separately)
NTT = 0;
{
for(i = 1, #KSOK, my(k = KSOK[i], lines = mapget(LINES, k), nc = 0, bad_d = 0, bN = 0, tN = 0, nr = 0, br = 0, na = 0, ba = 0, nh = 0, bh = 0, nT = 0, bT = 0, gen = 0, r, j);
  for(j = 1, #lines, r = lines[j];
    if(r[1] == "CHK", nc++; if(!r[4], bad_d++); bN += r[5]; tN += r[6]; nr += r[7]; br += r[8]; na += r[9]; ba += r[10]; nh += r[11]; bh += r[12]; nT += r[13]; bT += r[14]);
    if(r[1] == "GEN", gen = r));
  CHKTOT += [bad_d, bN, tN, nr, br, na, ba, nh, bh, bT]; NTT += nT;
  W(Str("| ", k, " | ", nc, " | ", if(gen, gen[4], "?"), " (singular ", if(gen, gen[5], "?"), ", dup ", if(gen, gen[6], "?"), ") | ", if(gen, gen[7], "?"), " | ", bad_d, " | ", bN, "/", tN, " | ", br, "/", nr, " | ", ba, "/", na, " | ", bh, "/", nh, " | ", bT, "/", nT, " |")));
}
W("");
\\ ------------------------------------------------------------------ controls
ctrlkeys(tag) = if(tag == "PCA", [8, 10, 12, 14], if(tag == "PCC", [8, 10, 12], if(tag == "NCA", [8, 10, 12, 13], if(tag == "NCB", [8, 10, 12], [8, 10]))));
CTRLS = Map(); \\ [tag,k] -> lines
{
for(t = 1, 5, my(tag = ["PCA", "PCC", "NCA", "NCB", "NCC"][t]);
  for(i = 1, #ctrlkeys(tag), my(k = ctrlkeys(tag)[i], v = loadfile(Str(DATA, "/ctrl_", tag, "_k", k, ".txt")));
    mapput(CTRLS, [tag, k], v)));
}
ctrlmap(tag, k) =
{
  my(v = mapget(CTRLS, [tag, k]), m = Map(), insts = Set(), r, i, done = 0);
  for(i = 1, #v, r = v[i];
    if(r[1] == tag, mapput(m, [r[3], r[4], r[5]], r); insts = setunion(insts, Set([[r[3], r[4]]])));
    if(r[1] == "DONE", done = 1));
  [m, insts, done];
}
cget(m, c, i, a) = if(mapisdefined(m, [c, i, a]), mapget(m, [c, i, a]), 0);
\\ paired bootstrap ratio for control data: A over B at weight w, groups by curve
cratio(m, insts, A, B, w, excl, withci) =
{
  my(cs = Set(vector(#insts, j, insts[j][1])), res = vector(#cs), lst, ra, rb, fl, ci);
  for(t = 1, #cs, lst = List();
    for(j = 1, #insts, if(insts[j][1] != cs[t], next);
      ra = cget(m, insts[j][1], insts[j][2], A); rb = cget(m, insts[j][1], insts[j][2], B);
      if(ra == 0 || rb == 0, next);
      listput(lst, log(cost(ra, w, 1) / cost(rb, w, 1))));
    res[t] = Vec(lst));
  fl = flat(res);
  if(#fl == 0, return([0., 0., 0., 0]));
  ci = if(withci, bootci(res, BOOT), [0., 0.]);
  [exp(mean(fl)), ci[1], ci[2], #fl];
}
CTRLRES = Map();
\\ PC-A
W("## Table 9. PC-A (IC-F against IC-D: can the harness resolve a q^(1/2) gap in operation counts?)");
W("");
W("| k | q | instances | IC-F trials gm | IC-D trials gm | measured ratio F/D (paired gm) | formula sqrt(q)/2 | measured/formula | IC-F T_F measured/formula (q^1.5/4) | solves F | solves D |");
W("|---|---|---|---|---|---|---|---|---|---|---|");
pcxs = List(); pcys = List();
{
for(i = 1, #ctrlkeys("PCA"), my(k = ctrlkeys("PCA")[i], cm = ctrlmap("PCA", k), m = cm[1], ins = cm[2], q = qof(k), tf = List(), td = List(), lr = List(), r1, r2, sF = 0, sD = 0, n = 0);
  for(j = 1, #ins, r1 = cget(m, ins[j][1], ins[j][2], 6); r2 = cget(m, ins[j][1], ins[j][2], 5);
    if(r1 == 0 || r2 == 0, next); n++;
    listput(tf, r1[11]); listput(td, r2[11]); listput(lr, log(r1[11] / r2[11]));
    if(!censored(r1), sF++); if(!censored(r2), sD++));
  if(n == 0, next);
  my(rat = exp(mean(Vec(lr))), form = sqrt(q) / 2., tfm = gmean(Vec(tf)) / (q^1.5 / 4));
  listput(pcxs, log(q)); listput(pcys, log(rat));
  mapput(CTRLRES, ["PCA", k], [n, gmean(Vec(tf)), gmean(Vec(td)), rat, form, rat / form, tfm, sF, sD]);
  W(Str("| ", k, " | ", q, " | ", n, " | ", fg(gmean(Vec(tf))), " | ", fg(gmean(Vec(td))), " | ", f3(rat), " | ", f3(form), " | ", f3(rat / form), " | ", f3(tfm), " | ", sF, "/", n, " | ", sD, "/", n, " |")));
}
PCASLOPE = if(#pcxs >= 3, olsslope(Vec(pcxs), Vec(pcys)), 0.);
W("");
W(Str("PC-A fitted slope of log(ratio F/D) on log q: ", f3(PCASLOPE), " (pass band [0.35, 0.65]; per-size measured/formula pass band [0.5, 2])"));
W("");
\\ PC-B
W("## Table 10. PC-B (planted-answer chain): verified-solve rates of the rho arms, per size");
W("");
W("| k | R_E verified | R_J verified | IC-G | IC-S | IC-D | x'+1 rejected / verified solves (all arms) |");
W("|---|---|---|---|---|---|---|");
PCB_RHO = [0, 0]; PCB_P1 = [0, 0];
{
for(i = 1, #KSOK, my(k = KSOK[i], s = vector(5, a, mapget(STAT, [k, a])), lines = mapget(LINES, k), v1 = 0, r1 = 0);
  for(j = 1, #lines, my(r = lines[j]); if((r[1] == "RHO" || r[1] == "IC") && r[8] && r[9], v1++; if(r[10] == 1, r1++)));
  PCB_RHO += [s[1][2] + s[2][2], s[1][1] + s[2][1]]; PCB_P1 += [r1, v1];
  W(Str("| ", k, " | ", s[1][2], "/", s[1][1], " | ", s[2][2], "/", s[2][1], " | ", s[3][2], "/", s[3][1], " | ", s[4][2], "/", s[4][1], " | ", s[5][2], "/", s[5][1], " | ", r1, "/", v1, " |")));
}
W("");
\\ PC-C
W("## Table 11. PC-C (can the pipeline rank a genus-2 index-calculus arm as the winner? IC-D against plain rho in a random Jacobian)");
W("");
W("| k | q | instances | IC-D/plain w=0 | w=0.1 | w=1 | upper CI < 1 at all w | IC-D solves | plain solves |");
W("|---|---|---|---|---|---|---|---|---|");
PCC_PASS = Map();
{
for(i = 1, #ctrlkeys("PCC"), my(k = ctrlkeys("PCC")[i], cm = ctrlmap("PCC", k), rr = vector(3), below = 1, sD = 0, sP = 0, n = 0, q = qof(k));
  if(#cm[2] == 0, next);
  for(t = 1, 3, rr[t] = cratio(cm[1], cm[2], 5, 7, WS[t], 0, 1); if(!(rr[t][3] < 1), below = 0));
  for(j = 1, #cm[2], my(r1 = cget(cm[1], cm[2][j][1], cm[2][j][2], 5), r2 = cget(cm[1], cm[2][j][1], cm[2][j][2], 7)); if(r1 == 0 || r2 == 0, next); n++; if(!censored(r1), sD++); if(r2[8] && r2[9], sP++));
  mapput(CTRLRES, ["PCC", k], [n, rr, below, sD, sP]);
  W(Str("| ", k, " | ", q, " | ", n, " | ", f3(rr[1][1]), " [", f3(rr[1][2]), ", ", f3(rr[1][3]), "] | ", f3(rr[2][1]), " [", f3(rr[2][2]), ", ", f3(rr[2][3]), "] | ", f3(rr[3][1]), " [", f3(rr[3][2]), ", ", f3(rr[3][3]), "] | ", if(below, "yes", "no"), " | ", sD, "/", n, " | ", sP, "/", n, " |")));
}
W("");
\\ NC-A
W("## Table 12. NC-A (null population: does IC-D profit from the gluing? glued / random-Jacobian ratio of IC-D relation-phase trials)");
W("");
W("| k | n glued | n random | glued trials gm | random trials gm | ratio glued/random | 95% CI (bootstrap) | CI inside [0.9,1.1] | CI disjoint from [0.85,1.15] | CI not inside [0.85,1.15] | random IC-D solves |");
W("|---|---|---|---|---|---|---|---|---|---|---|");
NCA_FAIL = 0; NCA_STRICT = 0;
{
for(i = 1, #ctrlkeys("NCA"), my(k = ctrlkeys("NCA")[i], cm = ctrlmap("NCA", k), gl = List(), rn = List(), ins = if(mapisdefined(INS, k), mapget(INS, k), []), bs, b, sg, sr, ci, ratio, nA, nB, inH5, disj, strict, sRnd = 0, m1, m2);
  if(#cm[2] == 0 || !mapisdefined(INS, k), next);
  for(j = 1, #ins, my(r = rec(k, ins[j][1], ins[j][2], 5)); if(r, listput(gl, log(r[11]))));
  for(j = 1, #cm[2], my(r = cget(cm[1], cm[2][j][1], cm[2][j][2], 5)); if(r, listput(rn, log(r[11])); if(!censored(r), sRnd++)));
  gl = Vec(gl); rn = Vec(rn); nA = #gl; nB = #rn;
  if(nA == 0 || nB == 0, next);
  bs = vector(BOOT);
  for(b = 1, BOOT, sg = 0; for(t = 1, nA, sg += gl[1 + random(nA)]); sr = 0; for(t = 1, nB, sr += rn[1 + random(nB)]); bs[b] = sg / nA - sr / nB);
  bs = vecsort(bs); ci = [exp(bs[ceil(0.025 * BOOT)]), exp(bs[floor(0.975 * BOOT)])];
  ratio = exp(mean(gl) - mean(rn));
  inH5 = (ci[1] >= 0.9 && ci[2] <= 1.1); disj = (ci[2] < 0.85 || ci[1] > 1.15); strict = !(ci[1] >= 0.85 && ci[2] <= 1.15);
  if(disj, NCA_FAIL++); if(strict, NCA_STRICT++);
  mapput(CTRLRES, ["NCA", k], [nA, nB, exp(mean(gl)), exp(mean(rn)), ratio, ci, inH5, disj, strict, sRnd]);
  W(Str("| ", k, " | ", nA, " | ", nB, " | ", f2(exp(mean(gl))), " | ", f2(exp(mean(rn))), " | ", f3(ratio), " | [", f3(ci[1]), ", ", f3(ci[2]), "] | ", if(inH5, "yes", "no"), " | ", if(disj, "yes", "no"), " | ", if(strict, "yes", "no"), " | ", sRnd, "/", nB, " |")));
}
W("");
\\ NC-B, NC-C
W("## Table 13. NC-B (wrong-answer detector) and NC-C (sabotaged transfer)");
W("");
W("| control | k | arm | instances | returned a value | verified (must be 0) |");
W("|---|---|---|---|---|---|");
NCB_VER = 0; NCC_VER = 0; NCB_RET = 0;
{
for(i = 1, #ctrlkeys("NCB"), my(k = ctrlkeys("NCB")[i], v = mapget(CTRLS, ["NCB", k]), n5 = 0, ret5 = 0, ver5 = 0, n1 = 0, ret1 = 0, ver1 = 0);
  for(j = 1, #v, my(r = v[j]); if(r[1] != "NCB", next);
    if(r[5] == 5, n5++; if(r[8], ret5++); if(r[9], ver5++));
    if(r[5] == 1, n1++; if(r[8], ret1++); if(r[9], ver1++)));
  NCB_VER += ver5 + ver1; NCB_RET += ret5 + ret1;
  W(Str("| NC-B | ", k, " | IC-D | ", n5, " | ", ret5, " | ", ver5, " |"));
  W(Str("| NC-B | ", k, " | R_E | ", n1, " | ", ret1, " | ", ver1, " |")));
}
{
for(i = 1, #ctrlkeys("NCC"), my(k = ctrlkeys("NCC")[i], v = mapget(CTRLS, ["NCC", k]), n5 = 0, ret5 = 0, ver5 = 0);
  for(j = 1, #v, my(r = v[j]); if(r[1] != "NCC", next); n5++; if(r[8], ret5++); if(r[9], ver5++));
  NCC_VER += ver5;
  W(Str("| NC-C | ", k, " | IC-D with flipped T(H) | ", n5, " | ", ret5, " | ", ver5, " |")));
}
W("");
\\ ------------------------------------------------------------------ falsifiers and band checks
W("## Table 14. Falsifiers and preregistered band checks (PASS = inside the band / not fired; MISS = outside / fired)");
W("");
W("| item | measured | rule | status |");
W("|---|---|---|---|");
FALS = List();
frow(item, meas, rule, ok) = { W(Str("| ", item, " | ", meas, " | ", rule, " | ", if(ok, "PASS", "MISS"), " |")); listput(FALS, [item, meas, rule, if(ok, "PASS", "MISS")]); }
\\ F1
F1HITS = List();
{
for(i = 1, #KSOK, my(k = KSOK[i]);
  for(p = 1, 3, my(hit = 1); for(t = 1, 3, if(!(mapget(RAT, [k, p, t])[3] < 1), hit = 0));
    if(hit, listput(F1HITS, [k, PAIRN[p]]))));
}
frow("F1 (any IC arm with upper CI of IC/R_E < 1 at every w, any size)", if(#F1HITS == 0, "no hit", Str(Vec(F1HITS))), "must not fire (a hit triggers replication on seed + 10^10)", #F1HITS == 0);
\\ F2 / exponents
fit1 = if(mapisdefined(FITS, ["IC-D trials (T_D)", KSOK[1]]), mapget(FITS, ["IC-D trials (T_D)", KSOK[1]]), 0);
if(fit1, frow("F2 IC-D T_D slope (k all)", Str(f3(fit1[1]), " [", f3(fit1[2]), ", ", f3(fit1[3]), "]"), "[0.85, 1.15]", fit1[1] >= 0.85 && fit1[1] <= 1.15));
fit2 = if(mapisdefined(FITS, ["R_E steps", KSOK[1]]), mapget(FITS, ["R_E steps", KSOK[1]]), 0);
if(fit2, frow("F2 R_E steps slope (k all)", Str(f3(fit2[1]), " [", f3(fit2[2]), ", ", f3(fit2[3]), "]"), "[0.4, 0.6]", fit2[1] >= 0.4 && fit2[1] <= 0.6));
{
if(#KS10 >= 3,
  fit3 = mapget(FITS, ["ratio IC-D/R_E (w=0, kappa_J=1)", KS10[1]]);
  frow("H4 slope of IC-D/R_E on q (k >= 10, w=0)", Str(f3(fit3[1]), " [", f3(fit3[2]), ", ", f3(fit3[3]), "]"), "[0.35, 0.60]", fit3[1] >= 0.35 && fit3[1] <= 0.60));
}
\\ F3 bands per size
{
for(i = 1, #KSOK, my(k = KSOK[i], pr = mapget(PRED, k), kp = mapget(kapJ, k));
  frow(Str("T_D/q median, k=", k), f3(pr[1]), if(k >= 10, "[0.45, 0.80]", "[0.40, 0.95]"), if(k >= 10, pr[1] >= 0.45 && pr[1] <= 0.80, pr[1] >= 0.40 && pr[1] <= 0.95));
  frow(Str("T_G/q median, k=", k), f3(pr[5]), "[0.95, 1.25]", pr[5] >= 0.95 && pr[5] <= 1.25);
  frow(Str("T_S measured/formula (gm), k=", k), f3(pr[6]), "[0.6, 1.7]", pr[6] >= 0.6 && pr[6] <= 1.7);
  if(k >= 10, frow(Str("R_E mean steps/sqrt(l/2), k=", k), f3(pr[7]), "[1.2, 1.9] (k >= 10)", pr[7] >= 1.2 && pr[7] <= 1.9);
    frow(Str("R_J mean steps/sqrt(l/2), k=", k), f3(pr[9]), "[1.2, 1.9] (k >= 10)", pr[9] >= 1.2 && pr[9] <= 1.9));
  my(s1 = mapget(STAT, [k, 1]), s2 = mapget(STAT, [k, 2]));
  frow(Str("PC-B rho verified-solve rate R_E, k=", k), Str(s1[2], "/", s1[1]), ">= 0.95", s1[2] >= 0.95 * s1[1]);
  frow(Str("PC-B rho verified-solve rate R_J, k=", k), Str(s2[2], "/", s2[1]), ">= 0.95", s2[2] >= 0.95 * s2[1]);
  frow(Str("p_split pooled, k=", k), f3(kp[2]), "[0.46, 0.54]", kp[2] >= 0.46 && kp[2] <= 0.54);
  frow(Str("kappa_J (I/M=10), k=", k), f3(kp[1][2]), "[2.0, 4.5]", kp[1][2] >= 2.0 && kp[1][2] <= 4.5);
  frow(Str("IC-D solve rate (F6), k=", k), f3(pr[11]), ">= 0.95", pr[11] >= 0.95));
}
\\ H4 point values
{
for(i = 1, #KSOK, my(k = KSOK[i], mn = 1e30);
  for(p = 1, 3, for(t = 1, 3, mn = min(mn, mapget(RAT, [k, p, t])[1])));
  frow(Str("H4 every IC arm / R_E > 1 at all w (min gm ratio), k=", k), f3(mn), "> 1", mn > 1));
}
if(mapisdefined(RAT, [8, 3, 1]), frow("H4 point value IC-D/R_E at k=8 (w=0)", f3(mapget(RAT, [8, 3, 1])[1]), ">= 5", mapget(RAT, [8, 3, 1])[1] >= 5));
if(mapisdefined(RAT, [16, 3, 1]), frow("H4 point value IC-D/R_E at k=16 (w=0)", f3(mapget(RAT, [16, 3, 1])[1]), ">= 30", mapget(RAT, [16, 3, 1])[1] >= 30));
\\ pooled R over k >= 10
{
  my(sE = List(), sJ = List(), vE = List(), vJ = List(), ins, r);
  for(i = 1, #KS10, my(k = KS10[i]); ins = mapget(INS, k);
    for(j = 1, #ins,
      r = rec(k, ins[j][1], ins[j][2], 1); if(r, listput(sE, r[11] / sqrt(r[7] / 2.)); if(!censored(r), listput(vE, r[11] / sqrt(r[7] / 2.))));
      r = rec(k, ins[j][1], ins[j][2], 2); if(r, listput(sJ, r[11] / sqrt(r[7] / 2.)); if(!censored(r), listput(vJ, r[11] / sqrt(r[7] / 2.))))));
  sE = Vec(sE); sJ = Vec(sJ); vE = Vec(vE); vJ = Vec(vJ);
  if(#sE, frow("R_E pooled mean steps/sqrt(l/2), k >= 10 (all instances / verified only)", Str(f3(mean(sE)), " / ", f3(mean(vE))), "[1.2, 1.9]", mean(sE) >= 1.2 && mean(sE) <= 1.9);
    frow("R_J pooled mean steps/sqrt(l/2), k >= 10 (all instances / verified only)", Str(f3(mean(sJ)), " / ", f3(mean(vJ))), "[1.2, 1.9]", mean(sJ) >= 1.2 && mean(sJ) <= 1.9));
}
\\ controls
pcaok = (#pcxs >= 3);
{
if(pcaok,
  my(allin = 1); for(i = 1, #ctrlkeys("PCA"), my(k = ctrlkeys("PCA")[i]); if(mapisdefined(CTRLRES, ["PCA", k]), my(z = mapget(CTRLRES, ["PCA", k])); if(!(z[6] >= 0.5 && z[6] <= 2), allin = 0)));
  frow("PC-A measured/formula in [0.5, 2] at every size", if(allin, "all in band", "some outside"), "[0.5, 2]", allin);
  frow("PC-A fitted slope", f3(PCASLOPE), "[0.35, 0.65]", PCASLOPE >= 0.35 && PCASLOPE <= 0.65);
  my(tfall = 1); for(i = 1, #ctrlkeys("PCA"), my(k = ctrlkeys("PCA")[i]); if(mapisdefined(CTRLRES, ["PCA", k]), my(z = mapget(CTRLRES, ["PCA", k])); if(!(z[7] >= 0.5 && z[7] <= 2), tfall = 0)));
  frow("T_F measured/formula in [0.5, 2] at every PC-A size", if(tfall, "all in band", "some outside"), "[0.5, 2]", tfall));
}
frow("PC-B rho verified-solve rate (R_E + R_J, all sizes)", Str(PCB_RHO[1], "/", PCB_RHO[2]), ">= 0.95", PCB_RHO[2] > 0 && PCB_RHO[1] / PCB_RHO[2] >= 0.95);
frow("PC-B transfer checks, Jacobian cross-checks", Str(CHKTOT[1] + CHKTOT[2] + CHKTOT[5] + CHKTOT[7] + CHKTOT[9] + CHKTOT[10], " failures"), "0 (100%)", CHKTOT[1] + CHKTOT[2] + CHKTOT[5] + CHKTOT[7] + CHKTOT[9] + CHKTOT[10] == 0);
{
for(i = 1, #ctrlkeys("PCC"), my(k = ctrlkeys("PCC")[i]);
  if(k >= 10 && mapisdefined(CTRLRES, ["PCC", k]), my(z = mapget(CTRLRES, ["PCC", k]));
    frow(Str("PC-C IC-D below plain rho (upper CI < 1, all w), k=", k), Str(z[3]), "yes", z[3] == 1);
    frow(Str("PC-C IC-D/plain at w=0, k=", k), f3(z[2][1][1]), "[0.2, 0.8]", z[2][1][1] >= 0.2 && z[2][1][1] <= 0.8)));
}
frow("NC-A (literal: CI disjoint from [0.85,1.15] at any size)", Str(NCA_FAIL, " sizes"), "0 sizes", NCA_FAIL == 0);
frow("NC-A (strict: CI not inside [0.85,1.15] at any size)", Str(NCA_STRICT, " sizes"), "0 sizes", NCA_STRICT == 0);
frow("NC-B verified returned values", Str(NCB_VER, " of ", NCB_RET, " returned"), "0 (100% rejected)", NCB_VER == 0);
frow("NC-B x'+1 rejected for every verified solve (main)", Str(PCB_P1[1], "/", PCB_P1[2]), "100%", PCB_P1[1] == PCB_P1[2]);
frow("NC-C verified solves with sabotaged T(H)", Str(NCC_VER), "0", NCC_VER == 0);
W("");

\\ ------------------------------------------------------------------ results.json
buildjson() =
{
  my(persz = List(), pszk = List(), fitk = List(), fitv = List(), ctrlk = List(), ctrlv = List(), falk = List(), falv = List(), fm, cm, i, RESOBJ);
  for(i = 1, #KSOK, my(k = KSOK[i], arms = List(), armk = List(), rats = List(), ratk = List(), pr = mapget(PRED, k), kp = mapget(kapJ, k));
    for(a = 1, 5, my(s = mapget(STAT, [k, a]));
      listput(armk, ARMN[a]);
      listput(arms, OBJ(["attempts", "verified", "wilson_lo", "wilson_hi", "core_gm", "core_median", "core_q1", "core_q3", "cost_w0_gm", "cost_w01_gm", "cost_w1_gm", "censored"],
        [s[1], s[2], s[3], s[4], s[5], s[6], s[7], s[8], s[9], s[10], s[11], s[12]])));
    for(p = 1, #PAIRS,
      my(o = vector(3, t, mapget(RAT, [k, p, t])));
      listput(ratk, PAIRN[p]);
      listput(rats, OBJ(["w0", "w01", "w1", "ranking", "gm_w0_excl_censored"],
        [OBJ(["gm", "lo", "hi", "n"], o[1]), OBJ(["gm", "lo", "hi", "n"], o[2]), OBJ(["gm", "lo", "hi", "n"], o[3]), mapget(RAT, [k, p, 4]), mapget(RATX, [k, p])[1]])));
    listput(pszk, Str("k", k));
    listput(persz, OBJ(["q", "instances", "complete", "arms", "ratios", "pred", "kappa_J", "p_split"],
      [qof(k), #mapget(INS, k), complete(k), OBJ(Vec(armk), Vec(arms)), OBJ(Vec(ratk), Vec(rats)),
       OBJ(["TD_over_q_median", "TD_min", "TD_max", "TD_frac_in_band", "TG_over_q_median", "TS_meas_over_formula_gm", "R_E_mean", "R_E_gm", "R_J_mean", "R_J_gm", "ICD_solve_rate"], pr),
       OBJ(["IM3", "IM10", "IM30"], kp[1]), kp[2]])));
  if(#FITS > 0, fm = Mat(FITS);
    for(i = 1, #fm[, 1], listput(fitk, Str(fm[i, 1][1], " k>=", fm[i, 1][2])); listput(fitv, OBJ(["slope", "lo", "hi"], fm[i, 2]))));
  if(#CTRLRES > 0, cm = Mat(CTRLRES);
    for(i = 1, #cm[, 1], listput(ctrlk, Str(cm[i, 1][1], " k=", cm[i, 1][2])); listput(ctrlv, cm[i, 2])));
  for(i = 1, #FALS, listput(falk, Str(i)); listput(falv, OBJ(["item", "measured", "rule", "status"], FALS[i])));
  RESOBJ = OBJ(["meta", "sizes", "per_size", "fits", "controls", "falsifiers", "checks_total", "pca_slope"],
    [OBJ(["bootstrap_B", "seed", "w_band", "primary_kappa_J", "units"], [BOOT, 31337, WS, 1, "operation counts (Eop for R_E; Jop for all Jacobian arms); wall times are in the raw files only and are contention-affected"]),
     KSOK, OBJ(Vec(pszk), Vec(persz)), OBJ(Vec(fitk), Vec(fitv)), OBJ(Vec(ctrlk), Vec(ctrlv)), OBJ(Vec(falk), Vec(falv)), CHKTOT, PCASLOPE]);
  write1(RESF, tojson(RESOBJ));
}
buildjson();
quit
