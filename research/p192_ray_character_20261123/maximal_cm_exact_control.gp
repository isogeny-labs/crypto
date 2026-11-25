\\ Exhaustive marked-sign replay on three maximal-CM prime-order curves.
\\ Run: gp -q research/p192_ray_character_20261123/maximal_cm_exact_control.gp

oddrep(v,r)=if(v%2,v,r-v);

coset_sign(E,G,indices,d,p,r)=
{
  my(z=1,P);
  for(k=1,#indices,
    P=ellmul(E,G,(indices[k]*d)%r);
    if(P==[0],error("zero orbit point"));
    z=lift(Mod(z,p)*P[2]));
  kronecker(z,p)
}

run_case(label,p,a,b,r,g,m,gx,gy,want_mismatches)=
{
  my(E=ellinit([0,0,0,a,b],p),G=[Mod(gx,p),Mod(gy,p)]);
  my(D=(p+1-r)^2-4*p,n=(r-1)/2,t=n/m,h,indices,base,mismatches=0,j);
  if(ellcard(E)!=r || ellmul(E,G,r)!=[0],error("curve or generator order"));
  if(!isfundamental(D),error("nonfundamental CM discriminant"));
  j=lift(Mod(1728*4*a^3,p)/Mod(4*a^3+27*b^2,p));
  if(znorder(Mod(g,r))!=r-1 || t<=1 || t%2!=1,error("scalar group"));
  h=lift(Mod(g,r)^t);
  indices=vector(m,k,oddrep(lift(Mod(h,r)^(k-1)),r));
  if(#Set(vector(m,k,min(indices[k],r-indices[k])))!=m,
     error("repeated signed subgroup class"));
  base=coset_sign(E,G,indices,1,p,r);
  for(d=1,n,
    if(coset_sign(E,G,indices,d,p,r)*base!=kronecker(d,r),mismatches++));
  print(label," p=",p," r=",r," D=",D," fundamental=1 j=",j," m=",m,
        " index=",t," signed_classes=",n," mismatches=",mismatches);
  if(mismatches!=want_mismatches,error("unexpected sign mismatch count"));
}

run_case("exact",107,6,15,97,5,16,2,79,0);
run_case("matched-inexact",107,7,6,97,5,16,1,96,32);
run_case("inexact",271,7,3,241,7,24,1,108,72);
