#include <algorithm>
#include <array>
#include <cmath>
#include <cstdint>
#include <fstream>
#include <functional>
#include <iomanip>
#include <iostream>
#include <map>
#include <numeric>
#include <sstream>
#include <stdexcept>
#include <string>
#include <tuple>
#include <vector>
using V=std::vector<double>;
void need(bool b,const char* s){if(!b)throw std::runtime_error(s);}
double mean(const V& x){need(!x.empty(),"empty mean");return std::accumulate(x.begin(),x.end(),0.0)/x.size();}
double var(const V& x){need(x.size()>1,"short variance");double m=mean(x),z=0;for(double a:x)z+=(a-m)*(a-m);return z/(x.size()-1);}
double cov(const V& x,const V& y){need(x.size()==y.size()&&x.size()>1,"covariance shape");double a=mean(x),b=mean(y),z=0;for(size_t i=0;i<x.size();i++)z+=(x[i]-a)*(y[i]-b);return z/(x.size()-1);}
double phi(double x){return .5*std::erfc(-x/std::sqrt(2.0));}
double power(double effect,double se){double lo=std::log(.95),hi=std::log(1.05),z=2.3263478740408408;double a=lo+z*se,b=hi-z*se;if(a>=b)return 0;return std::max(0.0,phi((b-effect)/se)-phi((a-effect)/se));}
std::vector<std::string> fields(const std::string& s){std::vector<std::string> v;std::stringstream ss(s);std::string x;while(std::getline(ss,x,','))v.push_back(x);return v;}
using Key=std::tuple<int,int,int>;
struct Interval{double estimate,lo,hi;size_t compositions;uint64_t ordered;};
Interval exact(const V& logs){need(logs.size()==8,"eight class logs");std::array<uint64_t,9> f{1};for(int i=1;i<=8;i++)f[i]=f[i-1]*i;std::vector<std::pair<double,uint64_t>> dist;uint64_t total=0;
 std::function<void(int,int,double,uint64_t)> go=[&](int i,int left,double sum,uint64_t denominator){if(i==7){uint64_t w=f[8]/(denominator*f[left]);dist.push_back({std::exp((sum+left*logs[i])/8),w});total+=w;return;}for(int n=0;n<=left;n++)go(i+1,left-n,sum+n*logs[i],denominator*f[n]);};go(0,8,0,1);need(total==16777216&&dist.size()==6435,"bootstrap enumeration");std::sort(dist.begin(),dist.end());auto q=[&](double p){uint64_t rank=std::llround((total-1)*p),c=0;for(auto [x,w]:dist){c+=w;if(c>rank)return x;}throw std::runtime_error("quantile");};return{std::exp(mean(logs)),q(.01),q(.99),dist.size(),total};}
