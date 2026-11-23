#include "../native/api.hpp"
#include <nlohmann/json.hpp>
#include <filesystem>
#include <fstream>
#include <iostream>
#include <random>
using namespace paper_ec;using J=nlohmann::json;
int main(int argc,char**argv){try{
 if(argc!=4){throw std::runtime_error("make-hot-confirmation SCALING_PUBLIC NEW_PUBLIC NEW_PRIVATE");}if(std::filesystem::exists(argv[2])||std::filesystem::exists(argv[3])){throw std::runtime_error("output exists");}std::vector<std::filesystem::path> paths;for(const auto&e:std::filesystem::directory_iterator(argv[1])){std::ifstream in(e.path());J f;in>>f;if(f.at("B")==640)paths.push_back(e.path());}std::sort(paths.begin(),paths.end());if(paths.size()!=48){throw std::runtime_error("p31 source count");}std::filesystem::create_directories(argv[2]);std::mt19937_64 random(0x484f54504f4f4c31ULL);J validation=J::array();
 for(const auto&path:paths)for(unsigned child=0;child<4;++child){std::ifstream in(path);J f;in>>f;const auto&c=f.at("curve");const Curve curve{c.at("p"),c.at("a"),c.at("b")};const Point g{c.at("P")[0],c.at("P")[1],false};const Word k=std::uniform_int_distribution<Word>(1,c.at("r").get<Word>()-1)(random);const Point q=point_mul(curve,k,g);if(q.infinity||!on_curve(curve,q)){throw std::runtime_error("public target");}f["curve"]["Q"]={q.x,q.y};f["id"]=f.at("id").get<std::string>()+"-hot-t"+std::to_string(child);f["public_seed"]=random()%1000000000ULL;std::ofstream out(std::filesystem::path(argv[2])/(f.at("id").get<std::string>()+".json"));out<<f.dump()<<'\n';validation.push_back({{"id",f.at("id")},{"scalar",k}});}
 std::ofstream out(argv[3]);out<<validation.dump(2)<<'\n';std::cout<<J({{"targets",validation.size()},{"field",2147483647},{"curves",3},{"targets_per_curve",64},{"scope","given public input creation, excluded identically; private scalar record never passed to solvers"}}).dump()<<'\n';
}catch(const std::exception&e){std::cerr<<e.what()<<'\n';return 1;}}
