#include "relation_openssl_arithmetic.hpp"
int main(int argc,char** argv){try{
    auto start=std::chrono::steady_clock::now();require(argc==2,"usage: transfer_openssl_check transfer.json");auto data=read_json(argv[1]);OpenSSLGroup ec;auto& rows=data.at("fixtures").arr;require(rows.size()==6,"complete fresh transfer panel");
    for(auto& row:rows){auto source=ec.point(read_point(row.at("point"))),norm=ec.point(read_point(row.at("norm")));ec.composition(source,norm);}
    require(read_point(rows[0].at("point")).infinity,"zero transfer input");auto g=ec.point(read_point(rows[1].at("point"))),negative=ec.point(read_point(rows[2].at("point"))),sum=ec.empty();ec.add(sum,g,1);ec.add(sum,negative,1);require(ec.zero(sum),"OpenSSL negative generator fixture");
    auto twice=ec.point(read_point(rows[3].at("point"))),neg_twice=ec.point(read_point(rows[5].at("point")));sum=ec.empty();ec.add(sum,twice,1);ec.add(sum,neg_twice,1);require(ec.zero(sum),"OpenSSL exceptional negative2G fixture");
    must_reject([&]{auto source=ec.point(read_point(rows[1].at("point"))),norm=ec.point(read_point(rows[1].at("norm")));ec.add(norm,g,1);ec.composition(source,norm);});
    auto us=std::chrono::duration_cast<std::chrono::microseconds>(std::chrono::steady_clock::now()-start).count();
    std::cout<<"{\"standing\":\"producer-unverified separate arithmetic cross-check\",\"fresh_companion_checks\":6,\"changed_norm_control_rejected\":true,\"exceptional_negative2G_binding\":true,\"wall_us\":"<<us<<",\"independent_geometric_verification\":false}\n";
}catch(const std::exception& e){std::cerr<<"REJECT "<<e.what()<<'\n';return 1;}}
