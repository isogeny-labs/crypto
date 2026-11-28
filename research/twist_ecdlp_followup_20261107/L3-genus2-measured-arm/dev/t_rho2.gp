default(parisizemax,1000000000);
read("research/twist_ecdlp_followup_20261107/L3-genus2-measured-arm/jac.gp"); read("research/twist_ecdlp_followup_20261107/L3-genus2-measured-arm/glue.gp"); read("research/twist_ecdlp_followup_20261107/L3-genus2-measured-arm/ic.gp"); read("research/twist_ecdlp_followup_20261107/L3-genus2-measured-arm/rho.gp");
Q=251; setrand(17); rejinit();
test() =
{
  my(got=0, A, B, cv, cl, G, H, x, TG, TH, P, vs, cnt, j, v2);
  while(got<2, A=random(Q);B=random(Q); if(A==0||B==0||(4*A^3+27*B^2)%Q==0,next); cv=build_pair(A,B); if(cv==0,next);
    got++; setcurve(cv); cl=cv[5];
    G=emul(cv[6],erandpt()); x=1+random(cl-1); H=emul(x,G); TG=tfer(G); TH=tfer(H);
    if(got<2,next);
    print("l=",cl);
    vs=List(); P=TG;
    for(i=1,cl-1, listput(vs,P[2]); P=jadd(P,TG));
    vs=Vec(vs);
    print(vs);
    cnt=0; for(i=1,#vs, if(vs[i]%4==0,cnt++));
    print("b%4==0: ",cnt," of ",#vs);
    v2=vector(32); for(i=1,#vs, j=((vs[i]>>2)%32)+1; v2[j]++);
    print(v2)
  );
}
test();
quit
