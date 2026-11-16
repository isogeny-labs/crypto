default(parisizemax,1000000000);
read("research/twist_ecdlp_followup_20261107/L3-genus2-measured-arm/jac.gp"); read("research/twist_ecdlp_followup_20261107/L3-genus2-measured-arm/glue.gp"); read("research/twist_ecdlp_followup_20261107/L3-genus2-measured-arm/ic.gp"); read("research/twist_ecdlp_followup_20261107/L3-genus2-measured-arm/rho.gp");
Q=251; setrand(17); rejinit();
test() =
{
  my(got=0, A, B, cv, cl, G, TG, P, TP);
  while(got<2, A=random(Q);B=random(Q); if(A==0||B==0||(4*A^3+27*B^2)%Q==0,next); cv=build_pair(A,B); if(cv==0,next);
    got++; setcurve(cv); cl=cv[5];
    G=emul(cv[6],erandpt()); TG=tfer(G);
    if(got<2,next);
    print("l=",cl, " CV=", cv);
    for(k=1,8, P=emul(k,G); TP=tfer(P); print(k," ",P," ",TP, " J: ", jmul(k,TG)))
  );
}
test();
quit
