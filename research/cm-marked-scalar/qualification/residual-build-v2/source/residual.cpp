// Fixed-label quotient/remainder action. Existing sealed source is included,
// never edited. No clocks in this producer or its qualification paths.
#define main cm_native_main
#include "native.cpp"
#undef main
#include <filesystem>
#include <array>
#include <tuple>
#include <iomanip>
#include <openssl/sha.h>

namespace residual {
static std::string hash(const std::string& bytes){unsigned char h[SHA256_DIGEST_LENGTH];SHA256(reinterpret_cast<const unsigned char*>(bytes.data()),bytes.size(),h);std::ostringstream s;for(auto c:h)s<<std::hex<<std::setw(2)<<std::setfill('0')<<unsigned(c);return s.str();}
static std::string read(const std::string& path){std::ifstream f(path,std::ios::binary);require(bool(f),"residual input missing");std::ostringstream s;s<<f.rdbuf();return s.str();}
static Json number(long n){Json j;j.kind=Json::Number;j.value=std::to_string(n);return j;}
static Json text(const std::string&s){Json j;j.kind=Json::String;j.value=s;return j;}
static Json object(std::initializer_list<std::pair<const std::string,Json>> list){Json j;j.kind=Json::Object;j.object=list;return j;}
static Big neg(Big b){if(!b.zero())BN_set_negative(b.p,!BN_is_negative(b.p));return b;}
struct Split{Big k,c;};
static Split split(Runtime& r,unsigned radix,int offset){
 Big s=centered(r.mu,r.n),v{long(radix)},rem,k,c;
 require(BN_nnmod(rem.p,s.p,v.p,r.field.ctx),"Euclidean remainder");
 Big twice;require(BN_lshift1(twice.p,rem.p),"remainder center shift");if(BN_cmp(twice.p,v.p)>0)require(BN_sub(rem.p,rem.p,v.p),"center remainder");
 Big delta(offset);require(BN_mul(delta.p,delta.p,v.p,r.field.ctx)&&BN_add(c.p,rem.p,delta.p)&&BN_sub(k.p,s.p,c.p),"residual offset");
 Big unused;require(BN_div(k.p,unused.p,k.p,v.p,r.field.ctx)&&unused.zero(),"exact quotient");
 Big identity;require(BN_mul(identity.p,k.p,v.p,r.field.ctx)&&BN_add(identity.p,identity.p,c.p)&&identity==s,"fixed scalar reconstruction");return {k,c};
}
static std::vector<int> digits(const Big& n,unsigned width){auto out=wnaf(n,width);if(BN_is_negative(n.p))for(auto& d:out)d=-d;return out;}
static Counts direct_count(const Big& n,unsigned width){
 if(n.zero())return {};
 auto ds=wnaf(n,width);uint64_t nonzero=0,negative=0;for(int d:ds){nonzero+=d!=0;negative+=d<0;}
 uint64_t table=1u<<(width-2);return {ds.size()-1+uint64_t(table>1),table-1+nonzero-1,negative+uint64_t(BN_is_negative(n.p)),table};
}
static Counts counter(const Split& s,unsigned radix,unsigned width){
 Big k=s.k,c=s.c;bool sign=BN_is_negative(k.p);if(sign){k=neg(k);c=neg(c);}
 auto kd=digits(k,width),cd=digits(c,2);uint64_t nonzero=0,negative=0;for(int d:kd){nonzero+=d!=0;negative+=d<0;}for(int d:cd){nonzero+=d!=0;negative+=d<0;}
 Counts out=radix==1?Counts{}:direct_count(Big(long(radix)),2);uint64_t table=kd.empty()?0:(1u<<(width-2));
 if(nonzero){out.doubles+=std::max(kd.size(),cd.size())-1+uint64_t(table>1);out.adds+=(table?table-1:0)+nonzero-1;out.negations+=negative+uint64_t(sign);out.table_points+=table+uint64_t(!cd.empty());}
 return out;
}
static void action(Runtime& r,EC_POINT* out,const EC_POINT* P,const Split& s,unsigned radix,unsigned width,Counts& count,bool explicit_maps=false){
 if(EC_POINT_is_at_infinity(r.group,P)){require(EC_POINT_set_to_infinity(r.group,out),"identity action");return;}
 auto Q=point(r.group);if(radix==1)require(EC_POINT_copy(Q.get(),P),"identity prefix");else if(explicit_maps){require(radix==unsigned(r.forward->degree),"map radix");Q=r.transported(P);}else public_mul(r,Q.get(),P,Big(long(radix)),2,count,false);
 Big k=s.k,c=s.c;bool sign=BN_is_negative(k.p);if(sign){k=neg(k);c=neg(c);}
 auto kd=digits(k,width),cd=digits(c,2);std::vector<Point> table;
 if(!kd.empty()){
  size_t size=1u<<(width-2);auto two=point(r.group);if(size>1){require(EC_POINT_dbl(r.group,two.get(),Q.get(),r.field.ctx),"joint table double");++count.doubles;}
  table.push_back(point(r.group));require(EC_POINT_copy(table.back().get(),Q.get()),"joint table initial");
  for(size_t i=1;i<size;++i){table.push_back(point(r.group));require(EC_POINT_add(r.group,table.back().get(),table[i-1].get(),two.get(),r.field.ctx),"joint table add");++count.adds;}
  count.table_points+=size;if(size>1){std::vector<EC_POINT*> raw;for(auto& p:table)raw.push_back(p.get());require(EC_POINTs_make_affine(r.group,raw.size(),raw.data(),r.field.ctx),"joint affine table");}
 }
 auto source=point(r.group);if(!cd.empty()){require(EC_POINT_copy(source.get(),P),"remainder table");++count.table_points;}
 auto selected=point(r.group);bool started=false;require(EC_POINT_set_to_infinity(r.group,out),"joint zero");
 for(size_t i=std::max(kd.size(),cd.size());i>0;--i){
  if(started){require(EC_POINT_dbl(r.group,out,out,r.field.ctx),"joint double");++count.doubles;}
  for(unsigned term=0;term<2;++term){int d=term?(i<=cd.size()?cd[i-1]:0):(i<=kd.size()?kd[i-1]:0);if(!d)continue;
   require(EC_POINT_copy(selected.get(),term?source.get():table.at((std::abs(d)-1)/2).get()),"joint select");if(d<0){require(EC_POINT_invert(r.group,selected.get(),r.field.ctx),"joint sign");++count.negations;}
   if(started){require(EC_POINT_add(r.group,out,out,selected.get(),r.field.ctx),"joint add");++count.adds;}else{require(EC_POINT_copy(out,selected.get()),"joint first");started=true;}
  }
 }
 if(sign&&started){require(EC_POINT_invert(r.group,out,r.field.ctx),"joint outer sign");++count.negations;}
}
static bool same(const Counts&a,const Counts&b){return a.doubles==b.doubles&&a.adds==b.adds&&a.negations==b.negations&&a.table_points==b.table_points;}
static std::vector<unsigned> radices(const Fixture& f){return {1,2,4,8,16,32,unsigned(f.marked.at("degree").integer())};}
static void qualify(const std::string& directory){unsigned checks=0,map_checks=0,bad_remainders=0;
 for(auto& f:fixtures(directory)){Runtime r(f,true);verify_relation(r);
  for(long input:{0L,1L,2L,17L,-1L}){auto P=point(r.group),expected=point(r.group);Big scalar(input),residue;require(BN_nnmod(residue.p,scalar.p,r.n.p,r.field.ctx)&&EC_POINT_mul(r.group,P.get(),residue.p,nullptr,nullptr,r.field.ctx)&&EC_POINT_mul(r.group,expected.get(),nullptr,P.get(),r.mu.p,r.field.ctx),"qualification inputs");
   for(unsigned radix:radices(f))for(int offset:{-64,0,64})for(unsigned width=2;width<=7;++width){auto s=split(r,radix,offset);Counts actual;auto out=point(r.group);action(r,out.get(),P.get(),s,radix,width,actual);require(EC_POINT_cmp(r.group,out.get(),expected.get(),r.field.ctx)==0,"residual point action");require(same(actual,input?counter(s,radix,width):Counts{}),"residual actual counters");++checks;
    if(radix==unsigned(r.forward->degree)){Counts map;action(r,out.get(),P.get(),s,radix,width,map,true);require(EC_POINT_cmp(r.group,out.get(),expected.get(),r.field.ctx)==0,"residual explicit map action");++map_checks;}
   }
   if(input==1){auto s=split(r,unsigned(r.forward->degree),0);require(BN_add_word(s.c.p,1),"corrupt remainder");Counts count;auto bad=point(r.group);action(r,bad.get(),P.get(),s,unsigned(r.forward->degree),5,count);require(EC_POINT_cmp(r.group,bad.get(),expected.get(),r.field.ctx)!=0,"wrong remainder accepted");++bad_remainders;}
  }
 }
 std::cout<<"{\"point_and_count_checks\":"<<checks<<",\"explicit_map_checks\":"<<map_checks<<",\"wrong_remainders_rejected\":"<<bad_remainders<<",\"timing_performed\":false}\n";
}
struct Plan{unsigned radix=0,width=0;int offset=0;Counts count;};
static auto key(const Plan& p,unsigned weight){return std::make_tuple(p.count.doubles+weight*p.count.adds,p.count.negations,p.count.doubles,p.count.adds,p.count.table_points,p.radix,std::abs(p.offset),p.offset,p.width);}
static void improve(Plan& best,const Plan& p,unsigned w){if(!best.width||key(p,w)<key(best,w))best=p;}
static Json json(const Plan&p){return object({{"radix",number(p.radix)},{"window",number(p.width)},{"offset",number(p.offset)},{"doubles",number(p.count.doubles)},{"adds",number(p.count.adds)},{"negations",number(p.count.negations)},{"table_points",number(p.count.table_points)}});}
static std::string protocol(const std::string& path,const std::string& directory){auto p=read_json(path);require(p.at("freeze").at("status").str()=="frozen","residual protocol not frozen");auto body=p;body.object.erase("freeze");auto digest=p.at("freeze").at("protocol_sha256").str();require(hash(canonical_json(body))==digest,"residual protocol digest");
 require(p.at("protocol_id").str()=="CM-RESIDUAL-SPLIT-V1","residual protocol identity");auto place=p.at("execution").at("placement");require(place.at("offset_min").integer()==-64&&place.at("offset_max").integer()==64&&place.at("direct_shift_min").integer()==-256&&place.at("direct_shift_max").integer()==256&&canonical_json(place.at("widths"))=="[2,3,4,5,6,7]"&&canonical_json(place.at("weights"))=="[1,2,3,4]","residual finite design");
 require(hash(read("/proc/self/exe"))==p.at("arms").at(size_t(0)).at("instrument").at("binary_sha256").str(),"residual binary binding");require(hash(read("research/cm-marked-scalar/residual.cpp"))==p.at("analysis_plan").at("script").at("sha256").str(),"residual source binding");require(place.at("input_files").array.size()==7,"seven residual inputs");for(const auto& f:place.at("input_files").array)require(hash(read(directory+"/"+f.at("path").str()))==f.at("sha256").str(),"residual fixture hash");return digest;
}
static void scan(const std::string& directory,const std::string& protocol_path,const std::string& output){auto digest=protocol(protocol_path,directory);require(!std::filesystem::exists(output),"write-once residual output");std::filesystem::create_directories(output);std::ofstream csv(output+"/scan.csv");require(bool(csv),"residual CSV");csv<<"curve,arm,radix,offset,window,doubles,adds,negations,table_points\n";Json plans;plans.kind=Json::Array;unsigned rows=0;
 for(auto& f:fixtures(directory)){Runtime r(f,true);Plan best[3][4];
  auto record=[&](int arm,Plan p){csv<<f.name<<','<<arm<<','<<p.radix<<','<<p.offset<<','<<p.width<<','<<p.count.doubles<<','<<p.count.adds<<','<<p.count.negations<<','<<p.count.table_points<<'\n';++rows;for(unsigned w=1;w<=4;++w)improve(best[arm][w-1],p,w);};
  for(unsigned radix:radices(f))for(int offset=-64;offset<=64;++offset){auto s=split(r,radix,offset);for(unsigned width=2;width<=7;++width)record(radix==unsigned(r.forward->degree)?2:1,{radix,width,offset,counter(s,radix,width)});}
  Big base=centered(r.mu,r.n);for(int j=-256;j<=256;++j){Big scalar(j);require(BN_mul(scalar.p,scalar.p,r.n.p,r.field.ctx)&&BN_add(scalar.p,scalar.p,base.p),"direct shift");for(unsigned width=2;width<=7;++width)record(0,{0,width,j,direct_count(scalar,width)});}
  for(unsigned w=1;w<=4;++w)plans.array.push_back(object({{"curve",text(f.name)},{"addition_weight",number(w)},{"direct",json(best[0][w-1])},{"binary_control",json(best[1][w-1])},{"relation",json(best[2][w-1])}}));
 }
 csv.close();require(rows==42480,"complete residual census");Json out=object({{"schema",text("cm-residual-plans/v1")},{"rows",number(rows)},{"protocol_sha256",text(digest)},{"scan_sha256",text(hash(read(output+"/scan.csv")))},{"plans",plans}});std::ofstream stream(output+"/plans.json");stream<<canonical_json(out)<<'\n';require(bool(stream),"residual plans");std::cout<<"{\"complete_scan_rows\":"<<rows<<",\"timing_performed\":false}\n";
}
static void qualify_plans(const std::string& directory,const std::string& path){const auto plans=read_json(path);unsigned count=0;for(auto& f:fixtures(directory)){Runtime r(f,true);for(const auto& entry:plans.at("plans").array){if(entry.at("curve").str()!=f.name)continue;for(long input:{0L,1L,2L,17L,-1L}){auto P=point(r.group),expected=point(r.group),out=point(r.group);Big scalar(input),residue;require(BN_nnmod(residue.p,scalar.p,r.n.p,r.field.ctx)&&EC_POINT_mul(r.group,P.get(),residue.p,nullptr,nullptr,r.field.ctx)&&EC_POINT_mul(r.group,expected.get(),nullptr,P.get(),r.mu.p,r.field.ctx),"selected inputs");for(const char* method:{"direct","binary_control","relation"}){auto plan=entry.at(method);unsigned radix=unsigned(plan.at("radix").integer()),width=unsigned(plan.at("window").integer());int offset=plan.at("offset").integer();Counts actual;
  if(radix){action(r,out.get(),P.get(),split(r,radix,offset),radix,width,actual);}else{Big s(offset);require(BN_mul(s.p,s.p,r.n.p,r.field.ctx)&&BN_add(s.p,s.p,centered(r.mu,r.n).p),"selected direct scalar");public_mul(r,out.get(),P.get(),s,width,actual,false);}
  require(EC_POINT_cmp(r.group,out.get(),expected.get(),r.field.ctx)==0,"selected point action");Counts saved{plan.at("doubles").unsigned_integer(),plan.at("adds").unsigned_integer(),plan.at("negations").unsigned_integer(),plan.at("table_points").unsigned_integer()};require(same(actual,input?saved:Counts{}),"selected actual counters");++count;
 }}}}require(count==300,"complete selected qualification");std::cout<<"{\"selected_point_and_count_checks\":"<<count<<",\"timing_performed\":false}\n";}
}
int main(int argc,char**argv){try{if(argc==3&&std::string(argv[1])=="--qualify"){residual::qualify(argv[2]);return 0;}if(argc==4&&std::string(argv[1])=="--qualify-plans"){residual::qualify_plans(argv[2],argv[3]);return 0;}require(argc==5&&std::string(argv[1])=="--scan","residual --qualify FIXTURES | --qualify-plans FIXTURES PLANS | --scan FIXTURES PROTOCOL NEW_OUTPUT");residual::scan(argv[2],argv[3],argv[4]);return 0;}catch(const std::exception&e){std::cerr<<"residual refused/failed: "<<e.what()<<'\n';return 1;}}
