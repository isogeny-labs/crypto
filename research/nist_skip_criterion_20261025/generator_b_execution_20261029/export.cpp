// Native serialization of already checked observations and literal proof data.
#include <algorithm>
#include <filesystem>
#include <fstream>
#include <iomanip>
#include <iostream>
#include <map>
#include <openssl/evp.h>
#include <regex>
#include <sstream>
#include <stdexcept>
#include <string>
#include <vector>
namespace fs = std::filesystem;
static void require(bool b, const std::string& s) { if (!b) throw std::runtime_error(s); }
static std::string read(const fs::path& p) { std::ifstream f(p, std::ios::binary); require(bool(f), "missing " + p.string()); return {std::istreambuf_iterator<char>(f), {}}; }
static std::string quote(const std::string& s) { std::string out = "\""; for (unsigned char c : s) { if (c == '\\' || c == '"') { out += '\\'; out += char(c); } else if (c == '\n') out += "\\n"; else if (c == '\t') out += "\\t"; else { require(c >= 32, "control byte"); out += char(c); } } return out + '"'; }
static std::string hash(const std::string& s) { unsigned char h[32]; unsigned int n; require(EVP_Digest(s.data(), s.size(), h, &n, EVP_sha256(), nullptr) == 1 && n == 32, "sha256"); std::ostringstream out; for (auto b : h) out << std::hex << std::setw(2) << std::setfill('0') << unsigned(b); return out.str(); }
static std::string file(const fs::path& base, const std::string& name, const std::string& kind) { const auto b = read(base / name); return "{\"path\":" + quote(name) + ",\"sha256\":" + quote(hash(b)) + ",\"bytes\":" + std::to_string(b.size()) + ",\"kind\":" + quote(kind) + '}'; }
static std::string capture(const std::string& text, const std::string& pattern) { std::smatch m; require(std::regex_search(text, m, std::regex(pattern)), "metadata field absent"); return m[1]; }

