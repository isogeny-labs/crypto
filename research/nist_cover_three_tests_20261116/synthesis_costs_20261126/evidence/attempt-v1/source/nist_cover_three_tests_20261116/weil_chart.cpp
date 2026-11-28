#include "weil3_arithmetic.hpp"
#include "weil_chart.hpp"
int main(int argc,char**argv){try{
    require(argc==2,"usage: weil_chart study-directory");std::string root=argv[1];
    initialize_cover(read_json(root+"/inputs/p192-d7-prym.json"));
    auto receipt=read_json(root+"/evidence/weil3-v2/result.json");
    std::vector<G> kernel;std::vector<Weil3> functions;std::vector<F> alpha;
    for(auto& row:receipt.at("functions").arr){
        G p{polynomial(row.at("P_U")),polynomial(row.at("P_V"))};kernel.push_back(p);
        functions.emplace_back(p);alpha.push_back(field(row.at("alpha")));
        require(functions.back().a==polynomial(row.at("A"))&&functions.back().b==polynomial(row.at("B")),"pinned principal function replay");
    }
    require(kernel.size()==27,"kernel receipt size");
    auto inputs=generic_inputs(kernel);int regular_checks=0,collision_checks=0,ramified=0;
    for(auto& z:inputs){
        auto arc=curve_arc(z.u,z.v,curve);
        for(auto& f:functions){require(regularized(f,arc)==f(z),"regular versus arc evaluation");++regular_checks;}
    }
    std::vector<F> values;
    for(int i=1;i<27;++i){
        auto arc=curve_arc(kernel[i].u,kernel[i].v,curve);
        auto [g,s,t]=xgcd(kernel[i].u,kernel[i].v);if(g.degree()>0)++ramified;
        auto negative=curve_arc(kernel[i].u,kernel[i].neg().v,curve);
        int opposite=0;for(int j=1;j<27;++j)if(kernel[j]==kernel[i].neg())opposite=j;
        require(opposite!=0,"opposite kernel class");
        F same=regularized(functions[i],arc),other=regularized(functions[opposite],arc);
        require(alpha[i]*regularized(functions[i],negative)==alpha[opposite]*other,"collision-chart symmetry");
        require(other==F(0),"f_minusP has zero at P because P-minusP is theta");
        values.push_back(same);values.push_back(other);collision_checks+=2;
        std::cerr<<"regularized kernel class "<<i<<'\n';
    }
    rejects([&]{CurveArc<F> unmoving{constant_jet(kernel[1].u),constant_jet(kernel[1].v)};
                regularized(functions[1],unmoving);},"unresolved zero denominator in stationary arc");
    std::cout<<"{\"scope\":\"P192-kernel1-special-chart-Weil3\",\"precision\":24,\"regular_comparisons\":"<<regular_checks<<",\"collision_evaluations\":"<<collision_checks<<",\"ramified_support_classes\":"<<ramified<<",\"rejected_controls\":1,\"standing\":\"producer-unverified\",\"quartic_constructed\":false,\"collision_values\":[";
    for(size_t i=0;i<values.size();++i){if(i)std::cout<<',';emit_f(values[i]);}std::cout<<"]}\n";
}catch(const std::exception&e){std::cerr<<"REJECT "<<e.what()<<'\n';return 1;}}
