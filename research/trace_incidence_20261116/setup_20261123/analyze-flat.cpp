#include <nlohmann/json.hpp>
#include "../native/api.hpp"
#include <filesystem>
#include <fstream>
#include <iostream>
#include <algorithm>
#include <map>
#include <random>
using J=nlohmann::json;using namespace paper_ec;
void need(bool b,const char*s){if(!b)throw std::runtime_error(s);}
J stats(std::vector<double> a){need(!a.empty(),"empty stats");std::sort(a.begin(),a.end());const auto n=a.size();double total=0;for(double x:a)total+=x;return {{"n",n},{"median",n%2?a[n/2]:(a[n/2]+a[n/2-1])/2},{"min",a.front()},{"max",a.back()},{"mean",total/n}};}
struct G{std::vector<double> wall,cpu,after,parse,rss,attempts,draws,context,generation,selection;unsigned solved=0;};
int main(int argc,char**argv){try{
 need(argc==5,"analyze-flat ROWS PUBLIC_FIXTURES PRIVATE_VALIDATION SUMMARY");std::ifstream in(argv[1]);std::string line;unsigned raw=0,warmups=0,replayed=0,generation_checks=0;
 std::map<std::string,J> fixtures;for(const auto&e:std::filesystem::directory_iterator(argv[2])){std::ifstream f(e.path());J j;f>>j;fixtures[e.path().filename().string()]=j;}need(fixtures.size()==480,"target panel");
 std::ifstream validation_input(argv[3]);J validation;validation_input>>validation;std::map<std::string,Word> expected;
 for(const auto&v:validation) { expected[v.at("id").get<std::string>()]=v.at("scalar").get<Word>(); }
 need(expected.size()==480,"known answer count");
 std::map<std::string,G> groups;std::map<std::string,std::map<std::string,J>> paired;
 while(std::getline(in,line)){
  J row=J::parse(line);++raw;const int round=row.at("round");const std::string arm=row.at("arm_label"),name=row.at("source_fixture");const auto&fixture=fixtures.at(name);const auto&c=fixture.at("curve");const int B=fixture.at("B");
  need(round>=0&&round<=5&&row.at("wrapper_exit")==0,"round/exit failure");need(row.at("seed")==fixture.at("public_seed"),"seed mismatch");
  const bool rho=arm.rfind("current-",0)==0||arm=="sealed-rho";const std::string mode=rho?"rho":row.at("mode").get<std::string>();need(row.at("fixture_id")==fixture.at("id"),"fixture id mismatch");const std::string status=row.at("status");const bool solved=status=="relation"||status=="solved";need(status=="relation"||status=="none"||status=="solved"||status=="censored","failed row");
  if(!rho){need(row.at("pair_kernel")=="barrett"&&row.at("timings_collected")==true,"kernel identity/timer mismatch");need(arm=="flat-ap4-aa"?mode=="ap4":arm.size()>=mode.size()&&arm.substr(arm.size()-mode.size())==mode,"mode identity");need(arm.rfind("tree-",0)==0||row.at("generator_container")=="flat","generator identity");}else{need(row.at("public_scalar_check").get<bool>()==solved,"rho verification flag");if(arm.rfind("current-",0)==0)need(row.at("configuration_policy")==arm.substr(8),"current rho policy mismatch");}
  double wall=0;unsigned draws=0,attempts=1;
  if(rho)wall=row.at("complete_adapter_wall_seconds");else{
   wall=row.at("complete").at("wall_seconds");const auto&trials=row.at("trials");attempts=trials.size();need(attempts>0&&attempts<=4&&(solved||attempts==4),"attempt tail");
   for(unsigned i=0;i<attempts;++i){const auto&t=trials[i];need(t.at("attempt")==i&&t.at("seed").get<Word>()==fixture.at("public_seed").get<Word>()+Word(i)*2654435761ULL,"trial identity");const unsigned scale=mode=="ap2"?2:mode=="ap4"?4:mode=="ap8"?8:1;
    need(t.at("sizes")==J::array({B,B,B*scale}),"pool sizes");const auto accepted=mode=="grow"&&i>0?B:B*(2+scale);need(t.at("draws").get<Word>()==Word(accepted)+t.at("rejected_infinity").get<Word>()+t.at("rejected_coordinate").get<Word>(),"generation accounting");draws+=t.at("draws").get<unsigned>();++generation_checks;
    if(mode.rfind("ap",0)==0){const auto&e=t.at("ap_counts");need(e.at("seed_pairs").get<Word>()==6+e.at("step_infinity").get<Word>()&&e.at("projective_additions")==t.at("draws")&&e.at("affine_batches").get<Word>()>=3,"uncharged progression construction");}
    need(t.at("status")==((solved&&i+1==attempts)?"relation":"none"),"invalid stopping");
   }
  }
  if(solved){need(row.at("recovered_scalar").get<Word>()==expected.at(fixture.at("id").get<std::string>()),"private known-answer mismatch");const Curve ec{c.at("p"),c.at("a"),c.at("b")};const Point p{c.at("P")[0],c.at("P")[1],false},q{c.at("Q")[0],c.at("Q")[1],false};need(point_mul(ec,row.at("recovered_scalar").get<Word>(),p)==q,"independent scalar replay failed");++replayed;}
  if(round==0){++warmups;continue;}const auto key=std::to_string(round)+"/"+name;need(!paired[key].count(arm),"duplicate target-arm");row["analysis_wall"]=wall;row["analysis_solved"]=solved;row["analysis_B"]=B;paired[key][arm]=row;
  auto&g=groups[arm+"/"+std::to_string(B)];g.wall.push_back(wall);g.attempts.push_back(attempts);g.solved+=solved;g.rss.push_back(row.at("process_peak_rss_kib"));
  if(!rho){g.cpu.push_back(row.at("complete").at("cpu_seconds"));g.after.push_back(row.at("after_input_ingestion").at("wall_seconds"));g.parse.push_back(row.at("input_ingestion").at("wall_seconds"));g.draws.push_back(draws);g.context.push_back(row.at("context_total").at("wall_seconds"));g.generation.push_back(row.at("generation").at("wall_seconds"));g.selection.push_back(row.at("selection").at("wall_seconds"));}
 }
 need(raw==31680&&warmups==5280&&paired.size()==2400,"incomplete panel");J out={{"schema","trace-target-lifetime-summary/v1"},{"raw_rows",raw},{"warmup_rows",warmups},{"measured_units",raw-warmups},{"scalar_replays",replayed},{"generation_checks",generation_checks},{"standing","unsigned producer checks"}};
 for(auto&[k,g]:groups){need(g.wall.size()==480,"group count");out["groups"][k]={{"complete_wall_s",stats(g.wall)},{"attempts",stats(g.attempts)},{"peak_rss_kib",stats(g.rss)},{"solved",g.solved},{"censored",g.wall.size()-g.solved}};if(!g.cpu.empty()){auto&o=out["groups"][k];o["complete_cpu_s"]=stats(g.cpu);o["after_input_wall_s"]=stats(g.after);o["ingestion_wall_s"]=stats(g.parse);o["peak_rss_kib"]=stats(g.rss);o["draws"]=stats(g.draws);o["context_wall_s"]=stats(g.context);o["generation_wall_s"]=stats(g.generation);o["selection_wall_s"]=stats(g.selection);}}
 std::map<std::string,std::vector<double>> ratios;unsigned equal=0;
 for(const auto&[key,p]:paired){(void)key;need(p.size()==11,"missing arm");
  for(const auto&mode:std::vector<std::string>{"ap2","ap4","ap8"}){need(p.at("tree-"+mode).at("trials")==p.at("flat-"+mode).at("trials"),"container changed trial sequence");++equal;}
  need(p.at("tree-ap4").at("trials")==p.at("flat-gen-ap4").at("trials")&&p.at("flat-ap4").at("trials")==p.at("flat-ap4-aa").at("trials"),"generator/AA changed trials");equal+=2;
  for(const auto&arm:std::vector<std::string>{"tree-ap2","tree-ap4","tree-ap8","flat-gen-ap4","flat-ap2","flat-ap4","flat-ap8","flat-ap4-aa"})for(const auto&base:std::vector<std::string>{"tree-ap2","tree-ap4","tree-ap8","flat-ap4","current-tuned8","current-tuned16","current-tuned"}){
   if(arm==base) { continue; }
   const auto&x=p.at(base);const auto&y=p.at(arm);if(!x.at("analysis_solved").get<bool>()||!y.at("analysis_solved").get<bool>())continue;ratios[base+"_over_"+arm+"/"+std::to_string(x.at("analysis_B").get<int>())].push_back(x.at("analysis_wall").get<double>()/y.at("analysis_wall").get<double>());
  }
 }
 for(auto&[key,r]:ratios){out["paired_completed_ratios"][key]=stats(r);}out["complete_target_round_units"]=equal;
 // Resample targets, not individual timing rounds; retain the curve strata.
 std::map<int,std::map<std::string,std::vector<std::string>>> strata;
 for(const auto&[name,f]:fixtures){const auto&c=f.at("curve");const std::string curve=c.at("p").dump()+"/"+c.at("a").dump()+"/"+c.at("b").dump();strata[f.at("B").get<int>()][curve].push_back(name);}
 auto cluster_mean=[&](const std::string&name,const std::string&arm){double sum=0;for(int round=1;round<=5;++round)sum+=paired.at(std::to_string(round)+"/"+name).at(arm).at("analysis_wall").get<double>();return sum/5;};
 for(const auto&[B,curves]:strata){need(curves.size()==3,"curve strata");for(const auto&[curve,names]:curves){(void)curve;need(names.size()==32,"target stratum size");}
  for(const auto&candidate:std::vector<std::string>{"tree-ap2","tree-ap4","tree-ap8","flat-gen-ap4","flat-ap2","flat-ap4","flat-ap8","flat-ap4-aa"})for(const auto&reference:std::vector<std::string>{"tree-ap2","tree-ap4","tree-ap8","flat-ap4","current-tuned8","current-tuned16","current-tuned"}){
   if(candidate==reference)continue;
   double ca=0,cb=0;unsigned incomplete=0;std::vector<std::vector<std::pair<double,double>>> values;
   for(const auto&[curve,names]:curves){(void)curve;std::vector<std::pair<double,double>> v;for(const auto&name:names){const double x=cluster_mean(name,candidate),y=cluster_mean(name,reference);v.push_back({x,y});ca+=x;cb+=y;for(int round=1;round<=5;++round){const auto&p=paired.at(std::to_string(round)+"/"+name);if(!p.at(candidate).at("analysis_solved").get<bool>()||!p.at(reference).at("analysis_solved").get<bool>())++incomplete;}}values.push_back(v);}
   std::mt19937_64 random(0x464c41544349ULL);std::vector<double> boot;boot.reserve(10000);
   for(unsigned rep=0;rep<10000;++rep){double sa=0,sb=0;for(const auto&v:values)for(std::size_t i=0;i<v.size();++i){const auto&xy=v[std::uniform_int_distribution<std::size_t>(0,v.size()-1)(random)];sa+=xy.first;sb+=xy.second;}boot.push_back(sa/sb);}std::sort(boot.begin(),boot.end());
   out["target_cluster_mean_ratios"][candidate+"_over_"+reference+"/"+std::to_string(B)]={{"ratio",ca/cb},{"bootstrap_q025",boot.at(249)},{"bootstrap_q95",boot.at(9499)},{"bootstrap_q975",boot.at(9749)},{"target_clusters",96},{"replicates",10000},{"incomplete_target_round_pairs",incomplete},{"scoped_wall_parity_criterion",incomplete==0&&boot.at(9499)<=1.0}};
  }
 }
 out["interpretation"]="Fresh480-target confirmation; all exact trial streams compared; every recovered scalar checked against independent affine arithmetic and private known answer. Full cold adapter costs including input/validation, labels, misses, indexes and recovery. Target-cluster stratified bootstrap, bounded costs with censors retained. Current rho module uses older compatible adapter crate; one unrelated root-oracle suite mismatch remains. Wall times on shared local CPU, no counted-operation admission, strongest-global-rho or Pareto claim.";
 std::ofstream file(argv[4]);file<<out.dump(2)<<'\n';std::cout<<J({{"rows",raw},{"scalar_replays",replayed},{"generation_checks",generation_checks},{"complete_target_round_units",equal},{"status","passed"}}).dump()<<'\n';
}catch(const std::exception&e){std::cerr<<e.what()<<'\n';return 1;}}
