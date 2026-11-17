// Post-data descriptive accounting of the full, already independently replayed panel.
// No speed, original-input solving-degree or signed verdict from these diagnostics.
#include <nlohmann/json.hpp>
#include <openssl/sha.h>
#include <fstream>
#include <sstream>
#include <iomanip>
#include <map>
#include <array>
#include <vector>
#include <stdexcept>
#include <iostream>
using J=nlohmann::ordered_json;
void need(bool b,const char*s){if(!b)throw std::runtime_error(s);}
std::string read(std::string p){std::ifstream f(p,std::ios::binary);need(bool(f),"input");return {std::istreambuf_iterator<char>(f),{}};}
std::string digest(std::string s){unsigned char h[SHA256_DIGEST_LENGTH];SHA256((const unsigned char*)s.data(),s.size(),h);std::ostringstream o;for(auto b:h)o<<std::hex<<std::setw(2)<<std::setfill('0')<<(int)b;return o.str();}
bool all_algebraic(const J&p){for(const auto&w:p["works"])if(w["result"]["kind"]!=1)return false;return true;}
bool all_verified(const J&p){for(const auto&w:p["works"])if(w["result"]["kind"]==0)return false;return true;}
int main(int argc,char**argv){try{need(argc==4,"describe INPUT EXACT_SHA OUTPUT");auto bytes=read(argv[1]);need(digest(bytes)==argv[2],"input binding");J doc=J::parse(bytes);need(doc["schema"]=="twist.projective-cold.audit/v1"&&doc["profiles"]==2730,"population");
 std::map<std::array<int,3>,J> rows;for(const auto&p:doc["profiles_data"]){std::array<int,3>key{p["route"],p["policy"],p["cap"]};need(rows.emplace(key,p).second,"duplicate profile");need(p["works"].size()==30&&p["diagnostic_ranks"].size()==31,"point coverage");}
 J matched=J::array();for(int baseline:{3,5})for(int cap:{3,4}){long long gain=0,loss=0,both=0,neither=0,blocked=0,rank_high=0,rank_low=0,rank_tie=0,profile_high=0,profile_low=0,profile_tie=0,progress_less=0,progress_more=0,progress_same=0,peak_less=0,peak_more=0,peak_same=0;
  for(int policy=0;policy<455;policy++){const auto&c=rows.at({4,policy,cap});const auto&b=rows.at({baseline,policy,cap});int cr=c["rank_numerator"],br=b["rank_numerator"];profile_high+=cr>br;profile_low+=cr<br;profile_tie+=cr==br;
   for(int q=1;q<31;q++){int x=c["diagnostic_ranks"][q],y=b["diagnostic_ranks"][q];rank_high+=x>y;rank_low+=x<y;rank_tie+=x==y;}
   for(int target=0;target<30;target++){const auto&x=c["works"][target];const auto&y=b["works"][target];need(x["target"]==target+1&&y["target"]==target+1,"target order");const auto&u=x["result"];const auto&v=y["result"];need(u["scalar"]==v["scalar"],"different original scalar");int a=u["kind"],z=v["kind"];blocked+=a==0||z==0;gain+=a==1&&z!=1;loss+=a!=1&&z==1;both+=a==1&&z==1;neither+=a!=1&&z!=1;
    int pm=u["counts"][10],pn=v["counts"][10];progress_less+=pm<pn;progress_more+=pm>pn;progress_same+=pm==pn;int dm=u["counts"][9],dn=v["counts"][9];peak_less+=dm<dn;peak_more+=dm>dn;peak_same+=dm==dn;
   }
  }
  matched.push_back(J{{"baseline_route",baseline},{"candidate_route",4},{"cap",cap},{"configuration_targets",13650},{"candidate_only_algebraic",gain},{"baseline_only_algebraic",loss},{"both_algebraic",both},{"neither_algebraic",neither},{"blocked_either",blocked},{"initial_query_rank_higher",rank_high},{"initial_query_rank_lower",rank_low},{"initial_query_rank_equal",rank_tie},{"profile_rank_sum_higher",profile_high},{"profile_rank_sum_lower",profile_low},{"profile_rank_sum_equal",profile_tie},{"progress_mults_lower",progress_less},{"progress_mults_higher",progress_more},{"progress_mults_equal",progress_same},{"named_chart_peak_lower",peak_less},{"named_chart_peak_higher",peak_more},{"named_chart_peak_equal",peak_same}});
 }
 J frontier=J::array();for(bool algebraic:{false,true})for(int cap:{3,4}){int best=-1,eligible_source=0;for(const auto&[key,p]:rows)if(key[0]!=4&&key[2]<=cap&&(algebraic?all_algebraic(p):all_verified(p))){eligible_source++;best=std::max(best,p["rank_numerator"].get<int>());}int candidates=0,high=0,low=0,tie=0,empty=0;for(int policy=0;policy<455;policy++){const auto&p=rows.at({4,policy,cap});if(!(algebraic?all_algebraic(p):all_verified(p)))continue;candidates++;int rank=p["rank_numerator"];if(best<0)empty++;else{high+=rank>best;low+=rank<best;tie+=rank==best;}}
  frontier.push_back(J{{"require_all30_algebraic",algebraic},{"candidate_requested_cap",cap},{"eligible_source_profiles",eligible_source},{"source_best_initial_rank_sum",best<0?J(nullptr):J(best)},{"eligible_candidate_profiles",candidates},{"candidate_higher",high},{"candidate_equal",tie},{"candidate_lower",low},{"empty_source_frontier",empty},{"qualification","requested named-chart cap only; original-input pullback witness degree unknown, no verified original-degree win"}});
 }
 J out={{"schema","twist.projective-posthoc-description/v1"},{"input_sha256",argv[2]},{"analysis_after_original_data",true},{"matched",matched},{"full_source_policy_frontier_diagnostic",frontier},{"speed_claim",false},{"original_solving_degree_claim",false},{"signed_verdict",false}};
 std::ifstream exists(argv[3]);need(!exists.good(),"fresh output");std::ofstream f(argv[3]);need(bool(f),"output");f<<out.dump()<<'\n';need(bool(f),"write");return 0;
 }catch(const std::exception&e){std::cerr<<e.what()<<'\n';return 101;}}
