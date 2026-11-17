\\ lib_a1c.gp -- SUPPLEMENTARY arm a1c (added after the main run; see DEVIATIONS.md): A1 with the cycle test extended from "2 steps back"
\\ to "any of the last 8 visited points".  Everything else identical to A1 (lib.gp).
vecsearch_any(v, x) = { for(i = 1, #v, if(v[i] == x, return(1))); 0; }
atrail_c(x0, y0, al0, be0, cap) =
{
  my(x = x0, y = y0, al = al0, be = be0, st = 0, j, xw, yw, l, x3, y3, lx, xm1 = 0, hx = vector(8), hp = 0, nn = NN, dd = DPB, dm = DPM, aa = A_, key, ph = PM1H);
  lx = lift(x);
  while(st < cap,
    j = ((lx >> dd) % 16) + 1;
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
    if(NEG,
      if(lift(y3) > ph, y3 = -y3; al = (nn - al) % nn; be = (nn - be) % nn);
      if(vecsearch_any(hx, x3),
        \\ revisit of one of the last 8 points (fruitless cycle of length 2,4,6,8): replace by canonical double
        if(y3 == 0, return([-1, 0, 0, 0, st]));
        l = (3 * x3^2 + 2 * aa * x3 + 1) / (2 * y3);
        x = l^2 - aa - 2 * x3; y = l * (x3 - x) - y3;
        nde++; cM += 3; cS += 2; cI += 1;
        al = (2 * al) % nn; be = (2 * be) % nn;
        if(lift(y) > ph, y = -y; al = (nn - al) % nn; be = (nn - be) % nn);
        x3 = x; y3 = y));
    hp = hp % 8 + 1; hx[hp] = x; xm1 = x; x = x3; y = y3;
    lx = lift(x);
    if(lx % dm == 0,
      key = if(NEG, lx, [lx, lift(y)]);
      return([1, key, al, be, st])));
  [0, 0, al, be, st];
}

trial_add_c(a, budget, neg) =
{
  my(Q, map = Map(), tot = 0, trails = 0, dps = 0, wasted = 0, cand = 0, ok = 0, sol = 0, al, be, pt, res, ev, den, a1, t0 = getwalltime(), Qx, cp, Wp);
  cM = 0; cS = 0; cI = 0; nadd = 0; ndd = 0; nde = 0; NEG = neg; PM1H = (P_ - 1) \ 2;
  Q = ellmul(Ea, Pa, a); Qx = Q[1];
  WX = vector(16); WY = vector(16);
  for(j = 1, 16, Wp = elladd(Ea, ellmul(Ea, Pa, WU[j]), ellmul(Ea, Q, WV[j])); WX[j] = Wp[1]; WY[j] = Wp[2]);
  while(!ok && tot < budget,
    trails++;
    al = random(NN); be = random(NN);
    if((al + be * a) % NN == 0, next);
    pt = elladd(Ea, if(al, ellmul(Ea, Pa, al), [0]), if(be, ellmul(Ea, Q, be), [0]));
    if(pt == [0], next);
    if(neg && lift(pt[2]) > PM1H, pt[2] = -pt[2]; al = (NN - al) % NN; be = (NN - be) % NN);
    cp = min(CAP, budget - tot);
    res = atrail_c(pt[1], pt[2], al, be, cp);
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
  [ok, tot, tot, 0, wasted, trails, dps, cM, cS, cI, 0, cand, [nadd, ndd, nde], getwalltime() - t0, sol];
}

