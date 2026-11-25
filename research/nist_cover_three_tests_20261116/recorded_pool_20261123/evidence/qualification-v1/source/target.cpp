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
int main(int argc,char** argv){try{
    require(argc==5,"usage: target INPUT-directory FROZEN.json AUDIT.json OUTPUT.json");std::string root=argv[1];initialize_cover(read_json(root+"/inputs/p192-d7-prym.json"));Level2Input level;
    auto model=read_json(root+"/evidence/p192-d7-kernel3-plane-quartic.json"),baseline=read_json(argv[2]),audit=read_json(argv[3]);EBPoly f;for(const auto& term:model.at("terms").arr){auto e=term.at("exponents");f.add({e.arr[0].integer(),e.arr[1].integer()},Z(mpz_class(term.at("coefficient").scalar)));}
    QuarticKM km(model,baseline.at("pool"));std::map<int,int> padding;for(int i:km.fibers[0].support)padding[i]=2;
    auto encode=[&](const Json& indices){auto d=padding;for(const auto& i:indices.arr)++d[i.integer()];return rpa_effective(km,f,d);};
    int old_checks=0;for(const auto& fiber:km.fibers){auto d=padding;for(int i:fiber.support)++d[i];auto a=rpa_effective(km,f,d),b=km.effective(d);require(km.equal(a,b)&&km.zero(a),"old simple fiber encoder agreement");++old_checks;}
    int principal=0,nonprincipal=0;for(const auto& candidate:audit.at("norm_quartets").arr){bool actual=km.zero(encode(candidate.at("indices"))),expected=candidate.at("canonical").scalar=="true";require(actual==expected,"geometric/KM degree4 classification");if(actual)++principal;else ++nonprincipal;}
    int pair_checks=0;for(const auto& candidate:audit.at("pair_relations").arr){const auto& q=candidate.at("indices").arr;require(q.size()==4,"pair relation support");auto a=padding,b0=padding;int anchor=km.fibers[0].support[0];a[anchor]+=2;b0[anchor]+=2;++a[q[0].integer()];++a[q[1].integer()];++b0[q[2].integer()];++b0[q[3].integer()];require(!km.equal(rpa_effective(km,f,a),rpa_effective(km,f,b0)),"distinct degree2 divisors nonprincipal on actual quartic");++pair_checks;}
    require(!km.zero(encode(audit.at("corrupted_reference"))),"changed reference divisor must be nonzero");
    std::ofstream out(argv[4]);require(bool(out),"new target result");out<<"{\"scope\":\"actual-frozen-target-Jacobian-classes\",\"standing\":\"producer-unverified\",\"old_encoder_agreements\":"<<old_checks<<",\"principal_quartets\":"<<principal<<",\"nonprincipal_norm_quartets\":"<<nonprincipal<<",\"nonprincipal_norm_pairs\":"<<pair_checks<<",\"changed_reference_rejected\":true,\"target_addflips\":"<<km.flips<<",\"new_curve_points\":0}\n";require(bool(out),"complete target output");
}catch(const std::exception& e){std::cerr<<"REJECT "<<e.what()<<'\n';return 1;}}
