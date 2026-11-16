default(parisizemax, 2000000000);
read("lib.gp");
read("g1_lib.gp");
read("raw/curves.txt");
\\ env: ARM (n1 | n1r64 | n1sab | n1c2), B, INST, TLO, THI, TOTAL (a-list length; trials TLO..THI are run)
ARM = getenv("ARM"); CELLB = eval(getenv("B")); CELLI = eval(getenv("INST"));
TLO = eval(getenv("TLO")); THI = eval(getenv("THI")); TOTAL = eval(getenv("TOTAL"));
{
ARMS = Map();
mapput(ARMS, "n1",    [21, 32, 0]);
mapput(ARMS, "n1r64", [22, 64, 0]);
mapput(ARMS, "n1sab", [23, 32, 1]);
mapput(ARMS, "n1c2",  [24, 32, 2]);
}
cfg = mapget(ARMS, ARM); armid = cfg[1]; R2 = cfg[2]; MODE = cfg[3];
b = CELLB; inst = CELLI;
rec0 = mapget(CURVES, [b, inst]);
setctx(rec0, 0);
setwalk(SB, "A1");   \\ MARM = n \ 2, DPB, CAP as E5 A1 (SB only feeds Hinfo, unused here)
Mn = MARM; BUDGET = floor(40 * sqrt(Mn));
print("hdr(\"gp\", ", version(), ");");
print("hdr(\"cell\", [\"", ARM, "\", ", b, ", ", inst, ", ", THI - TLO + 1, ", ", armid, "]);");
print("hdr(\"chunk\", [", TLO, ", ", THI, ", ", TOTAL, "]);");
print("hdr(\"p_A_d\", [", P_, ", ", A_, ", ", D_, "]);");
print("hdr(\"n\", ", NN, ");  hdr(\"n_other\", ", N1, ");");
print("hdr(\"r\", ", R2, ");  hdr(\"mode\", ", MODE, ");");
print("hdr(\"Marm\", ", Mn, ");  hdr(\"dpbits\", ", DPB, ");  hdr(\"cap\", ", CAP, ");  hdr(\"budget\", ", BUDGET, ");");
alsd = 20000000 + 100000*b + 10000*inst;     \\ same a-list seed rule as E5 (paired across arms)
setrand(alsd);
ALIST = alist(TOTAL, NN, 0, HI);
tsd = 40000000 + 100000*b + 10000*inst + 100*armid + TLO;
print("hdr(\"seeds\", [", alsd, ", ", 30000000 + 100000*b + 10000*inst, ", ", tsd, "]);");
setrand(30000000 + 100000*b + 10000*inst);   \\ E5 table seed; first 16 entries coincide with E5 A0/A1 tables
WU = vector(R2, j, random(NN)); WV = vector(R2, j, random(NN));
setrand(tsd);
T0 = getwalltime();
{
for(t = TLO, THI,
  my(a = ALIST[t], r);
  r = trial_n1(a, BUDGET);
  print("rec([\"", ARM, "\", ", b, ", ", inst, ", ", t, ", ", a, ", ", r[1], ", ", r[2], ", ", r[3], ", ", r[4], ", ", r[5], ", ", r[6], ", ", r[7], ", ",
        r[8], ", ", r[9], ", ", r[10], ", ", r[11], ", ", r[12], ", ", r[13], ", ", r[14], ", ", r[15], "]);");
);
}
print("hdr(\"done_ms\", ", getwalltime() - T0, ");");
