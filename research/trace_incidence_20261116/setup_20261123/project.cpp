#include <nlohmann/json.hpp>
#include <filesystem>
#include <fstream>
#include <iostream>
using J=nlohmann::json;
int main(int argc,char**argv){try{
 if(argc!=3)throw std::runtime_error("project SOURCE_FIXTURES NEW_OUTPUT");
 const std::filesystem::path dest(argv[2]);if(std::filesystem::exists(dest))throw std::runtime_error("output exists");std::filesystem::create_directories(dest);
 unsigned count=0;std::uint64_t before=0,after=0;
 for(const auto&e:std::filesystem::directory_iterator(argv[1])){
  const auto name=e.path().filename().string();if(name.find("_natural_s")==std::string::npos||name.find("validation")!=std::string::npos||e.path().extension()!=".json")continue;
  std::ifstream in(e.path());J j;in>>j;
  if(j.at("schema")!="prime-pools/v1"||j.at("kind")!="natural")throw std::runtime_error("bad schema");
  J out;for(const char*k:{"schema","kind","id","B","curve","public_seed"})out[k]=j.at(k);
  const auto bytes=out.dump()+"\n";std::ofstream file(dest/name);file<<bytes;
  before+=std::filesystem::file_size(e.path());after+=bytes.size();++count;
 }
 if(count!=60)throw std::runtime_error("wrong target count");
 std::cout<<J({{"fixtures",count},{"source_bytes",before},{"projected_bytes",after},{"public_fields_preserved",true},{"scope","per-input public projection, no arithmetic or known scalar used"}}).dump()<<'\n';
}catch(const std::exception&e){std::cerr<<e.what()<<'\n';return 1;}}
