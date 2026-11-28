\\ Frozen nontrivial supersingular 2-isogeny coordinate control, proposal 16.
p=1019; E=ellinit([Mod(1,p),Mod(0,p)]); K=[Mod(0,p),Mod(0,p)];
if(ellcard(E)!=p+1 || ellorder(E,K)!=2,error("supersingular and kernel guard"));
iso=ellisogeny(E,K); E2=ellinit(iso[1]); f=iso[2][1]; g=iso[2][2]; h=iso[2][3];
if(ellcard(E2)!=p+1,error("codomain cardinality guard"));
mappt(T)={my(u,v,d); if(T==[0],return([0])); u=subst(f,x,T[1]);v=subst(subst(g,x,T[1]),y,T[2]);d=subst(h,x,T[1]);if(d==0,error("map denominator"));[u/d^2,v/d^3]};
choosept()={my(T,q,Q,R);for(xx=1,p-1,q=Mod(xx^3+xx,p);if(issquare(q),T=[Mod(xx,p),sqrt(q)];if(ellorder(E,T)>12,Q=ellmul(E,T,2);R=ellmul(E,T,3);if(Q!=[0] && R!=[0] && T!=K && Q!=K && R!=K,return(T)))));error("point selection guard")};
P=choosept();Q=ellmul(E,P,2);R=elladd(E,P,Q);A=mappt(P);B=mappt(Q);C=mappt(R);
if(!ellisoncurve(E,P) || !ellisoncurve(E2,A) || !ellisoncurve(E2,B) || !ellisoncurve(E2,C),error("point on curve guard"));
if(elladd(E2,A,B)!=C,error("isogeny homomorphism guard"));
if(mappt(elladd(E,P,K))!=A,error("kernel quotient guard"));
print("lead16 curve_p=",p," source_card=",ellcard(E)," codomain_card=",ellcard(E2)," kernel=",K," point_order=",ellorder(E,P));
print("lead16 source_P=",P," source_2P=",Q," source_3P=",R);
print("lead16 image_P=",A," image_2P=",B," image_3P=",C," additive_identity=true kernel_coset_identity=true");
print("lead16 image_coordinate_product=",A[1]*B[1]," image_sum_coordinate=",C[1]," product_equal=",A[1]*B[1]==C[1]);
print("complete lead16 nontrivial isogeny control");
