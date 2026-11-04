// RFC 7748 section 4.1, exact GMP development fixture. This is a finite
// self-check, not a universal theorem or an independently verified claim.
#pragma once
#include <gmpxx.h>
#include <stdexcept>
namespace actual25519 {
using B=mpz_class;
struct P {B x=0,y=0;bool inf=true;bool operator==(const P&)const=default;};
struct E {
    B p,a,b;
    B m(B x)const {B r=x%p;if(r<0)r+=p;return r;}
    B inv(B x)const {x=m(x);B r;if(!mpz_invert(r.get_mpz_t(),x.get_mpz_t(),p.get_mpz_t()))throw std::runtime_error("big inversion");return r;}
    bool has(P q)const {return q.inf || m(q.y*q.y)==m(q.x*(q.x*q.x+a*q.x+b));}
    P add(P q,P r)const {
        if(q.inf)return r;
        if(r.inf)return q;
        if(q.x==r.x && m(q.y+r.y)==0)return {};
        B slope;
        if(q.x==r.x)slope=m((3*q.x*q.x+2*a*q.x+b)*inv(2*q.y));
        else slope=m((r.y-q.y)*inv(r.x-q.x));
        B x=m(slope*slope-a-q.x-r.x);return {x,m(slope*(q.x-x)-q.y),false};
    }
    P mul(B n,P q)const {P r;while(n>0){if(mpz_odd_p(n.get_mpz_t()))r=add(r,q);q=add(q,q);n/=2;}return r;}
    E quotient()const {return {p,m(-2*a),m(a*a-4*b)};}
    P phi(P q)const {
        if(q.inf || q.x==0)return {};
        B z=inv(q.x);return {m(q.x+a+b*z),m(q.y*(1-b*m(z*z))),false};
    }
    P dual(P q)const {P z=quotient().phi(q);if(!z.inf){z.x=m(z.x*inv(4));z.y=m(z.y*inv(8));}return z;}
};
}
inline void curve25519_fixture() {
    using namespace actual25519;
    auto check=[](bool b,const char* msg){if(!b)throw std::runtime_error(msg);};
    B p=(B(1)<<255)-19;
    B ell=(B(1)<<252)+B("27742317777372353535851937790883648493");
    E e{p,486662,1};P g{9,B("14781619447589544791020593568409986887264606134616475288964881837755586237401"),false};
    check(e.has(g),"RFC 7748 generator on curve");
    check(e.mul(ell,g).inf,"RFC generator subgroup order");
    E quotient=e.quotient();P image=e.phi(g);
    check(quotient.has(image) && !image.inf,"Curve25519 quotient image");
    check(quotient.mul(ell,image).inf,"Curve25519 quotient subgroup");
    B inverse2=(ell+1)/2;
    for(int k:{1,2,3,17,127}) {
        P q=e.mul(k,g);
        check(e.phi(q)==quotient.mul(k,image),"Curve25519 transport scalar fixture");
        check(e.dual(e.phi(q))==e.mul(2,q),"Curve25519 dual fixture");
        check(e.mul(inverse2,e.dual(e.phi(q)))==q,"Curve25519 inverse subgroup transport");
    }
    B trace=p+1-8*ell,disc=trace*trace-4*p;
    check(trace==B("-221938542218978828286815502327069187962"),"RFC trace fixture");
    for(int l:{3,5,7,11,13,17,19,23,29,31}) {
        B small=l;
        check(mpz_legendre(disc.get_mpz_t(),small.get_mpz_t())==(l<23?-1:1),"small-prime splitting fixture");
    }
}
