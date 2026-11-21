// Whole-policy cold-call external Ir collector. All scientific dispatch is shell/native.
#include <bits/stdc++.h>
#include <zlib.h>
#include <openssl/sha.h>
#include <nlohmann/json.hpp>
#include <valgrind/callgrind.h>
#include <valgrind/valgrind.h>
namespace Legacy {
#define main legacy_census_unused
#include "../cubic-complete/cubic-complete.cpp"
#undef main
}
namespace RootCover {
#define main rootcover_census_unused
#include "prepared.cpp"
#undef main
}
namespace fs=std::filesystem;using J=nlohmann::ordered_json;
void check(bool b,const char*s){if(!b)throw std::runtime_error(s);}
std::string bytes(const fs::path&p){std::ifstream f(p,std::ios::binary);check(bool(f),"input");return{std::istreambuf_iterator<char>(f),{}};}
std::string hash(const std::string&s){unsigned char d[SHA256_DIGEST_LENGTH];SHA256((const unsigned char*)s.data(),s.size(),d);std::ostringstream o;for(auto b:d)o<<std::hex<<std::setw(2)<<std::setfill('0')<<int(b);return o.str();}
void gz_new(const fs::path&p,const std::string&s){check(!fs::exists(p),"fresh compressed output");gzFile f=gzopen(p.c_str(),"wb9");check(f,"gzip open");check(gzwrite(f,s.data(),s.size())==int(s.size())&&gzclose(f)==Z_OK,"gzip full write");}
std::string gz_read(const fs::path&p){gzFile f=gzopen(p.c_str(),"rb");check(f,"gzip read");std::string s;char b[65536];int n;while((n=gzread(f,b,sizeof b))>0)s.append(b,n);check(n==0&&gzclose(f)==Z_OK,"gzip full read");return s;}
struct Rec{std::array<long long,19>v;};
template<class R> Rec packed(const R&r){auto c=r.count;return{{r.scalar,r.kind,r.rank,r.setup_m,r.shift,r.logs[0],r.logs[1],r.logs[2],c.base_pairs,c.query_pairs,c.branches,c.unknown,c.certified,c.root_checks,c.rows,c.setup_queries,c.online_queries,c.peak,r.progress_mults}};}
volatile unsigned long long probe_state=1;
void probe(){for(int i=0;i<1000000;i++)probe_state=probe_state*3+1;}
void corrupt(const fs::path&,const fs::path&,const std::string&);

