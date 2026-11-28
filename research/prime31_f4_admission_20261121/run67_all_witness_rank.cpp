// Prime31 run67: reference GENERIC exhaustive signed-triple grader ONLY.
// Two-component quotient fingerprints are a necessary filter, not sufficient;
// independently verify every reported witness with the exact 677-coordinate
// quotient and independent big-integer elliptic-curve law (run67_verify_all.py).
// This enumerative dictionary is NOT a non-generic index-calculus extractor.
#include <cstdint>
#include <cstdio>
#include <cstdlib>
#include <cassert>
#include <string>
#include <vector>
#include <unordered_map>
#include <fstream>
#include <sstream>
#include <iostream>
#include <chrono>
#include <algorithm>
using u64=uint64_t; using u32=uint32_t;
constexpr u64 p=2147483647ULL,a=847514846ULL,b=2087444724ULL,n=2147446831ULL;
struct Pt {u64 x=0,y=0;bool inf=true;};
static u64 padd(u64 x,u64 y){u64 q=x+y;return q>=p?q-p:q;}
static u64 psub(u64 x,u64 y){return x>=y?x-y:p+x-y;}
static u64 pmul(u64 x,u64 y){return x*y%p;}
static u64 ppow(u64 x,u64 e){u64 q=1;for(;e;e>>=1,x=pmul(x,x))if(e&1)q=pmul(q,x);return q;}
static Pt neg(Pt x){if(!x.inf&&x.y)x.y=p-x.y;return x;}
static Pt ecadd(Pt x,Pt y){
 if(x.inf)return y;if(y.inf)return x;
 if(x.x==y.x&&x.y!=y.y)return {};
 u64 s;
 if(x.x==y.x){if(x.y==0)return {};s=pmul(padd(pmul(3,pmul(x.x,x.x)),a),ppow(padd(x.y,x.y),p-2));}
 else{s=pmul(psub(y.y,x.y),ppow(psub(y.x,x.x),p-2));}
 u64 xx=psub(psub(pmul(s,s),x.x),y.x);
 return {xx,psub(pmul(s,psub(x.x,xx)),x.y),false};
}
static u64 encode(Pt x){return x.inf?UINT64_MAX:(x.x<<32)|x.y;}
struct Finger {u32 a,b;};
struct Signed {Pt q;Finger f;int index;};
struct Key {u64 point;u32 f1,f2;bool operator==(const Key& z)const{
 return point==z.point&&f1==z.f1&&f2==z.f2;}};
struct Hash {size_t operator()(const Key& k)const{
 u64 t=k.point^(u64(k.f1)<<33)^u64(k.f2)*0x9e3779b97f4a7c15ULL;
 t^=t>>30;t*=0xbf58476d1ce4e5b9ULL;t^=t>>27;t*=0x94d049bb133111ebULL;
 return size_t(t^(t>>31));}};
