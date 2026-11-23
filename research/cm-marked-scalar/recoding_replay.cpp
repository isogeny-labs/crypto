// Independent census and minimum replay. Reuses exact JSON/BIGNUM/fixture
// primitives, but neither wnaf(), analytic_count() nor the scanner's selection.
#define main cm_native_main
#include "native.cpp"
#undef main
#include <charconv>
#include <array>
#include <iomanip>
#include <tuple>
#include <openssl/sha.h>
static std::string hash_text(const std::string& text){unsigned char h[SHA256_DIGEST_LENGTH];SHA256(reinterpret_cast<const unsigned char*>(text.data()),text.size(),h);std::ostringstream s;for(auto c:h)s<<std::hex<<std::setw(2)<<std::setfill('0')<<unsigned(c);return s.str();}
static std::string contents(const std::string& path){std::ifstream f(path,std::ios::binary);require(bool(f),"replay file missing");std::ostringstream s;s<<f.rdbuf();return s.str();}
struct Counter{uint64_t d=0,a=0,neg=0,t=0;};
static Counter independent_count(const Big& signed_scalar,unsigned width){
 Big remaining=signed_scalar;BN_set_negative(remaining.p,0);require(!remaining.zero(),"zero scalar");unsigned bits=0,nonzero=0,negative=0;
 const unsigned modulus=1u<<width;
 while(!remaining.zero()){
  if(BN_is_odd(remaining.p)){
   int digit=int(BN_mod_word(remaining.p,modulus));if(unsigned(digit)>=modulus/2)digit-=int(modulus);
   if(digit<0){require(BN_add_word(remaining.p,unsigned(-digit)),"independent recode addition");++negative;}
   else require(BN_sub_word(remaining.p,unsigned(digit)),"independent recode subtraction");
   ++nonzero;
  }
  require(BN_rshift1(remaining.p,remaining.p),"independent recode shift");++bits;
 }
 uint64_t table=1u<<(width-2);return {bits-1+uint64_t(table>1),table-1+nonzero-1,negative+uint64_t(BN_is_negative(signed_scalar.p)),table};
}
static int integer(const std::string& text){int result=0;auto parsed=std::from_chars(text.data(),text.data()+text.size(),result);require(parsed.ec==std::errc()&&parsed.ptr==text.data()+text.size(),"CSV integer");return result;}
struct Plan{int j=0;unsigned width=0;Counter c;};
using PlanKey=std::tuple<uint64_t,uint64_t,uint64_t,uint64_t,int,int,unsigned>;
static PlanKey key(const Plan& p,unsigned weight){return {p.c.d+weight*p.c.a,p.c.neg,p.c.d,p.c.a,std::abs(p.j),p.j,p.width};}
static void minimize(Plan& best,const Plan& candidate,unsigned weight){if(best.width==0||key(candidate,weight)<key(best,weight))best=candidate;}
struct ScalarInfo{Big n,mu,correction;unsigned ell;};
static void check_plan(const Json& json,const Plan& p){require(json.at("shift").integer()==p.j&&json.at("window").unsigned_integer()==p.width&&json.at("doubles").unsigned_integer()==p.c.d&&json.at("adds").unsigned_integer()==p.c.a&&json.at("negations").unsigned_integer()==p.c.neg&&json.at("table_points").unsigned_integer()==p.c.t,"selected minimum mismatch");}
int main(int argc,char** argv){try{
 require(argc==5,"recoding-replay FIXTURES FROZEN_PROTOCOL RAW_CSV PLANS_JSON");
 auto protocol=read_json(argv[2]),plans=read_json(argv[4]);Json body=protocol;body.object.erase("freeze");auto protocol_hash=protocol.at("freeze").at("protocol_sha256").str();require(protocol.at("freeze").at("status").str()=="frozen"&&hash_text(canonical_json(body))==protocol_hash&&plans.at("protocol_sha256").str()==protocol_hash,"independent protocol binding");
 std::string raw=contents(argv[3]);require(hash_text(raw)==plans.at("scan_sha256").str(),"independent raw-data binding");
 std::map<std::string,ScalarInfo> scalars;BN_CTX* ctx=BN_CTX_new();require(ctx,"replay context");std::unique_ptr<BN_CTX,decltype(&BN_CTX_free)> context(ctx,BN_CTX_free);
 for(auto& fixture:fixtures(argv[1])){
  ScalarInfo s{Big(fixture.curve.at("n").str()),Big(fixture.marked.at("mu").str()),Big(),unsigned(fixture.marked.at("degree").integer())};Big inverse;
  require(BN_mod_inverse(inverse.p,Big(long(s.ell)).p,s.n.p,ctx)&&BN_mod_mul(s.correction.p,s.mu.p,inverse.p,s.n.p,ctx),"independent correction");scalars.emplace(fixture.name,s);
 }
 require(scalars.size()==5,"replay curve panel");
 std::map<std::string,std::array<std::array<Plan,4>,2>> best,legacy;std::set<std::tuple<std::string,int,int,int>> seen;
 std::istringstream in(raw);std::string line;require(bool(std::getline(in,line))&&line=="curve,family,window,shift,doubles,adds,negations,table_points","CSV header");
 while(std::getline(in,line)){
  std::vector<std::string> cells;std::istringstream row(line);std::string cell;while(std::getline(row,cell,','))cells.push_back(cell);require(cells.size()==8,"CSV shape");std::string curve=cells[0];require(scalars.count(curve),"CSV curve");int family=integer(cells[1]),width=integer(cells[2]),j=integer(cells[3]);require(family>=0&&family<=1&&width>=2&&width<=7&&j>=-256&&j<=256,"CSV finite design");require(seen.emplace(curve,family,width,j).second,"duplicate CSV row");
  auto& s=scalars.at(curve);Big scalar=family?s.correction:s.mu,twice;require(BN_lshift1(twice.p,scalar.p),"independent center");if(BN_cmp(twice.p,s.n.p)>0)require(BN_sub(scalar.p,scalar.p,s.n.p),"independent center subtraction");Big offset(j);require(BN_mul(offset.p,offset.p,s.n.p,ctx)&&BN_add(scalar.p,scalar.p,offset.p),"independent shift");
  Counter count=independent_count(scalar,unsigned(width));require(integer(cells[4])==int(count.d)&&integer(cells[5])==int(count.a)&&integer(cells[6])==int(count.neg)&&integer(cells[7])==int(count.t),"independent schedule mismatch");
  if(family){auto q=independent_count(Big(long(s.ell)),2);count.d+=q.d;count.a+=q.a;count.neg+=q.neg;count.t+=q.t;}
  Plan p{j,unsigned(width),count};for(unsigned w=1;w<=4;++w){minimize(best[curve][family][w-1],p,w);if(!j)minimize(legacy[curve][family][w-1],p,w);}
 }
 require(seen.size()==30780&&plans.at("rows").unsigned_integer()==30780,"complete independent census");std::set<std::pair<std::string,unsigned>> plan_keys;
 for(const auto& json:plans.at("plans").array){std::string curve=json.at("curve").str();unsigned w=unsigned(json.at("addition_weight").unsigned_integer());require(scalars.count(curve)&&w>=1&&w<=4&&plan_keys.emplace(curve,w).second,"plan identity");check_plan(json.at("direct"),best.at(curve)[0][w-1]);check_plan(json.at("fused"),best.at(curve)[1][w-1]);check_plan(json.at("legacy_direct"),legacy.at(curve)[0][w-1]);check_plan(json.at("legacy_fused"),legacy.at(curve)[1][w-1]);}
 require(plan_keys.size()==20,"complete selected-plan design");std::cout<<"{\"independent_census_replay_pass\":true,\"rows\":"<<seen.size()<<",\"selected_minimum_sets\":"<<plan_keys.size()<<",\"raw_sha256\":"<<json_quote(hash_text(raw))<<",\"protocol_sha256\":"<<json_quote(protocol_hash)<<",\"timing_performed\":false}\n";return 0;
}catch(const std::exception& e){std::cerr<<"independent recoding replay refused: "<<e.what()<<'\n';return 1;}}
