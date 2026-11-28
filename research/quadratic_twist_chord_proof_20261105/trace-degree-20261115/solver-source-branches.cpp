#define main preceding_fixture_main
#include "../trace-fixture-20261115/fixture.cpp"
#undef main
#include <map>
#include <set>
#include <deque>
#include <algorithm>
#include <sstream>
using M=unsigned;
int exponent(M m,int i){return (m>>(6*i))&63;}
int degree(M m,bool weighted=false){int n=0;for(int i=0;i<5;i++)n+=exponent(m,i)*(weighted&&(i==2||i==4)?2:1);return n;}
bool weighted=false;long long work=0;bool budget=false;
struct Order{bool operator()(M a,M b)const{int x=degree(a,weighted),y=degree(b,weighted);return x==y?a<b:x<y;}};
using Poly=std::map<M,F,Order>;
Poly constant(F c){Poly p;if(c)p[0]=c;return p;}
Poly variable(int i){return {{1u<<(6*i),1}};}
void put(Poly&p,M m,F c){if(++work>2000000&&budget)throw std::runtime_error("resource");F v=add(p[m],c);if(v)p[m]=v;else p.erase(m);}
Poly plusp(Poly a,const Poly&b,F s=1){for(auto [m,c]:b)put(a,m,mul(c,s));return a;}
Poly scalep(Poly a,F s){for(auto&[m,c]:a)c=mul(c,s);if(!s)a.clear();return a;}
Poly timesp(const Poly&a,const Poly&b){Poly p;for(auto [m,c]:a)for(auto [n,d]:b){for(int i=0;i<5;i++)if(exponent(m,i)+exponent(n,i)>63)throw std::runtime_error("exponent");put(p,m+n,mul(c,d));}return p;}
Poly powp(Poly a,int n){Poly p=constant(1);for(int i=0;i<n;i++)p=timesp(p,a);return p;}
int pd(const Poly&p,bool w){int k=-1;for(auto [m,c]:p)k=std::max(k,degree(m,w));return k;}
bool divides(M a,M b){for(int i=0;i<5;i++)if(exponent(a,i)>exponent(b,i))return false;return true;}
M lcm(M a,M b){M r=0;for(int i=0;i<5;i++)r|=unsigned(std::max(exponent(a,i),exponent(b,i)))<<(6*i);return r;}
Poly shifted(const Poly&p,M m,F s){Poly q;for(auto [n,c]:p)put(q,n+m,mul(c,s));return q;}
struct Row{Poly p;std::vector<Poly>w;};
std::vector<Poly> input;int cap,maxordinary,maxwitness;
int bound(const Row&r){int k=-1;for(unsigned i=0;i<r.w.size();i++)if(!r.w[i].empty())k=std::max(k,pd(r.w[i],weighted)+pd(input[i],weighted));return k;}
void guard(const Row&r,M shift=0){int b=bound(r)+degree(shift,weighted);if(b>cap||pd(r.p,weighted)+degree(shift,weighted)>cap)throw std::runtime_error("cap");maxwitness=std::max(maxwitness,b);maxordinary=std::max(maxordinary,pd(r.p,false)+degree(shift));}
Row move(const Row&r,M m,F s){guard(r,m);Row o{shifted(r.p,m,s),{}};for(auto&p:r.w)o.w.push_back(shifted(p,m,s));return o;}
Row subtract(Row a,const Row&b){a.p=plusp(a.p,b.p,neg(1));for(unsigned i=0;i<a.w.size();i++)a.w[i]=plusp(a.w[i],b.w[i],neg(1));guard(a);return a;}
Row normal(Row r,const std::vector<Row>&basis){while(!r.p.empty()){
 bool change=false;for(auto it=r.p.rbegin();it!=r.p.rend();++it){M m=it->first;F c=it->second;for(auto&b:basis){M lead=b.p.rbegin()->first;if(!divides(lead,m))continue;r=subtract(r,move(b,m-lead,mul(c,inv(b.p.rbegin()->second))));change=true;break;}if(change)break;}if(!change)break;
 }return r;}
