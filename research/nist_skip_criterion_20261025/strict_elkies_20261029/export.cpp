// Native metadata adapter only; no character or curve computation.
#include <filesystem>
#include <fstream>
#include <iomanip>
#include <iostream>
#include <openssl/evp.h>
#include <regex>
#include <sstream>
#include <stdexcept>
#include <string>
#include <vector>
namespace fs=std::filesystem;
static void need(bool b,const std::string& s){if(!b)throw std::runtime_error(s);}
static std::string read(const fs::path& p){std::ifstream f(p,std::ios::binary);need(bool(f),"missing "+p.string());return {std::istreambuf_iterator<char>(f),{}};}
static std::string q(const std::string& s){std::string r="\"";for(unsigned char c:s){if(c=='"'||c=='\\'){r+='\\';r+=char(c);}else if(c=='\n')r+="\\n";else if(c=='\t')r+="\\t";else{need(c>=32,"control byte");r+=char(c);}}return r+'"';}
static std::string hash(const std::string& s){unsigned char h[32];unsigned int n=0;need(EVP_Digest(s.data(),s.size(),h,&n,EVP_sha256(),nullptr)==1&&n==32,"hash");std::ostringstream out;for(unsigned char c:h)out<<std::hex<<std::setw(2)<<std::setfill('0')<<unsigned(c);return out.str();}
static std::string field(const std::string& s,const std::string& re){std::smatch m;need(std::regex_search(s,m,std::regex(re)),"field absent");return m[1];}
static std::string artifact(const fs::path& b,const std::string& p,const std::string& kind="raw"){const auto s=read(b/p);return "{\"path\":"+q(p)+",\"sha256\":"+q(hash(s))+",\"bytes\":"+std::to_string(s.size())+",\"kind\":"+q(kind)+'}';}
int main(int argc,char** argv){try{
 need(argc==5,"export STUDY BIN_DIR CREATED OUTPUT");fs::path b=argv[1],bin=argv[2],run=b/"runs/attempt-001";
 const auto protocol=read(b/"protocol.json"),summary=read(run/"results/summary.json");need(summary.find("strict-elkies-finite-replay/v1")!=std::string::npos,"wrong result class");need(read(run/"verification.json").find("\"all_six_files_equal\":true")!=std::string::npos,"verification absent");
 std::string commit=read(run/"source-commit.txt");while(!commit.empty()&&commit.back()=='\n')commit.pop_back();
 const std::string psha=field(protocol,"\"protocol_sha256\"\\s*:\\s*\"([0-9a-f]{64})\"");
 const std::vector<std::string> names{"positive","production","verification","negative_character","negative_calibration","negative_summary"};
 std::ofstream out(argv[4]);need(bool(out),"output");
 out<<"{\"schema\":\"experiment.export/v1\",\"export_id\":\"nist-strict-elkies-attempt-001\",\"created\":"<<q(argv[3])<<",\"protocol\":{\"path\":\"protocol.json\",\"sha256\":"<<q(psha)<<",\"protocol_id\":\"nist-strict-elkies-full21-20261029\",\"version\":1},\"producer\":{\"runtime\":\"codex\",\"model\":null,\"session\":\"strict-elkies-replay\",\"role\":\"author\"},\"source\":{\"repo\":\"crypto\",\"commit\":"<<q(commit)<<",\"dirty\":false},\"backend\":\"local\",\"host\":{\"path\":null,\"sha256\":null,\"class_id\":null,\"summary\":{\"cpu_model\":null,\"logical_cpus\":null,\"memory_bytes\":null,\"kernel\":null,\"hypervisor\":null,\"isolation_tier_max\":null}},\"arms\":[";
 for(size_t i=0;i<names.size();++i){if(i)out<<',';out<<"{\"id\":"<<q(names[i])<<",\"role\":"<<q(i==0?"control_positive":i==1?"candidate":i==2?"baseline":"control_negative")<<",\"binary_sha256\":"<<q(hash(read(bin/(i<2?"producer":"verify"))))<<",\"commit\":\"043a568661ebf1c40a4ead333889d814c32d045c\"}";}
 out<<"],\"runs\":[";
 for(size_t i=0;i<names.size();++i){const auto& n=names[i];const auto status=read(run/(n+".status"));const bool ok=status==(i>2?"1\n":"0\n");need(ok,"unexpected native phase status");if(i)out<<',';
  std::string counts=i==0?"{\"control_pass\":1}":i>2?"{\"expected_native_refusal\":1}":"{";
  if(i==1||i==2){bool first=true;for(const auto& key:std::vector<std::string>{"curve_choices","cutoffs","odd_prime_character_cells","label_assignments","strict_search_success_assignments","strict_fixed691_success_assignments"}){if(!first)counts+=',';first=false;counts+=q(key)+':'+field(summary,"\""+key+"\"\\s*:\\s*([0-9]+)");}counts+='}';}
  out<<"{\"run_id\":"<<q("attempt-001-"+n)<<",\"arm\":"<<q(n)<<",\"instance_id\":\"frozen-input-bundle\",\"seed\":null,\"replicate\":0,\"warmup\":false,\"status\":\"succeeded\",\"outcome_class\":\"completed\",\"attempts\":1,\"resumed\":0,\"checkpoint\":null,\"measurements\":{\"counted\":"<<counts<<",\"wall_seconds\":null,\"wall_valid\":false,\"cpu_user_seconds\":null,\"cpu_sys_seconds\":null,\"max_rss_bytes\":null,\"counters\":{}},\"isolation\":{\"tier\":null,\"contended\":null,\"blockers\":[\"Local exact computation; no performance clock\"]},\"verification\":{\"status\":\"verified\",\"verifier\":"<<q(i>2?"Expected native exit1 on fixed corrupted data; all other phases require0":"Separate native character/count/label replay and fixed arithmetic controls")<<"},\"exclusion\":{\"excluded\":false,\"rule\":null},\"artifacts\":["<<artifact(b,"runs/attempt-001/"+n+".stdout")<<','<<artifact(b,"runs/attempt-001/"+n+".stderr")<<','<<artifact(b,"runs/attempt-001/"+n+".status")<<"],\"started_at\":null,\"finished_at\":null,\"worker\":\"local-native-exact\",\"backend_ref\":\"runs/attempt-001/\"}";
 }
 out<<"],\"flow\":{\"planned\":6,\"attempted\":6,\"completed\":6,\"not_completed\":0,\"interrupted\":0,\"resumed\":0,\"excluded\":[]},\"files\":[";
 std::vector<std::string> files{"protocol.json","PROTOCOL.md","producer.cpp","verify.cpp","build.sh","run.sh","tamper.mjs","inputs.tsv","source.sha256","inputs.sha256","binaries.sha256","BUILD.json","evidence/preparation-delivery.json"};for(const auto& e:fs::recursive_directory_iterator(run))if(e.is_regular_file())files.push_back(fs::relative(e.path(),b).generic_string());bool first=true;for(const auto& f:files){if(!first)out<<',';first=false;out<<artifact(b,f);}
 out<<"],\"analysis\":{\"script\":"<<artifact(b,"producer.cpp","analysis")<<",\"blinded\":false,\"blinding_key_sha256\":null,\"outputs\":["<<artifact(b,"runs/attempt-001/results/summary.json","summary")<<','<<artifact(b,"runs/attempt-001/verification.json","summary")<<"],\"unmasked_at\":null},\"notes\":[{\"kind\":\"observation\",\"text\":\"Six phases completed: three ordinary native exits0, and three correct expected-refusal exits1. Negative controls are not successful verification of their corrupted data. All21original choices retained; P384/P521 have no same-field omitted controls. External reported outcome and labels were seen before replay. No historical, security or independent-holdout claim.\",\"by\":\"native metadata adapter\"}],\"reviews_dir\":\"reviews\"}\n";
 need(bool(out),"write");std::cout<<"Serialized six actual phase records, including three explicit expected refusals\n";return 0;
 }catch(const std::exception& e){std::cerr<<e.what()<<'\n';return 1;}}
