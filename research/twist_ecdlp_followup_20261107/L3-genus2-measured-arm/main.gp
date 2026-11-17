\\ main.gp -- one cell of the main experiment: size K, half HALF (1: curves 1-3, 2: curves 4-6).  Env: SRC, K, HALF, OUT (output file), MSEED.
\\ Writes one line per (instance, arm) plus CURVE / GEN / INST / CHK lines.
default(parisizemax, 1000000000);
read(Str(getenv("SRC"), "/lib.gp"));
KK = eval(getenv("K")); HALF = eval(getenv("HALF")); OUTF = getenv("OUT"); MASTER = eval(getenv("MSEED"));
NCURVES = 6; NINST = 4; NREFPAIRS = 334;

\\ ---- per-curve checks (use their own seeded stream; nothing here feeds the arms) ----
checks_curve(k, c, cv, insts) =
{
  my(q = Q, okd, nbadN = 0, nN = 8, nref = 0, nbadref = 0, nassoc = 0, nbadassoc = 0, nhom = 0, nbadhom = 0, nT = 0, nbadT = 0, D1, D2, D3, r1, r2, P1, P2, P3, T1, T2, T3, G, TG, TH, typ, l = cv[5], pt, pt2, nfb0, tmpfb = 0);
  setrand(MASTER * 100000 + k * 100 + c);
  jzero();
  okd = (countCp() == q + 1);
  for(t = 1, nN, D1 = jrand(); if(!jisid(jmul(CNJ, D1)), nbadN++));
  for(t = 1, NREFPAIRS,
    typ = t % 8;
    D1 = jrand();
    if(typ == 0, D2 = jrand(),
    if(typ == 1, D2 = D1,
    if(typ == 2, D2 = jneg(D1),
    if(typ == 3, pt = jrandpt(); pt2 = jrandpt(); D1 = jadd(jpoint(pt[1], pt[2]), jpoint(pt2[1], pt2[2])); pt2 = jrandpt(); D2 = jadd(jpoint(pt[1], pt[2]), jpoint(pt2[1], pt2[2])),
    if(typ == 4, pt = jrandpt(); D2 = jpoint(pt[1], pt[2]),
    if(typ == 5, D2 = IDJ,
    if(typ == 6, pt = jrandpt(); pt2 = jrandpt(); D1 = jpoint(pt[1], pt[2]); D2 = jpoint(pt2[1], pt2[2]),
    D2 = jrand())))))));
    r1 = jadd(D1, D2); r2 = jref(D1, D2); nref++;
    if(!jeq(r1, r2), nbadref++);
    if(#r1 == 4 && !jvalid(r1), nbadref++);
    r1 = jadd(r1, jneg(D2)); if(!jeq(r1, jref(D1, IDJ)) && !jeq(r1, D1), nbadref++));
  for(t = 1, 20, D1 = jrand(); D2 = jrand(); D3 = jrand(); nassoc++; if(!jeq(jadd(jadd(D1, D2), D3), jadd(D1, jadd(D2, D3))), nbadassoc++));
  for(t = 1, 30,
    P1 = erandpt(); P2 = erandpt(); P3 = eadd(P1, P2);
    T1 = tfer(P1); T2 = tfer(P2); T3 = tfer(P3);
    if(T1 == 0 || T2 == 0 || T3 == 0, next);
    nhom++; if(!jeq(jadd(T1, T2), T3), nbadhom++));
  for(i = 1, #insts,
    TG = insts[i][4]; TH = insts[i][5]; nT++;
    if(!(jvalid(TG) && jvalid(TH) && jisid(jmul(l, TG)) && !jisid(TG) && jisid(jmul(l, TH))), nbadT++));
  [okd, nbadN, nN, nref, nbadref, nassoc, nbadassoc, nhom, nbadhom, nT, nbadT];
}

main() =
{
  my(curves, cv, insts, c, i, inst, l, h, G, H, x, TG, TH, v, lo, hi, arm, ck, t0);
  curves = gen_size(KK, NCURVES, NINST);
  setrand(MASTER * 7 + KK); FBS = 1 + random(Q - 1);
  if(HALF == 1,
    emit(OUTF, ["GEN", KK, Q, GENSTAT[1], GENSTAT[2], GENSTAT[3], GENSTAT[4], FBS]));
  lo = if(HALF == 1, 1, 4); hi = if(HALF == 1, 3, 6);
  for(c = lo, hi,
    cv = curves[c]; setcurve(cv); insts = GENINST[c]; l = cv[5]; h = cv[6];
    emit(OUTF, ["CURVE", KK, c, Q, cv[1], cv[2], cv[3], cv[4], l, h, cv[7], cv[8], cv[10], cv[11], cv[12], cv[13], cv[14], cv[15], cv[16], cv[17], cv[18], cv[19], cv[20], cv[21]]);
    ck = checks_curve(KK, c, cv, insts);
    emit(OUTF, concat(["CHK", KK, c], ck));
    for(i = 1, NINST,
      inst = insts[i]; G = inst[1]; H = inst[2]; x = inst[3]; TG = inst[4]; TH = inst[5];
      emit(OUTF, ["INST", KK, c, i, Q, l, h, x, inst[6], inst[7], inst[8], inst[9]]);
      emit(OUTF, run_rho("RHO", KK, c, i, 1, "E", G, H, G, H, l, 0, 1));
      emit(OUTF, run_rho("RHO", KK, c, i, 2, "J", G, H, TG, TH, l, 0, 1));
      emit(OUTF, run_ic("IC", KK, c, i, 3, "G", G, H, TG, TH, l, 0));
      emit(OUTF, run_ic("IC", KK, c, i, 4, "S", G, H, TG, TH, l, 0));
      emit(OUTF, run_ic("IC", KK, c, i, 5, "D", G, H, TG, TH, l, 0))));
  emit(OUTF, ["DONE", KK, HALF]);
}
main();
quit
