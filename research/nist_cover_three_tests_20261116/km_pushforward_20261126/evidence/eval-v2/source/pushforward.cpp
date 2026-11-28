#include "verify_arithmetic.hpp"
int main(int argc,char** argv) { try {
    auto begin=std::chrono::steady_clock::now();
    if(argc==4&&std::string(argv[1])=="--compare") {
        auto a=read_json(argv[2]),b=read_json(argv[3]);Point x=point(a.at("elliptic_result")),y=point(b.at("elliptic_result"));
        require(x==y,"standalone elliptic point matches separately checked result");
        std::cout<<"{\"points_equal\":true}\n";return 0;
    }
    require(argc==6,"usage: pushforward model backmap extracted pool formal");
    auto model=read_json(argv[1]),back=read_json(argv[2]),data=read_json(argv[3]),pool=read_json(argv[4]),formal=read_json(argv[5]);
    require(model.at("p").scalar==prime_text,"fixed P192 target field");curve=read_bp(model.at("terms"),true);
    std::array<std::string,4> names{"X_numerator","X_denominator","Y_numerator","Y_denominator"};int degree=0;
    for(size_t i=0;i<4;++i) { maps[i]=read_bp(back.at(names[i]));degree=std::max(degree,maps[i].degree());homogeneous_maps[i]=homogenize(maps[i]); }
    require(degree==11&&global_identity(curve,maps),"actual global elliptic-map equation");precision=4*degree+1;homogeneous_curve=homogenize(curve,4);
    Cycle decoded=read_cycle(data.at("decoded").at("cycle"));Point result=norm(decoded);int decoded_poles=poles;
    residue_modulus=Poly{F(0),F(1)};Point reference;const auto& ids=formal.at("seed_indices").arr;require(ids.size()==3&&data.at("reference_points").arr.size()==3,"complete reference size");
    for(size_t i=0;i<3;++i) {
        const auto& p=pool.at("points").arr.at(size_t(ids[i].integer()));const auto& q=data.at("reference_points").arr[i];
        require(coefficient(p.at("x"))==coefficient(q.at("x"))&&coefficient(p.at("y"))==coefficient(q.at("y")),"actual reference coordinates bound");
        reference=add(reference,image(K(coefficient(p.at("x"))),K(coefficient(p.at("y")))));
    }
    result=add(result,negative(reference));auto us=std::chrono::duration_cast<std::chrono::microseconds>(std::chrono::steady_clock::now()-begin).count();
    std::cout<<"{\"scope\":\"actual-f-pushforward-without-expected-answer\",\"elliptic_result\":";emit(result);
    std::cout<<",\"expected_answer_read\":false,\"generating_scalar_read\":false,\"decoded_map_pole_charts\":"<<decoded_poles<<",\"factor_instances_with_multiplicity\":"<<instances<<",\"wall_us\":"<<us<<",\"signed_independent_verdict\":false}\n";
} catch(const std::exception& e) { std::cerr<<"REJECT "<<e.what()<<'\n';return 1; } }
