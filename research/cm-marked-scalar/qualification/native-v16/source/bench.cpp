#define main cm_qualification_entrypoint
#include "native.cpp"
#undef main
#include <chrono>
#include <iomanip>
#include <openssl/sha.h>
#include <cstdlib>
#include <random>
#include <sys/stat.h>
#include <sys/file.h>
#include <fcntl.h>
#include <sched.h>
#include <unistd.h>
using Clock=std::chrono::steady_clock;
static uint64_t ns(Clock::time_point t){return std::chrono::duration_cast<std::chrono::nanoseconds>(Clock::now()-t).count();}
static Big correction(Runtime&r,int ell){Big k,inv;require(BN_mod_inverse(inv.p,Big(ell).p,r.n.p,r.field.ctx),"degree inverse");require(BN_mod_mul(k.p,r.mu.p,inv.p,r.n.p,r.field.ctx),"correcting scalar");return k;}
static std::string encoded(Runtime&r,const EC_POINT*P){size_t size=EC_POINT_point2oct(r.group,P,POINT_CONVERSION_COMPRESSED,nullptr,0,r.field.ctx);require(size,"point encoding size");std::vector<unsigned char>b(size);require(EC_POINT_point2oct(r.group,P,POINT_CONVERSION_COMPRESSED,b.data(),b.size(),r.field.ctx)==size,"point encoding");std::ostringstream s;for(auto x:b)s<<std::hex<<std::setw(2)<<std::setfill('0')<<int(x);return s.str();}
struct Sample {Affine input;std::string expected;};
static Sample sample(const Fixture&f,unsigned index,const std::string&panel="calibration"){Runtime r(f,false);std::string seed="cm-marked-scalar-v1:"+panel+":"+f.name+":"+std::to_string(index);unsigned char digest[SHA256_DIGEST_LENGTH];SHA256(reinterpret_cast<const unsigned char*>(seed.data()),seed.size(),digest);Big k;require(BN_bin2bn(digest,sizeof(digest),k.p),"input scalar");require(BN_nnmod(k.p,k.p,r.n.p,r.field.ctx),"input reduction");if(k.zero())require(BN_one(k.p),"nonzero input");auto P=point(r.group),Y=point(r.group);require(EC_POINT_mul(r.group,P.get(),k.p,nullptr,nullptr,r.field.ctx),"public input generation");require(EC_POINT_mul(r.group,Y.get(),nullptr,P.get(),r.mu.p,r.field.ctx),"independent expected output");return {r.affine(P.get()),encoded(r,Y.get())};}
struct Result {uint64_t setup,maps,group,verify,total;Counts schedule;};
static Result compute(const Fixture&f,const Sample&s,const std::string&arm,unsigned window,bool clocks=true){
 auto now=[&](){return clocks?Clock::now():Clock::time_point{};}; auto elapsed=[&](Clock::time_point t){return clocks?ns(t):uint64_t(0);}; auto start=now();Result result{};{bool explicit_maps=arm=="maps";Runtime r(f,explicit_maps);int ell=f.closure.at("first").at("edge").at("degree").integer();Big k=(arm=="fused")?correction(r,ell):r.correction;auto P=point(r.group),Y=point(r.group);r.set(P.get(),s.input);require(EC_POINT_is_on_curve(r.group,P.get(),r.field.ctx)==1,"input on curve");uint64_t setup=elapsed(start);auto phase=now();Counts count;Point Q(nullptr,EC_POINT_free);const EC_POINT* source=P.get();
 if(arm=="maps"){Q=r.transported(P.get());source=Q.get();}else if(arm=="fused"){Q=point(r.group);public_mul(r,Q.get(),P.get(),Big(ell),2,count);source=Q.get();}uint64_t maps=elapsed(phase);phase=now();
 if(arm=="maps")public_mul(r,Y.get(),source,r.correction,window,count);
 else if(arm=="fused")public_mul(r,Y.get(),source,k,window,count);
 else if(arm=="window"||arm=="window-aa")public_mul(r,Y.get(),source,r.mu,window,count);
 else if(arm=="openssl-public"||arm=="openssl-public-aa")external_public(r,Y.get(),source,r.mu);
 else if(arm=="openssl-single")require(EC_POINT_mul(r.group,Y.get(),nullptr,source,r.mu.p,r.field.ctx),"OpenSSL scalar reference");
 else throw std::runtime_error("unknown arm");
 uint64_t group=elapsed(phase);phase=now();require(EC_POINT_is_on_curve(r.group,Y.get(),r.field.ctx)==1,"output curve");require(encoded(r,Y.get())==s.expected,"identical verified output");uint64_t verify=elapsed(phase);result={setup,maps,group,verify,0,count};}result.total=elapsed(start);return result;
}
static void structural(const std::vector<Fixture>&fs){for(auto&f:fs){auto s=sample(f,0);for(unsigned w=2;w<=7;++w)for(auto arm:{"window","maps","fused"}){auto result=compute(f,s,arm,w,false);std::cout<<"{\"curve\":\""<<f.name<<"\",\"arm\":\""<<arm<<"\",\"window\":"<<w<<",\"doubles\":"<<result.schedule.doubles<<",\"adds\":"<<result.schedule.adds<<",\"negations\":"<<result.schedule.negations<<",\"timing_reported\":false,\"correct\":true}"<<std::endl;}}}
int main(int argc,char**argv){try{require(argc>=3,"usage: bench --structural|--bench FIXTURE_DIR [POINTS ROUNDS INNER]");auto fs=fixtures(argv[2]);if(std::string(argv[1])=="--structural"){structural(fs);return 0;}
 require(std::string(argv[1])=="--bench"&&argc==7,"usage: bench --bench FIXTURE_DIR PANEL POINTS ROUNDS INNER");require(std::getenv("CM_NATIVE_ISOLATION_FD"),"native isolation admission missing");int isolation_fd=std::stoi(std::getenv("CM_NATIVE_ISOLATION_FD"));struct stat actual{},expected{};require(fstat(isolation_fd,&actual)==0&&stat("/tmp/crypto-bench.lock",&expected)==0&&actual.st_ino==expected.st_ino&&actual.st_dev==expected.st_dev,"isolation lock descriptor");require(flock(isolation_fd,LOCK_EX|LOCK_NB)==0,"isolation lock ownership");cpu_set_t cpumask;require(sched_getaffinity(0,sizeof(cpumask),&cpumask)==0&&CPU_COUNT(&cpumask)==1,"single reserved CPU");const char* frozen=std::getenv("CM_PROTOCOL_SHA");require(frozen&&std::string(frozen).size()==64&&std::string(frozen).find_first_not_of("0123456789abcdef")==std::string::npos,"frozen protocol binding missing");std::string panel_name=argv[3];require(panel_name=="calibration"||panel_name=="heldout","panel");unsigned points=std::stoul(argv[4]),rounds=std::stoul(argv[5]),inner=std::stoul(argv[6]);require(points&&points<=32&&rounds>=5&&rounds<=30&&inner&&inner<=32,"finite sample bounds");
 // All windows remain in raw rows. Reference selection is performed only by frozen analysis.
 for(auto&f:fs){std::vector<Sample> panel;for(unsigned i=0;i<points;++i)panel.push_back(sample(f,i,panel_name));for(unsigned w=2;w<=7;++w){std::vector<std::string>arms={"window","window-aa","maps","fused","openssl-public","openssl-public-aa","openssl-single"};for(unsigned round=0;round<=rounds;++round){std::mt19937_64 order(0x434d383838ULL+round+7919*w);std::shuffle(arms.begin(),arms.end(),order);for(unsigned i=0;i<points;++i)for(auto&arm:arms){uint64_t total=0,setup=0,maps=0,group=0,verify=0;for(unsigned j=0;j<inner;++j){auto q=compute(f,panel[i],arm,w);total+=q.total;setup+=q.setup;maps+=q.maps;group+=q.group;verify+=q.verify;}std::cout<<"{\"curve\":\""<<f.name<<"\",\"point\":"<<i<<",\"round\":"<<round<<",\"warmup\":"<<(round==0?"true":"false")<<",\"arm\":\""<<arm<<"\",\"window\":"<<w<<",\"inner\":"<<inner<<",\"panel\":\""<<panel_name<<"\",\"total_ns\":"<<total<<",\"setup_ns\":"<<setup<<",\"maps_ns\":"<<maps<<",\"group_ns\":"<<group<<",\"verify_ns\":"<<verify<<",\"bookkeeping_ns\":"<<(total-setup-maps-group-verify)<<",\"correct\":true,\"wall_admission_pending\":true,\"output\":\""<<panel[i].expected<<"\"}"<<std::endl;}}}}
 return 0;}catch(const std::exception&e){std::cerr<<"native comparison refused/failed: "<<e.what()<<std::endl;return 1;}}
