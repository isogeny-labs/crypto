// Preregistered complete-panel comparison with explicitly pinned historical inputs.
#define main rootcover_describe_main
#include "describe.cpp"
#undef main
int main(int argc,char**argv){try{
 need(argc==6,"compare NEW NEW_SHA HISTORICAL HISTORICAL_SHA OUTPUT");auto nb=read(argv[1]),ob=read(argv[3]);need(digest(nb)==argv[2]&&digest(ob)==argv[4],"both input hashes required");J n=J::parse(nb),o=J::parse(ob);need(n["schema"]=="twist.root-cover-cold.audit/v1"&&n["profiles"]==1365,"fresh population");need(o["schema"]=="twist.projective-cold.audit/v1"&&o["profiles"]==2730,"pinned prior population");need(n["maps"]==o["maps"],"same actual source/twist maps");
 std::map<std::array<int,3>,J> prior,fresh;for(const auto&p:o["profiles_data"])need(prior.emplace(std::array<int,3>{p["route"],p["policy"],p["cap"]},p).second,"old duplicate");for(const auto&p:n["profiles_data"])need(fresh.emplace(std::array<int,3>{p["route"],p["policy"],p["cap"]},p).second,"new duplicate");
 J comparisons=J::array();for(int route:{3,4,5})for(int oldcap:{3,4}){
  long long gain=0,loss=0,both=0,neither=0,blocked=0,verified=0,lower=0,equal=0,higher=0,rankmore=0,rankless=0,ranktie=0;
  for(int policy=0;policy<455;policy++){const auto&a=fresh.at({route,policy,3});const auto&b=prior.at({route,policy,oldcap});need(a["works"].size()==30&&b["works"].size()==30&&a["diagnostic_ranks"].size()==31&&b["diagnostic_ranks"].size()==31,"full source/target membership");
   for(int q=1;q<=30;q++){int ar=a["diagnostic_ranks"][q],br=b["diagnostic_ranks"][q];rankmore+=ar>br;rankless+=ar<br;ranktie+=ar==br;const auto&x=a["works"][q-1];const auto&y=b["works"][q-1];need(x["target"]==q&&y["target"]==q,"target binding");const auto&u=x["result"];const auto&v=y["result"];int uk=u["kind"],vk=v["kind"];gain+=uk==1&&vk!=1;loss+=uk!=1&&vk==1;both+=uk==1&&vk==1;neither+=uk!=1&&vk!=1;blocked+=uk==0||vk==0;
    if(uk!=0&&vk!=0){need(u["scalar"]==v["scalar"],"original scalar equality");verified++;}if(uk==1&&vk==1){int d=u["counts"][9],e=v["counts"][9];lower+=d<e;equal+=d==e;higher+=d>e;}
   }
  }
  comparisons.push_back(J{{"route",route},{"historical_cap",oldcap},{"fresh_cap",3},{"configuration_targets",13650},{"fresh_only_algebraic",gain},{"historical_only_algebraic",loss},{"both_algebraic",both},{"neither_algebraic",neither},{"blocked_either",blocked},{"both_original_scalars_verified",verified},{"both_algebraic_named_witness_peak_lower",lower},{"both_algebraic_named_witness_peak_equal",equal},{"both_algebraic_named_witness_peak_higher",higher},{"initial_query_rank_higher",rankmore},{"initial_query_rank_lower",rankless},{"initial_query_rank_equal",ranktie},{"original_input_scope",route==4?"compressed named field inputs; extension-coordinate pullback degree UNKNOWN":"identical extension-field membership cubics; not original S3 or intrinsic regularity"}});
 }
 J out={{"schema","twist.root-cover-historical-comparison/v1"},{"fresh_input_sha256",argv[2]},{"historical_input_sha256",argv[4]},{"comparisons",comparisons},{"historical_data_reexecuted",false},{"speed_claim",false},{"intrinsic_regularity_claim",false},{"signed_verdict",false}};std::ifstream exists(argv[5]);need(!exists.good(),"fresh comparison output");std::ofstream f(argv[5]);need(bool(f),"output");f<<out.dump()<<'\n';need(bool(f),"complete write");return 0;
}catch(const std::exception&e){std::cerr<<e.what()<<'\n';return 101;}}
