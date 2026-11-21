#include "json.hpp"
#include <sys/utsname.h>
#include <time.h>
#include <cstdlib>
#include <sched.h>
#include <sys/file.h>
#include <sys/resource.h>
#include <sys/wait.h>
#include <fcntl.h>
#include <unistd.h>
#include <signal.h>
#include <chrono>
#include <filesystem>
#include <fstream>
#include <iostream>
#include <map>
#include <set>
#include <sstream>
#include <string>
#include <thread>
#include <vector>
#include <cerrno>
#include <openssl/sha.h>
#include <iomanip>
static volatile sig_atomic_t interrupted=0,child_pid=0;
static void interrupted_signal(int){interrupted=1;if(child_pid>0)kill(child_pid,SIGTERM);}
namespace fs=std::filesystem;
struct Tick {uint64_t ticks=0;int parent=0;};
using Snapshot=std::map<int,Tick>;
static std::string read(const fs::path&p){std::ifstream f(p);std::ostringstream s;s<<f.rdbuf();return s.str();}
static std::string hash_file(const std::string&path){auto bytes=read(path);unsigned char hash[SHA256_DIGEST_LENGTH];SHA256(reinterpret_cast<const unsigned char*>(bytes.data()),bytes.size(),hash);std::ostringstream out;for(auto b:hash)out<<std::hex<<std::setw(2)<<std::setfill('0')<<int(b);return out.str();}
static std::string switches(){std::string out="{";bool comma=false;for(auto name:{"OPENSSL_CONF","OPENSSL_MODULES","OPENSSL_ia32cap","OMP_NUM_THREADS","RAYON_NUM_THREADS","LD_PRELOAD","LD_LIBRARY_PATH"}){if(comma)out+=",";comma=true;const char*v=std::getenv(name);out+=json_quote(name)+":"+(v?json_quote(v):"null");}return out+"}";}
static std::string host(){utsname u{};uname(&u);auto info=read("/proc/cpuinfo");std::string model="unknown";auto pos=info.find("model name");if(pos!=std::string::npos){auto colon=info.find(':',pos),end=info.find('\n',colon);model=info.substr(colon+1,end-colon-1);}timespec resolution{};clock_getres(CLOCK_MONOTONIC,&resolution);return "{\"cpu_model\":"+json_quote(model)+",\"kernel\":"+json_quote(u.release)+",\"machine\":"+json_quote(u.machine)+",\"logical_cpus\":"+std::to_string(sysconf(_SC_NPROCESSORS_ONLN))+",\"clock_resolution_ns\":"+std::to_string(resolution.tv_sec*1000000000LL+resolution.tv_nsec)+"}";}
static bool number(const std::string&s){return !s.empty()&&s.find_first_not_of("0123456789")==std::string::npos;}
static Snapshot snapshot(){Snapshot out;for(auto&e:fs::directory_iterator("/proc")){auto n=e.path().filename().string();if(!number(n))continue;auto s=read(e.path()/"stat");auto pos=s.rfind(')');if(pos==std::string::npos)continue;std::istringstream in(s.substr(pos+2));std::vector<std::string>v;std::string k;while(in>>k)v.push_back(k);if(v.size()>12)try{out[std::stoi(n)]={std::stoull(v[11])+std::stoull(v[12]),std::stoi(v[1])};}catch(...){} }return out;}
static double usage(const Snapshot&a,const Snapshot&b,const std::set<int>&exclude){uint64_t ticks=0;for(auto&[pid,v]:b)if(!exclude.count(pid)){uint64_t old=a.count(pid)?a.at(pid).ticks:0;if(v.ticks>old)ticks+=v.ticks-old;}return double(ticks)/sysconf(_SC_CLK_TCK);}
static double psi(const std::string&kind){std::istringstream in(read("/proc/pressure/"+kind));std::string row;while(std::getline(in,row))if(row.rfind("some ",0)==0){auto i=row.find("avg10=");if(i!=std::string::npos)return std::stod(row.substr(i+6));}return -1;}
static std::set<int> cpus(const std::string&s){std::set<int>v;std::istringstream in(s);std::string p;while(std::getline(in,p,',')){auto dash=p.find('-');int lo=std::stoi(p),hi=dash==std::string::npos?lo:std::stoi(p.substr(dash+1));if(lo<0||hi>=CPU_SETSIZE||hi<lo)throw std::runtime_error("CPU range");for(int i=lo;i<=hi;++i)v.insert(i);}return v;}
struct Restore {std::map<int,cpu_set_t>moved;~Restore(){for(auto&[tid,mask]:moved)sched_setaffinity(tid,sizeof(mask),&mask);}};
static int run(int argc,char**argv){
 if(argc<6||std::string(argv[4])!="--")throw std::runtime_error("usage: isolate CPUS RECEIPT ROWS -- COMMAND ...");
 if(fs::exists(argv[2])||fs::exists(argv[3]))throw std::runtime_error("write-once output already exists");
 struct sigaction action{};action.sa_handler=interrupted_signal;sigemptyset(&action.sa_mask);sigaction(SIGTERM,&action,nullptr);sigaction(SIGINT,&action,nullptr);#ifdef CM_ISOLATION_SELFTEST
 if(argc!=7||std::string(argv[5])!="/bin/sleep"||(std::string(argv[6])!="1"&&std::string(argv[6])!="30"))throw std::runtime_error("selftest permits only fixed sleep commands");constexpr bool selftest=true;
#else
 constexpr bool selftest=false;
#endif
 auto reserved=cpus(argv[1]);int self=getpid();cpu_set_t initial;if(sched_getaffinity(0,sizeof(initial),&initial))throw std::runtime_error("get affinity");cpu_set_t widened=initial;bool affinity_widened=false;for(int c:reserved)if(!CPU_ISSET(c,&initial)){CPU_SET(c,&widened);affinity_widened=true;}if(affinity_widened){if(sched_setaffinity(0,sizeof(widened),&widened)||sched_getaffinity(0,sizeof(initial),&initial))throw std::runtime_error("cannot widen inherited affinity");}for(int c:reserved){if(!CPU_ISSET(c,&initial))throw std::runtime_error("CPU outside current affinity");auto siblings=cpus(read("/sys/devices/system/cpu/cpu"+std::to_string(c)+"/topology/thread_siblings_list"));for(int s:siblings)if(!reserved.count(s))throw std::runtime_error("SMT sibling must also be reserved");}if(reserved.size()>=size_t(CPU_COUNT(&initial)))throw std::runtime_error("reservation leaves no CPU for other work");
 int lock=open("/tmp/crypto-bench.lock",O_CREAT|O_RDWR,0600);struct LockOwner{int fd;~LockOwner(){if(fd>=0)close(fd);}} lock_owner{lock};if(lock<0||flock(lock,LOCK_EX|LOCK_NB))throw std::runtime_error("benchmark lock busy");
 auto pre=snapshot();std::this_thread::sleep_for(std::chrono::seconds(2));auto after=snapshot();double pre_other=usage(pre,after,{self});double cpu_psi=psi("cpu"),memory_psi=psi("memory");bool quiet=pre_other<=0.2&&cpu_psi>=0&&cpu_psi<=5&&memory_psi>=0&&memory_psi<=5;
 if(!quiet&&!selftest){std::ofstream out(argv[2]);out<<"{\"schema\":\"isolated-bench/1\",\"native\":true,\"admitted\":false,\"reason\":\"busy preflight\",\"other_cpu_seconds\":"<<pre_other<<",\"psi_cpu_avg10\":"<<cpu_psi<<",\"psi_memory_avg10\":"<<memory_psi<<"}\n";return 3;}
 if(interrupted)throw std::runtime_error("interrupted before reservation");
 Restore restore;unsigned unmovable=0,kernel=0;
 for(auto&e:fs::directory_iterator("/proc")){auto name=e.path().filename().string();if(!number(name)||std::stoi(name)==self)continue;std::error_code error;for(auto&t:fs::directory_iterator(e.path()/"task",error)){if(error)break;auto tname=t.path().filename().string();if(!number(tname))continue;int tid=std::stoi(tname);cpu_set_t current,target;if(sched_getaffinity(tid,sizeof(current),&current))continue;target=current;bool overlap=false;for(int c:reserved)if(CPU_ISSET(c,&current)){CPU_CLR(c,&target);overlap=true;}if(!overlap)continue;if(CPU_COUNT(&target)&&sched_setaffinity(tid,sizeof(target),&target)==0)restore.moved[tid]=current;else if(read(e.path()/"cmdline").empty())++kernel;else ++unmovable;}}
 // Match legacy policy: record unmovable threads, enforce measured contention.
 cpu_set_t childmask;CPU_ZERO(&childmask);CPU_SET(*reserved.begin(),&childmask);if(interrupted)throw std::runtime_error("interrupted during reservation");auto before=snapshot();auto start=std::chrono::steady_clock::now();pid_t child=fork();if(child<0)throw std::runtime_error("fork");
 if(child==0){if(sched_setaffinity(0,sizeof(childmask),&childmask))_exit(125);std::string fd=std::to_string(lock);setenv("CM_NATIVE_ISOLATION_FD",fd.c_str(),1);int rows=open(argv[3],O_CREAT|O_EXCL|O_WRONLY,0600);if(rows<0)_exit(125);if(dup2(rows,STDOUT_FILENO)<0)_exit(125);close(rows);execvp(argv[5],argv+5);_exit(127);}
 child_pid=child;if(interrupted)kill(child,SIGTERM);int status=0;rusage ru{};while(wait4(child,&status,0,&ru)<0){if(errno==EINTR)continue;throw std::runtime_error("wait4");}child_pid=0;double wall=std::chrono::duration<double>(std::chrono::steady_clock::now()-start).count();double other=usage(before,snapshot(),{self,int(child)});bool contended=other>0.10*wall;int code=WIFEXITED(status)?WEXITSTATUS(status):128+WTERMSIG(status);
 std::ofstream out(argv[2]);out<<"{\"schema\":\"isolated-bench/1\",\"native\":true,\"admitted\":"<<(selftest?"false":"true")<<",\"selftest\":"<<(selftest?"true":"false")<<",\"cpus\":\""<<argv[1]<<"\",\"pinned_cpu\":"<<*reserved.begin()<<",\"affinity_widened\":"<<(affinity_widened?"true":"false")<<",\"threads_moved\":"<<restore.moved.size()<<",\"unmovable_user_threads\":"<<unmovable<<",\"unmovable_kernel_threads\":"<<kernel<<",\"preflight_other_cpu_seconds\":"<<pre_other<<",\"psi_cpu_avg10\":"<<cpu_psi<<",\"psi_memory_avg10\":"<<memory_psi<<",\"exit_status\":"<<code<<",\"rows_sha256\":\""<<hash_file(argv[3])<<"\",\"command_binary_sha256\":\""<<hash_file(argv[5])<<"\",\"isolation_binary_sha256\":\""<<hash_file(argv[0])<<"\",\"protocol_sha256\":"<<json_quote(std::getenv("CM_PROTOCOL_SHA")?std::getenv("CM_PROTOCOL_SHA"):"")<<",\"host\":"<<host()<<",\"result_switches\":"<<switches()<<",\"wall_seconds\":"<<wall<<",\"other_cpu_seconds\":"<<other<<",\"contended\":"<<(contended?"true":"false")<<",\"max_rss_kib\":"<<ru.ru_maxrss<<",\"voluntary_switches\":"<<ru.ru_nvcsw<<",\"involuntary_switches\":"<<ru.ru_nivcsw<<",\"minor_faults\":"<<ru.ru_minflt<<",\"major_faults\":"<<ru.ru_majflt<<",\"psi_cpu_after\":"<<psi("cpu")<<",\"psi_memory_after\":"<<psi("memory")<<"}\n";return code?code:((contended&&!selftest)?4:0);
}
int main(int argc,char**argv){try{return run(argc,argv);}catch(const std::exception&e){std::cerr<<"native isolation refused: "<<e.what()<<std::endl;return 2;}}
