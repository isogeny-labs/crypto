default(parisizemax, 2000000000);
read("lib.gp");
read("raw/curves.txt");
print("\\\\ check_identity.gp  gp ", version());
fq(x) = x^3 + A_ * x^2 + x;
\\ ---- test id 1: ladder vs ellmul, E and E', b in {20,24,28}, inst 1,2, 2000 random x, k in [2,53]
setrand(41000001);
{
for(bi = 1, 3,
  my(b = 20 + 4*(bi-1));
  for(inst = 1, 2,
    my(rec = mapget(CURVES, [b, inst]), tE = 0, tEp = 0, mE = 0, mEp = 0, iE = 0, iEp = 0, nx = 0, xx, f, y, pt, R, r, pk, cE = 0, cEp = 0);
    setctx(rec, 0);
    while(nx < 2000,
      xx = Mod(random(P_), P_);
      f = fq(xx);
      if(f == 0, next);
      nx++;
      if(issquare(f),
        cE++; pt = [xx, sqrt(f)];
        for(k = 2, 53,
          R = ellmul(E0, pt, k); r = lad(xx, kbits(k)); tE++;
          if(R == [0], iE++; if(r[2] != 0, mE++),
            if(r[2] == 0 || r[1] / r[2] != R[1], mE++))),
        cEp++; pt = [D_ * xx, sqrt(D_^3 * f)];
        if(!ellisoncurve(E1, pt), error("twist lift failed"));
        for(k = 2, 53,
          R = ellmul(E1, pt, k); r = lad(xx, kbits(k)); tEp++;
          if(R == [0], iEp++; if(r[2] != 0, mEp++),
            if(r[2] == 0 || r[1] / r[2] != R[1] / D_, mEp++)))));
    print("chk(\"ladder_b", b, "_i", inst, "\",[", cE, ",", cEp, ",", tE, ",", tEp, ",", mE, ",", mEp, ",", iE, ",", iEp, "]);  \\\\ [x_on_E, x_on_Et, tests_E, tests_Et, mismatch_E, mismatch_Et, inf_E, inf_Et]")));
}
\\ ---- test id 2: wrong-solution check (NC4), 1000 tests per instance and curve
setrand(41000002);
{
for(bi = 1, 6,
  my(b = 20 + 4*(bi-1));
  for(inst = 1, 2,
    my(rec = mapget(CURVES, [b, inst]));
    for(tw = 0, 1,
      my(okc = 0, wrongpass = 0, special = 0, a, Q, n, w);
      setctx(rec, tw); n = NN;
      for(t = 1, 1000,
        a = random(n - 3) + 2; Q = ellmul(Ea, Pa, a);
        if(ellmul(Ea, Pa, a)[1] == Q[1], okc++);
        w = (a + 1) % n;
        if(ellmul(Ea, Pa, w)[1] == Q[1], wrongpass++);
        if((2 * a + 1) % n == 0, special++));
      print("chk(\"nc4_b", b, "_i", inst, "_t", tw, "\",[", okc, ",", wrongpass, ",", special, "]);  \\\\ [true_passes_of_1000, wrong_a+1_passes, a with 2a+1=0 mod n]"))));
}
