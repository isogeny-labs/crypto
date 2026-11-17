#include <chrono>
// NEW F61 finite twist-assisted pairing/source-scalar reduction preflight; no timing/IC claim.
#include <array>
#include <vector>
#include <map>
#include <set>
#include <algorithm>
#include <optional>
#include <iostream>
#include <fstream>
#include <sstream>
#include <stdexcept>
bool BASE_POLY=false;
struct BaseScope{bool old;BaseScope(bool b):old(BASE_POLY){BASE_POLY=b;}~BaseScope(){BASE_POLY=old;}};
int mod(int x){x%=61;return x<0?x+61:x;}int power(int x,int n){int r=1;for(;n;n>>=1,x=mod(x*x))if(n&1)r=mod(r*x);return r;}
void need(bool b,const char*s){if(!b)throw std::runtime_error(s);}
struct F{int a,b;F(int x=0,int y=0):a(mod(x)),b(mod(y)){}auto operator<=>(const F&)const=default;};
F operator+(F x,F y){if(BASE_POLY){need(x.b==0&&y.b==0,"base add domain");return F(x.a+y.a);}return{ x.a+y.a,x.b+y.b};}F operator-(F x,F y){if(BASE_POLY){need(x.b==0&&y.b==0,"base subtract domain");return F(x.a-y.a);}return{x.a-y.a,x.b-y.b};}F operator-(F x){return{-x.a,-x.b};}F operator*(F x,F y){if(BASE_POLY){need(x.b==0&&y.b==0,"base multiply domain");return F(x.a*y.a);}return{x.a*y.a+59*x.b*y.b,x.a*y.b+x.b*y.a};}
F inverse(F x){if(BASE_POLY){need(x.b==0&&x.a!=0,"base inverse domain");return F(power(x.a,59));}int n=mod(x.a*x.a-59*x.b*x.b);need(n!=0,"zero field inverse");int v=power(n,59);return{x.a*v,-x.b*v};}F operator/(F x,F y){return x*inverse(y);}F pow(F x,int n){F r(1);for(;n;n>>=1,x=x*x)if(n&1)r=r*x;return r;}int encode(F x){return x.a+61*x.b;}
struct P{F x,y;bool o=true;auto operator<=>(const P&)const=default;};P neg(P p){if(!p.o)p.y=-p.y;return p;}
bool on(P p,int a,int b){return p.o||p.y*p.y==p.x*p.x*p.x+F(a)*p.x+F(b);}
P add(P p,P q,int a=21){if(p.o)return q;if(q.o)return p;if(p.x==q.x&&p.y+q.y==F())return{};F l=p.x==q.x?(F(3)*p.x*p.x+F(a))/(F(2)*p.y):(q.y-p.y)/(q.x-p.x);F x=l*l-p.x-q.x;return{x,l*(p.x-x)-p.y,false};}
P mul(P p,int n,int a=21){P r;for(;n;n>>=1,p=add(p,p,a))if(n&1)r=add(r,p,a);return r;}
P chi(P p){return p.o?p:P{p.x/F(59),p.y/F(0,59),false};}
std::optional<F> line(P p,P q,P at){if(p.o||q.o||at.o)return{};if(p.x==q.x&&p.y+q.y==F()){F z=at.x-p.x;return z==F()?std::nullopt:std::optional<F>(z);}F den=p.x==q.x?F(2)*p.y:q.x-p.x;if(den==F())return{};F l=p.x==q.x?(F(3)*p.x*p.x+F(21))/den:(q.y-p.y)/den;F num=at.y-p.y-l*(at.x-p.x);P sum=add(p,q);F v=sum.o?F(1):at.x-sum.x;if(num==F()||v==F())return{};return num/v;}
std::optional<F> miller(P p,P at){if(p.o||at.o)return{};F f(1);P v=p;for(int bit=8;bit;bit>>=1){auto z=line(v,v,at);if(!z)return{};f=f*f*(*z);v=add(v,v);if(31&bit){z=line(v,p,at);if(!z)return{};f=f*(*z);v=add(v,p);}}need(v.o,"wrong31 Miller endpoint");return f;}
std::optional<F> weil(P p,P q){if(p.o||q.o)return F(1);auto a=miller(p,q),b=miller(q,p);if(!a||!b)return{};return -(*a/ *b);}
std::optional<F> shifted(P p,P q,P s){if(p.o||q.o)return F(1);auto a=miller(p,add(q,s)),b=miller(p,s),c=miller(q,neg(s)),d=miller(q,add(p,neg(s)));if(!a||!b||!c||!d)return{};return (*a/ *b)*(*c/ *d);}
std::vector<P> group(P g,int a,int b){std::vector<P>v{P{}};P p;for(int n=1;n<31;n++){p=add(p,g,a);need(on(p,a,b)&&!p.o,"bad subgroup cycle");v.push_back(p);}need(add(p,g,a).o,"wrong order31");std::sort(v.begin()+1,v.end());need(std::set<P>(v.begin(),v.end()).size()==31,"duplicate subgroup point");return v;}
int field_bsgs(F g,F z){need(g!=F(1)&&pow(g,31)==F(1)&&pow(z,31)==F(1),"field subgroup");std::map<F,int>baby;F p(1);for(int j=0;j<6;j++){baby[p]=j;p=p*g;}F step=pow(g,25),at=z;for(int i=0;i<6;i++){auto it=baby.find(at);if(it!=baby.end()){int k=6*i+it->second;if(k<31&&pow(g,k)==z)return k;}at=at*step;}throw std::runtime_error("field BSGS exhausted");}
int recover(P unknown,P generator,P partner,F field_generator){auto z=weil(unknown,partner);need(z.has_value(),"unknown point pairing undefined");int k=field_bsgs(field_generator,*z);need(mul(generator,k)==unknown,"ORIGINAL source scalar check");return k;}
void point(P p){std::cout<<(p.o?-1:encode(p.x))<<' '<<(p.o?-1:encode(p.y));}

