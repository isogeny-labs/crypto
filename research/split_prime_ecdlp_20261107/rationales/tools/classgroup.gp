\\ Class-group facts for the Frobenius field K = Q(sqrt(t^2 - 4p)) of a census curve.
\\ Two stages, because quadclassunit is the only slow step:
\\   facts(label, p, n)                 fast: factor Delta, D_K, conductor f, analytic
\\                                      estimate of h(D_K) from the Euler product over l <= 10^6
\\   cl(label, p, n)                    slow: quadclassunit(D_K) under GRH -> h, cyc
\\   orders(label, p, n, h, cyc)        fast given h: order of the prime form above every
\\                                      split odd l <= 2000 (via the factorisation of h), the
\\                                      lcm of those orders up to B in {50, 163, 691, 2000},
\\                                      the index h / lcm (exact when Cl is cyclic, an upper
\\                                      bound on the index otherwise), the Mazur subset,
\\                                      and the smallest l whose form alone has order h.
\\ Usage: gp -q -f tools/classgroup.gp with the calls on stdin; JSON on stdout, one object per call.
\\ Dependency: PARI/GP 2.15.4 (Debian pari-gp). Wall seconds are descriptive only.

default(parisizemax, 4*10^9);
default(realprecision, 38);

mazur_odd = [3, 5, 7, 11, 13, 17, 19, 37, 43, 67, 163];
bounds = [50, 163, 691, 2000];

jsonvec(v) = Str("[", concat(vector(#v, i, Str(v[i], if (i < #v, ", ", "")))), "]");

disc(p, n) = { my(t = p + 1 - n); t^2 - 4*p; }

facts(label, p, n) = {
  my(D = disc(p, n), t0 = getwalltime(), F, DK, f, w, L, hest);
  F = factor(D); DK = coredisc(D); f = sqrtint(D \ DK);
  w = if (DK == -3, 6, if (DK == -4, 4, 2));
  \\ analytic class number h = w sqrt(|D_K|) L(1, chi) / (2 pi); L(1, chi) by the partial Euler
  \\ product over primes l <= 10^6 (heuristic tail; the estimate is not a class number)
  L = 1.0; forprime (ell = 2, 10^6, L *= 1 / (1 - kronecker(DK, ell) / ell));
  hest = w * sqrt(abs(DK)) * L / (2 * Pi);
  print("{\"stage\": \"facts\", \"label\": \"", label, "\", \"bits\": ", #binary(p), ", \"t\": ", t = p + 1 - n,
        ", \"Delta_factor_primes\": ", jsonvec(F[,1]~), ", \"Delta_factor_exps\": ", jsonvec(F[,2]~),
        ", \"Delta_largest_prime_bits\": ", #binary(vecmax(F[,1])),
        ", \"D_K\": ", DK, ", \"conductor\": ", f, ", \"L1_partial_1e6\": ", L,
        ", \"h_estimate_log2\": ", log(hest) / log(2),
        ", \"wall_s\": ", (getwalltime() - t0) / 1000.0, "}");
}

cl(label, p, n) = {
  my(D = disc(p, n), DK = coredisc(D), t0 = getwalltime(), Q);
  Q = quadclassunit(DK);
  print("{\"stage\": \"cl\", \"label\": \"", label, "\", \"D_K\": ", DK, ", \"h_K\": ", Q.no,
        ", \"h_K_bits\": ", #binary(Q.no), ", \"cyc\": ", jsonvec(Q.cyc), ", \"wall_s\": ", (getwalltime() - t0) / 1000.0, "}");
}

\\ order of a form F in a group of exponent dividing h with known factorisation fh
formorder(F, h, fh) = {
  my(o = h, id = qfbred(qfbprimeform(poldisc(F), 1)));
  for (i = 1, #fh~,
    my(q = fh[i, 1]);
    while (o % q == 0 && qfbred(qfbpow(F, o / q)) == id, o \= q));
  o;
}

orders(label, p, n, h, cyc) = {
  my(D = disc(p, n), DK = coredisc(D), fh = factor(h), t0 = getwalltime(), L = 1, spl = [], ords = [], idx = vector(#bounds), mz = [], mords = [], Lm = 1, Bgen = 0, F, o, cyclic = (#cyc == 1));
  forprime (ell = 3, vecmax(bounds),
    if (kronecker(DK, ell) == 1,
      F = qfbprimeform(DK, ell); o = formorder(F, h, fh);
      spl = concat(spl, ell); ords = concat(ords, o); L = lcm(L, o);
      if (Bgen == 0 && o == h, Bgen = ell);
      if (vecsearch(mazur_odd, ell), mz = concat(mz, ell); mords = concat(mords, o); Lm = lcm(Lm, o)));
    for (b = 1, #bounds, if (ell == precprime(bounds[b]), idx[b] = h / L)));
  print("{\"stage\": \"orders\", \"label\": \"", label, "\", \"h_K\": ", h, ", \"cyc\": ", jsonvec(cyc), ", \"cyclic\": ", cyclic,
        ", \"h_K_factor_primes\": ", jsonvec(fh[,1]~), ", \"h_K_factor_exps\": ", jsonvec(fh[,2]~),
        ", \"h_K_largest_prime_bits\": ", #binary(vecmax(fh[,1])),
        ", \"split_odd\": ", jsonvec(spl), ", \"form_orders\": ", jsonvec(ords),
        ", \"index_bound_by_split_le\": {", concat(vector(#bounds, b, Str("\"", bounds[b], "\": ", idx[b], if (b < #bounds, ", ", "")))), "}",
        ", \"index_exact\": ", cyclic, ", \"mazur_split\": ", jsonvec(mz), ", \"mazur_form_orders\": ", jsonvec(mords), ", \"index_bound_by_mazur\": ", h / Lm,
        ", \"smallest_l_with_full_order\": ", Bgen, ", \"wall_s\": ", (getwalltime() - t0) / 1000.0, "}");
}
