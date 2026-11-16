default(parisize, 2*10^9);
read("bcurves.gp");

tobits(e, m) = {my(P = lift(e.pol), s = 0); for(j = 0, m-1, if(polcoef(P, j) != 0, s += 2^j)); s};
fromint(v, a) = {my(s = 0*a, j = 0); while(v, if(v % 2, s += a^j); v \= 2; j++); s};

gnb_f(m, T) = {
  my(p = m*T + 1, k, w, g = 1, u, beta);
  if(!isprime(p), error("p not prime"));
  k = znorder(Mod(2, p)); w = ffgen(ffinit(2, k), 'w);
  while(g == 1, g = random(w)^((2^k - 1)/p));
  u = znprimroot(p)^((p - 1)/T);
  beta = sum(j = 0, T-1, g^lift(u^j));
  minpoly(beta, 'x)
};

\\ normal-basis coordinates (vector, index i = coefficient of beta^(2^i))
conv(z, m, Mi) = {my(v = tobits(z, m)); lift(Mi * vectorv(m, j, Mod((v >> (j-1)) % 2, 2)))};

\\ oriented integer: bit string read MSB-first with shift s and direction d
orient_int(cz, m, s, d) = sum(i = 0, m-1, cz[if(d, (s - i) % m, (s + i) % m) + 1] * 2^(m-1-i));

run_curve(c) = {
  my(name = c[1], m = c[2], T = c[3], Pm = c[4], b = c[5], gx = c[6], gy = c[7], W = c[8]);
  my(Py = subst(Pm, 'x, 'y), a = ffgen(Mod(1, 2)*Py, 'a), f, R, beta, Mt, Mi, e, cb, orient, s, d);
  f = gnb_f(m, T);
  R = polrootsmod(f, [Py, 2]);
  beta = subst(lift(lift(R[1])), 'y, a);
  Mt = matrix(m, m); e = beta;
  for(i = 1, m, my(v = tobits(e, m)); for(j = 1, m, Mt[j, i] = Mod((v >> (j-1)) % 2, 2)); e = e^2);
  Mi = Mt^(-1);
  cb = conv(fromint(b, a), m, Mi);
  orient = [];
  for(ss = 0, m-1, for(dd = 0, 1, if(orient_int(cb, m, ss, dd) == W, orient = concat(orient, [[ss, dd]]))));
  printf("%s: GNB type %d, orientations where normal-basis b equals SHA-1(seed) output: %s\n", name, T, orient);
  if(#orient == 0, return(0));
  s = orient[1][1]; d = orient[1][2];
  my(U = fromint(gx, a), V = fromint(gy, a), cc, lam, nh = 0);
  write("bin_targets.txt", name, " ", m, " Gx_nb ", orient_int(conv(U, m, Mi), m, s, d));
  write("bin_targets.txt", name, " ", m, " Gy_nb ", orient_int(conv(V, m, Mi), m, s, d));
  cc = U + 1;
  lam = sum(i = 0, (m-1)/2, cc^(4^i));
  if(lam^2 + lam != cc, print("  G has no half"); return(1));
  for(t = 0, 1,
    my(L = lam + t, X, Y, L2, U2, V2);
    X = (V + U*L + U)^(2^(m-1)); Y = L*X + X^2;
    L2 = X + Y/X; U2 = L2^2 + L2 + 1; V2 = X^2 + U2*(L2 + 1);
    if(U2 != U || V2 != V, error("halving check failed"));
    nh++;
    write("bin_targets.txt", name, " ", m, " half", t+1, "x_poly ", tobits(X, m));
    write("bin_targets.txt", name, " ", m, " half", t+1, "y_poly ", tobits(Y, m));
    write("bin_targets.txt", name, " ", m, " half", t+1, "x_nb ", orient_int(conv(X, m, Mi), m, s, d));
    write("bin_targets.txt", name, " ", m, " half", t+1, "y_nb ", orient_int(conv(Y, m, Mi), m, s, d)));
  printf("  both halves verified by doubling: %d of 2\n", nh);
  1
};

for(ci = 1, #BC, run_curve(BC[ci]));