int ROUTE=0;bool RECORD=true;
F numerator(P x,P y,P at){need(!x.o&&!y.o&&!at.o,"numerator domain");if(x.x==y.x&&x.y+y.y==F())return F(1);F den=x.x==y.x?F(2)*x.y:y.x-x.x;need(den!=F(),"slope denominator");F m=x.x==y.x?(F(3)*x.x*x.x+F(21))/den:(y.y-x.y)/den;F n=at.y-x.y-m*(at.x-x.x);need(n!=F(),"numerator zero");return n;}
F fast_tate(P q,P at){if(q.o)return F(1);need(!at.o,"nonzero partner");P v=q;F f(1);for(int bit=8;bit;bit>>=1){f=f*f*numerator(v,v,at);v=add(v,v);if(31&bit){f=f*numerator(v,q,at);v=add(v,q);}}need(v.o,"Miller endpoint");return pow(F(f.a,-f.b)/f,2);}
std::optional<F> pairing(P q,P s){return ROUTE<2?weil(q,s):std::optional<F>(fast_tate(q,s));}
using Poly=std::vector<F>;
void trim(Poly&a){while(!a.empty()&&a.back()==F())a.pop_back();}
int deg(const Poly&a){return int(a.size())-1;}
Poly scale(Poly a,F b){for(auto&x:a)x=x*b;trim(a);return a;}
Poly sub(Poly a,const Poly&b){a.resize(std::max(a.size(),b.size()));for(size_t i=0;i<b.size();i++)a[i]=a[i]-b[i];trim(a);return a;}
struct Unknown{};
int peak;
void bound(int n,int cap){if(n>cap)throw Unknown{};peak=std::max(peak,n);}
Poly product(const Poly&a,const Poly&b,int cap,int weight=0){if(a.empty()||b.empty())return {};bound(deg(a)+deg(b)+weight,cap);Poly c(a.size()+b.size()-1);for(size_t i=0;i<a.size();i++)for(size_t j=0;j<b.size();j++)c[i+j]=c[i+j]+a[i]*b[j];trim(c);return c;}
std::pair<Poly,Poly> divide(Poly a,const Poly&b,int cap){need(!b.empty(),"zero divisor");Poly q;if(deg(a)>=deg(b))q.resize(deg(a)-deg(b)+1);while(!a.empty()&&deg(a)>=deg(b)){int k=deg(a)-deg(b);F v=a.back()/b.back();q[k]=q[k]+v;Poly mono(k+1);mono[k]=v;a=sub(a,product(mono,b,cap));}trim(q);return{q,a};}
bool terminal_stop=false;
struct Cert{Poly f,h,d,u,v;};
Cert certify(const std::vector<F>&a,const std::vector<F>&b,F z,int cap){need(pow(z,31)==F(1)&&z.a!=60,"query chart domain");BaseScope mode(ROUTE==4);bound(3,cap);Poly f{F(1)},h{F(1)};
if(ROUTE==4){
  for(F x:a){need(x.a!=60&&mod(x.a*x.a-59*x.b*x.b)==1,"public chart domain");f=product(f,Poly{F(-x.b),F(x.a+1)},cap);}
  for(F x:b){need(x.a!=60&&mod(x.a*x.a-59*x.b*x.b)==1,"second chart domain");h=product(h,Poly{F((x.a+1)*z.b-x.b*(z.a+1)),F(-(x.a+1)*(z.a+1)+59*x.b*z.b)},cap);}
}else{for(auto x:a)f=product(f,Poly{F()-x,F(1)},cap);for(auto x:b)h=product(h,Poly{z,F()-x},cap);}
Poly r0=f,r1=h,u0{F(1)},u1,v0,v1{F(1)};while(!r1.empty()){auto[q,r]=divide(r0,r1,cap);if(terminal_stop&&r.empty()){r0=r1;u0=u1;v0=v1;break;}Poly u=sub(u0,product(q,u1,cap,3)),v=sub(v0,product(q,v1,cap,3));r0=r1;r1=r;u0=u1;u1=u;v0=v1;v1=v;}F k=F(1)/r0.back();r0=scale(r0,k);u0=scale(u0,k);v0=scale(v0,k);need(sub(sub(product(u0,f,cap),scale(product(v0,h,cap),F()-F(1))),r0).empty(),"Bezout identity");need(divide(f,r0,cap).second.empty()&&divide(h,r0,cap).second.empty(),"gcd divisibility");return{f,h,r0,u0,v0};}
bool root(const Poly&a,F x){if(ROUTE!=4)return false;BaseScope mode(true);F value,den(1),num(x.b),d(x.a+1);for(size_t i=a.size();i-->0;){value=value*num+a[i]*den;den=den*d;}return value==F();}
F eval(const Poly&a,F x){F z;for(size_t i=a.size();i-->0;)z=z*x+a[i];return z;}
std::string poly(const Poly&a){std::ostringstream o;o<<a.size();for(F x:a)o<<','<<encode(x);return o.str();}
using Row=std::array<int,3>;
int rel(int n){n%=31;return n<0?n+31:n;}int rinv(int n){for(int k=1;k<31;k++)if(rel(k*n)==1)return k;throw std::runtime_error("zero relation inverse");}
struct Matrix{std::map<int,std::array<int,4>>a;void insert(Row v,int m){std::array<int,4>r{rel(v[0]),rel(v[1]),rel(v[2]),rel(m)};for(auto&[p,b]:a){int c=r[p];for(int j=0;j<4;j++)r[j]=rel(r[j]-c*b[j]);}int p=0;while(p<3&&!r[p])p++;if(p==3){need(!r[3],"inconsistent setup");return;}int k=rinv(r[p]);for(int&x:r)x=rel(x*k);for(auto&[q,b]:a){int c=b[p];for(int j=0;j<4;j++)b[j]=rel(b[j]-c*r[j]);}a[p]=r;}std::array<int,3>logs(){need(a.size()==3,"incomplete setup");return{a.at(0)[3],a.at(1)[3],a.at(2)[3]};}};
struct Counts{int base_pairs=0,query_pairs=0,branches=0,unknown=0,certified=0,root_checks=0,rows=0,setup_queries=0,online_queries=0,peak=0;};
std::vector<Row> query(P q,P partner,const std::vector<P>&base,const std::vector<F>&images,int cap,Counts&count,std::ostringstream&trace,std::optional<F> supplied={}){std::vector<Row>rows;F z;if(cap>=3){auto v=supplied?supplied:pairing(q,partner);need(v.has_value(),"actual query pairing");z=*v;if(!supplied)count.query_pairs++;}if(RECORD)trace<<"QUERY "<<(q.o?-1:encode(q.x))<<' '<<(q.o?-1:encode(q.y));if(RECORD)trace<<' '<<encode(z)<<'\n';for(int sign:{-1,1})for(int sign2:{-1,1}){count.branches++;peak=0;if(RECORD)trace<<"BR "<<sign<<' '<<sign2<<' ';try{bound(3,cap);std::vector<F>a,b;for(F x:images){a.push_back(sign==1?x:(ROUTE>=4?F(x.a,-x.b):inverse(x)));b.push_back(sign2==1?x:(ROUTE>=4?F(x.a,-x.b):inverse(x)));}Cert c=certify(a,b,z,cap);std::vector<std::pair<int,int>>found;for(int i=0;i<3;i++){count.root_checks++;if(ROUTE==4?root(c.d,a[i]):eval(c.d,a[i])==F())for(int j=0;j<3;j++)if(a[i]*b[j]==z){need(add(sign==1?base[i]:neg(base[i]),sign2==1?base[j]:neg(base[j]))==q,"actual source row");Row row{};row[i]+=sign;row[j]+=sign2;rows.push_back(row);found.push_back({i,j});}}if(RECORD)trace<<"C "<<peak<<' '<<poly(c.f)<<' '<<poly(c.h)<<' '<<poly(c.d)<<' '<<poly(c.u)<<' '<<poly(c.v)<<' '<<found.size();for(auto[i,j]:found)if(RECORD)trace<<' '<<i<<','<<j;if(RECORD)trace<<'\n';count.certified++;count.rows+=found.size();}catch(const Unknown&){if(RECORD)trace<<"U "<<peak<<'\n';count.unknown++;}count.peak=std::max(count.peak,peak);}return rows;}
void emit(P p,std::ostringstream&t){t<<(p.o?-1:encode(p.x))<<' '<<(p.o?-1:encode(p.y));}