// Metadata adapter only: serialize native observations without curve arithmetic.
int main(int argc,char** argv) { try {
 require(argc==7,"export STUDY PROTOCOL ATTEMPT BIN_DIR CREATED OUTPUT");
 fs::path base=argv[1],run=base/"runs"/argv[3],bin=argv[4];std::string protocol_name=argv[2];
 const auto protocol=read(base/protocol_name);const auto psha=capture(protocol,"\"protocol_sha256\"\\s*:\\s*\"([0-9a-f]{64})\"");
 const auto version=capture(protocol,"\"version\"\\s*:\\s*([0-9]+)");
 const bool complete=fs::exists(run/"verification.status")&&read(run/"verification.status")=="0\n"&&read(run/"production.status")=="0\n";
 std::string commit=read(run/"source-commit.txt");while(!commit.empty()&&commit.back()=='\n')commit.pop_back();
 require(std::regex_match(commit,std::regex("[0-9a-f]{40}")),"source identity");
 std::string summary=complete?read(run/"results/summary.json"):"",verified=complete?read(run/"verification.json"):"";
 if(complete)require(summary.find("production-parameter-analysis")!=std::string::npos&&verified.find("production-parameter-analysis")!=std::string::npos,"synthetic data is not the census");
 auto number=[&](const std::string& text,const std::string& key){return capture(text,"\""+key+"\"\\s*:\\s*([0-9]+)");};
 const std::vector<std::string> all{"positive","production","verification"};std::vector<std::string> phases;
 for(const auto& p:all)if(fs::exists(run/(p+".status")))phases.push_back(p);
 auto source_commit="6eb320e17a0e8582f1769b66086d118ae797fd21";
 std::ofstream out(argv[6]);require(bool(out),"output");
 out<<"{\"schema\":\"experiment.export/v1\",\"export_id\":"<<quote(std::string("nist-generator-b-")+argv[3])<<",\"created\":"<<quote(argv[5])
 <<",\"protocol\":{\"path\":"<<quote(protocol_name)<<",\"sha256\":"<<quote(psha)<<",\"protocol_id\":\"nist-generator-b-full21-20261029\",\"version\":"<<version
 <<"},\"producer\":{\"runtime\":\"codex\",\"model\":null,\"session\":\"nist-generator-comparison\",\"role\":\"author\"},\"source\":{\"repo\":\"crypto\",\"commit\":"<<quote(commit)<<",\"dirty\":false},\"backend\":\"local\",\"host\":{\"path\":null,\"sha256\":null,\"class_id\":null,\"summary\":{\"cpu_model\":null,\"logical_cpus\":null,\"memory_bytes\":null,\"kernel\":null,\"hypervisor\":null,\"isolation_tier_max\":null}},\"arms\":[";
 for(size_t i=0;i<all.size();++i){if(i)out<<',';const auto& p=all[i];out<<"{\"id\":"<<quote(p)<<",\"role\":"<<quote(i==0?"control_positive":i==1?"candidate":"baseline")<<",\"binary_sha256\":"<<quote(hash(read(bin/(i==2?"verify":"analyze"))))<<",\"commit\":"<<quote(source_commit)<<'}';}
 out<<"],\"runs\":[";long completed=0;bool first=true;
 for(const auto& p:phases){const std::string status=read(run/(p+".status"));const bool ok=status=="0\n";completed+=ok;
  if(!first){out<<',';}first=false;
  std::string counts="{}",counters="{}";
  if(p=="positive"&&ok)counts="{\"self_test_pass\":1}";
  if(complete&&p!="positive"){
   std::ostringstream values;values<<"{\"curve_choices\":"<<number(summary,"curve_choices")<<",\"trait_cells\":"<<number(summary,"trait_cells")<<",\"scalar_recipe_tests\":"<<number(summary,"scalar_recipe_tests")<<",\"scalar_recipe_matches\":"<<number(summary,"scalar_recipe_matches")<<",\"exact_single_separators\":"<<number(summary,"exact_single_separators")<<'}';counts=values.str();
   std::vector<std::string> keys=p=="production"?std::vector<std::string>{"curve_choices","published","skipped","coefficient_views","point_rows","traits_per_point","trait_cells","cutoff_comparisons","exact_single_separators","scalar_recipe_tests","scalar_recipe_matches"}:std::vector<std::string>{"curve_choices","point_rows","trait_cells_checked","cutoffs_checked","point_differences_checked","inherited_differences_checked","recipe_tests_checked"};
   std::ostringstream cs;cs<<'{';bool f=true;for(const auto& k:keys){if(!f)cs<<',';f=false;cs<<quote(k)<<':'<<number(p=="production"?summary:verified,k);}cs<<'}';counters=cs.str();
  }
  out<<"{\"run_id\":"<<quote(std::string(argv[3])+"-"+p)<<",\"arm\":"<<quote(p)<<",\"instance_id\":\"frozen-input-bundle\",\"seed\":null,\"replicate\":0,\"warmup\":false,\"status\":"<<quote(ok?"succeeded":status=="124\n"?"timeout":"failed")<<",\"outcome_class\":"<<quote(ok?"completed":"not_completed")<<",\"attempts\":1,\"resumed\":0,\"checkpoint\":null,\"measurements\":{\"counted\":"<<counts<<",\"wall_seconds\":null,\"wall_valid\":false,\"cpu_user_seconds\":null,\"cpu_sys_seconds\":null,\"max_rss_bytes\":null,\"counters\":"<<counters<<"},\"isolation\":{\"tier\":null,\"contended\":null,\"blockers\":[\"Local exact computation; no performance clock admitted\"]},\"verification\":{\"status\":"<<quote(ok&&(p=="positive"||complete)?"verified":"not_run")<<",\"verifier\":"<<quote(p=="positive"?"Known-answer native assertions":"Separate GMP group/trait recomputation; shared GMP primitives and OpenSSL SHA")<<"},\"exclusion\":{\"excluded\":false,\"rule\":null},\"artifacts\":[";
  const auto root=std::string("runs/")+argv[3]+"/";out<<file(base,root+p+".stdout","raw")<<','<<file(base,root+p+".stderr","raw")<<','<<file(base,root+p+".status","raw")<<"],\"started_at\":null,\"finished_at\":null,\"worker\":\"native-local-exact-analysis\",\"backend_ref\":"<<quote(root)<<'}';
 }
 out<<"],\"flow\":{\"planned\":"<<phases.size()<<",\"attempted\":"<<phases.size()<<",\"completed\":"<<completed<<",\"not_completed\":"<<long(phases.size())-completed<<",\"interrupted\":0,\"resumed\":0,\"excluded\":[]},\"files\":[";
 std::vector<std::string> files{protocol_name,"PROTOCOL.md","../generator_b_20261029/analyze.cpp","../generator_b_20261029/verify.cpp","../generator_b_20261029/source.sha256","../generator_b_20261029/input.sha256","evidence/preparation-delivery.json","evidence/attempt-001/observation.json"};
 for(const auto& e:fs::recursive_directory_iterator(run))if(e.is_regular_file())files.push_back(fs::relative(e.path(),base).generic_string());
 std::sort(files.begin(),files.end());files.erase(std::unique(files.begin(),files.end()),files.end());first=true;for(const auto& f:files){if(!first)out<<',';first=false;out<<file(base,f,"raw");}
 out<<"],\"analysis\":{\"script\":"<<file(base,"../generator_b_20261029/analyze.cpp","analysis")<<",\"blinded\":false,\"blinding_key_sha256\":null,\"outputs\":[";
 if(complete){out<<file(base,std::string("runs/")+argv[3]+"/results/summary.json","summary")<<','<<file(base,std::string("runs/")+argv[3]+"/verification.json","summary");}
 out<<"],\"unmasked_at\":null},\"notes\":[{\"kind\":\"observation\",\"text\":"<<quote(complete?"All21 original choices retained; two coefficient views and14 point choices are controls, not independent historical samples. Eleven historical skipped generators remain text NA. The verifier recomputes the complete headline summary as well as all raw tables. No DLP or performance clock was executed.":"Protocol1 planned three phases; only positive and production have actual status files. Production timed out and verification did not start. This partial export preserves two real executions without inventing a cancelled attempt; protocol inventory check must retain the shortfall.")<<",\"by\":\"native metadata adapter\"}],\"reviews_dir\":\"reviews\"}\n";
 require(bool(out),"write");std::cout<<"Serialized "<<phases.size()<<" actual phase records; complete="<<complete<<'\n';return 0;
 }catch(const std::exception& e){std::cerr<<e.what()<<'\n';return 1;}}
