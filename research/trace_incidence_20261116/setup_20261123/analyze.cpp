#include <nlohmann/json.hpp>
#include "../native/api.hpp"
#include <filesystem>
#include <fstream>
#include <iostream>
#include <algorithm>
#include <map>
#include <set>
using J=nlohmann::json;using namespace paper_ec;
void need(bool b,const char*s){if(!b)throw std::runtime_error(s);}
J stats(std::vector<double> a){need(!a.empty(),"empty stats");std::sort(a.begin(),a.end());const auto n=a.size();double total=0;for(double x:a)total+=x;return {{"n",n},{"median",n%2?a[n/2]:(a[n/2]+a[n/2-1])/2},{"min",a.front()},{"max",a.back()},{"mean",total/n}};}
struct Unit{int B=0,round=0,attempts=0;std::string arm,fixture;bool rho=false,solved=false;double wall=0,cpu=0,after=0,parse=0,rss=0;};
struct Group{std::vector<double> wall,cpu,after,parse,rss,attempts;unsigned solved=0;};
int main(int argc,char**argv){try{
 need(argc==4,"analyze ROWS MINIMAL_FIXTURES NEW_SUMMARY");std::ifstream in(argv[1]);std::string line;unsigned raw=0,warmup_rows=0,replayed=0,agreements=0;
 std::map<std::string,J> fixtures;for(const auto&e:std::filesystem::directory_iterator(argv[2])){std::ifstream f(e.path());J j;f>>j;fixtures[e.path().filename().string()]=j;}need(fixtures.size()==60,"target panel");
 std::map<std::string,Unit> units;std::map<std::string,J> agreement;
 while(std::getline(in,line)){
  J row=J::parse(line);++raw;const int round=row.at("round"),attempt=row.at("attempt");const std::string arm=row.at("arm_label"),name=row.at("source_fixture");
  need(round>=0&&round<=5&&row.at("wrapper_exit")==0,"round/exit failure");if(round==0)++warmup_rows;
  const auto&f=fixtures.at(name);const auto&c=f.at("curve");const Word seed=f.at("public_seed").get<Word>()+Word(attempt)*2654435761ULL;
  need(row.at("seed").get<Word>()==seed,"attempt seed mismatch");
  const std::string key=std::to_string(round)+"/"+name+"/"+arm;auto&u=units[key];need(!u.solved&&attempt==u.attempts,"invalid attempt order");
  u.B=f.at("B");u.round=round;u.arm=arm;u.fixture=name;u.rho=arm.rfind("rho/",0)==0;u.attempts++;
  const std::string status=row.at("status");need(status=="relation"||status=="none"||status=="solved"||status=="censored","scientific failure");u.solved=status=="relation"||status=="solved";
  if(u.rho){need(attempt==0,"rho repeated attempt");u.wall+=row.at("complete_adapter_wall_seconds").get<double>();}
  else{
   const auto&t=row.at("complete");u.wall+=t.at("wall_seconds").get<double>();u.cpu+=t.at("cpu_seconds").get<double>();u.after+=row.at("after_input_ingestion").at("wall_seconds").get<double>();u.parse+=row.at("input_ingestion").at("wall_seconds").get<double>();u.rss=std::max(u.rss,row.at("peak_rss_kib").get<double>());
   const J signature={{"generation",row.at("generation")},{"status",status},{"scalar",u.solved?row.at("recovered_scalar"):J(nullptr)}};
   const auto akey=std::to_string(round)+"/"+name+"/"+std::to_string(attempt);if(agreement.count(akey)){need(agreement.at(akey)==signature,"IC generation/outcome mismatch");++agreements;}else agreement[akey]=signature;
  }
  if(u.solved){const Curve ec{c.at("p"),c.at("a"),c.at("b")};const Point p{c.at("P")[0],c.at("P")[1],false},q{c.at("Q")[0],c.at("Q")[1],false};need(point_mul(ec,row.at("recovered_scalar").get<Word>(),p)==q,"independent scalar replay failed");++replayed;}
 }
 need(units.size()==2520,"incomplete target-round-arm panel");std::map<std::string,Group> groups;std::map<std::string,std::map<std::string,Unit>> paired;
 for(const auto&[key,u]:units){(void)key;need(u.solved||u.attempts==(u.rho?1:4),"incomplete attempt tail");if(u.round==0)continue;auto&g=groups[u.arm+"/"+std::to_string(u.B)];g.wall.push_back(u.wall);g.attempts.push_back(u.attempts);g.solved+=u.solved;if(!u.rho){g.cpu.push_back(u.cpu);g.after.push_back(u.after);g.parse.push_back(u.parse);g.rss.push_back(u.rss);}paired[std::to_string(u.round)+"/"+u.fixture][u.arm]=u;}
 J out={{"schema","trace-public-setup-summary/v1"},{"raw_rows",raw},{"warmup_rows",warmup_rows},{"measured_rows",raw-warmup_rows},{"target_round_arm_units",units.size()},{"replayed_scalars",replayed},{"generation_outcome_agreements",agreements},{"standing","unsigned producer checks"}};
 for(auto&[key,g]:groups){need(g.wall.size()==60,"group size mismatch");out["groups"][key]={{"complete_wall_s",stats(g.wall)},{"attempts",stats(g.attempts)},{"solved",g.solved},{"censored",g.wall.size()-g.solved}};if(!g.cpu.empty()){out["groups"][key]["complete_cpu_s"]=stats(g.cpu);out["groups"][key]["after_input_wall_s"]=stats(g.after);out["groups"][key]["ingestion_wall_s"]=stats(g.parse);out["groups"][key]["peak_rss_kib"]=stats(g.rss);}}
 std::map<std::string,std::vector<double>> ratios,after_ratios;
 const std::vector<std::pair<std::string,std::string>> compare={{"prior/full","optimized/full"},{"prior/minimal","optimized/minimal"},{"prior/full","prior/minimal"},{"optimized/full","optimized/minimal"},{"optimized/minimal","rho/minimal"},{"optimized/full","rho/full"},{"pair-control/minimal","optimized/minimal"},{"rho/full","rho/minimal"}};
 for(const auto&[key,p]:paired){(void)key;need(p.size()==7,"missing arm in target-round");for(const auto&[a,b]:compare){const auto&x=p.at(a);const auto&y=p.at(b);if(!x.solved||!y.solved)continue;const auto k=a+"_over_"+b+"/"+std::to_string(x.B);ratios[k].push_back(x.wall/y.wall);if(!x.rho&&!y.rho)after_ratios[k].push_back(x.after/y.after);}}
 for(auto&[key,r]:ratios) { out["paired_complete_ratios"][key]=stats(r); }
 for(auto&[key,r]:after_ratios) { out["paired_after_input_ratios"][key]=stats(r); }
 out["interpretation"]="Legacy complete-call descriptive comparison. After-input IC and rho engine/adapter intervals differ; no canonical online parity or strongest-current-rho assertion. Censored units retained in bounded costs; paired ratios use completed pairs only.";
 std::ofstream file(argv[3]);file<<out.dump(2)<<'\n';std::cout<<J({{"raw",raw},{"units",units.size()},{"scalar_replays",replayed},{"generation_agreements",agreements},{"status","passed"}}).dump()<<'\n';
}catch(const std::exception&e){std::cerr<<e.what()<<'\n';return 1;}}
