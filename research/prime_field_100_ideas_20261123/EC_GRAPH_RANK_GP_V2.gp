\\ Parser repair of EC_GRAPH_RANK_GP.gp; same EC instance and assertions.
\\ V1 and its raw failed output are retained unchanged.
p=1009;
r=1013;
E=ellinit([Mod(1,p),Mod(14,p)]);
P=[Mod(0,p),Mod(425,p)];
if(ellcard(E)!=r,error("unexpected group cardinality"));
if(!isprime(r),error("subgroup order not prime"));
if(P[2]^2!=P[1]^3+P[1]+Mod(14,p),error("P not on curve"));

F=[P,ellmul(E,P,2),ellmul(E,P,3)];
A=[1,1,0;0,1,1;1,0,1];
B=[1,1,0;0,1,1;1,0,-1];
rhsA=[3,5,4];
rhsB=[3,5,-2];

relation_lhs(C,i)={
  my(T=ellmul(E,F[1],C[i,1]));
  for(j=2,3,T=elladd(E,T,ellmul(E,F[j],C[i,j])));
  T;
};

verify_all()={
  for(i=1,3,if(relation_lhs(A,i)!=ellmul(E,P,rhsA[i]),error("A row witness failed")));
  for(i=1,3,if(relation_lhs(B,i)!=ellmul(E,P,rhsB[i]),error("B row witness failed")));
  for(i=1,3,for(j=1,3,if((A[i,j]!=0)!=(B[i,j]!=0),error("support mismatch"))));
  if(matdet(A)!=2,error("unexpected A determinant"));
  if(matdet(B)!=0,error("unexpected B determinant"));
  \\ The first two rows of B have a 2x2 minor of 1, so rank(B)=2.
  if(B[1,1]*B[2,2]-B[1,2]*B[2,1]!=1,error("B rank minor failed"));
};
verify_all();

print("curve=y^2=x^3+x+14 p=",p," group_order=",r);
print("factor_base=",F);
print("support_equal=1 graph_vertices=3 graph_edges=3 graph_cycle_rank=1");
print("A_rows_verified=3 B_rows_verified=3");
print("det_A=2 rank_A_mod_1013=3 det_B=0 rank_B_mod_1013=2");
print("A_rhs=",rhsA," B_rhs=",rhsB);
