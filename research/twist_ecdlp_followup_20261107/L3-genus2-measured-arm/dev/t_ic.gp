default(parisizemax,1000000000);
read("research/twist_ecdlp_followup_20261107/L3-genus2-measured-arm/jac.gp"); read("research/twist_ecdlp_followup_20261107/L3-genus2-measured-arm/glue.gp"); read("research/twist_ecdlp_followup_20261107/L3-genus2-measured-arm/ic.gp");
Q=251; setrand(17); rejinit();
test() =
{
  my(got=0, A, B, cv, cl, G, H, x, TG, TH, res, arm, ok);
  while(got<3, A=random(Q);B=random(Q); if(A==0||B==0||(4*A^3+27*B^2)%Q==0,next); cv=build_pair(A,B); if(cv==0,next);
    got++; setcurve(cv); cl=cv[5];
    G=emul(cv[6],erandpt()); x=1+random(cl-1); H=emul(x,G); TG=tfer(G); TH=tfer(H);
    FBS=1+random(Q-1);
    print("curve l=",cl," x=",x);
    foreach(["G","S","D","F"], arm,
      setrand(100+got); jzero(); res=icrun(TG,TH,cl,arm,0);
      ok = res[1] && everify(G,H,res[2]);
      print(" arm ",arm," solved=",res[1]," xcand=",res[2]," verified=",ok," trials=",res[3]," jops=",res[4]," nsplit=",res[10]," FF/FL/LL=",[res[14],res[15],res[16]]," nsolve=",res[20]," rowops=",res[21]," dup=",res[18]));
  );
}
test();
quit