using Clock=std::chrono::steady_clock;long long ns(Clock::time_point a,Clock::time_point b){return std::chrono::duration_cast<std::chrono::nanoseconds>(b-a).count();}
struct Recovery{int scalar=-1,kind=0,rank=0,setup_m=0,shift=-1,progress_mults=0;std::array<int,3>logs{-1,-1,-1};Counts count;std::string trace;std::array<long long,5>times{};};
Recovery recover_cold(P unknown,P g,P unused,const std::vector<P>&base,int cap){auto t0=Clock::now();need(on(g,21,39)&&!g.o&&mul(g,31).o&&on(unknown,21,39)&&mul(unknown,31).o,"source domain");for(P b:base)need(on(b,21,39)&&!b.o&&mul(b,31).o,"public domain");auto t1=Clock::now();Recovery out;std::ostringstream trace;P u{F(37),F(14),false},partner=chi(u);need(on(u,23,54)&&on(partner,21,39)&&!partner.o&&mul(partner,31).o&&P{pow(partner.x,61),pow(partner.y,61),false}==neg(partner),"actual twist partner");std::vector<F>images;for(P b:base){auto v=pairing(b,partner);need(v&&*v!=F(1),"public base image");images.push_back(*v);out.count.base_pairs++;}
F zeta(1),public_image(1);if(ROUTE>=3){zeta=*pairing(g,partner);out.count.query_pairs++;need(zeta!=F(1)&&pow(zeta,31)==F(1),"primitive map");}Matrix matrix;P setup;
for(int m=1;m<=30&&matrix.a.size()<3;m++){setup=add(setup,g);out.count.setup_queries++;if(RECORD)trace<<"PUBLIC "<<m<<'\n';std::optional<F>provided;if(ROUTE>=3){public_image=public_image*zeta;out.progress_mults++;provided=public_image;}auto rows=query(setup,partner,base,images,cap,out.count,trace,provided);for(auto row:rows)matrix.insert(row,m);out.setup_m=m;}
out.rank=matrix.a.size();if(out.rank==3){out.logs=matrix.logs();for(int i=0;i<3;i++)need(mul(g,out.logs[i])==base[i],"admitted PUBLIC log check");}auto t2=Clock::now();F target_image(1);if(out.rank==3&&ROUTE>=3){target_image=*pairing(unknown,partner);out.count.query_pairs++;}auto t3=Clock::now();
if(out.rank==3){P shifted=unknown;for(int shift=0;shift<=30;shift++){out.count.online_queries++;if(RECORD)trace<<"ONLINE "<<shift<<'\n';if(shifted.o){out.scalar=rel(-shift);out.kind=2;out.shift=shift;if(RECORD)trace<<"IDENTITY\n";break;}auto rows=query(shifted,partner,base,images,cap,out.count,trace,ROUTE>=3?std::optional<F>(target_image):std::nullopt);if(!rows.empty()){int k=-shift;for(int i=0;i<3;i++)k+=rows[0][i]*out.logs[i];out.scalar=rel(k);out.kind=1;out.shift=shift;break;}shifted=add(shifted,g);if(ROUTE>=3){target_image=target_image*zeta;out.progress_mults++;}}}auto t4=Clock::now();if(out.rank==3)need(out.scalar>=0&&mul(g,out.scalar)==unknown,"ORIGINAL scalar");out.trace=trace.str();images.clear();images.shrink_to_fit();matrix.a.clear();std::ostringstream().swap(trace);auto t5=Clock::now();out.times={ns(t0,t1),ns(t1,t2),ns(t2,t3),ns(t3,t4),ns(t4,t5)};return out;}
#include <zlib.h>
#include <nlohmann/json.hpp>
#include <openssl/sha.h>
#include <filesystem>
#include <iomanip>
using J=nlohmann::ordered_json;namespace fs=std::filesystem;
std::string read(const fs::path&p){std::ifstream f(p,std::ios::binary);need(bool(f),"read file");return {std::istreambuf_iterator<char>(f),{}};}
void write(const fs::path&p,const std::string&s){need(!fs::exists(p),"fresh output");std::ofstream f(p,std::ios::binary);need(bool(f),"write output");f<<s;need(bool(f),"write complete");}
std::string digest(const std::string&s){unsigned char h[SHA256_DIGEST_LENGTH];SHA256((const unsigned char*)s.data(),s.size(),h);std::ostringstream o;for(auto x:h)o<<std::hex<<std::setw(2)<<std::setfill('0')<<(int)x;return o.str();}

