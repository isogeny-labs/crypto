#pragma once
// Build-only header is the unchanged point_replay.cpp with its CLI main removed.
#include "point_arithmetic.hpp"
// Separate genus3 balanced-map test; no fresh isogeny quotient is implemented here.
struct G {
    P u{F(1)},v;
    bool operator==(const G&b)const{return u==b.u&&v==b.v;}
    void check()const{require(u.degree()>=0&&u.degree()<=3&&u.c.back()==F(1)&&v.degree()<u.degree(),"genus3 Mumford shape");require(((v*v-curve)%u).degree()<0,"genus3 curve equation");}
    G neg()const{return {u,(-v)%u};}
    G add(const G&b)const{auto[d1,e1,e2]=xgcd(u,b.u);auto[d,c1,c2]=xgcd(d1,v+b.v);P uu=(u*b.u).exact(d*d);P vv=(c1*e1*u*b.v+c1*e2*b.u*v+c2*(v*b.v+curve)).exact(d)%uu;
        while(uu.degree()>3){P n=(curve-vv*vv).exact(uu).monic();vv=(-vv)%n;uu=n;}G out{uu.monic(),vv};out.check();return out;}
    G times(mpz_class n)const{G r,a=*this;while(n>0){if(mpz_odd_p(n.get_mpz_t()))r=r.add(a);n>>=1;if(n>0)a=a.add(a);}return r;}
};
EC times(EC a,mpz_class n){EC r;while(n>0){if(mpz_odd_p(n.get_mpz_t()))r=r.add(a);n>>=1;if(n>0)a=a.add(a);}return r;}
Z scalar(const F&a){require(a.a.degree()<=0,"non-base-field coefficient");return a.a.at(0);}
Poly<Z> rational(const P&a){std::vector<Z> out;for(auto&c:a.c)out.push_back(scalar(c));return Poly<Z>(out);}
void factors(Poly<Z> f,std::vector<Poly<Z>>&out){f=f.monic();if(f.degree()<1)return;if(f.degree()==1){out.push_back(f);return;}
    std::vector<Z> dc;for(int i=1;i<=f.degree();++i)dc.push_back(f.at(i)*Z(i));auto[g,s,t]=xgcd(f,Poly<Z>(dc));
    if(g.degree()>0){factors(g,out);factors(f.exact(g),out);return;}
    Poly<Z>x{Z(0),Z(1)};auto[h,a,b]=xgcd(f,powmod(x,prime,f)-x);
    if(h.degree()==0){require(f.degree()<=3,"factor degree limit");out.push_back(f);return;}
    if(h.degree()<f.degree()){factors(h,out);factors(f.exact(h),out);return;}
    for(int shift=0;shift<1024;++shift){auto q=powmod(x+Poly<Z>{Z(shift)},mpz_class((prime-1)/2),f)-Poly<Z>{Z(1)};auto[k,aa,bb]=xgcd(f,q);if(k.degree()>0&&k.degree()<f.degree()){factors(k,out);factors(f.exact(k),out);return;}}
    throw std::runtime_error("linear factor splitting limit");
}
// Lazily initialize field constants only after main has installed prime/modulus.
F& cover_c(){static F value;return value;}
F& cover_d(){static F value;return value;}
#define cc (cover_c())
#define dd (cover_d())
G pull(const EC&p){if(p.zero)return {};F xm=p.x-F(1),ds=dd+F(1);P u{-xm/(cc*cc*ds),-F(2)*xm/(cc*ds),F(1)};P linear{F(1),F(2)*cc};P v=(linear*linear).scale(p.y/(cc*cc*cc*cc))%u;
    G fiber{u,v};fiber.check();G rminus{P{F(1)/(F(2)*cc),F(1)},P{}};rminus.check();return fiber.add(rminus.neg());}
EC norm(const G&g){g.check();auto u=rational(g.u),v=rational(g.v);auto saved=modulus;Z c=scalar(cc),d=scalar(dd);std::vector<Poly<Z>> fs;factors(u,fs);EC total;
    // Keep the accumulated answer as base-field scalars across field switches.
    Z tx,ty;bool tz=true;int degree=0;
    for(auto factor:fs){degree+=factor.degree();modulus=factor;F X(Poly<Z>{Z(0),Z(1)});std::vector<F> vc;for(auto z:v.c)vc.emplace_back(Poly<Z>{z});F Y=P(vc).eval(X),C(Poly<Z>{c}),D0(Poly<Z>{d});EC image;
        if(X==F(0)){image=EC(F(1),C*C*C*C*Y);}else{F t=F(1)+F(1)/(C*X),den=t*t-F(1);if(den!=F(0))image=EC((t*t+D0)/den,Y/(X*X*X*X*den*den));}
        EC trace,conjugate=image;for(int i=0;i<factor.degree();++i){trace=trace.add(conjugate);if(!conjugate.zero)conjugate=EC(conjugate.x.power(prime),conjugate.y.power(prime));}
        if(!trace.zero){Z xx=scalar(trace.x),yy=scalar(trace.y);modulus=saved;EC previous=tz?EC{}:EC(F(Poly<Z>{tx}),F(Poly<Z>{ty}));EC sum=previous.add(EC(F(Poly<Z>{xx}),F(Poly<Z>{yy})));tz=sum.zero;if(!tz){tx=scalar(sum.x);ty=scalar(sum.y);}}
    }
    modulus=saved;require(degree==g.u.degree(),"norm factor degrees");return tz?EC{}:EC(F(Poly<Z>{tx}),F(Poly<Z>{ty}));
}
