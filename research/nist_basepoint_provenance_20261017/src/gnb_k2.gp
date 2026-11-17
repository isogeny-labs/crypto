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


halve(U, V, a, m) = {
  my(cc = U + a, lam, out = []);
  lam = sum(i = 0, (m-1)/2, cc^(4^i));
  if(lam^2 + lam != cc, return([]));
  for(t = 0, 1, my(L = lam + t, X = (V + U*L + U)^(2^(m-1)), Y = L*X + X^2, L2, U2, V2);
    L2 = X + Y/X; U2 = L2^2 + L2 + a; V2 = X^2 + U2*(L2 + 1);
    if(U2 != U || V2 != V, error("halving check failed"));
    out = concat(out, [[X, Y]]));
  out
};
read("kcurves.gp");
run_k(kc) = {
  my(name, m, a0, h, gx, gy, bc, Py, a, f, R, beta, Mt, Mi, e, cb, orient, s, d, lev, nxt);
  name = kc[1]; m = kc[2]; a0 = kc[3]; h = kc[4]; gx = kc[5]; gy = kc[6];
  bc = 0; for(t = 1, #BC, if(BC[t][2] == m, bc = BC[t]));
  Py = subst(bc[4], 'x, 'y); a = ffgen(Mod(1, 2)*Py, 'a);
  f = gnb_f(m, bc[3]); R = polrootsmod(f, [Py, 2]); beta = subst(lift(lift(R[1])), 'y, a);
  Mt = matrix(m, m); e = beta;
  for(i = 1, m, my(v = tobits(e, m)); for(j = 1, m, Mt[j, i] = Mod((v >> (j-1)) % 2, 2)); e = e^2);
  Mi = Mt^(-1);
  cb = conv(fromint(bc[5], a), m, Mi); orient = [];
  for(ss = 0, m-1, for(dd = 0, 1, if(orient_int(cb, m, ss, dd) == bc[8], orient = concat(orient, [[ss, dd]]))));
  if(#orient != 1, error("orientation"));
  s = orient[1][1]; d = orient[1][2];
  lev = [[fromint(gx, a), fromint(gy, a)]];
  for(step = 1, valuation(h, 2), nxt = []; for(i = 1, #lev, nxt = concat(nxt, halve(lev[i][1], lev[i][2], a0, m))); lev = nxt);
  write("gnb_k.log", name, " a=", a0, " h=", h, ": ", #lev, " points R with h*R = G");
  write("k_targets.txt", name, " ", m, " Gx_poly ", gx);
  write("k_targets.txt", name, " ", m, " Gx_nb ", orient_int(conv(fromint(gx, a), m, Mi), m, s, d));
  write("k_targets.txt", name, " ", m, " Gy_nb ", orient_int(conv(fromint(gy, a), m, Mi), m, s, d));
  for(i = 1, #lev,
    write("k_targets.txt", name, " ", m, " R", i, "x_poly ", tobits(lev[i][1], m));
    write("k_targets.txt", name, " ", m, " R", i, "y_poly ", tobits(lev[i][2], m));
    write("k_targets.txt", name, " ", m, " R", i, "x_nb ", orient_int(conv(lev[i][1], m, Mi), m, s, d));
    write("k_targets.txt", name, " ", m, " R", i, "y_nb ", orient_int(conv(lev[i][2], m, Mi), m, s, d)));
  1
};
for(ki = 1, #KC, run_k(KC[ki]));