J result(const Recovery&r){auto c=r.count;return J{{"scalar",r.scalar},{"kind",r.kind},{"rank",r.rank},{"setup_m",r.setup_m},{"shift",r.shift},{"logs",r.logs},{"counts",J::array({c.base_pairs,c.query_pairs,c.branches,c.unknown,c.certified,c.root_checks,c.rows,c.setup_queries,c.online_queries,c.peak,r.progress_mults})}};}
std::pair<std::string,J> profile(int route,int policy,int cap,P g,P s,const std::vector<P>&source,const std::vector<P>&base,int reps){ROUTE=route;terminal_stop=route!=0;std::ostringstream payload;if(RECORD){payload<<"PROJECTIVE "<<route<<' '<<policy<<' '<<cap<<" 61 59 31\nBASE";for(P b:base){payload<<' ';emit(b,payload);}payload<<'\n';}Counts dc;std::vector<F>images;for(P b:base){images.push_back(*pairing(b,s));dc.base_pairs++;}int rank_sum=0;J diagnostics=J::array();for(int qi=0;qi<31;qi++){if(RECORD)payload<<"DIAG "<<qi<<'\n';auto rows=query(source[qi],s,base,images,cap,dc,payload);Matrix m;for(auto row:rows)m.insert(row,0);if(qi)rank_sum+=m.a.size();diagnostics.push_back(m.a.size());if(RECORD)payload<<"RANK "<<m.a.size()<<'\n';}
J works=J::array();for(int qi=1;qi<31;qi++){J observations=J::array();J expected;for(int rep=0;rep<reps;rep++){auto r=recover_cold(source[qi],g,s,base,cap);J v=result(r);if(rep==0){expected=v;if(RECORD){payload<<"WORK "<<qi<<' ';payload<<r.scalar<<' '<<r.kind<<' '<<r.rank<<' '<<r.setup_m<<' '<<r.shift;for(auto x:r.logs)payload<<' '<<x;for(auto x:v["counts"])payload<<' '<<x;payload<<'\n'<<r.trace<<"END\n";}}else need(v==expected,"fresh repeated semantics");long long total=0;for(auto n:r.times)total+=n;observations.push_back(J{{"phase_ns",r.times},{"cold_ns",total},{"warmup",false}});}works.push_back(J{{"target",qi},{"result",expected},{"timings",observations}});}
J summary={{"route",route},{"policy",policy},{"cap",cap},{"diagnostic_ranks",diagnostics},{"rank_numerator",rank_sum},{"works",works},{"matrix_degree",nullptr},{"polynomial_field",route==4?"F61":"F61^2"},{"original_input_pullback_witness_degree",nullptr}};return{payload.str(),summary};}
int main(int argc,char**argv){try{need(argc==4,"produce NEW C|A|B");need(std::string(argv[1])=="produce","produce");std::string tag=argv[3];need(tag=="C"||tag=="A"||tag=="B","tag");RECORD=tag=="C";fs::path out=argv[2];need(fs::create_directory(out),"fresh directory");P g{F(0),F(10),false},s=chi(P{F(37),F(14),false});auto source=group(g,21,39);std::vector<P>reps;for(P q:source)if(!q.o&&q<neg(q))reps.push_back(q);need(reps.size()==15,"15inversepairs");J maps=J::array();for(int route=3;route<6;route++){ROUTE=route;F z=*pairing(g,s);need(z!=F(1)&&pow(z,31)==F(1),"primitive");for(P q:source){F value=*pairing(q,s);need(pow(value,31)==F(1),"image");maps.push_back(J::array({route,q.o?-1:encode(q.x),q.o?-1:encode(q.y),encode(value)}));}for(P x:source)for(P y:source)need(*pairing(add(x,y),s)==*pairing(x,s)*(*pairing(y,s)),"actual whole-domain homomorphism");}
J index={{"schema","twist.projective-cold/v1"},{"tag",tag},{"maps",maps},{"cases",J::array()}};for(int route=3;route<6;route++){int policy=0;for(int a=0;a<15;a++)for(int b=a+1;b<15;b++)for(int c=b+1;c<15;c++,policy++)for(int cap:{3,4}){std::vector<P>base{reps[a],reps[b],reps[c]};auto[payload,summary]=profile(route,policy,cap,g,s,source,base,1);std::string name=std::to_string(route)+"-"+std::to_string(policy)+"-"+std::to_string(cap)+".txt";if(RECORD){gzFile file=gzopen((out/(name+".gz")).c_str(),"wb9");need(file,"gzip open");need(gzwrite(file,payload.data(),payload.size())==int(payload.size()),"gzip write");need(gzclose(file)==Z_OK,"gzip close");}index["cases"].push_back(J{{"path",name},{"sha256",RECORD?J(digest(payload)):J(nullptr)},{"summary",summary}});}need(policy==455,"455policies");}{auto raw=index.dump()+"\n";gzFile file=gzopen((out/"index.json.gz").c_str(),"wb9");need(file,"index gzip open");need(gzwrite(file,raw.data(),raw.size())==int(raw.size()),"index gzip write");need(gzclose(file)==Z_OK,"index gzip close");}return 0;}catch(const std::exception&e){std::cerr<<e.what()<<'\n';return 101;}}
