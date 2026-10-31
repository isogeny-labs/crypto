// Integer symbolic identities and finite character/bound arithmetic.
// Certificates match identity.certificate/v1. Exact coefficient checks are
// independent of certificate evaluation; neither is a formal geometric proof.
#include "json.hpp"
#include <openssl/sha.h>
#include <array>
#include <cmath>
#include <cstdint>
#include <fstream>
#include <iomanip>
#include <iostream>
#include <map>
#include <sstream>
#include <stdexcept>
#include <string>
#include <vector>
using json=nlohmann::json;
constexpr uint64_t MOD=(uint64_t(1)<<61)-1;
std::array<unsigned char,32>sha(const std::string&s){std::array<unsigned char,32>h{};SHA256(reinterpret_cast<const unsigned char*>(s.data()),s.size(),h.data());return h;}
std::string hex(const std::array<unsigned char,32>&h){std::ostringstream o;for(auto v:h)o<<std::hex<<std::setw(2)<<std::setfill('0')<<int(v);return o.str();}
uint64_t next(uint64_t&s){s+=0x9e3779b97f4a7c15ULL;uint64_t z=s;z=(z^(z>>30))*0xbf58476d1ce4e5b9ULL;z=(z^(z>>27))*0x94d049bb133111ebULL;return z^(z>>31);}
json var(const std::string&s){return{{"op","var"},{"args",s}};}json con(int64_t c){return{{"op","const"},{"args",c}};}
json add(std::initializer_list<json>v){return{{"op","add"},{"args",json(v)}};}json mul(std::initializer_list<json>v){return{{"op","mul"},{"args",json(v)}};}
json power(json a,int n){json v=json::array();for(int i=0;i<n;i++)v.push_back(a);return{{"op","mul"},{"args",v}};}
uint64_t eval(const json&e,const std::map<std::string,uint64_t>&v){std::string op=e["op"];if(op=="var")return v.at(e["args"].get<std::string>());if(op=="const"){int64_t c=e["args"];return c<0?MOD-uint64_t(-c)%MOD:uint64_t(c)%MOD;}if(op=="neg"){auto a=eval(e["args"],v);return a?MOD-a:0;}uint64_t a=op=="mul"?1:0;for(auto&t:e["args"]){uint64_t b=eval(t,v);a=op=="mul"?uint64_t(__uint128_t(a)*b%MOD):(a+b)%MOD;}return a;}
using Poly=std::map<std::vector<int>,int64_t>;
Poly exact(const json&e,const std::vector<std::string>&vars){std::string op=e["op"];if(op=="const")return{{std::vector<int>(vars.size()),e["args"].get<int64_t>()}};if(op=="var"){std::vector<int>v(vars.size());bool found=false;for(size_t i=0;i<vars.size();i++)if(e["args"]==vars[i]){v[i]=1;found=true;}if(!found)throw std::runtime_error("undeclared variable");return{{v,1}};}if(op=="neg"){auto a=exact(e["args"],vars);for(auto&[k,v]:a)v=-v;return a;}Poly p;if(op=="mul")p[std::vector<int>(vars.size())]=1;for(auto&t:e["args"]){auto a=exact(t,vars);if(op=="add"){for(auto&[k,v]:a)p[k]+=v;}else{Poly b;for(auto&[i,c]:p)for(auto&[j,d]:a){auto k=i;for(size_t n=0;n<k.size();n++)k[n]+=j[n];b[k]+=c*d;}p=b;}}return p;}
json issue(const std::string&name,const std::vector<std::string>&vars,int degree,json lhs,json rhs,const std::string&source="GEOMETRIC_CHART_BOUND.md / projective_divisor.cpp; exact integer expansion"){
 json statement={{"name",name},{"source",source},{"variables",vars},{"degree_bound",degree},{"lhs",lhs},{"rhs",rhs}};
 auto a=exact(lhs,vars),b=exact(rhs,vars);for(auto&[k,v]:b)a[k]-=v;for(auto&[k,v]:a)if(v)throw std::runtime_error("integer coefficient identity failed");
 std::string id="IDC1h"+hex(sha(statement.dump())).substr(0,16);uint64_t seed=20261025;std::string material=id;for(int i=0;i<8;i++)material.push_back(char(seed>>(8*i)));material+="cert";auto digest=sha(material);uint64_t state=0;for(int i=0;i<8;i++)state|=uint64_t(digest[i])<<(8*i);std::string values;
 for(int k=0;k<64;k++){std::map<std::string,uint64_t>v;for(auto&s:vars)v[s]=next(state)%MOD;uint64_t l=eval(lhs,v),r=eval(rhs,v);if(l!=r)throw std::runtime_error("certificate evaluation disagreement");for(int i=0;i<8;i++)values.push_back(char(l>>(8*i)));}
 return{{"schema","identity.certificate/v1"},{"id",id},{"statement",statement},{"modulus",MOD},{"points",64},{"seed",seed},{"value_digest",hex(sha(values))},{"independence","Issued by native C++ port of the documented derivation; also checked by exact integer coefficient expansion. Same session; no independent reviewer or formal proof."},{"proof",nullptr}};
}
json read(const std::string&p){std::ifstream f(p);json d;f>>d;return d;}void write(const std::string&p,const json&d){std::ofstream f(p);f<<d.dump(2)<<'\n';}
int main(int argc,char**argv){try{
 if(argc!=2)throw std::runtime_error("usage: proof_audit RESULTS_DIR");
 std::string d=argv[1];auto u=var("u"),p=var("p"),X=var("X"),Y=var("Y"),Z=var("Z");auto shift=add({u,con(19)});
 auto c1=issue("positive chart-bound polynomial after p=u+19",{"u"},3,add({mul({con(2),power(shift,3)}),mul({con(-31),power(shift,2)}),mul({con(-42),shift}),con(-36)}),add({mul({con(2),power(u,3)}),mul({con(83),power(u,2)}),mul({con(946),u}),con(1693)}));
 auto c2=issue("projective Fermat chart transformation",{"X","Y","Z"},4,add({power(X,4),power(Y,4),power(add({Z,mul({con(2),Y})}),4)}),add({power(X,4),mul({con(17),power(Y,4)}),power(Z,4),mul({con(8),Y,power(Z,3)}),mul({con(24),power(Y,2),power(Z,2)}),mul({con(32),power(Y,3),Z})}));
 auto c3=issue("symmetric-square point count specialization",{"p"},2,add({power(add({p,con(1)}),2),power(p,2),con(1)}),mul({con(2),add({power(p,2),p,con(1)})}));
 write(d+"/chart-bound.identity.json",c1);write(d+"/projective-transform.identity.json",c2);write(d+"/symmetric-square.identity.json",c3);
 json bounds=json::array();for(uint64_t prime:{19,109,157}){uint64_t b=5*prime*prime+7*prime+6;uint64_t r=prime==19?6871:prime==109?1293169:3868159;double fail=double(3*b)/double(r-1);if(3*b>=r-1)throw std::runtime_error("recorded chart inequality failed");bounds.push_back({{"p",prime},{"r",r},{"bad_class_upper_bound",b},{"bad_common_multipliers_upper_bound",3*b},{"deterministic_trial_bound",3*b+1},{"independent_uniform_trial_failure_bound",fail},{"eight_independent_uniform_failures_bound",std::pow(fail,8)}});}
 auto ob=read(d+"/obstruction-native.json");json stars=json::array();int replays=0;for(auto&row:ob["cases"]){if(!row.contains("screen"))continue;replays++;if(row["screen"]["usable_count"]!=0)continue;int product=1,N=0;for(auto&n:row["neighbors"]){if(!n.contains("screen"))continue;for(auto&k:n["screen"]["kernels"]){uint64_t a=k["representative"],prime=row["p"];uint64_t v=1,e=(prime-1)/2;while(e){if(e&1)v=v*a%prime;a=a*a%prime;e>>=1;}int sign=v==1?1:-1;if(k["square"]!=(sign==1))throw std::runtime_error("Legendre recheck failed");product*=sign;N++;}}stars.push_back({{"p",row["p"]},{"neighbor_candidates",N},{"neighbor_product_character",product},{"all_negative_product_would_be",N%2?-1:1},{"scope","two fixed centers; no rational-function identity established"}});}
 write(d+"/proof-audit.json",{{"status","passed"},{"certificate_ids",{c1["id"],c2["id"],c3["id"]}},{"exact_integer_coefficient_checks",3},{"formal_geometric_proof",false},{"native_prepared_curve_replays",replays},{"chart_bounds",bounds},{"finite_star_characters",stars},{"extra_step_claim","unresolved; labelled modular divisors and arithmetic constants not computed"}});std::cout<<"proof audit passed\n";
 return 0;
 }catch(const std::exception&e){std::cerr<<e.what()<<'\n';return 2;}}
