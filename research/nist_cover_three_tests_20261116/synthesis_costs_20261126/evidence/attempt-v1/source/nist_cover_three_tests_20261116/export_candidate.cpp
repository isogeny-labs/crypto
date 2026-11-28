// Metadata export only; arithmetic and model coefficients come from native descent.
#include "json.hpp"
#include <iostream>
int main(int argc,char** argv){try{
    require(argc==3,"usage: export_candidate prym.json descent.json");auto input=read_json(argv[1]),result=read_json(argv[2]);
    std::cout<<"{\"schema\":\"candidate.plane-quartic/v1\",\"p\":\""<<input.at("source_model").at("p").scalar<<"\",\"homogeneous_variables\":[\"X\",\"Y\",\"Z\"],\"terms\":[";
    bool first=true;
    for(auto& row:result.at("quartic_Fp").arr){if(!first)std::cout<<',';first=false;std::cout<<"{\"exponents\":[";for(int i=0;i<3;++i)std::cout<<(i?",":"")<<row.at("exponents").arr.at(i).integer();std::cout<<"],\"coefficient\":\""<<row.at("coefficient").scalar<<"\"}";}
    std::cout<<"],\"source_cover\":\"P192-d7\",\"kernel\":1,\"origin\":\"actual quotient theta2 vector followed by Riemann reconstruction and explicit curve descent\",\"standing\":\"producer-unverified\",\"Serre_twist_checked\":false,\"divisor_map_constructed\":false}\n";
}catch(const std::exception&e){std::cerr<<"REJECT "<<e.what()<<'\n';return 1;}}
