// Independent GMP reconstruction, schedule count and exhaustive selection.
// Does not include residual.cpp, native.cpp, wnaf or producer selection code.
#include "json.hpp"
#include <gmpxx.h>
#include <openssl/sha.h>
#include <array>
#include <charconv>
#include <iomanip>
#include <iostream>
#include <set>
#include <tuple>
using Int=mpz_class;
static void check(bool b,const char*s){if(!b)throw std::runtime_error(s);}
static std::string read(const std::string& path){std::ifstream f(path,std::ios::binary);check(bool(f),"replay file missing");std::ostringstream s;s<<f.rdbuf();return s.str();}
static std::string hash(const std::string& bytes){unsigned char h[SHA256_DIGEST_LENGTH];SHA256(reinterpret_cast<const unsigned char*>(bytes.data()),bytes.size(),h);std::ostringstream s;for(auto c:h)s<<std::hex<<std::setw(2)<<std::setfill('0')<<unsigned(c);return s.str();}
static int integer(const std::string&s){int n=0;auto r=std::from_chars(s.data(),s.data()+s.size(),n);check(r.ec==std::errc()&&r.ptr==s.data()+s.size(),"replay CSV integer");return n;}
static Int center(Int v,const Int&n){if(2*v>n)v-=n;return v;}
struct Digits{uint64_t len=0,nonzero=0,negative=0;};
static Digits recode(Int scalar,unsigned width,bool signed_digits){
 bool sign=scalar<0;Int target=scalar;if(sign)scalar=-scalar;Int reconstruct=0,power=1;Digits d;
 const long modulus=1L<<width;
 while(scalar!=0){long digit=0;if(mpz_odd_p(scalar.get_mpz_t())){Int small=scalar%modulus;digit=small.get_si();if(digit>=modulus/2)digit-=modulus;scalar-=digit;}scalar/=2;reconstruct+=digit*power;power*=2;++d.len;if(digit){++d.nonzero;d.negative+=signed_digits&&sign?digit>0:digit<0;}}
 if(sign)reconstruct=-reconstruct;
 check(reconstruct==target,"independent digit reconstruction");return d;
}
struct Count{uint64_t d=0,a=0,n=0,t=0;};
static Count direct(const Int&s,unsigned width){if(s==0)return {};auto digits=recode(s,width,false);uint64_t table=1u<<(width-2);return {digits.len-1+uint64_t(table>1),table-1+digits.nonzero-1,digits.negative+uint64_t(s<0),table};}
static Count joint(Int k,Int c,unsigned radix,unsigned width){bool sign=k<0;if(sign){k=-k;c=-c;}auto kd=recode(k,width,true),cd=recode(c,2,true);Count out=radix==1?Count{}:direct(Int(radix),2);uint64_t nz=kd.nonzero+cd.nonzero,table=kd.len?(1u<<(width-2)):0;
 if(nz){out.d+=std::max(kd.len,cd.len)-1+uint64_t(table>1);out.a+=(table?table-1:0)+nz-1;out.n+=kd.negative+cd.negative+uint64_t(sign);out.t+=table+uint64_t(cd.len!=0);}return out;}
