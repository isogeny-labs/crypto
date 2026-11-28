#pragma once
#include "closed_fibers_arithmetic.hpp"
#include "quartic_km_v3_arithmetic.hpp"
CFPoly pc_read_poly(const Json& row){std::vector<Z> c;for(auto& value:row.arr)c.emplace_back(mpz_class(value.scalar));return CFPoly(c);}
EC pc_read_point(const Json& row){if(row.at("infinity").scalar=="true")return {};return EC(F(CFPoly{Z(mpz_class(row.at("x").scalar))}),F(CFPoly{Z(mpz_class(row.at("y").scalar))}));}
CFPoly pc_power_mod(CFPoly a,int n,const CFPoly& u){CFPoly out{Z(1)};for(int i=0;i<n;++i)out=(out*a)%u;return out;}
struct PublicClassAdapter {
    QuarticKM& km;
    std::vector<int> auxiliary,excluded;
    explicit PublicClassAdapter(QuarticKM& k,const std::vector<ClosedFiber>& fibers):km(k){
        std::set<int> evaluation(km.pool.begin(),km.pool.end());
        for(int i=int(km.points.size())-1;i>=0&&auxiliary.size()<6;--i){
            bool usable=!evaluation.count(i)&&km.points[i].x!=km.fibers[0].x;
            for(auto& f:fibers){if(f.u.eval(Z(km.points[i].x))==Z(0))usable=false;}
            if(usable)auxiliary.push_back(i);else excluded.push_back(i);
        }
        require(auxiliary.size()==6,"six fixed disjoint auxiliary rational points");
    }
    Class encode(const ClosedFiber& fiber)const{
        require(km.mons.size()==34,"large-model degree9 section basis34");
        Space constraints(6,Vector(km.mons.size(),Z(0)));CFPoly x{Z(0),Z(1)};
        for(size_t j=0;j<km.mons.size();++j){auto e=km.mons[j];auto value=(pc_power_mod(x,e[0],fiber.u)*pc_power_mod(fiber.v,e[1],fiber.u))%fiber.u;for(int i=0;i<6;++i)constraints[i][j]=value.at(i);}
        for(int index:auxiliary){const auto& p=km.points.at(size_t(index));Vector row;for(auto e:km.mons)row.push_back(zpow(Z(p.x),e[0])*zpow(p.y,e[1]));constraints.push_back(row);}
        auto coefficients=nullspace(constraints,int(km.mons.size()));require(coefficients.size()==22,"closed-fiber effective12 RR coefficient dimension22");Space values;
        for(auto& row:coefficients){values.push_back(linear_combination(row,km.v3));}
        auto space=row_basis(values,evaluation_count);require(space.size()==22,"closed-fiber effective12 evaluation dimension22");return {space};
    }
};
Class pc_times(QuarticKM& km,Class value,mpz_class n){Class out=km.identity;while(n>0){if(mpz_odd_p(n.get_mpz_t()))out=km.add(out,value);n>>=1;if(n>0)value=km.add(value,value);}return out;}
void pc_emit_class(const Class& value){std::cout<<'[';for(size_t i=0;i<value.w.size();++i){if(i)std::cout<<',';std::cout<<'[';for(size_t j=0;j<value.w[i].size();++j)std::cout<<(j?",":"")<<'"'<<value.w[i][j].x<<'"';std::cout<<']';}std::cout<<']';}
