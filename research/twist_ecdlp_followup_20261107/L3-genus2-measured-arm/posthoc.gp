\\ posthoc.gp -- POST-FREEZE exploratory diagnostics (listed in DEVIATIONS.md, section "After the code freeze").  Not part of the preregistered analysis.
\\ Env: DATA (raw dir), OUTF (output text file).  Reads only the raw out/*.txt lines.
default(parisizemax, 1000000000);
DATA = getenv("DATA"); OUTF = getenv("OUTF");
setrand(31337);
P(s) = write(OUTF, s);
loadfile(f) = iferr(readvec(f), e, []);
mean(v) = vecsum(v) / #v;
gm(v) = exp(vecsum(vector(#v, i, log(v[i]))) / #v);
olsslope(xs, ys) = { my(mx = mean(xs), my_ = mean(ys)); sum(i = 1, #xs, (xs[i] - mx) * (ys[i] - my_)) / sum(i = 1, #xs, (xs[i] - mx)^2); }
f3(x) = strprintf("%.3f", x);
KS = [8, 9, 10, 11, 12, 13, 14, 15, 16];
write(OUTF, "");
P("# posthoc.txt (exploratory, post-freeze; every number computed from the raw out/*.txt lines)");
P("");
\\ ---- A. NC-A pooled over sizes
P("## A. NC-A pooled glued/random ratio of IC-D relation-phase trials (equal weight per size k in 8,10,12,13); bootstrap B = 2000 resampling instances within each group and size");
{
  my(ks = [8, 10, 12, 13], gl = vector(4), rn = vector(4), diffs, bs = vector(2000), b, i, t, n, s1, s2, dd, srt);
  for(i = 1, 4, my(k = ks[i], L1 = concat(loadfile(Str(DATA, "/main_k", k, "_h1.txt")), loadfile(Str(DATA, "/main_k", k, "_h2.txt"))), L2 = loadfile(Str(DATA, "/ctrl_NCA_k", k, ".txt")), a = List(), c = List());
    for(j = 1, #L1, if(L1[j][1] == "IC" && L1[j][5] == 5, listput(a, log(L1[j][11]))));
    for(j = 1, #L2, if(L2[j][1] == "NCA", listput(c, log(L2[j][11]))));
    gl[i] = Vec(a); rn[i] = Vec(c));
  dd = mean(vector(4, i, mean(gl[i]) - mean(rn[i])));
  for(b = 1, 2000, s2 = 0;
    for(i = 1, 4, n = #gl[i]; s1 = 0; for(t = 1, n, s1 += gl[i][1 + random(n)]); my(m1 = s1 / n); n = #rn[i]; s1 = 0; for(t = 1, n, s1 += rn[i][1 + random(n)]); s2 += m1 - s1 / n);
    bs[b] = s2 / 4);
  srt = vecsort(bs);
  P(Str("pooled ratio glued/random = ", f3(exp(dd)), ", 95% CI [", f3(exp(srt[50])), ", ", f3(exp(srt[1950])), "]; H5 band [0.9, 1.1]; reject band [0.85, 1.15]"));
}
P("");
\\ ---- B. rho diagnostics
P("## B. Rho baseline diagnostics (R_E): cofactor, l, setup share, steps against sqrt(l/2)");
P("");
P("| k | q | mean h | gm l | gm steps | gm sqrt(l/2) | gm steps / gm sqrt(l/2) | mean share of Eops outside walk steps+escapes (table + starts) | IC-D: mean share of Jops in setup (jsetup/jops) |");
P("|---|---|---|---|---|---|---|---|---|");
{
  my(xs = List(), ys1 = List(), ys2 = List());
  for(i = 1, #KS, my(k = KS[i], L = concat(loadfile(Str(DATA, "/main_k", k, "_h1.txt")), loadfile(Str(DATA, "/main_k", k, "_h2.txt"))), hs = List(), ls = List(), st = List(), sl = List(), sh = List(), sd = List(), q = precprime(2^k));
    for(j = 1, #L, my(r = L[j]);
      if(r[1] == "CURVE", listput(hs, r[10]));
      if(r[1] == "RHO" && r[5] == 1, listput(ls, r[7]); listput(st, r[11]); listput(sl, sqrt(r[7] / 2.)); listput(sh, (r[18] - r[11] - r[14] - r[15]) / r[18]));
      if(r[1] == "IC" && r[5] == 5, listput(sd, r[17] / r[12])));
    hs = Vec(hs); ls = Vec(ls); st = Vec(st); sl = Vec(sl); sh = Vec(sh); sd = Vec(sd);
    listput(xs, log(gm(sl))); listput(ys1, log(gm(st)));
    P(Str("| ", k, " | ", q, " | ", f3(mean(hs)), " | ", strprintf("%.1f", gm(ls)), " | ", strprintf("%.1f", gm(st)), " | ", strprintf("%.2f", gm(sl)), " | ", f3(gm(st) / gm(sl)), " | ", f3(mean(sh)), " | ", f3(mean(sd)), " |")));
  P("");
  P(Str("slope of log(gm steps) on log(gm sqrt(l/2)) over k = 8..16: ", f3(olsslope(Vec(xs), Vec(ys1))), " (a sqrt(l/2) law gives 1)"));
}
P("");
\\ ---- C. phase-only ratio (relation/walk steps only, setup excluded): IC-D trials over R_E walk steps
P("## C. Phase-only diagnostic: gm(IC-D trials) / gm(R_E steps + escape doublings) per size (excludes table construction, starts and IC setup; not an arm cost, not used for ranking)");
P("");
P("| k | gm IC-D trials | gm R_E steps+escapes | ratio |");
P("|---|---|---|---|");
{
  my(xs = List(), ys = List());
  for(i = 1, #KS, my(k = KS[i], L = concat(loadfile(Str(DATA, "/main_k", k, "_h1.txt")), loadfile(Str(DATA, "/main_k", k, "_h2.txt"))), a = List(), b = List(), q = precprime(2^k));
    for(j = 1, #L, my(r = L[j]);
      if(r[1] == "IC" && r[5] == 5, listput(a, r[11]));
      if(r[1] == "RHO" && r[5] == 1, listput(b, r[11] + r[14] + r[15])));
    a = Vec(a); b = Vec(b);
    listput(xs, log(q)); listput(ys, log(gm(a) / gm(b)));
    P(Str("| ", k, " | ", strprintf("%.1f", gm(a)), " | ", strprintf("%.1f", gm(b)), " | ", strprintf("%.2f", gm(a) / gm(b)), " |")));
  P("");
  P(Str("slope of the phase-only ratio on log q (k = 8..16): ", f3(olsslope(Vec(xs), Vec(ys)))));
}
quit
