// Exact fixed-fixture arithmetic; no field point search or timing harness.
#include <gmpxx.h>
#include <iostream>
#include <stdexcept>
#include <utility>

using Element = std::pair<mpz_class, mpz_class>; // a + b*pi, pi^2=-pi-2
static Element multiply(const Element& x, const Element& y) {
  return {x.first*y.first-2*x.second*y.second,
          x.first*y.second+x.second*y.first-x.second*y.second};
}
static Element power(unsigned n) {
  Element result{1,0}, base{0,1};
  while(n) {
    if(n&1U) result=multiply(result,base);
    base=multiply(base,base);
    n>>=1U;
  }
  return result;
}
static std::pair<mpz_class,mpz_class> recurrence(unsigned n) {
  mpz_class a=0,b=1,t0=2,t1=-1;
  for(unsigned i=2;i<=n;++i) {
    mpz_class c=-b-2*a,t=-t1-2*t0;
    a=b;b=c;t0=t1;t1=t;
  }
  return {b,t1};
}
int main() {
  try {
    for(unsigned n: {4U,7U,571U,7351U,9551U}) {
      const auto value=power(n);
      const auto [u,t]=recurrence(n);
      const mpz_class c=abs(u), q=mpz_class(1)<<n;
      if(value.second!=u || 2*value.first-value.second!=t ||
         t*t-4*q!=-7*c*c) throw std::runtime_error("arithmetic disagreement");
      if((n==4 && c!=3) || (n==7 && c!=7))
        throw std::runtime_error("small fixed value disagreement");
      if(n==571 && c!=mpz_class("35655896511340624687810845487372082980344185179452596885179918179218090432562595510057"))
        throw std::runtime_error("published K-571 disagreement");
      std::cout << "{\"n\":" << n << ",\"trace\":\"" << t
                << "\",\"frobenius_conductor\":\"" << c
                << "\",\"conductor_bits\":" << mpz_sizeinbase(c.get_mpz_t(),2)
                << ",\"ring_recurrence_agree\":true,\"discriminant_identity\":true}\n";
    }
  } catch(const std::exception& e) {std::cerr<<e.what()<<'\n';return 2;}
}