static u32 np(u32 t){return t?u32(n-t):0;}
static u32 fa(u32 x,u32 y){u64 z=u64(x)+y;return u32(z>=n?z-n:z);}
static Key make_key(Pt q,Finger f){return {encode(q),f.a,f.b};}
static std::vector<Pt> base_points(const std::string& file){
 std::ifstream f(file);assert(f);std::vector<Pt> v;u64 x,y;char sep;
 while(f>>x>>sep>>y){assert(sep==',');v.push_back({x,y,false});}
 assert(v.size()==677);return v;
}
static std::vector<Finger> fingerprints(const std::string& file){
 std::ifstream f(file);assert(f);std::vector<Finger> v;u64 x,y;char sep;
 while(f>>x>>sep>>y){assert(sep==','&&x<n&&y<n);v.push_back({u32(x),u32(y)});}
 assert(v.size()==677);return v;
}
static std::vector<Pt> plain_public(const std::string& file){
 std::ifstream f(file);assert(f);std::vector<Pt> v;u64 x,y;char sep;
 while(f>>x>>sep>>y){assert(sep==',');v.push_back({x,y,false});}
 assert(v.size()==4096);return v;
}
static std::vector<Pt> adaptive_public(const std::string& file){
 std::ifstream f(file);assert(f);std::vector<Pt> v;std::string line;getline(f,line);
 while(getline(f,line)){
  std::replace(line.begin(),line.end(),',',' ');std::istringstream in(line);
  u64 ix,x,y;in>>ix>>x>>y;assert(ix==v.size());v.push_back({x,y,false});
 }
 assert(v.size()==4096);return v;
}
static void run(const std::vector<Pt>& queries,const std::vector<Signed>& signed_base,
 const std::unordered_map<Key,std::vector<uint32_t>,Hash>& table,
 int m,std::string cohort,std::ostream& out){
 size_t positive=0,pair_validations=0,groups_scanned=0;
 auto start=std::chrono::steady_clock::now();
 for(size_t ix=0;ix<queries.size();ix++){
  Pt Q=queries[ix];bool found=false;
  for(size_t si=0;si<signed_base.size()&&!found;si++){
   auto &r=signed_base[si];Pt diff=ecadd(Q,neg(r.q));
   Key key={encode(diff),np(r.f.a),np(r.f.b)};
   auto it=table.find(key);groups_scanned++;
   if(it==table.end())continue;
   for(uint32_t z:it->second){
    uint32_t j=z/signed_base.size(),k=z%signed_base.size();
    auto &s=signed_base[j],&t=signed_base[k];
    assert(encode(ecadd(ecadd(r.q,s.q),t.q))==encode(Q));
    assert(fa(fa(r.f.a,s.f.a),t.f.a)==0&&fa(fa(r.f.b,s.f.b),t.f.b)==0);
    pair_validations++;found=true;positive++;
    out<<cohort<<","<<ix<<","<<r.index<<","<<s.index<<","<<t.index<<"\n";
    break;
   }
  }
 }
 auto elapsed=std::chrono::duration<double>(std::chrono::steady_clock::now()-start).count();
 std::cout<<"m="<<m<<" cohort="<<cohort<<" N="<<queries.size()
  <<" quotient_fingerprint_hits="<<positive<<" positive_witnesses="<<pair_validations
  <<" group_probes="<<groups_scanned<<" scan_sec="<<elapsed
  <<" generic_grader_only=true\n";
}
int main(int argc,char**argv){
 if(argc!=7){std::cerr<<"usage: ./run67 m base.csv fingerprints.csv sha.csv adaptive.csv witness_output.csv\n";return 2;}
 int m=std::stoi(argv[1]);auto base=base_points(argv[2]);auto f=fingerprints(argv[3]);
 auto sha=plain_public(argv[4]);auto adaptive=adaptive_public(argv[5]);
 std::vector<Signed> signed_base;signed_base.reserve(1354);
 for(size_t i=0;i<base.size();i++){
  signed_base.push_back({base[i],f[i],int(i+1)});
  signed_base.push_back({neg(base[i]),Finger{np(f[i].a),np(f[i].b)},-int(i+1)});
 }
 std::unordered_map<Key,std::vector<uint32_t>,Hash> pair_table;
 pair_table.reserve(1'000'000);
 auto start=std::chrono::steady_clock::now();
 for(size_t i=0;i<signed_base.size();i++)for(size_t j=i;j<signed_base.size();j++){
  auto &s=signed_base[i],&t=signed_base[j];
  auto key=make_key(ecadd(s.q,t.q),{fa(s.f.a,t.f.a),fa(s.f.b,t.f.b)});
  pair_table[key].push_back(uint32_t(i*signed_base.size()+j));
 }
 std::cout<<"m="<<m<<" pair_rows=917335 unique_key_count="<<pair_table.size()
  <<" construction_seconds="
  <<std::chrono::duration<double>(std::chrono::steady_clock::now()-start).count()<<"\n";
 std::ofstream out(argv[6]);assert(out);out<<"cohort,index,w0,w1,w2\n";
 run(sha,signed_base,pair_table,m,"sha256",out);
 run(adaptive,signed_base,pair_table,m,"w8",out);
}
