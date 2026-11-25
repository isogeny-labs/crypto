#include <algorithm>
#include <array>
#include <cassert>
#include <chrono>
#include <cmath>
#include <cstdint>
#include <fstream>
#include <iostream>
#include <map>
#include <set>
#include <stdexcept>
#include <unordered_map>
#include <vector>
using U=uint32_t;using W=uint64_t;using X=__uint128_t;using Clock=std::chrono::steady_clock;
W mix(W x){x=(x^(x>>30))*0xbf58476d1ce4e5b9ULL;x=(x^(x>>27))*0x94d049bb133111ebULL;return x^(x>>31);}
struct Hash{size_t operator()(W x)const{return mix(x);}};
U p,tw,a,b,c,zeta;W ell,mu,mu2,state;unsigned bits;
double seconds(Clock::time_point t){return std::chrono::duration<double>(Clock::now()-t).count();}
U add(U x,U y){W v=W(x)+y;return U(v>=p?v-p:v);}U neg(U x){return x?p-x:0;}U sub(U x,U y){return add(x,neg(y));}
U mul(U x,U y){W v=W(x)*y;v=(v&p)+(v>>bits);return U(v>=p?v-p:v);}
U power(U x,W n){U v=1;for(;n;n>>=1,x=mul(x,x))if(n&1)v=mul(v,x);return v;}
U inv(U x){assert(x);return power(x,p-2);}U f(U x){U xx=mul(x,x);return mul(tw,add(add(add(mul(xx,xx),mul(a,xx)),mul(b,x)),c));}
W ladd(W x,W y){X v=X(x)+y;return W(v>=ell?v-ell:v);}W lneg(W x){return x?ell-x:0;}W lmul(W x,W y){return W(X(x)*y%ell);}
W lpower(W x,W n){W v=1;for(;n;n>>=1,x=lmul(x,x))if(n&1)v=lmul(v,x);return v;}
W linv(W x){assert(x);if(x==1)return 1;if(x==mu)return mu2;if(x==mu2)return mu;return lpower(x,ell-2);}
struct Point{U x,y;};W key(Point P){return (W(P.x)<<32)|P.y;}Point point(W k){return {U(k>>32),U(k)};}
std::pair<Point,W> canonical(Point P){if(!P.y)return {P,0};U y1=mul(P.y,zeta),y2=mul(y1,zeta);U ymin=std::min({P.y,y1,y2});W co=ymin==P.y?1:ymin==y1?mu2:mu;return {{P.x,ymin},co};}
W draw(){state+=0x9e3779b97f4a7c15ULL;return mix(state);}
struct Item{W id,co;};
struct Terms{std::array<Item,4> v{};unsigned n=0;void put(W id,W co){for(unsigned i=0;i<n;++i)if(v[i].id==id){v[i].co=ladd(v[i].co,co);return;}assert(n<4);v[n++]={id,co};}void trim(){unsigned k=0;for(unsigned i=0;i<n;++i)if(v[i].co)v[k++]=v[i];n=k;}};
struct PairStream{W size,mask,index=0;unsigned half;std::array<W,6> keys;
 PairStream(W n,W seed):size(n*(n-1)/2){unsigned len=0;for(W t=size;t;t>>=1)++len;half=std::max(1u,(len+1)/2);mask=(W(1)<<half)-1;for(unsigned i=0;i<6;++i)keys[i]=mix(seed+i*0x9e3779b97f4a7c15ULL);}
 W permute(W v)const{W l=v>>half,r=v&mask;for(W k:keys){W t=l^(mix(r+k)&mask);l=r;r=t;}return (l<<half)|r;}
 bool next(U&i,U&j,W&rank){if(index>=size)return false;rank=permute(index++);while(rank>=size)rank=permute(rank);W ii=W((1+std::sqrt(double(1+8*rank)))/2);while(ii*(ii-1)/2>rank)--ii;while((ii+1)*ii/2<=rank)++ii;i=U(ii);j=U(rank-ii*(ii-1)/2);assert(j<i);return true;}
};
struct Query{U i,j,phase,dx,yi,yj;};
struct Node{W point,parent_co;int parent;std::array<U,3> fb{};std::array<W,3> co{};unsigned n,depth;U i,j,phase;};
struct Closed{U i,j,phase;std::map<U,W> row;};
void accumulate(std::map<U,W>&row,U i,W v){W z=ladd(row[i],v);if(z)row[i]=z;else row.erase(i);}
bool line(Point P,Point Q,U inverse_dx,U inverse2tw,std::array<Point,4>&points){
 U m=mul(sub(P.y,Q.y),inverse_dx),bb=sub(P.y,mul(m,P.x));
 U cc3=neg(mul(mul(m,m),m)),cc2=sub(mul(tw,a),mul(3,mul(mul(m,m),bb))),cc1=sub(mul(tw,b),mul(3,mul(m,mul(bb,bb)))),cc0=sub(mul(tw,c),mul(mul(bb,bb),bb));
 U q32=add(cc3,mul(P.x,tw)),q31=add(cc2,mul(P.x,q32)),q30=add(cc1,mul(P.x,q31));assert(add(cc0,mul(P.x,q30))==0);
 U q21=add(q32,mul(Q.x,tw)),q20=add(q31,mul(Q.x,q21));assert(add(q30,mul(Q.x,q20))==0);
 U disc=sub(mul(q21,q21),mul(mul(4,tw),q20)),root=power(disc,(W(p)+1)/4);if(mul(root,root)!=disc)return false;
 U r=mul(add(neg(q21),root),inverse2tw),s=mul(sub(neg(q21),root),inverse2tw);
 points={P,Q,Point{r,add(mul(m,r),bb)},Point{s,add(mul(m,s),bb)}};return true;
}
int main(int argc,char**argv){try{
 if(argc!=3)throw std::runtime_error("usage: collector input.txt output.json");std::ifstream input(argv[1]);W seed,cap;U small,tree_limit,fixed,count;
 input>>p>>tw>>a>>b>>c>>ell>>mu>>zeta>>seed>>small>>tree_limit>>cap>>fixed>>count;if(!input)throw std::runtime_error("bad input");
 bits=0;for(W n=W(p)+1;n>1;n>>=1)++bits;assert(p==((W(1)<<bits)-1)&&p%4==3&&(p-1)%9==0&&((p-1)/9)%3!=0);
 assert(zeta!=1&&power(zeta,3)==1);mu2=lmul(mu,mu);assert(mu!=1&&lmul(mu2,mu)==1&&ladd(ladd(1,mu),mu2)==0);state=seed;
 auto total=Clock::now();std::vector<Point> fb;std::unordered_map<W,U,Hash> index;index.reserve(2*small);
 auto insert=[&](Point P){assert(power(P.y,3)==f(P.x));auto C=canonical(P);if(!C.second)return;W k=key(C.first);if(index.count(k))return;index[k]=U(fb.size());fb.push_back(C.first);};
 for(U i=0;i<count;++i){Point P;input>>P.x>>P.y;assert(input);insert(P);}U root9=1;W m9=(p-1)/9;
 for(U base=2;root9==1||power(root9,3)==1;++base){assert(base<1000);root9=power(base,m9);}assert(power(root9,9)==1);
 W d=(m9%3==1?(2*m9+1)/3:(m9+1)/3);assert(3*d%m9==1);W samples=0;
 while(fb.size()<small){assert(!fixed);++samples;U xx=U(draw()%p),value=f(xx);if(!value)continue;U yy=power(value,d);bool found=false;
  for(unsigned j=0;j<3;++j){if(power(yy,3)==value){insert({xx,yy});found=true;break;}yy=mul(yy,root9);}if(!found)continue;
 }assert(fb.size()==small);double setup=seconds(total);auto start=Clock::now();U inv2tw=inv(mul(2,tw));
 std::unordered_map<W,U,Hash> tree_index;tree_index.reserve(2*tree_limit+16);std::vector<Node> nodes;nodes.reserve(tree_limit);std::vector<Closed> closed;std::set<std::vector<std::pair<U,W>>> seen_rows;
 W pairs=0,split=0,fp=0,pp=0,full=0,disconnected=0,duplicates=0;PairStream stream(small,seed+111);U phase_pos=3,ii=0,jj=0,phase_start=0;W rank=0;bool exhausted=false;
 std::vector<Query> batch;std::vector<U> prefix;const unsigned BLOCK=4096;
 while(pairs<cap&&closed.size()<size_t(small)+128&&!exhausted){batch.clear();prefix.clear();prefix.push_back(1);
  while(batch.size()<BLOCK&&pairs+batch.size()<cap){if(phase_pos==3){if(!stream.next(ii,jj,rank)){exhausted=true;break;}phase_pos=0;phase_start=U(mix(rank+seed)%3);}U phase=(phase_start+phase_pos++)%3;Point P=fb[ii],Q=fb[jj];Q.y=mul(Q.y,power(zeta,phase));U dx=sub(P.x,Q.x);assert(dx);batch.push_back({ii,jj,phase,dx,P.y,Q.y});prefix.push_back(mul(prefix.back(),dx));}
  if(batch.empty())break;U inverse=inv(prefix.back());std::vector<U> inverses(batch.size());for(size_t q=batch.size();q-->0;){inverses[q]=mul(inverse,prefix[q]);inverse=mul(inverse,batch[q].dx);}
  for(size_t q=0;q<batch.size()&&closed.size()<size_t(small)+128;++q){const auto Q=batch[q];++pairs;std::array<Point,4> points;if(!line({fb[Q.i].x,Q.yi},{fb[Q.j].x,Q.yj},inverses[q],inv2tw,points))continue;++split;
   Terms fs,ls;for(Point P:points){auto C=canonical(P);if(!C.second)continue;W k=key(C.first);auto at=index.find(k);if(at!=index.end())fs.put(at->second,C.second);else ls.put(k,C.second);}fs.trim();ls.trim();assert(ls.n<=2);
   if(ls.n==0)++full;else if(ls.n==1)++fp;else ++pp;
   int old=-1,newer=-1;unsigned old_count=0;for(unsigned k=0;k<ls.n;++k){if(tree_index.count(ls.v[k].id)){old=int(k);++old_count;}else newer=int(k);}
   if(nodes.size()<tree_limit&&newer>=0&&((ls.n==1)||(ls.n==2&&old_count==1))){Item child=ls.v[unsigned(newer)];W ci=linv(child.co);Node node{};node.point=child.id;node.parent=old>=0?int(tree_index.at(ls.v[unsigned(old)].id)):-1;node.parent_co=old>=0?lneg(lmul(ls.v[unsigned(old)].co,ci)):0;node.depth=node.parent<0?1:nodes[unsigned(node.parent)].depth+1;
    node.n=fs.n;assert(fs.n<=3);for(unsigned k=0;k<fs.n;++k){node.fb[k]=U(fs.v[k].id);node.co[k]=lneg(lmul(fs.v[k].co,ci));}node.i=Q.i;node.j=Q.j;node.phase=Q.phase;tree_index[child.id]=U(nodes.size());nodes.push_back(node);continue;}
   if(old_count!=ls.n){++disconnected;continue;}
   std::map<U,W> row;for(unsigned k=0;k<fs.n;++k)accumulate(row,U(fs.v[k].id),fs.v[k].co);
   for(unsigned k=0;k<ls.n;++k){int id=int(tree_index.at(ls.v[k].id));W factor=ls.v[k].co;while(id>=0){const Node&n=nodes[unsigned(id)];for(unsigned h=0;h<n.n;++h)accumulate(row,n.fb[h],lmul(factor,n.co[h]));factor=lmul(factor,n.parent_co);id=n.parent;}}
   if(row.empty()){++duplicates;continue;}W normal=linv(row.begin()->second);for(auto&v:row)v.second=lmul(v.second,normal);
   std::vector<std::pair<U,W>> row_key(row.begin(),row.end());if(!seen_rows.insert(row_key).second){++duplicates;continue;}closed.push_back({Q.i,Q.j,Q.phase,std::move(row)});
  }
  if(pairs&&pairs%10000000<BLOCK)std::cerr<<"pairs "<<pairs<<" tree "<<nodes.size()<<" rows "<<closed.size()<<" seconds "<<seconds(start)<<'\n';
 }
 double collection=seconds(start);std::ofstream out(argv[2]);out.precision(15);out<<"{\"p\":"<<p<<",\"ell\":"<<ell<<",\"small\":"<<small<<",\"tree_limit\":"<<tree_limit<<",\"pair_cap\":"<<cap<<",\"pairs\":"<<pairs<<",\"split_lines\":"<<split<<",\"full\":"<<full<<",\"FP\":"<<fp<<",\"PP\":"<<pp<<",\"disconnected\":"<<disconnected<<",\"zero_rows\":"<<duplicates<<",\"fb_sampling_attempts\":"<<samples<<",\"setup_seconds\":"<<setup<<",\"collection_seconds\":"<<collection<<",\"total_seconds_excluding_output\":"<<seconds(total)<<",\"status\":\""<<(closed.size()>=size_t(small)+128?"collected":"incomplete")<<"\",\"points\":[";
 for(size_t i=0;i<fb.size();++i){if(i)out<<',';out<<'['<<fb[i].x<<','<<fb[i].y<<']';}out<<"],\"tree\":[";
 for(size_t i=0;i<nodes.size();++i){if(i)out<<',';const Node&n=nodes[i];out<<"{\"point\":["<<point(n.point).x<<','<<point(n.point).y<<"],\"parent\":"<<n.parent<<",\"parent_co\":"<<n.parent_co<<",\"depth\":"<<n.depth<<",\"anchors\":["<<n.i<<','<<n.j<<','<<n.phase<<"],\"fb\":[";for(unsigned j=0;j<n.n;++j){if(j)out<<',';out<<'['<<n.fb[j]<<','<<n.co[j]<<']';}out<<"]}";}out<<"],\"closed\":[";
 for(size_t i=0;i<closed.size();++i){if(i)out<<',';const auto&r=closed[i];out<<"{\"anchors\":["<<r.i<<','<<r.j<<','<<r.phase<<"],\"row\":[";bool first=true;for(auto v:r.row){if(!first)out<<',';first=false;out<<'['<<v.first<<','<<v.second<<']';}out<<"]}";}out<<"],\"novelty_claim\":false,\"full50x_claim\":false}\n";
 std::cerr<<"completed p "<<p<<" pairs "<<pairs<<" tree "<<nodes.size()<<" rows "<<closed.size()<<" total "<<seconds(total)<<'\n';
 }catch(const std::exception&e){std::cerr<<e.what()<<'\n';return 1;}return 0;}
