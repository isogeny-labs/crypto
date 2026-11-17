#include "json.hpp"
#include <fstream>
#include <sstream>
#include <map>
#include <set>
#include <vector>
#include <algorithm>
#include <cmath>
#include <iostream>
using json=nlohmann::json;
using Rows=std::vector<std::map<std::string,std::string>>;
void need(bool b,const std::string&s){if(!b)throw std::runtime_error(s);}
std::vector<std::string> split(const std::string&s){std::stringstream in(s);std::vector<std::string>v;std::string x;while(std::getline(in,x,','))v.push_back(x);if(!s.empty()&&s.back()==',')v.push_back("");return v;}
Rows csv(const std::string&p){std::ifstream f(p);need(bool(f),"read "+p);std::string s;std::getline(f,s);auto h=split(s);Rows r;while(std::getline(f,s)){if(s.empty())continue;auto v=split(s);need(v.size()==h.size(),"CSV arity "+p);std::map<std::string,std::string>a;for(size_t i=0;i<h.size();i++)a[h[i]]=v[i];r.push_back(std::move(a));}return r;}
long double number(const std::string&s){need(!s.empty(),"missing numeric value");size_t n=0;auto v=std::stold(s,&n);need(n==s.size()&&std::isfinite(v),"numeric value");return v;}
void close(long double a,long double b,const std::string&msg){need(std::fabs(a-b)<=1e-5L,msg);}
struct Stat{size_t n;long double lo,mean,hi;};
Stat statistics(const std::vector<long double>&v){need(!v.empty(),"empty statistics");long double sum=0;for(auto x:v)sum+=x;return{v.size(),*std::min_element(v.begin(),v.end()),sum/v.size(),*std::max_element(v.begin(),v.end())};}
int main(int argc,char**argv){try{
 need(argc==3,"REPORT_DIRECTORY VALIDATION_JSON");std::string root=argv[1];auto master=csv(root+"/per_curve.csv");need(master.size()==46,"curve count");size_t cases=0,statrows=0,regcases=0,derived=0;
 for(const auto&curve:master){auto id=curve.at("curve");need(number(curve.at("point_order"))==18014494963342201.L&&number(curve.at("curve_order"))==72057979853368804.L,"orders");
  for(const auto&alg:{std::string("rho"),std::string("bsgs")}){
   auto a=csv(root+"/tables/queries/"+alg+"/C"+id+"_original.csv"),b=csv(root+"/tables/queries/"+alg+"/C"+id+"_replay.csv");need(a.size()==100&&b.size()==100,"query count");std::map<std::string,std::string> baseline;std::set<std::string> seen;
   std::vector<long double> group,mul;
   for(const auto&r:a){need(r.at("verified")=="true","original verification");need(!baseline.count(r.at("instance")),"original unique");baseline[r.at("instance")]=r.at("group_ops");group.push_back(number(r.at("group_ops")));}
   for(const auto&r:b){need(seen.insert(r.at("instance")).second,"replay unique");need(r.at("public_equation_checked")=="true"&&r.at("matched_original_stats")=="true","replay verification");need(baseline.at(r.at("instance"))==r.at("group_ops"),"paired original count");mul.push_back(number(r.at("Fp.mont_mul")));for(const auto&mod:{"Fp","scalar_mod_r","other_modulus"}){std::string p=mod;need(number(r.at(p+".mont_redc"))>=number(r.at(p+".mont_mul"))+number(r.at(p+".mont_square")),"reduction/product accounting");}}
   auto gs=statistics(group),ms=statistics(mul);for(const auto&item:{std::make_pair("group",gs),std::make_pair("Fp_M",ms)}){auto p=alg+"_"+item.first;close(number(curve.at(p+"_min")),item.second.lo,"headline min");close(number(curve.at(p+"_mean")),item.second.mean,"headline mean");close(number(curve.at(p+"_max")),item.second.hi,"headline max");}
   auto rows=csv(root+"/tables/field/"+alg+"/C"+id+".csv");
   for(const auto&r:rows){std::vector<long double> v;auto metric=r.at("metric"),gen=r.at("generator");for(const auto&x:b){if(gen!="all"&&x.at("generator")!=gen)continue;std::string k=metric;if(k=="instrumented_process_cpu_ns")k="instrumented_cpu_ns";auto at=k.find(".mont_products_total_derived");if(at!=std::string::npos){auto p=k.substr(0,at);v.push_back(number(x.at(p+".mont_mul"))+number(x.at(p+".mont_square")));derived++;}else v.push_back(number(x.at(k)));}auto s=statistics(v);need(size_t(number(r.at("n")))==s.n,"statistics sample size");close(number(r.at("min")),s.lo,"metric min");close(number(r.at("mean")),s.mean,"metric mean");close(number(r.at("max")),s.hi,"metric max");statrows++;}
   cases+=a.size();
  }
  auto regs=csv(root+"/tables/regularity/C"+id+".csv");need(regs.size()==320,"regularity sample size");std::map<std::string,int> stages;std::vector<long double> v;
  for(const auto&r:regs){auto stage=r.at("stage");int expected=stage=="positive_control"?25:26;need(number(r.at("reg_J"))==expected&&number(r.at("reg_quotient"))==expected-1,"regularity certificate projection");stages[stage]++;v.push_back(number(r.at("reg_J")));}
  need(stages["setup"]==200&&stages["target"]==100&&stages["positive_control"]==20,"regularity stages");auto s=statistics(v);close(number(curve.at("reg_J_all_min")),s.lo,"regularity min");close(number(curve.at("reg_J_all_mean")),s.mean,"regularity mean");close(number(curve.at("reg_J_all_max")),s.hi,"regularity max");regcases+=regs.size();
 }
 json audit={{"passed",true},{"method","Independent C++ recomputation from public numeric CSV projections; no private inputs"},{"query_cases_checked",cases},{"field_summary_rows_checked",statrows},{"derived_M_plus_S_case_contributions_checked",derived},{"regularity_rows_checked",regcases},{"curves",46},{"modular_multiply_and_square_calls_disjoint",true},{"comparison_absolute_rounding_tolerance",1e-5},{"does_not_replace_independent_point_equation_or_algebraic_proof_replay",true}};
 std::ifstream exists(argv[2]);need(!exists,"write once");std::ofstream out(argv[2]);out<<audit.dump(2)<<'\n';std::cout<<audit.dump()<<'\n';return 0;
 }catch(const std::exception&e){std::cerr<<"PUBLIC_METRICS_ERROR "<<e.what()<<'\n';return 2;}}
