#include "complete_maps_arithmetic.hpp"
#include <iomanip>
struct TRFiber {CFPoly u,v;G reduced;int saturated=0;};
P tr_embed(const CFPoly& p){std::vector<F> c;for(auto z:p.c)c.push_back(F(CFPoly{z}));return P(c);}
G tr_reduce(CFPoly u,CFPoly v,const CFPoly& h){require(u.degree()>=0,"transpose finite divisor support");u=u.monic();v=v%u;require(((v*v-h)%u).degree()<0,"transpose raw source incidence");while(u.degree()>3){auto q=(h-v*v).exact(u).monic();require(q.degree()<u.degree(),"transpose strict Cantor reduction");v=(-v)%q;u=q;}G r{tr_embed(u),tr_embed(v)};r.check();return r;}
TRFiber tr_fiber(const MMap& map,const Z& tx,const Z& ty){
    require(map.f.eval(tx,ty)==Z(0),"transpose target curve input");std::array<CFPoly,4> eq;for(int k=0;k<2;++k){Z power(1);for(auto& row:map.global.at(k?"V":"U").arr){auto c=pc_read_poly(row.at("c")),scale=map.den.exact(c);eq[size_t(2*k)]=eq[size_t(2*k)]+(pc_read_poly(row.at("a"))*scale).scale(power);eq[size_t(2*k+1)]=eq[size_t(2*k+1)]+(pc_read_poly(row.at("b"))*scale).scale(power);power=power*tx;}}eq[2]=eq[2]-map.den.scale(ty);
    TRFiber out;out.u=cf_gcd(cf_gcd(eq[0]*eq[3]-eq[2]*eq[1],eq[0]*eq[0]-eq[1]*eq[1]*map.h),eq[2]*eq[2]-eq[3]*eq[3]*map.h).monic();require(out.u.degree()>=0,"nonzero transpose elimination support");for(;;){auto g=cf_gcd(out.u,map.den);if(g.degree()==0)break;out.u=out.u.exact(g).monic();++out.saturated;}
    require(out.u.degree()>0&&out.u.degree()<=9,"transpose affine degree at most9");auto[g,s,t]=xgcd(eq[1]%out.u,eq[3]%out.u);auto[gg,ss,tt]=xgcd(g,out.u);require(gg.degree()==0,"transpose source y separation");out.v=(-(s*eq[0]+t*eq[2])*ss)%out.u;require(((out.v*out.v-map.h)%out.u).degree()<0,"transpose recovered source equation");require(((eq[0]+eq[1]*out.v)%out.u).degree()<0&&((eq[2]+eq[3]*out.v)%out.u).degree()<0,"transpose both graph equations");out.reduced=tr_reduce(out.u,out.v,map.h);return out;
}
void tr_emit_g(const G& p){std::cout<<"{\"U\":";cf_emit_poly(rational(p.u));std::cout<<",\"V\":";cf_emit_poly(rational(p.v));std::cout<<'}';}
