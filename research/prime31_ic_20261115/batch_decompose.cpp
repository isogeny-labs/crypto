// Full signed-three-summand relation oracle: reference vs batch-inverted query.
// Not a standalone discrete-log solver; compare against prime31_ic.cpp for full cold cost.
#define main pair_setup_main
#include "batch_pairs.cpp"
#undef main
constexpr uint64_t N=2147446831ULL;
static const Point G={625037715,2100627251,false};
Point scalar(Point q,uint64_t k){Point r;for(;k;k>>=1,q=add(q,q))if(k&1)r=add(r,q);return r;}
struct Hit {Pair pair{};size_t k=0;int s=0;bool ok=false;};
Hit reference(Point r,const vector<Point>& f,const Table& h){
  for(size_t k=0;k<f.size();++k){
    auto q=h.find(key(add(r,neg(f[k]))));if(q!=h.end())return {q->second,k,1,true};
    q=h.find(key(add(r,f[k])));if(q!=h.end())return {q->second,k,-1,true};
  }return {};
}
Hit batched_query(Point r,const vector<Point>& f,const Table& h){
  if(r.o)return reference(r,f,h);
  const size_t b=f.size();vector<uint64_t> d(b),pre(b+1,1);
  for(size_t k=0;k<b;++k){d[k]=r.x==f[k].x?1:sub(f[k].x,r.x);pre[k+1]=mm(pre[k],d[k]);}
  uint64_t t=inv(pre.back());vector<Point> minus(b),plus(b);
  for(size_t kk=b;kk-->0;){uint64_t id=mm(t,pre[kk]);t=mm(t,d[kk]);
    if(r.x==f[kk].x){minus[kk]=add(r,neg(f[kk]));plus[kk]=add(r,f[kk]);}
    else {minus[kk]=slope_add(r,neg(f[kk]),id);plus[kk]=slope_add(r,f[kk],id);}
  }
  for(size_t k=0;k<b;++k){auto q=h.find(key(minus[k]));if(q!=h.end())return {q->second,k,1,true};
    q=h.find(key(plus[k]));if(q!=h.end())return {q->second,k,-1,true};}
  return {};
}
bool witness(Point r,const vector<Point>& f,Hit h){
  if(!h.ok)return false;
  Point v=relation(f,h.pair);
  return add(v,h.s==1?f[h.k]:neg(f[h.k]))==r;
}

#ifndef INCLUDE_AS_LIBRARY
int main(int argc,char**argv){
  size_t b=argc>1?stoul(argv[1]):384;size_t qcount=argc>2?stoul(argv[2]):200;
  if(b<8||b>1500||qcount<1||qcount>1000)return 2;
  auto f=base(b);auto h=batched(f);mt19937_64 gen(26021115+b);
  vector<Point> queries{f[0]};queries.reserve(qcount+1);
  for(size_t j=0;j<qcount;++j)queries.push_back(scalar(G,1+gen()%(N-1)));
  uint64_t ref_ns=0,bat_ns=0,old_inv=0,new_inv=0,old_mul=0,new_mul=0;
  size_t positive=0,negative=0;
  for(Point r:queries){
    ct={};auto x=chrono::steady_clock::now();Hit a=reference(r,f,h);auto y=chrono::steady_clock::now();Cnt ca=ct;
    ct={};auto z=chrono::steady_clock::now();Hit c=batched_query(r,f,h);auto w=chrono::steady_clock::now();Cnt cb=ct;
    if(a.ok!=c.ok)throw runtime_error("answer mismatch");
    if(a.ok){if(a.k!=c.k||a.s!=c.s||a.pair.i!=c.pair.i||a.pair.j!=c.pair.j||a.pair.si!=c.pair.si||a.pair.sj!=c.pair.sj)throw runtime_error("relation mismatch");if(!witness(r,f,c))throw runtime_error("bad relation");++positive;}
    else ++negative;
    ref_ns+=chrono::duration_cast<chrono::nanoseconds>(y-x).count();bat_ns+=chrono::duration_cast<chrono::nanoseconds>(w-z).count();old_inv+=ca.inv;new_inv+=cb.inv;old_mul+=ca.mul;new_mul+=cb.mul;
  }
  cout<<"{\"p\":"<<P<<",\"b\":"<<b<<",\"queries\":"<<queries.size()
      <<",\"positives\":"<<positive<<",\"negatives\":"<<negative
      <<",\"reference_ms\":"<<ref_ns/1e6<<",\"batched_ms\":"<<bat_ns/1e6
      <<",\"speedup\":"<<double(ref_ns)/bat_ns
      <<",\"reference_inversions\":"<<old_inv<<",\"batched_inversions\":"<<new_inv
      <<",\"reference_multiplies\":"<<old_mul<<",\"batched_multiplies\":"<<new_mul<<"}\n";
}
#endif