Row monic(Row r){if(r.p.empty())return r;F v=inv(r.p.rbegin()->second);r.p=scalep(r.p,v);for(auto&p:r.w)p=scalep(p,v);return r;}
std::vector<Row> solve(){std::vector<Row>b;std::deque<std::pair<int,int>>pairs;
 for(unsigned i=0;i<input.size();i++){if(input[i].empty())continue;Row r{input[i],std::vector<Poly>(input.size())};r.w[i]=constant(1);guard(r);r=monic(normal(r,b));if(!r.p.empty()){for(unsigned j=0;j<b.size();j++)pairs.push_back({j,b.size()});b.push_back(r);}}
 while(!pairs.empty()){auto [i,j]=pairs.front();pairs.pop_front();M mi=b[i].p.rbegin()->first,mj=b[j].p.rbegin()->first;bool coprime=true;for(int v=0;v<5;v++)if(exponent(mi,v)&&exponent(mj,v))coprime=false;if(coprime)continue;M v=lcm(mi,mj);Row r=subtract(move(b[i],v-b[i].p.rbegin()->first,1),move(b[j],v-b[j].p.rbegin()->first,1));r=monic(normal(r,b));if(!r.p.empty()){if(b.size()>=1000)throw std::runtime_error("resource");for(unsigned k=0;k<b.size();k++)pairs.push_back({k,b.size()});b.push_back(r);}}
 return b;
}
std::string poly(const Poly&p){if(p.empty())return "-";std::ostringstream o;bool f=true;for(auto [m,c]:p){if(!f)o<<',';f=false;o<<m<<':'<<c;}return o.str();}
bool grid(P p,int gamma,int beta){return p.o||(sub(p.x.a,gamma)/7==0&&mul(p.x.b,inv(beta))/7==0);}
std::set<int> base[3][4];std::vector<P> source;int gammas[]={7,28,35},betas[]={10,11,8},ids[]={12,37,42};
Poly fb(int g,int branch,const Poly&x){std::set<int>xs;if(branch<0){for(int k=0;k<4;k++)xs.insert(base[g][k].begin(),base[g][k].end());}else xs=base[g][branch];Poly p=constant(1);for(int v:xs)p=timesp(p,plusp(x,constant(neg(v))));return p;}
std::vector<Poly> system(int g,int left,int right,P q){auto u=variable(0),v=variable(1),a0=variable(2),z=variable(3),a1=variable(4);F s=q.x.a,t=q.y.a;
 if(true){auto xy=timesp(u,v),sum=plusp(u,v),dif=plusp(u,v,neg(1));auto f=plusp(scalep(powp(dif,2),mul(s,s)),scalep(plusp(timesp(sum,plusp(xy,constant(AA))),constant(scale(BB,2))),scale(s,-2)));f=plusp(f,powp(plusp(xy,constant(neg(AA))),2));f=plusp(f,scalep(sum,scale(BB,-4)));return {f,fb(g,left,u),fb(g,right,v)};}
 std::vector<Poly>in;Poly xx[2];int branches[]={left,right};Poly ls[]={v,z},as[]={a0,a1};
 for(int i=0;i<2;i++){int br=branches[i];xx[i]=br?plusp(powp(ls[i],2),scalep(plusp(as[i],constant(gammas[g])),neg(2))):ls[i];auto y=plusp(timesp(u,plusp(xx[i],constant(neg(s)))),constant(neg(t)));auto curve=plusp(plusp(plusp(powp(y,2),powp(xx[i],3),neg(1)),scalep(xx[i],neg(AA))),constant(neg(BB)));in.push_back(curve);
  if(!br){in.push_back(fb(g,0,xx[i]));in.push_back(as[i]);}
  else {F zz=scale(betas[g],br),dd=mul(delta,mul(zz,zz));auto f=plusp(powp(ls[i],4),scalep(timesp(xx[i],powp(ls[i],2)),neg(6)));f=plusp(f,scalep(powp(xx[i],2),neg(3)));f=plusp(f,scalep(timesp(y,ls[i]),neg(mod(8))));f=plusp(f,constant(scale(add(AA,dd),-4)));in.push_back(f);in.push_back(plusp(powp(as[i],7),as[i],neg(1)));}}
 in.push_back(plusp(plusp(xx[0],xx[1]),plusp(powp(u,2),constant(neg(s))),neg(1)));
 auto prod=plusp(plusp(scalep(powp(u,2),s),scalep(u,scale(t,2))),constant(add(mul(s,s),AA)));in.push_back(plusp(timesp(xx[0],xx[1]),prod,neg(1)));return in;
}
int main(){try{AA=1;BB=8;delta=8;source=points(AA,BB);auto twist=points(18,14);if(source.size()!=47||twist.size()!=53)throw std::runtime_error("fixture");
 for(P q:source)for(P t:twist){P p=sum(scalar(q,24),embed(t));if(p.o||q.o)continue;for(int g=0;g<3;g++)if(grid(p,gammas[g],betas[g])){int b=mul(p.x.b,inv(betas[g]));if(b/7)throw std::runtime_error("membership");int br=std::min(b,mod(-b));base[g][br].insert(q.x.a);}}
 std::cout<<"TD2 7 3 8 1 8 47 153\n";
 for(int g=0;g<3;g++)for(int k=0;k<4;k++){std::cout<<"BASE "<<g<<' '<<k<<' ';bool first=true;for(int x:base[g][k]){if(!first)std::cout<<',';first=false;std::cout<<x;}std::cout<<'\n';}
 P q=source[1];std::cout<<"TARGET "<<q.x.a<<' '<<q.y.a<<'\n';int serial=0;
 auto run=[&](int g,int l,int r,int cp,bool control){weighted=false;work=0;budget=false;input=control?std::vector<Poly>{plusp(variable(0),constant(neg(1))),plusp(variable(1),constant(neg(2)))}:system(g,l,r,q);cap=cp;maxordinary=-1;maxwitness=-1;budget=true;std::vector<Row>b;std::string status="complete";try{b=solve();}catch(const std::runtime_error&e){status=e.what();b.clear();}budget=false;
  std::cout<<"CASE "<<serial++<<' '<<g<<' '<<l<<' '<<r<<' '<<cp<<' '<<status<<' '<<maxordinary<<' '<<maxwitness<<' '<<work<<' '<<input.size()<<' '<<b.size()<<'\n';for(auto&p:input)std::cout<<"I "<<poly(p)<<'\n';for(auto&row:b){std::cout<<"B "<<poly(row.p);for(auto&w:row.w)std::cout<<' '<<poly(w);std::cout<<'\n';}
  std::vector<std::array<int,4>>roots;int extraction_pairs=0;
  if(status=="complete"&&!control){auto contains=[&](int branch,int x){if(branch>=0)return base[g][branch].count(x)>0;for(int k=0;k<4;k++)if(base[g][k].count(x))return true;return false;};for(P x:source)for(P y:source){if(x.o||y.o||!contains(l,x.x.a)||!contains(r,y.x.a))continue;extraction_pairs++;if(sum(x,y)==q)roots.push_back({x.x.a,x.y.a,y.x.a,y.y.a});}}
  std::cout<<"ROOTS "<<roots.size()<<' '<<extraction_pairs<<'\n';for(auto r:roots)std::cout<<"R "<<r[0]<<' '<<r[1]<<' '<<r[2]<<' '<<r[3]<<'\n';std::cout<<"END\n";
 };
 run(-1,-1,-1,2,true);for(int g=0;g<3;g++)for(int cp:{14,16,20}){run(g,-1,-1,cp,false);for(int l=0;l<4;l++)for(int r=0;r<4;r++)run(g,l,r,cp,false);}if(serial!=154)throw std::runtime_error("coverage");
 }catch(const std::exception&e){std::cerr<<e.what()<<'\n';return 101;}}
