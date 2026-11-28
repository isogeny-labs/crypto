#include <gmpxx.h>
#include <array>
#include <map>
#include <vector>
#include <queue>
#include <set>
#include <iostream>
#include <stdexcept>
#include <algorithm>
using M=std::array<int,4>;
struct Order { bool operator()(M a,M b)const {int da=0,db=0;for(int i=0;i<4;++i){da+=a[i];db+=b[i];}if(da!=db)return da>db;for(int i=3;i>=0;--i)if(a[i]!=b[i])return a[i]<b[i];return false;} };
using P=std::map<M,mpz_class,Order>;
mpz_class prime;
mpz_class mod(mpz_class a){mpz_mod(a.get_mpz_t(),a.get_mpz_t(),prime.get_mpz_t());return a;}
mpz_class inv(mpz_class a){mpz_class b;if(!mpz_invert(b.get_mpz_t(),a.get_mpz_t(),prime.get_mpz_t()))throw std::runtime_error("noninvertible");return b;}
bool divides(M a,M b){for(int i=0;i<4;++i)if(a[i]>b[i])return false;return true;}
M minus(M a,M b){for(int i=0;i<4;++i)a[i]-=b[i];return a;}
M plus(M a,M b){for(int i=0;i<4;++i)a[i]+=b[i];return a;}
void add(P& a,M m,mpz_class c){auto v=mod(a[m]+c);if(v==0)a.erase(m);else a[m]=v;}
P con(mpz_class c){P a;if(mod(c)!=0)a[M{}]=mod(c);return a;}
P var(int i){M m{};m[i]=1;return P{{m,1}};}
P operator+(P a,const P&b){for(auto&[m,c]:b)add(a,m,c);return a;}
P operator-(P a,const P&b){for(auto&[m,c]:b)add(a,m,-c);return a;}
P operator*(const P&a,const P&b){P r;for(auto&[m,c]:a)for(auto&[n,d]:b)add(r,plus(m,n),c*d);return r;}
P operator*(mpz_class c,P a){for(auto&[m,d]:a)d=mod(c*d);for(auto i=a.begin();i!=a.end();)if(i->second==0)i=a.erase(i);else ++i;return a;}
P pow(P a,int n){P b=con(1);while(n){if(n&1)b=b*a;a=a*a;n>>=1;}return b;}
P shift(P a,M m,mpz_class c){P b;for(auto&[n,d]:a)add(b,plus(n,m),c*d);return b;}
P normal(P a,const std::vector<P>&g){P r;while(!a.empty()){auto [m,c]=*a.begin();bool reduced=false;for(auto&b:g)if(divides(b.begin()->first,m)){a=a-shift(b,minus(m,b.begin()->first),c);reduced=true;break;}if(!reduced){r[m]=c;a.erase(a.begin());}}return r;}
struct Pair {int i,j,degree;M l;};
struct PairOrder{bool operator()(const Pair&a,const Pair&b)const{return a.degree!=b.degree?a.degree>b.degree:Order{}(b.l,a.l);}};
std::vector<P> gb(const std::vector<P>& eq){
 std::vector<P>g;std::priority_queue<Pair,std::vector<Pair>,PairOrder>q;std::set<std::pair<int,int>>done;
 auto insert=[&](P a){a=normal(a,g);if(a.empty())return; a=inv(a.begin()->second)*a;int j=static_cast<int>(g.size());for(int i=0;i<j;++i){M l{};bool disjoint=true;int d=0;for(int k=0;k<4;++k){l[k]=std::max(g[i].begin()->first[k],a.begin()->first[k]);d+=l[k];if(g[i].begin()->first[k]&&a.begin()->first[k])disjoint=false;}if(disjoint)done.insert({i,j});else q.push({i,j,d,l});}g.push_back(a);std::cerr<<"basis "<<g.size()<<" terms "<<a.size()<<" pairs "<<q.size()<<'\n';};
 for(auto&a:eq)insert(a);
 while(!q.empty()){auto t=q.top();q.pop();bool chain=false;for(int k=0;k<static_cast<int>(g.size());++k)if(k!=t.i&&k!=t.j&&divides(g[k].begin()->first,t.l)&&done.count(std::minmax(t.i,k))&&done.count(std::minmax(t.j,k))){chain=true;break;}if(!chain){P s=shift(g[t.i],minus(t.l,g[t.i].begin()->first),1)-shift(g[t.j],minus(t.l,g[t.j].begin()->first),1);insert(s);}done.insert({t.i,t.j});if(g.size()>2000)throw std::runtime_error("basis size bound");}
 return g;
}
int main(int argc,char**argv){try{if(argc!=4)throw std::runtime_error("usage: reconstruct prime b d");prime=mpz_class(argv[1]);mpz_class b(argv[2]),d(argv[3]);if(prime<5)throw std::runtime_error("characteristic");
 // Raw D(u)=u(u-1)[(u+d)^3-3(u+d)(u-1)^2+b(u-1)^3].
 mpz_class a=mod(b-2);if(a==0)throw std::runtime_error("singular normalization");
 std::array<mpz_class,6> c{0, -d*d*d+3*d+b,d*d*d-3*d*d-9*d-4*b+3,3*d*d+6*d+6*b-9,8-4*b,b-2};
 std::array<mpz_class,6> f{};for(int i=0;i<6;++i){mpz_class s=1;for(int j=0;j<4-i;++j)s=mod(s*a);f[i]=i==5?mpz_class(1):mod(c[i]*s);}if(f[1]==0)throw std::runtime_error("zero f1");
 P w=var(0),u=var(1),k=var(2),h=var(3),lam=2*w-3*u;
 P z=inv(2)*(f[4]*lam-w*w+3*(u*u)+3*(k*k));
 std::vector<P> eq{2*pow(k,3)+2*(w*z)-pow(u,3)-6*(u*k*k)-f[3]*lam,z*z+2*(w*pow(k,3))-3*pow(k,4)-3*(u*u*k*k)-f[2]*lam,2*(z*pow(k,3))-3*(u*pow(k,4))-f[1]*lam,h*lam-con(1)};
 auto g=gb(eq);
 // Recheck every input and S-polynomial after completing the basis.
 for(auto&e:eq)if(!normal(e,g).empty())throw std::runtime_error("input remainder");
 for(size_t i=0;i<g.size();++i)for(size_t j=0;j<i;++j){M l{};for(int z0=0;z0<4;++z0)l[z0]=std::max(g[i].begin()->first[z0],g[j].begin()->first[z0]);if(!normal(shift(g[i],minus(l,g[i].begin()->first),1)-shift(g[j],minus(l,g[j].begin()->first),1),g).empty())throw std::runtime_error("S remainder");}
 std::set<M>basis{M{}};std::vector<M>todo{M{}};while(!todo.empty()){M m=todo.back();todo.pop_back();for(int i=0;i<4;++i){M n=m;++n[i];bool std=true;for(auto&t:g)if(divides(t.begin()->first,n)){std=false;break;}if(std&&basis.insert(n).second)todo.push_back(n);if(basis.size()>80)throw std::runtime_error("dimension bound");}}
 if(basis.size()!=40)throw std::runtime_error("not dimension40");
 std::vector<M>bb(basis.begin(),basis.end());std::map<M,int>index;for(int i=0;i<40;++i)index[bb[i]]=i;
 using V=std::vector<mpz_class>;auto vectorize=[&](P a){V v(40);a=normal(a,g);for(auto&[m,c0]:a){if(!index.count(m))throw std::runtime_error("nonstandard remainder");v[index.at(m)]=c0;}return v;};
 std::vector<V>mk;for(auto&m:bb)mk.push_back(vectorize(P{{plus(m,M{0,0,1,0}),1}}));
 V v=vectorize(con(1));std::vector<V>aug(40,V(44));for(int j=0;j<40;++j){for(int i=0;i<40;++i)aug[i][j]=v[i];V next(40);for(int i=0;i<40;++i)for(int z0=0;z0<40;++z0)next[z0]+=v[i]*mk[i][z0];for(auto&x:next)x=mod(x);v=next;}
 std::array<V,4>rhs{v,vectorize(w),vectorize(u),vectorize(lam)};for(int i=0;i<40;++i)for(int j=0;j<4;++j)aug[i][40+j]=rhs[j][i];
 for(int j=0;j<40;++j){int pivot=j;while(pivot<40&&aug[pivot][j]==0)++pivot;if(pivot==40)throw std::runtime_error("k not primitive");std::swap(aug[j],aug[pivot]);mpz_class scale=inv(aug[j][j]);for(int k0=j;k0<44;++k0)aug[j][k0]=mod(aug[j][k0]*scale);for(int i=0;i<40;++i)if(i!=j&&aug[i][j]!=0){mpz_class z0=aug[i][j];for(int k0=j;k0<44;++k0)aug[i][k0]=mod(aug[i][k0]-z0*aug[j][k0]);}}
 std::cout<<"{\"dimension\":40,\"p\":\""<<prime<<"\",\"b\":\""<<b<<"\",\"d\":\""<<d<<"\"";
 for(int j=0;j<4;++j){std::cout<<",\""<<(j==0?"Q":j==1?"W":j==2?"U":"L")<<"\":[";for(int i=0;i<40;++i){if(i)std::cout<<',';std::cout<<'"'<<(j==0?mod(-aug[i][40+j]):aug[i][40+j])<<'"';}if(j==0)std::cout<<",\"1\"";std::cout<<']';}std::cout<<"}\n";
 }catch(std::exception&e){std::cerr<<e.what()<<'\n';return 1;}}
