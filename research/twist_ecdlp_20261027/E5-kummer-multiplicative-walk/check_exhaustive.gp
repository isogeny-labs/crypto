default(parisizemax, 2000000000);
read("lib.gp");
read("raw/curves.txt");
print("\\\\ check_exhaustive.gp  gp ", version());
\\ ---- test id 3: exhaustive class test on 3 small fields (p ~ 2^13)
setrand(41000003);
{
for(ci = 1, 3,
  my(p, A, E, par, cls, step, nz, cross = 0, tot = 0, fx, r, x2, f2, comps, hasE, hasT, mixed = 0, ncE = 0, ncT = 0, root, ra, rb);
  while(1, p = randomprime([2^12, 2^13 - 1]); A = random(p - 5) + 3; if(A != 2 && A != p - 2, break));
  A_ = A; P_ = p; A24 = Mod(A + 2, p) / 4; ONE = Mod(1, p);
  par = vector(p, i, i);
  cls = vector(p);
  for(x = 0, p - 1, fx = (x^3 + A * x^2 + x) % p; cls[x + 1] = kronecker(fx, p));
  nz = 0;
  for(x = 0, p - 1,
    if(cls[x + 1] == 0, next);
    nz++;
    foreach(SB, k,
      r = lad(Mod(x, p), kbits(k));
      if(r[2] == 0, next);
      x2 = lift(r[1] / r[2]);
      if(cls[x2 + 1] == 0, next);
      tot++;
      if(cls[x2 + 1] != cls[x + 1], cross++);
      ra = x + 1; while(par[ra] != ra, par[ra] = par[par[ra]]; ra = par[ra]);
      rb = x2 + 1; while(par[rb] != rb, par[rb] = par[par[rb]]; rb = par[rb]);
      if(ra != rb, par[ra] = rb)));
  hasE = vector(p); hasT = vector(p);
  for(x = 0, p - 1,
    if(cls[x + 1] == 0, next);
    root = x + 1; while(par[root] != root, root = par[root]);
    if(cls[x + 1] == 1, hasE[root] = 1, hasT[root] = 1));
  for(i = 1, p, if(hasE[i] && hasT[i], mixed++, if(hasE[i], ncE++, if(hasT[i], ncT++))));
  print("chk(\"classes_", ci, "\",[", p, ",", A, ",", nz, ",", tot, ",", cross, ",", ncE, ",", ncT, ",", mixed, "]);  \\\\ [p, A, x with f!=0, step-map evaluations, class crossings, components E-only, components E'-only, MIXED components]"));
}
\\ ---- test id 4: PC1 exhaustive small instance, b = 10, p ~ 2^13
setrand(41000004);
{
my(rec = gen_inst(10, 1), n, xs, mk, kl, setnames = ["SB", "SIDX"], sets, hi, inH, Hl, pm, inter, ok, mism = 0, mism2 = 0, nin = 0, an, sample, M, st, q, hd, mults, r, T, succ, budg, outn, outs);
print("chk(\"pc1_curve\",", rec, ");");
setctx(rec, 0);
n = NN;
xs = vector(n - 1, e, lift(ellmul(Ea, Pa, e)[1]));
for(si = 1, 2,
  if(si == 1, mults = SB,
    my(fa = factor(n - 1)[, 1], g = 0); for(i = 1, #fa, if(fa[i] % 2 == 1, g = fa[i]; break));
    mults = vector(6, i, lift(Mod([2, 3, 5, 7, 11, 13][i], n)^g)));
  hi = Hinfo(n, mults);
  \\ closure of H under multiplication by the multipliers
  inH = Hclosure(n, mults);
  Hl = select(e -> inH[e], [1..n-1]);
  mk = vector(P_ + 1);
  foreach(Hl, m, mk[xs[m] + 1] = 1);
  mism = 0; mism2 = 0; nin = 0; sample = [];
  for(a = 1, n - 1,
    inter = 0;
    foreach(Hl, m, if(mk[xs[(m * a) % n] + 1], inter = 1; break));
    pm = (inH[a] || inH[n - a]);
    if(inter != pm, mism++);
    if(pm != inpmH(a, hi, n), mism2++);
    if(pm, nin++; if(a >= 2 && a <= n - 2, sample = concat(sample, [a]))));
  print("chk(\"pc1_orbit_set", si, "\",[", n, ",", #Hl, ",", hi[4], ",", hi[5], ",", nin, ",", mism, ",", mism2, "]);  \\\\ [n, |H| by closure, N (znlog), index +-H, #a in +-H, mismatches (x-orbit intersection vs +-H closure), mismatches (closure vs znlog)]");
  if(si == 2 && hi[4] < 64, print("chk(\"pc1_walk_set", si, "\",[-1]);  \\\\ degenerate S_idx (N<64)"); next);
  \\ walks, budget 200 sqrt(N), stratified sample of 400 in-set a
  setwalk(mults, "K");
  BUDGET = floor(200 * sqrt(HI[4]));
  an = min(400, #sample);
  M = vector(an, i, sample[1 + ((i - 1) * #sample) \ an]);
  succ = 0; outn = 0; outs = 0;
  my(tr = 0, dp = 0, stp = 0); foreach(M, a, r = trial_kpure(a, BUDGET, 0); succ += r[1]; tr += r[6]; dp += r[7]; stp += r[2]);
  print("chk(\"pc1_walk_set", si, "\",[", an, ",", succ, ",", BUDGET, ",", HI[4], ",", DPB, ",", tr, ",", dp, ",", stp, "]);  \\\\ [in-set a walked, verified solves, budget steps, N, dpbits, total trails, total DPs, total steps]");
  \\ out-of-set a (if any): none should be recovered
  my(outl = select(a -> !inpmH(a, hi, n), [2..n-2]), no);
  no = min(100, #outl);
  for(i = 1, no, r = trial_kpure(outl[1 + ((i - 1) * #outl) \ no], BUDGET, 0); outs += r[1]);
  print("chk(\"pc1_out_set", si, "\",[", #outl, ",", no, ",", outs, "]);  \\\\ [#a not in +-H, walked, verified solves (expected 0)]");
);
}
\\ ---- test id 5: PC2 orbit size, 20 random P at b = 16, SA / SB / SC
setrand(41000005);
{
my(rec = gen_inst(16, 1), n, hinfo, G, mults, names = ["SA", "SB", "SC"], allm = [SA, SB, SC], Pp, h, Hl, inH, q, szA, szB, visited, queue, qh, qt, r, x0, x1, X);
print("chk(\"pc2_curve\",", rec, ");");
setctx(rec, 0);
n = N0; h = rec[7];
for(pi = 1, 20,
  Pp = [0]; while(Pp == [0], Pp = ellmul(E0, random(E0), h));
  for(si = 1, 3,
    mults = allm[si];
    hinfo = Hinfo(n, mults);
    \\ (a) distinct x([m]P), m in H, H by closure on exponents, points by ellmul (independent of the ladder)
    inH = Hclosure(n, mults);
    visited = Map();
    for(m = 1, n - 1, if(inH[m], mapput(visited, lift(ellmul(E0, Pp, m)[1]), 1)));
    szA = #visited;
    \\ (b) x-only BFS from x(P) with the ladder under every multiplier
    visited = vectorsmall(P_); x0 = lift(Pp[1]); visited[x0 + 1] = 1; queue = vectorsmall(n + 10); qh = 1; qt = 1; queue[1] = x0; szB = 1;
    while(qh <= qt,
      x1 = queue[qh]; qh++;
      foreach(mults, k,
        r = lad(Mod(x1, P_), kbits(k));
        if(r[2] == 0, next);
        X = lift(r[1] / r[2]);
        if(!visited[X + 1], visited[X + 1] = 1; szB++; qt++; queue[qt] = X)));
    print("chk(\"pc2_", names[si], "_", pi, "\",[", hinfo[4], ",", szA, ",", szB, "]);  \\\\ [N predicted, |{x([m]P)}| by ellmul, |BFS orbit| by ladder]")));
}
