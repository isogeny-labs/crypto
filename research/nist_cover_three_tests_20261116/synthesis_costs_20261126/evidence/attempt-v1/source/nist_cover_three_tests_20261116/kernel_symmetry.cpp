#include "quotient_arithmetic.hpp"
P polynomial_power(P a,int n){P out{F(1)};for(int i=0;i<n;++i)out=out*a;return out;}
int main(int argc,char** argv){try{
    require(argc==2,"usage: kernel_symmetry study-directory");std::string root=argv[1];initialize_cover(read_json(root+"/inputs/p192-d7-prym.json"));
    P x{F(0),F(1)},den{F(1),F(2)*cc},transformed;
    for(int i=0;i<=curve.degree();++i)transformed=transformed+(polynomial_power(-x,i)*polynomial_power(den,8-i)).scale(curve.at(i));
    require(transformed==curve,"actual normalized cover involution equation");
    std::ifstream in(root+"/evidence/final-kernel-check/kernels.jsonl");std::string line;std::map<int,std::set<V>> kernels;
    while(std::getline(in,line)){Parser parser{line};auto row=parser.value();Basis b;for(auto& a:row.at("rref_basis").arr){V v{};for(int i=0;i<6;++i)v[i]=a.arr[i].integer();b.push_back(v);}auto all=elements(b);require(all.size()==27,"archived kernel cardinality");kernels.emplace(row.at("kernel").integer(),std::set<V>(all.begin(),all.end()));}
    require(kernels.size()==8,"all eight archived kernels");std::map<int,int> permutation;
    for(auto& [id,all]:kernels){std::set<V> image;for(auto v:all){for(int i=2;i<6;++i)v[i]=m3(-v[i]);image.insert(v);}int match=0,count=0;for(auto& [other,set]:kernels)if(image==set){match=other;++count;}require(count==1,"unique involution kernel image");permutation.emplace(id,match);}
    for(auto& [a,b]:permutation)require(permutation.at(b)==a,"kernel permutation involution");
    std::set<int> visited;std::vector<std::vector<int>> orbits;for(auto& [a,b]:permutation)if(!visited.count(a)){visited.insert(a);visited.insert(b);std::vector<int> orbit{a};if(b!=a)orbit.push_back(b);orbits.push_back(orbit);}
    rejects([&]{auto bad=permutation;bad[1]=1;require(bad.at(bad.at(2))==2&&bad.at(bad.at(1))==1,"corrupt permutation");},"changed kernel involution");
    std::cout<<"{\"scope\":\"P192-d7-recorded-kernel-cover-involution\",\"curve_identity_checked\":true,\"kernel_vectors_checked\":216,\"standing\":\"producer-unverified\",\"permutation\":[";bool first=true;for(auto& [a,b]:permutation){if(!first)std::cout<<',';first=false;std::cout<<'['<<a<<','<<b<<']';}
    std::cout<<"],\"orbits\":[";for(size_t i=0;i<orbits.size();++i){if(i)std::cout<<',';std::cout<<'[';for(size_t j=0;j<orbits[i].size();++j)std::cout<<(j?",":"")<<orbits[i][j];std::cout<<']';}std::cout<<"]}\n";
}catch(const std::exception& e){std::cerr<<"REJECT "<<e.what()<<'\n';return 1;}}
