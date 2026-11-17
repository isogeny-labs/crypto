// Exact fixed-degree Boolean Macaulay row reduction. Not F4 or closure.
#include <algorithm>
#include <chrono>
#include <cstdint>
#include <filesystem>
#include <fstream>
#include <iomanip>
#include <iostream>
#include <list>
#include <map>
#include <memory>
#include <openssl/evp.h>
#include <sstream>
#include <stdexcept>
#include <string>
#include <set>
#include <vector>
using U=uint64_t; using Row=std::vector<U>; namespace fs=std::filesystem;
static bool before(U a,U b){int da=__builtin_popcountll(a),db=__builtin_popcountll(b);return da!=db?da>db:a<b;}
static void require(bool v,const std::string& s){if(!v)throw std::runtime_error(s);}
static void put(std::ostream& o,U v){o.write(reinterpret_cast<const char*>(&v),8);require(bool(o),"write failed");}
static U get(std::istream& i){U v=0;i.read(reinterpret_cast<char*>(&v),8);require(bool(i),"truncated record");return v;}
static Row canonical(Row r){std::sort(r.begin(),r.end(),before);Row o;for(size_t i=0;i<r.size();){size_t j=i+1;while(j<r.size()&&r[j]==r[i])j++;if((j-i)%2)o.push_back(r[i]);i=j;}return o;}
static Row add(const Row&a,const Row&b){Row out;out.reserve(a.size()+b.size());std::set_symmetric_difference(a.begin(),a.end(),b.begin(),b.end(),std::back_inserter(out),before);return out;}
static void write_row(std::ostream&o,const Row&r){put(o,r.size());for(U x:r)put(o,x);}
static Row read_row(std::istream&i,U max_bytes=16777216){U n=get(i);require(n<=max_bytes/8,"row scratch limit");Row r(n);for(U&x:r)x=get(i);require(std::is_sorted(r.begin(),r.end(),before)&&std::adjacent_find(r.begin(),r.end())==r.end(),"noncanonical row");return r;}
static std::string sha256(const fs::path&p){std::ifstream f(p,std::ios::binary);require(bool(f),"hash input open");auto ctx=std::unique_ptr<EVP_MD_CTX,decltype(&EVP_MD_CTX_free)>(EVP_MD_CTX_new(),EVP_MD_CTX_free);require(ctx&&EVP_DigestInit_ex(ctx.get(),EVP_sha256(),nullptr)==1,"SHA init");char b[65536];while(f){f.read(b,sizeof b);EVP_DigestUpdate(ctx.get(),b,f.gcount());}unsigned char digest[32];unsigned len;EVP_DigestFinal_ex(ctx.get(),digest,&len);std::ostringstream s;for(unsigned i=0;i<len;i++)s<<std::hex<<std::setw(2)<<std::setfill('0')<<unsigned(digest[i]);return s.str();}
static void header(std::istream&i,U&n,U&d){char m[8];i.read(m,8);require(std::string(m,8)=="BMROW001","row magic");n=get(i);d=get(i);require(n<=64&&d<=8,"unsupported shape");}
static void generate(const fs::path&input,unsigned degree,const fs::path&out,U disk_cap){
 require(!fs::exists(out),"output exists");std::ifstream in(input);std::string magic;unsigned n,neq;in>>magic>>n>>neq;require(magic=="BM-ANF-1"&&n>0&&n<=64&&degree<=8,"ANF header");
 std::ofstream o(out,std::ios::binary);o.write("BMROW001",8);put(o,n);put(o,degree);U rows=0,products=0;
 for(unsigned eq=0;eq<neq;eq++){size_t len;require(bool(in>>len)&&len<=2000000,"ANF length");Row p(len);for(U&t:p){require(bool(in>>t),"ANF truncated");require(n==64||(t>>n)==0,"mask outside variables");}p=canonical(std::move(p));if(p.empty())continue;unsigned pd=0;for(U t:p)pd=std::max(pd,unsigned(__builtin_popcountll(t)));if(pd>degree)continue;
  auto emit=[&](U t){products++;Row r;for(U x:p)r.push_back(x|t);r=canonical(std::move(r));if(r.empty())return;require(U(o.tellp())+8+r.size()*8<=disk_cap,"generated-row disk budget");write_row(o,r);rows++;};
  auto visit=[&](auto&&self,unsigned start,unsigned remaining,U mask)->void{emit(mask);if(!remaining)return;for(unsigned bit=start;bit<n;bit++)self(self,bit+1,remaining-1,mask|(U(1)<<bit));};visit(visit,0,degree-pd,0);
 }
 std::string extra;require(!(in>>extra),"trailing ANF data");o.flush();std::cout<<"{\"generated_rows\":"<<rows<<",\"products\":"<<products<<",\"n_vars\":"<<n<<",\"D\":"<<degree<<",\"row_bytes\":"<<fs::file_size(out)<<"}\n";
}
struct Counts{U processed=0,zero=0,xors=0,reads=0,writes=0,hits=0,misses=0,cache_peak=0,max_row=0,swaps=0,feature_ns=0,policy_ns=0;};
struct Slot{U offset=0;};
struct Cached{Row row;std::list<U>::iterator position;};
struct DiskBasis{
 fs::path dir;std::fstream journal;std::map<U,Slot> index;std::map<U,Cached> cache;std::list<U> lru;U cache_limit,cache_used=0;Counts&c;
 static constexpr U metadata_cap=67108864,disk_cap=1073741824;
 DiskBasis(fs::path d,U b,Counts&cnt,bool resume,U tail):dir(d),cache_limit(b),c(cnt){
  if(resume){fs::resize_file(dir/"pivots.bin",tail);std::ifstream f(dir/"pivots.bin",std::ios::binary);while(U(f.tellg())<tail){U pos=f.tellg();Row r=read_row(f);require(!r.empty(),"zero pivot");index[r.front()]={pos};require(index.size()<=metadata_cap/128,"pivot metadata budget");}require(U(f.tellg())==tail,"checkpoint journal boundary");}
  journal.open(dir/"pivots.bin",std::ios::in|std::ios::out|std::ios::binary|(resume?std::ios::openmode(0):std::ios::trunc));require(bool(journal),"pivot journal open");
 }
 void remember(U lead,const Row&r){auto old=cache.find(lead);if(old!=cache.end()){cache_used-=old->second.row.size()*8;lru.erase(old->second.position);cache.erase(old);}U bytes=r.size()*8;if(bytes>cache_limit)return;while(cache_used+bytes>cache_limit&&!lru.empty()){U key=lru.back();cache_used-=cache.at(key).row.size()*8;cache.erase(key);lru.pop_back();}lru.push_front(lead);cache.emplace(lead,Cached{r,lru.begin()});cache_used+=bytes;c.cache_peak=std::max(c.cache_peak,cache_used);}
 Row load(U lead){auto hit=cache.find(lead);if(hit!=cache.end()){c.hits++;lru.splice(lru.begin(),lru,hit->second.position);return hit->second.row;}c.misses++;journal.clear();journal.seekg(index.at(lead).offset);Row r=read_row(journal);c.reads+=8+r.size()*8;remember(lead,r);return r;}
 void save(const Row&r){require(!r.empty(),"save zero pivot");journal.clear();journal.seekp(0,std::ios::end);U off=journal.tellp();require(off+8+r.size()*8<=disk_cap,"pivot journal disk budget");if(!index.count(r.front()))require(index.size()<metadata_cap/128,"pivot metadata budget");write_row(journal,r);index[r.front()]={off};c.writes+=8+r.size()*8;remember(r.front(),r);}
 U tail(){journal.flush();journal.clear();journal.seekp(0,std::ios::end);return U(journal.tellp());}
};
static void checkpoint(DiskBasis&basis,U offset,const std::string&hash,const std::string&policy,const Counts&c){
 U tail=basis.tail();std::ofstream f(basis.dir/"checkpoint.tmp");f<<"BM-CHECKPOINT-1 "<<hash<<" "<<policy<<" "<<basis.cache_limit<<" "<<offset<<" "<<tail<<"\n";
 f<<c.processed<<" "<<c.zero<<" "<<c.xors<<" "<<c.reads<<" "<<c.writes<<" "<<c.hits<<" "<<c.misses<<" "<<c.cache_peak<<" "<<c.max_row<<" "<<c.swaps<<" "<<c.feature_ns<<" "<<c.policy_ns<<"\n";f.close();require(bool(f),"checkpoint write");fs::rename(basis.dir/"checkpoint.tmp",basis.dir/"checkpoint.txt");
}
// Independent dense bit representation and elimination loops for the controls.
// The oracle does not call the candidate's sorted-vector XOR operation.
static std::vector<Row> dense_rref(const std::vector<Row>&rows){
 Row columns;for(const Row&r:rows)columns.insert(columns.end(),r.begin(),r.end());std::sort(columns.begin(),columns.end(),before);columns.erase(std::unique(columns.begin(),columns.end()),columns.end());
 U words=(columns.size()+63)/64;require(words==0||rows.size()<=268435456/(words*8),"dense reference 256MiB budget");std::map<U,size_t> location;for(size_t i=0;i<columns.size();i++)location[columns[i]]=i;
 std::vector<Row> matrix(rows.size(),Row(words));for(size_t i=0;i<rows.size();i++)for(U term:rows[i]){size_t c=location.at(term);matrix[i][c/64]^=U(1)<<(c%64);}
 size_t rank=0;for(size_t c=0;c<columns.size()&&rank<matrix.size();c++){size_t p=rank;while(p<matrix.size()&&!(matrix[p][c/64]&(U(1)<<(c%64))))p++;if(p==matrix.size())continue;std::swap(matrix[p],matrix[rank]);for(size_t j=0;j<matrix.size();j++)if(j!=rank&&(matrix[j][c/64]&(U(1)<<(c%64))))for(size_t w=0;w<words;w++)matrix[j][w]^=matrix[rank][w];rank++;}
 std::vector<Row> out;for(size_t i=0;i<rank;i++){Row row;for(size_t c=0;c<columns.size();c++)if(matrix[i][c/64]&(U(1)<<(c%64)))row.push_back(columns[c]);out.push_back(std::move(row));}return out;
}
static std::vector<Row> reference(const fs::path&input,U max_rows){std::ifstream f(input,std::ios::binary);U n,d;header(f,n,d);std::vector<Row> rows;while(f.peek()!=EOF){require(rows.size()<max_rows,"reference verification row limit");rows.push_back(read_row(f));}return dense_rref(rows);}
static int reduce(const fs::path&input,const fs::path&dir,U cache_bytes,const std::string&policy,U verify_limit,U wall_seconds,U stop_after,U checkpoint_every,bool resume){
 require(policy=="first"||policy=="min-weight","unknown legal policy");require(cache_bytes<=67108864,"cache cap");auto begin=std::chrono::steady_clock::now();std::string hash=sha256(input);Counts c;U offset=24,tail=0;
 if(resume){std::ifstream f(dir/"checkpoint.txt");std::string magic,h,p;U oldcache;require(bool(f>>magic>>h>>p>>oldcache>>offset>>tail)&&magic=="BM-CHECKPOINT-1"&&h==hash&&p==policy&&oldcache==cache_bytes,"checkpoint identity mismatch");require(bool(f>>c.processed>>c.zero>>c.xors>>c.reads>>c.writes>>c.hits>>c.misses>>c.cache_peak>>c.max_row>>c.swaps>>c.feature_ns>>c.policy_ns),"checkpoint counters");}else{require(!fs::exists(dir),"run directory already exists");fs::create_directories(dir);}
 DiskBasis basis(dir,cache_bytes,c,resume,tail);std::ifstream f(input,std::ios::binary);U n,d;header(f,n,d);f.seekg(offset);std::ofstream trace(dir/(resume?"trace-resume.jsonl":"trace.jsonl"));require(bool(trace),"trace open");
 bool paused=false,budget=false;
 while(f.peek()!=EOF){
  Row row=read_row(f);c.max_row=std::max(c.max_row,U(row.size()));
  while(!row.empty()){
   require(std::chrono::steady_clock::now()-begin<=std::chrono::seconds(wall_seconds),"wall budget inside row; resume previous checkpoint");
   U lead=row.front();auto p=basis.index.find(lead);if(p==basis.index.end()){basis.save(row);break;}
   Row pivot=basis.load(lead);auto feature_begin=std::chrono::steady_clock::now();U incoming_weight=row.size(),pivot_weight=pivot.size();auto feature_end=std::chrono::steady_clock::now();bool swap=policy=="min-weight"&&incoming_weight<pivot_weight;auto decision_end=std::chrono::steady_clock::now();c.feature_ns+=std::chrono::duration_cast<std::chrono::nanoseconds>(feature_end-feature_begin).count();c.policy_ns+=std::chrono::duration_cast<std::chrono::nanoseconds>(decision_end-feature_end).count();
   trace<<"{\"row\":"<<c.processed<<",\"leading_mask\":"<<lead<<",\"incoming_weight\":"<<incoming_weight<<",\"pivot_weight\":"<<pivot_weight<<",\"replace_pivot\":"<<(swap?"true":"false")<<"}\n";require(U(trace.tellp())<=67108864,"trace disk budget");
   if(swap){basis.save(row);std::swap(row,pivot);c.swaps++;}c.xors+=row.size()+pivot.size();row=add(row,pivot);c.max_row=std::max(c.max_row,U(row.size()));require(row.size()*8<=16777216,"row scratch limit after fill-in");
  }
  if(row.empty()){c.zero++;}c.processed++;offset=U(f.tellg());if(checkpoint_every&&c.processed%checkpoint_every==0)checkpoint(basis,offset,hash,policy,c);
  if(stop_after&&c.processed>=stop_after){paused=true;break;}
  if(std::chrono::steady_clock::now()-begin>std::chrono::seconds(wall_seconds)){budget=true;break;}
 }
 checkpoint(basis,offset,hash,policy,c);trace.flush();bool verified=false;U ref_rank=0;
 if(!paused&&!budget&&verify_limit){auto ref=reference(input,verify_limit);ref_rank=ref.size();require(ref_rank==basis.index.size(),"rank mismatch");std::vector<Row> disk_rows;for(auto&p:basis.index)disk_rows.push_back(basis.load(p.first));require(dense_rref(disk_rows)==ref,"independent dense RREF mismatch");verified=true;}
 double wall=std::chrono::duration<double>(std::chrono::steady_clock::now()-begin).count();
 std::ostringstream result;result<<std::setprecision(17)<<"{\"schema\":\"bounded-macaulay-result/v1\",\"status\":\""<<(paused?"checkpoint_paused":budget?"wall_budget":"completed")<<"\",\"input_sha256\":\""<<hash<<"\",\"n_vars\":"<<n<<",\"D\":"<<d<<",\"policy\":\""<<policy<<"\",\"processed_rows\":"<<c.processed<<",\"rank\":"<<basis.index.size()<<",\"zero_rows\":"<<c.zero<<",\"sparse_entries_visited_by_xor\":"<<c.xors<<",\"pivot_read_bytes\":"<<c.reads<<",\"pivot_write_bytes\":"<<c.writes<<",\"cache_hits\":"<<c.hits<<",\"cache_misses\":"<<c.misses<<",\"cache_payload_peak_bytes\":"<<c.cache_peak<<",\"cache_limit_bytes\":"<<cache_bytes<<",\"max_row_terms\":"<<c.max_row<<",\"pivot_replacements\":"<<c.swaps<<",\"feature_ns\":"<<c.feature_ns<<",\"policy_decision_ns\":"<<c.policy_ns<<",\"elapsed_seconds_shared_host_including_verification\":"<<wall<<",\"exact_mutual_row_space_verified\":"<<(verified?"true":"false")<<",\"reference_rank\":"<<ref_rank<<",\"resumed\":"<<(resume?"true":"false")<<"}";
 std::ofstream out(dir/(resume?"result-resume.json":"result.json"));out<<result.str()<<"\n";std::cout<<result.str()<<"\n";return paused?75:budget?124:0;
}
static void selftest(){
 U seed=9173;auto rng=[&](){seed^=seed<<13;seed^=seed>>7;seed^=seed<<17;return seed;};
 for(unsigned test=0;test<100;test++){Row a,b;std::set<U> a_set,b_set;for(unsigned i=0;i<32;i++){if(rng()&1)a_set.insert(rng()%64);if(rng()&1)b_set.insert(rng()%64);}a.assign(a_set.begin(),a_set.end());b.assign(b_set.begin(),b_set.end());a=canonical(a);b=canonical(b);Row result=add(a,b);for(U t:a_set){bool expected=b_set.count(t)==0;require((std::find(result.begin(),result.end(),t)!=result.end())==expected,"XOR A support");}for(U t:b_set){bool expected=a_set.count(t)==0;require((std::find(result.begin(),result.end(),t)!=result.end())==expected,"XOR B support");}require(add(result,b)==a,"XOR roundtrip");auto r1=dense_rref({a,b,result}),r2=dense_rref({a,b});require(r1==r2,"dense oracle sum invariance");}
 require(canonical({7,7,1,1,0})==Row({0}),"parity normalization");require(dense_rref({Row{3,1},Row{3,1}}).size()==1,"dense duplicate rank");std::cout<<"{\"selftest_cases\":100,\"passed\":true}\n";
}
int main(int argc,char**argv){try{
 require(argc>1,"generate ANF D OUT DISK_CAP | reduce ROWS DIR CACHE POLICY VERIFY_ROWS WALL STOP_AFTER CHECKPOINT_EVERY [resume]");std::string op=argv[1];
 if(op=="generate"){require(argc==6,"generate args");generate(argv[2],std::stoul(argv[3]),argv[4],std::stoull(argv[5]));return 0;}
 if(op=="selftest"){require(argc==2,"selftest args");selftest();return 0;}
 if(op=="reduce"){require(argc==10||argc==11,"reduce args");require(argc!=11||std::string(argv[10])=="resume","unknown restart flag");return reduce(argv[2],argv[3],std::stoull(argv[4]),argv[5],std::stoull(argv[6]),std::stoull(argv[7]),std::stoull(argv[8]),std::stoull(argv[9]),argc==11);}
 throw std::runtime_error("unknown command");
}catch(const std::exception&e){std::cerr<<"{\"status\":\"failed_closed\",\"reason\":\""<<e.what()<<"\"}\n";return 2;}}
