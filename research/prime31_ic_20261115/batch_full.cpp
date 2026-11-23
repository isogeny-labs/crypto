// Complete prime-field index-calculus solver with batched relation oracle.
// Same ordinary cofactor-one E/F_(2^31-1), complete cold cost including all failures.
#define INCLUDE_AS_LIBRARY
#include "batch_decompose.cpp"
#include <cmath>
static constexpr uint64_t NORD=2147446831ULL;
static uint64_t nalgebra=0;
uint64_t nadd(uint64_t x,uint64_t y){uint64_t z=x+y;return z>=NORD?z-NORD:z;}
uint64_t nsub(uint64_t x,uint64_t y){return x>=y?x-y:NORD+x-y;}
uint64_t nmul(uint64_t x,uint64_t y){++nalgebra;return x*y%NORD;}
uint64_t ninv(uint64_t x){if(!x)throw runtime_error("N inverse 0");int64_t t=0,u=1,r=NORD,s=x;while(s){int64_t q=r/s,z=t-q*u;t=u;u=z;z=r-q*s;r=s;s=z;}if(r!=1)throw runtime_error("nonunit");return (t%int64_t(NORD)+NORD)%NORD;}
struct SparseRank {
 size_t b,rank=0,pos=0,dependent=0; vector<vector<uint32_t>> rows;
 explicit SparseRank(size_t B):b(B),rows(B){}
 void submit(uint64_t known,Hit rel){
  vector<uint32_t> v(b+1,0);
  auto bump=[&](int idx,int sign){v[idx]=uint32_t(sign>0?nadd(v[idx],1):nsub(v[idx],1));};
  bump(rel.pair.i,rel.pair.si);bump(rel.pair.j,rel.pair.sj);bump(rel.k,rel.s);v[b]=known; ++pos;
  for(size_t j=0;j<b;++j){uint64_t coeff=v[j];if(!coeff)continue;
    if(rows[j].empty()){uint64_t t=ninv(coeff);for(size_t k=j;k<=b;++k)if(v[k])v[k]=uint32_t(nmul(v[k],t));rows[j]=move(v);++rank;return;}
    const auto &w=rows[j];for(size_t k=j;k<=b;++k)if(w[k])v[k]=uint32_t(nsub(v[k],nmul(coeff,w[k])));
  }if(v[b])throw runtime_error("contradictory relation");++dependent;
 }
 vector<uint32_t> solve() const {
  if(rank!=b)throw runtime_error("rank incomplete");
  vector<uint32_t> out(b);
  for(int j=int(b)-1;j>=0;--j){uint64_t v=rows[j][b];for(size_t k=j+1;k<b;++k)if(rows[j][k])v=nsub(v,nmul(rows[j][k],out[k]));out[j]=uint32_t(v);}return out;
 }
};
uint64_t getlog(Hit r,const vector<uint32_t>&logs){
 uint64_t v=0;auto upd=[&](int idx,int s){v=s>0?nadd(v,logs[idx]):nsub(v,logs[idx]);};
 upd(r.pair.i,r.pair.si);upd(r.pair.j,r.pair.sj);upd(r.k,r.s);return v;
}
uint64_t bsgs(Point q){
 uint64_t m=uint64_t(sqrtl((long double)NORD))+1;
 unordered_map<uint64_t,uint32_t> h;h.reserve(2*m);Point cur;
 for(uint64_t j=0;j<m;++j){h.emplace(key(cur),uint32_t(j));cur=add(cur,G);}
 Point step=neg(scalar(G,m));cur=q;
 for(uint64_t i=0;i<=m;++i){auto it=h.find(key(cur));if(it!=h.end()){
   uint64_t val=(i*m+it->second)%NORD;if(!(scalar(G,val)==q))throw runtime_error("BSGS mismatch");return val;}
  cur=add(cur,step);
 }throw runtime_error("BSGS failed");
}
int main(int argc,char **argv){
 try{
 size_t B=argc>1?stoul(argv[1]):384;uint64_t seed=argc>2?stoull(argv[2]):20261114,cap=argc>3?stoull(argv[3]):1000000;
 bool walk=(argc>4 && string(argv[4])=="walk");
 if(B<8||B>1500)throw runtime_error("bad B");
 if(!(scalar(G,NORD).o))throw runtime_error("generator invalid");
 mt19937_64 gen(seed);uint64_t secret=1+gen()%(NORD-1);Point q=scalar(G,secret);
 ct={};nalgebra=0;auto start=chrono::steady_clock::now();
 vector<Point> f=base(B);Table h=batched(f);SparseRank linear(B);
 uint64_t trials=0,walk_known=0,walk_step=0;Point walk_target,walk_delta;
 if(walk){walk_known=1+gen()%(NORD-1);walk_step=1+gen()%(NORD-1);
  walk_target=scalar(G,walk_known);walk_delta=scalar(G,walk_step);}
 while(linear.rank<B&&trials<cap){
  uint64_t known=walk?walk_known:1+gen()%(NORD-1);
  Point target=walk?walk_target:scalar(G,known);Hit rel=batched_query(target,f,h);++trials;
  if(rel.ok){if(!witness(target,f,rel))throw runtime_error("relation not a real group sum");linear.submit(known,rel);}
  if(walk){walk_known=nadd(walk_known,walk_step);walk_target=add(walk_target,walk_delta);}
 }
 if(linear.rank<B){cout<<"{\"status\":\"censored\",\"b\":"<<B<<",\"seed\":"<<seed<<",\"rank\":"<<linear.rank<<",\"trials\":"<<trials<<"}\n";return 2;}
 auto logs=linear.solve();for(size_t i=0;i<B;++i)if(!(scalar(G,logs[i])==f[i]))throw runtime_error("factor base scalar mismatch");
 Point r=q;uint64_t shifts=0,answer=0;Hit rel;
 for(;shifts<NORD;++shifts){rel=batched_query(r,f,h);if(rel.ok){answer=nsub(getlog(rel,logs),shifts%NORD);if(!(scalar(G,answer)==q))throw runtime_error("target scalar mismatch");break;}r=add(r,G);}
 if(shifts==NORD)throw runtime_error("individual log failed");
 auto stop=chrono::steady_clock::now();Cnt ic=ct;uint64_t linear_ops=nalgebra;
 ct={};auto bs0=chrono::steady_clock::now();uint64_t baseline=bsgs(q);auto bs1=chrono::steady_clock::now();Cnt bs=ct;
 if(baseline!=secret||answer!=secret)throw runtime_error("wrong answer");
 cout<<"{\"status\":\"complete\",\"p\":"<<P<<",\"order\":"<<NORD<<",\"b\":"<<B<<",\"seed\":"<<seed
   <<",\"target\":["<<q.x<<","<<q.y<<"],\"verified_log\":"<<answer
   <<",\"mode\":\""<<(walk?"walk":"scalar")<<"\",\"trials\":"<<trials<<",\"positive\":"<<linear.pos<<",\"rank\":"<<linear.rank
   <<",\"dependent\":"<<linear.dependent<<",\"target_shifts\":"<<shifts
   <<",\"ic_ms\":"<<chrono::duration<double,milli>(stop-start).count()
   <<",\"ic_adds\":"<<ic.add<<",\"ic_inversions\":"<<ic.inv<<",\"ic_fpmuls\":"<<ic.mul
   <<",\"linear_nmults\":"<<linear_ops<<",\"bsgs_ms\":"<<chrono::duration<double,milli>(bs1-bs0).count()
   <<",\"bsgs_adds\":"<<bs.add<<",\"bsgs_inversions\":"<<bs.inv<<",\"bsgs_fpmuls\":"<<bs.mul<<"}\n";
 return 0;
 }catch(const exception&e){cerr<<e.what()<<"\n";return 1;}
}
