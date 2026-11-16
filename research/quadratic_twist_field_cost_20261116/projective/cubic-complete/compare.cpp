// Exact preregistered yield/degree comparisons. No clock or counter-speed claim.
#include <nlohmann/json.hpp>
#include <openssl/sha.h>
#include <fstream>
#include <iostream>
#include <iomanip>
#include <map>
#include <array>
#include <sstream>
#include <stdexcept>
using J=nlohmann::ordered_json;
void need(bool b,const char*s){if(!b)throw std::runtime_error(s);}
std::string read(const char*p){std::ifstream f(p,std::ios::binary);need(bool(f),"declared input");return{std::istreambuf_iterator<char>(f),{}};}
std::string hash(const std::string&s){unsigned char h[32];SHA256((const unsigned char*)s.data(),s.size(),h);std::ostringstream out;for(auto x:h)out<<std::hex<<std::setw(2)<<std::setfill('0')<<int(x);return out.str();}
bool algebraic(const J&p){for(const auto&w:p["works"])if(w["result"]["kind"]!=1)return false;return true;}
int main(int argc,char**argv){try{
 need(argc==5,"compare HEAD OLD ROOT OUTPUT; new audited JSON on stdin");std::string fresh{std::istreambuf_iterator<char>(std::cin),{}};auto old=read(argv[2]),root=read(argv[3]);need(hash(old)=="9e48248cede5094609574f2ff78bf4f85901ae04a931bcd08755520f032ef95b"&&hash(root)=="832f103357feb6beafbf636939a039058ece28f790b45d374f902b68601765fe","exact pinned prior independent receipts");J n=J::parse(fresh),o=J::parse(old),r=J::parse(root);need(n["schema"]=="twist.cubic-complete.audit/v1"&&n["head"]==argv[1]&&n["profiles"]==1365&&n["target_workloads"]==40950,"complete new actual domain");
 std::map<std::array<int,3>,J> nm,om,rm;for(auto&p:n["profiles_data"])need(nm.emplace(std::array<int,3>{p["route"],p["policy"],p["cap"]},p).second,"new duplicate");for(auto&p:o["profiles_data"])need(om.emplace(std::array<int,3>{p["route"],p["policy"],p["cap"]},p).second,"old duplicate");for(auto&p:r["profiles_data"])need(rm.emplace(std::array<int,3>{p["route"],p["policy"],p["cap"]},p).second,"root duplicate");need(nm.size()==1365&&om.size()==2730&&rm.size()==1365,"all policy/cap populations");
 J comparisons=J::array();for(int baseline=0;baseline<3;baseline++)for(int route:{3,4,5}){
  int cap=baseline==1?4:3;const auto&bm=baseline==2?rm:om;long long gain=0,loss=0,both=0,neither=0,verified=0,blocked=0,lower=0,equal=0,higher=0,rmore=0,rless=0,rtie=0;
  for(int policy=0;policy<455;policy++){const auto&a=nm.at({route,policy,3});const auto&b=bm.at({route,policy,cap});need(a["works"].size()==30&&b["works"].size()==30&&a["diagnostic_ranks"].size()==31&&b["diagnostic_ranks"].size()==31,"full membership");for(int q=1;q<=30;q++){
   int ar=a["diagnostic_ranks"][q],br=b["diagnostic_ranks"][q];rmore+=ar>br;rless+=ar<br;rtie+=ar==br;const auto&x=a["works"][q-1];const auto&y=b["works"][q-1];need(x["target"]==q&&y["target"]==q,"original target binding");const auto&u=x["result"];const auto&v=y["result"];int uk=u["kind"],vk=v["kind"];gain+=uk==1&&vk!=1;loss+=uk!=1&&vk==1;both+=uk==1&&vk==1;neither+=uk!=1&&vk!=1;blocked+=uk==0||vk==0;if(uk!=0&&vk!=0){need(u["scalar"]==v["scalar"],"same original scalar");verified++;}if(uk==1&&vk==1){int d=u["counts"][9],e=v["counts"][9];lower+=d<e;equal+=d==e;higher+=d>e;}
  }}comparisons.push_back(J{{"baseline",baseline==2?"historical quadratic-only rootcover":baseline==1?"historical GCDcap4":"historical GCDcap3"},{"route",route},{"fresh_cap",3},{"historical_cap",cap},{"configuration_targets",13650},{"fresh_only_algebraic",gain},{"historical_only_algebraic",loss},{"both_algebraic",both},{"neither_algebraic",neither},{"blocked_either",blocked},{"same_original_scalars_checked",verified},{"witness_peak_lower",lower},{"witness_peak_equal",equal},{"witness_peak_higher",higher},{"initial_rank_higher",rmore},{"initial_rank_lower",rless},{"initial_rank_equal",rtie}});
 }
 for(int p=0;p<455;p++)for(int route:{3,5}){const auto&a=nm.at({4,p,3});const auto&b=nm.at({route,p,3});need(a["diagnostic_ranks"]==b["diagnostic_ranks"],"matched complete useful ranks");for(int q=0;q<30;q++)for(const char*key:{"scalar","kind","rank","setup_m","shift","logs"})need(a["works"][q]["result"][key]==b["works"][q]["result"][key],"matched actual logical recovery");}
 J frontier=J::array();for(bool require_alg:{false,true}){int best=-1,sources=0,eligible=0,above=0,tie=0,below=0;for(int route:{3,5})for(int p=0;p<455;p++){const auto&s=nm.at({route,p,3});if(require_alg&&!algebraic(s))continue;sources++;best=std::max(best,s["rank_numerator"].get<int>());}need(sources>0,"nonempty updated strongest-source frontier");for(int p=0;p<455;p++){const auto&c=nm.at({4,p,3});if(require_alg&&!algebraic(c))continue;eligible++;int rank=c["rank_numerator"];above+=rank>best;tie+=rank==best;below+=rank<best;}frontier.push_back(J{{"require_all30_algebraic",require_alg},{"eligible_sources",sources},{"eligible_candidates",eligible},{"source_best_initial_rank_sum",best},{"candidate_above",above},{"candidate_tie",tie},{"candidate_below",below}});}
 std::ifstream existing(argv[4]);need(!existing.good(),"fresh output");std::ofstream f(argv[4]);f<<J{{"schema","twist.cubic-complete.comparison/v1"},{"head",argv[1]},{"new_raw_receipt_sha256",hash(fresh)},{"legacy_receipt_sha256",hash(old)},{"quadratic_receipt_sha256",hash(root)},{"stats",n["stats"]},{"comparisons",comparisons},{"matched_all_profiles_tie",true},{"updated_source_frontier",frontier},{"historical_arithmetic_reexecuted",false},{"timing_or_instruction_speed_claim",false},{"intrinsic_regularity_claim",false},{"compressed_pullback_degree",nullptr},{"signed_verdict",false}}.dump()<<'\n';need(bool(f),"output complete");return 0;
 }catch(const std::exception&e){std::cerr<<e.what()<<'\n';return 101;}}
