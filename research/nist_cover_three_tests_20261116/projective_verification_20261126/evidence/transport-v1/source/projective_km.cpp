#include "quartic_km_v3_arithmetic.hpp"
#include <chrono>
int main(int argc,char** argv) { try {
    auto start=std::chrono::steady_clock::now();require(argc==3,"usage: projective-km quartic relations");
    auto model=read_json(argv[1]),relations=read_json(argv[2]);QuarticKM km(model,relations.at("pool"));require(km.fibers[0].x==56,"fixed projective relation reference");
    Space rows;int sections=0;
    for (size_t i=0;i<km.mons.size();++i) if (km.mons[i][0]+km.mons[i][1]<=6) {
        Vector row=km.v3[i];
        for (size_t j=0;j<km.pool.size();++j) { Z l(km.points[size_t(km.pool[j])].x-56);row[j]=row[j]*l*l; }
        rows.push_back(std::move(row));++sections;
    }
    require(sections==22,"complete degree-six section basis");Class value{row_basis(rows,evaluation_count)};
    require(value.w.size()==22 && km.zero(value),"actual projective hyperplane class is zero");
    auto nonce=km.point_class(km.fibers[1].support[0]);require(!km.zero(nonce),"actual nonzero target control class");
    require(!km.zero(km.add(value,nonce)),"changed projective principal class rejected");
    auto us=std::chrono::duration_cast<std::chrono::microseconds>(std::chrono::steady_clock::now()-start).count();
    std::cout<<"{\"standing\":\"unregistered actual GMP target KM verification\",\"projective_principal_KM_zero\":true,\"effective_divisor_degree\":12,\"class_dimension\":22,\"evaluation_points\":85,\"changed_class_rejected\":true,\"target_addflip_calls\":"<<km.flips<<",\"wall_us\":"<<us<<",\"class_space\":[";
    for (size_t i=0;i<value.w.size();++i) {
        if (i) std::cout<<',';
        std::cout<<'[';
        for (size_t j=0;j<value.w[i].size();++j) std::cout<<(j?",":"")<<'"'<<value.w[i][j].x<<'"';
        std::cout<<']';
    }
    std::cout<<"],\"signed_independent_verdict\":false}\n";return 0;
} catch (const std::exception& e) { std::cerr<<"REJECT "<<e.what()<<'\n';return 1; } }
