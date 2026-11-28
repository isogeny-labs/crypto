\\ rho.gp -- negation-map Pollard rho (G1 N1 design) on a generic group given by closures, used for R_E (on E) and R_J (on Jac(C')),
\\ and the plain rho (A0: no negation map, no cycle test) used in PC-C.
\\ A group is described by a vector GR = [gadd, gneg, gisid, gcanon, gsel, gkey]:
\\   gadd(P,R) group addition (also doubles; counted by the group's own counters), gneg(P), gisid(P), gcanon(P) = 1 if P is the canonical
\\   representative of {P,-P}, gsel(P) = negation-invariant integer used for selector and DP test, gkey(P) = integer identifying the canonical point.
\\ Walk: 32-entry table W_j = u_j G + v_j H, selector j = ((gsel(P) >> d) mod 32) + 1, DP iff gsel(P) mod 2^d == 0,
\\ d = max(2, floor(log2(sqrt(l/2)) / 2)); 2-cycle test + Brent saved-point test, escape by canonical doubling; trail cap 20*2^d; budget 40*sqrt(l/2) steps.
\\ Counters of the rho layer (globals): rSTEP rTRAIL rDP rESC2 rESCB rCAND rWASTE.

rhobits(l) = max(2, floor(log(sqrt(l / 2.)) / log(2) / 2));

\\ generic scalar multiplication through the group's add (MSB first double-and-add), n >= 1
rhomul(GR, n, P) =
{
  my(bits, R, i, gadd = GR[1]);
  bits = binary(n); R = P;
  for(i = 2, #bits, R = gadd(R, R); if(bits[i], R = gadd(R, P)));
  R;
}

\\ run: GR group, G, H, l, mode 0 = negation map (N1), 1 = plain (A0).  Returns [ok, xcand, steps, trails, dps, esc2, escB, cand, waste].
rhorun(GR, G, H, l, mode) =
{
  my(gadd = GR[1], gneg = GR[2], gisid = GR[3], gcan = GR[4], gsel = GR[5], gkey = GR[6],
     d = rhobits(l), dm, r2 = 32, WP = vector(32), WU = vector(32), WV = vector(32), j, budget, steps = 0, trails = 0, dps = 0, esc2 = 0, escB = 0,
     cand = 0, waste = 0, ok = 0, sol = 0, map = Map(), al, be, P, cap, st, R, xm1, xs, pw, lam, esc, dead, isdp, ev, den, a1, kk, lx, lcap, pt, Tt);
  dm = 1 << d; budget = ceil(40 * sqrt(l / 2.)); lcap = 20 * dm;
  for(j = 1, r2,
    WU[j] = 1 + random(l - 1); WV[j] = 1 + random(l - 1);
    WP[j] = gadd(rhomul(GR, WU[j], G), rhomul(GR, WV[j], H)));
  while(!ok && steps < budget,
    trails++;
    al = random(l); be = random(l);
    if(al == 0 && be == 0, next);
    P = if(al && be, gadd(rhomul(GR, al, G), rhomul(GR, be, H)), if(al, rhomul(GR, al, G), rhomul(GR, be, H)));
    if(gisid(P), next);
    if(mode == 0 && !gcan(P), P = gneg(P); al = (l - al) % l; be = (l - be) % l);
    cap = min(lcap, budget - steps);
    st = 0; xm1 = -1; xs = gkey(P); pw = 1; lam = 0; dead = 0; isdp = 0;
    while(st < cap,
      lx = gsel(P);
      j = ((lx >> d) % r2) + 1;
      R = gadd(P, WP[j]);
      st++;
      if(gisid(R), dead = 1; break);
      al = (al + WU[j]) % l; be = (be + WV[j]) % l;
      if(mode == 0,
        if(!gcan(R), R = gneg(R); al = (l - al) % l; be = (l - be) % l);
        esc = 0;
        kk = gkey(R);
        if(kk == xm1, esc = 1; esc2++);
        if(!esc,
          if(kk == xs, esc = 2; escB++, lam++; if(lam == pw, xs = kk; pw *= 2; lam = 0)));
        if(esc,
          R = gadd(R, R);
          if(gisid(R), dead = 1; break);
          al = (2 * al) % l; be = (2 * be) % l;
          if(!gcan(R), R = gneg(R); al = (l - al) % l; be = (l - be) % l);
          xs = gkey(R); pw = 1; lam = 0; xm1 = -1,
          xm1 = gkey(P)));
      P = R;
      if(gsel(P) % dm == 0, isdp = 1; break));
    steps += st;
    if(isdp,
      dps++; kk = gkey(P);
      if(mapisdefined(map, kk, &ev),
        cand++;
        den = (be - ev[2]) % l;
        if(den == 0, waste++,
          a1 = ((ev[1] - al) * lift(Mod(den, l)^-1)) % l;
          if(a1 != 0 && RHOCHECK(a1), ok = 1; sol = a1, waste++)),
        mapput(map, kk, [al, be])));
  );
  [ok, sol, steps, trails, dps, esc2, escB, cand, waste];
}
\\ RHOCHECK(x') is set by the caller (closure: does x' G = H on the verifier?) -- used only to stop the walk on the first
\\ candidate that verifies (an unverified candidate counts as waste, as in G1).

\\ ---- group descriptors ----
\\ E: points [x,y] or [0]
E_gadd(P, R) = eadd(P, R);
E_gneg(P) = eneg(P);
E_gisid(P) = (#P == 1);
E_gcan(P) = (P[2] <= QH);
E_gsel(P) = P[1];
E_gkey(P) = P[1];
\\ J: 4-vectors (and the identity)
J_gadd(P, R) = jadd(P, R);
J_gneg(P) = jneg(P);
J_gisid(P) = jisid(P);
J_gcan(P) = if(#P != 4, 1, if(P[3] != 0, P[3] <= QH, P[4] <= QH));
\\ selector: u1 (not u0: u0 is constant on the image of E for this gluing family, DEVIATIONS D1)
J_gsel(P) = if(#P == 4, P[1], 0);
J_gkey(P) = jkey(P);
EGR = [E_gadd, E_gneg, E_gisid, E_gcan, E_gsel, E_gkey];
JGR = [J_gadd, J_gneg, J_gisid, J_gcan, J_gsel, J_gkey];
