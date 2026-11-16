default(parisizemax, 2000000000);
read("lib.gp");
read("lib_a1c.gp");
read("raw/curves.txt");
ARM = getenv("ARM"); CELLB = eval(getenv("B")); CELLI = eval(getenv("INST")); TRIALS = eval(getenv("TRIALS"));
RAWDIR = getenv("RAWDIR"); \\ unused (stdout is redirected by the driver)
\\ arm table: name -> [armid, kind, twist, set, amode]
{
ARMS = Map();
mapput(ARMS, "kpure",    [1,  "pure",  0, "SB", 1]);
mapput(ARMS, "kmix",     [2,  "mix",   0, "SB", 0]);
mapput(ARMS, "a0",       [3,  "a0",    0, "SB", 0]);
mapput(ARMS, "a1",       [4,  "a1",    0, "SB", 0]);
mapput(ARMS, "a1c",      [14, "a1c",   0, "SB", 0]);
mapput(ARMS, "kpureT",   [5,  "pure",  1, "SB", 1]);
mapput(ARMS, "cross",    [6,  "cross", 0, "SB", 0]);
mapput(ARMS, "kpure_sa", [7,  "pure",  0, "SA", 1]);
mapput(ARMS, "kpure_sc", [8,  "pure",  0, "SC", 1]);
mapput(ARMS, "kmix_sa",  [9,  "mix",   0, "SA", 0]);
mapput(ARMS, "kmix_sc",  [10, "mix",   0, "SC", 0]);
mapput(ARMS, "pc1",      [11, "pure",  0, "SIDX", 1]);
mapput(ARMS, "nc1",      [12, "pure",  0, "SIDX", 2]);
mapput(ARMS, "kmixidx",  [13, "mix",   0, "SIDX", 0]);
}
cfg = mapget(ARMS, ARM);
armid = cfg[1]; kind = cfg[2]; tw = cfg[3]; setname = cfg[4]; amode = cfg[5];
b = CELLB; inst = CELLI;
rec0 = mapget(CURVES, [b, inst]);
setctx(rec0, tw);
if(setname == "SA", mults = SA, setname == "SB", mults = SB, setname == "SC", mults = SC, mults = 0);
{
if(setname == "SIDX",
  my(fa = factor(NN - 1)[, 1], g = 0);
  for(i = 1, #fa, if(fa[i] % 2 == 1, g = fa[i]; break));
  SIDXG = g;
  mults = vector(6, i, lift(Mod([2, 3, 5, 7, 11, 13][i], NN)^g)));
}
wk = if(kind == "a0", "A0", kind == "a1" || kind == "a1c", "A1", "K");
setwalk(mults, wk);
Mn = MARM;
if(kind == "cross", Mn = HI[4]);
BUDGETF = if(kind == "cross", 30, 40);
BUDGET = floor(BUDGETF * sqrt(Mn));
print("hdr(\"gp\", ", version(), ");");
print("hdr(\"cell\", [\"", ARM, "\", ", b, ", ", inst, ", ", TRIALS, ", ", armid, "]);");
print("hdr(\"p_A_d\", [", P_, ", ", A_, ", ", D_, "]);");
print("hdr(\"n\", ", NN, ");  hdr(\"n_other\", ", if(tw, N0, N1), ");");
print("hdr(\"mults\", ", MULT, ");");
print("hdr(\"Hinfo\", ", [HI[1], HI[2], HI[3], HI[4], HI[5]], ");  \\\\ [G, |H|, -1inH, N, index of +-H]");
print("hdr(\"Marm\", ", Mn, ");  hdr(\"dpbits\", ", DPB, ");  hdr(\"cap\", ", CAP, ");  hdr(\"budget\", ", BUDGET, ");");
if(setname == "SIDX", print("hdr(\"sidx_g\", ", SIDXG, ");"));
if(setname == "SIDX" && HI[4] < 64, print("hdr(\"skipped_degenerate_N\", ", HI[4], ");"); quit);
\\ a list (paired across arms)
alsd = 20000000 + 100000*b + 10000*inst;
setrand(alsd);
amodeuse = amode;
if(kind == "cross", amodeuse = 0);
ALIST = alist(TRIALS, NN, amodeuse, HI);
print("hdr(\"seeds\", [", alsd, ", ", 30000000 + 100000*b + 10000*inst, ", ", 10000000 + 100000*b + 10000*inst + 100*armid, "]);");
\\ additive step table
setrand(30000000 + 100000*b + 10000*inst);
WU = vector(16, j, random(NN)); WV = vector(16, j, random(NN));
\\ cross set-up: P on E (E0), Q' on E_d (E1)
setrand(10000000 + 100000*b + 10000*inst + 100*armid);
T0 = getwalltime();
{
for(t = 1, TRIALS,
  my(a = ALIST[t], r, Qp, np);
  if(kind == "pure", r = trial_kpure(a, BUDGET, 0),
     kind == "mix",  r = trial_kmix(a, BUDGET),
     kind == "a0",   r = trial_add(a, BUDGET, 0),
     kind == "a1",   r = trial_add(a, BUDGET, 1),
     kind == "a1c",  r = trial_add_c(a, BUDGET, 1),
     kind == "cross",
       \\ P on E, Q' = [a mod n'] P' on E_d
       np = N1;
       Qp = ellmul(E1, P1pt, 2 + (a % (np - 3)));
       SRCS = [[E0, P0pt, ONE], [E1, Qp, Mod(D_, P_)]];
       r = trial_kpure(a, BUDGET, 1));
  print("rec([\"", ARM, "\", ", b, ", ", inst, ", ", t, ", ", a, ", ", r[1], ", ", r[2], ", ", r[3], ", ", r[4], ", ", r[5], ", ", r[6], ", ", r[7], ", ",
        r[8], ", ", r[9], ", ", r[10], ", ", r[11], ", ", r[12], ", ", r[13], ", ", r[14], ", ", r[15], "]);");
);
}
print("hdr(\"done_ms\", ", getwalltime() - T0, ");");
