// Synthetic SOFTWARE TESTS. The nanoseconds here are fabricated constants,
// never benchmark observations and never inputs to a research speed claim.
#define main cm_analysis_main
#include "analysis.cpp"
#undef main
#include <cstdio>
#include <unistd.h>
int main(){try{
 char path[]="/tmp/cm-analysis-test-XXXXXX";int fd=mkstemp(path);if(fd<0)throw std::runtime_error("mkstemp");close(fd);
 struct Cleanup{const char* p;~Cleanup(){std::remove(p);}}cleanup{path};
 std::vector<std::string> rows;
 for(auto curve:{"P-384","P-521"})for(unsigned w=2;w<=7;++w)for(auto arm:{"window","window-aa","maps","fused","openssl-public","openssl-public-aa","openssl-single"})for(unsigned p=0;p<4;++p)for(unsigned r=0;r<=6;++r){
  std::ostringstream s;s<<"{\"curve\":\""<<curve<<"\",\"window\":"<<w<<",\"arm\":\""<<arm<<"\",\"point\":"<<p<<",\"round\":"<<r<<",\"inner\":2,\"warmup\":"<<(r?"false":"true")<<",\"panel\":\"calibration\",\"correct\":true,\"total_ns\":100,\"setup_ns\":10,\"maps_ns\":10,\"group_ns\":50,\"verify_ns\":10,\"bookkeeping_ns\":20,\"output\":\"synthetic-software-test\"}";rows.push_back(s.str());
 }
 auto write=[&](const std::vector<std::string>& input){std::ofstream out(path);for(auto& row:input)out<<row<<'\n';};
 write(rows);auto positive=load(path,"calibration");if(positive.size()!=2)throw std::runtime_error("positive design");
 unsigned rejected=0;auto reject=[&](const std::vector<std::string>& input){write(input);try{load(path,"calibration");}catch(const std::exception&){++rejected;return;}throw std::runtime_error("mutation accepted");};
 auto missing=rows;missing.pop_back();reject(missing);auto duplicate=rows;duplicate.push_back(rows.front());reject(duplicate);
 for(const auto& replacement:std::vector<std::pair<std::string,std::string>>{
  {"\"total_ns\":100","\"total_ns\":100.5"},
  {"\"total_ns\":100","\"total_ns\":-100"},
  {"\"total_ns\":100","\"total_ns\":18446744073709551616"},
  {"\"setup_ns\":10","\"setup_ns\":18446744073709551615"},
  {"\"correct\":true","\"correct\":false"},
  {"\"panel\":\"calibration\"","\"panel\":\"heldout\""},
  {"\"warmup\":true","\"warmup\":false"},
  {"\"point\":0","\"point\":0.5"},
  {"synthetic-software-test","altered-output"}}){
   auto mutated=rows;auto where=mutated[0].find(replacement.first);if(where==std::string::npos)throw std::runtime_error("mutation site");mutated[0].replace(where,replacement.first.size(),replacement.second);reject(mutated);
 }
 std::cout<<"{\"software_test\":true,\"synthetic_rows\":"<<rows.size()<<",\"positive_design\":true,\"mutations_rejected\":"<<rejected<<",\"research_timing_performed\":false}\n";return 0;
}catch(const std::exception& e){std::cerr<<e.what()<<'\n';return 1;}}
