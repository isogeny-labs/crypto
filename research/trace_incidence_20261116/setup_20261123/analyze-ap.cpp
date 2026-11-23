#include <nlohmann/json.hpp>
#include "../native/api.hpp"
#include <filesystem>
#include <fstream>
#include <iostream>
#include <algorithm>
#include <map>
using J=nlohmann::json;using namespace paper_ec;
void need(bool b,const char*s){if(!b)throw std::runtime_error(s);}
J stats(std::vector<double> a){need(!a.empty(),"empty stats");std::sort(a.begin(),a.end());const auto n=a.size();double total=0;for(double x:a)total+=x;return {{"n",n},{"median",n%2?a[n/2]:(a[n/2]+a[n/2-1])/2},{"min",a.front()},{"max",a.back()},{"mean",total/n}};}
struct G{std::vector<double> wall,cpu,after,parse,rss,attempts,draws,context,generation,selection;unsigned solved=0;};
int main(int argc,char**argv){try{
 need(argc==4,"analyze-target ROWS PUBLIC_FIXTURES SUMMARY");std::ifstream in(argv[1]);std::string line;unsigned raw=0,warmups=0,replayed=0,generation_checks=0;
 std::map<std::string,J> fixtures;for(const auto&e:std::filesystem::directory_iterator(argv[2])){std::ifstream f(e.path());J j;f>>j;fixtures[e.path().filename().string()]=j;}need(fixtures.size()==60,"target panel");
 std::map<std::string,G> groups;std::map<std::string,std::map<std::string,J>> paired;
 while(std::getline(in,line)){
  J row=J::parse(line);++raw;const int round=row.at("round");const std::string arm=row.at("arm_label"),name=row.at("source_fixture");const auto&fixture=fixtures.at(name);const auto&c=fixture.at("curve");const int B=fixture.at("B");
  need(round>=0&&round<=5&&row.at("wrapper_exit")==0,"round/exit failure");need(row.at("seed")==fixture.at("public_seed"),"seed mismatch");
  const bool rho=arm.rfind("rho-",0)==0;const std::string status=row.at("status");const bool solved=status=="relation"||status=="solved";need(status=="relation"||status=="none"||status=="solved"||status=="censored","failed row");
  double wall=0;unsigned draws=0,attempts=1;
  if(rho)wall=row.at("complete_adapter_wall_seconds");else{
   wall=row.at("complete").at("wall_seconds");const auto&trials=row.at("trials");attempts=trials.size();need(attempts>0&&attempts<=4&&(solved||attempts==4),"attempt tail");
   for(unsigned i=0;i<attempts;++i){const auto&t=trials[i];need(t.at("attempt")==i&&t.at("seed").get<Word>()==fixture.at("public_seed").get<Word>()+Word(i)*2654435761ULL,"trial identity");const unsigned scale=(arm=="asym2"||arm=="ap2")?2:(arm=="asym4"||arm=="ap4")?4:(arm=="asym8"||arm=="ap8")?8:arm=="grow"?i+1:1;
    need(t.at("sizes")==J::array({B,B,B*scale}),"pool sizes");const auto accepted=arm=="grow"&&i>0?B:B*(2+scale);need(t.at("draws").get<Word>()==Word(accepted)+t.at("rejected_infinity").get<Word>()+t.at("rejected_coordinate").get<Word>(),"generation accounting");draws+=t.at("draws").get<unsigned>();++generation_checks;
    if(arm.rfind("ap",0)==0){const auto&e=t.at("ap_counts");need(e.at("seed_pairs").get<Word>()==6+e.at("step_infinity").get<Word>()&&e.at("projective_additions")==t.at("draws")&&e.at("affine_batches").get<Word>()>=3,"uncharged progression construction");}
    need(t.at("status")==((solved&&i+1==attempts)?"relation":"none"),"invalid stopping");
   }
  }
  if(solved){const Curve ec{c.at("p"),c.at("a"),c.at("b")};const Point p{c.at("P")[0],c.at("P")[1],false},q{c.at("Q")[0],c.at("Q")[1],false};need(point_mul(ec,row.at("recovered_scalar").get<Word>(),p)==q,"independent scalar replay failed");++replayed;}
  if(round==0){++warmups;continue;}const auto key=std::to_string(round)+"/"+name;need(!paired[key].count(arm),"duplicate target-arm");row["analysis_wall"]=wall;row["analysis_solved"]=solved;row["analysis_B"]=B;paired[key][arm]=row;
  auto&g=groups[arm+"/"+std::to_string(B)];g.wall.push_back(wall);g.attempts.push_back(attempts);g.solved+=solved;g.rss.push_back(row.at("process_peak_rss_kib"));
  if(!rho){g.cpu.push_back(row.at("complete").at("cpu_seconds"));g.after.push_back(row.at("after_input_ingestion").at("wall_seconds"));g.parse.push_back(row.at("input_ingestion").at("wall_seconds"));g.draws.push_back(draws);g.context.push_back(row.at("context_total").at("wall_seconds"));g.generation.push_back(row.at("generation").at("wall_seconds"));g.selection.push_back(row.at("selection").at("wall_seconds"));}
 }
 need(raw==5040&&warmups==840&&paired.size()==300,"incomplete panel");J out={{"schema","trace-target-lifetime-summary/v1"},{"raw_rows",raw},{"warmup_rows",warmups},{"measured_units",raw-warmups},{"scalar_replays",replayed},{"generation_checks",generation_checks},{"standing","unsigned producer checks"}};
 for(auto&[k,g]:groups){need(g.wall.size()==60,"group count");out["groups"][k]={{"complete_wall_s",stats(g.wall)},{"attempts",stats(g.attempts)},{"peak_rss_kib",stats(g.rss)},{"solved",g.solved},{"censored",g.wall.size()-g.solved}};if(!g.cpu.empty()){auto&o=out["groups"][k];o["complete_cpu_s"]=stats(g.cpu);o["after_input_wall_s"]=stats(g.after);o["ingestion_wall_s"]=stats(g.parse);o["peak_rss_kib"]=stats(g.rss);o["draws"]=stats(g.draws);o["context_wall_s"]=stats(g.context);o["generation_wall_s"]=stats(g.generation);o["selection_wall_s"]=stats(g.selection);}}
 std::map<std::string,std::vector<double>> ratios;unsigned equal=0;
 for(const auto&[key,p]:paired){(void)key;need(p.size()==14,"missing mode");++equal;
  for(const auto&arm:std::vector<std::string>{"reuse","asym4","ap1","ap2","ap4","ap8"})for(const auto&base:std::vector<std::string>{"reuse","asym4","rho-3-1","rho-3-8","rho-4-1","rho-4-8","rho-5-1","rho-5-8","rho-6-1","rho-6-8"}){
   if(arm==base) { continue; }
   const auto&x=p.at(base);const auto&y=p.at(arm);if(!x.at("analysis_solved").get<bool>()||!y.at("analysis_solved").get<bool>())continue;ratios[base+"_over_"+arm+"/"+std::to_string(x.at("analysis_B").get<int>())].push_back(x.at("analysis_wall").get<double>()/y.at("analysis_wall").get<double>());
  }
 }
 for(auto&[key,r]:ratios){out["paired_completed_ratios"][key]=stats(r);}out["complete_target_round_units"]=equal;
 out["interpretation"]="Charged progression-pool complete-target comparison against older sealed rho parameter grid, shared local CPU. Includes original-input selective parsing and every attempted/rejected mixed label. Per-mode censoring retained; completed-pair ratios are conditional. No canonical online phase admission or strongest-current-rho claim.";
 std::ofstream file(argv[3]);file<<out.dump(2)<<'\n';std::cout<<J({{"rows",raw},{"scalar_replays",replayed},{"generation_checks",generation_checks},{"complete_target_round_units",equal},{"status","passed"}}).dump()<<'\n';
}catch(const std::exception&e){std::cerr<<e.what()<<'\n';return 1;}}
