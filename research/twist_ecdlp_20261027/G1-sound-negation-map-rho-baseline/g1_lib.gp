\\ g1_lib.gp -- negation-map rho arm N1 for G1-sound-negation-map-rho-baseline (PREREGISTRATION.md).
\\ Requires lib.gp (E5 copy, byte-identical) to be read first.  Reuses setctx/setwalk/dpbits/alist/Hinfo from it.
\\
\\ Walk: additive r-adding walk, R2 table entries W_j = u_j P + v_j Q, selector j = ((lift(x) >> DPB) mod R2) + 1 (x only, so
\\ negation-invariant), canonical point = the one with lift(y) <= (p-1)/2, exponents (alpha, beta) negated with the point.
\\ Cycle handling, MODE:
\\   0 = N1 (working): 2-cycle test (new x == x one step back) + Brent saved-point test; either one triggers ESCAPE.
\\   1 = N1-sab (negative control): NO cycle test at all (trails leave a cycle only by cap).
\\   2 = N1-c2 (partial sabotage): 2-cycle test only (E5 A1 semantics but r = R2).
\\ ESCAPE = replace the current canonical point by its canonical double (exponents doubled, sign-fixed); counted 3M+2S+1I;
\\   Brent state is reset to the new point.
\\ Trails end at a DP (lift(x) mod 2^DPB == 0), at the cap (CAP = 20*2^DPB), or when a step hits x == x_W, y != y_W (dead trail).
\\ Counters (globals): cM cS cI, nadd ndd n2c nbr (escapes by 2-cycle test / Brent test).  Native counters are derived in analysis:
\\   SEL = DPT = CAN = steps, CYC = 2*steps (MODE 0), LOOK = dps, START = trails.

atrail2(x0, y0, al0, be0, cap) =
{
  my(x = x0, y = y0, al = al0, be = be0, st = 0, j, xw, yw, l, x3, y3, lx, xm1 = 0, nn = NN, dd = DPB, dm = DPM, aa = A_, ph = PM1H,
     r2 = R2, xs = x0, pw = 1, lam = 0, esc, mode = MODE);
  lx = lift(x);
  while(st < cap,
    j = ((lx >> dd) % r2) + 1;
    xw = WX[j]; yw = WY[j];
    if(x == xw,
      if(y == yw,
        l = (3 * x^2 + 2 * aa * x + 1) / (2 * y); x3 = l^2 - aa - 2 * x; y3 = l * (x - x3) - y;
        ndd++; cM += 3; cS += 2; cI += 1,
        st++; return([-1, 0, 0, 0, st])),
      l = (yw - y) / (xw - x); x3 = l^2 - aa - x - xw; y3 = l * (x - x3) - y;
      nadd++; cM += 2; cS += 1; cI += 1);
    al = (al + WU[j]) % nn; be = (be + WV[j]) % nn;
    st++;
    if(lift(y3) > ph, y3 = -y3; al = (nn - al) % nn; be = (nn - be) % nn);
    if(mode != 1,
      esc = 0;
      if(x3 == xm1, esc = 1; n2c++);
      if(mode == 0 && !esc,
        if(x3 == xs, esc = 2; nbr++,
          lam++; if(lam == pw, xs = x3; pw *= 2; lam = 0)));
      if(esc,
        if(y3 == 0, return([-1, 0, 0, 0, st]));
        l = (3 * x3^2 + 2 * aa * x3 + 1) / (2 * y3);
        x = l^2 - aa - 2 * x3; y = l * (x3 - x) - y3;
        cM += 3; cS += 2; cI += 1;
        al = (2 * al) % nn; be = (2 * be) % nn;
        if(lift(y) > ph, y = -y; al = (nn - al) % nn; be = (nn - be) % nn);
        x3 = x; y3 = y;
        xs = x3; pw = 1; lam = 0;
        xm1 = 0,
        xm1 = x));
    if(mode == 1, xm1 = x);
    x = x3; y = y3;
    lx = lift(x);
    if(lx % dm == 0, return([1, lx, al, be, st])));
  [0, 0, al, be, st];
}

\\ trial: same structure/start/verification as E5 trial_add(neg = 1), with atrail2.  Record layout identical to E5 plus a counter vector.
trial_n1(a, budget) =
{
  my(Q, map = Map(), tot = 0, trails = 0, dps = 0, wasted = 0, cand = 0, ok = 0, sol = 0, al, be, pt, res, ev, den, a1, t0 = getwalltime(), Qx, cp, Wp);
  cM = 0; cS = 0; cI = 0; nadd = 0; ndd = 0; n2c = 0; nbr = 0; PM1H = (P_ - 1) \ 2;
  Q = ellmul(Ea, Pa, a); Qx = Q[1];
  WX = vector(R2); WY = vector(R2);
  for(j = 1, R2, Wp = elladd(Ea, ellmul(Ea, Pa, WU[j]), ellmul(Ea, Q, WV[j])); WX[j] = Wp[1]; WY[j] = Wp[2]);
  while(!ok && tot < budget,
    trails++;
    al = random(NN); be = random(NN);
    if((al + be * a) % NN == 0, next);
    pt = elladd(Ea, if(al, ellmul(Ea, Pa, al), [0]), if(be, ellmul(Ea, Q, be), [0]));
    if(pt == [0], next);
    if(lift(pt[2]) > PM1H, pt[2] = -pt[2]; al = (NN - al) % NN; be = (NN - be) % NN);
    cp = min(CAP, budget - tot);
    res = atrail2(pt[1], pt[2], al, be, cp);
    tot += res[5];
    if(res[1] == 1,
      dps++;
      if(mapisdefined(map, res[2], &ev),
        cand++;
        den = (res[4] - ev[2]) % NN;
        if(den == 0, wasted++,
          a1 = ((ev[1] - res[3]) * lift(Mod(den, NN)^-1)) % NN;
          if(a1 != 0 && ellmul(Ea, Pa, a1)[1] == Qx, ok = 1; sol = a1, wasted++)),
        mapput(map, res[2], [res[3], res[4]])));
  );
  if(!ok, tot = budget);
  [ok, tot, tot, 0, wasted, trails, dps, cM, cS, cI, 0, cand, [nadd, ndd, n2c, nbr], getwalltime() - t0, sol];
}
