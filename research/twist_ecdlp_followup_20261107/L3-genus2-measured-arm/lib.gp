\\ lib.gp -- loader and shared helpers.  Variable SRC = directory of the scripts (set by caller via getenv("SRC")).
SRC = getenv("SRC");
read(Str(SRC, "/jac.gp")); read(Str(SRC, "/glue.gp")); read(Str(SRC, "/ic.gp")); read(Str(SRC, "/rho.gp")); read(Str(SRC, "/gen.gp"));
RHOMODE = 0;   \\ 0: verify on E (everify), 1: accept every candidate (NC-B), 2: verify in a random Jacobian (PC-C plain rho)
RHOCHECKJ(xp) =
{
  my(s1 = jops, s2 = cM, s3 = cS, s4 = cI, s5 = nfb, s6 = nadd2, s7 = ndbl2, s8 = nidn, r);
  r = jeq(jmul(xp, RGT), RHT);
  jops = s1; cM = s2; cS = s3; cI = s4; nfb = s5; nadd2 = s6; ndbl2 = s7; nidn = s8;
  r;
}
RHOCHECK(xp) = if(RHOMODE == 0, everify(RG, RH, xp), if(RHOMODE == 1, 1, RHOCHECKJ(xp)));
armseed(k, c, i, a) = MASTER * 10^6 + k * 10^4 + c * 100 + i * 10 + a;
emit(f, v) = write(f, v);
\\ field layouts of the raw lines (documentation, also used by analyze.gp)
RHOF = ["tag","k","c","i","arm","q","l","solved","ver","p1rej","steps","trails","dps","esc2","escB","cand","waste","core","cM","cS","cI","nfb","wall"];
ICF = ["tag","k","c","i","arm","q","l","solved","ver","p1rej","trials","jops","cM","cS","cI","nfb","jsetup","nsplit","ndeg1","ndeg0","ndisc0","nFF","nFL","nLL","nrej","ndup","nrestart","nsolve","rowops","rowdels","nnz","nsq","nacc","wall"];

\\ run one rho arm (R_E: grp "E", R_J: grp "J"); G,H on E (verifier), (PG,PH) points in the arm's group.  Returns the emitted vector.
run_rho(tag, k, c, i, aid, grp, G, H, PG, PH, l, mode, verifier) =
{
  my(r, t0, ver = 0, p1 = -1, v);
  RG = G; RH = H;
  RHOMODE = if(verifier, 0, 1);
  setrand(armseed(k, c, i, aid));
  t0 = getwalltime();
  if(grp == "E", ezero(); r = rhorun(EGR, PG, PH, l, mode), jzero(); r = rhorun(JGR, PG, PH, l, mode));
  t0 = getwalltime() - t0;
  if(r[1], ver = everify(G, H, r[2]); p1 = !everify(G, H, (r[2] % l) + 1));
  v = [tag, k, c, i, aid, Q, l, r[1], ver, p1, r[3], r[4], r[5], r[6], r[7], r[8], r[9], if(grp == "E", eops, jops), if(grp == "E", eM, cM), if(grp == "E", eS, cS), if(grp == "E", eI, cI), if(grp == "E", 0, nfb), t0];
  v;
}
run_ic(tag, k, c, i, aid, arm, G, H, PG, PH, l, sabotage) =
{
  my(r, ver = 0, p1 = -1, v);
  setrand(armseed(k, c, i, aid));
  jzero();
  r = icrun(PG, PH, l, arm, sabotage);
  if(r[1], ver = everify(G, H, r[2]); p1 = !everify(G, H, (r[2] % l) + 1));
  v = [tag, k, c, i, aid, Q, l, r[1], ver, p1, r[3], r[4], r[5], r[6], r[7], r[8], r[9], r[10], r[11], r[12], r[13], r[14], r[15], r[16], r[17], r[18], r[19], r[20], r[21], r[22], r[23], r[24], r[25], r[26]];
  v;
}