struct Plan{unsigned radix=0,width=0;int offset=0;Count c;};
static auto rank(const Plan&p,unsigned weight){return std::make_tuple(p.c.d+weight*p.c.a,p.c.n,p.c.d,p.c.a,p.c.t,p.radix,std::abs(p.offset),p.offset,p.width);}
static void update(Plan&best,const Plan&p,unsigned w){if(!best.width||rank(p,w)<rank(best,w))best=p;}
static void selected(const Json&j,const Plan&p){check(j.at("radix").integer()==int(p.radix)&&j.at("window").integer()==int(p.width)&&j.at("offset").integer()==p.offset&&j.at("doubles").unsigned_integer()==p.c.d&&j.at("adds").unsigned_integer()==p.c.a&&j.at("negations").unsigned_integer()==p.c.n&&j.at("table_points").unsigned_integer()==p.c.t,"independent minimum mismatch");}
struct Curve{Int n,mu;unsigned ell;};
int main(int argc,char**argv){try{
 check(argc==5,"residual-replay FIXTURES FROZEN_PROTOCOL RAW_CSV PLANS");const std::string directory=argv[1];auto protocol=read_json(argv[2]),plans=read_json(argv[4]);auto body=protocol;body.object.erase("freeze");const auto digest=protocol.at("freeze").at("protocol_sha256").str();check(protocol.at("freeze").at("status").str()=="frozen"&&protocol.at("protocol_id").str()=="CM-RESIDUAL-SPLIT-V1"&&hash(canonical_json(body))==digest&&plans.at("protocol_sha256").str()==digest,"independent protocol binding");
 const auto place=protocol.at("execution").at("placement");check(place.at("offset_min").integer()==-64&&place.at("offset_max").integer()==64&&place.at("direct_shift_min").integer()==-256&&place.at("direct_shift_max").integer()==256&&canonical_json(place.at("widths"))=="[2,3,4,5,6,7]"&&canonical_json(place.at("weights"))=="[1,2,3,4]","independent finite protocol design");
 check(hash(read("/proc/self/exe"))==place.at("independent_replay_binary_sha256").str()&&hash(read("research/cm-marked-scalar/residual_replay.cpp"))==place.at("independent_replay_source_sha256").str(),"independent executable binding");check(place.at("input_files").array.size()==7,"independent seven inputs");
 for(const auto& f:place.at("input_files").array)check(hash(read(directory+"/"+f.at("path").str()))==f.at("sha256").str(),"independent fixture binding");
 const auto curves=read_json(directory+"/published-curves.json"),marks=read_json(directory+"/marked-relations.json");std::map<std::string,Curve> inputs;
 for(const auto& curve:curves.at("rows").array){const auto name=curve.at("name").str();const Json* mark=nullptr;for(const auto&m:marks.at("rows").array)if(m.at("name").str()==name){check(!mark,"duplicate marked curve");mark=&m;}check(mark,"missing marked curve");check(inputs.emplace(name,Curve{Int(curve.at("n").str()),Int(mark->at("mu").str()),unsigned(mark->at("degree").integer())}).second,"duplicate curve");}check(inputs.size()==5,"five replay inputs");
 const auto raw=read(argv[3]);check(hash(raw)==plans.at("scan_sha256").str(),"independent raw binding");std::set<std::tuple<std::string,int,int,int,int>> seen;std::map<std::string,std::array<std::array<Plan,4>,3>> best;std::istringstream stream(raw);std::string line;check(bool(std::getline(stream,line))&&line=="curve,arm,radix,offset,window,doubles,adds,negations,table_points","residual CSV header");
 while(std::getline(stream,line)){std::istringstream row(line);std::vector<std::string> cells;std::string cell;while(std::getline(row,cell,','))cells.push_back(cell);check(cells.size()==9,"residual CSV shape");check(inputs.count(cells[0]),"residual CSV curve");const auto& input=inputs.at(cells[0]);int arm=integer(cells[1]),radix=integer(cells[2]),offset=integer(cells[3]),width=integer(cells[4]);check(arm>=0&&arm<=2&&width>=2&&width<=7,"residual CSV design");check(seen.emplace(cells[0],arm,radix,offset,width).second,"duplicate residual row");Count count;Int base=center(input.mu,input.n);
  if(arm==0){check(radix==0&&offset>=-256&&offset<=256,"direct finite design");count=direct(Int(base+offset*input.n),unsigned(width));}
  else{const std::set<int> binary={1,2,4,8,16,32};check(offset>=-64&&offset<=64&&((arm==1&&binary.count(radix))||(arm==2&&radix==int(input.ell))),"joint finite design");Int remainder=base%radix;if(remainder<0)remainder+=radix;if(2*remainder>radix)remainder-=radix;const Int c=remainder+radix*offset,delta=base-c;check(delta%radix==0,"independent quotient remainder");const Int k=delta/radix;check(k*radix+c==base,"independent fixed scalar");count=joint(k,c,unsigned(radix),unsigned(width));}
  check(integer(cells[5])==int(count.d)&&integer(cells[6])==int(count.a)&&integer(cells[7])==int(count.n)&&integer(cells[8])==int(count.t),"independent counter mismatch");Plan p{unsigned(radix),unsigned(width),offset,count};for(unsigned w=1;w<=4;++w)update(best[cells[0]][arm][w-1],p,w);
 }
 check(seen.size()==42480&&plans.at("rows").unsigned_integer()==42480,"complete residual census");std::set<std::pair<std::string,unsigned>> sets;
 for(const auto& entry:plans.at("plans").array){auto name=entry.at("curve").str();auto w=entry.at("addition_weight").unsigned_integer();check(inputs.count(name)&&w>=1&&w<=4&&sets.emplace(name,unsigned(w)).second,"selected residual identity");selected(entry.at("direct"),best.at(name)[0][w-1]);selected(entry.at("binary_control"),best.at(name)[1][w-1]);selected(entry.at("relation"),best.at(name)[2][w-1]);}
 check(sets.size()==20,"complete residual minimum sets");std::cout<<"{\"independent_rows\":"<<seen.size()<<",\"minimum_sets\":"<<sets.size()<<",\"integer_backend\":\"GMP\",\"protocol_sha256\":"<<json_quote(digest)<<",\"timing_performed\":false}\n";return 0;
}catch(const std::exception&e){std::cerr<<"residual independent replay refused: "<<e.what()<<'\n';return 1;}}
