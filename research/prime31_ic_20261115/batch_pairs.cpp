// Exact Semaev S3 / shared-denominator pair table construction comparison.
// This optimizes factor-base relation setup; it is not a new subgeneric ECDLP algorithm.
#include <algorithm>
#include <chrono>
#include <tuple>
#include <utility>
#include <cstdint>
#include <iostream>
#include <random>
#include <stdexcept>
#include <unordered_map>
#include <vector>
using namespace std;
constexpr uint64_t P=2147483647ULL, A=847514846ULL, BC=2087444724ULL;
struct Point {uint32_t x=0,y=0;bool o=true;bool operator==(Point q) const {return o==q.o && (o || (x==q.x && y==q.y));}};
struct Pair{uint16_t i,j;int8_t si,sj;};
struct Cnt{uint64_t mul=0, inv=0, add=0;} ct;
uint64_t mm(uint64_t x,uint64_t y){++ct.mul;return x*y%P;}
uint64_t sub(uint64_t x,uint64_t y){return x>=y?x-y:P+x-y;}
uint64_t pw(uint64_t a,uint64_t e){uint64_t z=1;for(;e;e>>=1,a=mm(a,a))if(e&1)z=mm(z,a);return z;}
uint64_t inv(uint64_t x){++ct.inv; if(!x)throw runtime_error("inverse zero");int64_t a=P,b=x,u=0,v=1;while(b){int64_t q=a/b;tie(a,b)=make_pair(b,a-q*b);tie(u,v)=make_pair(v,u-q*v);}if(a!=1)throw runtime_error("not unit");return (u%int64_t(P)+P)%P;}
Point neg(Point p){if(!p.o && p.y)p.y=P-p.y;return p;}
uint64_t key(Point p){return p.o?UINT64_MAX:((uint64_t)p.x<<31)|p.y;}
Point slope_add(Point u,Point v,uint64_t den_inv){
  uint64_t slope=mm(sub(v.y,u.y),den_inv);
  uint64_t x=sub(sub(mm(slope,slope),u.x),v.x);
  uint64_t y=sub(mm(slope,sub(u.x,x)),u.y);
  return {uint32_t(x),uint32_t(y),false};
}
Point add(Point u,Point v){++ct.add;if(u.o)return v;if(v.o)return u;if(u.x==v.x){if((uint64_t(u.y)+v.y)%P==0)return {};uint64_t xx=mm(u.x,u.x),s=mm((3*xx+A)%P,inv(2*uint64_t(u.y)%P));uint64_t x=sub(sub(mm(s,s),u.x),v.x),y=sub(mm(s,sub(u.x,x)),u.y);return {uint32_t(x),uint32_t(y),false};}return slope_add(u,v,inv(sub(v.x,u.x)));}
vector<Point> base(size_t n){vector<Point> f;for(uint64_t x=0;x<P&&f.size()<n;++x){uint64_t rhs=(mm(mm(x,x),x)+mm(A,x)+BC)%P;if(!rhs||pw(rhs,(P-1)/2)!=1)continue;uint64_t y=pw(rhs,(P+1)/4);if(mm(y,y)!=rhs)throw runtime_error("sqrt");f.push_back({uint32_t(x),uint32_t(min(y,P-y)),false});}return f;}
using Table=unordered_map<uint64_t,Pair>;
void put(Table&h,Point p,size_t i,size_t j,int si,int sj){h.emplace(key(p),Pair{uint16_t(i),uint16_t(j),int8_t(si),int8_t(sj)});}
Table ordinary(const vector<Point>& f){Table h;h.reserve(2*f.size()*f.size());for(size_t i=0;i<f.size();++i)for(size_t j=i;j<f.size();++j)for(int si:{-1,1})for(int sj:{-1,1})put(h,add(si==1?f[i]:neg(f[i]),sj==1?f[j]:neg(f[j])),i,j,si,sj);return h;}
Table batched(const vector<Point>& f){
  Table h;h.reserve(2*f.size()*f.size());
  struct Item{uint16_t i,j;uint32_t d;};vector<Item> items;items.reserve(8192);
  auto flush=[&](){if(items.empty())return;
    vector<uint64_t> prefix(items.size()+1,1);
    for(size_t k=0;k<items.size();++k)prefix[k+1]=mm(prefix[k],items[k].d);
    uint64_t t=inv(prefix.back());
    for(size_t kk=items.size();kk-->0;){auto s=items[kk];uint64_t id=mm(prefix[kk],t);t=mm(t,s.d);
      Point pi=f[s.i],pj=f[s.j];
      Point sum=slope_add(pi,pj,id),diff=slope_add(pi,neg(pj),id);
      put(h,neg(sum),s.i,s.j,-1,-1);put(h,neg(diff),s.i,s.j,-1,1);
      put(h,diff,s.i,s.j,1,-1);put(h,sum,s.i,s.j,1,1);
    }items.clear();};
  // Note: first-kept representations may differ, but every stored relation must verify.
  for(size_t i=0;i<f.size();++i){
    Point twice=add(f[i],f[i]);
    put(h,neg(twice),i,i,-1,-1);put(h,{},i,i,-1,1);
    put(h,{},i,i,1,-1);put(h,twice,i,i,1,1);
    for(size_t j=i+1;j<f.size();++j){items.push_back({uint16_t(i),uint16_t(j),uint32_t(sub(f[j].x,f[i].x))});if(items.size()==8192)flush();}
  }flush();return h;
}
Point relation(const vector<Point>&f,Pair z){return add(z.si==1?f[z.i]:neg(f[z.i]),z.sj==1?f[z.j]:neg(f[z.j]));}
int main(int argc,char**argv){size_t b=argc>1?stoul(argv[1]):384;if(b<8||b>1500)return 2;auto f=base(b);ct={};auto t0=chrono::steady_clock::now();auto ref=ordinary(f);auto t1=chrono::steady_clock::now();Cnt cr=ct;ct={};auto fast=batched(f);auto t2=chrono::steady_clock::now();Cnt cf=ct;
  if(ref.size()!=fast.size())throw runtime_error("table cardinality mismatch");
  for(const auto &kv:ref)if(fast.find(kv.first)==fast.end())throw runtime_error("keyset mismatch");
  size_t checked=0;for(auto it=fast.begin();it!=fast.end();++it,++checked)if(key(relation(f,it->second))!=it->first)throw runtime_error("invalid fast relation");
  double slow_ms=chrono::duration<double,milli>(t1-t0).count(),fast_ms=chrono::duration<double,milli>(t2-t1).count();
  cout<<"{\"p\":"<<P<<",\"b\":"<<b<<",\"pairs\":"<<fast.size()<<",\"compared_keys\":"<<ref.size()<<",\"checked_relations\":"<<checked<<",\"slow_ms\":"<<slow_ms<<",\"fast_ms\":"<<fast_ms<<",\"speedup\":"<<slow_ms/fast_ms<<",\"slow_inverse\":"<<cr.inv<<",\"fast_inverse\":"<<cf.inv<<",\"slow_field_multiplies\":"<<cr.mul<<",\"fast_field_multiplies\":"<<cf.mul<<",\"slow_adds\":"<<cr.add<<",\"fast_adds\":"<<cf.add<<"}\n";
  return 0;
}
