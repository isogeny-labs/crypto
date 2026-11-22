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
void put(Poly&p,M m,F c){if(++work>200000&&budget)throw std::runtime_error("resource");F v=add(p[m],c);if(v)p[m]=v;else p.erase(m);}
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
bool sourceSplit=false;
std::vector<Poly> system(int g,int left,int right,P q){auto u=variable(0),v=variable(1),a0=variable(2),z=variable(3),a1=variable(4);F s=q.x.a,t=q.y.a;
 if(left<0||sourceSplit){auto xy=timesp(u,v),sum=plusp(u,v),dif=plusp(u,v,neg(1));auto f=plusp(scalep(powp(dif,2),mul(s,s)),scalep(plusp(timesp(sum,plusp(xy,constant(AA))),constant(scale(BB,2))),scale(s,-2)));f=plusp(f,powp(plusp(xy,constant(neg(AA))),2));f=plusp(f,scalep(sum,scale(BB,-4)));return {f,fb(g,left,u),fb(g,right,v)};}
 std::vector<Poly>in;Poly xx[2];int branches[]={left,right};Poly ls[]={v,z},as[]={a0,a1};
 for(int i=0;i<2;i++){int br=branches[i];xx[i]=br?plusp(powp(ls[i],2),scalep(plusp(as[i],constant(gammas[g])),neg(2))):ls[i];auto y=plusp(timesp(u,plusp(xx[i],constant(neg(s)))),constant(neg(t)));auto curve=plusp(plusp(plusp(powp(y,2),powp(xx[i],3),neg(1)),scalep(xx[i],neg(AA))),constant(neg(BB)));in.push_back(curve);
  if(!br){in.push_back(fb(g,0,xx[i]));in.push_back(as[i]);}
  else {F zz=scale(betas[g],br),dd=mul(delta,mul(zz,zz));auto f=plusp(powp(ls[i],4),scalep(timesp(xx[i],powp(ls[i],2)),neg(6)));f=plusp(f,scalep(powp(xx[i],2),neg(3)));f=plusp(f,scalep(timesp(y,ls[i]),neg(mod(8))));f=plusp(f,constant(scale(add(AA,dd),-4)));in.push_back(f);in.push_back(plusp(powp(as[i],7),as[i],neg(1)));}}
 in.push_back(plusp(plusp(xx[0],xx[1]),plusp(powp(u,2),constant(neg(s))),neg(1)));
 auto prod=plusp(plusp(scalep(powp(u,2),s),scalep(u,scale(t,2))),constant(add(mul(s,s),AA)));in.push_back(plusp(timesp(xx[0],xx[1]),prod,neg(1)));return in;
}
using Pair=std::array<int,4>;
struct Answer{std::set<Pair>rows;int calls=0,maximum=-1;long long terms=0,pairs=0;bool generic=false;};
P generator{{5,0},{11,0},false};
void verify_basis(const std::vector<Row>&b){
 for(unsigned i=0;i<input.size();i++){Row r{input[i],std::vector<Poly>(input.size())};r.w[i]=constant(1);guard(r);if(!normal(r,b).p.empty())throw std::logic_error("input normal form");}
 for(unsigned i=0;i<b.size();i++)for(unsigned j=0;j<i;j++){M k=lcm(b[i].p.rbegin()->first,b[j].p.rbegin()->first);Row r=subtract(move(b[i],k-b[i].p.rbegin()->first,1),move(b[j],k-b[j].p.rbegin()->first,1));if(!normal(r,b).p.empty())throw std::logic_error("S normal form");}
}
std::set<int> roots_of_base(int g,int br){std::set<int>x;if(br<0)for(int k=0;k<4;k++)x.insert(base[g][k].begin(),base[g][k].end());else x=base[g][br];return x;}
Answer branch(int g,int mode,int l,int r,P q,bool qualification=false){
 weighted=mode==2;sourceSplit=mode!=2;work=0;budget=false;
 input=qualification?std::vector<Poly>{plusp(variable(0),constant(neg(1))),plusp(variable(1),constant(neg(2)))}:system(g,l,r,q);
 cap=20;maxordinary=-1;maxwitness=-1;budget=true;std::vector<Row>b;std::string status="complete";
 try{b=solve();verify_basis(b);}catch(const std::runtime_error&e){status=e.what();if(status!="cap"&&status!="resource")throw;b.clear();}budget=false;
 std::cout<<"CASE "<<l<<' '<<r<<' '<<status<<' '<<maxordinary<<' '<<maxwitness<<' '<<work<<' '<<input.size()<<' '<<b.size()<<'\n';
 for(auto&p:input)std::cout<<"I "<<poly(p)<<'\n';for(auto&z:b){std::cout<<"B "<<poly(z.p);for(auto&w:z.w)std::cout<<' '<<poly(w);std::cout<<'\n';}
 Answer a;a.calls=1;a.terms=work;a.maximum=maxwitness;
 if(status=="complete"&&!qualification){auto left=roots_of_base(g,l),right=roots_of_base(g,r);for(P x:source)for(P y:source){if(x.o||y.o||!left.count(x.x.a)||!right.count(y.x.a))continue;a.pairs++;if(sum(x,y)==q)a.rows.insert({x.x.a,x.y.a,y.x.a,y.y.a});}}
 std::cout<<"ROOTS "<<a.rows.size()<<' '<<a.pairs<<'\n';for(auto x:a.rows)std::cout<<"R "<<x[0]<<' '<<x[1]<<' '<<x[2]<<' '<<x[3]<<'\n';std::cout<<"ENDCASE\n";return a;
}
std::vector<P> representatives(int g){std::vector<P>b;for(int x:roots_of_base(g,-1)){P p;for(P q:source)if(!q.o&&q.x.a==x&&(p.o||q.y.a<p.y.a))p=q;if(p.o)throw std::logic_error("base representative");b.push_back(p);}return b;}
std::vector<int> coefficient(Pair p,const std::vector<P>&b){std::vector<int>v(b.size());for(int side=0;side<2;side++){int x=p[side*2],y=p[side*2+1];bool found=false;for(unsigned i=0;i<b.size();i++)if(b[i].x.a==x){int sign=y==b[i].y.a?1:y==neg(b[i].y.a)?-1:0;if(!sign)throw std::logic_error("base sign");v[i]=(v[i]+sign+47)%47;found=true;break;}if(!found)throw std::logic_error("base column");}return v;}
int mod47(int n){n%=47;return n<0?n+47:n;}
int inv47(int x){if(!x)throw std::logic_error("coefficient zero");int a=x,r=1,k=45;for(;k;k>>=1,a=a*a%47)if(k&1)r=r*a%47;return r;}
struct Linear{int n;std::map<int,std::vector<int>>r;
 explicit Linear(int size):n(size){}
 void add(std::vector<int>v,int rhs){v.push_back(rhs);for(auto [p,row]:r){int c=v[p];for(int i=0;i<=n;i++)v[i]=mod47(v[i]-c*row[i]);}int pivot=-1;for(int i=0;i<n;i++)if(v[i]){pivot=i;break;}if(pivot<0){if(v[n])throw std::logic_error("inconsistent public relation");return;}int z=inv47(v[pivot]);for(int&i:v)i=i*z%47;for(auto&[p,row]:r){int c=row[pivot];for(int i=0;i<=n;i++)row[i]=mod47(row[i]-c*v[i]);}r[pivot]=v;}
 std::vector<int>logs(){if((int)r.size()!=n)return {};std::vector<int>x(n);for(auto [p,row]:r)x[p]=row[n];return x;}
};
void setup_public(int g){
 AA=1;BB=8;delta=8;source=points(AA,BB);auto twist=points(18,14);if(source.size()!=47||twist.size()!=53||!on(generator)||!scalar(generator,47).o||generator.o)throw std::logic_error("domain");
 for(int k=0;k<4;k++)base[g][k].clear();int pairs=0;
 for(P q:source)for(P t:twist){P p=sum(scalar(q,24),embed(t));pairs++;if(!on(p)||!(sum(p,frob(p))==q))throw std::logic_error("actual source trace");if(p.o||q.o||!grid(p,gammas[g],betas[g]))continue;int z=mul(p.x.b,inv(betas[g]));if(z/7)throw std::logic_error("grid branch");base[g][std::min(z,mod(-z))].insert(q.x.a);}
 for(int k=0;k<4;k++){std::cout<<"BASE "<<k<<' ';bool first=true;for(int x:base[g][k]){if(!first)std::cout<<',';first=false;std::cout<<x;}std::cout<<'\n';}std::cout<<"SETUP_PAIRS "<<pairs<<'\n';
}
int serial=0;
Answer query(int g,int mode,int phase,int label,P q,Linear&lin,const std::vector<P>&b){
 int count=q.o?0:mode==0?1:16;std::cout<<"CALL "<<serial++<<' '<<phase<<' '<<label<<' '<<(q.o?-1:q.x.a)<<' '<<(q.o?-1:q.y.a)<<' '<<count<<'\n';Answer all;
 auto merge=[&](const Answer&a){all.calls+=a.calls;all.terms+=a.terms;all.pairs+=a.pairs;all.maximum=std::max(all.maximum,a.maximum);all.rows.insert(a.rows.begin(),a.rows.end());};
 if(q.o){all.generic=true;for(P p:b){all.rows.insert({p.x.a,p.y.a,p.x.a,neg(p.y.a)});all.rows.insert({p.x.a,neg(p.y.a),p.x.a,p.y.a});}std::cout<<"IDENTITY "<<all.rows.size()<<'\n';for(auto p:all.rows)std::cout<<"R "<<p[0]<<' '<<p[1]<<' '<<p[2]<<' '<<p[3]<<'\n';}
 else if(mode==0)merge(branch(g,mode,-1,-1,q));else for(int l=0;l<4;l++)for(int r=0;r<4;r++)merge(branch(g,mode,l,r,q));
 if(phase==0)for(auto p:all.rows)lin.add(coefficient(p,b),label);
 std::cout<<"ENDCALL "<<lin.r.size()<<' '<<all.rows.size()<<'\n';return all;
}
void recover(P target,int g,int mode){
 setup_public(g);if(!on(target)||target.o)throw std::logic_error("target");auto b=representatives(g);Linear lin(b.size());serial=0;
 std::cout<<"COLUMNS "<<b.size();for(P p:b)std::cout<<' '<<p.x.a<<':'<<p.y.a;std::cout<<'\n';Answer total;
 auto charge=[&](const Answer&a){total.calls+=a.calls;total.terms+=a.terms;total.pairs+=a.pairs;total.maximum=std::max(total.maximum,a.maximum);};
 for(int m=1;m<47;m++){charge(query(g,mode,0,m,scalar(generator,m),lin,b));if(lin.r.size()==b.size())break;}
 auto logs=lin.logs();std::cout<<"LOGS "<<logs.size();for(int x:logs)std::cout<<' '<<x;std::cout<<'\n';
 std::string status="blocked";int value=-1,shift=-1;
 if(logs.size()==b.size()){for(unsigned i=0;i<b.size();i++)if(!(scalar(generator,logs[i])==b[i]))throw std::logic_error("derived public log");status="failed";
  for(int s=0;s<47;s++){auto a=query(g,mode,1,s,sum(target,scalar(generator,s)),lin,b);charge(a);if(a.rows.empty())continue;auto v=coefficient(*a.rows.begin(),b);int k=0;for(unsigned i=0;i<v.size();i++)k=mod47(k+v[i]*logs[i]);k=mod47(k-s);if(!(scalar(generator,k)==target))throw std::logic_error("original source scalar");status=a.generic?"generic":"algebraic";value=k;shift=s;break;}}
 std::cout<<"RESULT "<<status<<' '<<value<<' '<<shift<<' '<<serial<<' '<<total.calls<<' '<<total.terms<<' '<<total.pairs<<' '<<total.maximum<<' '<<lin.r.size()<<'\n';std::cout<<"ENDWORK\n";
}
int main(){try{
 AA=1;BB=8;delta=8;std::cout<<"TC1 7 3 8 1 8 47 53 20 200000 9\nQUAL\n";branch(-1,0,-1,-1,generator,true);std::cout<<"ENDQUAL\n";
 P target{{22,0},{12,0},false};int id=0;for(int g=0;g<3;g++)for(int mode=0;mode<3;mode++){std::cout<<"WORK "<<id++<<' '<<g<<' '<<mode<<'\n';recover(target,g,mode);}if(id!=9)throw std::logic_error("scope");
 }catch(const std::exception&e){std::cerr<<e.what()<<'\n';return 101;}}