int main(int argc,char**argv){try{
 if(argc==5&&std::string(argv[1])=="corrupt"){corrupt(argv[2],argv[3],argv[4]);return 0;}
 check(argc==8,"counter STREAM START END OUT RAWBASE HEAD preflight|full");check(RUNNING_ON_VALGRIND>0,"actual external counter required");std::string tag=argv[1];check(tag=="A"||tag=="B","fresh stream identity");int first=std::stoi(argv[2]),end=std::stoi(argv[3]);check(0<=first&&first<end&&end<=455,"complete declared policy interval");fs::path out=fs::canonical(argv[4]);std::string rawbase=argv[5],head=argv[6];check(std::string(argv[7])=="preflight"||std::string(argv[7])=="full","scope kind");bool preflight=std::string(argv[7])=="preflight";if(preflight)check(first==0&&end==1,"engineering scope fixed before data");else check(end-first<=40,"bounded raw batch, no unfiled full population in one process");check(head.size()==40,"actual source head");check(rawbase==(out/"raw-counter").string(),"fresh raw output scope");
 auto resource_start=std::chrono::steady_clock::now();
 Legacy::RECORD=false;RootCover::RECORD=false;Legacy::terminal_stop=true;RootCover::terminal_stop=true;
 Legacy::P lg{Legacy::F(0),Legacy::F(10),false},ls=Legacy::chi(Legacy::P{Legacy::F(37),Legacy::F(14),false});
 RootCover::P ng{RootCover::F(0),RootCover::F(10),false},ns=RootCover::chi(RootCover::P{RootCover::F(37),RootCover::F(14),false});
 auto oldpoints=Legacy::group(lg,21,39);auto newpoints=RootCover::group(ng,21,39);std::vector<Legacy::P>oldreps;std::vector<RootCover::P>newreps;
 for(auto p:oldpoints)if(!p.o&&p<Legacy::neg(p))oldreps.push_back(p);for(auto p:newpoints)if(!p.o&&p<RootCover::neg(p))newreps.push_back(p);check(oldreps.size()==15&&newreps.size()==15,"complete public factor-base domain");
 for(int qi=0;qi<31;qi++)check(oldpoints[qi].o==newpoints[qi].o&&Legacy::encode(oldpoints[qi].x)==RootCover::encode(newpoints[qi].x)&&Legacy::encode(oldpoints[qi].y)==RootCover::encode(newpoints[qi].y),"same actual original points");
 std::vector<std::array<int,3>>policies;for(int a=0;a<15;a++)for(int b=a+1;b<15;b++)for(int c=b+1;c<15;c++)policies.push_back({a,b,c});check(policies.size()==455,"full policy membership");
 struct Plan{int method,route,policy;std::string label,unique;bool inside,outside,counted;std::vector<Legacy::P>lb;std::vector<RootCover::P>nb;std::array<Rec,30>results;};
 std::vector<Plan>plans;plans.reserve(6*(end-first)+15);
 auto add_plan=[&](int method,int route,int policy,std::string label,bool inside,bool outside,bool counted){
  auto ids=policies.at(policy);std::string unique=tag+"-"+head+"-"+label+"-m"+std::to_string(method)+"-r"+std::to_string(route)+"-p"+std::to_string(policy)+"-d"+std::to_string(3)+"-n30";
  plans.push_back(Plan{method,route,policy,label,unique,inside,outside,counted,{oldreps[ids[0]],oldreps[ids[1]],oldreps[ids[2]]},{newreps[ids[0]],newreps[ids[1]],newreps[ids[2]]},{}});
 };
 for(int route:{3,4,5})for(int method:{0,1})for(int rep=0;rep<2;rep++)add_plan(method,route,first,"warmup-"+std::to_string(rep),false,false,false);
 if(tag=="A")for(int policy=first;policy<end;policy++)for(int route:{3,4,5})for(int method:{0,1})add_plan(method,route,policy,"plain",false,false,true);
 else for(int policy=end-1;policy>=first;policy--)for(int route:{5,4,3})for(int method:{1,0})add_plan(method,route,policy,"plain",false,false,true);
 add_plan(0,3,first,"repeat-aa",false,false,true);add_plan(0,3,first,"positive-inside",true,false,true);add_plan(0,3,first,"negative-outside",false,true,true);
 auto old_fn=&Legacy::recover_cold;auto new_fn=&RootCover::recover_cold;
 asm volatile("" : "+r"(old_fn), "+r"(new_fn) :: "memory");
 // No collector allocation, compression, hashing or serialization between intervals.
 // All inputs, labels and numeric result storage already exist. Tool dumps use tool memory.
 for(auto&plan:plans){
  Legacy::ROUTE=plan.route;RootCover::ROUTE=plan.route;if(plan.outside)probe();
  if(plan.counted){CALLGRIND_ZERO_STATS;CALLGRIND_TOGGLE_COLLECT;}
  for(int qi=1;qi<31;qi++){
   if(plan.method==0){auto r=old_fn(oldpoints[qi],lg,ls,plan.lb,3);plan.results[qi-1]=packed(r);}
   else{auto r=new_fn(newpoints[qi],ng,ns,plan.nb,3);plan.results[qi-1]=packed(r);}
  }
  if(plan.inside)probe();if(plan.counted){CALLGRIND_TOGGLE_COLLECT;CALLGRIND_DUMP_STATS_AT(plan.unique.c_str());}
 }
 // Only now serialize and losslessly compact this bounded batch's NEW raw dumps.
 std::ostringstream records,warmups;records<<"S twist.policy-counter.v1 61 59 31 "<<tag<<' '<<first<<' '<<end<<' '<<head<<'\n';warmups<<"S twist.policy-warmup.v1 "<<tag<<' '<<first<<' '<<head<<'\n';int part=0;
 for(const auto&plan:plans){
  if(!plan.counted){warmups<<"W "<<plan.method<<' '<<plan.route<<' '<<plan.policy<<' '<<plan.label<<'\n';for(int qi=1;qi<31;qi++){auto p=oldpoints[qi];warmups<<"R "<<qi<<' '<<Legacy::encode(p.x)<<' '<<Legacy::encode(p.y);for(auto x:plan.results[qi-1].v)warmups<<' '<<x;warmups<<'\n';}continue;}
  fs::path raw=rawbase+"."+std::to_string(++part);std::string original=bytes(raw);check(!original.empty(),"actual profiler dump");long long declared=-1;std::istringstream lines(original);std::string line;while(std::getline(lines,line))if(line.starts_with("summary:")){std::istringstream row(line.substr(8));row>>declared;}check(declared>0,"external positive instruction summary");
  std::string name="counter-"+std::to_string(part)+".raw.gz";fs::path compressed=out/name;gz_new(compressed,original);check(gz_read(compressed)==original,"lossless inflate byte equality BEFORE raw removal");std::string compressed_bytes=bytes(compressed);
  records<<"P "<<part<<' '<<plan.method<<' '<<plan.route<<' '<<plan.policy<<' '<<(plan.3)<<' '<<plan.label<<' '<<name<<' '<<hash(original)<<' '<<original.size()<<' '<<hash(compressed_bytes)<<' '<<compressed_bytes.size()<<' '<<declared<<'\n';
  for(int qi=1;qi<31;qi++){auto p=oldpoints[qi];records<<"R "<<qi<<' '<<Legacy::encode(p.x)<<' '<<Legacy::encode(p.y);for(auto x:plan.results[qi-1].v)records<<' '<<x;records<<'\n';}
  check(raw.parent_path()==out&&raw.filename().string().starts_with("raw-counter."),"only new redundant dump copy");check(fs::remove(raw),"remove only exact-byte restored new raw copy");
 }
 gz_new(out/"cases.tsv.gz",records.str());gz_new(out/"warmups.tsv.gz",warmups.str());check(part==6*(end-first)+3,"entire declared counter interval population");
 auto elapsed=std::chrono::duration_cast<std::chrono::nanoseconds>(std::chrono::steady_clock::now()-resource_start).count();
 check(!fs::exists(out/"resource-observation.json"),"fresh resource metadata");std::ofstream resource(out/"resource-observation.json");resource<<J{{"schema","twist.resource-observation.v1"},{"wall_ns",elapsed},{"counter_intervals",part},{"speed_claim",false},{"purpose","Runtime/storage envelope only; producer main excludes external tool startup"}}.dump()<<'\n';check(bool(resource),"resource metadata write");return 0;
}catch(const std::exception&e){std::cerr<<e.what()<<'\n';return 101;}}

