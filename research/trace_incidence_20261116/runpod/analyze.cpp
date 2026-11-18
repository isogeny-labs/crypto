#include <nlohmann/json.hpp>
#include <algorithm>
#include <cmath>
#include <fstream>
#include <iostream>
#include <map>
#include <numeric>
#include <string>
#include <tuple>
#include <vector>
using J=nlohmann::json;
struct Samples {std::vector<double> cpu,wall,rss,child;unsigned rows=0,completed=0,censored=0,failed=0,incomplete=0,attempts=0;};
struct Target {unsigned B=0,round=0,attempts=0;std::string arm,fixture,last;double cpu=0,wall=0,rss=0,child=0;bool failed=false,missing=false;};
double median(std::vector<double> values){if(values.empty())return NAN;std::sort(values.begin(),values.end());auto n=values.size();return n%2?values[n/2]:(values[n/2-1]+values[n/2])*0.5;}
J distribution(const std::vector<double>& values){if(values.empty())return nullptr;return {{"n",values.size()},{"median",median(values)},{"minimum",*std::min_element(values.begin(),values.end())},{"maximum",*std::max_element(values.begin(),values.end())}};}
J summary(const Samples& s){return {{"rows",s.rows},{"completed",s.completed},{"censored",s.censored},{"failed",s.failed},{"incomplete",s.incomplete},{"attempts_charged",s.attempts},{"cpu_seconds",distribution(s.cpu)},{"wall_seconds",distribution(s.wall)},{"parent_peak_rss_kib",distribution(s.rss)},{"maximum_child_rss_kib",distribution(s.child)}};}
int main(int argc,char**argv){
 try {
 if(argc!=2)return 2;
 std::ifstream file(argv[1]);if(!file)return 2;
 std::map<std::tuple<std::string,std::string,unsigned>,Target> targets;
 std::map<std::string,Samples> roots,translation,rho;
 std::map<std::pair<std::string,unsigned>,double> rho_units;
 J degree_histogram=J::object();unsigned auxiliary_calls=0,codomain_calls=0,curve_attempts=0;
 J allstatus=J::object();unsigned raw=0,measured=0,warmup=0,scalar_checks=0,existence_checks=0;
 std::map<std::string,std::uint64_t> target_scalars;
 std::map<std::tuple<std::string,unsigned,unsigned>,bool> attempt_truth;
 bool incomplete_json_tail=false;
 while(true){
  file>>std::ws;if(file.peek()==std::char_traits<char>::eof())break;
  J row;try{file>>row;}catch(const J::parse_error&){if(file.eof()){incomplete_json_tail=true;break;}throw;}++raw;
  const auto source_name=row.value("source_fixture","");
  if(row.contains("recovered_scalar")&&!row.at("recovered_scalar").is_null()){const auto k=row.at("recovered_scalar").get<std::uint64_t>();const auto found=target_scalars.find(source_name);if(found!=target_scalars.end()){if(found->second!=k)throw std::runtime_error("scalar disagreement across independent mechanisms");++scalar_checks;}else target_scalars[source_name]=k;}if(row.value("phase","")=="warmup"){++warmup;continue;}if(row.value("phase","")!="measured")continue;++measured;
  const auto arm=row.value("arm_label","unknown"),fixture=row.value("source_fixture","unknown"),status=row.value("status","unknown");
  auto kind=row.value("experiment","");if(kind.empty()&&fixture=="paper83")kind="paper-root";
  const auto statuskey=kind+"/"+arm+"/"+status;allstatus[statuskey]=allstatus.value(statuskey,0u)+1;
  if(kind=="paper-root"){
   auto& s=roots[arm];++s.rows;if(status!="passed"){++s.failed;continue;}++s.completed;
   s.cpu.push_back(row.at("batch").at("cpu_seconds").get<double>()/row.at("repetitions").get<unsigned>());s.wall.push_back(row.at("batch").at("wall_seconds").get<double>()/row.at("repetitions").get<unsigned>());s.rss.push_back(row.value("peak_rss_kib",0.0));continue;
  }
  if(kind=="translation"){
   const auto key=std::to_string(row.at("degree").get<unsigned>())+"/"+row.at("case").get<std::string>()+"/"+arm;auto& s=translation[key];++s.rows;if(status!="passed"){++s.failed;continue;}++s.completed;s.cpu.push_back(row.at("complete").at("cpu_seconds"));s.wall.push_back(row.at("complete").at("wall_seconds"));s.rss.push_back(row.value("peak_rss_kib",0.0));continue;
  }
  if(arm=="rho"){
   auto& s=rho[fixture.substr(1,3)];++s.rows;if(status=="solved"&&row.value("public_scalar_check",false)){++s.completed;s.wall.push_back(row.at("complete_adapter_wall_seconds"));rho_units[{fixture,row.at("round").get<unsigned>()}]=row.at("complete_adapter_wall_seconds");}else if(status=="censored")++s.censored;else ++s.failed;continue;
  }
  if(kind=="incidence" || (!fixture.empty()&&fixture[0]=='b')){
   if(row.contains("gcd_degree")){auto key=arm+"/"+std::to_string(row.at("gcd_degree").get<unsigned>());degree_histogram[key]=degree_histogram.value(key,0u)+1;}
   if(row.contains("auxiliary")){++auxiliary_calls;curve_attempts+=row.at("auxiliary").value("curve_attempts",0u);}
   if(row.contains("codomain_auxiliary")){++codomain_calls;curve_attempts+=row.at("codomain_auxiliary").value("curve_attempts",0u);}
   const auto round=row.at("round").get<unsigned>();auto& t=targets[{arm,fixture,round}];t.arm=arm;t.fixture=fixture;t.round=round;t.B=row.value("B",unsigned(std::stoul(fixture.substr(1,3))));
   if(row.at("attempt").get<unsigned>()!=t.attempts || t.last=="relation")throw std::runtime_error("invalid attempt sequence");
   ++t.attempts;t.last=status;t.failed|=status=="failed";
   if(status!="failed"){const auto key=std::make_tuple(fixture,round,row.at("attempt").get<unsigned>());const bool informative=status=="relation";const auto found=attempt_truth.find(key);if(found!=attempt_truth.end()){if(found->second!=informative)throw std::runtime_error("independent pair/quotient/root existence disagreement");++existence_checks;}else attempt_truth[key]=informative;}
   if(row.contains("complete")){t.cpu+=row.at("complete").at("cpu_seconds").get<double>();t.wall+=row.at("complete").at("wall_seconds").get<double>();}else t.missing=true;
   t.rss=std::max(t.rss,row.value("peak_rss_kib",0.0));t.child=std::max(t.child,row.value("maximum_child_rss_kib",0.0));
  }
 }
 std::map<std::string,Samples> incidence;
 for(const auto& item:targets){const auto& t=item.second;auto& s=incidence[std::to_string(t.B)+"/"+t.arm];++s.rows;s.attempts+=t.attempts;
  if(t.failed)++s.failed;else if(t.last=="relation")++s.completed;else if(t.attempts==4)++s.censored;else ++s.incomplete;
  if(!t.missing){s.cpu.push_back(t.cpu);s.wall.push_back(t.wall);}s.rss.push_back(t.rss);s.child.push_back(t.child);
 }
 J output={{"schema","trace-runpod-descriptive-summary/v1"},{"pcc_standing","unverified"},{"raw_rows",raw},{"measured_incidence_gcd_degree_histogram",degree_histogram},{"measured_lazy_auxiliary_calls",auxiliary_calls},{"measured_codomain_auxiliary_calls",codomain_calls},{"measured_auxiliary_curve_attempts",curve_attempts},{"incomplete_json_tail",incomplete_json_tail},{"cross_mechanism_scalar_agreement_checks",scalar_checks},{"cross_mechanism_attempt_existence_checks",existence_checks},{"warmup_rows_retained",warmup},{"measured_rows",measured},{"status_counts",allstatus},{"expected_measured_target_units_per_incidence_arm",300},{"interpretation","Incidence time is bounded charged work across all attempted pools, including censored/failed units; it is not automatically time to a completed solve. Root time is per call with supplied literal datum and separately charged isogeny construction. No L2, fitted exponent, fastest-tool or validated speed claim."}};
 for(const auto& group:roots)output["paper_root_per_call"][group.first]=summary(group.second);
 for(const auto& group:translation)output["translation"][group.first]=summary(group.second);
 for(const auto& group:incidence)output["incidence_bounded_target_work"][group.first]=summary(group.second);
 for(const auto& group:rho)output["rho_adapter_by_B"][group.first]=summary(group.second);
 J paired=J::object();
 for(const auto& item:targets){const auto& t=item.second;if(t.last!="relation"||t.failed||t.missing)continue;const auto found=targets.find({"known",t.fixture,t.round});if(found==targets.end())continue;const auto& b=found->second;if(b.last!="relation"||b.failed||b.missing||t.cpu<=0||t.wall<=0)continue;const auto key=std::to_string(t.B)+"/"+t.arm;paired[key]["cpu"].push_back(b.cpu/t.cpu);paired[key]["wall"].push_back(b.wall/t.wall);}
 for(auto it=paired.begin();it!=paired.end();++it){std::vector<double> cpu=it.value().at("cpu"),wall=it.value().at("wall");output["paired_known_over_arm_completed_only"][it.key()]={{"cpu_ratio",distribution(cpu)},{"wall_ratio",distribution(wall)}};}
 std::map<std::string,std::vector<double>> rho_ratios;
 for(const auto& item:targets){const auto& t=item.second;const auto found=rho_units.find({t.fixture,t.round});if(t.last=="relation"&&!t.failed&&!t.missing&&found!=rho_units.end()&&found->second>0)rho_ratios[std::to_string(t.B)+"/"+t.arm].push_back(t.wall/found->second);}
 for(const auto& item:rho_ratios)output["paired_arm_over_rho_wall_completed_only"][item.first]=distribution(item.second);
 for(const auto& candidate:std::vector<std::string>{"dense","trace","pullback","dense-isogeny","trace-isogeny","pullback-isogeny"}){if(roots.count(candidate)&&roots.count("pullback")){const auto& a=roots.at(candidate);const auto& b=roots.at("pullback");if(!a.cpu.empty()&&!b.cpu.empty())output["derived_root_cpu_ratio_of_medians_over_pullback"][candidate]=median(a.cpu)/median(b.cpu);}}
 for(const auto& group:translation){const auto& key=group.first;if(key.size()>6&&key.substr(key.size()-6)=="/dense"){const auto counterpart=key.substr(0,key.size()-6)+"/trace";if(translation.count(counterpart)&&!group.second.cpu.empty()&&!translation.at(counterpart).cpu.empty())output["derived_translation_cpu_ratio_of_medians_dense_over_trace"][key.substr(0,key.size()-6)]=median(group.second.cpu)/median(translation.at(counterpart).cpu);}}
 std::cout<<output.dump(2)<<'\n';
 return 0;
 }catch(const std::exception& e){std::cerr<<e.what()<<"\n";return 1;}
}
