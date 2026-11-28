#include <gmpxx.h>
#include <iostream>
#include <stdexcept>
#include <vector>
using Poly=std::vector<long>;
using Matrix=std::vector<std::vector<mpz_class>>;
static Poly mul(const Poly&a,const Poly&b) {
  const auto n=a.size(); Poly c(n);
  for(std::size_t i=0;i<n;++i) for(std::size_t j=0;j<n;++j)
    c[(i+j)%n]+=a[i]*b[j];
  return c;
}
static long tr(const Poly&a) {
  long v=static_cast<long>(a.size())*a[0];
  for(auto x:a) v-=x;
  if(v%2) throw std::runtime_error("nonintegral real trace");
  return v/2;
}
static mpz_class det(Matrix m) {
  const auto n=m.size(); mpz_class prev=1;
  for(std::size_t k=0;k+1<n;++k) {
    const mpz_class pivot=m[k][k];
    if(pivot==0) throw std::runtime_error("zero positive-definite pivot");
    for(std::size_t i=k+1;i<n;++i) {
      for(std::size_t j=k+1;j<n;++j) {
        mpz_class v=m[i][j]*pivot-m[i][k]*m[k][j];
        if(!mpz_divisible_p(v.get_mpz_t(),prev.get_mpz_t()))
          throw std::runtime_error("nonexact Bareiss division");
        m[i][j]=v/prev;
      }
      m[i][k]=0;
    }
    prev=pivot;
  }
  return m[n-1][n-1];
}
static void check(int n) {
  const int h=(n-1)/2;
  std::vector<Poly> e(h,Poly(n)); e[0][0]=1;
  for(int j=1;j<h;++j) e[j][j]=e[j][n-j]=1;
  Poly minus_s(n),u2(n); minus_s[1]=minus_s[n-1]=-1;
  u2[0]=8; u2[2]=-3; u2[n-2]=-3;
  Matrix m(2*h,std::vector<mpz_class>(2*h));
  for(int i=0;i<h;++i) for(int j=0;j<h;++j) {
    auto p=mul(e[i],e[j]);
    m[i][j]=2*tr(p);
    m[i][j+h]=m[i+h][j]=tr(mul(minus_s,p));
    m[i+h][j+h]=tr(mul(u2,p));
  }
  mpz_class expected,a,b;
  mpz_ui_pow_ui(a.get_mpz_t(),7,h);
  mpz_ui_pow_ui(b.get_mpz_t(),n,n-2); expected=a*b;
  auto actual=det(m);
  if(actual!=expected) throw std::runtime_error("discriminant mismatch");
  std::cout<<"n="<<n<<"; dimension="<<2*h<<"; gram_discriminant="<<actual
           <<"; exact_divisions=pass; expected_discriminant=pass\n";
}
int main() {
  try { for(int n:{3,5,11,131}) check(n); }
  catch(const std::exception&e) { std::cerr<<e.what()<<'\n'; return 1; }
}
