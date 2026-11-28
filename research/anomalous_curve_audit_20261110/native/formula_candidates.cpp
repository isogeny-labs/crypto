#include <gmpxx.h>
#include <iostream>
#include <stdexcept>

// Untrusted exact point witnesses for subsequent kernel checking.
const mpz_class p("730750818665451459112596905638433048232067471723");
const mpz_class a("425706413842211054102700238164133538302169176474");
const mpz_class b("203362936548826936673264444982866339953265530166");
mpz_class mod(mpz_class x) {x%=p;if(x<0)x+=p;return x;}
mpz_class inverse(const mpz_class& x) {
  mpz_class z;if(!mpz_invert(z.get_mpz_t(),x.get_mpz_t(),p.get_mpz_t()))
    throw std::runtime_error("nonunit");
  return z;
}
struct Point {bool infinity;mpz_class x,y;};
Point add(const Point& u,const Point& v) {
  if(u.infinity)return v;
  if(v.infinity)return u;
  if(u.x==v.x&&mod(u.y+v.y)==0)return {true,0,0};
  const mpz_class slope=u.x==v.x?
    mod((3*u.x*u.x+a)*inverse(mod(2*u.y))):
    mod((v.y-u.y)*inverse(mod(v.x-u.x)));
  const mpz_class x=mod(slope*slope-u.x-v.x);
  return {false,x,mod(slope*(u.x-x)-u.y)};
}
Point multiply(mpz_class n,Point u) {
  Point acc{true,0,0};
  while(n>0) {
    if(mpz_odd_p(n.get_mpz_t()))acc=add(acc,u);
    n/=2;if(n>0)u=add(u,u);
  }
  return acc;
}
void print(const char* name,const Point& u) {
  std::cout<<name<<"={\"infinity\":"<<(u.infinity?"true":"false")
    <<",\"x\":\""<<u.x<<"\",\"y\":\""<<u.y<<"\"}\n";
}
mpz_class powmod(const mpz_class& base,const mpz_class& exponent) {
  mpz_class z;mpz_powm(z.get_mpz_t(),base.get_mpz_t(),exponent.get_mpz_t(),p.get_mpz_t());return z;
}
int main() {
  Point u{true,0,0};
  for(int x=0;x<1000;x++) {
    const mpz_class xx=x,rhs=mod(xx*xx*xx+a*xx+b),y=powmod(rhs,(p+1)/4);
    if(mod(y*y)==rhs){u={false,xx,y};break;}
  }
  if(u.infinity)throw std::runtime_error("no point within constructor bound");
  print("formula_point",u);
  print("p_multiple",multiply(p,u));
  print("known_answer_Q",multiply(mpz_class("210393287966660756596132643172172640405085536179"),u));
}
