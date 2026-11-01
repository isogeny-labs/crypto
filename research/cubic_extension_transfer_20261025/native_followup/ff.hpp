#pragma once
// Native finite-field support. FLINT 3.x; no Python or Sage runtime calls.
#include <flint/fq_nmod.h>
#include <flint/fq_nmod_poly.h>
#include <flint/fq_nmod_poly_factor.h>
#include <flint/nmod_poly.h>
#include <array>
#include <algorithm>
#include <cstdint>
#include <fstream>
#include <iostream>
#include <regex>
#include <stdexcept>
#include <string>
#include <vector>
#include "json.hpp"
using json=nlohmann::json;
inline fq_nmod_ctx_t FC;
inline flint_rand_t FR;
inline uint64_t FP;
inline void require(bool b,const std::string& s){if(!b)throw std::runtime_error(s);}
inline void field_start(uint64_t p,int degree,uint64_t seed=20261025){
 FP=p;fq_nmod_ctx_init_ui(FC,p,degree,"v");flint_randinit(FR);flint_randseed(FR,seed,seed^0x7f3a81UL);
}
inline void field_stop(){flint_randclear(FR);fq_nmod_ctx_clear(FC);}
struct F {
 fq_nmod_t v;
 F(){fq_nmod_init(v,FC);}
 F(long n):F(){fq_nmod_set_si(v,n,FC);}
 F(const F& x):F(){fq_nmod_set(v,x.v,FC);}
 F(F&& x) noexcept:F(){fq_nmod_swap(v,x.v,FC);}
 ~F(){fq_nmod_clear(v,FC);}
 F& operator=(const F& x){if(this!=&x)fq_nmod_set(v,x.v,FC);return *this;}
 F& operator=(F&& x) noexcept{if(this!=&x)fq_nmod_swap(v,x.v,FC);return *this;}
 bool zero()const{return fq_nmod_is_zero(v,FC);}
 explicit operator bool()const{return !zero();}
 F operator-()const{F z;fq_nmod_neg(z.v,v,FC);return z;}
 friend F operator+(const F&a,const F&b){F z;fq_nmod_add(z.v,a.v,b.v,FC);return z;}
 friend F operator-(const F&a,const F&b){F z;fq_nmod_sub(z.v,a.v,b.v,FC);return z;}
 friend F operator*(const F&a,const F&b){F z;fq_nmod_mul(z.v,a.v,b.v,FC);return z;}
 friend F operator/(const F&a,const F&b){require(bool(b),"finite-field division by zero");F z;fq_nmod_inv(z.v,b.v,FC);fq_nmod_mul(z.v,z.v,a.v,FC);return z;}
 friend bool operator==(const F&a,const F&b){return fq_nmod_equal(a.v,b.v,FC);}
 friend bool operator!=(const F&a,const F&b){return !(a==b);}
 F& operator+=(const F&a){fq_nmod_add(v,v,a.v,FC);return *this;}
 F pow(uint64_t e)const{F z;fq_nmod_pow_ui(z.v,v,e,FC);return z;}
 F frob(int e=1)const{F z=*this;for(int i=0;i<e;i++)z=z.pow(FP);return z;}
 F sqrt()const{F z;require(fq_nmod_sqrt(z.v,v,FC),"nonsquare square-root request");return z;}
 bool square()const{return fq_nmod_is_square(v,FC);}
 long constant()const{require(nmod_poly_degree(v)<=0,"not in prime field");return nmod_poly_get_coeff_ui(v,0);}
 std::vector<uint64_t> coeffs()const{std::vector<uint64_t>a(fq_nmod_ctx_degree(FC));for(size_t i=0;i<a.size();i++)a[i]=nmod_poly_get_coeff_ui(v,i);return a;}
 static F random(){F z;fq_nmod_rand(z.v,FR,FC);return z;}
 static F generator(){F z;fq_nmod_gen(z.v,FC);return z;}
};
inline std::vector<F> roots(const std::vector<F>& c){
 fq_nmod_poly_t f;fq_nmod_poly_init(f,FC);
 for(size_t i=0;i<c.size();i++)fq_nmod_poly_set_coeff(f,i,c[i].v,FC);
 fq_nmod_poly_factor_t fac;fq_nmod_poly_factor_init(fac,FC);fq_nmod_poly_roots(fac,f,0,FC);
 std::vector<F> out;for(long i=0;i<fac->num;i++){F a,b;fq_nmod_poly_get_coeff(a.v,fac->poly+i,0,FC);fq_nmod_poly_get_coeff(b.v,fac->poly+i,1,FC);out.push_back(-a/b);}
 fq_nmod_poly_factor_clear(fac,FC);fq_nmod_poly_clear(f,FC);
 std::sort(out.begin(),out.end(),[](const F&a,const F&b){return a.coeffs()<b.coeffs();});return out;
}
inline std::vector<long> parse_poly(std::string s){
 s=std::regex_replace(s,std::regex("\\s+"),"");std::vector<long> c(1);size_t i=0;
 while(i<s.size()){
  int sign=1;if(s[i]=='+'||s[i]=='-'){sign=s[i]=='-'?-1:1;i++;}
  size_t j=i;while(j<s.size()&&s[j]!='+'&&s[j]!='-')j++;
  std::string t=s.substr(i,j-i);require(!t.empty(),"empty polynomial term");size_t v=t.find_first_of("xzuvb");long a;int degree=0;
  if(v==std::string::npos)a=std::stol(t);else{std::string k=t.substr(0,v);if(!k.empty()&&k.back()=='*')k.pop_back();a=k.empty()?1:std::stol(k);degree=1;size_t e=t.find('^');if(e!=std::string::npos)degree=std::stoi(t.substr(e+1));}
  require(degree>=0&&degree<100,"polynomial degree bound");if(c.size()<=size_t(degree))c.resize(degree+1);c[degree]+=sign*a;i=j;
 }return c;
}
inline F eval_poly(const std::vector<long>&c,const F&z){F a;for(auto i=c.rbegin();i!=c.rend();++i)a=a*z+F(*i);return a;}
inline std::pair<F,F> parse_curve(const std::string&s,const F&z){
 std::smatch m;require(std::regex_search(s,m,std::regex("x\\^3 \\+ \\(([^)]+)\\)\\*x \\+ \\(([^)]+)\\)")),"unsupported explicit curve spelling");return{eval_poly(parse_poly(m[1]),z),eval_poly(parse_poly(m[2]),z)};
}
using V3=std::array<F,3>;using M3=std::array<V3,3>;
inline F dot(const V3&a,const V3&b){F z;for(int i=0;i<3;i++)z+=a[i]*b[i];return z;}
inline F det(const M3&a){return a[0][0]*(a[1][1]*a[2][2]-a[1][2]*a[2][1])-a[0][1]*(a[1][0]*a[2][2]-a[1][2]*a[2][0])+a[0][2]*(a[1][0]*a[2][1]-a[1][1]*a[2][0]);}
inline M3 inverse(M3 a){M3 b{};for(int i=0;i<3;i++)b[i][i]=1;for(int i=0;i<3;i++){int k=i;while(k<3&&!a[k][i])k++;require(k<3,"singular 3 by 3 matrix");std::swap(a[i],a[k]);std::swap(b[i],b[k]);F z=a[i][i];for(int j=0;j<3;j++){a[i][j]=a[i][j]/z;b[i][j]=b[i][j]/z;}for(int t=0;t<3;t++)if(t!=i){F q=a[t][i];for(int j=0;j<3;j++){a[t][j]=a[t][j]-q*a[i][j];b[t][j]=b[t][j]-q*b[i][j];}}}return b;}
inline M3 matmul(const M3&a,const M3&b){M3 z{};for(int i=0;i<3;i++)for(int j=0;j<3;j++)for(int k=0;k<3;k++)z[i][j]+=a[i][k]*b[k][j];return z;}
struct Point{F x,y;bool infinity=true;Point()=default;Point(F xx,F yy):x(std::move(xx)),y(std::move(yy)),infinity(false){}};
inline bool operator==(const Point&a,const Point&b){return a.infinity||b.infinity?a.infinity==b.infinity:a.x==b.x&&a.y==b.y;}
struct EC{
 F a,b;
 Point add(const Point&P,const Point&Q)const{
  if(P.infinity)return Q;
  if(Q.infinity)return P;
  F m;
  if(P.x==Q.x){if(P.y+Q.y==F(0))return Point();m=(F(3)*P.x*P.x+a)/(F(2)*P.y);}else m=(P.y-Q.y)/(P.x-Q.x);
  F x=m*m-P.x-Q.x;return Point(x,m*(P.x-x)-P.y);
 }
 Point neg(const Point&P)const{return P.infinity?P:Point(P.x,-P.y);}
 Point mul(uint64_t n,Point P)const{Point Q;while(n){if(n&1)Q=add(Q,P);P=add(P,P);n>>=1;}return Q;}
 Point random()const{for(int t=0;t<10000;t++){F x=F::random(),v=x*x*x+a*x+b;if(v.square())return Point(x,v.sqrt());}throw std::runtime_error("point sampling exhausted");}
 std::vector<F> three_x()const{return roots({-a*a,F(12)*b,F(6)*a,F(0),F(3)});}
 EC quotient3(const F&x)const{return EC{a-F(10)*(F(3)*x*x+a),b-F(14)*(F(5)*x*x*x+F(3)*a*x+F(2)*b)};}
 F j()const{return F(1728)*F(4)*a.pow(3)/(F(4)*a.pow(3)+F(27)*b*b);}
};
inline json read_json(const std::string&path){std::ifstream f(path);require(bool(f),"cannot read "+path);json d;f>>d;return d;}
inline void write_json(const std::string&path,const json&d){std::ofstream f(path);require(bool(f),"cannot write "+path);f<<d.dump(2)<<'\n';}
