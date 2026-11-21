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
Cert certify(const std::vector<F>&a,const std::vector<F>&b,F z,int cap){
 need(pow(z,31)==F(1)&&z.a!=60,"query chart domain");BaseScope mode(ROUTE==4);bound(3,cap);Poly f{F(1)},h{F(1)};
 if(ROUTE==4){
  for(F x:a){need(x.a!=60&&mod(x.a*x.a-59*x.b*x.b)==1,"public chart domain");f=product(f,Poly{F(-x.b),F(x.a+1)},cap);}
  for(F x:b){need(x.a!=60&&mod(x.a*x.a-59*x.b*x.b)==1,"second chart domain");h=product(h,Poly{F((x.a+1)*z.b-x.b*(z.a+1)),F(-(x.a+1)*(z.a+1)+59*x.b*z.b)},cap);}
 }else{for(auto x:a)f=product(f,Poly{F()-x,F(1)},cap);for(auto x:b)h=product(h,Poly{z,F()-x},cap);}
 need(deg(f)==3&&deg(h)==3,"actual cubic leading coefficients");
 F uf=h.back(),vh=-f.back();Poly r=sub(scale(f,uf),scale(h,-vh));
 need(deg(r)<=2,"leading term cancellation");
 if(r.empty()){
  F u=inverse(f.back());Poly d=scale(f,u);
  need(product(Poly{u},f,cap)==d,"proportional original-input witness");
  return{f,h,d,Poly{u},Poly{}};
 }
 need(sub(sub(product(Poly{uf},f,cap),scale(product(Poly{vh},h,cap),F(-1))),r).empty(),"original input root-cover identity");
 return{f,h,r,Poly{uf},Poly{vh}};
}

F eval(const Poly&,F);
// Tonelli--Shanks over the actual selected field; no factor-base/root lookup.
std::optional<F> square_root(F n){
 const int order=BASE_POLY?61:3721;
 if(n==F())return F();
 if(pow(n,(order-1)/2)!=F(1))return{};
 int odd=order-1,s=0;while((odd&1)==0){odd>>=1;s++;}
 F nonresidue=BASE_POLY?F(2):F(0,1);
 need(pow(nonresidue,(order-1)/2)==F(-1),"declared field nonsquare");
 F c=pow(nonresidue,odd),t=pow(n,odd),x=pow(n,(odd+1)/2);int m=s;
 while(t!=F(1)){
  F probe=t;int i=0;while(probe!=F(1)&&i<m){probe=probe*probe;i++;}
  need(i>0&&i<m,"Tonelli--Shanks invariant");F b=pow(c,1<<(m-i-1));x=x*b;c=b*b;t=t*c;m=i;
 }
 need(x*x==n,"field square root certificate");return x;
}
// Classical Cardano, coefficients ONLY; no factor-base roots or scalar labels.
// Residue exponent is used only after checking the exact residue hypothesis.
F cubic_root(F x){
 const int order=BASE_POLY?61:3721,m=(order-1)/3,e=BASE_POLY?7:827;
 if(x==F())return F();
 if(pow(x,m)!=F(1))throw Unknown{};
 F u=pow(x,e);need(u*u*u==x,"checked field cubic root");return u;
}
std::vector<F> cubic_roots(const Poly&f){
 need(deg(f)==3,"actual cubic input");F a=f[2]/f[3],b=f[1]/f[3],c=f[0]/f[3];
 F p=b-a*a/F(3),q=F(2)*a*a*a/F(27)-a*b/F(3)+c;
 auto delta=square_root(q*q/F(4)+p*p*p/F(27));if(!delta)throw Unknown{};
 F rad=-q/F(2)+*delta;if(rad==F())rad=-q/F(2)-*delta;
 std::vector<F>all;
 if(rad==F()){need(p==F()&&q==F(),"zero resolvent branch");all={-a/F(3),-a/F(3),-a/F(3)};}
 else{
  F u=cubic_root(rad),v=-p/(F(3)*u),w(13);
  need(w!=F(1)&&w*w+w+F(1)==F(),"actual primitive cube root of unity");
  need(F(-3)*u*v==p&&-(u*u*u+v*v*v)==q,"compatible Cardano branch");
  F up=u,vp=v;for(int j=0;j<3;j++){all.push_back(up+vp-a/F(3));up=up*w;vp=vp*w*w;}
 }
 // Actual factorization certificate proves completeness INCLUDING multiplicity.
 Poly factor{F(1)};for(F x:all){need(eval(f,x)==F(),"actual cubic root");factor=product(factor,Poly{-x,F(1)},3);}
 need(factor==scale(f,inverse(f.back())),"complete cubic factorization certificate");
 std::sort(all.begin(),all.end());all.erase(std::unique(all.begin(),all.end()),all.end());return all;
}
std::vector<F> unrestricted_cover_roots(const Poly&r){
 need(!r.empty()&&deg(r)<=3,"supported nonzero cover required");if(deg(r)==3)return cubic_roots(r);std::vector<F>out;
 if(deg(r)==0)return out;
 if(deg(r)==1)out.push_back(-r[0]/r[1]);
 else{F disc=r[1]*r[1]-F(4)*r[2]*r[0];auto delta=square_root(disc);if(delta){out.push_back((-r[1]+*delta)/(F(2)*r[2]));out.push_back((-r[1]-*delta)/(F(2)*r[2]));}}
 std::sort(out.begin(),out.end());out.erase(std::unique(out.begin(),out.end()),out.end());for(F x:out)need(eval(r,x)==F(),"extracted cover root");return out;
}

