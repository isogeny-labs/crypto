#pragma once
// Exact arithmetic for the bounded native recognizer, not a proof backend.
#include <gmpxx.h>
#include <algorithm>
#include <iostream>
#include <stdexcept>
#include <vector>
namespace weber {
using I=mpz_class;
using Q=mpq_class;
inline void require(bool ok,const char* message) {
    if(!ok)throw std::runtime_error(message);
}
struct E {
    Q a=0,b=0; // a+b*omega, omega^2+omega+1=0
    E()=default;
    E(long n):a(n){}
    E(Q aa,Q bb=0):a(std::move(aa)),b(std::move(bb)){a.canonicalize();b.canonicalize();}
    bool zero()const{return a==0&&b==0;}
};
inline bool operator==(const E& x,const E& y){return x.a==y.a&&x.b==y.b;}
inline bool operator!=(const E& x,const E& y){return !(x==y);}
inline E operator+(const E& x,const E& y){return {x.a+y.a,x.b+y.b};}
inline E operator-(const E& x){return {-x.a,-x.b};}
inline E operator-(const E& x,const E& y){return x+(-y);}
inline E operator*(const E& x,const E& y){
    return {x.a*y.a-x.b*y.b,x.a*y.b+x.b*y.a-x.b*y.b};
}
inline E inverse(const E& x){
    Q norm=x.a*x.a-x.a*x.b+x.b*x.b;
    require(norm!=0,"zero quadratic denominator");
    return {(x.a-x.b)/norm,-x.b/norm};
}
inline E operator/(const E& x,const E& y){return x*inverse(y);}
inline E conjugate(const E& x){return {x.a-x.b,-x.b};}
using P=std::vector<E>;
inline void trim(P& a){while(!a.empty()&&a.back().zero())a.pop_back();}
inline P scalar(const E& a){return a.zero()?P{}:P{a};}
inline P operator+(P a,const P& b){
    a.resize(std::max(a.size(),b.size()));
    for(size_t i=0;i<b.size();++i)a[i]=a[i]+b[i];
    trim(a);return a;
}
inline P operator-(P a){for(E& v:a)v=-v;return a;}
inline P operator-(const P& a,const P& b){return a+(-b);}
inline P operator*(const P& a,const P& b){
    if(a.empty()||b.empty())return {};
    P c(a.size()+b.size()-1);
    for(size_t i=0;i<a.size();++i)for(size_t j=0;j<b.size();++j)c[i+j]=c[i+j]+a[i]*b[j];
    trim(c);return c;
}
inline P scale(P a,const E& c){for(E& v:a)v=v*c;trim(a);return a;}
inline std::pair<P,P> divide(P a,const P& b){
    require(!b.empty(),"zero polynomial divisor");
    if(a.size()<b.size())return {{},a};
    P q(a.size()-b.size()+1);E leading=inverse(b.back());
    while(a.size()>=b.size()){
        size_t k=a.size()-b.size();E c=a.back()*leading;q[k]=q[k]+c;
        for(size_t j=0;j<b.size();++j)a[k+j]=a[k+j]-c*b[j];
        trim(a);
    }
    trim(q);return {q,a};
}
inline P monic(P a){return a.empty()?a:scale(a,inverse(a.back()));}
inline Q norm(const E& a){return a.a*a.a-a.a*a.b+a.b*a.b;}
inline I rounded(const Q& a){
    I numerator=2*a.get_num()+a.get_den(),denominator=2*a.get_den(),out;
    mpz_fdiv_q(out.get_mpz_t(),numerator.get_mpz_t(),denominator.get_mpz_t());return out;
}
inline E integer_gcd(E a,E b){
    require(a.a.get_den()==1&&a.b.get_den()==1&&b.a.get_den()==1&&b.b.get_den()==1,"nonintegral content input");
    if(a.b==0&&b.b==0){I g;mpz_gcd(g.get_mpz_t(),a.a.get_num().get_mpz_t(),b.a.get_num().get_mpz_t());return E(Q(g));}
    while(!b.zero()){
        E ratio=a/b,q{Q(rounded(ratio.a)),Q(rounded(ratio.b))},r=a-q*b;
        require(norm(r)<norm(b),"Eisenstein content norm did not decrease");
        a=std::move(b);b=std::move(r);
    }
    return a;
}
inline P primitive(P a){
    if(a.empty())return a;
    I denominator=1;
    for(const E& v:a){mpz_lcm(denominator.get_mpz_t(),denominator.get_mpz_t(),v.a.get_den().get_mpz_t());
        mpz_lcm(denominator.get_mpz_t(),denominator.get_mpz_t(),v.b.get_den().get_mpz_t());}
    a=scale(a,E(Q(denominator)));E content;
    for(const E& v:a){content=integer_gcd(content,v);if(norm(content)==1)break;}
    require(!content.zero(),"zero primitive content");
    for(E& v:a){v=v/content;require(v.a.get_den()==1&&v.b.get_den()==1,"nonexact integral content division");}
    return a;
}
inline P primitive_remainder(P a,const P& b){
    require(!b.empty(),"zero primitive polynomial divisor");
    while(a.size()>=b.size()){
        size_t k=a.size()-b.size();E lead=a.back();a=scale(a,b.back());
        for(size_t j=0;j<b.size();++j)a[k+j]=a[k+j]-lead*b[j];
        trim(a);a=primitive(std::move(a));
    }
    return a;
}
inline P gcd(P a,P b){
    a=primitive(std::move(a));b=primitive(std::move(b));
    while(!b.empty()){P next=primitive_remainder(a,b);a=std::move(b);b=std::move(next);}
    return monic(a);
}
inline P derivative(const P& a){
    if(a.size()<2)return {};
    P out(a.size()-1);for(size_t i=1;i<a.size();++i)out[i-1]=a[i]*E(static_cast<long>(i));
    trim(out);return out;
}
inline P reduce(const P& a,const P& c){return divide(a,c).second;}
inline P multiply_mod(const P& a,const P& b,const P& c){return reduce(a*b,c);}
inline P evaluate_mod(const P& a,const P& x,const P& c){
    P out;for(auto i=a.rbegin();i!=a.rend();++i)out=multiply_mod(out,x,c)+scalar(*i);
    return out;
}
inline P inverse_mod(P a,const P& c){
    P previous=c,s0,s1{E(1)};
    while(!a.empty()){
        auto qr=divide(previous,a);P next=s0-qr.first*s1;
        previous=std::move(a);a=std::move(qr.second);s0=std::move(s1);s1=std::move(next);
    }
    require(previous.size()==1,"nonunit field denominator");
    return reduce(scale(s0,inverse(previous[0])),c);
}
struct F {
    P n,d;
    F():d{E(1)}{}
    explicit F(long a):n(scalar(E(a))),d{E(1)}{}
    F(P nn,P dd):n(std::move(nn)),d(std::move(dd)){
        trim(n);trim(d);require(!d.empty(),"zero rational-function denominator");
        if(n.empty()){d={E(1)};return;}
        P common=gcd(n,d);n=divide(n,common).first;d=divide(d,common).first;
        E lead=inverse(d.back());n=scale(n,lead);d=scale(d,lead);
    }
};
inline F operator+(const F& a,const F& b){return {a.n*b.d+b.n*a.d,a.d*b.d};}
inline F operator-(const F& a,const F& b){return {a.n*b.d-b.n*a.d,a.d*b.d};}
inline F operator*(const F& a,const F& b){return {a.n*b.n,a.d*b.d};}
inline F operator/(const F& a,const F& b){return {a.n*b.d,a.d*b.n};}
inline F power(F a,unsigned n){F out(1);while(n){if(n&1)out=out*a;n>>=1;if(n)a=a*a;}return out;}
inline F coefficient(const E& a){return {scalar(a),{E(1)}};}
struct SymbolicPoint {F f=F(1),g=F(1);};
inline SymbolicPoint twice(const SymbolicPoint& p){
    F w(P{E(0),E(1)},P{E(1)}),one(1);
    F f=F(9)*w*power(p.f,4)/(F(4)*(w+one)*power(p.g,2))-F(2)*p.f;
    F g=F(3)*w*power(p.f,2)/(F(2)*(w+one)*p.g)*(p.f-f)-p.g;
    return {f,g};
}
inline SymbolicPoint add_base(const SymbolicPoint& p){
    F w(P{E(0),E(1)},P{E(1)}),one(1);
    F slope=(one-p.g)/(one-p.f);
    F f=(w+one)/w*power(slope,2)-p.f-one;
    F g=slope*(p.f-f)-p.g;
    return {f,g};
}
inline SymbolicPoint times(unsigned n){
    require(n>0,"symbolic identity has no affine functions");SymbolicPoint out;
    unsigned top=1;while(top<=n/2)top<<=1;
    for(top>>=1;top;top>>=1){out=twice(out);if(n&top)out=add_base(out);}
    return out;
}
struct Kernel {P c,nx,ny,qx,qy;};
inline Kernel kernel(unsigned u){
    SymbolicPoint a=times(u-1),b=times(2*u-1);
    F fx=a.f-coefficient(E(Q(0),Q(1)))*b.f,fy=a.g+b.g;
    P c=gcd(fx.n,fy.n);require(c.size()==(3*u*(u-1))/6+1,"unexpected Weber degree");
    for(const P* denominator:{&a.f.d,&b.f.d,&a.g.d,&b.g.d})
        require(gcd(c,*denominator).size()==1,"kernel includes a pole");
    auto dx=divide(fx.n,c),dy=divide(fy.n,c);
    require(dx.second.empty()&&dy.second.empty(),"kernel quotient certificate failed");
    return {c,fx.n,fy.n,dx.first,dy.first};
}
inline std::ostream& operator<<(std::ostream& out,const E& a){return out<<a.a<<' '<<a.b;}
inline std::istream& operator>>(std::istream& in,E& a){return in>>a.a>>a.b;}
} // namespace weber
