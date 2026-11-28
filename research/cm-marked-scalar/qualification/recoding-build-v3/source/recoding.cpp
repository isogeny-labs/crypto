#define main cm_native_main
#include "native.cpp"
#undef main
#include <filesystem>
#include <tuple>
#include <iomanip>
#include <openssl/sha.h>
static std::string digest(const std::string& bytes){unsigned char h[SHA256_DIGEST_LENGTH];SHA256(reinterpret_cast<const unsigned char*>(bytes.data()),bytes.size(),h);std::ostringstream out;for(auto c:h)out<<std::hex<<std::setw(2)<<std::setfill('0')<<unsigned(c);return out.str();}
static std::string file_digest(const std::string& path){std::ifstream in(path,std::ios::binary);require(bool(in),"hash input missing");std::ostringstream bytes;bytes<<in.rdbuf();return digest(bytes.str());}
static Json text_token(const std::string& value){Json j;j.kind=Json::String;j.value=value;return j;}
static Json integer_token(long n){Json j;j.kind=Json::Number;j.value=std::to_string(n);return j;}
static Json fields(std::initializer_list<std::pair<const std::string,Json>> list){Json j;j.kind=Json::Object;j.object=list;return j;}
static Counts analytic_count(const Big& scalar,unsigned width){
 auto digits=wnaf(scalar,width);require(!digits.empty(),"nonzero scalar plan");unsigned nonzero=0,negative=0;Big reconstruct;
 for(auto it=digits.rbegin();it!=digits.rend();++it){require(BN_lshift1(reconstruct.p,reconstruct.p),"digit reconstruction shift");if(*it){++nonzero;if(*it<0)++negative;Big term(*it);require(BN_add(reconstruct.p,reconstruct.p,term.p),"digit reconstruction add");}}
 if(BN_is_negative(scalar.p))BN_set_negative(reconstruct.p,!reconstruct.zero());
 require(reconstruct==scalar,"signed digit reconstruction");
 size_t table=1u<<(width-2);return {uint64_t(digits.size()-1+(table>1)),uint64_t(table-1+nonzero-1),uint64_t(negative+BN_is_negative(scalar.p)),uint64_t(table)};
}
static Counts sum(Counts a,const Counts& b){a.doubles+=b.doubles;a.adds+=b.adds;a.negations+=b.negations;a.table_points+=b.table_points;return a;}
static Big translate(const Big& base,const Big& order,int shift,BN_CTX* ctx){Big delta(shift),scalar;require(BN_mul(delta.p,delta.p,order.p,ctx)&&BN_add(scalar.p,base.p,delta.p),"modular scalar translation");return scalar;}
struct Choice{int shift=0;unsigned window=0;Counts count;};
using Rank=std::tuple<uint64_t,uint64_t,uint64_t,uint64_t,int,int,unsigned>;
static Rank rank_choice(const Choice& p,unsigned weight){return {p.count.doubles+weight*p.count.adds,p.count.negations,p.count.doubles,p.count.adds,std::abs(p.shift),p.shift,p.window};}
static void improve(Choice& best,const Choice& p,unsigned weight){if(!best.window||rank_choice(p,weight)<rank_choice(best,weight))best=p;}
static Json choice_json(const Choice& c){return fields({{"shift",integer_token(c.shift)},{"window",integer_token(c.window)},{"doubles",integer_token(c.count.doubles)},{"adds",integer_token(c.count.adds)},{"negations",integer_token(c.count.negations)},{"table_points",integer_token(c.count.table_points)}});}
static std::string check_protocol(const std::string& path,const std::string& directory){
 auto protocol=read_json(path);require(protocol.at("freeze").at("status").str()=="frozen","recoding protocol not frozen");auto hash=protocol.at("freeze").at("protocol_sha256").str();Json body=protocol;body.object.erase("freeze");require(digest(canonical_json(body))==hash,"recoding protocol digest");
 require(file_digest("/proc/self/exe")==protocol.at("arms").at(size_t(0)).at("instrument").at("binary_sha256").str(),"recoding binary binding");
 for(const auto& file:protocol.at("execution").at("placement").at("input_files").array)require(file_digest(directory+"/"+file.at("path").str())==file.at("sha256").str(),"recoding fixture binding");
 return hash;
}
static void scan(const std::string& directory,const std::string& protocol_path,const std::string& out_directory){
 auto protocol_hash=check_protocol(protocol_path,directory);require(!std::filesystem::exists(out_directory),"write-once recoding output");std::filesystem::create_directories(out_directory);std::ofstream csv(out_directory+"/scan.csv");require(bool(csv),"scan output");csv<<"curve,family,window,shift,doubles,adds,negations,table_points\n";
 Json plans;plans.kind=Json::Array;unsigned rows=0;
 for(auto& f:fixtures(directory)){
  Runtime r(f,true);Big bases[]={centered(r.mu,r.n),centered(r.correction,r.n)};Counts small=analytic_count(Big(r.forward->degree),2);Choice best[2][4],legacy[2][4];
  for(unsigned family=0;family<2;++family)for(unsigned width=2;width<=7;++width)for(int shift=-256;shift<=256;++shift){
   Big scalar=translate(bases[family],r.n,shift,r.field.ctx);Choice raw{shift,width,analytic_count(scalar,width)};csv<<f.name<<','<<family<<','<<width<<','<<shift<<','<<raw.count.doubles<<','<<raw.count.adds<<','<<raw.count.negations<<','<<raw.count.table_points<<'\n';++rows;
   Choice charged=raw;if(family)charged.count=sum(charged.count,small);
   for(unsigned w=1;w<=4;++w){improve(best[family][w-1],charged,w);if(!shift)improve(legacy[family][w-1],charged,w);}
  }
  for(unsigned w=1;w<=4;++w)plans.array.push_back(fields({{"curve",text_token(f.name)},{"addition_weight",integer_token(w)},{"direct",choice_json(best[0][w-1])},{"fused",choice_json(best[1][w-1])},{"legacy_direct",choice_json(legacy[0][w-1])},{"legacy_fused",choice_json(legacy[1][w-1])}}));
 }
 csv.close();require(rows==30780,"complete finite recoding scan");Json output=fields({{"schema",text_token("cm-recoding-plans/v1")},{"protocol_sha256",text_token(protocol_hash)},{"scan_sha256",text_token(file_digest(out_directory+"/scan.csv"))},{"rows",integer_token(rows)},{"plans",plans}});std::ofstream out(out_directory+"/plans.json");out<<canonical_json(output)<<'\n';require(bool(out),"plan output");std::cout<<"{\"complete_scan_rows\":"<<rows<<",\"timing_performed\":false}\n";
}
static bool equal_counts(const Counts& a,const Counts& b){return a.doubles==b.doubles&&a.adds==b.adds&&a.negations==b.negations&&a.table_points==b.table_points;}
static void check_point_plan(Runtime& r,const EC_POINT* P,const EC_POINT* expected,unsigned width,int shift,bool fused){
 Big base=centered(fused?r.correction:r.mu,r.n),scalar=translate(base,r.n,shift,r.field.ctx),residue;require(BN_nnmod(residue.p,scalar.p,r.n.p,r.field.ctx),"translated residue");require(residue==(fused?r.correction:r.mu),"translated scalar label");Counts actual;auto out=point(r.group);Point small(nullptr,EC_POINT_free);const EC_POINT* source=P;
 if(fused){small=point(r.group);public_mul(r,small.get(),P,Big(r.forward->degree),2,actual);source=small.get();}
 public_mul(r,out.get(),source,scalar,width,actual,false);require(EC_POINT_cmp(r.group,out.get(),expected,r.field.ctx)==0,"translated point action");Counts predicted;if(!EC_POINT_is_at_infinity(r.group,P)){predicted=analytic_count(scalar,width);if(fused)predicted=sum(predicted,analytic_count(Big(r.forward->degree),2));}
 if(!equal_counts(actual,predicted))std::cerr<<"count mismatch curve="<<r.f.name<<" window="<<width<<" shift="<<shift<<" fused="<<fused<<" actual="<<actual.doubles<<','<<actual.adds<<','<<actual.negations<<','<<actual.table_points<<" predicted="<<predicted.doubles<<','<<predicted.adds<<','<<predicted.negations<<','<<predicted.table_points<<'\n';
 require(equal_counts(actual,predicted),"analytic and actual group counts");
}
static void qualify_recoding(const std::string& directory,const std::string& plans_path=""){
 Json plans;if(!plans_path.empty())plans=read_json(plans_path).at("plans");unsigned checks=0;
 for(auto& f:fixtures(directory)){Runtime r(f,true);for(long input:{0,1,2,17,-1}){
  Big k(input);if(input<0)require(BN_add(k.p,k.p,r.n.p),"negative input");auto P=point(r.group),expected=point(r.group);require(EC_POINT_mul(r.group,P.get(),k.p,nullptr,nullptr,r.field.ctx)&&EC_POINT_mul(r.group,expected.get(),nullptr,P.get(),r.mu.p,r.field.ctx),"independent point reference");
  if(plans_path.empty()){for(unsigned width=2;width<=7;++width)for(int shift:{-256,-1,0,1,256})for(bool fused:{false,true}){check_point_plan(r,P.get(),expected.get(),width,shift,fused);++checks;}}
  else for(const auto& plan:plans.array)if(plan.at("curve").str()==f.name)for(auto arm:{"direct","fused"}){const auto& c=plan.at(arm);check_point_plan(r,P.get(),expected.get(),unsigned(c.at("window").integer()),c.at("shift").integer(),std::string(arm)=="fused");++checks;}
 }}
 require(checks==(plans_path.empty()?1500u:200u),"complete recoding qualification");std::cout<<"{\"recoding_point_and_schedule_checks\":"<<checks<<",\"timing_performed\":false}\n";
}
int main(int argc,char** argv){try{
 if(argc==3&&std::string(argv[1])=="--qualify"){qualify_recoding(argv[2]);return 0;}
 if(argc==4&&std::string(argv[1])=="--qualify-plans"){qualify_recoding(argv[2],argv[3]);return 0;}
 require(argc==5&&std::string(argv[1])=="--scan","recoding --qualify FIXTURES | --qualify-plans FIXTURES PLANS | --scan FIXTURES FROZEN_PROTOCOL OUT_DIRECTORY");scan(argv[2],argv[3],argv[4]);return 0;
}catch(const std::exception& e){std::cerr<<"native recoding refused/failed: "<<e.what()<<'\n';return 1;}}
