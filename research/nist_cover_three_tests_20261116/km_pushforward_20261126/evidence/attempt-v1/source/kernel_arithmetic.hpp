#pragma once
#include "cover_arithmetic.hpp"
std::string gkey(const G&a){return a.u.key()+"|"+a.v.key();}
G reduce4(P u,P v){require(((v*v-curve)%u).degree()<0,"Prym pullback equation");while(u.degree()>3){P n=(curve-v*v).exact(u).monic();v=(-v)%n;u=n;}G a{u.monic(),v};a.check();return a;}
G pull_prym(const Json&point,F lc){P u=polynomial(point.arr.at(0)),v=polynomial(point.arr.at(1));require(u.degree()==2&&v.degree()<=1,"Prym pullback fixture shape");F u0=u.at(0)/(lc*lc),u1=u.at(1)/lc,v0=v.at(0)/(lc*lc),v1=v.at(1)/lc;
    P X{F(0),F(1)},A{F(1),cc},cx=X.scale(cc),A2=A*A,cx2=cx*cx;P U=(cx2*cx2).scale(u0)+(cx2*A2).scale(u1)+A2*A2;require(U.degree()==4,"Prym pullback degree4");U=U.monic();auto[g,s,t]=xgcd(A,U);require(g.degree()==0,"Prym pullback pole");P V=((X*X*X)*(A2.scale(v1)+cx2.scale(v0))).scale(F(1)/cc)*s;return reduce4(U,V%U);
}
G gfrob(const G&a){auto u=a.u.c,v=a.v.c;for(auto&x:u)x=x.power(prime);for(auto&x:v)x=x.power(prime);return {P(u),P(v)};}
G combination(const std::vector<G>&basis,const std::vector<int>&a){require(basis.size()==a.size(),"coordinate length");G out;for(size_t i=0;i<a.size();++i){require(a[i]>=0&&a[i]<3,"coordinate F3");if(a[i])out=out.add(a[i]==1?basis[i]:basis[i].neg());}return out;}
void emit_f(const F&a){std::cout<<'[';for(int i=0;i<modulus.degree();++i)std::cout<<(i?",":"")<<'"'<<a.a.at(i).x<<'"';std::cout<<']';}
void emit_p(const P&p){std::cout<<'[';for(int i=0;i<=p.degree();++i){if(i)std::cout<<',';emit_f(p.at(i));}std::cout<<']';}
