#include "quartic_km_v3_arithmetic.hpp"
#include <chrono>
#include <cstdlib>
using Coefficients=std::vector<Vector>;
Coefficients reconstruct(const QuarticKM& km,const Class& input) {
    require(input.w.size()==22 && row_basis(input.w,evaluation_count).size()==22,"valid RR space dimension");
    Coefficients result;
    for (const auto& row:input.w) {
        require(row.size()==evaluation_count,"input evaluation width");Space equations;
        for (int i=0;i<evaluation_count;++i) {
            Vector eq;for (const auto& v:km.v3) eq.push_back(v[size_t(i)]);eq.push_back(-row[size_t(i)]);equations.push_back(std::move(eq));
        }
        auto kernel=nullspace(equations,int(km.v3.size()+1));require(kernel.size()==1 && kernel[0].back()!=Z(0),"class row belongs to V3");
        Vector c(kernel[0].begin(),kernel[0].end()-1);Z scale=kernel[0].back().inv();for (auto& x:c) x=x*scale;
        require(linear_combination(c,km.v3)==row,"exact numerator reconstruction");result.push_back(std::move(c));
    }
    return result;
}
int infinity_multiplicity(const QuarticKM& km,const Class& input) {
    auto coefficients=reconstruct(km,input);int degree=-1;
    for (const auto& row:coefficients) for (size_t i=0;i<row.size();++i) if (row[i]!=Z(0)) degree=std::max(degree,km.mons[i][0]+km.mons[i][1]);
    int m=9-degree;require(m>=0 && m<=3,"effective degree12 infinity multiplicity");return m;
}
Class shift(const QuarticKM& km,const Class& input,int exponent) {
    Space rows=input.w;
    for (auto& row:rows) for (size_t i=0;i<row.size();++i) {
        Z l(km.points[size_t(km.pool[i])].x-56),factor=zpow(l,std::abs(exponent));
        row[i]=exponent<0?row[i]/factor:row[i]*factor;
    }
    return {row_basis(rows,evaluation_count)};
}
struct Normalized { int removed=0;Class finite; };
Normalized normalize(const QuarticKM& km,const Class& input) {
    int m=infinity_multiplicity(km,input);Class finite=shift(km,input,m);
    require(infinity_multiplicity(km,finite)==0,"normalization removes all infinity support");return {m,finite};
}
int main(int argc,char** argv) { try {
    auto start=std::chrono::steady_clock::now();require(argc==3,"usage: normalize-km quartic relations");
    auto model=read_json(argv[1]),relations=read_json(argv[2]);QuarticKM km(model,relations.at("pool"));require(km.fibers[0].x==56,"fixed KM reference");
    std::vector<Z> qc(5,Z(0));for (const auto& [e,c]:km.terms) if (e[0]+e[1]==4) qc[size_t(e[1])]=qc[size_t(e[1])]+c;
    Poly<Z> q(qc),x{Z(0),Z(1)};q=q.monic();require(q.degree()==4 && q.at(0)!=Z(0),"declared infinity and x-leading chart");
    auto p=x%q;for (int i=1;i<=4;++i) { p=powmod(p,prime,q);if (i==2) require(std::get<0>(xgcd(q,p-x)).degree()==0,"GMP infinity irreducibility subfield check"); }
    require(p==x%q,"GMP infinity Frobenius closure");
    auto nonzero=km.point_class(km.fibers[1].support[0]);require(!km.zero(nonzero),"actual nonzero Jacobian fixture");
    std::vector<int> multiplicities;int principal=0,nonprincipal=0;
    for (int m=0;m<=3;++m) {
        Class input=shift(km,km.identity,-m);auto result=normalize(km,input);
        require(result.removed==m && result.finite==km.identity && km.equal(input,result.finite),"principal infinity normalization");
        multiplicities.push_back(result.removed);++principal;
    }
    for (int m=1;m<=2;++m) {
        Class input=shift(km,nonzero,-m);auto result=normalize(km,input);
        require(result.removed==m && result.finite==nonzero && km.equal(input,result.finite),"nonzero infinity normalization");
        multiplicities.push_back(result.removed);++nonprincipal;
    }
    Class bad=km.identity;bad.w[0][0]=bad.w[0][0]+Z(1);bool rejected=false;
    try { (void)normalize(km,bad); } catch (const std::exception&) { rejected=true; }
    require(rejected,"changed evaluation entry rejected");
    auto us=std::chrono::duration_cast<std::chrono::microseconds>(std::chrono::steady_clock::now()-start).count();
    std::cout<<"{\"standing\":\"unregistered native KM infinity normalization\",\"GMP_infinity_irreducibility_checked\":true,\"principal_class_checks\":"<<principal<<",\"nonzero_class_checks\":"<<nonprincipal<<",\"detected_multiplicities\":[";
    for (size_t i=0;i<multiplicities.size();++i) std::cout<<(i?",":"")<<multiplicities[i];
    std::cout<<"],\"all_outputs_have_zero_infinity_multiplicity\":true,\"canonical_output_and_KM_equalities_checked\":true,\"changed_evaluation_rejected\":true,\"valid_RR_input_required\":true,\"wall_us\":"<<us<<",\"signed_independent_verdict\":false}\n";return 0;
} catch (const std::exception& e) { std::cerr<<"REJECT "<<e.what()<<'\n';return 1; } }
