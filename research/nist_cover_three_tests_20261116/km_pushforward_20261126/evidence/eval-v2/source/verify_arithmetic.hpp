#pragma once
// Separate OpenSSL curve-map/Frobenius evaluation; no GMP arithmetic imported.
#include "projective_base.hpp"
struct Cycle { Poly u,x,y;int coordinate=0; };
BP curve;std::array<BP,4> maps;std::array<HP,4> homogeneous_maps;HP homogeneous_curve;
int poles=0,closed_points=0,instances=0;
K eval(const BP& p,const K& x,const K& y) {
    K r;for(const auto& [e,c]:p.t)r=r+K(c)*power(x,e[0])*power(y,e[1]);return r;
}
Point image(const K& x,const K& y) {
    require(eval(curve,x,y)==K(0),"closed point on actual quartic");
    K a=eval(maps[0],x,y),b=eval(maps[1],x,y),c=eval(maps[2],x,y),d=eval(maps[3],x,y);
    if(b!=K(0)&&d!=K(0)) { Point r(a/b,c/d);valid(r);return r; }
    auto jet=lift(homogeneous_curve,{R{x},R{y},R{K(1)}},2);
    Value vx=ratio(homogeneous_maps[0],homogeneous_maps[1],jet),vy=ratio(homogeneous_maps[2],homogeneous_maps[3],jet);
    Point r=endpoint(vx,vy);if(r.zero)++poles;return r;
}
Point negative(const Point& p) { return p.zero?p:Point(p.x,-p.y); }
Cycle read_cycle(const Json& row) {
    Cycle c{read_poly(row.at("support")),read_poly(row.at("x")),read_poly(row.at("y")),row.at("coordinate_lambda").integer()};
    require(c.u.degree()==3&&c.u.c.back()==F(1)&&c.x.degree()<3&&c.y.degree()<3,"entire degree-three cycle");
    require(c.coordinate>=0&&c.coordinate<=308,"retained separating-coordinate cap");
    Poly z{F(0),F(1)};require((c.x+c.y.scale(F(c.coordinate)))%c.u==z%c.u,"separated coordinate reconstruction");
    auto incidence=[&](const Poly& y) { Poly r;for(const auto& [e,a]:curve.t)r=(r+(power_mod(c.x,e[0],c.u)*power_mod(y,e[1],c.u)).scale(a))%c.u;return r; };
    require(incidence(c.y).c.empty(),"complete cycle quartic incidence with multiplicities");
    return c;
}
Poly polynomial_value(const Poly& f,const Poly& x,const Poly& u) { Poly r;for(int i=f.degree();i>=0;--i)r=(r*x+Poly{f.at(i)})%u;return r; }
Point norm(const Cycle& c) {
    auto factors=checked_factors(c.u);Point total;std::vector<Poly> seen;
    for(const auto& factor:factors) {
        bool prior=false;for(const auto& old:seen)if(old==factor)prior=true;if(prior)continue;seen.push_back(factor);
        int m=0;for(const auto& other:factors)if(other==factor)++m;instances+=m;++closed_points;
        residue_modulus=factor;Point p=image(K(c.x),K(c.y)),trace,conjugate=p;
        for(int i=0;i<factor.degree();++i) { trace=add(trace,conjugate);if(!conjugate.zero)conjugate=Point(conjugate.x.frobenius(),conjugate.y.frobenius()); }
        require(conjugate==p,"closed elliptic image Frobenius closure");
        if(!trace.zero)require(trace.x.p.degree()<=0&&trace.y.p.degree()<=0,"elliptic trace prime-field descent");
        total=add(total,times(trace,m));
    }return total;
}
void exact_binding(const Cycle& decoded,const Cycle& original) {
    Poly z=(original.x+original.y.scale(F(decoded.coordinate)))%original.u;
    require(polynomial_value(decoded.u,z,original.u).c.empty()&&polynomial_value(decoded.x,z,original.u)==original.x&&polynomial_value(decoded.y,z,original.u)==original.y,"exact repeated cycle binding");
    std::vector<std::vector<F>> columns;Poly p{F(1)};
    for(int i=0;i<3;++i) { std::vector<F> v;for(int j=0;j<3;++j)v.push_back(p.at(j));columns.push_back(v);p=(p*z)%original.u; }
    require(matrix_rank(columns)==3,"complete repeated cycle algebra rank");
}
