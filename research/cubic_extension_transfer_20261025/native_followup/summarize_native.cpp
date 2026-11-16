// Native analysis of frozen run records; no rerunning algorithms here.
#include "json.hpp"
#include <algorithm>
#include <cmath>
#include <fstream>
#include <iomanip>
#include <iostream>
#include <map>
#include <sstream>
#include <stdexcept>
#include <string>
#include <vector>
using json=nlohmann::json;
json read(const std::string&p){std::ifstream f(p);if(!f)throw std::runtime_error("missing "+p);json d;f>>d;return d;}
json stats(std::vector<double>v){if(v.empty())throw std::runtime_error("empty sample");std::sort(v.begin(),v.end());double median=v.size()%2?v[v.size()/2]:(v[v.size()/2-1]+v[v.size()/2])/2;return{{"n",v.size()},{"median",median},{"min",v.front()},{"max",v.back()}};}
int main(int argc,char**argv){try{
 if(argc!=2)throw std::runtime_error("usage: summarize_native RESULTS_DIR");
 std::string d=argv[1];auto reservation=read(d+"/pinned-diagnostic/reservation.json");if(reservation["status"]!="complete")throw std::runtime_error("run did not complete");
 std::map<std::string,std::vector<json>>groups,aa;int verified=0,contended=0,qualified=0;json raw=json::array();
 for(auto&receipt:reservation["runs"]){auto row=read(d+"/pinned-diagnostic/"+receipt["stdout"].get<std::string>());if(receipt["exit_status"]!=0||row["verified"]!=true||row["scalar"]!=(row["p"]==109?1729:271828))throw std::runtime_error("unsuccessful or wrong-answer run");verified++;contended+=receipt["contended"].get<bool>();qualified+=receipt["qualified_wall_time"].get<bool>();
  row["receipt"]=receipt;raw.push_back(row);std::string key=std::to_string(row["p"].get<int>())+":"+row["arm"].get<std::string>();if(receipt["job"]["control"]=="main")groups[key].push_back(row);if(row["seed"]==20261025&&(row["arm"]=="source-rho-neg"||row["arm"]=="target-ic"))aa[key].push_back(row);
 }
 json rows=json::array(),controls=json::array();std::ofstream csv(d+"/solver-counts.csv");csv<<"p,arm,n,M_median,M_min,M_max,I_median,I_min,I_max,solver_seconds_median,solver_seconds_min,solver_seconds_max,qualified_wall_time\n";
 for(auto&[key,v]:groups){if(v.size()!=5)throw std::runtime_error("main arm lacks five samples");json r={{"p",v[0]["p"]},{"arm",v[0]["arm"]}};for(auto metric:{"M","I","NAT","LA","solver_seconds","stage_total_seconds","target_fixture_seconds"}){std::vector<double>values;for(auto&row:v)if(row.contains(metric))values.push_back(row[metric]);if(!values.empty())r[metric]=stats(values);}r["qualified_wall_time"]=false;rows.push_back(r);csv<<r["p"]<<','<<r["arm"].get<std::string>()<<",5,"<<r["M"]["median"]<<','<<r["M"]["min"]<<','<<r["M"]["max"]<<','<<r["I"]["median"]<<','<<r["I"]["min"]<<','<<r["I"]["max"]<<','<<r["solver_seconds"]["median"]<<','<<r["solver_seconds"]["min"]<<','<<r["solver_seconds"]["max"]<<",false\n";
 }
 for(auto&[key,v]:aa){if(v.size()!=3)throw std::runtime_error("A/A control lacks main plus two repetitions");bool same=true;for(size_t i=1;i<v.size();i++)for(auto metric:{"M","I","NAT","LA","scalar","steps"})if(v[i][metric]!=v[0][metric])same=false;if(!same)throw std::runtime_error("A/A counted output changed");std::vector<double>time;for(auto&row:v)time.push_back(row["solver_seconds"]);controls.push_back({{"p",v[0]["p"]},{"arm",v[0]["arm"]},{"same_seed_counted_output_equal",true},{"solver_seconds_unqualified",stats(time)}});}
 json result={{"status","passed"},{"requested_runs",58},{"completed_runs",verified},{"main_runs",50},{"additional_AA_runs",8},{"correct_answers",verified},{"contended_runs_by_visible_tick_monitor",contended},{"qualified_wall_time_runs",qualified},{"isolation_limits",{"CPU and memory PSI unavailable","Unmovable system threads remain eligible for reserved CPU","Virtual host neighbors and frequency uncontrolled"}},{"scope","Native direct source controls and synthetic target quartic stages; no native elliptic transfer or end-to-end comparison"},{"groups",rows},{"AA_controls",controls},{"external_fastest_tool_comparison","not performed"},{"independent_host_replay","not performed"}};
 std::ofstream out(d+"/benchmark-summary.json");out<<result.dump(2)<<'\n';
 std::ofstream svg(d+"/native-counts.svg");svg<<"<svg xmlns='http://www.w3.org/2000/svg' width='900' height='570' viewBox='0 0 900 570'><rect width='900' height='570' fill='white'/><g font-family='sans-serif' fill='#182334'><text x='35' y='30' font-size='19'>Native solver operation counts: five seeds per arm</text><text x='35' y='51' font-size='12'>M = base-field multiplications/squarings. Median with min/max; logarithmic axis.</text>";
 std::vector<std::string>arms={"source-rho","source-rho-neg","target-ic","target-rho","target-rho-neg"};std::vector<std::string>labels={"source rho","source rho +/-","target IC","target rho","target rho +/-"};
 const double left=240,right=840;double lower=4,upper=9;auto x=[&](double v){return left+(std::log10(std::max(1.,v))-lower)/(upper-lower)*(right-left);};
 for(int power=4;power<=9;power++){double xx=x(std::pow(10,power));svg<<"<path d='M"<<xx<<" 80 V460' stroke='#e3e8ef'/><text x='"<<xx-10<<"' y='480' font-size='11'>10^"<<power<<"</text>";}
 int index=0;for(int prime:{109,157})for(size_t a=0;a<arms.size();a++){json row;for(auto&r:rows)if(r["p"]==prime&&r["arm"]==arms[a])row=r;double y=92+index*36;svg<<"<text x='32' y='"<<y+4<<"' font-size='13'>p="<<prime<<" "<<labels[a]<<"</text><path d='M"<<x(row["M"]["min"])<<' '<<y<<" H"<<x(row["M"]["max"])<<"' stroke='#405773' stroke-width='2'/><circle cx='"<<x(row["M"]["median"])<<"' cy='"<<y<<"' r='5' fill='"<<(prime==109?"#3265ae":"#c06424")<<"'/>";index++;}
 svg<<"<text x='35' y='513' font-size='12'>Run: RUN_MANIFEST.json; raw receipts: results/pinned-diagnostic/reservation.json</text><text x='35' y='535' font-size='12'>Target construction/transfer/inversion absent. Counts are solver-stage diagnostics.</text><text x='35' y='555' font-size='12'>Wall times are unqualified: missing PSI and incomplete core reservation.</text></g></svg>\n";
 std::cout<<result.dump(2)<<'\n';return 0;
 }catch(const std::exception&e){std::cerr<<e.what()<<'\n';return 2;}}
