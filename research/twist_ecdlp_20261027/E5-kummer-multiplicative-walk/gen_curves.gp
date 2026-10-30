default(parisizemax, 2000000000);
read("lib.gp");
print("\\\\ gen_curves.gp  gp ", version(), "  (setrand(1234567 + 1000*b + inst))");
print("\\\\ record: curve([b, inst, p, A, d, n, h, np, hp, xP, yP, XPp, YPp])");
{
for(bi = 1, 6,
  my(b = 20 + 4*(bi-1));
  for(inst = 1, 2,
    my(sd = 1234567 + 1000*b + inst, v, p, n, np);
    setrand(sd);
    v = gen_inst(b, inst);
    p = v[3]; n = v[6]; np = v[8];
    print("\\\\ b=", b, " inst=", inst, " seed=", sd, " p=", p, " bits(p)=", #binary(p), " n=", n, " bits(n)=", #binary(n), " h=", v[7],
          " n'=", np, " bits(n')=", #binary(np), " h'=", v[9], " d=", v[5]);
    print("\\\\   factor(n-1)=", factor(n-1), "  factor(n'-1)=", factor(np-1));
    print("curve(", v, ");")));
}
\\ Supplementary instance for the proper-H cells (pc1/nc1/kmixidx): b = 24 instance 2 has n-1 = 4*q, q prime, so the
\\ S_idx subgroup is degenerate (N = 2).  The first inst >= 3 at b = 24 whose smallest odd prime divisor g of n-1 is <= 13 is added.
{
my(found = 0, inst = 3);
while(!found && inst < 60,
  my(sd = 1234567 + 1000*24 + inst, v, n, fa, g = 0);
  setrand(sd);
  v = gen_inst(24, inst);
  n = v[6]; fa = factor(n - 1)[, 1];
  for(i = 1, #fa, if(fa[i] % 2 == 1, g = fa[i]; break));
  if(g > 0 && g <= 13,
    found = 1;
    print("\\\\ SUPPLEMENTARY_INST=", inst, " (b=24, seed=", sd, ", n=", n, ", smallest odd prime of n-1 = ", g, ", factor(n-1)=", factor(n-1), ")");
    print("curve(", v, ");"));
  inst++);
}

