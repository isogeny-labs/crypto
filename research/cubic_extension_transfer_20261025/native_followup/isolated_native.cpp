// Native replacement of the isolation contract in tools/isolated_bench.py.
// Missing PSI is a strict preflight failure. --diagnostic may collect reserved,
// serialized observations but never labels them qualified wall-time evidence.
#include "json.hpp"
#include <algorithm>
#include <cerrno>
#include <chrono>
#include <cstring>
#include <filesystem>
#include <fstream>
#include <iostream>
#include <map>
#include <set>
#include <sstream>
#include <string>
#include <thread>
#include <vector>
#include <fcntl.h>
#include <sched.h>
#include <signal.h>
#include <sys/file.h>
#include <sys/resource.h>
#include <sys/wait.h>
#include <unistd.h>
using json=nlohmann::json;namespace fs=std::filesystem;
using Clock=std::chrono::steady_clock;
struct Proc {std::string name;long parent=0;uint64_t ticks=0,start=0;};
std::map<int,Proc> snapshot(){
 std::map<int,Proc>out;std::error_code ec;
 for(const auto&entry:fs::directory_iterator("/proc",ec)){
  std::string n=entry.path().filename();if(n.empty()||!std::all_of(n.begin(),n.end(),::isdigit))continue;
  std::ifstream f(entry.path()/"stat");std::string s;std::getline(f,s);auto pos=s.rfind(')');auto left=s.find('(');if(pos==std::string::npos||left==std::string::npos)continue;
  std::istringstream rest(s.substr(pos+2));std::vector<std::string>v;std::string w;while(rest>>w)v.push_back(w);if(v.size()<20)continue;
  out.emplace(std::stoi(n),Proc{s.substr(left+1,pos-left-1),std::stol(v[1]),std::stoull(v[11])+std::stoull(v[12]),std::stoull(v[19])});
 }return out;
}
double other_cpu(const std::map<int,Proc>&before,const std::map<int,Proc>&after,const std::set<int>&exclude){
 uint64_t ticks=0;for(auto&[pid,p]:after){if(exclude.count(pid))continue;auto it=before.find(pid);uint64_t old=(it!=before.end()&&it->second.start==p.start)?it->second.ticks:0;if(p.ticks>=old)ticks+=p.ticks-old;}return double(ticks)/sysconf(_SC_CLK_TCK);
}
json psi(const std::string&kind){
 for(auto path:{"/proc/pressure/"+kind,"/sys/fs/cgroup/"+kind+".pressure"}){std::ifstream f(path);std::string row;if(!std::getline(f,row))continue;json d={{"path",path}};do{std::istringstream ss(row);std::string name,token;ss>>name;while(ss>>token){auto p=token.find('=');if(p!=std::string::npos)d[name][token.substr(0,p)]=std::stod(token.substr(p+1));}}while(std::getline(f,row));return d;}return nullptr;
}
json conditions(){std::ifstream f("/proc/loadavg");double a=0,b=0,c=0;f>>a>>b>>c;return{{"cpu_psi",psi("cpu")},{"memory_psi",psi("memory")},{"load_average",{a,b,c}}};}
bool high_psi(const json&d){for(auto key:{"cpu_psi","memory_psi"})if(!d[key].is_null())for(auto&[name,row]:d[key].items())if(row.is_object()&&row.contains("avg10")&&row["avg10"].get<double>()>1.0)return true;return false;}
struct Reservation{
 std::vector<std::pair<int,cpu_set_t>>moved;std::vector<int>left;
 Reservation(int cpu){std::error_code ec;for(const auto&p:fs::directory_iterator("/proc",ec)){
  auto n=p.path().filename().string();if(n.empty()||!std::all_of(n.begin(),n.end(),::isdigit))continue;int pid=std::stoi(n);if(pid==getpid())continue;
  for(const auto&t:fs::directory_iterator(p.path()/"task",ec)){int tid=std::stoi(t.path().filename().string());cpu_set_t original;if(sched_getaffinity(tid,sizeof(original),&original)!=0||!CPU_ISSET(cpu,&original))continue;cpu_set_t next=original;CPU_CLR(cpu,&next);if(CPU_COUNT(&next)>0&&sched_setaffinity(tid,sizeof(next),&next)==0)moved.push_back({tid,original});else if(errno!=ESRCH)left.push_back(tid);}
 }}
 ~Reservation(){for(auto&[tid,mask]:moved)sched_setaffinity(tid,sizeof(mask),&mask);}
};
double seconds(timeval t){return double(t.tv_sec)+double(t.tv_usec)*1e-6;}
int main(int argc,char**argv){try{
 if(argc<4)throw std::runtime_error("usage: isolated_native MANIFEST OUTDIR CPU [--diagnostic]");
 bool diagnostic=argc>4&&std::string(argv[4])=="--diagnostic";int cpu=std::stoi(argv[3]);std::ifstream mf(argv[1]);json manifest;mf>>manifest;fs::create_directories(argv[2]);std::string out=argv[2];
 int lock=open("/tmp/crypto-bench.lock",O_CREAT|O_RDWR,0600);if(lock<0||flock(lock,LOCK_EX|LOCK_NB))throw std::runtime_error("exclusive benchmark lock unavailable");
 cpu_set_t allowed;if(sched_getaffinity(0,sizeof(allowed),&allowed)||!CPU_ISSET(cpu,&allowed))throw std::runtime_error("requested CPU not allowed");
 std::ifstream smt("/sys/devices/system/cpu/cpu"+std::to_string(cpu)+"/topology/thread_siblings_list");std::string siblings;std::getline(smt,siblings);if(siblings!=std::to_string(cpu))throw std::runtime_error("SMT sibling reservation is required");
 Reservation reservation(cpu);auto start=snapshot();auto t=Clock::now();std::this_thread::sleep_for(std::chrono::milliseconds(1000));auto finish=snapshot();double settle=std::chrono::duration<double>(Clock::now()-t).count();double pre_cpu=other_cpu(start,finish,{getpid()});json pre=conditions();bool pressure= !pre["cpu_psi"].is_null()&&!pre["memory_psi"].is_null();
 json record={{"schema","cubic-native-reservation/v1"},{"mode",diagnostic?"diagnostic":"strict"},{"cpu",cpu},{"smt_siblings",siblings},{"threads_moved",reservation.moved.size()},{"unmovable_threads_on_reserved_cpu",reservation.left},{"preflight",pre},{"preflight_other_cpu_seconds",pre_cpu},{"settle_seconds",settle},{"manifest",manifest},{"runs",json::array()}};
 auto save=[&](){std::ofstream f(out+"/reservation.json");f<<record.dump(2)<<'\n';};
 if(!pressure&&!diagnostic){record["status"]="blocked";record["reason"]="CPU and memory PSI are unavailable; strict isolation cannot be qualified";save();std::cerr<<record["reason"]<<'\n';return 4;}
 if(pre_cpu>0.05*settle||high_psi(pre)||(!diagnostic&&!reservation.left.empty())){record["status"]="blocked";record["reason"]="host is not quiet or reserved CPU has unmovable threads";save();std::cerr<<record["reason"]<<'\n';return 4;}
 record["reservation_complete"]=reservation.left.empty();
 if(diagnostic&&!reservation.left.empty())record["reservation_limit"]="Unmovable system threads remain eligible for the selected CPU; diagnostic pinning is not a complete core reservation";
 record["status"]="running";record["qualified_wall_time"]=pressure&&!diagnostic;save();int index=0;
 for(auto&job:manifest["jobs"]){
  std::vector<std::string>cmd=job["command"].get<std::vector<std::string>>();std::string stem=out+"/job-"+std::to_string(index);auto before=snapshot();auto cond=conditions();auto begin=Clock::now();pid_t child=fork();if(child<0)throw std::runtime_error("fork failed");
  if(child==0){cpu_set_t mask;CPU_ZERO(&mask);CPU_SET(cpu,&mask);if(sched_setaffinity(0,sizeof(mask),&mask))_exit(125);setenv("RAYON_NUM_THREADS","1",1);setenv("OMP_NUM_THREADS","1",1);int fd=open((stem+".json").c_str(),O_CREAT|O_EXCL|O_WRONLY,0600);int er=open((stem+".stderr.txt").c_str(),O_CREAT|O_EXCL|O_WRONLY,0600);if(fd<0||er<0)_exit(124);dup2(fd,STDOUT_FILENO);dup2(er,STDERR_FILENO);close(fd);close(er);rlimit limit{1800,1800};setrlimit(RLIMIT_CPU,&limit);std::vector<char*>args;for(auto&s:cmd)args.push_back(s.data());args.push_back(nullptr);execvp(args[0],args.data());_exit(127);}
  int status=0;rusage usage{};if(wait4(child,&status,0,&usage)<0)throw std::runtime_error("wait failed");double wall=std::chrono::duration<double>(Clock::now()-begin).count();auto after=snapshot();double others=other_cpu(before,after,{getpid(),child});bool contended=others>0.05*wall||high_psi(conditions());
  json r={{"index",index},{"job",job},{"exit_status",WIFEXITED(status)?WEXITSTATUS(status):128+WTERMSIG(status)},{"process_wall_seconds",wall},{"user_seconds",seconds(usage.ru_utime)},{"system_seconds",seconds(usage.ru_stime)},{"max_rss_kib",usage.ru_maxrss},{"voluntary_context_switches",usage.ru_nvcsw},{"involuntary_context_switches",usage.ru_nivcsw},{"minor_faults",usage.ru_minflt},{"major_faults",usage.ru_majflt},{"other_cpu_seconds",others},{"contended",contended},{"conditions_before",cond},{"conditions_after",conditions()},{"qualified_wall_time",pressure&&!diagnostic&&!contended},{"stdout","job-"+std::to_string(index)+".json"},{"stderr","job-"+std::to_string(index)+".stderr.txt"}};
  record["runs"].push_back(r);save();std::cout<<"job "<<index<<" exit "<<r["exit_status"]<<" contended "<<contended<<'\n'<<std::flush;index++;
 }
 record["status"]="complete";save();flock(lock,LOCK_UN);close(lock);return 0;
 }catch(const std::exception&e){std::cerr<<e.what()<<'\n';return 2;}}
