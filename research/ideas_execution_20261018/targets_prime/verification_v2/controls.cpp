#define main diagnostic_main_unused
#include "../diagnostic.cpp"
#undef main
#include "vendor/json.hpp"
using Json=nlohmann::json;
using Path=std::filesystem::path;
static std::vector<Json> load(const Path& path){std::ifstream in(path);if(!in)throw std::runtime_error("control input open");std::vector<Json> out;std::string line;while(std::getline(in,line))out.push_back(Json::parse(line));return out;}
static void write(const Path& path,const std::vector<Json>& rows){if(std::filesystem::exists(path))throw std::runtime_error("control output exists");std::ofstream out(path);for(const Json& row:rows)out<<row.dump()<<"\n";if(!out)throw std::runtime_error("control output failure");}
int main(int argc,char**argv){try{
 if(argc!=2)throw std::runtime_error("usage: controls FRESH_CONTROL_DIRECTORY");Path dir=argv[1];if(std::filesystem::exists(dir))throw std::runtime_error("control directory exists");std::filesystem::create_directories(dir);
 Path source="/workspace/crypto/research/ideas_execution_20261018/targets_prime/runs/run-20261018-strict4-v1";auto rows=load(source/"results.jsonl"),certs=load(source/"certificates.jsonl");
 auto emit=[&](const std::string& name,const std::vector<Json>& input,const std::vector<Json>& certificates){Path out=dir/name;std::filesystem::create_directory(out);write(out/"results.jsonl",input);write(out/"certificates.jsonl",certificates);};
 auto changed=certs;changed[0]["target_id"]=0;emit("wrong-target-id",rows,changed);
 changed=certs;changed.erase(changed.begin());emit("missing-one",rows,changed);
 emit("missing-all",rows,{});
 changed=certs;changed.push_back(certs[0]);emit("duplicate-certificate",rows,changed);
 changed=certs;Json extra=certs[0];extra["target_id"]=100000;changed.push_back(extra);emit("extra-target",rows,changed);
 U cid=certs[0]["curve_id"].get<U>(),tid=certs[0]["target_id"].get<U>();size_t position=0;for(;position<rows.size();++position)if(rows[position]["curve_id"]==cid&&rows[position]["target_id"]==tid)break;
 if(position==rows.size())throw std::runtime_error("control binding missing");U p=rows[position]["p"].get<U>();
 changed=certs;changed[0]["target"][1]=(p-changed[0]["target"][1].get<U>())%p;emit("altered-on-curve-target",rows,changed);
 Curve curve(p,false,rows[position]["a"].get<U>(),rows[position]["b"].get<U>());U B=rows[position]["B"].get<U>();P base;
 for(U x=0;x<B&&base.inf;++x){auto it=curve.roots.find(curve.rhs(x));if(it!=curve.roots.end())base={x,it->second.front(),false};}
 if(base.inf)throw std::runtime_error("repeat-point control has empty base");P target=curve.mul(4,base);if(target.inf)throw std::runtime_error("repeat-point control infinity");
 auto consistent=rows;consistent[position]["target"]=Json::array({target.x,target.y});changed=certs;changed[0]["target"]=Json::array({target.x,target.y});changed[0]["points"]=Json::array();for(int i=0;i<4;++i)changed[0]["points"].push_back(Json::array({base.x,base.y}));changed[0]["distinct_abscissae"]=true;emit("false-strict-claim",consistent,changed);
 Path duplicate=dir/"duplicate-json-key";std::filesystem::create_directory(duplicate);write(duplicate/"results.jsonl",rows);std::ofstream out(duplicate/"certificates.jsonl");std::string first=certs[0].dump();first.pop_back();out<<first<<",\"curve_id\":"<<cid<<"}\n";for(size_t i=1;i<certs.size();++i)out<<certs[i].dump()<<"\n";
 Curve singular(17,true,14,2);bool old=singular.nonsingular();auto& f=singular.f;U proper=f.add(f.mul(4,f.mul(f.mul(14,14),14)),f.mul(27%17,f.mul(2,2)));if(!old||proper!=0)throw std::runtime_error("discriminant regression did not exhibit defect");
 std::ofstream regression(dir/"discriminant-regression.json");regression<<Json({{"p",17},{"extension_degree",2},{"a",14},{"b",2},{"old_nonsingular_gate",old},{"correct_discriminant",proper},{"singular",true}}).dump()<<"\n";
 std::cout<<"Created eight semantic certificate controls and one singular-discriminant counterexample; original datasets unchanged.\n";return 0;
}catch(const std::exception& error){std::cerr<<error.what()<<"\n";return 1;}}
