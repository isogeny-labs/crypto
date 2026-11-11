default(parisizemax,1000000000);
read("research/twist_ecdlp_followup_20261107/L3-genus2-measured-arm/jac.gp"); read("research/twist_ecdlp_followup_20261107/L3-genus2-measured-arm/glue.gp"); read("research/twist_ecdlp_followup_20261107/L3-genus2-measured-arm/ic.gp"); read("research/twist_ecdlp_followup_20261107/L3-genus2-measured-arm/rho.gp");
Q=251; setrand(17); rejinit();
RHOCHECK(xp) = everify(RG,RH,xp);
test() =
{
  my(got=0, A, B, cv, cl, G, H, x, TG, TH, res);
  while(got<3, A=random(Q);B=random(Q); if(A==0||B==0||(4*A^3+27*B^2)%Q==0,next); cv=build_pair(A,B); if(cv==0,next);
    got++; setcurve(cv); cl=cv[5];
    G=emul(cv[6],erandpt()); x=1+random(cl-1); H=emul(x,G); TG=tfer(G); TH=tfer(H);
    print("curve l=",cl," x=",x);
    RG=G; RH=H;
    for(s=1,5, setrand(s); ezero(); res=rhorun(EGR,G,H,cl,0); print("  RE ",res," eops=",eops," sqrt(l/2)=",sqrt(cl/2.)));
    for(s=1,3, setrand(s); jzero(); res=rhorun(JGR,TG,TH,cl,0); print("  RJ ",res," jops=",jops," nfb=",nfb));
  );
}
test();
quit
