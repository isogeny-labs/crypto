\\ glue.gp -- E-side arithmetic with counters, curve-pair construction (E, C, C'), admissibility filter, transfer map T, instances.
\\ Requires jac.gp.  Globals: Q (prime), EA, EB (E: y^2 = x^3 + EA x + EB), counters eops eM eS eI.

ezero() = { eops = 0; eM = 0; eS = 0; eI = 0; }
eadd(P, R) =
{
  my(q = Q, l, x1, y1, x2, y2, x3, y3);
  eops++;
  if(#P == 1, return(R)); if(#R == 1, return(P));
  x1 = P[1]; y1 = P[2]; x2 = R[1]; y2 = R[2];
  if(x1 == x2,
    if((y1 + y2) % q == 0, return([0]));
    l = (3 * x1 * x1 + EA) % q * lift(Mod(2 * y1, q)^-1) % q; x3 = (l * l - 2 * x1) % q; y3 = (l * (x1 - x3) - y1) % q;
    eM += 2; eS += 2; eI += 1; return([x3, y3]));
  l = (y2 - y1) * lift(Mod(x2 - x1, q)^-1) % q; x3 = (l * l - x1 - x2) % q; y3 = (l * (x1 - x3) - y1) % q;
  eM += 2; eS += 1; eI += 1;
  [x3, y3];
}
eneg(P) = if(#P == 1, P, [P[1], (Q - P[2]) % Q]);
emul(n, P) =
{
  my(bits, R, i);
  if(n == 0, return([0]));
  bits = binary(n); R = P;
  for(i = 2, #bits, R = eadd(R, R); if(bits[i], R = eadd(R, P)));
  R;
}
\\ independent PARI verification: does x' G = H on E (uses ellmul, never counted)
everify(G, H, xp) =
{
  my(E = ellinit([EA, EB], Q), pg = [Mod(G[1], Q), Mod(G[2], Q)], r);
  if(xp % Q == 0 && 0, return(0));
  r = ellmul(E, pg, xp);
  if(#H == 1, return(r == [0]));
  r != [0] && r[1] == Mod(H[1], Q) && r[2] == Mod(H[2], Q);
}
erandpt() =
{
  my(x0, fx, y0, q = Q);
  while(1, x0 = random(q); fx = (x0^3 + EA * x0 + EB) % q;
    if(kronecker(fx, q) == 1, y0 = lift(sqrt(Mod(fx, q))); if(random(2), y0 = (q - y0) % q); return([x0, y0])));
}
nonresq(q) = { my(d = 2); while(kronecker(d, q) != -1, d++); d; }

\\ ---------------------------------------------------------------------------------------------
\\ build the gluing data for E (A,B) and a chosen permutation index; returns 0 on failure with reason in the global GLREASON
\\ allperms3 as in E6; pattern determined from roots.
allperms3 = [[2,1,3],[1,3,2],[2,3,1],[3,1,2],[3,2,1]];   \\ non-identity permutations of 3 roots
\\ interpolating polynomial coefficients (g0,g1,g2) for the permutation; returns [g0,g1,g2] (integers) or 0
interpg(roots, pat, A_, B_, pidx) =
{
  my(q = Q, r1, s, p, qd, g2, g1, g0, rr, perm, ys, g, f, den);
  if(pat == 3,
    perm = allperms3[pidx]; rr = roots;
    ys = vector(3, i, rr[perm[i]]);
    g = polinterpolate(vector(3, i, Mod(rr[i], q)), vector(3, i, Mod(ys[i], q)), X);   \\ polynomial in X
    g = g + 0 * X;
    return([lift(polcoeff(g, 0)), lift(polcoeff(g, 1)), lift(polcoeff(g, 2))]));
  \\ pattern [1,2]: roots = [r1]; the unique non-identity equivariant permutation swaps the two conjugate roots
  if(pidx > 1, return(0));
  r1 = roots[1];
  f = Mod(1, q) * (X^3 + A_ * X + B_);
  qd = f / (X - r1);                      \\ monic quadratic X^2 + s X + p
  s = lift(polcoeff(qd, 1)); p = lift(polcoeff(qd, 0));
  den = (p + s * r1 + r1^2) % q;
  if(den == 0, return(0));
  g2 = (2 * r1 + s) * lift(Mod(den, q)^-1) % q;
  g1 = (g2 * s - 1) % q; g0 = (g2 * p - s) % q;
  [g0, g1, g2];
}

\\ counter vector for rejections (named)
REJNAMES = ["a_pattern3", "b_cofactor", "b_ltwist", "c_g2zero", "c_singular", "c_noWpt", "d_countq1", "other"];
rejinit() = { REJ = vector(#REJNAMES); }
rej(i) = { REJ[i]++; 0; }

\\ Try to build an admissible curve pair from (A,B). Returns a vector or 0.
\\ [A,B,nE,nEt,l,h,pat,pidx,g,kappa,mu,a,r,rho,c1,sigma,f3,f2,f1,f0,rootcalls]
build_pair(A_, B_) =
{
  my(q = Q, roots, pat, E, nE, nEt, fa, l, h, pidx, g, kap, mu, aa, r, rho, ok, Fx, Fs, cc, M5, sig, M2, f3, f2, f1, f0, npi, rc = 0, res, rr, k, c1, fmu, found);
  roots = polrootsmod(Mod(1, q) * (X^3 + A_ * X + B_)); rc++;
  roots = vector(#roots, i, lift(roots[i]));
  if(#roots == 0, return(rej(1)));
  pat = if(#roots == 3, 3, 1);
  E = ellinit([A_, B_], q); nE = ellcard(E); nEt = 2 * q + 2 - nE;
  fa = factor(nE); l = fa[#fa~, 1]; h = nE / l;
  if(h % 2 != 0 || h > 16 || 16 * l < q || h < 2, return(rej(2)));
  if(nEt % l == 0, return(rej(3)));
  npi = if(pat == 3, 5, 1);
  for(pidx = 1, npi,
    g = interpg(roots, pat, A_, B_, pidx);
    if(g == 0 || g[3] == 0, rej(4); next);
    kap = lift(Mod(g[3], q)^-1); mu = (-g[2] * kap) % q;
    fmu = ((-mu)^3 + A_ * (-mu) + B_) % q;
    if(fmu == 0, rej(5); next);
    aa = 1; while(kronecker((-kap * aa) % q, q) != -1, aa++);
    found = 0;
    for(i = 1, #roots,
      r = roots[i]; k = (r + mu) % q;
      if(k != 0 && kronecker(aa * k % q, q) == 1,
        rho = lift(sqrt(Mod(k * lift(Mod(aa, q)^-1), q))); rc++; found = 1; break));
    if(!found, rej(6); next);
    \\ model: F(x) = f(a x^2 - mu); shift x = rho + t; X = 1/t; X' = c1 X; shift to kill X'^4
    Fx = subst(Mod(1, q) * (X^3 + A_ * X + B_), X, Mod(aa, q) * X^2 - mu);
    Fs = subst(Fx, X, X + rho);
    if(polcoeff(Fs, 0) != 0, rej(8); next);
    cc = vector(7, i, lift(polcoeff(Fs, i - 1)));     \\ cc[i] = c_{i-1}
    c1 = cc[2];
    if(c1 == 0, rej(8); next);
    M5 = Mod(1, q) * (X^5 + cc[3] * X^4 + cc[4] * c1 * X^3 + cc[5] * c1^2 * X^2 + cc[6] * c1^3 * X + cc[7] * c1^4);
    sig = lift(Mod(cc[3], q) / 5);
    M2 = subst(M5, X, X - sig);
    if(polcoeff(M2, 4) != 0 || poldegree(M2) != 5, rej(8); next);
    f3 = lift(polcoeff(M2, 3)); f2 = lift(polcoeff(M2, 2)); f1 = lift(polcoeff(M2, 1)); f0 = lift(polcoeff(M2, 0));
    return([A_, B_, nE, nEt, l, h, pat, pidx, g, kap, mu, aa, r, rho, c1, sig, f3, f2, f1, f0, rc]));
  rej(8) ;
}
\\ #C'(F_q) by Legendre sum (consistency check d); needs jinit
countCp() =
{
  my(q = Q, s = 1, r, xx);
  for(xx = 0, q - 1,
    r = xx; r = (r * xx + F3) % q; r = (r * xx + F2) % q; r = (r * xx + F1) % q; r = (r * xx + F0) % q;
    s += 1 + kronecker(r, q));
  s;
}

\\ install a curve pair: sets E globals, Jacobian model, and transfer constants
setcurve(cv) =
{
  my(q = Q);
  EA = cv[1]; EB = cv[2];
  CV = cv;
  CL = cv[5]; CNE = cv[3]; CNET = cv[4]; CNJ = cv[3] * cv[4];
  jinit(q, cv[17], cv[18], cv[19], cv[20]);
  TMU = cv[11]; TIA = lift(Mod(cv[12], q)^-1); TRHO2 = cv[14]^2 % q; TC5 = cv[15]; TC5SQ = TC5^2 % q;
  TTWORC5 = 2 * cv[14] * TC5 % q; TIC5 = lift(Mod(TC5, q)^-1); TSIG = cv[16]; TSIG2 = TSIG^2 % q;
  tM = 0; tS = 0; tI = 0;
}
\\ transfer map T(P) = [phi^*P - 2W] for P = [x0,y0] in E(F_q) (not O); returns the 4-vector, or 0 if s = 0 or d = 0 (counted by caller)
\\ field ops counted into tM tS tI
tfer(P) =
{
  my(q = Q, s, dd, id, p0, p1, ua, ub, y0c, p1s, t, c, d, p1p0);
  if(#P == 1, return(IDJ));
  s = (P[1] + TMU) % q;
  if(s == 0, return(0));
  dd = (TRHO2 - s * TIA) % q;
  if(dd == 0, return(0));
  id = lift(Mod(dd, q)^-1);
  p0 = TC5SQ * id % q; p1 = TTWORC5 * id % q;
  ua = (p1 - 2 * TSIG) % q;
  ub = (TSIG2 - TSIG * p1 + p0) % q;
  y0c = P[2] * TIC5 % q;
  p1s = p1 * p1 % q; t = (p1s - p0) % q;
  p1p0 = p1 * p0 % q;
  c = y0c * t % q;
  d = y0c * ((p1p0 - t * TSIG) % q) % q;
  \\ counts: M: s*TIA, p0, p1, TSIG*p1, y0c, c, p1p0, t*TSIG, y0c*(..) = 9 ; S: 1 ; I: 1
  tM += 9; tS += 1; tI += 1;
  [ua, ub, c, d];
}
\\ is the 4-vector a valid reduced divisor of C' (u | f - v^2)?
jvalid(D) =
{
  my(p = mkpol(D), r);
  r = (FPOL - p[2]^2) % p[1];
  r == 0;
}
