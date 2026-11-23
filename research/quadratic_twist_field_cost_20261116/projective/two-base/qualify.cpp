// Native source/input qualification and immutable manifest IO; no solver arithmetic.
#include <nlohmann/json.hpp>
#include <openssl/sha.h>
#include <filesystem>
#include <fstream>
#include <sstream>
#include <iomanip>
#include <iostream>
#include <stdexcept>
#include <cstdio>
#include <ctime>
#include <fcntl.h>
#include <unistd.h>
using J=nlohmann::ordered_json;namespace fs=std::filesystem;
void need(bool b,const char*s){if(!b)throw std::runtime_error(s);}
std::string read(const fs::path&p){std::ifstream f(p,std::ios::binary);need(bool(f),"missing declared input");return{std::istreambuf_iterator<char>(f),{}};}
std::string hash(const fs::path&p){auto s=read(p);unsigned char h[SHA256_DIGEST_LENGTH];SHA256((const unsigned char*)s.data(),s.size(),h);std::ostringstream o;for(auto x:h)o<<std::hex<<std::setw(2)<<std::setfill('0')<<int(x);return o.str();}
std::string command(const char*s){FILE*f=popen(s,"r");need(f,"git metadata command");std::string out;char b[4096];while(fgets(b,sizeof b,f))out+=b;need(pclose(f)==0,"git metadata command exit");while(!out.empty()&&(out.back()=='\n'||out.back()=='\r'))out.pop_back();return out;}
void create(const fs::path&p,const J&j){auto s=j.dump(2)+"\n";int f=open(p.c_str(),O_WRONLY|O_CREAT|O_EXCL,0600);need(f>=0,"refuse replacing a manifest");size_t at=0;while(at<s.size()){auto n=write(f,s.data()+at,s.size()-at);need(n>0,"manifest write");at+=size_t(n);}need(close(f)==0,"manifest close");}
void validate_paths(const J&files,bool repository){for(auto it=files.begin();it!=files.end();++it){auto name=it.key();for(auto part:fs::path(name))need(part!="..","input traversal");need(!fs::is_symlink(name)&&fs::is_regular_file(name),"regular declared file");if(repository){need(name.starts_with("research/quadratic_twist_field_cost_20261116/projective/"),"repository scope");}else{auto actual=fs::canonical(name).string();need(actual.starts_with("/workspace/twist-")||(actual.starts_with("/tmp/twist-first-row-run01-20261116/")||actual.starts_with("/tmp/twist-setup-rank-run02-20261116/")||actual.starts_with("/tmp/twist-two-base-20261116/")||actual.starts_with("/tmp/twist-two-base-cargo/")||actual.starts_with("/tmp/twist-setup-rank-20261116/"))||actual.starts_with("/workspace/canonical-protocol-20261115/"),"external input scope");}need(hash(name)==it.value().get<std::string>(),"declared file hash mismatch");}}
int main(int argc,char**argv){try{
 need(argc==6,"qualify build|pre|post PINS REF OUT MANIFEST");std::string mode=argv[1];need(mode=="build"||mode=="pre"||mode=="post","mode");fs::path out=fs::canonical(argv[4]);need(out.string().starts_with("/tmp/twist-two-base-run"),"fresh output scope");
 auto pins=J::parse(read(argv[2]));auto ref=J::parse(read(argv[3]));auto head=command("git rev-parse HEAD");need(command("git branch --show-current")=="codex/twist-field-cost","branch");need(command("git remote get-url origin")=="https://github.com/isogeny-labs/crypto.git","repository");need(command("git status --porcelain").empty(),"clean committed source");need(ref["head"]==head,"reference head mismatch");need(ref["state"]=="open"&&ref["auto_merge"].is_null()&&ref["source"]=="connected-github-read","connected reference");
 auto protocol=J::parse(read("research/quadratic_twist_field_cost_20261116/projective/two-base/protocol.json"));need(protocol["freeze"]["status"]=="frozen","frozen metadata protocol required");need(protocol["extensions"]["pins_sha256"]==hash(argv[2]),"source PINS binding mismatch");
 if(mode=="pre"){auto age=std::time(nullptr)-ref["observed_epoch"].get<long long>();need(age>=0&&age<=60,"fresh pre-call reference required");}
 validate_paths(pins["repo_files"],true);validate_paths(pins["external_files"],false);
 validate_paths(J::parse(read(pins["mathlib_inventory"].get<std::string>())),false);
 validate_paths(J::parse(read(pins["rust_dependency_inventory"].get<std::string>())),false);
 need(protocol["execution"]["placement"]["absolute_deadline"]=="2026-11-16T19:45:00Z"&&protocol["execution"]["placement"]["start_at"]=="2026-11-16T18:45:00Z"&&protocol["execution"]["timeout_seconds"]==3600,"explicit new typed budget");
 need(protocol["analysis_plan"]["script"]["path"]=="research/quadratic_twist_field_cost_20261116/projective/two-base/analyze.cpp"&&protocol["analysis_plan"]["script"]["sha256"]==hash("research/quadratic_twist_field_cost_20261116/projective/two-base/analyze.cpp"),"exact typed actual analyzer");
 auto corpus=J::parse(read(pins["legacy_controls_manifest"].get<std::string>()));validate_paths(corpus["files"],false);

 J bins=J::object();for(const char*name:{"qualifier.immutable","producer","audit","references","analysis","semantic","math-audit","idc","source-cap2-probe"}){auto p=out/name;bins[name]=J{{"sha256",hash(p)},{"bytes",fs::file_size(p)}};}
 if(mode=="post"){
  auto pre=J::parse(read(out/"pre-call-manifest.json"));need(pre["binaries"]==bins&&pre["source_commit"]==head,"pre-call source/binaries changed");
  std::istringstream stream(read(out/"stages.tsv"));std::string line;size_t index=0;for(;std::getline(stream,line);){std::istringstream row(line);std::string name;int actual,expected;need(bool(row>>name>>actual>>expected),"stage row");need(index<pins["stage_plan"].size(),"extra stage");const auto&plan=pins["stage_plan"][index++];need(plan["name"]==name&&plan["expected_exit"]==expected&&actual==expected,"stage outcome/order");}need(index==pins["stage_plan"].size(),"incomplete execution plan");
 }
 if(mode=="pre"){auto age=std::time(nullptr)-ref["observed_epoch"].get<long long>();need(age>=0&&age<=60,"reference expired while qualifying inputs");}
 create(argv[5],J{{"schema","twist.native-execution-manifest/v1"},{"mode",mode},{"source_commit",head},{"pins_sha256",hash(argv[2])},{"reference_sha256",hash(argv[3])},{"legacy_controls_manifest_sha256",hash(pins["legacy_controls_manifest"].get<std::string>())},{"legacy_control_paths",corpus["files"].size()},{"binaries",bins},{"signed_verdict",false}});return 0;
 }catch(const std::exception&e){std::cerr<<e.what()<<'\n';return 101;}}
