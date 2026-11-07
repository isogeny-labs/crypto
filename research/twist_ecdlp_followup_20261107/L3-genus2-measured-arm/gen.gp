\\ gen.gp -- curve and instance generation for one size k (all randomness from the generation stream setrand(SEED0*1000 + k)).
\\ Globals expected: MASTER (master seed incl. control tag offset).  Sets Q.  Returns the list of curves with their instances.
\\ NO checks that consume randomness are done here, so the sample is identical whichever cell later processes it.

gen_size(k, ncurves, ninst) =
{
  my(q = precprime(2^k), curves = List(), seenAB = Map(), A, B, cv, tries = 0, nsing = 0, ndupAB = 0, insts, G, H, x, TG, TH, P, red, nre, l, h, c, i);
  Q = q;
  setrand(MASTER * 1000 + k);
  rejinit();
  while(#curves < ncurves,
    A = 1 + random(q - 1); B = 1 + random(q - 1); tries++;
    if((4 * A^3 + 27 * B^2) % q == 0, nsing++; next);
    if(mapisdefined(seenAB, [A, B]), ndupAB++; next);
    cv = build_pair(A, B);
    if(cv == 0, next);
    mapput(seenAB, [A, B], 1);
    listput(curves, cv));
  GENSTAT = [tries, nsing, ndupAB, REJ];
  \\ instances
  GENINST = vector(ncurves);
  for(c = 1, ncurves,
    cv = curves[c]; setcurve(cv); l = cv[5]; h = cv[6];
    insts = vector(ninst);
    for(i = 1, ninst,
      nre = 0;
      while(1,
        P = erandpt(); G = emul(h, P);
        if(#G == 1 || emul(l, G) != [0], nre++; next);
        x = 1 + random(l - 1);
        H = emul(x, G);
        tM = 0; tS = 0; tI = 0;
        TG = tfer(G); TH = tfer(H);
        if(TG == 0 || TH == 0, nre++; next);
        break);
      insts[i] = [G, H, x, TG, TH, tM, tS, tI, nre]);
    GENINST[c] = insts);
  Vec(curves);
}
