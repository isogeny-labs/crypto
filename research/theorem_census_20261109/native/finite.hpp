#pragma once
#include <algorithm>
#include <cassert>
#include <cstdint>
#include <stdexcept>
#include <vector>

// Polynomial-basis GF(p^n). Encodings are base-p digits, not integer residues.
struct Field {
 int p,n,q,modulus;
 std::vector<int> plus,times,negative,inverse;
 Field(int pp,int nn,int mm):p(pp),n(nn),q(1),modulus(mm){
  for(int i=0;i<n;i++)q*=p;
  plus.resize(q*q);times.resize(q*q);negative.resize(q);inverse.resize(q);
  for(int a=0;a<q;a++)for(int b=0;b<q;b++){
   int aa=a,bb=b,z=0,place=1;
   for(int i=0;i<n;i++){z+=((aa%p+bb%p)%p)*place;place*=p;aa/=p;bb/=p;}
   plus[a*q+b]=z;
   std::vector<int> u(n),v(n),w(2*n-1),mod(n+1);
   aa=a;bb=b;int m=modulus;
   for(int i=0;i<n;i++){u[i]=aa%p;v[i]=bb%p;aa/=p;bb/=p;}
   for(int i=0;i<=n;i++){mod[i]=m%p;m/=p;}
   if(mod[n]!=1)throw std::runtime_error("modulus must be monic");
   for(int i=0;i<n;i++)for(int j=0;j<n;j++)w[i+j]=(w[i+j]+u[i]*v[j])%p;
   for(int i=2*n-2;i>=n;i--)for(int j=0;j<n;j++)w[i-n+j]=(w[i-n+j]-w[i]*mod[j]%p+p)%p;
   z=0;place=1;for(int i=0;i<n;i++){z+=w[i]*place;place*=p;}times[a*q+b]=z;
  }
  for(int a=0;a<q;a++)for(int b=0;b<q;b++){
   if(add(a,b)==0)negative[a]=b;
   if(mul(a,b)==1)inverse[a]=b;
  }
  for(int a=1;a<q;a++)if(!inverse[a])throw std::runtime_error("reducible modulus");
 }
 int add(int a,int b)const{return plus[a*q+b];}
 int mul(int a,int b)const{return times[a*q+b];}
 int neg(int a)const{return negative[a];}
 int sub(int a,int b)const{return add(a,neg(b));}
 int div(int a,int b)const{if(!b)throw std::runtime_error("division by zero");return mul(a,inverse[b]);}
 int c(int k)const{k%=p;return k<0?k+p:k;}
 int power(int a,std::uint64_t e)const{int z=1;for(;e;e>>=1,a=mul(a,a))if(e&1)z=mul(z,a);return z;}
};

struct Point {int x=0,y=0;bool inf=true;bool operator==(const Point& z)const{return inf==z.inf&&(inf||(x==z.x&&y==z.y));}};
struct Curve {
 const Field& f;
 int a1,a2,a3,a4,a6;
 Curve(const Field& ff,int b1,int b2,int b3,int b4,int b6):f(ff),a1(b1),a2(b2),a3(b3),a4(b4),a6(b6){}
 bool on(Point P)const{
  if(P.inf)return true;
  int x=P.x,y=P.y;
  int lhs=f.add(f.add(f.mul(y,y),f.mul(a1,f.mul(x,y))),f.mul(a3,y));
  int rhs=f.add(f.add(f.add(f.power(x,3),f.mul(a2,f.mul(x,x))),f.mul(a4,x)),a6);
  return lhs==rhs;
 }
 Point neg(Point P)const{if(P.inf)return P;return {P.x,f.neg(f.add(P.y,f.add(f.mul(a1,P.x),a3))),false};}
 Point add(Point P,Point Q)const{
  if(P.inf)return Q;
  if(Q.inf)return P;
  if(Q==neg(P))return {};
  int x=P.x,y=P.y,u=Q.x,v=Q.y,s;
  if(x==u){
   int numerator=f.sub(f.add(f.add(f.mul(f.c(3),f.mul(x,x)),f.mul(f.c(2),f.mul(a2,x))),a4),f.mul(a1,y));
   int denominator=f.add(f.add(f.mul(f.c(2),y),f.mul(a1,x)),a3);
   s=f.div(numerator,denominator);
  }else s=f.div(f.sub(v,y),f.sub(u,x));
  int intercept=f.sub(y,f.mul(s,x));
  int X=f.sub(f.sub(f.sub(f.add(f.mul(s,s),f.mul(a1,s)),a2),x),u);
  int Y=f.neg(f.add(f.add(f.mul(f.add(s,a1),X),intercept),a3));
  Point R{X,Y,false};assert(on(R));return R;
 }
 Point mul(std::uint64_t n,Point P)const{Point R{};for(;n;n>>=1,P=add(P,P))if(n&1)R=add(R,P);return R;}
 int discriminant()const{
  auto M=[&](int x,int y){return f.mul(x,y);};auto A=[&](int x,int y){return f.add(x,y);};
  int b2=A(M(a1,a1),M(f.c(4),a2)),b4=A(M(f.c(2),a4),M(a1,a3)),b6=A(M(a3,a3),M(f.c(4),a6));
  int b8=f.sub(A(A(M(M(a1,a1),a6),M(f.c(4),M(a2,a6))),M(a2,M(a3,a3))),A(M(a1,M(a3,a4)),M(a4,a4)));
  return A(A(f.neg(M(M(b2,b2),b8)),f.neg(M(f.c(8),f.power(b4,3)))),A(f.neg(M(f.c(27),M(b6,b6))),M(f.c(9),M(b2,M(b4,b6)))));
 }
 std::vector<Point> points()const{
  std::vector<Point> ps{{}};
  for(int x=0;x<f.q;x++)for(int y=0;y<f.q;y++)if(on({x,y,false}))ps.push_back({x,y,false});
  return ps;
 }
};

inline void field_controls(const Field& f){
 for(int a=0;a<f.q;a++){
  assert(f.add(a,f.neg(a))==0);
  if(a)assert(f.mul(a,f.inverse[a])==1);
  assert(f.power(a,f.q)==a);
 }
 // All triples on the small extension fields, sufficient arithmetic controls
 // for larger fields use a fixed deterministic grid, not claimed exhaustive.
 int bound=std::min(f.q,32);
 for(int a=0;a<bound;a++)for(int b=0;b<bound;b++)for(int c=0;c<bound;c++){
  assert(f.mul(a,f.add(b,c))==f.add(f.mul(a,b),f.mul(a,c)));
  assert(f.mul(a,f.mul(b,c))==f.mul(f.mul(a,b),c));
 }
}