int main(int argc,char**argv){try{need(argc==2,"records-A.csv required");std::ifstream in(argv[1]);need(bool(in),"CSV unavailable");std::string line;std::getline(in,line);need(line=="class,volcano,level,model,arm,p,a,b,r,gx,gy,qx,qy,recovered,status,s,total_gae,target_index","CSV schema");std::map<Key,double> primary,control;std::array<uint64_t,8> p{},r{};size_t count=0;
 while(std::getline(in,line)){auto x=fields(line);need(x.size()==18,"CSV columns");if(x[2]!="0"||x[3]!="0"||(x[4]!="rho-neg"&&x[4]!="rho-neg-aa"))continue;need(x[14]=="verified","unverified observation");int c=std::stoi(x[0]),v=std::stoi(x[1]),t=std::stoi(x[17]);need(c>=0&&c<8&&v>=0&&v<4,"class/volcano range");auto key=Key{c,v,t};auto& dst=x[4]=="rho-neg"?primary:control;need(dst.emplace(key,std::stod(x[15])).second,"duplicate cell");p[c]=std::stoull(x[5]);r[c]=std::stoull(x[8]);count++;}
 need(primary.size()==6144&&control.size()==1024,"crater sample count");V logs,primary_var,seed_var,target_raw,target_var;std::array<std::array<double,2>,8> mus{};
 for(int c=0;c<8;c++){V a,b;for(int t=0;t<192;t++){a.push_back(primary.at({c,0,t}));b.push_back((primary.at({c,1,t})+primary.at({c,2,t})+primary.at({c,3,t}))/3);}double ma=mean(a),mb=mean(b);mus[c]={ma,mb};logs.push_back(std::log(mb/ma));V influence;for(int t=0;t<192;t++)influence.push_back(b[t]/mb-a[t]/ma);primary_var.push_back(var(influence));V u,v,diff;for(int t=0;t<32;t++){u.push_back(influence[t]);double aa=control.at({c,0,t}),bb=(control.at({c,1,t})+control.at({c,2,t})+control.at({c,3,t}))/3;v.push_back(bb/mb-aa/ma);diff.push_back(u.back()-v.back());}double raw=cov(u,v);target_raw.push_back(raw);target_var.push_back(std::max(0.0,raw));seed_var.push_back(var(diff)/2);}
 auto result=exact(logs),null=exact(V(8,0)),constant=exact(V(8,std::log(1.1)));
 need(null.estimate==1&&null.lo==1&&null.hi==1,"null enumeration control");
 need(std::abs(constant.lo-1.1)<1e-12&&std::abs(constant.hi-1.1)<1e-12,"constant ratio control");
 V two_point{0,0,0,0,std::log(1.2),std::log(1.2),std::log(1.2),std::log(1.2)};
 auto binomial=exact(two_point);
 need(std::abs(binomial.lo-std::pow(1.2,1.0/8))<1e-12&&
      std::abs(binomial.hi-std::pow(1.2,7.0/8))<1e-12,"binomial quantile control");
 need(power(0,.005)>.99&&power(std::log(1.1),.005)<1e-9,"power controls");
 need(std::abs(result.estimate-1.0111251967610384)<1e-12,"headline mismatch");
 double class_var=var(logs),noise=mean(primary_var)/192,tau=std::max(0.0,class_var-noise);
 std::cout<<std::setprecision(17)<<"{\n\"schema\":\"conductor-gap.crater-design/v1\",\"source\":\""<<argv[1]<<"\",\"measurements_added\":0,\n\"primary_crater_records\":"<<primary.size()<<",\"control_crater_records\":"<<control.size()<<",\n\"enumerated_whole_class_bootstrap\":{\"ratio\":"<<result.estimate<<",\"ci98\":["<<result.lo<<","<<result.hi<<"],\"compositions\":"<<result.compositions<<",\"ordered_resamples\":"<<result.ordered<<",\"equivalent_within_5pct\":"<<(result.lo>=.95&&result.hi<=1.05?"true":"false")<<"},\n\"planning_model\":{\"observed_class_log_variance\":"<<class_var<<",\"estimated_existing_sampling_variance_per_class\":"<<noise<<",\"truncated_between_class_variance\":"<<tau<<",\"average_seed_component\":"<<mean(seed_var)<<",\"average_target_component_truncated\":"<<mean(target_var)<<",\"warning\":\"Exploratory normal/delta-method power model; only 32 two-seed targets per class; negative target covariance truncated; class heterogeneity is not established or ruled out. No guaranteed power or population inference.\"},\n\"classes\":[";
 for(int c=0;c<8;c++){if(c)std::cout<<",";std::cout<<"{\"class\":"<<c<<",\"field_p\":"<<p[c]<<",\"field_bits\":"<<(64-__builtin_clzll(p[c]))<<",\"subgroup_r\":"<<r[c]<<",\"subgroup_bits\":"<<(64-__builtin_clzll(r[c]))<<",\"generic_work_exponent\":"<<.5*std::log2(double(r[c]))<<",\"ratio\":"<<std::exp(logs[c])<<",\"influence_variance\":"<<primary_var[c]<<",\"seed_component\":"<<seed_var[c]<<",\"target_covariance_raw\":"<<target_raw[c]<<",\"target_component_truncated\":"<<target_var[c]<<"}";}std::cout<<"],\n\"power_scenarios\":[";bool first=true;
 for(int classes:{8,16,32})for(int targets:{100,192,1000})for(int seeds:{1,2,5,10}){double within=(mean(target_var)+mean(seed_var)/seeds)/(classes*targets),se=std::sqrt(within+tau/classes),fixed=std::sqrt(within);if(!first)std::cout<<",";first=false;std::cout<<"{\"classes\":"<<classes<<",\"targets_per_crater\":"<<targets<<",\"seeds_per_target\":"<<seeds<<",\"rho_calls\":"<<uint64_t(classes)*4*targets*seeds<<",\"approx_se_log_ratio_fixed_cohort\":"<<fixed<<",\"approx_power_at_ratio1_fixed_cohort\":"<<power(0,fixed)<<",\"approx_power_at_observed_ratio_fixed_cohort\":"<<power(mean(logs),fixed)<<",\"approx_power_at_ratio1_with_estimated_heterogeneity\":"<<power(0,se)<<",\"approx_power_at_observed_ratio_with_estimated_heterogeneity\":"<<power(mean(logs),se)<<"}";}
 std::cout<<"]\n}\n";
 }catch(const std::exception&e){std::cerr<<e.what()<<"\n";return 1;}}
