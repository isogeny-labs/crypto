#pragma once
// Native replay of the archived P-192 d7 torsion certificate. No timings/solver claims.
#include "json.hpp"
#include <gmpxx.h>
#include <algorithm>
#include <iostream>
#include <set>
#include <tuple>
mpz_class prime;
struct Z {
    mpz_class x;
    Z(long v=0):x(v){reduce();} Z(const mpz_class&v):x(v){reduce();}
    void reduce(){mpz_mod(x.get_mpz_t(),x.get_mpz_t(),prime.get_mpz_t());}
    Z operator+(Z b)const{return Z(mpz_class(x+b.x));} Z operator-(Z b)const{return Z(mpz_class(x-b.x));}
    Z operator-()const{return Z(mpz_class(-x));} Z operator*(Z b)const{return Z(mpz_class(x*b.x));}
    Z inv()const{mpz_class a;require(mpz_invert(a.get_mpz_t(),x.get_mpz_t(),prime.get_mpz_t())!=0,"Fp inverse");return Z(a);}
    Z operator/(Z b)const{return *this*b.inv();}bool operator==(Z b)const{return x==b.x;}bool operator!=(Z b)const{return !(*this==b);}
};
template<class T> struct Poly {
    std::vector<T> c;
    Poly()=default;Poly(std::initializer_list<T> v):c(v){trim();}explicit Poly(std::vector<T> v):c(std::move(v)){trim();}
    void trim(){while(!c.empty()&&c.back()==T(0))c.pop_back();}
    int degree()const{return int(c.size())-1;}T at(int i)const{return i>=0&&i<int(c.size())?c[i]:T(0);}
    bool operator==(const Poly&b)const{return c==b.c;}bool operator!=(const Poly&b)const{return !(*this==b);}
    Poly operator+(const Poly& b)const{std::vector<T> r(std::max(c.size(),b.c.size()));for(int i=0;i<int(r.size());++i)r[i]=at(i)+b.at(i);return Poly(r);}
    Poly operator-()const{auto r=c;for(auto&v:r)v=-v;return Poly(r);}Poly operator-(const Poly&b)const{return *this+-b;}
    Poly operator*(const Poly&b)const{if(c.empty()||b.c.empty())return {};std::vector<T> r(c.size()+b.c.size()-1);for(int i=0;i<int(c.size());++i)for(int j=0;j<int(b.c.size());++j)r[i+j]=r[i+j]+c[i]*b.c[j];return Poly(r);}
    Poly scale(T a)const{auto r=c;for(auto&v:r)v=v*a;return Poly(r);}
    std::pair<Poly,Poly> divrem(const Poly&b)const{require(b.degree()>=0,"zero polynomial divisor");Poly r=*this;std::vector<T> q(std::max(0,degree()-b.degree()+1));T inv=b.c.back().inv();while(r.degree()>=b.degree()){
        int k=r.degree()-b.degree();T v=r.c.back()*inv;q[k]=v;for(int j=0;j<=b.degree();++j)r.c[k+j]=r.c[k+j]-v*b.c[j];r.trim();}return {Poly(q),r};}
    Poly operator%(const Poly&b)const{return divrem(b).second;}
    Poly exact(const Poly&b)const{auto qr=divrem(b);require(qr.second.degree()<0,"nonexact polynomial division");return qr.first;}
    Poly monic()const{return c.empty()?*this:scale(c.back().inv());}
    T eval(T a)const{T r(0);for(int i=degree();i>=0;--i)r=r*a+c[i];return r;}
    std::string key()const{std::string s;for(auto&v:c)s+=v.key()+";";return s;}
};
template<class T> std::tuple<Poly<T>,Poly<T>,Poly<T>> xgcd(Poly<T>a,Poly<T>b){Poly<T> s{T(1)},t,ss,tt{T(1)};while(b.degree()>=0){auto qr=a.divrem(b);auto ns=s-qr.first*ss,nt=t-qr.first*tt;a=b;b=qr.second;s=ss;ss=ns;t=tt;tt=nt;}require(a.degree()>=0,"zero gcd");T z=a.c.back().inv();return {a.scale(z),s.scale(z),t.scale(z)};}
template<class T> Poly<T> powmod(Poly<T>a,mpz_class n,const Poly<T>&m){Poly<T> r{T(1)};while(n>0){if(mpz_odd_p(n.get_mpz_t()))r=(r*a)%m;n>>=1;if(n>0)a=(a*a)%m;}return r;}
Poly<Z> modulus;
struct F {
    Poly<Z> a;
    F(long v=0):a{Z(v)}{}explicit F(Poly<Z>v):a(v%modulus){}
    F operator+(const F&b)const{return F(a+b.a);}F operator-()const{return F(-a);}F operator-(const F&b)const{return F(a-b.a);}
    F operator*(const F&b)const{return F((a*b.a)%modulus);}F inv()const{auto [g,s,t]=xgcd(a,modulus);require(g.degree()==0,"extension inverse");return F(s);}
    F operator/(const F&b)const{return *this*b.inv();}bool operator==(const F&b)const{return a==b.a;}bool operator!=(const F&b)const{return !(*this==b);}
    F power(mpz_class n)const{return F(powmod(a,n,modulus));}
    std::string key()const{std::string s;for(auto&v:a.c)s+=v.x.get_str()+",";return s;}
};
Poly<Z> base_poly(const Json&j){std::vector<Z> v;for(auto&e:j.arr)v.emplace_back(mpz_class(e.scalar));return Poly<Z>(v);}
F field(const Json&j){return F(base_poly(j));}
Poly<F> polynomial(const Json&j){std::vector<F> v;for(auto&e:j.arr)v.push_back(field(e));return Poly<F>(v);}
using P=Poly<F>;
P curve;
struct D {
    P u{F(1)},v;
    bool operator==(const D&b)const{return u==b.u&&v==b.v;}
    bool operator!=(const D&b)const{return !(*this==b);}
    void check()const{require(u.degree()>=0&&u.degree()<=2&&u.c.back()==F(1)&&v.degree()<u.degree(),"Mumford shape");require(((v*v-curve)%u).degree()<0,"Mumford curve equation");}
    D neg()const{return {u,(-v)%u};}
    D add(const D&b)const{
        auto [d1,e1,e2]=xgcd(u,b.u);auto [d,c1,c2]=xgcd(d1,v+b.v);
        P uu=(u*b.u).exact(d*d);
        P vv=(c1*e1*u*b.v+c1*e2*b.u*v+c2*(v*b.v+curve)).exact(d)%uu;
        while(uu.degree()>2){P n=(curve-vv*vv).exact(uu).monic();vv=(-vv)%n;uu=n;}
        D r{uu.monic(),vv};r.check();return r;
    }
    D frob()const{auto a=u.c,b=v.c;for(auto&x:a)x=x.power(prime);for(auto&x:b)x=x.power(prime);return {P(a),P(b)};}
    std::string key()const{return u.key()+"|"+v.key();}
};
F result(const P&u,const P&h){require(u.degree()==2,"quadratic resultant input");P r=h%u;F a=r.at(1),b=r.at(0);return b*b-u.at(1)*a*b+u.at(0)*a*a;}
void init_field(const Json&j){prime=mpz_class(j.at("source_model").at("p").scalar);require(mpz_probab_prime_p(prime.get_mpz_t(),40)>0,"prime-field primality screen");modulus=base_poly(j.at("finite_field_modulus_coefficients"));require(modulus.degree()==8&&modulus.c.back()==Z(1),"field degree8");
    Poly<Z>x{Z(0),Z(1)},a=x;for(int i=1;i<=8;++i){a=powmod(a,prime,modulus);if(i==4){auto[g,s,t]=xgcd(modulus,a-x);require(g.degree()==0,"reducible degree8 field");}}require(a==x,"field Frobenius closure");}
