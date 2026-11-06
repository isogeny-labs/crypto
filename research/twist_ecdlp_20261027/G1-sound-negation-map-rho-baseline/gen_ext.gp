default(parisizemax, 2000000000);
read("lib.gp");
print("\\\\ gen_ext.gp  gp ", version(), "  extension instances 3,4 at b = 24..40, same gen_inst and seed rule as E5 (setrand(1234567 + 1000*b + inst))");
{
for(b = 24, 40, if(b % 4 == 0,
  for(inst = 3, 4,
    my(sd = 1234567 + 1000*b + inst, v);
    setrand(sd);
    v = gen_inst(b, inst);
    print("\\\\ EXT b=", b, " inst=", inst, " seed=", sd, " n=", v[6], " n'=", v[8], " factor(n-1)=", factor(v[6]-1));
    print("curve(", v, ");"))));
}
