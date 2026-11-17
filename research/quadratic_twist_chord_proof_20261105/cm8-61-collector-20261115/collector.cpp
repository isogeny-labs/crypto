// Finite conditional-state analysis of independently replayed partial-certificate kernels.
#include <nlohmann/json.hpp>
#include <openssl/sha.h>
#include <array>
#include <vector>
#include <map>
#include <fstream>
#include <iostream>
#include <sstream>
#include <filesystem>
using J=nlohmann::json;using V=std::array<int,3>;using S=std::array<int,9>;
void need(bool x){if(!x)throw std::runtime_error("collector contract");}int f(int x){x%=31;return x<0?x+31:x;}int inv(int x){need(x!=0);for(int i=1;i<31;i++)if(f(x*i)==1)return i;throw std::runtime_error("inverse");}
S canonical(std::vector<V> a){int k=0;for(int j=0;j<3;j++){int i=k;while(i<(int)a.size()&&!f(a[i][j]))i++;if(i==(int)a.size())continue;std::swap(a[i],a[k]);int z=inv(f(a[k][j]));for(int&x:a[k])x=f(x*z);for(int u=0;u<(int)a.size();u++)if(u!=k){int c=f(a[u][j]);for(int t=0;t<3;t++)a[u][t]=f(a[u][t]-c*a[k][t]);}k++;}S s{};for(int i=0;i<k;i++)for(int j=0;j<3;j++)s[3*i+j]=a[i][j];return s;}
S join(S s,const std::vector<V>&r){std::vector<V>a=r;for(int i=0;i<3;i++)a.push_back({s[3*i],s[3*i+1],s[3*i+2]});return canonical(a);}int rank(S s){int n=0;for(int i=0;i<3;i++)n+=(s[3*i]||s[3*i+1]||s[3*i+2]);return n;}
std::string read(std::string p){std::ifstream f(p);need(bool(f));return{std::istreambuf_iterator<char>(f),{}};}std::string sha(std::string s){unsigned char b[32];SHA256((const unsigned char*)s.data(),s.size(),b);std::ostringstream o;const char*h="0123456789abcdef";for(auto x:b)o<<h[x>>4]<<h[x&15];return o.str();}
J analyze(std::string dir){J index=J::parse(read(dir+"/index.json"));need(sha(read(dir+"/index.json"))=="cf7b8cc7cfc4b400849f01babeb2e9b54e4a128515fcb86e14474585d0a01225");std::map<std::string,std::string>pins;for(auto r:index["cases"])pins[r["path"]]=r["sha256"];J cases=J::array();for(int cap=0;cap<=8;cap++){
 std::array<J,2>data;std::vector<V>kernel[2][30];J queries=J::array();for(int arm=0;arm<2;arm++){std::string name=std::string(arm?"matched":"main")+"-0-"+std::to_string(cap)+".json";std::string bytes=read(dir+"/"+name);need(sha(bytes)==pins.at(name));data[arm]=J::parse(bytes);need(data[arm]["queries"].size()==31);for(int q=0;q<30;q++)for(auto row:data[arm]["queries"][q+1]["rows"])kernel[arm][q].push_back(row.get<V>());}
 for(int q=0;q<30;q++)queries.push_back({{"source",data[0]["queries"][q+1]["query"]},{"image",data[1]["queries"][q+1]["query"]},{"source_rows",kernel[0][q]},{"twist_rows",kernel[1][q]}});
 std::vector<S>states(1);std::map<S,int>seen{{S{},0}};J rows=J::array();for(size_t k=0;k<states.size();k++){S s=states[k];int r=rank(s);long a=0,b=0,d=0,ss=0,st=0;std::vector<int>dest;
 for(int q=0;q<30;q++){S x=join(s,kernel[0][q]),y=join(s,kernel[1][q]),z=join(x,kernel[1][q]);a+=rank(x)-r;b+=rank(y)-r;d+=rank(z)-r;for(S t:{x,y,z}){auto it=seen.find(t);if(it==seen.end()){int id=states.size();seen[t]=id;states.push_back(t);it=seen.find(t);}dest.push_back(it->second);}for(int u=0;u<30;u++){ss+=rank(join(x,kernel[0][u]))-r;st+=rank(join(x,kernel[1][u]))-r;}}
 need(states.size()<=1988);rows.push_back({{"basis",s},{"rank",r},{"destinations",dest},{"source_one_numerator",a},{"twist_one_numerator",b},{"dual_same_query_numerator",d},{"two_iid_source_numerator",ss},{"iid_source_then_twist_numerator",st},{"dual_beats_two_iid_source",30*d>ss},{"one_denominator",30},{"two_denominator",900}});
 }
 cases.push_back({{"cap",cap},{"source_policy",0},{"queries",queries},{"states",rows},{"source_call_cost",1},{"twist_call_cost",1},{"dual_call_cost",2},{"two_source_call_cost",2},{"parent_source_base",data[0]["base"]},{"parent_twist_base",data[1]["base"]}});
 }return J{{"schema","twist.cm61-collector-preflight/v1"},{"fixture",{{"field",61},{"r",31}}},{"parent_index_sha256",sha(read(dir+"/index.json"))},{"cases",cases}};}
void write(std::string path,J j){need(!std::filesystem::exists(path));std::ofstream f(path);f<<j.dump()<<'\n';need(bool(f));}
int main(int n,char**v){try{if(n==4&&std::string(v[1])=="produce"){write(v[3],analyze(v[2]));return 0;}if(n==5&&std::string(v[1])=="mutate"){J j=J::parse(read(v[2])),old=j;std::string m=v[4];auto&c=j["cases"][0];if(m=="source")j["fixture"]["field"]=59;else if(m=="free-second-call")c["dual_call_cost"]=1;else if(m=="missing-state")c["states"].erase(c["states"].end()-1);else if(m=="state")c["states"][0]["basis"][0]=1;else if(m=="rank")c["states"][0]["rank"]=1;else if(m=="increment")c["states"][0]["source_one_numerator"]=1;else if(m=="multiplicity")c["states"][0]["two_denominator"]=899;else if(m=="coupling")c["queries"][0]["image"]=c["queries"][1]["image"];else if(m=="missing-failed-query")c["queries"].erase(c["queries"].end()-1);else if(m=="missing-cap")j["cases"].erase(j["cases"].end()-1);else throw std::runtime_error("unknown mode");need(j!=old);write(v[3],j);return 0;}return 2;}catch(const std::exception&e){std::cerr<<e.what()<<'\n';return 101;}}
