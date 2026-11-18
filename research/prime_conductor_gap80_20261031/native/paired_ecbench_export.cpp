// Native input preparation only. Measurement remains in ecbench.
#define main complete_pipeline_fixture_entry
#include "complete_pipeline.cpp"
#undef main
#include <openssl/sha.h>
#include <filesystem>
#include <fstream>
#include <set>

namespace fs=std::filesystem;
static_assert(sizeof(unsigned long)==8,"the ecbench u64 codec needs a 64-bit host");
static unsigned long word(const char* s){Z v(s);require(v>=0&&mpz_fits_ulong_p(v.get_mpz_t()),"invalid u64 option");return v.get_ui();}
static unsigned long shared_scalar(unsigned long r,unsigned long seed,unsigned long index){
 std::string input="{\"domain\":\"uniform_scalar_sha256_shared_v1\",\"parts\":["+std::to_string(r)+","+std::to_string(seed)+","+std::to_string(index)+"]}";
 unsigned char digest[SHA256_DIGEST_LENGTH];require(SHA256(reinterpret_cast<const unsigned char*>(input.data()),input.size(),digest),"SHA256 failed");
 uint64_t v=0;for(unsigned j=0;j<8;j++)v=(v<<8)|digest[j];return 1+v%(r-1);
}
static void point_json(std::ostream& o,const Point& P){require(!P.inf,"finite prepared point required");o<<"[\""<<P.x<<"\",\""<<P.y<<"\"]";}
static bool same_point(const Point& P,const Point& Q){return P.inf==Q.inf&&(P.inf||(P.x==Q.x&&P.y==Q.y));}
static void curve_spec(std::ostream& o,const Z&A,const Z&B,const Z&N,const Z&r,const Point&G){
 o<<"{\"kind\":\"prime_explicit\",\"p\":"<<p<<",\"a\":"<<A<<",\"b\":"<<B<<",\"group_order\":"<<N<<",\"r\":"<<r<<",\"gx\":"<<G.x<<",\"gy\":"<<G.y<<"}";
}
static void write_new(const fs::path& dir,const char*name,const std::string& data){
 std::ofstream f(dir/name,std::ios::binary);require(bool(f),"output creation failed");f<<data;f.flush();require(bool(f),"output write failed");
}
struct FieldContext {
 fmpz_t prime;
 FieldContext(){fmpz_init(prime);fmpz_set_mpz(prime,p.get_mpz_t());fmpz_mod_ctx_init(ctx,prime);}
 ~FieldContext(){fmpz_mod_ctx_clear(ctx);fmpz_clear(prime);}
};
int main(int argc,char**argv){try{
 require(argc>=5&&std::string(argv[1])=="--export","usage: paired_ecbench_export --export MAX_WINDOW BLOCK NEW_DIRECTORY [--seed N] [--development-targets N] [--baby-window B] [--projective-batch CAP]");
 unsigned long window=bound(argv[2]),block=bound(argv[3]),seed=20261118,count=32,batch=0;Z baby=0;
 bool development=false,seed_set=false;
 for(int i=5;i<argc;i++){
   std::string option=argv[i];require(i+1<argc,"missing option value");
   if(option=="--seed"&&!seed_set){seed=word(argv[++i]);seed_set=true;}
   else if(option=="--development-targets"&&!development){count=bound(argv[++i]);require(count<=32,"at most 32 targets");development=true;}
   else if(option=="--baby-window"&&baby==0){baby=Z(argv[++i]);require(baby>0,"positive baby window required");}
   else if(option=="--projective-batch"&&batch==0){batch=bound(argv[++i]);require(batch<=65536,"projective batch cap exceeds 65536");}
   else throw std::runtime_error("unknown/duplicate option");
 }
 fs::path directory(argv[4]);require(!directory.empty()&&!fs::exists(directory),"output path already exists or is empty");
 Z ell,r;Point K,G;K.inf=false;G.inf=false;
 require(bool(std::cin>>p>>a>>b>>ell>>K.x>>K.y>>r>>G.x>>G.y),"input: p a b ell Kx Ky r Gx Gy");
 std::string extra;require(!(std::cin>>extra),"trailing case input");
 // This is a consumer compatibility gate, not a bound on all isogeny methods.
 require(p>=5&&p<(Z(1)<<62),"PrimeExplicit field unsupported: requires 5 <= p < 2^62; no large construction attempted");
 require(r>=5&&mpz_fits_ulong_p(r.get_mpz_t()),"PrimeExplicit subgroup order must fit u64 and be at least 5");
 a=mod(a);b=mod(b);Z A=a,B=b;
 require(mod(4*a*a*a+27*b*b)!=0,"singular source");
 require(K.x>=0&&K.x<p&&K.y>=0&&K.y<p&&G.x>=0&&G.x<p&&G.y>=0&&G.y<p,"noncanonical input coordinates");
 pari_init(256000000,2000000);
 require(ell>=5&&ell!=p&&isprime(gen(p))&&isprime(gen(ell))&&isprime(gen(r)),"field, degree and subgroup must be eligible primes");
 require(on(K,a,b)&&on(G,a,b)&&mul(K,ell).inf&&mul(G,r).inf,"source/kernel/subgroup witness failed");
 GEN E=ellinit(mkvec5(gen_0,gen_0,gen_0,gen(A),gen(B)),gen(p),DEFAULTPREC);Z N=integer(ellcard(E,nullptr));
 require(N%r==0&&N%ell==0,"source order divisibility");require(mod(p+1-N)!=0,"ordinary source required");
 std::vector<unsigned long> scalars;std::vector<Point> targets;std::set<unsigned long> unique;
 for(unsigned long i=0;i<count;i++){auto s=shared_scalar(r.get_ui(),seed,i);scalars.push_back(s);unique.insert(s);targets.push_back(mul(G,Z(s)));}
 require(development||unique.size()==count,"fixed shared law has repeated scalars: freeze another seed before a full measurement");
 std::vector<Point> inputs{G};inputs.insert(inputs.end(),targets.begin(),targets.end());inputs.push_back(K);inputs.push_back({});
 Result result;
 {FieldContext context;result=construct(ell,K,inputs,block,window,baby,batch);}
 require(result.covered==(ell-1)/2&&result.images[count+1].inf&&result.images[count+2].inf,"complete map/exceptional-point gate failed");
 Point H=result.images[0];a=result.ca;b=result.cb;
 require(!H.inf&&on(H,a,b)&&mul(H,r).inf,"transported generator lost subgroup injectivity");
 for(unsigned long i=0;i<count;i++)require(same_point(result.images[i+1],mul(H,Z(scalars[i]))),"transported target does not equal shared scalar times image generator");
 GEN F=ellinit(mkvec5(gen_0,gen_0,gen_0,gen(a),gen(b)),gen(p),DEFAULTPREC);require(integer(ellcard(F,nullptr))==N,"endpoint orders differ");
 Z iso_a=mod(16*A),iso_b=mod(64*B);Point iso_g{mod(4*G.x),mod(8*G.y),false};
 a=iso_a;b=iso_b;require(on(iso_g,a,b)&&mul(iso_g,r).inf,"coordinate-isomorphism generator check");
 std::vector<Point> iso_targets;
 for(unsigned long i=0;i<count;i++){
   Point Q{mod(4*targets[i].x),mod(8*targets[i].y),false};
   require(on(Q,a,b)&&same_point(Q,mul(iso_g,Z(scalars[i]))),"coordinate-isomorphism target check");iso_targets.push_back(Q);
 }
 std::ostringstream packet;
 packet<<"{\"schema\":\"conductor.transported-inputs/v1\",\"p\":\""<<p<<"\",\"ell\":\""<<ell<<"\",\"r\":\""<<r<<"\",\"group_order\":\""<<N<<"\",\"source\":[\""<<A<<"\",\""<<B<<"\"],\"destination\":[\""<<result.ca<<"\",\""<<result.cb<<"\"],\"coordinate_control\":{\"u\":\"2\",\"a\":\""<<iso_a<<"\",\"b\":\""<<iso_b<<"\"},\"kernel\":";point_json(packet,K);
 packet<<",\"generators\":{\"source\":";point_json(packet,G);packet<<",\"destination\":";point_json(packet,H);packet<<",\"coordinate_control\":";point_json(packet,iso_g);
 packet<<"},\"scalar_law\":\"uniform_scalar_sha256_shared_v1\",\"target_seed\":\""<<seed<<"\",\"requested_target_count\":32,\"actual_target_count\":"<<count<<",\"distinct_scalars\":"<<unique.size()<<",\"development_fixture\":"<<(development?"true":"false")<<",\"kernel_pairs_covered\":\""<<result.covered<<"\",\"targets\":[";
 for(unsigned long i=0;i<count;i++){
   if(i)packet<<",";packet<<"{\"index\":"<<i<<",\"scalar\":\""<<scalars[i]<<"\",\"source\":";point_json(packet,targets[i]);packet<<",\"destination\":";point_json(packet,result.images[i+1]);packet<<",\"coordinate_control\":";point_json(packet,iso_targets[i]);packet<<"}";
 }
 packet<<"],\"conductor_gap\":null,\"runtime_measurements\":null,\"registry_curve_ids\":null,\"high_fidelity_admitted\":false,\"remaining_obligations\":[\"paired endpoint scheduling and algorithm seeds in the canonical harness\",\"scalar measurement integration\",\"fastest prime wall reference integration\",\"registry and independent map/endpoint evidence\",\"source/image binding and cloud admission\"],\"validation_scope\":\"producer complete coverage, point membership, subgroup and homomorphism controls; no independent execution verdict\"}\n";
 std::ostringstream spec;
 spec<<"{\"schema\":\"ecbench.spec/v1\",\"label\":\"transported endpoint input planning\",\"description\":\"Planning fixture only: shared targets, prospective ECDLP arms and within-curve A/A; paired endpoint admission and external map costs remain open.\",\"workloads\":{\"curves\":[";
 curve_spec(spec,A,B,N,r,G);spec<<",";curve_spec(spec,result.ca,result.cb,N,r,H);spec<<",";curve_spec(spec,iso_a,iso_b,N,r,iso_g);
 spec<<"],\"targets_per_curve\":"<<count<<",\"target_seed\":"<<seed<<",\"target_kind\":\"planted_shared\"},\"arms\":["
 <<"{\"name\":\"rho-counted\",\"role\":\"reference\",\"method\":{\"id\":\"rho.negation\",\"params\":{\"cap_multiple\":\"64\"}}},"
 <<"{\"name\":\"bsgs\",\"role\":\"baseline\",\"method\":{\"id\":\"bsgs.textbook\"}},"
 <<"{\"name\":\"bsgs-neg\",\"role\":\"candidate\",\"method\":{\"id\":\"bsgs.negation\"}},"
 <<"{\"name\":\"ic-input-control\",\"role\":\"candidate\",\"method\":{\"id\":\"ic.pipeline\",\"params\":{\"factor_base\":\"prime-abscissa:size=8\",\"oracle\":\"rootfind-subtract:m=2\",\"solver\":\"\",\"linalg\":\"incremental-gauss\",\"targets\":\"walk\",\"max_trials\":\"1000000\",\"solver_budget_seconds\":\"0\"}}},"
 <<"{\"name\":\"rho-aa\",\"role\":\"control\",\"method\":{\"id\":\"rho.negation\",\"params\":{\"cap_multiple\":\"64\"}}}],"
 <<"\"measurement\":{\"rounds\":5,\"warmup\":1,\"order\":\"alternate\",\"seed\":20261118,\"isolation_required\":\"L2\",\"timeout_seconds\":600}}\n";
 std::ostringstream scalar;
 scalar<<"{\"schema\":\"conductor.scalar-input-matrix/v1\",\"prepared_input\":\"prepared.json\",\"base_points\":\"targets[*].<endpoint>\",\"scalars\":\"targets[*].scalar\",\"endpoints\":[\"source\",\"destination\",\"coordinate_control\"],\"combinations\":\"cartesian product of base-point and scalar indices\",\"planned_calls_per_round_per_endpoint\":"<<count*count<<",\"rounds\":5,\"warmup\":1,\"driver_status\":\"canonical scalar measurement integration pending\",\"runtime_measurements\":null}\n";
 require(fs::create_directory(directory),"output directory reservation failed");
 write_new(directory,"prepared.json",packet.str());write_new(directory,"spec.json",spec.str());write_new(directory,"scalar-workload.json",scalar.str());
 std::cout<<"{\"status\":\"inputs_prepared\",\"target_count\":"<<count<<",\"distinct_scalars\":"<<unique.size()<<",\"runtime_measurements\":null,\"high_fidelity_admitted\":false}\n";
 pari_close();flint_cleanup();return 0;
}catch(const std::exception&e){std::cerr<<"paired-ecbench-export: "<<e.what()<<'\n';return 2;}}
