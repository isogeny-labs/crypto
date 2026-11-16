\\ jac.gp -- explicit-formula Mumford arithmetic on Jac(C'), C': V^2 = X^5 + F3 X^3 + F2 X^2 + F1 X + F0 over F_Q (Q prime > 5),
\\ with field-operation counters, plus a reference Cantor in PARI polynomial arithmetic (checks and rare special cases).
\\ Element of the Jacobian: 4-vector [a,b,c,d] (integers in [0,Q)) = (u = X^2+aX+b, v = cX+d), the reduced divisor of degree 2 (class D-2W);
\\ any other element (deg u < 2, incl. identity) is a 2-vector [u,v] of t_POL with Mod coefficients (produced only by the Cantor fallback).
\\ Counters (globals): cM cS cI = field mults/squarings/inversions of the explicit formulas;  jops = Jop count (every add or double, fast or fallback);
\\ nfb = number of fallback (Cantor) Jops; nadd2 = explicit adds, ndbl2 = explicit doubles, nidn = identity shortcuts (not counted as Jop).
\\ Negation (v -> -v) is native (CANON) and not a Jop.

jinit(q,f3,f2,f1,f0) =
{
  Q = q; F3 = f3 % q; F2 = f2 % q; F1 = f1 % q; F0 = f0 % q; QH = (q - 1) \ 2;
  FPOL = Mod(1, q) * (X^5 + F3 * X^3 + F2 * X^2 + F1 * X + F0);
  IDJ = [Mod(1, q) + 0 * X, Mod(0, q) + 0 * X];
}
jzero() = { cM = 0; cS = 0; cI = 0; jops = 0; nfb = 0; nadd2 = 0; ndbl2 = 0; nidn = 0; }

\\ ---------------- reference / fallback Cantor (PARI polynomial arithmetic; counters not touched except nfb by the caller) -------------
mkpol(D) = if(#D == 4, [Mod(1, Q) * (X^2 + D[1] * X + D[2]), Mod(1, Q) * (D[3] * X + D[4])], D);
bezq(a, b) =
{ \\ [e1,e2,d] with e1*a + e2*b = d monic gcd; a,b t_POL or scalars over F_Q (not both zero)
  my(r, lc);
  if(b == 0 || poldegree(b) < 0, lc = pollead(a); return([1 / lc, 0 * X, a / lc]));
  if(poldegree(b) == 0, return([0 * X, 1 / polcoeff(b, 0) + 0 * X, 1 + 0 * X]));
  if(a == 0 || poldegree(a) < 0, lc = pollead(b); return([0 * X, 1 / lc + 0 * X, b / lc]));
  if(poldegree(a) == 0, return([1 / polcoeff(a, 0) + 0 * X, 0 * X, 1 + 0 * X]));
  r = bezout(a, b); lc = pollead(r[3]);
  [r[1] / lc, r[2] / lc, r[3] / lc];
}
cantor_ref(u1, v1, u2, v2) =
{
  my(f = FPOL, r, e1, e2, d1, c1, c2, d, s1, s2, s3, u, v, lc, t);
  r = bezq(u1, u2); e1 = r[1]; e2 = r[2]; d1 = r[3];
  r = bezq(d1, v1 + v2); c1 = r[1]; c2 = r[2]; d = r[3];
  s1 = c1 * e1; s2 = c1 * e2; s3 = c2;
  u = (u1 * u2) / (d^2);
  v = (s1 * u1 * v2 + s2 * u2 * v1 + s3 * (v1 * v2 + f)) / d;
  u = u + 0 * X; v = v + 0 * X;
  v = v % u;
  while(poldegree(u) > 2,
    u = (f - v^2) / u; lc = pollead(u); u = u / lc; v = (-v) % u);
  if(poldegree(u) < 0, return(IDJ));
  v = v % u;
  [u, v];
}
\\ canonical output of a pair of polynomials: 4-vector if deg u == 2 else the pair
fromreg(u, v) =
{
  my(a, b, c, d);
  if(poldegree(u) == 2,
    a = lift(polcoeff(u, 1)); b = lift(polcoeff(u, 0));
    c = lift(polcoeff(v, 1)); d = lift(polcoeff(v, 0));
    return([a % Q, b % Q, c % Q, d % Q]));
  [u + 0 * X, v + 0 * X];
}
jisid(D) = if(#D == 4, 0, poldegree(D[1]) == 0);
jneg(D) = if(#D == 4, [D[1], D[2], (Q - D[3]) % Q, (Q - D[4]) % Q], [D[1], -D[2]]);
jeq(D1, D2) = if(#D1 == 4 && #D2 == 4, D1 == D2, if(#D1 == 4 || #D2 == 4, 0, D1[1] == D2[1] && D1[2] == D2[2]));
jfallback(D1, D2) =
{
  my(p1, p2, r);
  nfb++; jops++;
  p1 = mkpol(D1); p2 = mkpol(D2);
  r = cantor_ref(p1[1], p1[2], p2[1], p2[2]);
  fromreg(r[1], r[2]);
}
jref(D1, D2) = { my(p1 = mkpol(D1), p2 = mkpol(D2), r); r = cantor_ref(p1[1], p1[2], p2[1], p2[2]); fromreg(r[1], r[2]); }

\\ ---------------- explicit reduction step shared by add and double ----------------
\\ input: u = u1*u2 (or u^2) top coefficients U3,U2, and v = v3 X^3 + v2 X^2 + v1 X + v0 (v3 != 0) with u | f - v^2.
\\ returns [a,b,c,d] of the reduced divisor ((f-v^2)/u monic, -v mod that).
jredu(U3, U2, v3, v2, v1, v0) =
{
  my(m1, m2, m3, m4, D5, D4, q2, q1, q0, iq, A, B, AB, A2, r1, r0, q = Q);
  m1 = v3 * v3 % q; m2 = v3 * v2 % q; m3 = v2 * v2 % q; m4 = v3 * v1 % q;       \\ 2S? (m1,m3 squares) 2M
  D5 = (1 - 2 * m2) % q; D4 = (-(m3 + 2 * m4)) % q;
  q2 = (-m1) % q;
  q1 = (D5 - q2 * U3) % q;                                                       \\ 1M
  q0 = (D4 - q2 * U2 - q1 * U3) % q;                                             \\ 2M
  iq = lift(Mod(q2, q)^-1);                                                      \\ 1I
  A = q1 * iq % q; B = q0 * iq % q;                                              \\ 2M
  A2 = A * A % q; AB = A * B % q;                                                \\ 1S 1M
  r1 = (v1 - v2 * A + v3 * (A2 - B)) % q;                                        \\ 2M
  r0 = (v0 - v2 * B + v3 * AB) % q;                                              \\ 2M
  cM += 2 + 1 + 2 + 2 + 1 + 2 + 2; cS += 3; cI += 1;                             \\ total reduction: M=12 (v3v2,v3v1,q2U3,q2U2,q1U3,A,B,AB,v2A,v3(),v2B,v3AB) S=3 I=1
  [A, B, (q - r1) % q, (q - r0) % q];
}

\\ ---------------- explicit double (general case: deg u = 2, c != 0, Res(u,v) != 0, resulting v3 != 0) ----------------
jdbl_fast(P) =
{
  my(q = Q, a, b, c, d, a2, ab, c2, k1, k0, kk1, kk0, rho1, cd, ac, N1, N0, ir, s1, s0, v3, v2, v1, v0, U3, U2);
  a = P[1]; b = P[2]; c = P[3]; d = P[4];
  if(c == 0, return(0));
  a2 = a * a % q; ab = a * b % q; c2 = c * c % q;                                \\ 2S?? (a2,c2) 1M (ab)
  k1 = (F3 - b + a2) % q;
  k0 = (F2 - c2 + ab - a * k1) % q;                                              \\ 1M
  kk1 = (2 * a2 - b + k1) % q; kk0 = (2 * ab + k0) % q;
  cd = c * d % q;
  rho1 = (d * d - a * cd + b * c2) % q;                                          \\ 1S 2M (a*cd, b*c2)  [cd counted below]
  if(rho1 == 0, return(0));
  ac = a * c % q;
  N1 = (kk0 * c - kk1 * d) % q;                                                  \\ 2M
  N0 = (kk0 * (ac - d) - b * c % q * kk1) % q;                                   \\ ac:1M, kk0*(..):1M, b*c:1M, *kk1:1M
  ir = lift(Mod(2 * rho1, q)^-1);                                                \\ 1I
  s1 = (-N1 * ir) % q; s0 = (-N0 * ir) % q;                                      \\ 2M
  v3 = s1;
  if(v3 == 0, return(0));
  v2 = (s0 + a * s1) % q; v1 = (a * s0 + b * s1 + c) % q; v0 = (b * s0 + d) % q; \\ 4M
  U3 = 2 * a % q; U2 = (a2 + 2 * b) % q;
  \\ counts: S: a2, c2, d*d = 3 ; M: ab, a*k1, cd, a*cd, b*c2, ac, kk0*c, kk1*d, kk0*(ac-d), b*c, (bc)*kk1, s1, s0, a*s1, a*s0, b*s1, b*s0 = 17 ; I: 1
  cM += 17; cS += 3; cI += 1;
  jredu(U3, U2, v3, v2, v1, v0);
}

\\ ---------------- explicit add (general case: both deg 2, u2-u1 of degree 1, coprime, v3 != 0) ----------------
jadd_fast(P, Q2) =
{
  my(q = Q, a1, b1, c1, d1, a2, b2, c2, d2, e1, e0, e01, rho1, dc, dd, n1, n0, ir, s1, s0, v3, v2, v1, v0, U3, U2, a2e1);
  a1 = P[1]; b1 = P[2]; c1 = P[3]; d1 = P[4]; a2 = Q2[1]; b2 = Q2[2]; c2 = Q2[3]; d2 = Q2[4];
  e1 = (a2 - a1) % q; e0 = (b2 - b1) % q;
  if(e1 == 0, return(0));
  dc = (c2 - c1) % q; dd = (d2 - d1) % q;
  e01 = e0 * e1 % q;
  rho1 = (e0 * e0 - a2 * e01 + b2 * e1 * e1) % q;                                \\ S: e0^2, e1^2 (2S); M: e0*e1, a2*e01, b2*(e1^2) (3M)
  if(rho1 == 0, return(0));
  n1 = (dd * e1 - dc * e0) % q;                                                  \\ 2M
  a2e1 = a2 * e1 % q;                                                            \\ 1M
  n0 = (dd * (a2e1 - e0) - b2 * dc % q * e1) % q;                                \\ dd*(.):1M, b2*dc:1M, *e1:1M
  ir = lift(Mod(rho1, q)^-1);                                                    \\ 1I
  s1 = n1 * ir % q; s0 = n0 * ir % q;                                            \\ 2M
  v3 = s1;
  if(v3 == 0, return(0));
  v2 = (s0 + a1 * s1) % q; v1 = (a1 * s0 + b1 * s1 + c1) % q; v0 = (b1 * s0 + d1) % q;   \\ 4M
  U3 = (a1 + a2) % q; U2 = (b1 + b2 + a1 * a2) % q;                              \\ 1M
  \\ counts: S 2 ; M: 3 + 2 + 1 + 3 + 2 + 4 + 1 = 16 ; I 1
  cM += 16; cS += 2; cI += 1;
  jredu(U3, U2, v3, v2, v1, v0);
}

\\ group addition (counts one Jop).  Fast path when both operands are degree-2 and the explicit formula applies; Cantor fallback otherwise.
jadd(P, R) =
{
  my(res, q = Q);
  if(#P == 4 && #R == 4,
    if(P == R,
      res = jdbl_fast(P);
      if(res, jops++; ndbl2++; return(res)),
      if(P[1] == R[1] && P[2] == R[2],
        if((P[3] + R[3]) % q == 0 && (P[4] + R[4]) % q == 0, nidn++; jops++; return(IDJ)));   \\ P + (-P) = O (trivial; still one Jop)
      res = jadd_fast(P, R);
      if(res, jops++; nadd2++; return(res))));
  jfallback(P, R);
}
jdbl(P) = jadd(P, P);
\\ scalar multiplication by n >= 0 (double-and-add, MSB first); every add/double is a counted Jop. n = 0 returns the identity.
jmul(n, P) =
{
  my(bits, R, i);
  if(n == 0, return(IDJ));
  bits = binary(n); R = P;
  for(i = 2, #bits, R = jadd(R, R); if(bits[i], R = jadd(R, P)));
  R;
}
\\ Mumford form of the point (x0,y0) of C' as the degree-1 element [X - x0, y0]
jpoint(x0, y0) = [Mod(1, Q) * X - x0 % Q, Mod(y0 % Q, Q) + 0 * X];
\\ random affine point (x, y) with y canonical? returns [x0,y0] with y0 random sign
jrandpt() =
{
  my(x0, fx, y0);
  while(1,
    x0 = random(Q); fx = subst(lift(FPOL), X, x0) % Q;
    if(kronecker(fx, Q) == 1,
      y0 = lift(sqrt(Mod(fx, Q)));
      if(random(2), y0 = (Q - y0) % Q);
      return([x0, y0])));
}
\\ random Jacobian element: the sum of two random points (counted as Jops)
jrand() = { my(p1 = jrandpt(), p2 = jrandpt()); jadd(jpoint(p1[1], p1[2]), jpoint(p2[1], p2[2])); }