// Constrained extraction on the ACTUAL nonzero norm-one image domain.
// Original f/h/R witnesses and degree caps are unchanged.
bool norm_one(F x){return x!=F()&&x*F(x.a,-x.b)==F(1);}
std::vector<F> cover_roots(const Poly&r){
 if(BASE_POLY)return unrestricted_cover_roots(r);
 need(!r.empty()&&deg(r)<=3,"actual constrained cover");
 if(deg(r)==2){
  F a=r[2],b=r[1],c=r[0],ca(a.a,-a.b),cb(b.a,-b.b),cc(c.a,-c.b);
  F slope=cc*b-a*cb,constant=cc*c-a*ca;
  // cc*R-a*Rsharp is at most linear; every norm-one root annihilates it.
  if(slope!=F()){
   F x=-constant/slope;
   if(norm_one(x)&&eval(r,x)==F())return{x};
   return{};
  }
  if(constant!=F())return{};
 }
 auto roots=unrestricted_cover_roots(r);
 roots.erase(std::remove_if(roots.begin(),roots.end(),[](F x){return !norm_one(x);}),roots.end());
 return roots;
}

bool root(const Poly&a,F x){if(ROUTE!=4)return false;BaseScope mode(true);F value,den(1),num(x.b),d(x.a+1);for(size_t i=a.size();i-->0;){value=value*num+a[i]*den;den=den*d;}return value==F();}
F eval(const Poly&a,F x){F z;for(size_t i=a.size();i-->0;)z=z*x+a[i];return z;}
std::string poly(const Poly&a){std::ostringstream o;o<<a.size();for(F x:a)o<<','<<encode(x);return o.str();}
using Row=std::array<int,3>;
int rel(int n){n%=31;return n<0?n+31:n;}int rinv(int n){for(int k=1;k<31;k++)if(rel(k*n)==1)return k;throw std::runtime_error("zero relation inverse");}
struct Matrix{std::map<int,std::array<int,4>>a;void insert(Row v,int m){std::array<int,4>r{rel(v[0]),rel(v[1]),rel(v[2]),rel(m)};for(auto&[p,b]:a){int c=r[p];for(int j=0;j<4;j++)r[j]=rel(r[j]-c*b[j]);}int p=0;while(p<3&&!r[p])p++;if(p==3){need(!r[3],"inconsistent setup");return;}int k=rinv(r[p]);for(int&x:r)x=rel(x*k);for(auto&[q,b]:a){int c=b[p];for(int j=0;j<4;j++)b[j]=rel(b[j]-c*r[j]);}a[p]=r;}std::array<int,3>logs(){need(a.size()==3,"incomplete setup");return{a.at(0)[3],a.at(1)[3],a.at(2)[3]};}};
struct Counts{int base_pairs=0,query_pairs=0,branches=0,unknown=0,certified=0,root_checks=0,rows=0,setup_queries=0,online_queries=0,peak=0;};
std::vector<Row> query(P q,P partner,const std::vector<P>&base,const std::vector<F>&images,int cap,Counts&count,std::ostringstream&trace,std::optional<F> supplied={}){
 std::vector<Row>rows;F z;if(cap>=3){auto value=supplied?supplied:pairing(q,partner);need(value.has_value(),"actual query pairing");z=*value;if(!supplied)count.query_pairs++;}
 if(RECORD)trace<<"QUERY "<<(q.o?-1:encode(q.x))<<' '<<(q.o?-1:encode(q.y))<<' '<<encode(z)<<'\n';
 for(int sign:{-1,1})for(int sign2:{-1,1}){count.branches++;peak=0;if(RECORD)trace<<"BR "<<sign<<' '<<sign2<<' ';
  try{bound(3,cap);std::vector<F>a,b;for(F x:images){a.push_back(sign==1?x:(ROUTE>=4?F(x.a,-x.b):inverse(x)));b.push_back(sign2==1?x:(ROUTE>=4?F(x.a,-x.b):inverse(x)));}
   Cert c=certify(a,b,z,cap);std::vector<F>roots;{BaseScope mode(ROUTE==4);roots=cover_roots(c.d);}
   std::vector<std::pair<int,int>>found;
   for(F x:roots){count.root_checks++;bool both;{BaseScope mode(ROUTE==4);both=eval(c.f,x)==F()&&eval(c.h,x)==F();}if(!both)continue;
    for(int i=0;i<3;i++)if(ROUTE==4?F(a[i].b)==x*F(a[i].a+1):a[i]==x)for(int j=0;j<3;j++)if(a[i]*b[j]==z){
     need(add(sign==1?base[i]:neg(base[i]),sign2==1?base[j]:neg(base[j]))==q,"actual source row");Row row{};row[i]+=sign;row[j]+=sign2;rows.push_back(row);found.push_back({i,j});
    }
   }
   std::sort(found.begin(),found.end());
   if(RECORD){trace<<"C "<<peak<<' '<<poly(c.f)<<' '<<poly(c.h)<<' '<<poly(c.d)<<' '<<poly(c.u)<<' '<<poly(c.v)<<' '<<roots.size();for(F x:roots)trace<<' '<<encode(x);trace<<' '<<found.size();for(auto[i,j]:found)trace<<' '<<i<<','<<j;trace<<'\n';}
   count.certified++;count.rows+=found.size();
  }catch(const Unknown&){if(RECORD)trace<<"U "<<peak<<'\n';count.unknown++;}
  count.peak=std::max(count.peak,peak);
 }
 return rows;
}

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
std::pair<std::string,J> profile(int route,int policy,int cap,P g,P s,const std::vector<P>&source,const std::vector<P>&base,int reps){ROUTE=route;terminal_stop=route!=0;std::ostringstream payload;if(RECORD){payload<<"ROOTCOVER "<<route<<' '<<policy<<' '<<cap<<" 61 59 31\nBASE";for(P b:base){payload<<' ';emit(b,payload);}payload<<'\n';}Counts dc;std::vector<F>images;for(P b:base){images.push_back(*pairing(b,s));dc.base_pairs++;}int rank_sum=0;J diagnostics=J::array();for(int qi=0;qi<31;qi++){if(RECORD)payload<<"DIAG "<<qi<<'\n';auto rows=query(source[qi],s,base,images,cap,dc,payload);Matrix m;for(auto row:rows)m.insert(row,0);if(qi)rank_sum+=m.a.size();diagnostics.push_back(m.a.size());if(RECORD)payload<<"RANK "<<m.a.size()<<'\n';}
J works=J::array();for(int qi=1;qi<31;qi++){J observations=J::array();J expected;for(int rep=0;rep<reps;rep++){auto r=recover_cold(source[qi],g,s,base,cap);J v=result(r);if(rep==0){expected=v;if(RECORD){payload<<"WORK "<<qi<<' ';payload<<r.scalar<<' '<<r.kind<<' '<<r.rank<<' '<<r.setup_m<<' '<<r.shift;for(auto x:r.logs)payload<<' '<<x;for(auto x:v["counts"])payload<<' '<<x;payload<<'\n'<<r.trace<<"END\n";}}else need(v==expected,"fresh repeated semantics");long long total=0;for(auto n:r.times)total+=n;observations.push_back(J{{"phase_ns",r.times},{"cold_ns",total},{"warmup",false}});}works.push_back(J{{"target",qi},{"result",expected},{"timings",observations}});}
J summary={{"route",route},{"policy",policy},{"cap",cap},{"diagnostic_ranks",diagnostics},{"rank_numerator",rank_sum},{"works",works},{"matrix_degree",nullptr},{"polynomial_field",route==4?"F61":"F61^2"},{"original_input_pullback_witness_degree",nullptr}};return{payload.str(),summary};}
int produce_main(int argc,char**argv){try{need(argc==4,"produce NEW C|A|B");need(std::string(argv[1])=="produce","produce");std::string tag=argv[3];need(tag=="C"||tag=="A"||tag=="B","tag");RECORD=tag=="C";fs::path out=argv[2];need(fs::create_directory(out),"fresh directory");P g{F(0),F(10),false},s=chi(P{F(37),F(14),false});auto source=group(g,21,39);std::vector<P>reps;for(P q:source)if(!q.o&&q<neg(q))reps.push_back(q);need(reps.size()==15,"15inversepairs");J maps=J::array();for(int route=3;route<6;route++){ROUTE=route;F z=*pairing(g,s);need(z!=F(1)&&pow(z,31)==F(1),"primitive");for(P q:source){F value=*pairing(q,s);need(pow(value,31)==F(1),"image");maps.push_back(J::array({route,q.o?-1:encode(q.x),q.o?-1:encode(q.y),encode(value)}));}for(P x:source)for(P y:source)need(*pairing(add(x,y),s)==*pairing(x,s)*(*pairing(y,s)),"actual whole-domain homomorphism");}
J index={{"schema","twist.root-cover-cold/v1"},{"tag",tag},{"maps",maps},{"cases",J::array()}};for(int route=3;route<6;route++){int policy=0;for(int a=0;a<15;a++)for(int b=a+1;b<15;b++)for(int c=b+1;c<15;c++,policy++)for(int cap:{3}){std::vector<P>base{reps[a],reps[b],reps[c]};auto[payload,summary]=profile(route,policy,cap,g,s,source,base,1);std::string name=std::to_string(route)+"-"+std::to_string(policy)+"-"+std::to_string(cap)+".txt";if(RECORD){gzFile file=gzopen((out/(name+".gz")).c_str(),"wb9");need(file,"gzip open");need(gzwrite(file,payload.data(),payload.size())==int(payload.size()),"gzip write");need(gzclose(file)==Z_OK,"gzip close");}index["cases"].push_back(J{{"path",name},{"sha256",RECORD?J(digest(payload)):J(nullptr)},{"summary",summary}});}need(policy==455,"455policies");}{auto raw=index.dump()+"\n";gzFile file=gzopen((out/"index.json.gz").c_str(),"wb9");need(file,"index gzip open");need(gzwrite(file,raw.data(),raw.size())==int(raw.size()),"index gzip write");need(gzclose(file)==Z_OK,"index gzip close");}return 0;}catch(const std::exception&e){std::cerr<<e.what()<<'\n';return 101;}}

