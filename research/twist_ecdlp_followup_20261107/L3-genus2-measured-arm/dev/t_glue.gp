default(parisizemax,1000000000);
read("research/twist_ecdlp_followup_20261107/L3-genus2-measured-arm/jac.gp"); read("research/twist_ecdlp_followup_20261107/L3-genus2-measured-arm/glue.gp");
Q=251; setrand(7); rejinit();
test() =
{
  my(got=0, tries=0, A, B, cv, nc, bad, badh, E, P1, P2, P3, T1, T2, T3, Gp, G, TG, cl, D1, R);
  while(got<4, A=random(Q);B=random(Q); if(A==0||B==0||(4*A^3+27*B^2)%Q==0,next); tries++; cv=build_pair(A,B); if(cv==0,next);
    got++; setcurve(cv); jzero();
    nc=countCp(); cl=cv[5];
    print("curve A,B=",[A,B]," nE=",cv[3]," nEt=",cv[4]," l=",cl," h=",cv[6]," pat=",cv[7]," pidx=",cv[8]," #C'=",nc," (q+1=",Q+1,")");
    bad=0; for(t=1,8,D1=jrand(); R=jmul(CNJ,D1); if(!jisid(R),bad++)); print(" N*D!=0: ",bad);
    badh=0;
    for(t=1,30, P1=erandpt();P2=erandpt(); P3=eadd(P1,P2); T1=tfer(P1);T2=tfer(P2);T3=tfer(P3);
       if(T1==0||T2==0||T3==0,next);
       if(!jeq(jadd(T1,T2),T3),badh++); if(!jvalid(T1),badh++));
    print(" hom bad=",badh);
    Gp=erandpt(); G=emul(cv[6],Gp); print(" lG=O: ",emul(cl,G)==[0]);
    TG=tfer(G); print(" lT(G)=0: ",jisid(jmul(cl,TG))," T(G)!=0: ",!jisid(TG));
  );
  print(REJ, " tries=",tries);
}
test();
quit