std::vector<std::string> tokens(const std::string&s){std::istringstream in(s);std::string t;std::vector<std::string>out;while(in>>t)out.push_back(t);return out;}
std::string joined(const std::vector<std::string>&w){std::ostringstream o;for(size_t i=0;i<w.size();i++){if(i)o<<' ';o<<w[i];}return o.str();}
void corrupt(const fs::path&source,const fs::path&out,const std::string&mode){
 check(fs::create_directory(out),"fresh false directory");for(const auto&e:fs::directory_iterator(source)){check(e.is_regular_file()&&!e.is_symlink(),"regular source corpus");fs::create_hard_link(e.path(),out/e.path().filename());}
 auto original=gz_read(source/"cases.tsv.gz");std::vector<std::string>lines;std::istringstream input(original);std::string line;while(std::getline(input,line))lines.push_back(line);bool done=false;std::map<std::string,std::string>names;
 if(mode=="source"){auto w=tokens(lines[0]);check(w.size()==9,"source header");w[8]=std::string(40,'0');lines[0]=joined(w);done=true;}
 for(size_t i=1;i<lines.size()&&!done;i++){
  auto w=tokens(lines[i]);if(mode=="scalar"&&w[0]=="R"){w[4]=std::to_string((std::stoll(w[4])+1)%31);lines[i]=joined(w);done=true;break;}
  if(mode=="witness"&&w[0]=="R"){w[21]=std::to_string(std::stoll(w[21])+1);lines[i]=joined(w);done=true;break;}
  if(w[0]!="P")continue;
  if(mode=="cost"){w[12]=std::to_string(std::stoull(w[12])+1);lines[i]=joined(w);done=true;break;}
  if(mode=="missing"){check(i+31<=lines.size(),"full interval site");lines.erase(lines.begin()+i,lines.begin()+i+31);done=true;break;}
  if(mode!="raw-summary"&&mode!="uncharged-method")continue;
  std::string raw=gz_read(source/w[7]);std::vector<std::string>rl;std::istringstream rin(raw);while(std::getline(rin,line))rl.push_back(line);
  auto definition=[&](const std::string&s){auto eq=s.find('=');auto close=s.find(')',eq);if(eq==std::string::npos||close==std::string::npos)return std::string();auto id=s.substr(eq+2,close-eq-2);auto name=s.substr(close+1);auto pos=name.find_first_not_of(' ');if(pos!=std::string::npos)names[id]=name.substr(pos);return id;};
  for(const auto&s:rl)if(s.starts_with("fn=")||s.starts_with("cfn="))definition(s);
  if(mode=="uncharged-method"&&w[2]!="1")continue;
  long long adjustment=mode=="raw-summary"?1:0;bool current=false,skip=false;size_t np=0;for(const auto&s:rl)if(s.starts_with("positions:"))np=tokens(s.substr(10)).size();
  if(mode=="uncharged-method")for(auto&s:rl){
   if(s.starts_with("fn=")){auto id=definition(s);current=names[id].find("RootCover::recover_cold")!=std::string::npos;continue;}
   if(s.starts_with("calls=")){skip=true;continue;}auto cost=tokens(s);if(cost.size()!=np+1)continue;bool numeric=true;for(char c:cost.back())if(c<'0'||c>'9')numeric=false;if(!numeric)continue;if(skip){skip=false;continue;}
   if(current){auto value=std::stoll(cost.back());adjustment-=value;cost.back()="0";s=joined(cost);}
  }
  check(adjustment!=0,"actual non-null counter mutation required");for(auto&s:rl)for(const std::string prefix:{"summary:","totals:"})if(s.starts_with(prefix)){auto old=std::stoll(s.substr(prefix.size()));check(old+adjustment>0,"positive false external total");s=prefix+" "+std::to_string(old+adjustment);}
  std::ostringstream changed;for(const auto&s:rl)changed<<s<<'\n';auto false_raw=changed.str();check(false_raw!=raw,"changed raw counter required");fs::remove(out/w[7]);gz_new(out/w[7],false_raw);check(gz_read(out/w[7])==false_raw,"false raw byte retention");auto compressed=bytes(out/w[7]);w[8]=hash(false_raw);w[9]=std::to_string(false_raw.size());w[10]=hash(compressed);w[11]=std::to_string(compressed.size());w[12]=std::to_string(std::stoll(w[12])+adjustment);lines[i]=joined(w);done=true;
 }
 check(done,"requested real false site missing");std::ostringstream changed;for(const auto&s:lines)changed<<s<<'\n';check(changed.str()!=original,"non-null payload");fs::remove(out/"cases.tsv.gz");gz_new(out/"cases.tsv.gz",changed.str());check(gz_read(source/"cases.tsv.gz")==original,"original cases unchanged");std::cout<<J{{"mode",mode},{"original_sha256",hash(original)},{"changed_sha256",hash(changed.str())},{"null",false}}.dump()<<'\n';
}
