// Independent group-enumeration reference. Geometry execution requires checkpoint.
#include <nlohmann/json.hpp>
#include <openssl/sha.h>
#include <array>
#include <algorithm>
#include <fstream>
#include <iomanip>
#include <iostream>
#include <map>
#include <optional>
#include <set>
#include <sstream>
#include <stdexcept>
#include <vector>
#include <fcntl.h>
#include <unistd.h>
using J=nlohmann::json;
void need(bool b,const char* m){if(!b)throw std::runtime_error(m);}
int mod(int a){a%=37;return a<0?a+37:a;}
struct F {
 std::array<int,2> c;
 F(int a=0,int b=0):c{mod(a),mod(b)}{}
 bool operator==(const F& b)const{return c==b.c;}
 bool operator<(const F& b)const{return c<b.c;}
 F operator+(F b)const{return {c[0]+b.c[0],c[1]+b.c[1]};}
 F operator-()const{return {-c[0],-c[1]};}
 F operator-(F b)const{return *this+-b;}
 F operator*(F b)const{return {c[0]*b.c[0]+35*c[1]*b.c[1],c[0]*b.c[1]+c[1]*b.c[0]};}
 F pow(int n)const{F r(1),b=*this;for(;n;n/=2,b=b*b)if(n%2)r=r*b;return r;}
 F operator/(F b)const{need(!(b==F()),"zero denominator");return *this*b.pow(1367);}
 F frob()const{return pow(37);}
};
struct P {F x,y;bool operator==(const P& b)const{return x==b.x&&y==b.y;}};
using O=std::optional<P>;
O minus(O p){return p?O(P{p->x,-p->y}):O();}
O frob(O p){return p?O(P{p->x.frob(),p->y.frob()}):O();}
bool curve(O p,int a=7,int b=19){return !p||p->y*p->y==p->x.pow(3)+F(a)*p->x+F(b);}
O add(O p,O q){
 if(!p)return q;
 if(!q)return p;
 if(p->x==q->x&&p->y+q->y==F())return {};
 F l;
 if(p->x==q->x){need(p->y==q->y,"invalid equal abscissa");l=(F(3)*p->x*p->x+F(7))/(F(2)*p->y);}
 else l=(p->y-q->y)/(p->x-q->x);
 F x=l*l-p->x-q->x;return P{x,l*(p->x-x)-p->y};
}
O mul(O p,int n){O r;for(int i=0;i<n;i++)r=add(r,p);return r;}
J point(O p){return p?J::array({p->x.c[0],p->x.c[1],p->y.c[0],p->y.c[1]}):J(nullptr);}
J target(O p){return p?J::array({p->x.c[0],p->y.c[0]}):J(nullptr);}
J inverse(O p){
 if(!p)return nullptr;
 F x=F(35)*p->x,y=F(0,35)*p->y;
 need(x.c[1]==0&&y.c[1]==0,"inverse not rational");
 need(curve(P{x,y},28,33),"inverse twist equation");return J::array({x.c[0],y.c[0]});
}
std::string bytes(const std::string& path){std::ifstream f(path,std::ios::binary);need(bool(f),"file read");return {std::istreambuf_iterator<char>(f),{}};}
std::string sha(const std::string& s){unsigned char h[SHA256_DIGEST_LENGTH];SHA256((const unsigned char*)s.data(),s.size(),h);std::ostringstream o;for(auto b:h)o<<std::hex<<std::setw(2)<<std::setfill('0')<<int(b);return o.str();}
J parse(const std::string& s){
 std::vector<std::set<std::string>> keys;
 auto cb=[&](int,J::parse_event_t e,J& v){
  if(e==J::parse_event_t::object_start)keys.emplace_back();
  if(e==J::parse_event_t::key)need(keys.back().insert(v.get<std::string>()).second,"duplicate JSON key");
  if(e==J::parse_event_t::object_end)keys.pop_back();
  if(e==J::parse_event_t::value)need(!v.is_number_float(),"floating point encoding forbidden");
  return true;
 };return J::parse(s,cb);
}
std::vector<O> universe(){
 std::map<F,std::vector<F>> roots;
 for(int i=0;i<37;i++)for(int j=0;j<37;j++){F y(i,j);roots[y*y].push_back(y);}
 std::vector<O> all{O()};
 for(int i=0;i<37;i++)for(int j=0;j<37;j++){
  F x(i,j),rhs=x.pow(3)+F(7)*x+F(19);
  auto it=roots.find(rhs);if(it!=roots.end())for(F y:it->second)all.push_back(P{x,y});
 }return all;
}
J fiber(O q,const std::vector<O>& all){
 std::vector<J> rows;std::vector<bool> bits(1369,false);
 for(O r:all){
  if(add(r,frob(r))!=q)continue;
  O c=add(r,minus(mul(q,10)));need(frob(c)==minus(c),"anti-invariant residual");
  J params=nullptr;std::string branch;
  if(!q){
   if(!r)branch="infinity";
   else {branch="identity";params=inverse(r);}
  }else{
   need(bool(r),"infinite nonidentity fiber point");O z=frob(r);F l;
   if(!(r->x==z->x))l=(r->y-z->y)/(r->x-z->x);
   else {need(r==z,"vertical nonidentity fiber");l=(F(3)*r->x*r->x+F(7))/(F(2)*r->y);}
   need(l.c[1]==0,"slope not rational");F s=l*l-q->x,t=(F(2)*r->x-s)/F(0,1);
   need(t.c[1]==0,"parameter not rational");params=J::array({l.c[0],t.c[0]});branch=t==F()?"tangent":"secant";
  }
  if(!params.is_null()){int k=params[0].get<int>()*37+params[1].get<int>();need(!bits[k],"duplicate parameter");bits[k]=true;}
  rows.push_back(J{{"branch",branch},{"parameters",params},{"point",point(r)},{"twist",inverse(c)}});
 }
 std::sort(rows.begin(),rows.end(),[](const J&a,const J&b){
  if(a["parameters"].is_null())return !b["parameters"].is_null();
  if(b["parameters"].is_null())return false;
  return std::array<int,2>{a["parameters"][0],a["parameters"][1]}<std::array<int,2>{b["parameters"][0],b["parameters"][1]};
 });
 return J{{"query",target(q)},{"pair_outcomes",bits},{"records",rows}};
}
int main(int argc,char**argv){try{
 if(argc==2&&std::string(argv[1])=="--help"){std::cout<<"audit REPO MANIFEST MANIFEST_SHA256 fixed|full DATASET NEW_RECEIPT\n";return 0;}
 if(argc!=8||std::string(argv[1])!="audit")return 2;
 std::string repo=argv[2],manifest_bytes=bytes(argv[3]);need(sha(manifest_bytes)==argv[4],"manifest hash");J m=parse(manifest_bytes);
 const std::string base="research/quadratic_twist_chord_proof_20261105/native-preflight-20261113/";
 J expected_m={{"schema","twist.trace-fiber-inputs/v1"}};
 for(auto kv:std::vector<std::pair<std::string,std::string>>{{"producer","producer/src/main.rs"},{"auditor","auditor.cpp"},{"contract","CONTRACT.json"}}){
  std::string path=base+kv.second;expected_m[kv.first]={{"path",path},{"sha256",sha(bytes(repo+"/"+path))}};
 }need(m==expected_m,"source manifest binding");
 std::string raw=bytes(argv[6]),side=argv[6];auto dot=side.find_last_of('.');side=(dot==std::string::npos?side:side.substr(0,dot))+".sha256";
 need(bytes(side)==sha(raw)+"\n","dataset sidecar");J data=parse(raw);std::string scope=argv[5];need(scope=="fixed"||scope=="full","scope");need(data.at("scope")==scope,"expected scope");
 O p=P{F(1),F(8)};need(curve(p)&&!mul(p,19),"generator order");std::vector<O> qs;
 for(int i=1;i<19;i++)qs.push_back(mul(p,i));
 std::sort(qs.begin(),qs.end(),[](O a,O b){return std::array<int,2>{a->x.c[0],a->y.c[0]}<std::array<int,2>{b->x.c[0],b->y.c[0]};});
 need(std::adjacent_find(qs.begin(),qs.end())==qs.end(),"duplicate subgroup point");
 if(scope=="fixed")qs={p};else qs.insert(qs.begin(),O());
 auto all=universe();J cases=J::array();size_t count=0;
 for(O q:qs){J c=fiber(q,all);count+=c["records"].size();cases.push_back(c);}
 J fixture={{"p",37},{"a",7},{"b",19},{"d",35},{"twist_a",28},{"twist_b",33},{"r",19},{"generator",{1,8}},{"half_scalar",10}};
 J expected={{"schema","twist.trace-fiber-geometry/v1"},{"fixture",fixture},{"scope",scope},{"sources",m},{"queries",cases}};
 need(data==expected,"independent geometry mismatch");
 J receipt={{"status","geometry_replayed"},{"dataset_sha256",sha(raw)},{"queries",qs.size()},{"fiber_points",count},{"extension_curve_points",all.size()},{"scalar_recoveries",0},{"relation_solver_executed",false},{"sources",m}};
 std::string output=receipt.dump(2)+"\n";int fd=open(argv[7],O_WRONLY|O_CREAT|O_EXCL,0600);need(fd>=0,"fresh receipt creation");
 size_t offset=0;while(offset<output.size()){auto n=write(fd,output.data()+offset,output.size()-offset);if(n<=0){close(fd);unlink(argv[7]);throw std::runtime_error("receipt write");}offset+=size_t(n);}
 if(close(fd)!=0){unlink(argv[7]);throw std::runtime_error("receipt close");}return 0;
 }catch(const std::exception&e){std::cerr<<e.what()<<"\n";return 101;}}
