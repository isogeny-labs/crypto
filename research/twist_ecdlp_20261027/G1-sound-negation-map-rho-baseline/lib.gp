\\ lib.gp -- shared code for E5-kummer-multiplicative-walk (PREREGISTRATION.md)
default(realprecision, 38);

\\ ---------------- output helper (check scripts) ----------------
emit(name, v) = print("chk(\"", name, "\",", v, ");");

\\ ---------------- curve registry ----------------
\\ record: [b, inst, p, A, d, n, h, np, hp, xP, yP, XPp, YPp]
\\   E : y^2 = x^3 + A x^2 + x ; E_d : [0, A d, 0, d^2, 0] with X = d x ; P order n on E, P' order np on E_d
CURVES = Map();
curve(v) = mapput(CURVES, [v[1], v[2]], v);

\\ ---------------- instance generation ----------------
gen_inst(b, binst) =
{
  my(bp = b + 3, p, A, E, N, h, Nt, hp, np, d, Ed, G, P, Gt, Pt, found = 0, nm);
  while(!found,
    p = randomprime([2^(bp-1), 2^bp - 1]);
    A = random(p - 5) + 3;
    if(A == 2 || A == p - 2, next);
    E = ellinit([0, A, 0, 1, 0], p);
    if(E == [], next);
    N = ellcard(E);
    h = 0;
    foreach([4, 8], hh, if(N % hh == 0 && isprime(N \ hh) && #binary(N \ hh) == b, h = hh));
    if(!h, next);
    Nt = 2*p + 2 - N;
    hp = 0;
    for(hh = 1, 16, if(hp == 0 && Nt % hh == 0 && isprime(Nt \ hh), hp = hh));
    if(!hp || #binary(Nt \ hp) < b - 1, next);
    found = 1);
  np = Nt \ hp;
  d = 2; while(kronecker(d, p) != -1, d++);
  Ed = ellinit([0, (A*d) % p, 0, (d^2) % p, 0], p);
  if(ellcard(Ed) != Nt, error("twist order mismatch"));
  P = [0];
  while(P == [0], G = random(E); P = ellmul(E, G, h));
  Pt = [0];
  while(Pt == [0], Gt = random(Ed); Pt = ellmul(Ed, Gt, hp));
  [b, binst, p, A, d, N \ h, h, np, hp, lift(P[1]), lift(P[2]), lift(Pt[1]), lift(Pt[2])];
}

\\ ---------------- multiplier sets ----------------
SA = [2, 3, 5, 7];
SB = [2, 3, 5, 7, 11, 13, 17, 19];
SC = primes(16);

\\ ---------------- context ----------------
\\ globals: P_ (prime), A_, D_, E0, E1, A24, ONE, NN (active order), Ea, Pa, DSC, Eq/Qdsc (cross), CB, CINST
setctx(rec, tw) =
{
  CB = rec[1]; CINST = rec[2]; P_ = rec[3]; A_ = rec[4]; D_ = rec[5];
  E0 = ellinit([0, A_, 0, 1, 0], P_);
  E1 = ellinit([0, (A_*D_) % P_, 0, (D_^2) % P_, 0], P_);
  A24 = Mod(A_ + 2, P_) / 4;
  ONE = Mod(1, P_);
  N0 = rec[6]; N1 = rec[8];
  P0pt = [Mod(rec[10], P_), Mod(rec[11], P_)];
  P1pt = [Mod(rec[12], P_), Mod(rec[13], P_)];
  if(tw == 0,
    NN = N0; Ea = E0; Pa = P0pt; DSC = ONE,
    NN = N1; Ea = E1; Pa = P1pt; DSC = Mod(D_, P_));
  0;
}

\\ ---------------- H structure (exact) ----------------
\\ returns [G, |H|, minus1inH, N, indexpm, g0, fa]
Hinfo(n, mults) =
{
  my(g0 = znprimroot(n), fa = factor(n - 1), ls, G, Hs, m1, N, ipm);
  ls = vector(#mults, i, znlog(Mod(mults[i], n), g0, fa));
  G = gcd(concat([n - 1], ls));
  Hs = (n - 1) \ G;
  m1 = (((n - 1) / 2) % G == 0);
  N = if(m1, Hs \ 2, Hs);
  ipm = (n - 1) / (if(m1, Hs, 2*Hs));
  [G, Hs, m1, N, ipm, g0, fa];
}
inpmH(a, hi, n) =
{
  my(G = hi[1], l);
  if(G == 1, return(1));
  l = znlog(Mod(a, n), hi[6], hi[7]);
  (l % G == 0) || ((l + (n - 1) / 2) % G == 0);
}

dpbits(M) = max(3, logint(max(1, floor(sqrt(M) / 16)), 2));

\\ set walk parameters. kind: "K" (multiplicative) or "A0"/"A1"
setwalk(mults, kind) =
{
  my(v);
  MULT = mults; RR = #mults;
  MBITS = vector(RR, j, v = binary(MULT[j]); vector(#v - 1, i, v[i + 1]));
  HI = Hinfo(NN, MULT);
  if(kind == "K", MARM = HI[4], kind == "A0", MARM = NN, MARM = NN \ 2);
  DPB = dpbits(MARM); DPM = 2^DPB; CAP = 20 * DPM;
  BUDGET = 0;  \\ set by caller
  0;
}

randH() = { my(r = Mod(1, NN)); for(j = 1, RR, r *= Mod(MULT[j], NN)^random(NN - 1)); lift(r); }

\\ ---------------- x-only Montgomery ladder ----------------
\\ counters: cM cS cI (globals), reset by caller.  xDBL = 3M+2S, xADD = 4M+2S (Zdiff multiplied explicitly,
\\ PREREGISTRATION 3: "xADD = 4 M + 2 S").  Start (R0,R1) = (P, 2P) [1 xDBL]; then (xADD + xDBL) per remaining bit.
lad(x, bits) =
{
  my(a24 = A24, one = ONE, X0 = x, Z0 = one, X1, Z1, s, t, ss, tt, c, v1, v2, Xs, Zs, nb = #bits);
  s = X0 + Z0; t = X0 - Z0; ss = s^2; tt = t^2; c = ss - tt; X1 = ss * tt; Z1 = c * (tt + a24 * c);
  for(i = 1, nb,
    v1 = (X0 - Z0) * (X1 + Z1); v2 = (X0 + Z0) * (X1 - Z1);
    Xs = one * (v1 + v2)^2; Zs = x * (v1 - v2)^2;
    if(bits[i],
      X0 = Xs; Z0 = Zs;
      s = X1 + Z1; t = X1 - Z1; ss = s^2; tt = t^2; c = ss - tt; X1 = ss * tt; Z1 = c * (tt + a24 * c),
      X1 = Xs; Z1 = Zs;
      s = X0 + Z0; t = X0 - Z0; ss = s^2; tt = t^2; c = ss - tt; X0 = ss * tt; Z0 = c * (tt + a24 * c)));
  cM += 3 + 7 * nb; cS += 2 + 4 * nb;
  [X0, Z0];
}
kbits(k) = { my(v = binary(k)); vector(#v - 1, i, v[i + 1]); }

\\ trail of the multiplicative walk.  returns [status, xlift, e, steps]; status 1 = DP, 0 = cap, -1 = infinity
ktrail(x0, e0, cap) =
{
  my(x = x0, e = e0, st = 0, j, r, lx, dd = DPB, rm = RR, dm = DPM, nn = NN);
  lx = lift(x);
  while(st < cap,
    j = ((lx >> dd) % rm) + 1;
    r = lad(x, MBITS[j]);
    HIST[j]++; st++;
    if(r[2] == 0, return([-1, 0, 0, st]));
    x = r[1] / r[2]; cM += 1; cI += 1;
    e = (e * MULT[j]) % nn;
    lx = lift(x);
    if(lx % dm == 0, return([1, lx, e, st])));
  [0, lx, e, st];
}

\\ ---------------- trial: K-pure (and cross) ----------------
\\ SRCS = [[curve, point, dscale], [curve, point, dscale]]  (index 1 = P side, 2 = Q side)
trial_kpure(a, budget, cross) =
{
  my(Q, mapP = Map(), mapQ = Map(), tot = 0, TP = 0, TQ = 0, trails = 0, dps = 0, wasted = 0, cand = 0, ok = 0, sol = 0,
     side, e0, pt, x0, r, xk, ev, a1, t0 = getwalltime(), Qx, src, cp);
  cM = 0; cS = 0; cI = 0; HIST = vector(RR);
  if(!cross, Q = ellmul(Ea, Pa, a); Qx = Q[1]; SRCS = [[Ea, Pa, DSC], [Ea, Q, DSC]]);
  while(!ok && tot < budget,
    side = trails % 2; src = SRCS[side + 1]; trails++;
    e0 = randH();
    pt = ellmul(src[1], src[2], e0);
    if(pt == [0], next);
    x0 = pt[1] / src[3];
    cp = min(CAP, budget - tot);
    r = ktrail(x0, e0, cp);
    tot += r[4]; if(side, TQ += r[4], TP += r[4]);
    if(r[1] == 1,
      dps++; xk = r[2];
      if(side == 0,
        if(mapisdefined(mapQ, xk, &ev),
          cand++;
          if(!cross,
            a1 = (r[3] * lift(Mod(ev, NN)^-1)) % NN;
            if(a1 != 0 && ellmul(Ea, Pa, a1)[1] == Qx, ok = 1; sol = a1, wasted++)),
          if(mapisdefined(mapP, xk), wasted++, mapput(mapP, xk, r[3]))),
        if(mapisdefined(mapP, xk, &ev),
          cand++;
          if(!cross,
            a1 = (ev * lift(Mod(r[3], NN)^-1)) % NN;
            if(a1 != 0 && ellmul(Ea, Pa, a1)[1] == Qx, ok = 1; sol = a1, wasted++)),
          if(mapisdefined(mapQ, xk), wasted++, mapput(mapQ, xk, r[3])))));
  );
  if(!ok, tot = budget);
  [ok, tot, TP, TQ, wasted, trails, dps, cM, cS, cI, 0, cand, HIST, getwalltime() - t0, sol];
}

\\ ---------------- trial: K-mix ----------------
trial_kmix(a, budget) =
{
  my(Q, map = Map(), tot = 0, trails = 0, dps = 0, wasted = 0, cand = 0, ok = 0, sol = 0, r, s, pt, x0, res, xk, ev, u, v, a1, den, num,
     t0 = getwalltime(), Qx, cp, eps);
  cM = 0; cS = 0; cI = 0; HIST = vector(RR);
  Q = ellmul(Ea, Pa, a); Qx = Q[1];
  while(!ok && tot < budget,
    trails++;
    r = random(NN); s = random(NN);
    if((r + s * a) % NN == 0, next);
    pt = elladd(Ea, if(r, ellmul(Ea, Pa, r), [0]), if(s, ellmul(Ea, Q, s), [0]));
    x0 = pt[1] / DSC;
    cp = min(CAP, budget - tot);
    res = ktrail(x0, 1, cp);
    tot += res[4];
    if(res[1] == 1,
      dps++; xk = res[2];
      u = (r * res[3]) % NN; v = (s * res[3]) % NN;
      if(mapisdefined(map, xk, &ev),
        cand++;
        for(si = 0, 1,
          eps = if(si, -1, 1);
          den = (eps * ev[2] - v) % NN; num = (u - eps * ev[1]) % NN;
          if(!ok && den != 0,
            a1 = (num * lift(Mod(den, NN)^-1)) % NN;
            if(a1 != 0 && ellmul(Ea, Pa, a1)[1] == Qx, ok = 1; sol = a1)));
        if(!ok, wasted++),
        mapput(map, xk, [u, v])));
  );
  if(!ok, tot = budget);
  [ok, tot, tot, 0, wasted, trails, dps, cM, cS, cI, 0, cand, HIST, getwalltime() - t0, sol];
}

\\ ---------------- additive baselines A0 / A1 ----------------
\\ globals: WX WY (Mod), WU WV (ints), NEG (0/1), PM1H = (p-1)/2, AD counters nadd ndd nde
\\ returns [status, key, al, be, steps]
atrail(x0, y0, al0, be0, cap) =
{
  my(x = x0, y = y0, al = al0, be = be0, st = 0, j, xw, yw, l, x3, y3, lx, xm1 = 0, nn = NN, dd = DPB, dm = DPM, aa = A_, key, ph = PM1H);
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
      if(x3 == xm1,
        \\ fruitless 2-cycle: replace by canonical double
        if(y3 == 0, return([-1, 0, 0, 0, st]));
        l = (3 * x3^2 + 2 * aa * x3 + 1) / (2 * y3);
        x = l^2 - aa - 2 * x3; y = l * (x3 - x) - y3;
        nde++; cM += 3; cS += 2; cI += 1;
        al = (2 * al) % nn; be = (2 * be) % nn;
        if(lift(y) > ph, y = -y; al = (nn - al) % nn; be = (nn - be) % nn);
        x3 = x; y3 = y));
    xm1 = x; x = x3; y = y3;
    lx = lift(x);
    if(lx % dm == 0,
      key = if(NEG, lx, [lx, lift(y)]);
      return([1, key, al, be, st])));
  [0, 0, al, be, st];
}

trial_add(a, budget, neg) =
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
    res = atrail(pt[1], pt[2], al, be, cp);
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

\\ a-list: mode 0 all, 1 in +-H, 2 outside +-H ; uniform in [2, n-2]
alist(T, n, mode, hi) =
{
  my(v = vector(T), a);
  for(t = 1, T,
    while(1,
      a = random(n - 3) + 2;
      if(mode == 0, break);
      if(mode == 1 && inpmH(a, hi, n), break);
      if(mode == 2 && !inpmH(a, hi, n), break));
    v[t] = a);
  v;
}

\\ plain ladder check helper
xmul_lad(x, k) = { my(r = lad(x, kbits(k))); r; }

\\ exponent closure of H = <mults> in (Z/n)^*, as a 0/1 Vecsmall indexed by 1..n-1 (pointer queue, O(|H| r))
Hclosure(n, mults) =
{
  my(inH = vectorsmall(n - 1), q = vectorsmall(n), head = 1, tail = 1, m, m2);
  inH[1] = 1; q[1] = 1;
  while(head <= tail,
    m = q[head]; head++;
    foreach(mults, k,
      m2 = (m * k) % n;
      if(m2 && !inH[m2], inH[m2] = 1; tail++; q[tail] = m2)));
  inH;
}
