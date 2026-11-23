// Actual unknown-coefficient rational solver; bounded correctness baseline only.
// This explicitly enumerates p*(p-1) coefficient pairs. It is not sub-exponential.
#include <algorithm>
#include <cstdint>
#include <iostream>
#include <set>
#include <sstream>
#include <stdexcept>
#include <string>
#include <utility>
#include <vector>

namespace coeff {
using Poly = std::vector<int>; // constant first
struct Field {
    int p;
    int mod(long long x) const { x %= p; return static_cast<int>(x < 0 ? x + p : x); }
    int power(int x, unsigned e) const { int y=1; for (;e;e>>=1,x=mod(x*x)) if(e&1U)y=mod(y*x); return y; }
    int inv(int x) const { if(!mod(x))throw std::invalid_argument("zero inverse"); return power(mod(x),static_cast<unsigned>(p-2)); }
};
bool prime(int n) { if(n<2)return false;for(int d=2;d*d<=n;++d)if(n%d==0)return false;return true; }
void trim(Poly &a) { while(a.size()>1&&a.back()==0)a.pop_back();if(a.empty())a.push_back(0); }
bool zero(const Poly&a) { return a.size()==1&&a[0]==0; }
Poly add(const Field&f,Poly a,const Poly&b,int sign=1) {
    a.resize(std::max(a.size(),b.size()),0);
    for(std::size_t i=0;i<b.size();++i)a[i]=f.mod(a[i]+sign*b[i]);
    trim(a);return a;
}
Poly scale(const Field&f,Poly a,int c) {for(int&v:a)v=f.mod(v*c);trim(a);return a;}
Poly mul(const Field&f,const Poly&a,const Poly&b) {
    Poly z(a.size()+b.size()-1,0);
    for(std::size_t i=0;i<a.size();++i)for(std::size_t j=0;j<b.size();++j)z[i+j]=f.mod(z[i+j]+a[i]*b[j]);
    trim(z);return z;
}
std::pair<Poly,Poly> divrem(const Field&f,Poly a,Poly b) {
    trim(a);trim(b);if(zero(b))throw std::invalid_argument("zero polynomial divisor");
    Poly q(a.size()>=b.size()?a.size()-b.size()+1:1,0);
    const int b_inv=f.inv(b.back());
    while(!zero(a)&&a.size()>=b.size()){
        std::size_t offset=a.size()-b.size();int k=f.mod(a.back()*b_inv);q[offset]=k;
        for(std::size_t j=0;j<b.size();++j)a[offset+j]=f.mod(a[offset+j]-k*b[j]);
        trim(a);
    }
    trim(q);return {q,a};
}
Poly gcd(const Field&f,Poly a,Poly b) {
    while(!zero(b)){Poly r=divrem(f,a,b).second;a=b;b=r;}
    return scale(f,a,f.inv(a.back()));
}
Poly derivative(const Field&f,const Poly&a) {
    Poly z(std::max<std::size_t>(1,a.size()-1),0);
    for(std::size_t i=1;i<a.size();++i)z[i-1]=f.mod(static_cast<long long>(i)*a[i]);
    trim(z);return z;
}
int evaluate(const Field&f,const Poly&a,int x) {int y=0;for(auto i=a.rbegin();i!=a.rend();++i)y=f.mod(y*x+*i);return y;}
struct Config {Field field;int a,b,t,c,rx,ry;};
int argument(const char*s) {
    std::string v(s);if(v.empty()||v.find_first_not_of("0123456789")!=std::string::npos)throw std::invalid_argument("nonnegative decimal parameters required");
    std::size_t used=0;int z=std::stoi(v,&used);if(used!=v.size())throw std::invalid_argument("invalid integer");return z;
}
Config parse(int argc,char**argv) {
    if(argc!=8)throw std::invalid_argument("usage: norm_solver p a b t c target_x target_y");
    Config z{{argument(argv[1])},argument(argv[2]),argument(argv[3]),argument(argv[4]),argument(argv[5]),argument(argv[6]),argument(argv[7])};
    const auto&f=z.field;
    if(f.p<5||f.p>257||!prime(f.p))throw std::invalid_argument("bounded prime field required:5<=p<=257");
    for(int v:{z.a,z.b,z.c,z.rx,z.ry})if(v<0||v>=f.p)throw std::invalid_argument("parameter outside field");
    if(!f.mod(4LL*z.a*z.a*z.a+27LL*z.b*z.b))throw std::invalid_argument("singular curve");
    int order=1;for(int x=0;x<f.p;++x)for(int y=0;y<f.p;++y)if(f.mod(y*y)==f.mod(x*x*x+z.a*x+z.b))++order;
    if(!prime(order)||order==f.p)throw std::invalid_argument("non-anomalous prime-order whole curve required");
    if(z.t<1||z.t>8||((f.p-1)%(1U<<z.t))||z.c==0||f.power(z.c,static_cast<unsigned>((f.p-1)/(1U<<z.t)))!=1)throw std::invalid_argument("nonzero split membership coset required");
    if(f.mod(z.ry*z.ry)!=f.mod(z.rx*z.rx*z.rx+z.a*z.rx+z.b))throw std::invalid_argument("target is not on curve");
    return z;
}
std::string key(std::vector<std::pair<int,int>> pts) {
    std::sort(pts.begin(),pts.end());std::ostringstream s;
    for(std::size_t i=0;i<pts.size();++i){if(i)s<<';';s<<pts[i].first<<':'<<pts[i].second;}return s.str();
}
struct Result {std::set<std::string> models;std::uint64_t coefficient_pairs=0;std::uint64_t membership_passes=0;std::uint64_t root_field_candidates=0;};
Result solve(const Config&z) {
    const auto&f=z.field;const Poly curve{z.b,z.a,0,1};Result result;
    for(int u=0;u<f.p;++u)for(int w=1;w<f.p;++w){
        ++result.coefficient_pairs;
        const int v=f.mod(z.ry*w-z.rx*z.rx-u*z.rx);
        const Poly a{v,u,1};
        const Poly norm=add(f,mul(f,a,a),scale(f,curve,f.mod(w*w)),-1);
        auto qr=divrem(f,norm,{f.mod(-z.rx),1});
        if(!zero(qr.second))throw std::runtime_error("target elimination failed");
        const Poly&U=qr.first;
        if(U.size()!=4||U.back()!=1)throw std::runtime_error("unexpected norm degree");
        if(evaluate(f,U,z.rx)==0||gcd(f,U,derivative(f,U))!=Poly{1})continue;
        Poly remainder{0,1};
        for(int j=0;j<z.t;++j)remainder=divrem(f,mul(f,remainder,remainder),U).second;
        if(remainder!=Poly{z.c})continue;
        ++result.membership_passes;
        result.root_field_candidates+=static_cast<unsigned>(f.p);
        std::vector<std::pair<int,int>> points;
        const int minus_inverse=f.mod(-f.inv(w));
        for(int x=0;x<f.p;++x)if(evaluate(f,U,x)==0){
            const int y=f.mod(evaluate(f,a,x)*minus_inverse);
            if(x==z.rx||f.power(x,1U<<z.t)!=z.c||f.mod(y*y)!=f.mod(x*x*x+z.a*x+z.b))throw std::runtime_error("rational recovery verification failed");
            points.emplace_back(x,y);
        }
        if(points.size()!=3)throw std::runtime_error("split squarefree membership did not yield three roots");
        if(!result.models.insert(key(points)).second)throw std::runtime_error("duplicate monic norm model");
    }
    return result;
}
} // namespace coeff
#ifndef NORM_SOLVER_NO_MAIN
int main(int argc,char**argv) {
    try {
        auto z=coeff::parse(argc,argv);auto result=coeff::solve(z);
        for(const auto&model:result.models)std::cout<<"model\t"<<model<<'\n';
        std::cout<<"summary\t"<<result.models.size()<<'\t'<<result.coefficient_pairs<<'\t'<<result.membership_passes<<'\n';
        return 0;
    }catch(const std::exception&e){std::cerr<<e.what()<<'\n';return 2;}
}
#endif