std::string gunzip(const fs::path&p){gzFile f=gzopen(p.c_str(),"rb");need(f,"gzip input");std::string s;char buf[65536];int n;while((n=gzread(f,buf,sizeof buf))>0)s.append(buf,n);need(n==0&&gzclose(f)==Z_OK,"gzip complete read");return s;}
void gzip_new(const fs::path&p,const std::string&s){need(!fs::exists(p),"new gzip only");gzFile f=gzopen(p.c_str(),"wb9");need(f,"gzip output");need(gzwrite(f,s.data(),s.size())==int(s.size())&&gzclose(f)==Z_OK,"complete gzip output");}
void corrupt(const fs::path&source,const fs::path&out,const std::string&mode){
 need(fs::create_directory(out),"fresh false corpus");for(const auto&e:fs::directory_iterator(source)){need(e.is_regular_file()&&!e.is_symlink(),"regular corpus input");fs::create_hard_link(e.path(),out/e.path().filename());}
 std::string raw_index=gunzip(source/"index.json.gz");J index=J::parse(raw_index);std::string name=index["cases"][0]["path"];fs::path raw_path=source/(name+".gz");std::string original=gunzip(raw_path),changed=original;
 if(mode=="scalar")index["cases"][0]["summary"]["works"][0]["result"]["scalar"]=(index["cases"][0]["summary"]["works"][0]["result"]["scalar"].get<int>()+1)%31;
 else if(mode=="log")index["cases"][0]["summary"]["works"][0]["result"]["logs"][0]=(index["cases"][0]["summary"]["works"][0]["result"]["logs"][0].get<int>()+1)%31;
 else if(mode=="cost")index["cases"][0]["summary"]["works"][0]["result"]["counts"][0]=index["cases"][0]["summary"]["works"][0]["result"]["counts"][0].get<int>()+1;
 else if(mode=="missing-policy")index["cases"].erase(index["cases"].begin());
 else if(mode=="map")index["maps"][1][3]=(index["maps"][1][3].get<int>()+1)%3721;
 else if(mode=="source"){auto pos=changed.find(" 61 59 31\n");need(pos!=std::string::npos,"actual source header");changed.replace(pos,10," 59 59 31\n");}
 else{
  std::istringstream input(original);std::string line;std::ostringstream output;bool done=false;
  while(std::getline(input,line)){
   std::vector<std::string>t;std::istringstream row(line);std::string word;while(row>>word)t.push_back(word);
   if(!done&&mode=="columns"&&t.size()==7&&t[0]=="BASE"){std::swap(t[1],t[3]);std::swap(t[2],t[4]);done=true;}
   if(!done&&mode=="missing-failure"&&t.size()>=4&&t[0]=="BR"&&t[3]=="U"){done=true;continue;}
   if(!done&&t.size()>11&&t[0]=="BR"&&t[3]=="C"){
    if(mode=="cap"){t[4]="4";done=true;}
    if(mode=="witness"){auto comma=t[8].find(',');need(comma!=std::string::npos,"constant witness");int old=std::stoi(t[8].substr(comma+1));t[8]=t[8].substr(0,comma+1)+std::to_string((old+1)%3721);done=true;}
    if(mode=="root"&&std::stoi(t[10])>0){t[11]=std::to_string((std::stoi(t[11])+1)%3721);done=true;}
    if(mode=="row"&&std::stoi(t[11+std::stoi(t[10])])>0){size_t at=12+std::stoi(t[10]);auto comma=t[at].find(',');need(comma!=std::string::npos,"actual row witness");t[at]=std::to_string((std::stoi(t[at].substr(0,comma))+1)%3)+t[at].substr(comma);done=true;}
     if(mode=="branch"){t[1]=t[1]=="-1"?"1":"-1";done=true;}
   }
   if(done){for(size_t i=0;i<t.size();i++){if(i)output<<' ';output<<t[i];}output<<'\n';}else output<<line<<'\n';
  }
  need(done,"requested actual non-null false-control site absent");changed=output.str();
 }
 if(changed!=original){fs::remove(out/(name+".gz"));gzip_new(out/(name+".gz"),changed);index["cases"][0]["sha256"]=digest(changed);}
 std::string false_index=index.dump()+"\n";need(changed!=original||false_index!=raw_index,"genuine false payload required");fs::remove(out/"index.json.gz");gzip_new(out/"index.json.gz",false_index);
 need(gunzip(source/"index.json.gz")==raw_index&&gunzip(raw_path)==original,"original corpus unchanged");
 std::cout<<J{{"control",mode},{"changed_case",changed!=original},{"original_case_sha256",digest(original)},{"changed_case_sha256",digest(changed)},{"original_index_sha256",digest(raw_index)},{"false_index_sha256",digest(false_index)},{"null",false}}.dump()<<'\n';
}
int unused_legacy_main(int argc,char**argv){try{if(argc==5&&std::string(argv[1])=="corrupt"){corrupt(argv[2],argv[3],argv[4]);return 0;}return produce_main(argc,argv);}catch(const std::exception&e){std::cerr<<e.what()<<'\n';return 101;}}

