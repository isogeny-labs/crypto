#include "vendor/json.hpp"
#include <array>
#include <filesystem>
#include <fstream>
#include <iostream>
#include <map>
#include <string>
#include <vector>
using Json=nlohmann::json;
using Key=std::pair<unsigned,unsigned>;
static std::map<Key,Json> load(const std::filesystem::path& path){std::ifstream in(path);if(!in)throw std::runtime_error("comparison input open");std::map<Key,Json> out;std::string line;while(std::getline(in,line)){Json row=Json::parse(line);if(row.value("item",0)!=1)continue;Key key{row.at("p").get<unsigned>(),row.at("seed").get<unsigned>()};if(!out.emplace(key,row).second)throw std::runtime_error("duplicate comparison key");}return out;}
static bool flag(const Json& j){return j.is_boolean()?j.get<bool>():j.get<unsigned>()!=0;}
int main(int argc,char**argv){try{
 if(argc!=5)throw std::runtime_error("usage: compare ORIGINAL_RESULTS NEW_RESULTS CANDIDATE_TRACE OUTPUT.json");auto old=load(argv[1]),current=load(argv[2]);if(old.size()!=24||current.size()!=24)throw std::runtime_error("24-fixture population mismatch");
 std::array<const char*,13> identity{"p","seed","a","b","nonresidue","order","r","cofactor","generator","target","known_answer_k","torsion","curve_draws"};
 Json comparisons=Json::array();unsigned changed=0,old_hits=0,new_hits=0,old_rho=0,new_rho=0,measured_changed=0;
 for(const auto& [key,row]:old){auto found=current.find(key);if(found==current.end())throw std::runtime_error("replication fixture key missing");const auto& next=found->second;Json identities=Json::array(),measurements=Json::array();for(const char* field:identity)if(row.at(field)!=next.at(field))identities.push_back(field);for(auto it=row.begin();it!=row.end();++it)if(!next.contains(it.key())||next.at(it.key())!=it.value())measurements.push_back(it.key());changed+=!identities.empty();measured_changed+=!measurements.empty();old_hits+=flag(row.at("two_sum_found"));new_hits+=flag(next.at("two_sum_found"));old_rho+=flag(row.at("rho_ok"));new_rho+=flag(next.at("rho_ok"));
  Json oi=Json::object(),ni=Json::object();for(const char* field:identity){oi[field]=row.at(field);ni[field]=next.at(field);}comparisons.push_back({{"p",key.first},{"seed",key.second},{"old_identity",oi},{"new_identity",ni},{"changed_identity_fields",identities},{"changed_record_fields",measurements}});
 }
 std::ifstream trace(argv[3]);if(!trace)throw std::runtime_error("candidate trace open");unsigned draws=0,accepted=0,singular=0,subgroup=0;std::string line;while(std::getline(trace,line)){Json row=Json::parse(line);++draws;if(flag(row["accepted"]))++accepted;else if(row.value("reason","")=="singular")++singular;else if(row.value("reason","")=="subgroup_condition")++subgroup;else throw std::runtime_error("unknown rejection reason");}if(accepted!=24||accepted+singular+subgroup!=draws)throw std::runtime_error("candidate-trace coverage mismatch");
 Json result={{"schema","targets-prime-replication-comparison/v2"},{"classification","review-driven corrected replication, not external independent review"},{"old_fixtures",old.size()},{"new_fixtures",current.size()},{"changed_accepted_identities",changed},{"changed_record_rows",measured_changed},{"old_two_sum_hits",old_hits},{"new_two_sum_hits",new_hits},{"old_rho_successes",old_rho},{"new_rho_successes",new_rho},{"new_sampler_draws",draws},{"new_sampler_accepted",accepted},{"new_singular_rejections",singular},{"new_subgroup_rejections",subgroup},{"comparisons",comparisons},{"original_denominators_preserved",true}};
 if(std::filesystem::exists(argv[4]))throw std::runtime_error("comparison output exists");std::ofstream out(argv[4]);out<<result.dump(2)<<"\n";if(!out)throw std::runtime_error("comparison output failure");std::cout<<"Compared24 fixture identities; changed="<<changed<<", original/new hits="<<old_hits<<"/"<<new_hits<<", original/new rho="<<old_rho<<"/"<<new_rho<<"\n";return 0;
}catch(const std::exception& error){std::cerr<<error.what()<<"\n";return 1;}}
