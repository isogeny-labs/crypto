#include "context.hpp"
int main(int argc,char** argv) { try {
    auto begin=std::chrono::steady_clock::now();require(argc==10,"usage: prepare cover model relations formal actual complete boundary transfer output-dir");
    initialize_cover(read_json(argv[1]));Level2Input level;Context c(argv);std::string dir=argv[9];
    auto zero=[] { std::cout<<"{\"type\":\"zero\"}"; };
    for(int m=0;m<=3;++m)fixture(dir,"principal-m"+std::to_string(m),shift(c.km,c.km.identity,-m),zero);
    int p=c.km.fibers[1].support[0],anchor=c.km.fibers[0].support[0];Class nonzero=c.km.point_class(p);require(!c.km.zero(nonzero),"nonzero fixed point-class fixture");
    for(int m=1;m<=2;++m)fixture(dir,"nonzero-m"+std::to_string(m),shift(c.km,nonzero,-m),[&] {
        std::cout<<"{\"type\":\"point_difference\",\"positive\":";point_json(c.km.points[size_t(p)]);std::cout<<",\"negative\":";point_json(c.km.points[size_t(anchor)]);std::cout<<'}';
    });
    auto complete=read_json(argv[6]),boundary=read_json(argv[7]),transfer=read_json(argv[8]);Class cg=class_input(complete.at("class_spaces").arr.at(2),c.km);
    auto public_expected=[&](int multiple) {
        const auto& g=transfer.at("fixtures").arr.at(1).at("point");require(g.at("infinity").scalar=="false","nonzero public G fixture");
        std::cout<<"{\"type\":\"public_multiple\",\"multiple\":"<<multiple<<",\"point\":{\"infinity\":false,\"x\":\""<<g.at("x").scalar<<"\",\"y\":\""<<g.at("y").scalar<<"\"}}";
    };
    fixture(dir,"public-FG",cg,[&] { public_expected(6); });fixture(dir,"public-double-FG",c.km.add(cg,cg),[&] { public_expected(12); });
    std::array<std::string,3> names{"special-rr2","triple-reference","double-single-reference"};
    for(size_t i=0;i<3;++i) {
        const auto& b=boundary.at("cases").arr.at(i);Class input=class_input(b.at("input_class"),c.km);MCycle cycle=stored_cycle(b.at("input_cycle"));
        fixture(dir,names[i],input,[&] { std::cout<<"{\"type\":\"cycle\",\"rr_dimension\":"<<(i?1:2)<<",\"exact_support_required\":"<<(i?"true":"false")<<",\"cycle\":";cm_emit_cycle(cycle);std::cout<<'}'; });
    }
    MCycle pole=ob_cycle(c.f,{c.km.points[0]},{3});Class pole_class=c.adapter.encode(pole,1).value;
    fixture(dir,"triple-pole",pole_class,[&] { std::cout<<"{\"type\":\"cycle\",\"rr_dimension\":1,\"exact_support_required\":true,\"pole_required\":true,\"cycle\":";cm_emit_cycle(pole);std::cout<<'}'; });
    Class bad=cg;bad.w[0][0]=bad.w[0][0]+Z(1);fixture(dir,"off-ambient",bad,zero);
    auto us=std::chrono::duration_cast<std::chrono::microseconds>(std::chrono::steady_clock::now()-begin).count();
    std::cout<<"{\"valid_cases\":12,\"negative_cases\":1,\"wall_us\":"<<us<<",\"target_addflips\":"<<c.km.flips<<"}\n";
} catch(const std::exception& e) { std::cerr<<"REJECT "<<e.what()<<'\n';return 1; } }
