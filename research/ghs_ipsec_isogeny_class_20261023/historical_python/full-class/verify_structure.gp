\\ Exact arithmetic certificate of a subgroup of size H.
\\ Unlike quadclassunit's completeness claim, these checks do not assume GRH.
\\ All factor primalities, form discriminants, and element orders are checked.
D=-10552365537501154524618819851856659078153017463;
H=74173379224089213153040;
M=H/2;
Id=Qfb(1,1,(1-D)/4);
g=Qfb(23629414465372871452881,-6581837293224020957647,112102715613021321449878);
u=Qfb(19602239115122732202289,19602239115122732202289,139481684357556702809814);
assert_eq(a,b,msg) = if(a!=b,error(msg));
order_cert(f,n) = {
  my(F=factor(n),c=Vec(f));
  assert_eq(c[2]^2-4*c[1]*c[3],D,"wrong form discriminant");
  assert_eq(gcd(c[1],gcd(c[2],c[3])),1,"form not primitive");
  if(c[1]<=0 || c[3]<=0,error("form not positive definite"));
  assert_eq(qfbpow(f,n),Id,"claimed power not principal");
  assert_eq(qfbnupow(f,n),Id,"NUCOMP power not principal");
  for(i=1,matsize(F)[1],
    assert_eq(isprime(F[i,1]),1,"factor not prime");
    if(qfbpow(f,n/F[i,1])==Id,error("claimed order too large"));
    assert_eq(qfbpow(f,n/F[i,1]),qfbnupow(f,n/F[i,1]),"power methods disagree");
  );
  return(F);
};
ord_known(f,n) = {
  my(o=n,F=factor(n));
  assert_eq(qfbnupow(f,n),Id,"order does not divide supplied multiple");
  for(i=1,matsize(F)[1],
    while(o%F[i,1]==0 && qfbnupow(f,o/F[i,1])==Id,o/=F[i,1]);
  );
  return(o);
};
print("DISCRIMINANT_PRIME_FACTORS=",[1362337,395147821952353991,19602239115122732202289]);
assert_eq(isprime(1362337)*isprime(395147821952353991)*isprime(19602239115122732202289),1,"D factor primality");
assert_eq(-1362337*395147821952353991*19602239115122732202289,D,"D factor product");
print("ORDER_G_FACTORS=",order_cert(g,M));
print("ORDER_U_FACTORS=",order_cert(u,2));
if(u==Id || u==qfbpow(g,M/2),error("second generator is not independent"));
assert_eq(qfbcomp(g,u),qfbcomp(u,g),"group not commutative");
print("UNCONDITIONAL_SUBGROUP_ORDER=",H);
print("UNCONDITIONAL_LOWER_BOUND_VERIFIED=1");
print("G1=",g);
print("G2=",u);
print("G1_HALFWAY_INVOLUTION=",qfbpow(g,M/2));
print("FROBENIUS_CLASS_ORDER=",ord_known(qfbprimeform(D,2),H));
for(i=1,5,ell=[2,3,7,17,31][i]; print("PRIME_IDEAL_ORDER_",ell,"=",ord_known(qfbprimeform(D,ell),H)));
print("GENUS_CHARACTERS_3=",vector(3,i,kronecker([1362337,-395147821952353991,19602239115122732202289][i],3)));
print("GENUS_CHARACTERS_7=",vector(3,i,kronecker([1362337,-395147821952353991,19602239115122732202289][i],7)));
print("GENUS_CHARACTERS_17=",vector(3,i,kronecker([1362337,-395147821952353991,19602239115122732202289][i],17)));
print("GENUS_CHARACTERS_31=",vector(3,i,kronecker([1362337,-395147821952353991,19602239115122732202289][i],31)));
print("ALL_CERTIFICATE_CHECKS_PASSED=1");
quit;
