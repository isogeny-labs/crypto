#include <flint/fmpz.h>
#include <flint/fmpz_factor.h>
#include <gmpxx.h>
#include <iostream>
#include <map>
#include <stdexcept>
#include <vector>

// Untrusted mathematical proof-input construction, not a primality verdict.
struct Candidate {mpz_class base;std::vector<std::pair<mpz_class,unsigned long>> factors;};
std::map<mpz_class,Candidate> candidates;
mpz_class powmod(const mpz_class& a,const mpz_class& e,const mpz_class& n) {
  mpz_class r;mpz_powm(r.get_mpz_t(),a.get_mpz_t(),e.get_mpz_t(),n.get_mpz_t());return r;
}
void construct(const mpz_class& n) {
  if(candidates.count(n))return;
  if(n<2)throw std::runtime_error("candidate below two");
  Candidate c;const mpz_class predecessor=n-1;
  fmpz_t z;fmpz_init(z);fmpz_set_mpz(z,predecessor.get_mpz_t());
  fmpz_factor_t fac;fmpz_factor_init(fac);
  std::cerr<<"factoring predecessor of "<<n<<std::endl;
  fmpz_factor(fac,z);mpz_class product=1;
  for(slong i=0;i<fac->num;i++) {
    mpz_class q;fmpz_get_mpz(q.get_mpz_t(),fac->p+i);
    c.factors.emplace_back(q,fac->exp[i]);
    for(unsigned long j=0;j<fac->exp[i];j++)product*=q;
  }
  fmpz_factor_clear(fac);fmpz_clear(z);
  if(product!=predecessor)throw std::runtime_error("incorrect factor product");
  bool found=false;
  for(unsigned long base=1;base<=10000&&!found;base++) {
    c.base=base;found=powmod(c.base,predecessor,n)==1;
    for(const auto& [q,e]:c.factors) {
      (void)e;if(powmod(c.base,predecessor/q,n)==1)found=false;
    }
  }
  if(!found)throw std::runtime_error("no witness within constructor bound");
  for(const auto& [q,e]:c.factors){(void)e;construct(q);}
  candidates.emplace(n,c);
}
int main() {
  construct(mpz_class("730750818665451459112596905638433048232067471723"));
  bool first=true;std::cout<<"[\n";
  for(const auto& [n,c]:candidates) {
    if(!first)std::cout<<",\n";
    first=false;
    std::cout<<"{\"n\":\""<<n<<"\",\"base\":\""<<c.base<<"\",\"factors\":[";
    bool firstFactor=true;
    for(const auto& [q,e]:c.factors) {
      if(!firstFactor)std::cout<<",";
      firstFactor=false;
      std::cout<<"{\"q\":\""<<q<<"\",\"exponent\":"<<e<<"}";
    }
    std::cout<<"]}";
  }
  std::cout<<"\n]\n";
}
