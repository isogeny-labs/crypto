#include <gmpxx.h>
#include <algorithm>
#include <iostream>
#include <map>
#include <stdexcept>
#include <string>
#include <vector>
using Q=mpq_class; using Z=mpz_class; using P=std::vector<Q>;
P add(P a,const P& b){a.resize(std::max(a.size(),b.size()));for(size_t i=0;i<b.size();++i)a[i]+=b[i];return a;}
P scale(P a,Q c){for(auto&v:a)v*=c;return a;}
P mul(const P&a,const P&b,size_t cap=100000){if(a.empty()||b.empty())return {};P c(std::min(cap,a.size()+b.size()-1));for(size_t i=0;i<a.size();++i)for(size_t j=0;j<b.size()&&i+j<c.size();++j)c[i+j]+=a[i]*b[j];return c;}
Q ratio(int n,int d){Q x(n,d);x.canonicalize();return x;}
P power(P a,int n,size_t cap=100000){P b{1};while(n){if(n&1)b=mul(b,a,cap);n/=2;if(n)a=mul(a,a,cap);}return b;}
P chart(const P&f,int t,int alpha,int q){P v;for(auto i=f.rbegin();i!=f.rend();++i){v=mul(v,P{Q(t),Q(alpha)},q);if(v.empty())v.resize(1);v[0]+=*i;}v.resize(q);return v;}
int val(int t){if(!t)throw std::runtime_error("v2 zero");int v=0;while(t%2==0){t/=2;++v;}return v;}
Z mod(Z x,const Z&m){mpz_mod(x.get_mpz_t(),x.get_mpz_t(),m.get_mpz_t());return x;}
Z residue(const Q&x,const Z&m){Z inv;if(!mpz_invert(inv.get_mpz_t(),x.get_den().get_mpz_t(),m.get_mpz_t()))throw std::runtime_error("nonintegral rational");return mod(x.get_num()*inv,m);}
std::string poly(const P&f){std::string s;for(const auto&x:f){if(!s.empty())s+=",";s+=x.get_str();}return s;}
struct Node{int t,q;P f,d;Q e;};
struct Fixture{int a,u,M,w,place;P f;std::vector<Node> nodes;int target;};
Fixture make(int a,int u,int M,int w,int place){
 int L=1<<a,alpha=2*L,h=(3*M+u-1)/u,full=(1<<M)-1;
 std::vector<int> grid;for(int s=-L;s<L;++s)grid.push_back(s);
 Q norm=1;for(int s:grid)if(s)norm*=Q(-s);
 std::map<int,P> b;P D;
 for(int d:grid){P v{1};for(int s:grid)if(s!=d)v=mul(v,P{Q(-s),1});v=scale(v,1/norm);b[d]=v;D=add(D,v);}
 if(D.empty()||D[0]!=1)throw std::runtime_error("D(0)");
 std::vector<std::pair<int,int>> A,B;
 int target;
 if(w==0){A={{0,1}};B={{0,1}};target=1;}
 else if(w==1){A={{0,2},{full,-1}};B={{0,-3},{full,5}};target=-11;}
 else{A={{1,1},{2,-2}};B={{4,3},{full,1}};target=0;}
 std::vector<std::vector<P>> N(M,std::vector<P>(M));
 for(int r=0;r<M;++r)for(int c=0;c<M;++c){int rp=place?r%2:0,cp=place?c%2:0,rs=place?r/2:r,cs=place?c/2:c;if(rp==cp)N[r][c]=mul(b.at(cs-rs),b.at(cs-rs));}
 P F;
 for(auto [y,ay]:A)for(auto [x,bx]:B){P term{Q(ay*bx)};for(int r=0;r<M;++r){P n;for(int c=0;c<M;++c)if((x>>c)&1)n=add(n,N[r][c]);term=mul(term,((y>>r)&1)?n:add(D,scale(n,-1)));}F=add(F,term);}
 if(F.empty()||F[0]!=target)throw std::runtime_error("dense origin");
 std::map<int,int> orders;for(int t:grid)orders[t]=t?h*std::max(0,u-val(t)):0;
 int sumq=0;for(auto [t,q]:orders)sumq+=q;
 for(size_t j=sumq;j<F.size();++j)if(F[j]!=0)throw std::runtime_error("degree margin");
 Fixture f{a,u,M,w,place,F,{},target};
 for(auto [t,q]:orders)if(q){P d{1};Q e=1;for(auto [s,qs]:orders)if(s!=t&&qs){d=mul(d,power(P{1,ratio(alpha,t-s)},qs,q),q);Q fac(-s,t-s);fac.canonicalize();for(int j=0;j<qs;++j)e*=fac;}d.resize(q);auto fj=chart(F,t,alpha,q);for(const Q&x:fj)if(mpz_even_p(x.get_den().get_mpz_t()))throw std::runtime_error("nonintegral input chart");for(const Q&x:d)if(mpz_even_p(x.get_den().get_mpz_t()))throw std::runtime_error("nonintegral basis chart");f.nodes.push_back({t,q,fj,d,e});}
 return f;
}
Q recover(const Fixture&f,int p,int lift,int defect){
 int L=1<<f.a,alpha=2*L,h=(3*f.M+f.u-1)/f.u;Q out=0;
 for(const auto&n:f.nodes){int pt=p+8*h*(1<<(f.a-val(n.t)))+8*f.M*(f.a+1);Z modulus=Z(1)<<pt;std::vector<Z> ds(n.q),fs(n.q),inv(n.q),ys(n.q);for(int j=0;j<n.q;++j){ds[j]=residue(n.d[j],modulus);fs[j]=residue(n.f[j],modulus);}if(ds[0]!=1)throw std::runtime_error("basis constant");inv[0]=1;
  for(int j=1;j<n.q;++j){Z s=0;for(int k=1;k<=j;++k)s+=ds[k]*inv[j-k];inv[j]=mod(-s,modulus);}
  for(int j=0;j<n.q;++j){Z s=0;for(int k=0;k<=j;++k)s+=fs[k]*inv[j-k];ys[j]=mod(s,modulus);if(lift)ys[j]+=modulus*((n.t+2*j+2*L)%5-2);if(defect==1)ys[j]=0;if(defect==2&&n.t==1&&j==0)ys[j]+=1;}
  Q x(-n.t,alpha);x.canonicalize();Q value=0;for(auto j=ys.rbegin();j!=ys.rend();++j)value=value*x+Q(*j);out+=n.e*value;
 }
 return out;
}
void row(const Fixture&f,int p,int lift,int defect){Q r=recover(f,p,lift,defect);bool integral=mpz_odd_p(r.get_den().get_mpz_t());bool equal=false;if(integral)equal=residue(r,Z(1)<<p)==mod(Z(f.target),Z(1)<<p);bool ok=defect==1?!equal:defect==2?!integral:equal;
 std::cout<<(defect?"control":"case")<<'\t'<<f.a<<'\t'<<f.u<<'\t'<<f.M<<'\t'<<f.w<<'\t'<<f.place<<'\t'<<p<<'\t'<<lift<<'\t'<<defect<<'\t'<<f.target<<'\t'<<r.get_num()<<'\t'<<r.get_den()<<'\t'<<ok<<'\n'<<std::flush;
 if(!ok){std::cerr<<"FAIL polynomial="<<poly(f.f)<<'\n';for(const auto&n:f.nodes)std::cerr<<"node="<<n.t<<" q="<<n.q<<" input="<<poly(n.f)<<" basis="<<poly(n.d)<<" E0="<<n.e<<'\n';throw std::runtime_error("recovery acceptance");}}
int main(){try{
 std::cout<<"kind\ta\tu\tM\tweights\tplacement\tp\tlift\tdefect\ttarget\tnumerator\tdenominator\taccepted\n";
 for(auto c:std::vector<std::vector<int>>{{3,3,3},{3,3,4},{3,3,5},{4,3,4},{4,4,4}})for(int w=0;w<3;++w)for(int place=0;place<2;++place){auto f=make(c[0],c[1],c[2],w,place);for(int p:{1,4,9})for(int lift=0;lift<2;++lift)row(f,p,lift,0);}
 auto f=make(3,3,3,0,0);row(f,4,0,1);row(f,4,0,2);
 bool affine=(Q(3)-Q(2)==1)&&(Q(1)-Q(2)!=1);std::cout<<"affine\t3\t3\t3\t0\t0\t4\t0\t3\t1\t-1\t1\t"<<affine<<'\n';if(!affine)return 3;
 }catch(const std::exception&e){std::cerr<<e.what()<<'\n';return 2;}return 0;}
