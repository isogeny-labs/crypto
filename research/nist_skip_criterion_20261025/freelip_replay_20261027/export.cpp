/* Native format adapter for already observed records; no primality arithmetic. */
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
static void need(bool b,const std::string&s){if(!b)throw std::runtime_error(s);}
static std::string read(const fs::path&p){std::ifstream f(p,std::ios::binary);need(bool(f),"missing "+p.string());return {std::istreambuf_iterator<char>(f),{}};}
static std::string quote(const std::string&s){std::string o="\"";for(unsigned char c:s){if(c=='"'||c=='\\'){o+='\\';o+=char(c);}else if(c=='\n')o+="\\n";else if(c=='\r')o+="\\r";else if(c=='\t')o+="\\t";else{need(c>=32,"control character");o+=char(c);}}return o+'"';}
static std::string sha(const std::string&s){unsigned char h[32];unsigned int n=0;need(EVP_Digest(s.data(),s.size(),h,&n,EVP_sha256(),nullptr)==1&&n==32,"hash");std::ostringstream o;for(auto b:h)o<<std::hex<<std::setw(2)<<std::setfill('0')<<unsigned(b);return o.str();}
static std::string file(const fs::path&base,const std::string&name,const char*kind){const auto b=read(base/name);return "{\"path\":"+quote(name)+",\"sha256\":"+quote(sha(b))+",\"bytes\":"+std::to_string(b.size())+",\"kind\":"+quote(kind)+"}";}
static std::vector<std::string> split(const std::string&s){std::vector<std::string> v;std::istringstream f(s);std::string x;while(std::getline(f,x,'\t'))v.push_back(x);return v;}
int main(int argc,char**argv){try{
 need(argc==5,"usage: export STUDY CREATED OUT SOURCE-COMMIT");const fs::path base=argv[1];need(std::regex_match(argv[4],std::regex("[0-9a-f]{40}")),"source commit");const std::string protocol=read(base/"protocol.json");std::smatch m;need(std::regex_search(protocol,m,std::regex("\"protocol_sha256\"\\s*:\\s*\"([0-9a-f]{64})\"")),"protocol hash");
 std::ofstream o(argv[3]);need(bool(o),"output");o<<"{\"schema\":\"experiment.export/v1\",\"export_id\":\"nist-freelip-full21-20261027-001\",\"created\":"<<quote(argv[2])<<",\"protocol\":{\"path\":\"protocol.json\",\"sha256\":"<<quote(m[1])<<",\"protocol_id\":\"nist-freelip-full21-20261027\",\"version\":3},\"producer\":{\"runtime\":\"codex\",\"model\":null,\"session\":\"nist-skip-criterion\",\"role\":\"author\"},\"source\":{\"repo\":\"crypto\",\"commit\":"<<quote(argv[4])<<",\"dirty\":null},\"backend\":\"local\",\"host\":{\"path\":null,\"sha256\":null,\"class_id\":null,\"summary\":{\"cpu_model\":null,\"logical_cpus\":null,\"memory_bytes\":null,\"kernel\":null,\"hypervisor\":null,\"isolation_tier_max\":null}},\"arms\":[";
 bool first=true;for(const std::string version:{"1.0","1.1"})for(const std::string method:{"probprime","ordinary"}){if(!first)o<<',';first=false;const auto bin="runs/replay-001/bin/v"+version+"-replay";o<<"{\"id\":"<<quote("v"+version.substr(0,1)+version.substr(2)+"-"+method)<<",\"role\":"<<quote(method=="ordinary"?"reference":version=="1.0"?"baseline":"candidate")<<",\"binary_sha256\":"<<quote(sha(read(base/bin)))<<",\"commit\":\"939664878a4bb679d8a74aa9f0fef39f43b4a5d4\"}";}
 o<<"],\"runs\":[";std::istringstream table(read(base/"results/cohort.tsv"));std::string line;std::getline(table,line);std::vector<std::string> all_files;long attempted=0,complete=0,not_complete=0;first=true;
 while(std::getline(table,line)){const auto r=split(line);need(r.size()==6,"table schema");need(r[0]=="1.0"||r[0]=="1.1","version");need(r[1]=="probprime"||r[1]=="ordinary","method");need(r[5]=="ACCEPTED"||r[5]=="REJECTED_SOFTWARE"||r[5]=="INDETERMINATE","classification");
  std::istringstream inventory(read(base/"inputs/cohort.tsv"));std::string row,id;std::getline(inventory,row);while(std::getline(inventory,row)){const auto q=split(row);need(q.size()==5,"inventory");if(q[1]==r[2]){id=q[0];break;}}need(!id.empty(),"missing identity");
  const std::string dir="runs/replay-001/v"+r[0]+"-"+r[1]+"/"+id;const bool has_status=fs::exists(base/dir/"process.status");if(!has_status)continue;int exit_code=-1;std::istringstream status(read(base/dir/"process.status"));need(bool(status>>exit_code),"status");const bool valid=exit_code==0&&r[5]!="INDETERMINATE";const auto arm="v"+r[0].substr(0,1)+r[0].substr(2)+"-"+r[1];
  if(!first) { o<<','; }
  first=false;++attempted;valid?++complete:++not_complete;
  o<<"{\"run_id\":"<<quote(arm+"-"+id)<<",\"arm\":"<<quote(arm)<<",\"instance_id\":"<<quote(r[2])<<",\"seed\":7157891,\"replicate\":0,\"warmup\":false,\"status\":"<<quote(valid?"succeeded":exit_code==124?"timeout":"failed")<<",\"outcome_class\":"<<quote(valid?"completed":"not_completed")<<",\"attempts\":1,\"resumed\":0,\"checkpoint\":null,\"measurements\":{\"counted\":{";
  if(valid)o<<"\"legacy_return\":"<<(r[5]=="ACCEPTED"?1:0);
  o<<"},\"wall_seconds\":null,\"wall_valid\":false,\"cpu_user_seconds\":null,\"cpu_sys_seconds\":null,\"max_rss_bytes\":null,\"counters\":{\"software_classification\":"<<quote(r[5])<<"}},\"isolation\":{\"tier\":null,\"contended\":null,\"blockers\":[\"Shared-host functional replay; no clock admitted\"]},\"verification\":{\"status\":"<<quote(valid?"verified":"error")<<",\"verifier\":\"Source-faithful native input/configuration/return observer; no new prime proof\"},\"exclusion\":{\"excluded\":false,\"rule\":null},\"artifacts\":[";
  bool af=true;for(const std::string name:{"input","stdout","stderr","process.status"}){if(!af)o<<',';af=false;const auto f=dir+"/"+name;o<<file(base,f,name=="input"?"raw":"log");all_files.push_back(f);}o<<"],\"started_at\":null,\"finished_at\":null,\"worker\":\"local-qemu-user-mode\",\"backend_ref\":"<<quote(dir)<<'}';
 }
 need(attempted<=84,"extra calls");o<<"],\"flow\":{\"planned\":84,\"attempted\":"<<attempted<<",\"completed\":"<<complete<<",\"not_completed\":"<<not_complete<<",\"interrupted\":0,\"resumed\":0,\"excluded\":[]},\"files\":[";first=true;for(const auto&f:all_files){if(!first)o<<',';first=false;o<<file(base,f,"raw");}
 o<<"],\"analysis\":{\"script\":"<<file(base,"summarize.cpp","analysis")<<",\"blinded\":false,\"blinding_key_sha256\":null,\"outputs\":["<<file(base,"results/cohort.tsv","summary")<<','<<file(base,"results/summary.json","summary")<<"],\"unmasked_at\":null},\"notes\":[{\"kind\":\"observation\",\"text\":\"84 software calls on21 distinct already-known-prime orders, not84 independent historical choices. Original version3 observer treated wrapped roots as unknown; corrected saved validation is post hoc and does not rerun legacy code. Unrecorded clock/rusage/host fields remain null.\",\"by\":\"native-source study\"}],\"reviews_dir\":\"reviews\",\"backend_record\":"<<read(base/"results/summary.json")<<"}\n";
 std::cout<<"Native unified export: "<<attempted<<" calls, "<<complete<<" complete, "<<not_complete<<" incomplete.\n";return 0;
}catch(const std::exception&e){std::cerr<<e.what()<<'\n';return 1;}}
