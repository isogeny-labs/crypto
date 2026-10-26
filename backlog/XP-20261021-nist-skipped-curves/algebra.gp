\\ Independent native PARI checks. No DiSSECT mathematical code is imported.
\\ Auxiliary points below are witnesses, never reconstructed historical G.
check(ok, text) = if(!ok, error(text));
quoted(n) = Str("\"", n, "\"");
jsonvec(v) = Str("[", strjoin(apply(quoted, v), ","), "]");
factorjson(F) = Str("[", strjoin(vector(matsize(F)[1], i, Str("[", quoted(F[i,1]), ",", F[i,2], "]")), ","), "]");

audit() = {
  read(getenv("CURVE_INPUTS"));
  my(records=List(), pairs=0);
  for(i=1, #INPUTS,
    my(c=INPUTS[i], label=c[1], p=c[2], a=c[3], b=c[4], n=c[5]);
    check(isprime(p) && isprime(n), "field/order primality");
    my(E=ellinit([a,b],p), t=p+1-n, D=t^2-4*p);
    check(E.disc != 0 && t%p != 0 && t^2 <= 4*p, "ordinary nonsingular Hasse-bounded curve");
    check(ellsea(E)==n, "independent SEA order differs from input");
    my(d=2); while(kronecker(d,p)!=-1,d++);
    my(T=ellinit([a*d^2,b*d^3],p), nt=2*(p+1)-n);
    check(ellsea(T)==nt, "true quadratic twist count");
    check(ellsea(ellinit([a,-b],p))==if(p%4==1,n,nt), "negative-b sign relation");
    my(Ft=alarm(120,factor(nt)));
    check(prod(k=1,matsize(Ft)[1],isprime(Ft[k,1])) && factorback(Ft)==nt, "twist factor certificate");
    my(FD=alarm(120,factor(-D)), DK=quaddisc(D), f=sqrtint(D/DK));
    check(prod(k=1,matsize(FD)[1],isprime(FD[k,1])) && factorback(FD)==-D && D==f^2*DK, "CM discriminant certificate");
    my(Fn=alarm(120,factor(n-1)), emb=znorder(Mod(p,n),[n-1,Fn]));
    check(prod(k=1,matsize(Fn)[1],isprime(Fn[k,1])) && factorback(Fn)==n-1, "embedding factor certificate");
    my(xg=0, rhs);
    while(1, rhs=Mod(xg^3+a*xg+b,p); if(issquare(rhs),break); xg++);
    my(yg=lift(sqrt(rhs))); yg=min(yg,p-yg);
    my(G=[Mod(xg,p),Mod(yg,p)]);
    check(ellisoncurve(E,G) && ellmul(E,G,n)==[0], "auxiliary point order");
    my(F3=factor(elldivpol(E,3)), kernel=0, roots3=0);
    for(k=1,matsize(F3)[1], if(poldegree(F3[k,1])==1, roots3++; kernel=F3[k,1]));
    check(kernel!=0 && p%3==1 && t%3==1, "declared repeated-eigenvalue 3-torsion fixtures");
    my(full3=if(roots3==4,2,6));
    my(v0=2, v1=t, v2);
    for(k=2,full3, v2=t*v1-p*v0; v0=v1; v1=v2);
    check((p^full3+1-v1)%9==0, "full 3-torsion necessary cardinality condition");
    my(iso=ellisogeny(E,kernel), C=ellinit(iso[1],p), Q=ellisogenyapply(iso[2],G));
    check(ellsea(C)==n && Q!=[0] && ellisoncurve(C,Q) && ellmul(C,Q,n)==[0], "explicit 3-isogeny and subgroup transport");
    my(r=-polcoef(kernel,0)/polcoef(kernel,1), H='x-r, u=6*r^2+2*a, v=4*(r^3+a*r+b));
    my(F='x*H^2+u*H+v, L=H^3-u*H-2*v);
    check(('x^3+a*'x+b)*L^2-F^3-C.a4*F*H^4-C.a6*H^6==0, "normalized Velu rational-map identity");
    my(primes=[2,3,5,7,11,13,17,19,23,29,31], neighbors=List());
    foreach(primes,l, listput(neighbors, Str("[",l,",",jsonvec(lift(polrootsmod(polmodular(l,0,E.j,'x)))),"]")));
    print("{\"kind\":\"curve\",\"fixture\":\"",label,"\",\"order_verified\":true,\"twist_order_verified\":true,\"isogeny_verified\":true,",
      "\"p\":",quoted(p),",\"order\":",quoted(n),",\"trace\":",quoted(t),",\"trace_degree2\":",quoted(t^2-2*p),",\"j\":",quoted(lift(E.j)),
      ",\"frobenius_discriminant\":",quoted(D),",\"cm_discriminant\":",quoted(DK),",\"frobenius_conductor\":",quoted(f),
      ",\"embedding_degree\":",quoted(emb),",\"twist_order\":",quoted(nt),",\"twist_factors\":",factorjson(Ft),
      ",\"negative_b_relation\":\"",if(p%4==1,"base_field_isomorphic","quadratic_twist"),"\",\"auxiliary_G\":",jsonvec([xg,yg]),
      ",\"torsion3\":{\"least\":2,\"full\":",full3,",\"relative\":",full3/2,"},\"rational_isogeny_neighbors\":[",strjoin(Vec(neighbors),","),"],",
      "\"isogeny3\":{\"kernel_coefficients\":",jsonvec(vector(2,k,lift(polcoef(kernel,k-1)))),",\"codomain_coefficients\":",jsonvec(lift(iso[1])),",\"image_G\":",jsonvec(lift(Q)),"}}");
    listput(records,[label,p,t,D,lift(E.j)]);
  );
  for(i=1,#records, for(j=i+1,#records,
    my(A=records[i], B=records[j]); if(A[2]!=B[2],next);
    pairs++;
    print("{\"kind\":\"pair\",\"a\":\"",A[1],"\",\"b\":\"",B[1],
      "\",\"same_j\":",if(A[5]==B[5],"true","false"),
      ",\"same_base_field_isogeny_class\":",if(A[3]==B[3],"true","false"),
      ",\"traces_equal_up_to_sign\":",if(abs(A[3])==abs(B[3]),"true","false"),
      ",\"same_ordinary_CM_field\":",if(issquare(A[4]*B[4]),"true","false"),"}");
  ));
  print("{\"kind\":\"complete\",\"curves\":",#records,",\"pairs\":",pairs,"}");
};
iferr(audit(),err,print("AUDIT FAILED: ",err);quit(1));
quit(0);
