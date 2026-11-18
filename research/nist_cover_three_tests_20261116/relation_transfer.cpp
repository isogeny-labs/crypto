#include "complete_km_arithmetic.hpp"
#include "relation_pipeline_arithmetic.hpp"
int main(int argc,char** argv){try{
    RPTimer timer;require(argc==7,"usage: relation_transfer input-root global.json bridge.json formal.json public.json relation-result.json");
    std::string root=argv[1];initialize_cover(read_json(root+"/inputs/p192-d7-prym.json"));Level2Input level;
    auto model=read_json(root+"/evidence/p192-d7-kernel3-plane-quartic.json"),global=read_json(argv[2]),bridge=read_json(argv[3]),formal=read_json(argv[4]),pub=read_json(argv[5]),relations=read_json(argv[6]);
    EBPoly f;for(auto& t:model.at("terms").arr){auto& e=t.at("exponents").arr;f.add({e[0].integer(),e[1].integer()},Z(mpz_class(t.at("coefficient").scalar)));}
    MMap map(global,f);QuarticKM km(model,relations.at("pool"));CMAdapter adapter(km,f,formal);G reference;
    for(int id:adapter.reference){auto& p=km.points[size_t(id)];reference=reference.add(tr_fiber(map,Z(p.x),p.y).reduced);}
    auto& publics=pub.at("public_class_spaces").arr;EC g=pc_read_point(publics[1].at("source_point")),q=pc_read_point(publics[3].at("source_point"));
    std::vector<EC> sources{EC{},g,g.neg(),times(g,2),q,times(g,2).neg()},norms;
    std::vector<G> duals;std::vector<MForward> forwards;std::vector<Class> classes;std::vector<RPStage> fixture_costs;
    timer.mark("input_setup_and_transpose_reference");
    for(size_t i=0;i<sources.size();++i){auto d=pull(sources[i]);G dual;Class image=km.identity;MForward forward;
        if(d.u.degree()>0){forward=map.forward(d);require(forward.shift<0,"direct reduced-source forward chart");image=adapter.encode(forward.cycle,d.u.degree()).value;
            std::vector<CTFiber> witnesses;dual=ct_closed(map,forward.cycle.u,forward.cycle.x,forward.cycle.y,witnesses).add(reference.times(d.u.degree()).neg());
            require(dual==d.times(3),"fresh actual dual-forward composition3");}
        else require(km.zero(image),"fresh transfer zero class");
        auto n=norm(dual);require(n==times(sources[i],6),"fresh cover-isogeny-dual-norm composition6");norms.push_back(n);duals.push_back(dual);classes.push_back(image);forwards.push_back(forward);
        timer.mark("fresh_transfer_fixture_"+std::to_string(i));std::cerr<<"fresh complete transfer "<<i<<" passed\n";}
    require(km.equal(classes[1],gb_class(bridge.at("images").arr[0].at("class_space"),km)),"fresh generator target class binding");
    require(km.equal(classes[4],gb_class(bridge.at("images").arr[1].at("class_space"),km)),"fresh public target class binding");
    require(km.zero(km.add(classes[1],classes[2]))&&km.zero(km.add(classes[3],classes[5])),"fresh target negatives");
    require(km.equal(km.add(classes[1],classes[1]),classes[3]),"fresh target doubling");
    require(sources[5].add(times(g,2)).zero,"negative2G exceptional old shift fixture");timer.mark("target_bridge_negation_and_doubling_checks");
    std::cout<<"{\"standing\":\"producer-unverified\",\"fresh_complete_transfer_checks\":6,\"actual_target_KM_executed\":true,\"dual_source_composition3_checks\":5,\"old_shift_infinity_case_checked\":true,\"fixtures\":[";
    for(size_t i=0;i<sources.size();++i){if(i)std::cout<<',';std::cout<<"{\"point\":";rp_point(std::cout,sources[i]);std::cout<<",\"norm\":";rp_point(std::cout,norms[i]);std::cout<<",\"source_dual\":";tr_emit_g(duals[i]);
        if(i){std::cout<<",\"forward_cycle\":";cm_emit_cycle(forwards[i].cycle);}std::cout<<'}';}
    std::cout<<"],\"stages\":[";for(size_t i=0;i<timer.stages.size();++i){if(i)std::cout<<',';auto& s=timer.stages[i];std::cout<<"{\"name\":\""<<s.name<<"\",\"wall_us\":"<<s.wall_us<<",\"cpu_us\":"<<s.cpu_us<<'}';}
    std::cout<<"],\"target_addflip_calls\":"<<km.flips<<",\"signed_independent_verdict\":false}\n";
}catch(const std::exception& e){std::cerr<<"REJECT "<<e.what()<<'\n';return 1;}}
