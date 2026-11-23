#include <nlohmann/json.hpp>
#include "../native/api.hpp"
#include <filesystem>
#include <fstream>
#include <iostream>
#include <algorithm>
#include <random>
using J=nlohmann::json;using namespace paper_ec;
int main(int argc,char**argv){try{
 if(argc!=4)throw std::runtime_error("make-holdout SOURCE_PUBLIC_FIXTURES NEW_PUBLIC_DIR NEW_VALIDATION_FILE");
 if(std::filesystem::exists(argv[2])||std::filesystem::exists(argv[3])) { throw std::runtime_error("output exists"); }
 std::filesystem::create_directories(argv[2]);
 std::vector<std::filesystem::path> paths;for(const auto&e:std::filesystem::directory_iterator(argv[1]))paths.push_back(e.path());std::sort(paths.begin(),paths.end());if(paths.size()!=60)throw std::runtime_error("source target count");
 std::mt19937_64 random(0x484f4c444f555431ULL);J validation=J::array();
 for(const auto&path:paths){std::ifstream in(path);J j;in>>j;auto&c=j.at("curve");const Curve curve{c.at("p"),c.at("a"),c.at("b")};const Point p{c.at("P")[0],c.at("P")[1],false};const Word r=c.at("r");const Word k=std::uniform_int_distribution<Word>(1,r-1)(random);const Point q=point_mul(curve,k,p);if(q.infinity||!on_curve(curve,q))throw std::runtime_error("invalid holdout point");
  c["Q"]={q.x,q.y};j["id"]=j.at("id").get<std::string>()+"-holdout20261123";j["public_seed"]=random()%1000000000ULL;std::ofstream out(std::filesystem::path(argv[2])/path.filename());out<<j.dump()<<'\n';validation.push_back({{"id",j.at("id")},{"scalar",k}});
 }
 std::ofstream v(argv[3]);v<<validation.dump(2)<<'\n';std::cout<<J({{"targets",paths.size()},{"public_format","only given curve/order/P/Q/seed; no frozen pools or known scalar"},{"validation_path",argv[3]},{"construction_excluded_from_solver_times","same public DLP inputs supplied to all arms"}}).dump()<<'\n';
}catch(const std::exception&e){std::cerr<<e.what()<<'\n';return 1;}}