void prym(const Json&j,const Json&pair){init_field(j);F b(Poly<Z>{Z(mpz_class(j.at("source_model").at("b").scalar))});F d(j.at("source_model").at("d").integer());P x{F(0),F(1)},one{F(1)},xd=x+P{d},xm=x-one;
    P original=x*xm*(xd*xd*xd-(xd*xm*xm).scale(F(3))+(xm*xm*xm).scale(b));require(original.degree()==5,"Prym degree5");
    // Producer uses X=lc*u, Y=lc^2*w: Y^2=lc^4*f(X/lc), monic degree5.
    F lc=original.c.back(),scale=lc*lc*lc*lc;std::vector<F> normalized;
    for(int i=0;i<=5;++i){normalized.push_back(original.at(i)*scale);scale=scale/lc;}
    curve=P(normalized);require(curve.c.back()==F(1),"normalized Prym model");
    P derivative;std::vector<F> dc;for(int i=1;i<=curve.degree();++i)dc.push_back(curve.at(i)*F(i));derivative=P(dc);auto[cg,cs,ct]=xgcd(curve,derivative);require(cg.degree()==0,"Prym nonsmooth");
    std::vector<D> basis;for(auto&point:j.at("basis_Mumford_coefficients").arr){require(point.arr.size()==2,"basis shape");D a{polynomial(point.arr[0]),polynomial(point.arr[1])};a.check();require(a!=D{}&&a.add(a)==a.neg()&&a.add(a).add(a)==D{},"basis order3");basis.push_back(a);}require(basis.size()==4,"Prym basis size");
    std::vector<D> all{D{}};for(auto a:basis){auto old=all;for(auto w:old){all.push_back(w.add(a));all.push_back(w.add(a.neg()));}}
    std::set<std::string> distinct;for(auto a:all){a.check();distinct.insert(a.key());}require(distinct.size()==81,"Prym basis independence");
    const auto&m=j.at("Prym_Frobenius_matrix_mod3");require(m.arr.size()==4,"Frobenius4");for(int i=0;i<4;++i){D expected;require(m.arr[i].arr.size()==4,"Frobenius columns");for(int k=0;k<4;++k){int n=m.arr[i].arr[k].integer();require(n>=0&&n<3,"Frobenius coefficient");if(n)expected=expected.add(n==1?basis[k]:basis[k].neg());}require(expected==basis[i].frob(),"actual Prym Frobenius mismatch");}
    auto&data=pair.at("Prym_pairing");std::vector<P> tangents;std::vector<F> lambdas;for(auto&c:data.at("basis_cubic_tangent_functions").arr)tangents.push_back(polynomial(c));require(tangents.size()==4,"tangent count");
    for(int i=0;i<4;++i){auto&u=basis[i].u;auto&v=basis[i].v;auto&c=tangents[i];require(c.degree()==3&&(c%u)==v,"tangent restriction");P q=(c*c-curve).exact(u*u*u);require(q.degree()==0&&q.at(0)!=F(0),"tangent cubic divisor identity");lambdas.push_back(F(1)/q.at(0));}
    F zeta=field(data.at("primitive_third_root_coefficients"));require(zeta!=F(1)&&zeta.power(3)==F(1),"primitive third root");auto&form=data.at("canonical_Prym_Weil_matrix_mod3");
    for(int i=0;i<4;++i)for(int k=i+1;k<4;++k){F n=result(basis[k].u,basis[k].v-tangents[i]),den=result(basis[i].u,basis[i].v-tangents[k]);require(n!=F(0)&&den!=F(0),"Weil evaluation collision");F e=n/den*lambdas[i]/lambdas[k];require(e==zeta.power(form.arr[i].arr[k].integer())&&e.power(3)==F(1),"Prym pairing mismatch");}
    // Every recovered point also obeys order3, inverse, and Frobenius^8.
    for(auto a:all){require(a.add(a).add(a)==D{},"full torsion order");D f=a;for(int k=0;k<8;++k)f=f.frob();require(f==a,"torsion field degree bound");}
    std::cout<<"PASS P192 d7: irreducible Fp8; smooth genus2 model; 4 order3 basis divisors; 81 distinct torsion classes; actual Frobenius; 4 tangent identities; 6 Weil pairings; 81 order/Frobenius8 checks\n";
}
struct EC {
    F x,y;bool zero=true;
    EC()=default;EC(F a,F b):x(a),y(b),zero(false){}
    bool operator==(const EC&b)const{return zero==b.zero&&(zero||(x==b.x&&y==b.y));}
    EC neg()const{return zero?*this:EC(x,-y);}
    EC add(const EC&b)const{if(zero)return b;if(b.zero)return *this;if(x==b.x&&y==-b.y)return {};F s=(*this==b)?(F(3)*x*x-F(3))/(F(2)*y):(b.y-y)/(b.x-x);F xx=s*s-x-b.x;return {xx,s*(x-xx)-y};}
};
void elliptic(const Json&j){init_field(j);F b(Poly<Z>{Z(mpz_class(j.at("source_model").at("b").scalar))});std::vector<EC> a;for(auto&point:j.at("basis_point_coefficients").arr){EC q(field(point.arr.at(0)),field(point.arr.at(1)));require(q.y*q.y==q.x*q.x*q.x-F(3)*q.x+b,"elliptic equation");require(q.add(q)==q.neg(),"elliptic order3");a.push_back(q);}require(a.size()==2&&!(a[0]==a[1])&&!(a[0]==a[1].neg()),"elliptic basis independence");
    auto&m=j.at("Frobenius_matrix_mod3");for(int i=0;i<2;++i){EC expected;for(int k=0;k<2;++k){int n=m.arr.at(i).arr.at(k).integer();require(n>=0&&n<3,"elliptic Frobenius entry");if(n)expected=expected.add(n==1?a[k]:a[k].neg());}require(expected==EC(a[i].x.power(prime),a[i].y.power(prime)),"elliptic Frobenius mismatch");}
    auto line=[](const EC&p,const EC&q){return q.y-p.y-(F(3)*p.x*p.x-F(3))/(F(2)*p.y)*(q.x-p.x);};F e=-line(a[0],a[1])/line(a[1],a[0]);F zeta=field(j.at("primitive_third_root_coefficients"));require(zeta!=F(1)&&zeta.power(3)==F(1),"elliptic primitive root");require(e==zeta.power(j.at("canonical_Weil_matrix_mod3").arr.at(0).arr.at(1).integer()),"elliptic pairing");
    std::cout<<"PASS P192 E: irreducible Fp8; 2 independent order3 points; actual Frobenius; tangent Weil pairing\n";
}