std::string compact_profile(int route,int policy,P g,P partner,const std::vector<P>&source,const std::vector<P>&base){
 ROUTE=route;RECORD=true;std::ostringstream raw;raw<<"CARDANO "<<route<<' '<<policy<<" 3 61 59 31\nBASE";for(P x:base){raw<<' ';emit(x,raw);}raw<<'\n';Counts count;std::vector<F>images;for(P x:base)images.push_back(*pairing(x,partner));
 for(int qi=0;qi<31;qi++){raw<<"DIAG "<<qi<<'\n';auto rows=query(source[qi],partner,base,images,3,count,raw);Matrix m;for(auto row:rows)m.insert(row,0);raw<<"RANK "<<m.a.size()<<'\n';}
 // Fresh cold calls; diagnostics are NEVER read by recovery. All possible query
 // witnesses are already explicitly emitted once above; work sequence follows
 // setup_m/shift and is independently recomputed, not a cached answer table.
 RECORD=false;for(int qi=1;qi<31;qi++){
  auto r=recover_cold(source[qi],g,partner,base,3);raw<<"WORK "<<qi<<' '<<r.scalar<<' '<<r.kind<<' '<<r.rank<<' '<<r.setup_m<<' '<<r.shift;for(int x:r.logs)raw<<' '<<x;
  auto c=r.count;for(int x:{c.base_pairs,c.query_pairs,c.branches,c.unknown,c.certified,c.root_checks,c.rows,c.setup_queries,c.online_queries,c.peak,r.progress_mults})raw<<' '<<x;raw<<"\nEND\n";
 }RECORD=true;return raw.str();
}
void stream_produce(const std::string&tag,const std::string&head){
 need(tag=="C"||tag=="A"||tag=="B","tag");P g{F(0),F(10),false},partner=chi(P{F(37),F(14),false});auto source=group(g,21,39);std::vector<P>reps;for(P q:source)if(!q.o&&q<neg(q))reps.push_back(q);need(reps.size()==15,"full public pool");
 std::cout<<"TWIST-NORM-LINEAR 1 "<<head<<' '<<tag<<" 61 59 31 21 39 23 54 0 10 37 14\n";
 int profiles=0;for(int route=3;route<6;route++){ROUTE=route;F image=*pairing(g,partner);need(image!=F(1)&&pow(image,31)==F(1),"source-preserving primitive pairing");for(P x:source)for(P y:source)need(*pairing(add(x,y),partner)==*pairing(x,partner)*(*pairing(y,partner)),"actual full-domain map");
  int policy=0;for(int i=0;i<15;i++)for(int j=i+1;j<15;j++)for(int k=j+1;k<15;k++,policy++){
   std::string body=compact_profile(route,policy,g,partner,source,{reps[i],reps[j],reps[k]});std::cout<<"CASE "<<route<<' '<<policy<<" 3 "<<body.size()<<' '<<digest(body)<<'\n';if(tag=="C")std::cout<<body;std::cout<<"ENDCASE\n";profiles++;
  }need(policy==455,"full policy census");
 }need(profiles==1365,"all profiles");std::cout<<"DONE 1365\n";need(bool(std::cout),"complete stream output");
}
void selftest(const std::string&head){
 std::cout<<"CUBIC-SELFTEST "<<head<<"\n";
 for(int domain=0;domain<2;domain++){
  ROUTE=3;BASE_POLY=false;std::vector<F>alphabet;
  if(domain==0){for(int i=0;i<61;i++)alphabet.push_back(F(i));}
  else{F z=fast_tate(P{F(0),F(10),false},chi(P{F(37),F(14),false}));need(z!=F(1)&&pow(z,31)==F(1),"actual extension alphabet");alphabet.push_back(F());F at(1);for(int i=0;i<31;i++){alphabet.push_back(at);at=at*z;}}
  std::sort(alphabet.begin(),alphabet.end());need(std::set<F>(alphabet.begin(),alphabet.end()).size()==alphabet.size(),"distinct actual alphabet");BaseScope mode(domain==0);
  long long cases=0;for(size_t i=0;i<alphabet.size();i++)for(size_t j=i;j<alphabet.size();j++)for(size_t k=j;k<alphabet.size();k++){
   peak=0;Poly f{F(1)};for(F x:{alphabet[i],alphabet[j],alphabet[k]})f=product(f,Poly{-x,F(1)},3);
   auto got=cubic_roots(f);std::vector<F>expected{alphabet[i],alphabet[j],alphabet[k]};expected.erase(std::unique(expected.begin(),expected.end()),expected.end());need(got==expected,"all split/repeated cubic roots");std::cout<<"S "<<domain<<' '<<i<<' '<<j<<' '<<k<<' '<<poly(f)<<' '<<got.size();for(F x:got)std::cout<<' '<<encode(x);std::cout<<'\n';cases++;
  }std::cout<<"DOMAIN "<<domain<<' '<<cases<<'\n';
 }std::cout<<"SELFTEST-DONE\n";
}
void patch(const std::string&mode){
 std::string raw{std::istreambuf_iterator<char>(std::cin),{}};auto first=raw.find("CASE "),lineend=raw.find('\n',first);need(first!=std::string::npos&&lineend!=std::string::npos,"actual first profile");int route,policy,cap;size_t len;std::string marker,oldhash;std::istringstream row(raw.substr(first,lineend-first));row>>marker>>route>>policy>>cap>>len>>oldhash;std::string original=raw.substr(lineend+1,len),changed=original;need(digest(original)==oldhash,"original profile hash");
 size_t offset=first,oldlen=lineend+1+len+8-first;std::string replacement;
 if(mode=="missing-policy")replacement="";
 else if(mode=="source"){offset=0;oldlen=raw.find('\n')+1;replacement=raw.substr(0,oldlen);auto at=replacement.find(" 61 59 31");need(at!=std::string::npos,"source site");replacement.replace(at,9," 59 59 31");}
 else{
  std::istringstream input(original);std::ostringstream output;std::string line;bool done=false,skip=false;
  while(std::getline(input,line)){
   std::vector<std::string>t;std::istringstream in(line);std::string x;while(in>>x)t.push_back(x);
   if(!done&&mode=="missing-query"&&line=="DIAG 0"){skip=true;done=true;continue;}if(skip){if(line=="DIAG 1")skip=false;else continue;}
   bool remove=false;
   if(!done&&!t.empty()){
    if(mode=="cap"&&t[0]=="CARDANO"){t[3]="4";done=true;}
    if(mode=="columns"&&t[0]=="BASE"){std::swap(t[1],t[3]);std::swap(t[2],t[4]);done=true;}
    if(mode=="rank"&&t[0]=="RANK"){t[1]=std::to_string((std::stoi(t[1])+1)%4);done=true;}
    if(t[0]=="WORK"&&((mode=="scalar")||(mode=="log")||(mode=="cost"))){size_t at=mode=="scalar"?2:mode=="log"?7:10;t[at]=std::to_string((std::stoi(t[at])+1)%(mode=="cost"?1000000:31));done=true;}
    if(t[0]=="BR"&&t.size()>11&&t[3]=="C"){
     if(mode=="row"&&std::stoi(t[11+std::stoi(t[10])])>0){size_t at=12+std::stoi(t[10]);auto comma=t[at].find(',');need(comma!=std::string::npos,"actual row witness");t[at]=std::to_string((std::stoi(t[at].substr(0,comma))+1)%3)+t[at].substr(comma);done=true;}
     if(mode=="branch"){t[1]=t[1]=="-1"?"1":"-1";done=true;}
     if(mode=="witness"){auto pos=t[8].find(',');need(pos!=std::string::npos,"actual witness");t[8]=t[8].substr(0,pos+1)+std::to_string((std::stoi(t[8].substr(pos+1))+1)%3721);done=true;}
     if(mode=="root"&&std::stoi(t[10])>0){t[11]=std::to_string((std::stoi(t[11])+1)%3721);done=true;}
     if(mode=="drop-proportional"&&t[7].starts_with("4,")){t={"BR",t[1],t[2],"U","3"};done=true;}
     if(mode=="missing-empty-branch"&&t[10]=="0"){done=true;remove=true;}
    }
   }
   if(remove)continue;if(done){for(size_t i=0;i<t.size();i++){if(i)output<<' ';output<<t[i];}output<<'\n';}else output<<line<<'\n';
  }
  need(done,"actual non-null control site required");changed=output.str();need(changed!=original,"genuinely altered profile required");std::ostringstream frame;frame<<"CASE "<<route<<' '<<policy<<' '<<cap<<' '<<changed.size()<<' '<<digest(changed)<<'\n'<<changed<<"ENDCASE\n";replacement=frame.str();
 }
 need(raw.substr(offset,oldlen)!=replacement,"genuine payload change");std::cout<<"PATCH "<<offset<<' '<<oldlen<<' '<<replacement.size()<<' '<<digest(raw.substr(offset,oldlen))<<' '<<digest(replacement)<<'\n'<<replacement;
}
void apply_patch_file(const fs::path&delta,const fs::path&hashout){
 std::string raw{std::istreambuf_iterator<char>(std::cin),{}};auto text=read(delta);auto end=text.find('\n');std::istringstream in(text.substr(0,end));std::string name,oldhash,newhash;size_t offset,len,n;in>>name>>offset>>len>>n>>oldhash>>newhash;need(name=="PATCH"&&offset+len<=raw.size(),"patch scope");std::string replacement=text.substr(end+1);need(replacement.size()==n&&digest(replacement)==newhash&&digest(raw.substr(offset,len))==oldhash,"exact altered payload binding");raw.replace(offset,len,replacement);write(hashout,digest(raw)+"\n");std::cout<<raw;need(bool(std::cout),"false stream emission");
}
int main(int argc,char**argv){try{
 if(argc==4&&std::string(argv[1])=="produce"){stream_produce(argv[2],argv[3]);return 0;}
 if(argc==3&&std::string(argv[1])=="selftest"){selftest(argv[2]);return 0;}
 if(argc==3&&std::string(argv[1])=="corrupt"){patch(argv[2]);return 0;}
 if(argc==4&&std::string(argv[1])=="apply"){apply_patch_file(argv[2],argv[3]);return 0;}
 return 2;
}catch(const std::exception&e){std::cerr<<e.what()<<'\n';return 101;}}
