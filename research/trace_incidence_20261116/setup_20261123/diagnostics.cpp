// Supplemental accounting, not a replacement for frozen acceptance analysis.
#include <nlohmann/json.hpp>
#include <filesystem>
#include <fstream>
#include <iostream>
#include <map>
#include <set>
using J=nlohmann::json;
int main(int argc,char**argv){try{
 if(argc!=4) { throw std::runtime_error("diagnostics ROWS PUBLIC_DIR OUTPUT"); }
 J out;std::map<unsigned,std::set<std::string>> distinct;std::map<unsigned,unsigned> total;
 for(const auto&e:std::filesystem::directory_iterator(argv[2])){std::ifstream in(e.path());J f;in>>f;const unsigned B=f.at("B");const auto&c=f.at("curve");const std::string identity=c.at("p").dump()+"/"+c.at("a").dump()+"/"+c.at("b").dump()+"/"+c.at("Q").dump();distinct[B].insert(identity);++total[B];out["field_prime_by_B"][std::to_string(B)]=c.at("p");}
 for(const auto&[B,n]:total)out["public_sampling"][std::to_string(B)]={{"sampled_targets",n},{"distinct_curve_and_Q",distinct[B].size()},{"repeated_public_points",n-distinct[B].size()}};
 std::ifstream in(argv[1]);std::string line;unsigned raw=0,warm=0,measured=0;
 while(std::getline(in,line)){const J row=J::parse(line);++raw;if(row.at("round")==0){++warm;continue;}++measured;const auto arm=row.at("arm_label").get<std::string>();if(!row.contains("trials"))continue;const std::string key=arm+"/"+row.at("B").dump();auto&g=out["measured_construction_totals"][key];if(g.is_null())g={{"attempts",0ULL},{"draws",0ULL},{"rejected_infinity",0ULL},{"rejected_coordinate",0ULL},{"seed_pairs",0ULL},{"step_infinity",0ULL},{"projective_additions",0ULL},{"affine_batches",0ULL},{"uninformative",0ULL}};
  for(const auto&t:row.at("trials")){g["attempts"]=g.at("attempts").get<std::uint64_t>()+1;for(const auto*k:{"draws","rejected_infinity","rejected_coordinate"})g[k]=g.at(k).get<std::uint64_t>()+t.at(k).get<std::uint64_t>();for(const auto*k:{"seed_pairs","step_infinity","projective_additions","affine_batches"})g[k]=g.at(k).get<std::uint64_t>()+t.at("ap_counts").at(k).get<std::uint64_t>();}
  g["uninformative"]=g.at("uninformative").get<std::uint64_t>()+row.value("fast_uninformative",std::uint64_t(0));
 }
 out["flow"]={{"planned",31680},{"attempted",raw},{"completed_wrappers",raw},{"warmup_excluded_by_rule",warm},{"measured",measured}};out["scope"]="Native supplement; targets sampled with replacement, repeated public points disclosed; construction counts are totals over measured target-rounds, no unmeasured field-operation estimates.";std::ofstream result(argv[3]);result<<out.dump(2)<<'\n';std::cout<<out.at("flow").dump()<<'\n';
}catch(const std::exception&e){std::cerr<<e.what()<<'\n';return 1;}}
