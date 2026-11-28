// Descriptive supplement from frozen native summaries; no acceptance-rule change.
#include <nlohmann/json.hpp>
#include <filesystem>
#include <fstream>
#include <iostream>
#include <map>
#include <vector>
using J=nlohmann::json;
struct Arm{std::string name;double wall,rss;};
int main(int argc,char**argv){try{
 if(argc!=3){throw std::runtime_error("observed-frontier RESULTS_DIR OUTPUT");}J result;
 const std::vector<std::pair<std::string,std::string>> files={{"setup","summary-a1.json"},{"target","summary-target-a1.json"},{"ap","summary-ap-a1.json"},{"holdout","summary-holdout-a1.json"},{"flat","summary-flat-a1.json"},{"field","summary-field-a1.json"},{"scaling","summary-scaling-a1.json"}};
 for(const auto&[stage,file]:files){std::ifstream in(std::filesystem::path(argv[1])/file);J data;in>>data;std::map<std::string,std::vector<Arm>> panels;
  for(const auto&[key,g]:data.at("groups").items()){const auto split=key.rfind('/');const auto name=key.substr(0,split),B=key.substr(split+1);if(name.size()>3&&name.substr(name.size()-3)=="-aa")continue;const bool complete=g.at("solved")==g.at("complete_wall_s").at("n");if(!complete||!g.contains("peak_rss_kib")){result["excluded"][stage].push_back(key);continue;}panels[B].push_back({name,g.at("complete_wall_s").at("mean"),g.at("peak_rss_kib").at("mean")});}
  for(const auto&[B,arms]:panels)for(const auto&x:arms){bool dominated=false;for(const auto&y:arms)if(y.wall<=x.wall&&y.rss<=x.rss&&(y.wall<x.wall||y.rss<x.rss))dominated=true;if(!dominated)result["observed_mean_frontiers"][stage][B].push_back({{"arm",x.name},{"mean_complete_wall_s",x.wall},{"mean_process_peak_rss_kib",x.rss}});}
  if(stage=="scaling")for(const auto&[B,arms]:panels){(void)arms;const auto&candidate=data.at("groups").at("field-quarter/"+B);for(const auto&reference:std::vector<std::string>{"baseline-balanced","field-balanced","current-tuned4","current-tuned8","current-tuned16","current-tuned32","current-tuned64","current-tuned128"}){const auto&base=data.at("groups").at(reference+"/"+B);const auto&ratio=data.at("target_cluster_mean_ratios").at("field-quarter_over_"+reference+"/"+B);const bool complete=candidate.at("censored")==0&&base.at("censored")==0;auto&r=result["scaling_comparisons"][B][reference];r={{"candidate","field-quarter"},{"candidate_mean_complete_wall_s",candidate.at("complete_wall_s").at("mean")},{"reference_mean_complete_wall_s",base.at("complete_wall_s").at("mean")},{"both_complete",complete},{"candidate_mean_process_rss_kib",candidate.at("peak_rss_kib").at("mean")},{"reference_mean_process_rss_kib",base.at("peak_rss_kib").at("mean")}};if(complete)r.update({{"speedup_ratio_of_means",1.0/ratio.at("ratio").get<double>()},{"bootstrap_speedup_q025",1.0/ratio.at("bootstrap_q975").get<double>()},{"bootstrap_speedup_q975",1.0/ratio.at("bootstrap_q025").get<double>()}});}}
 }
 result["scope"]="Finite observed nondominance by mean complete wall and mean process-peak RSS; completing arms only, AA aliases excluded. No uncertainty-adjusted/global Pareto theorem. Derived speedup is reciprocal of native mean-cost ratio, not median paired ratio; censor comparisons never promoted.";std::ofstream out(argv[2]);out<<result.dump(2)<<'\n';
}catch(const std::exception&e){std::cerr<<e.what()<<'\n';return 1;}}
