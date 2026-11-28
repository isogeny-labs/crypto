\\ controls.gp -- positive and negative controls.  Env: SRC, CTRL in {PCA, PCC, NCA, NCB, NCC}, K, OUT, MSEED (base master seed).
\\ Control master seed = base + tag * 10^9 (PC-A 1, PC-C 2, NC-A 3, NC-B 4, NC-C 5).
default(parisizemax, 1000000000);
read(Str(getenv("SRC"), "/lib.gp"));
CTRL = getenv("CTRL"); KK = eval(getenv("K")); OUTF = getenv("OUT"); BASEM = eval(getenv("MSEED"));
CTAG = if(CTRL == "PCA", 1, if(CTRL == "PCC", 2, if(CTRL == "NCA", 3, if(CTRL == "NCB", 4, 5))));
MASTER = BASEM + CTAG * 10^9;

\\ random genus-2 curve y^2 = X^5 + f3 X^3 + f2 X^2 + f1 X + f0 with condition cond ("PCC": N = h' l'' h' <= 4; "NCA": largest prime factor >= q^2/32)
\\ returns [f3,f2,f1,f0,N,l] or 0 after maxtry failures
rand_curveJ(q, cond, maxtry) =
{
  my(f, N, fa, l, t, f3, f2, f1, f0, ntry = 0);
  while(ntry < maxtry,
    ntry++;
    f3 = random(q); f2 = random(q); f1 = random(q); f0 = random(q);
    f = Mod(1, q) * (X^5 + f3 * X^3 + f2 * X^2 + f1 * X + f0);
    if(poldegree(gcd(f, f')) > 0, next);
    N = subst(hyperellcharpoly(f), x, 1);
    fa = factor(N); l = fa[#fa~, 1];
    if(cond == "PCC", if(N / l > 4, next), if(l * 32 < q^2 || (N / l) % l == 0, next));
    return([f3, f2, f1, f0, N, l, ntry]));
  0;
}
\\ one instance on a random Jacobian: Gt of order l, Ht = x Gt
rand_instJ(N, l) =
{
  my(D1, Gt, x, Ht);
  while(1, D1 = jrand(); Gt = jmul(N / l, D1); if(!jisid(Gt), break));
  x = 1 + random(l - 1); Ht = jmul(x, Gt);
  [Gt, Ht, x];
}
run_icJ(tag, k, c, i, aid, arm, Gt, Ht, l) =
{
  my(r, ver = 0, p1 = -1);
  setrand(armseed(k, c, i, aid));
  jzero(); r = icrun(Gt, Ht, l, arm, 0);
  if(r[1], ver = jeq(jmul(r[2], Gt), Ht); p1 = !jeq(jmul((r[2] % l) + 1, Gt), Ht));
  concat([tag, k, c, i, aid, Q, l, r[1], ver, p1], r[3..26]);
}
run_plainJ(tag, k, c, i, aid, Gt, Ht, l) =
{
  my(r, ver = 0, p1 = -1, t0);
  RGT = Gt; RHT = Ht; RHOMODE = 2;
  setrand(armseed(k, c, i, aid));
  jzero(); t0 = getwalltime(); r = rhorun(JGR, Gt, Ht, l, 1); t0 = getwalltime() - t0;
  if(r[1], ver = jeq(jmul(r[2], Gt), Ht));
  [tag, k, c, i, aid, Q, l, r[1], ver, -1, r[3], r[4], r[5], r[6], r[7], r[8], r[9], jops, cM, cS, cI, nfb, t0];
}

pca() =
{
  my(curves, cv, insts, l, inst, c, i);
  curves = gen_size(KK, 4, 5);
  setrand(MASTER * 7 + KK); FBS = 1 + random(Q - 1);
  emit(OUTF, ["GEN", KK, Q, GENSTAT[1], GENSTAT[2], GENSTAT[3], GENSTAT[4], FBS]);
  for(c = 1, 4,
    cv = curves[c]; setcurve(cv); insts = GENINST[c]; l = cv[5];
    for(i = 1, 5,
      inst = insts[i];
      emit(OUTF, run_ic("PCA", KK, c, i, 5, "D", inst[1], inst[2], inst[4], inst[5], l, 0));
      emit(OUTF, run_ic("PCA", KK, c, i, 6, "F", inst[1], inst[2], inst[4], inst[5], l, 0))));
  emit(OUTF, ["DONE", KK, CTRL]);
}
pcc_nca() =
{
  my(ncur = 5, nins = 4, q = precprime(2^KK), cur, c, i, inst, l, N, nt, tot = 0);
  Q = q; setrand(MASTER * 1000 + KK);
  setrand(MASTER * 7 + KK); FBS = 1 + random(q - 1);
  setrand(MASTER * 1000 + KK);
  for(c = 1, ncur,
    cur = rand_curveJ(q, if(CTRL == "PCC", "PCC", "NCA"), 400);
    if(cur == 0, emit(OUTF, ["NOCURVE", KK, c]); break);
    jinit(q, cur[1], cur[2], cur[3], cur[4]); l = cur[6]; N = cur[5];
    emit(OUTF, ["CURVEJ", KK, c, q, cur[1], cur[2], cur[3], cur[4], N, l, cur[7]]);
    for(i = 1, nins,
      inst = rand_instJ(N, l);
      emit(OUTF, run_icJ(CTRL, KK, c, i, 5, "D", inst[1], inst[2], l));
      if(CTRL == "PCC", emit(OUTF, run_plainJ(CTRL, KK, c, i, 7, inst[1], inst[2], l)))));
  emit(OUTF, ["DONE", KK, CTRL]);
}
ncb() =
{
  my(curves, cv, insts, l, inst, c, i, P, Hp, THp, G, r, v, ver);
  curves = gen_size(KK, 4, 5);
  setrand(MASTER * 7 + KK); FBS = 1 + random(Q - 1);
  setrand(MASTER * 555 + KK);
  for(c = 1, 4,
    cv = curves[c]; setcurve(cv); insts = GENINST[c]; l = cv[5];
    for(i = 1, 5,
      inst = insts[i]; G = inst[1];
      while(1, P = erandpt(); if(emul(l, P) != [0], tM = 0; THp = tfer(P); if(THp != 0, break)));
      Hp = P;
      v = run_ic("NCB", KK, c, i, 5, "D", G, Hp, inst[4], THp, l, 0); emit(OUTF, v);
      v = run_rho("NCB", KK, c, i, 1, "E", G, Hp, G, Hp, l, 0, 0); emit(OUTF, v)));
  emit(OUTF, ["DONE", KK, CTRL]);
}
ncc() =
{
  my(curves, cv, insts, l, inst, c, i, v);
  curves = gen_size(KK, 4, 5);
  setrand(MASTER * 7 + KK); FBS = 1 + random(Q - 1);
  for(c = 1, 4,
    cv = curves[c]; setcurve(cv); insts = GENINST[c]; l = cv[5];
    for(i = 1, 5,
      inst = insts[i];
      v = run_ic("NCC", KK, c, i, 5, "D", inst[1], inst[2], inst[4], jneg(inst[5]), l, 0); emit(OUTF, v)));
  emit(OUTF, ["DONE", KK, CTRL]);
}
if(CTRL == "PCA", pca());
if(CTRL == "PCC" || CTRL == "NCA", pcc_nca());
if(CTRL == "NCB", ncb());
if(CTRL == "NCC", ncc());
quit
