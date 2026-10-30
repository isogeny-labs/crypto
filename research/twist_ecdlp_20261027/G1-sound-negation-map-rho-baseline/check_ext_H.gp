default(parisizemax, 2000000000);
read("lib.gp"); read("raw/curves.txt");
{
foreach([24,28,32,36,40], b, foreach([1,2,3,4],  i,
  my(r = mapget(CURVES, [b,i]), hi = Hinfo(r[6], SB));
  print("b=", b, " inst=", i, " n=", r[6], " index(+-H in (Z/n)^*)=", hi[5], " N=", hi[4], " twist_ok n'=", r[8])));
}
