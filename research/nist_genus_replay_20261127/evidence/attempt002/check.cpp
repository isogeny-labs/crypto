#include "json.hpp"
#include <gmpxx.h>
#include <array>
#include <iostream>
#include <numeric>
using Z=mpz_class;
using P=std::vector<Z>;
Z p;
P q;
Z mod(Z a){mpz_mod(a.get_mpz_t(),a.get_mpz_t(),p.get_mpz_t());return a;}
Z inv(Z a){Z b;require(mpz_invert(b.get_mpz_t(),a.get_mpz_t(),p.get_mpz_t())!=0,"inverse");return b;}
void trim(P&a){while(!a.empty()&&a.back()==0)a.pop_back();}
P add(P a,const P&b){a.resize(std::max(a.size(),b.size()));for(size_t i=0;i<b.size();++i)a[i]=mod(a[i]+b[i]);trim(a);return a;}
P scale(P a,Z c){for(auto&v:a)v=mod(v*c);trim(a);return a;}
P sub(P a,const P&b){return add(a,scale(b,-1));}
P remainder(P a,const P&b){require(!b.empty(),"zero divisor");Z c=inv(b.back());while(a.size()>=b.size()){size_t k=a.size()-b.size();Z v=mod(a.back()*c);for(size_t i=0;i<b.size();++i)a[k+i]=mod(a[k+i]-v*b[i]);trim(a);}return a;}
P gcd(P a,P b){while(!b.empty()){P c=remainder(a,b);a=b;b=c;}require(!a.empty(),"zero gcd");return scale(a,inv(a.back()));}
P product(const P&a,const P&b){if(a.empty()||b.empty())return{};P c(a.size()+b.size()-1);for(size_t i=0;i<a.size();++i)for(size_t j=0;j<b.size();++j)c[i+j]+=a[i]*b[j];for(auto&v:c)v=mod(v);return remainder(c,q);}
P power(P a,Z n){P b{1};while(n>0){if(mpz_odd_p(n.get_mpz_t()))b=product(b,a);a=product(a,a);n>>=1;}return b;}
P compose(const P&a,const P&b){P r;for(size_t i=a.size();i-->0;)r=add(product(r,b),P{a[i]});return r;}
P derivative(P a){P d;for(size_t i=1;i<a.size();++i)d.push_back(mod(a[i]*i));trim(d);return d;}
P readpoly(const Json&j){P r;for(auto&v:j.arr){Z a(v.scalar);require(a>=0&&a<p,"noncanonical coefficient");r.push_back(a);}trim(r);return r;}
// Frobenius k -> A(k), s -> s B(k), where s^2=L(k).
struct Map{P a,b;};
Map compose(const Map&f,const Map&g){return{compose(f.a,g.a),product(g.b,compose(f.b,g.a))};}
Map exponent(Map a,int n){Map r{{0,1},{1}};while(n){if(n&1)r=compose(r,a);a=compose(a,a);n>>=1;}return r;}
bool identity(const Map&a){return a.a==P({0,1})&&a.b==P({1});}
using HP=std::vector<P>;
HP hmul(const HP&a,const HP&b){HP c(a.size()+b.size()-1);for(size_t i=0;i<a.size();++i)for(size_t j=0;j<b.size();++j)c[i+j]=add(c[i+j],product(a[i],b[j]));return c;}
int main(int argc,char**argv)try{
 require(argc==5,"usage: check packet expected-p expected-b expected-d");Json j=read_json(argv[1]);p=Z(argv[2]);Z b(argv[3]),d(argv[4]);require(p>3,"characteristic");
 require(Z(j.at("p").scalar)==p&&Z(j.at("b").scalar)==b&&Z(j.at("d").scalar)==d,"model binding");
 q=readpoly(j.at("Q"));require(q.size()==41&&q.back()==1,"degree40 monic Q");require(gcd(q,derivative(q)).size()==1,"Q not squarefree");
 P W=readpoly(j.at("W")),U=readpoly(j.at("U")),L=readpoly(j.at("L")),K{0,1};require(W.size()<=40&&U.size()<=40&&L.size()<=40,"coordinate degree");require(gcd(q,K).size()==1&&gcd(q,L).size()==1,"zero k/lambda");require(L==sub(scale(W,2),scale(U,3)),"lambda relation");
 // Independently expand the written quintic before normalizing X=(b-2)u.
 P X{0,1},Xm{p-1,1},Xd{mod(d),1};P bracket=add(sub(power(Xd,3),scale(product(Xd,power(Xm,2)),3)),scale(power(Xm,3),b));P raw=product(product(X,Xm),bracket);require(raw.size()==6,"raw quintic degree");Z alpha=raw[5];require(alpha!=0,"leading coefficient");P f(6);for(int i=0;i<5;++i){Z s=1;for(int z0=0;z0<4-i;++z0)s=mod(s*alpha);f[i]=mod(raw[i]*s);}f[5]=1;
 require(gcd(f,derivative(f)).size()==1,"singular curve");require(mod(b*b-4)!=0&&mod(d+1)!=0&&mod(b-d*d*d+3*d)!=0,"singular cover");
 P K2=product(K,K),K3=product(K2,K);P Zc=scale(add(sub(scale(L,f[4]),product(W,W)),add(scale(product(U,U),3),scale(K2,3))),inv(2));
 HP C{K3,Zc,W,P{1}},Div{K2,U,P{1}};HP left=hmul(C,C),right=hmul(hmul(Div,Div),Div);for(int i=0;i<6;++i)left[i]=sub(left[i],scale(L,f[i]));require(left==right,"principal cubic coefficient identity");
 std::cout<<"PASS squarefree degree40, model binding, and all principal-cubic coefficients\n";
 Map frob{power(K,p),power(L,(p-1)/2)};require(product(product(frob.b,frob.b),L)==compose(L,frob.a),"signed Frobenius relation");
 // The eighty distinct reduced order3 classes exhaust J[3] minus zero.
 // Its Frobenius lies in GSp4(F3), whose order is103680.
 std::vector<Map>bits{frob};for(int i=1;i<17;++i)bits.push_back(compose(bits.back(),bits.back()));
 auto nth=[&](int n){Map r{{0,1},{1}};for(int i=0;n;++i,n>>=1)if(n&1)r=compose(r,bits[i]);return r;};
 int degree=103680;require(identity(nth(degree)),"Frobenius outside GSp4 bound");for(int ell:{2,3,5})while(degree%ell==0&&identity(nth(degree/ell)))degree/=ell;
 require(identity(nth(degree)),"final Frobenius identity");for(int ell:{2,3,5})if(degree%ell==0)require(!identity(nth(degree/ell)),"nonminimal Frobenius degree");
 std::cout<<"derived_full_genus2_three_torsion_field_degree="<<degree<<"\n";
 return 0;
}catch(std::exception&e){std::cerr<<e.what()<<'\n';return 1;}
