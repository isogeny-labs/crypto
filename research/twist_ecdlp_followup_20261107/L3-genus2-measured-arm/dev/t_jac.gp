default(parisizemax,1000000000);
read("research/twist_ecdlp_followup_20261107/L3-genus2-measured-arm/jac.gp");
q=10007; setrand(5);
f=0; while(1, f=X^5+random(q)*X^3+random(q)*X^2+random(q)*X+random(q); if(poldegree(gcd(Mod(f,q),Mod(f',q)))==0,break));
jinit(q,polcoeff(f,3),polcoeff(f,2),polcoeff(f,1),polcoeff(f,0));
jzero();
N=subst(hyperellcharpoly(Mod(f,q)),x,1); print("N=",N);
bad=0; ndef=0;
{for(t=1,3000,D1=jrand(); D2=jrand(); r1=jadd(D1,D2); r2=jref(D1,D2); if(r1!=r2,bad++;if(bad<4,print("BAD ",D1," ",D2," ",r1," ",r2))));}
print("bad add=",bad," nfb=",nfb," nadd2=",nadd2," ndbl2=",ndbl2);
bad=0;
for(t=1,2000,D1=jrand(); r1=jdbl(D1); r2=jref(D1,D1); if(r1!=r2,bad++));
print("bad dbl=",bad," ndbl2=",ndbl2);
\\ order check
bad=0; for(t=1,8,D=jrand(); R=jmul(N,D); if(!jisid(R),bad++)); print("N*D!=0 count=",bad);
\\ associativity
bad=0; for(t=1,300,A=jrand();B=jrand();C=jrand(); if(!jeq(jadd(jadd(A,B),C),jadd(A,jadd(B,C))),bad++)); print("assoc bad=",bad);
jzero(); for(t=1,2000,A=jrand());
jzero(); D=jrand(); jzero(); for(t=1,3000,D=jadd(D,D2)); print("walk: jops=",jops," nfb=",nfb," cM/jop=",cM/jops*1.," cS=",cS/jops*1.," cI=",cI/jops*1.);
quit
