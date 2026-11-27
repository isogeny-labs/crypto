#include "complete_km_arithmetic.hpp"

// Same complete Newton-jet construction as cm_rational_point_class, now for
// a complete effective degree-12 divisor, including tangent/hyperflex support.
Class rpa_effective(QuarticKM& km,const EBPoly& f,const std::map<int,int>& divisor){
    Space constraints;int degree=0;
    for(const auto& item:divisor){int index=item.first,m=item.second;require(m>=0&&m<=12,"degree12 support multiplicity");degree+=m;if(!m)continue;
        const auto& p=km.points.at(size_t(index));std::vector<Z> coefficients(size_t(m+1),Z(0));coefficients[size_t(m)]=Z(1);CFPoly modulus(coefficients),t{Z(0),Z(1)},x{Z(p.x)},y{p.y};
        const bool xlocal=f.partial(1).eval(Z(p.x),p.y)!=Z(0);if(xlocal)x=x+t;else{require(f.partial(0).eval(Z(p.x),p.y)!=Z(0),"smooth recorded point chart");y=y+t;}
        auto derivative=f.partial(xlocal?1:0);
        for(int precision=1;precision<m;precision*=2){auto error=cm_curve(f,modulus,x,y),d=cm_curve(derivative,modulus,x,y);if(xlocal)y=(y-error*cm_inverse(d,modulus))%modulus;else x=(x-error*cm_inverse(d,modulus))%modulus;}
        require(cm_curve(f,modulus,x,y).degree()<0,"complete recorded divisor curve jets");
        std::vector<CFPoly> jets;for(auto exponent:km.mons)jets.push_back((pc_power_mod(x,exponent[0],modulus)*pc_power_mod(y,exponent[1],modulus))%modulus);
        for(int order=0;order<m;++order){Vector row;for(const auto& jet:jets)row.push_back(jet.at(order));constraints.push_back(std::move(row));}
    }
    require(degree==12&&constraints.size()==12,"complete effective degree12");auto coefficients=nullspace(constraints,int(km.mons.size()));require(coefficients.size()==22,"complete divisor RR22");Space values;for(const auto& c:coefficients)values.push_back(linear_combination(c,km.v3));auto out=row_basis(values,evaluation_count);require(out.size()==22,"complete divisor evaluation22");return {out};
}
