\\ Independent normalized dual-composition control on all33 original points.
must(v,msg)=if(!v,error(msg));
{
 iferr(
 my(data=read("dual-audit.gpvec"),cv=data[1],main=data[2],dual=data[4],pr=cv[1],degree=cv[4],E,D,expected,scaled);
 must(dual[5]==degree&&dual[6]==pr&&dual[4]==(degree-1)/2,"dual exact parameters/coverage");
 must(#dual[3]==33&&dual[2]==main[3],"dual input identity/coverage");
 E=ellinit([0,0,0,cv[7],cv[8]],pr);
 must(dual[1]==[lift(Mod(degree^4*cv[7],pr)),lift(Mod(degree^6*cv[8],pr))],"normalized double quotient");
 D=ellinit(concat([0,0,0],dual[1]),pr);
 for(jj=1,33,
   expected=ellmul(E,cv[14][jj],degree);
   scaled=[Mod(degree^2,pr)*expected[1],Mod(degree^3,pr)*expected[2]];
   must(ellisoncurve(D,dual[3][jj])&&scaled==dual[3][jj],"normalized dual composition"));
 must(gcd(degree,cv[11])==1,"invertible recovery multiplier");
 print("PASS33 normalized dual compositions; inverse scaling and ell inverse recover original subgroup; no ECDLP timing claim");
 ,failure,print(failure);quit(2));
}
quit(0);
