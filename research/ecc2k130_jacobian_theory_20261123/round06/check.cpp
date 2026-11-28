#include <gmpxx.h>
#include <iostream>
#include <stdexcept>
#include <vector>
using Z=mpz_class;
static void need(bool b,const char*s){if(!b)throw std::runtime_error(s);}
static Z two(unsigned n){Z z=1;return z<<n;}
static unsigned pw(unsigned a,unsigned e,unsigned p){unsigned b=1;while(e){if(e&1)b=b*a%p;a=a*a%p;e>>=1;}return b;}
static Z rem(Z a,const Z&b){while(a!=0&&mpz_sizeinbase(a.get_mpz_t(),2)>=mpz_sizeinbase(b.get_mpz_t(),2)){Z c=b<<(mpz_sizeinbase(a.get_mpz_t(),2)-mpz_sizeinbase(b.get_mpz_t(),2));a^=c;}return a;}
static Z gcd(Z a,Z b){while(b!=0){Z c=rem(a,b);a=b;b=c;}return a;}
static Z sq(const Z&a){Z b=0;for(unsigned i=0;i<mpz_sizeinbase(a.get_mpz_t(),2);++i)if(mpz_tstbit(a.get_mpz_t(),i))mpz_setbit(b.get_mpz_t(),2*i);return b;}
int main(){try{
  need(pw(2,130,131)==1&&pw(2,65,131)!=1&&pw(2,26,131)!=1&&pw(2,10,131)!=1,"order130");
  Z phi=two(131)-1,rhs=two(131)+1,x=2,z=x;
  for(unsigned j=1;j<=130;++j){z=rem(sq(z),phi);if(j==65||j==26||j==10)need(gcd(phi,z^x)==1,"Rabin gcd");}
  need(z==x,"Rabin terminal");
  need(rem(rhs,Z(3))==0,"linear factor");
  // Binary multiplication by 1+T is xor with a one-place shift.
  need((phi^(phi<<1))==rhs,"target reduced numerator");
  std::cout<<"Phi131_irreducible_over_F2=pass; reduced_target_times_(1+T)=1+T^131\n";
  unsigned divs=0;
  for(unsigned d=1;d<=262;++d)if(rem(rhs,two(d)+1)==0){need(d==1||d==131,"unexpected branch binomial");++divs;std::cout<<"possible_single_branch_degree="<<d<<'\n';}
  need(divs==2,"two branch products");
  need(gcd(rhs,two(130))==1,"squarefree target");
  unsigned genusCases=0;
  for(int h=0;h<=131;++h)for(int b:{1,131})for(int c=1;c<=131;++c){
    if(h+129!=2*h-2+b*c)continue; // gX=h+130; c=(ramification_break+1)/2.
    int fbase=(b==1)?130:0;
    if(fbase>h)continue;
    ++genusCases;
    need((h==130&&b==1&&c==1)||(h==0&&b==131&&c==1),"unexpected ramified genus");
    std::cout<<"ramified: base_genus="<<h<<"; cover_genus="<<h+130<<"; base_2rank="<<fbase<<"; branch_degree="<<b<<"; ramification_break="<<2*c-1<<'\n';
  }
  need(genusCases==2,"ramified cases");
  need(2*131-1==131+130,"etale genus");
  std::cout<<"etale: base_genus=131; cover_genus=261; base_2rank=131\n";
  for(unsigned d=1;d<=131;++d){
    // Split and inert Euler factors agree mod2: (1+T^d)^2=1+T^(2d).
    Z a=two(d)+1;need(sq(a)==two(2*d)+1,"local Euler identity");
  }
  std::cout<<"local_Euler_factor_identities=pass for degrees1..131\n";
  need(5-2*3==-1,"two copies A cannot be a full genus260 Jacobian");
  need(5-(3-3)==5,"A*E0 base N2");
  need((5-3)<2*3,"etale A*E0 base cover fiber failure");
  std::cout<<"base_Jacobian_A: proposed_cover_N2=-1; excluded\n";
  std::cout<<"etale_base_Jacobian_A_times_E0: base_N1=3; base_N2=5; proposed_cover_N2=2 <6; excluded\n";
  unsigned pairs=0;
  for(unsigned a=1;a<=131;++a){unsigned b=132-a;need((a-1)+(b-1)==130,"hyperelliptic partition dimensions");if(a<=b){++pairs;if(a==1)std::cout<<"etale_hyperelliptic_base_simple_Prym: partition=1+131 forces_genus130_hyperelliptic_factor\n";}}
  need(pairs==66,"unordered partition sizes");
  std::cout<<"no cover, Prym realization or subgroup map constructed\n";
  return 0;
}catch(const std::exception&e){std::cerr<<e.what()<<'\n';return 1;}}
