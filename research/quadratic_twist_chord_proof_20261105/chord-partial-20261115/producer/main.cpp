// Versioned partial-certificate chord instrument; source must be published before produce.
#include <nlohmann/json.hpp>
#include <openssl/sha.h>
#include <algorithm>
#include <array>
#include <filesystem>
#include <fstream>
#include <iostream>
#include <map>
#include <optional>
#include <set>
#include <sstream>
#include <stdexcept>
#include <vector>
using J=nlohmann::json; using Poly=std::vector<int>; using Row=std::array<int,3>;
void need(bool v,const char*m){if(!v)throw std::runtime_error(m);} int f(int x,int p=37){x%=p;return x<0?x+p:x;} int pw(int x,int n,int p=37){int r=1;for(;n;n>>=1,x=f(x*x,p))if(n&1)r=f(r*x,p);return r;}
void trim(Poly&a){while(!a.empty()&&!a.back())a.pop_back();} int deg(const Poly&a){return int(a.size())-1;}
Poly add(Poly a,const Poly&b,int sign=1){a.resize(std::max(a.size(),b.size()));for(size_t i=0;i<b.size();i++)a[i]=f(a[i]+sign*b[i]);trim(a);return a;}
Poly scale(Poly a,int c){for(int&x:a)x=f(x*c);trim(a);return a;} Poly mul(const Poly&a,const Poly&b){if(a.empty()||b.empty())return {};Poly r(a.size()+b.size()-1);for(size_t i=0;i<a.size();i++)for(size_t j=0;j<b.size();j++)r[i+j]=f(r[i+j]+a[i]*b[j]);trim(r);return r;}
std::pair<Poly,Poly> divide(Poly a,const Poly&b){need(!b.empty(),"zero divisor");Poly q(std::max(0,deg(a)-deg(b)+1));while(deg(a)>=deg(b)){int d=deg(a)-deg(b),c=f(a.back()*pw(b.back(),35));q[d]=c;for(size_t i=0;i<b.size();i++)a[i+d]=f(a[i+d]-c*b[i]);trim(a);}trim(q);return {q,a};}
int eval(const Poly&a,int x){int r=0;for(auto i=a.rbegin();i!=a.rend();++i)r=f(r*x+*i);return r;}
int proddeg(const Poly&a,const Poly&b){return a.empty()||b.empty()?-1:deg(a)+deg(b);}
struct Cert{bool ok=false;int denied=-1;Poly h,a,b;int degree=-1;};
Cert certify(const Poly&F,const Poly&G,int cap){
 if(std::max(deg(F),deg(G))>cap)return {false,std::max(deg(F),deg(G)),{},{},{},-1};
 Poly r0=F,r1=G,a0{1},a1{},b0{},b1{1};
 while(!r1.empty()){
  auto qr=divide(r0,r1);const Poly&q=qr.first;
  // Check multiplication degrees before constructing the next witness products.
  int before=std::max({q.empty()||a1.empty()||F.empty()?-1:deg(q)+deg(a1)+deg(F),q.empty()||b1.empty()||G.empty()?-1:deg(q)+deg(b1)+deg(G),proddeg(a0,F),proddeg(b0,G)});
  if(before>cap)return {false,before,{},{},{},-1};
  Poly a2=add(a0,mul(q,a1),-1),b2=add(b0,mul(q,b1),-1);r0=r1;r1=qr.second;a0=a1;a1=a2;b0=b1;b1=b2;
 }
 need(!r0.empty(),"zero ideal unexpected");int z=pw(r0.back(),35);Cert c{true,-1,scale(r0,z),scale(a0,z),scale(b0,z),-1};c.degree=std::max(proddeg(c.a,F),proddeg(c.b,G));need(c.degree<=cap,"certificate cap");need(add(mul(c.a,F),mul(c.b,G))==c.h,"expanded Bezout identity");need(divide(F,c.h).second.empty()&&divide(G,c.h).second.empty(),"generator reduction");return c;
}
J cj(const Cert&c){if(!c.ok)return J{{"status","unknown"},{"denied_bound",c.denied}};return J{{"status","certified"},{"gcd",c.h},{"a",c.a},{"b",c.b},{"expanded_degree",c.degree}};}
struct Pt{int x,y;auto operator<=>(const Pt&)const=default;};using Point=std::optional<Pt>;
Point neg(Point p){if(p)p->y=f(-p->y);return p;} bool on(Point p,int A,int B){return !p||f(p->y*p->y)==f(p->x*p->x*p->x+A*p->x+B);}
Point plus(Point p,Point q,int A){if(!p)return q;if(!q)return p;if(p->x==q->x&&f(p->y+q->y)==0)return {};int l=p->x==q->x?f((3*p->x*p->x+A)*pw(2*p->y,35)):f((q->y-p->y)*pw(q->x-p->x,35));int x=f(l*l-p->x-q->x);return Pt{x,f(l*(p->x-x)-p->y)};}
Point times(Point p,int k,int A){Point r;for(;k;k>>=1,p=plus(p,p,A))if(k&1)r=plus(r,p,A);return r;}
Point phi(Point p){if(!p||p->x==4)return {};int z=pw(p->x-4,35);return Pt{f(p->x+18*z),f(p->y*(1-18*z*z))};}
Point inverse(Point p){if(!p||p->x==29)return {};int z=pw(p->x-29,35);Point d=Pt{f(28*(p->x+35*z)),f(14*p->y*(1-35*z*z))};return times(d,10,7);}
J pj(Point p){return p?J::array({p->x,p->y}):J(nullptr);} std::vector<Point> group(int A,int B){std::vector<Point> g{Point()};for(int x=0;x<37;x++)for(int y=0;y<37;y++){Point p=Pt{x,y};if(on(p,A,B)&&!times(p,19,A))g.push_back(p);}need(g.size()==19,"subgroup coverage");return g;}
using Base=std::array<Pt,3>;std::vector<Base> policies(int A,int B){auto g=group(A,B);std::vector<Pt> half;for(auto p:g)if(p&&*p<*neg(p))half.push_back(*p);need(half.size()==9,"signed base population");std::vector<Base> b;for(size_t i=0;i<half.size();i++)for(size_t j=i+1;j<half.size();j++)for(size_t k=j+1;k<half.size();k++)b.push_back({half[i],half[j],half[k]});return b;}
int rank(std::vector<Row> a){int k=0;for(int col=0;col<3;col++){int i=k;while(i<int(a.size())&&!f(a[i][col],19))i++;if(i==int(a.size()))continue;std::swap(a[i],a[k]);int z=pw(a[k][col],17,19);for(int j=col;j<3;j++)a[k][j]=f(a[k][j]*z,19);for(int u=0;u<int(a.size());u++)if(u!=k){int t=a[u][col];for(int j=col;j<3;j++)a[u][j]=f(a[u][j]-t*a[k][j],19);}k++;}return k;}
int locate(const Base&b,Pt p){for(int i=0;i<3;i++){if(b[i]==p)return i+1;if(*neg(b[i])==p)return -i-1;}throw std::runtime_error("point outside signed base");}
struct Query{J data;std::vector<Row> rows;int degree=-1;};
Query oracle(const Base&base,int A,int B,Point Q,int cap){
 J w=J::array();std::set<Row> rows;int maxdegree=-1;J distinct,diagonal;std::vector<int> counts;
 auto witness=[&](Pt p,Pt q){need(on(p,A,B)&&on(q,A,B)&&plus(p,q,A)==Q,"invalid signed lift");int i=locate(base,p),j=locate(base,q);Row r{};r[std::abs(i)-1]+=i>0?1:-1;r[std::abs(j)-1]+=j>0?1:-1;rows.insert(r);w.push_back({i,j});};
 if(!Q){for(int i=0;i<3;i++)for(int s:{1,-1}){Pt p=s==1?base[i]:*neg(base[i]);witness(p,*neg(p));}distinct={{"status","identity_generic"}};diagonal=distinct;counts={0,0};}
 else{
  int a=Q->x,b=Q->y,c1=f(base[0].x+base[1].x+base[2].x),c2=f(base[0].x*base[1].x+base[0].x*base[2].x+base[1].x*base[2].x),c3=f(base[0].x*base[1].x*base[2].x);
  Poly s{f(-a),0,1},p{f(a*a+A),f(2*b),a};Poly U=add(add(mul(s,s),p,-1),scale(s,-c1));U=add(U,{c2});Poly V=add(mul(add({c1},s,-1),p),{f(-c3)}),W=add(V,scale(U,a));
  Poly delta=add(mul(s,s),scale(p,-4));Poly mid=scale(s,pw(2,35)),membership{1};for(Pt bp:base)membership=mul(membership,add(mid,{f(-bp.x)}));
  Cert dc=certify(U,W,cap),tc=certify(delta,membership,cap);distinct=cj(dc);diagonal=cj(tc);distinct["inputs"]={U,W};diagonal["inputs"]={delta,membership};int lr=0,xr=0;
  for(int branch=0;branch<2;branch++){
   const Cert&c=branch==0?dc:tc;if(!c.ok)continue;maxdegree=std::max(maxdegree,c.degree);
   J roots=J::array();for(int l=0;l<37;l++){lr++;if(eval(c.h,l))continue;roots.push_back(l);int ss=eval(s,l),pp=eval(p,l),dd=f(ss*ss-4*pp);if((branch==0&&dd==0)||(branch==1&&dd!=0))continue;std::vector<int> xs;for(int x=0;x<37;x++){xr++;if(f(x*x-ss*x+pp)==0)xs.push_back(x);}if(branch==0){need(xs.size()==2,"distinct roots fail splitting");for(int x:xs){int y=f(ss-x);Pt rp{x,f(l*(x-a)-b)},rq{y,f(l*(y-a)-b)};witness(rp,rq);}}else{need(xs.size()==1,"diagonal split");int x=xs[0];Pt r{x,f(l*(x-a)-b)};witness(r,r);}}
   if(branch==0)distinct["roots"]=roots;else diagonal["roots"]=roots;
  }counts={lr,xr};
 }
 std::sort(w.begin(),w.end());need(std::adjacent_find(w.begin(),w.end())==w.end(),"duplicate ordered witness");std::vector<Row> rr(rows.begin(),rows.end());return {J{{"query",pj(Q)},{"distinct",distinct},{"diagonal",diagonal},{"witnesses",w},{"rows",rr},{"rank",rank(rr)},{"actual_expanded_degree",maxdegree<0?J(nullptr):J(maxdegree)},{"lambda_evaluations",counts[0]},{"base_root_evaluations",counts[1]}},rr,maxdegree};
}
std::optional<Row> solve_logs(const std::vector<std::array<int,4>>& equations){auto a=equations;int k=0;for(int col=0;col<3;col++){int i=k;while(i<int(a.size())&&!f(a[i][col],19))i++;if(i==int(a.size()))return {};std::swap(a[i],a[k]);int z=pw(a[k][col],17,19);for(int j=col;j<4;j++)a[k][j]=f(a[k][j]*z,19);for(int u=0;u<int(a.size());u++)if(u!=k){int t=a[u][col];for(int j=col;j<4;j++)a[u][j]=f(a[u][j]-t*a[k][j],19);}k++;}for(auto r:a)if(!r[0]&&!r[1]&&!r[2])need(!f(r[3],19),"inconsistent logs");return Row{a[0][3],a[1][3],a[2][3]};}
J configuration(const std::string&role,int id,const Base&base,int cap){
 bool twist=role=="twist"||role=="matched";int A=twist?28:7,B=twist?33:19;Point G=twist?Point(Pt{32,29}):Point(Pt{1,8});auto source=group(7,19);std::vector<Point> queries;for(Point q:source)queries.push_back(twist?phi(q):q);
 std::vector<Query> diagnostics;J cells=J::array();for(Point q:queries){auto a=oracle(base,A,B,q,cap);cells.push_back(a.data);diagnostics.push_back(a);}
 J recoveries=J::array();long calls=0,le=0,xe=0;int maxD=-1;
 auto fresh=[&](Point q)->Query{auto r=oracle(base,A,B,q,cap);auto it=std::find(queries.begin(),queries.end(),q);need(it!=queries.end(),"query outside subgroup");size_t ix=it-queries.begin();need(r.data==diagnostics[ix].data,"fresh query changed");calls++;le+=r.data["lambda_evaluations"].get<int>();xe+=r.data["base_root_evaluations"].get<int>();maxD=std::max(maxD,r.degree);return r;};
 for(size_t target=1;target<source.size();target++){
  std::vector<std::array<int,4>> equations;std::vector<Row> collected;J setup=J::array();Point q;std::optional<Row> logs;
  for(int m=1;m<19;m++){q=plus(q,G,A);auto r=fresh(q);for(auto row:r.rows){collected.push_back(row);equations.push_back({row[0],row[1],row[2],m});}setup.push_back({{"m",m},{"query",pj(q)},{"rank",rank(collected)},{"rows",r.rows}});if(rank(collected)==3){logs=solve_logs(equations);break;}}
  J rec{{"source_query",pj(source[target])},{"setup",setup}};
  if(!logs){rec["status"]="blocked_setup";rec["logs"]=nullptr;rec["shifts"]=J::array();rec["scalar"]=nullptr;recoveries.push_back(rec);continue;}
  for(int i=0;i<3;i++){need(times(G,f((*logs)[i],19),A)==Point(base[i]),"public log point check");}
  rec["logs"]=*logs;J shifts=J::array();Point R=twist?phi(source[target]):source[target];int recovered=-1;
  for(int shift=0;shift<19;shift++){
   auto r=fresh(R);shifts.push_back({{"shift",shift},{"query",pj(R)},{"rows",r.rows}});if(!r.rows.empty()){Row row=r.rows.front();int k=f(row[0]*(*logs)[0]+row[1]*(*logs)[1]+row[2]*(*logs)[2]-shift,19);need(times(Point(Pt{1,8}),k,7)==source[target],"original source scalar check");recovered=k;break;}R=plus(R,G,A);
  }need(recovered>=0,"target not recovered despite full setup");rec["status"]="recovered_original";rec["shifts"]=shifts;rec["scalar"]=recovered;recoveries.push_back(rec);
 }
 J b=J::array();for(Pt p:base)b.push_back(pj(p));return J{{"role",role},{"policy",id},{"base",b},{"cap",cap},{"queries",cells},{"recoveries",recoveries},{"cold_oracle_calls",calls},{"cold_lambda_evaluations",le},{"cold_base_root_evaluations",xe},{"cold_max_expanded_degree",maxD<0?J(nullptr):J(maxD)}};
}
std::string bytes(const std::string&p){std::ifstream f(p,std::ios::binary);need(bool(f),"read file");return {std::istreambuf_iterator<char>(f),{}};}std::string sha(const std::string&s){unsigned char b[32];SHA256((const unsigned char*)s.data(),s.size(),b);const char*h="0123456789abcdef";std::string r;for(auto x:b){r+=h[x>>4];r+=h[x&15];}return r;}
void write_new(const std::string&p,const J&v){need(!std::filesystem::exists(p),"fresh output required");std::ofstream f(p);need(bool(f),"create file");f<<v.dump()<<'\n';need(bool(f),"write file");}
J produce(){auto e=policies(7,19),t=policies(28,33);need(e.size()==84&&t.size()==84,"policy scope");auto ge=group(7,19),gt=group(28,33);for(Point a:ge){need(inverse(phi(a))==a,"inverse source");for(Point b:ge)need(phi(plus(a,b,7))==plus(phi(a),phi(b),28),"subgroup additivity");}for(Point a:gt)need(phi(inverse(a))==a,"inverse target");
 J route=J::array();for(Point q:ge)route.push_back({{"source",pj(q)},{"image",pj(phi(q))},{"back",pj(inverse(phi(q)))}});
 J cases=J::array();for(const std::string role:{"main","twist","matched","pullback"})for(int i=0;i<84;i++){
  Base b=role=="main"||role=="matched"?e[i]:t[i];if(role=="matched")for(Pt&x:b)x=*phi(x);if(role=="pullback")for(Pt&x:b)x=*inverse(x);
  for(int cap=0;cap<=8;cap++)cases.push_back(configuration(role,i,b,cap));
 }return J{{"schema","twist.chord-partial/v1"},{"fixture",{{"p",37},{"E",{7,19}},{"T",{28,33}},{"r",19},{"P",{1,8}},{"U",{32,29}}}},{"cases",cases},{"route",route}};
}
int main(int argc,char**argv){try{if(argc==2&&std::string(argv[1])=="--help"){std::cout<<"produce NEW.json | mutate INPUT NEW.json MODE\n";return 0;}if(argc==3&&std::string(argv[1])=="produce"){write_new(argv[2],produce());return 0;}if(argc==5&&std::string(argv[1])=="mutate"){J v=J::parse(bytes(argv[2])),old=v;std::string m=argv[4];if(m=="source")v["fixture"]["E"][0]=8;else if(m=="map")v["route"][1]["image"][0]=0;else if(m=="missing-case")v["cases"].erase(v["cases"].end()-1);else{
  bool done=false;for(auto&c:v["cases"]){if(m=="cap"){c["cap"]=9;done=true;break;}if(m=="columns"){c["base"][0][0]=0;done=true;break;}for(auto&q:c["queries"]){if(m=="rank"){q["rank"]=4;done=true;break;}if(m=="row"&&!q["rows"].empty()){q["rows"][0][0]=q["rows"][0][0].get<int>()+1;done=true;break;}if(m=="diagonal"){auto &ww=q["witnesses"];auto wi=std::find_if(ww.begin(),ww.end(),[](const J&z){return z[0].get<int>()==z[1].get<int>();});if(wi!=ww.end()){ww.erase(wi);done=true;break;}}if(m=="missing-failed-query"&&q["distinct"]["status"]=="unknown"){auto qi=std::find(c["queries"].begin(),c["queries"].end(),q);c["queries"].erase(qi);done=true;break;}if(m=="witness"&&q["distinct"]["status"]=="certified"){auto &av=q["distinct"]["a"];if(av.empty())av.push_back(1);else av[0]=f(av[0].get<int>()+1);done=true;break;}if(m=="root"&&q["distinct"].contains("roots")){q["distinct"]["roots"].push_back(37);done=true;break;}}if(done)break;for(auto&r:c["recoveries"]){if(r["status"]!="recovered_original")continue;if(m=="log"){r["logs"][0]=f(r["logs"][0].get<int>()+1,19);done=true;break;}if(m=="scalar"){r["scalar"]=f(r["scalar"].get<int>()+1,19);done=true;break;}}if(done)break;
  }need(done,"inapplicable mutation");}need(v!=old,"mutation unchanged");write_new(argv[3],v);return 0;}return 2;}catch(const std::exception&e){std::cerr<<e.what()<<'\n';return 101;}}
