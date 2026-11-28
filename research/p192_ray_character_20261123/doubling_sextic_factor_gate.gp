\\ Factorization control for the duplication sextic D_2=psi_4/(4y).
\\ Run: gp -q research/p192_ray_character_20261123/doubling_sextic_factor_gate.gp
\\ The report proves the general irreducibility criterion; this checks its inputs.

check_case(label,p,a,b,r,small)=
{
  my(x='x,D,F);
  if(p%4!=3 || r%4!=1,error("mod-4 input mismatch"));
  if(small && ellcard(ellinit([0,0,0,a,b],p))!=r,error("group order mismatch"));
  D=x^6+5*a*x^4+20*b*x^3-5*a^2*x^2-4*a*b*x-a^3-8*b^2;
  F=factor(Mod(1,p)*D);
  if(matsize(F)[1]!=1 || poldegree(lift(F[1,1]))!=6 || F[1,2]!=1,
     error("sextic is reducible"));
  print(label," p_mod4=",p%4," r_mod4=",r%4," sextic_degree=6 irreducible=1");
}

check_case("exact-79",79,0,3,97,1);
check_case("inexact-79",79,1,6,97,1);
check_case("exact-223",223,8,7,241,1);
check_case("inexact-223",223,2,48,241,1);
check_case("inexact-271",271,7,3,241,1);

p=6277101735386680763835789423207666416083908700390324961279;
r=6277101735386680763835789423176059013767194773182842284081;
b=2455155546008943817740293915197451784769108058161191238065;
check_case("P-192",p,-3,b,r,0);
