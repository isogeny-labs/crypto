/* Recompute DP rate ratios from native diagnostic records. No walk execution.
 * Paired bootstrap resamples complete seed namespaces, never individual steps
 * or timing repetitions. Input is the concatenated two-arm JSONL, one protocol. */
#include <algorithm>
#include <array>
#include <cmath>
#include <iostream>
#include <iomanip>
#include <map>
#include <regex>
#include <set>
#include <stdexcept>
#include <string>
#include <tuple>
#include <vector>
#include <cstdint>
#include <cctype>
struct Row {uint64_t dp,advanced,attempts,seed,walk,table;};
// Strict subset used by nist.seeded_dp/v1: strings, unsigned integers,
// arrays. Reject duplicate keys, wrong types, overflow and trailing bytes.
struct Json {
 const std::string&s;size_t at=0;
 std::map<std::string,std::pair<char,std::string>> fields;
 void ws(){while(at<s.size()&&std::isspace((unsigned char)s[at]))at++;}
 bool take(char c){ws();if(at<s.size()&&s[at]==c){at++;return true;}return false;}
 void need(char c){if(!take(c))throw std::runtime_error("invalid JSON syntax");}
 std::string str(){need('"');size_t start=at;while(at<s.size()&&s[at]!='"'){
  if(s[at]=='\\'||(unsigned char)s[at]<32)throw std::runtime_error("unsupported JSON string");
  at++;}
  auto out=s.substr(start,at-start);need('"');return out;}
 std::pair<char,std::string> value(int depth=0){
  if(depth>8)throw std::runtime_error("array depth");
  ws();
  if(at<s.size()&&s[at]=='"')return {'s',str()};
  if(take('[')){if(!take(']')){do{value(depth+1);}while(take(','));need(']');}return {'a',""};}
  size_t start=at;while(at<s.size()&&s[at]>='0'&&s[at]<='9')at++;
  auto n=s.substr(start,at-start);if(n.empty()||(n.size()>1&&n[0]=='0'))throw std::runtime_error("unsigned JSON integer required");
  (void)std::stoull(n);return {'n',n};
 }
 explicit Json(const std::string&line):s(line){need('{');if(!take('}')){do{
  auto key=str();need(':');auto v=value();if(!fields.emplace(key,v).second)throw std::runtime_error("duplicate JSON key");
 }while(take(','));need('}');}ws();if(at!=s.size())throw std::runtime_error("trailing JSON");}
 std::string get(const std::string&key,char type)const{
  auto i=fields.find(key);if(i==fields.end()||i->second.first!=type)throw std::runtime_error("missing/wrong type: "+key);return i->second.second;
 }
};
static uint64_t number(const Json&s,const std::string&key){
 return std::stoull(s.get(key,'n'));
}
static uint64_t draw(uint64_t&s){uint64_t z=(s+=0x9e3779b97f4a7c15ull);z=(z^(z>>30))*0xbf58476d1ce4e5b9ull;z=(z^(z>>27))*0x94d049bb133111ebull;return z^(z>>31);}
int main(){try{
 std::cout<<std::setprecision(17);
 using Key=std::tuple<std::string,uint64_t,uint64_t,uint64_t>; // curve,W,iters,bits
 using Pair=std::array<Row,2>;
 std::map<Key,std::map<uint64_t,Pair>> groups;std::set<std::tuple<Key,uint64_t,int>> seen;
 std::string s;
 while(std::getline(std::cin,s)){
  if(s.empty())continue;
  Json record(s);
  if(record.get("schema",'s')!="nist.seeded_dp/v1")throw std::runtime_error("unexpected schema");
  auto curve=record.get("curve",'s');if(curve!="P-256"&&curve!="P-384")throw std::runtime_error("curve");
  auto bits=number(record,"dp_bits");if(bits<1||bits>24)throw std::runtime_error("dp bits");
  Key key{curve,number(record,"W"),number(record,"iters"),bits};auto seed=number(record,"seed_id");auto arm_value=number(record,"best2");
  if(arm_value>1)throw std::runtime_error("invalid arm");
  int arm=(int)arm_value;
  if(!seen.emplace(key,seed,arm).second)throw std::runtime_error("duplicate seed/arm (timing repetitions are not samples)");
  Row r{number(record,"emitted_dp"),number(record,"advanced"),number(record,"attempts"),seed,number(record,"walk_seed"),number(record,"table_seed")};
  auto used=r.advanced;
  for(const auto*field:{"cycle_detect","seek","inf"}){auto n=number(record,field);if(n>r.attempts||used>r.attempts-n)throw std::runtime_error("accounting overflow/mismatch");used+=n;}
  if(r.attempts!=used||r.dp>r.advanced||r.attempts==0)throw std::runtime_error("accounting mismatch");
  groups[key][seed][arm]=r;
 }
 if(groups.empty())throw std::runtime_error("empty input");
 for(auto&[key,records]:groups){
  if(records.size()<8)throw std::runtime_error("at least 8 matched seed namespaces required (still not proof of independence)");
  std::set<std::pair<uint64_t,uint64_t>> namespaces;std::vector<Pair> rows;
  for(auto&[seed,pair]:records){
   if(!seen.count({key,seed,0})||!seen.count({key,seed,1}))throw std::runtime_error("unpaired arm");
   if(pair[0].walk!=pair[1].walk||pair[0].table!=pair[1].table||pair[0].attempts!=pair[1].attempts)throw std::runtime_error("unmatched namespace/work");
   if(!namespaces.emplace(pair[0].walk,pair[0].table).second)throw std::runtime_error("reused namespace under another seed id");
   rows.push_back(pair);
  }
  auto calc=[&](const std::vector<size_t>&idx){
   std::array<double,2>d{},a{},t{};for(auto i:idx)for(int b=0;b<2;b++){d[b]+=rows[i][b].dp;a[b]+=rows[i][b].advanced;t[b]+=rows[i][b].attempts;}
   if(d[0]==0||d[1]==0||a[0]==0||a[1]==0)throw std::runtime_error("too few DP events for rate ratio bootstrap; retain raw data");
   double scale=std::ldexp(1.0,(int)std::get<3>(key));
   return std::array<double,6>{d[0]*scale/t[0],d[1]*scale/t[1],d[0]*scale/a[0],d[1]*scale/a[1],(d[1]/a[1])/(d[0]/a[0]),(d[1]/t[1])/(d[0]/t[0])};
  };
  std::vector<size_t> ids(rows.size());for(size_t i=0;i<ids.size();i++)ids[i]=i;auto point=calc(ids);
  std::array<std::vector<double>,6> boots;uint64_t rng=20261027;
  for(int b=0;b<10000;b++){for(auto&i:ids)i=draw(rng)%rows.size();auto v=calc(ids);for(int k=0;k<6;k++)boots[k].push_back(v[k]);}
  for(auto&v:boots)std::sort(v.begin(),v.end());
  std::cout<<"{\"schema\":\"nist.seeded_dp.summary/v1\",\"curve\":\""<<std::get<0>(key)<<"\",\"W\":"<<std::get<1>(key)<<",\"iters\":"<<std::get<2>(key)<<",\"dp_bits\":"<<std::get<3>(key)<<",\"seed_clusters\":"<<rows.size()<<",\"bootstrap_seed\":20261027,\"resamples\":10000,\"metrics\":[";
  const char*names[]={"plain_dp_over_physical_expectation","best2_dp_over_physical_expectation","plain_dp_over_eligible_expectation","best2_dp_over_eligible_expectation","best2_over_plain_eligible_rate","best2_over_plain_physical_rate"};
  for(int k=0;k<6;k++)std::cout<<(k?",":"")<<"{\"name\":\""<<names[k]<<"\",\"point\":"<<point[k]<<",\"lo\":"<<boots[k][249]<<",\"hi\":"<<boots[k][9749]<<"}";
  std::cout<<"],\"interval\":\"95% paired seed-cluster percentile bootstrap; unadjusted exploratory intervals\"}\n";
 }
 return 0;
 }catch(const std::exception&e){std::cerr<<e.what()<<"\n";return 1;}}
