default(parisizemax, 2000000000);
read("lib.gp");
setrand(1234567+1000*24+3); v=gen_inst(24,3); print("inst3 n=", v[6], " factor(n-1)=", factor(v[6]-1));
