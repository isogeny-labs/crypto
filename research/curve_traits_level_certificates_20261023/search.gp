\\ Search for a point certificate of the depth-1 volcano level at ell of a
\\ curve over GF(q), q = p^k0 (registry basis z), and print it in the tower
\\ basis GF(q^m) = GF(q)[w]/(g) that curve_traits verifies (points.rs,
\\ verify_level_certificate).  Inputs (set before reading this file):
\\   p, k0, modv (low coefficients of the registry modulus), av, bv (a, b in
\\   the registry basis), t (trace over GF(q)), ell, seed, tries.
\\ Output, one line: CERT <kind> <m> <twist> <g> <d> <points>, every field
\\ element as its coefficient vector in the tower basis.
setrand(seed);
z = ffgen(Mod(1, p) * Polrev(concat(modv, [1]), 'z), 'z);
el(v) = subst(Polrev(v, 'z), 'z, z);
a = el(av); b = el(bv); q = p^k0;
k = znorder(Mod(t, ell) / 2);
twist = (k % 2 == 0); m = if (twist, k / 2, k);
x = ffgen(ffinit(p, k0 * m, 'x), 'x);
emb = ffembed(z, x); inv = ffinvmap(emb);
\\ g: the minimal polynomial of x over GF(q), so L = GF(q)[w]/(g), w -> x.
c = x; G = 'W - x; for (i = 1, m - 1, c = c^q; G *= ('W - c));
g = Pol(apply(u -> ffmap(inv, u), Vec(G)), 'W);
if (poldegree(g) != m || !polisirreducible(g), error("bad g"));
\\ Tower coordinates of y in L: reduce y.pol(W) mod g over GF(q).
coords(y) = my(Y = Pol(Vec(y.pol), 'W) * z^0, R = Y % g); \
  vector(m, i, my(u = polcoef(R, i - 1, 'W) + 0 * z); \
    my(cf = Vecrev(u.pol)); concat(cf, vector(k0 - #cf))) ;
d = 0;
if (twist, \
  if (m % 2 == 1, d = z; while (issquare(d), d += 1); dL = ffmap(emb, d), \
    dL = random(x); while (issquare(dL), dL = random(x))), \
  dL = x^0);
Vk(n) = my(u = 2, w = t, y); if (n == 0, return(2)); for (i = 2, n, y = t * w - q * u; u = w; w = y); w;
N = if (twist, q^m + 1 + Vk(m), q^m + 1 - Vk(m));
v = valuation(N, ell); cof = N / ell^v;
aL = ffmap(emb, a); bL = ffmap(emb, b);
E = ellinit([aL * dL^2, bL * dL^3]);
lowpts = List(); kind = "none"; pts = [];
for (i = 1, tries, \
  my(P = random(E), Q = ellmul(E, P, cof), e = 0, R = Q); \
  while (R != [0], R = ellmul(E, R, ell); e++); \
  if (e == v, kind = "floor"; pts = [Q]; break); \
  if (e >= 1, my(T = ellmul(E, Q, ell^(e - 1))); \
    for (j = 1, #lowpts, if (ellweilpairing(E, lowpts[j], T, ell) != 1, kind = "crater"; pts = [lowpts[j], T]; break)); \
    if (kind == "crater", break); listput(lowpts, T)));
gv = vector(m, i, my(u = polcoef(g, i - 1, 'W) + 0 * z, cf = Vecrev(u.pol)); concat(cf, vector(k0 - #cf)));
dv = if (!twist, [], coords(dL));
\\ Self-check of the conversion: each point satisfies the (twisted) curve
\\ equation in GF(q)[w]/(g).
tw(vv) = Mod(sum(i = 1, m, el(vv[i]) * 'W^(i - 1)), g);
D = if (twist, tw(dv), Mod(1, g));
for (i = 1, #pts, my(X = tw(coords(pts[i][1])), Y = tw(coords(pts[i][2]))); \
  if (Y^2 != X^3 + a * D^2 * X + b * D^3, error("conversion check failed")));
print("CERT|", kind, "|", m, "|", twist, "|", v, "|", gv, "|", dv, "|", vector(#pts, i, [coords(pts[i][1]), coords(pts[i][2])]));
