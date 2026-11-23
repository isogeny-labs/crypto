// Complete three-summand factor-base index-calculus pilot on ordinary prime-order E/F_(2^31-1).
// The pair-table method is not novel or subgeneric; it is an honest index-calculus baseline.
#include <cmath>
// C++17. compile: g++ -O3 -std=c++17 -DNDEBUG -o prime31_ic prime31_ic.cpp
#include <algorithm>
#include <chrono>
#include <cstdint>
#include <cstdlib>
#include <iostream>
#include <map>
#include <random>
#include <stdexcept>
#include <string>
#include <unordered_map>
#include <vector>
using namespace std;
static constexpr uint64_t P=2147483647ULL, N=2147446831ULL;
static constexpr uint32_t A=847514846, BCOEF=2087444724;
struct Pt { uint32_t x=0, y=0; bool inf=true; bool operator==(const Pt&o) const{return inf==o.inf && (inf || (x==o.x&&y==o.y));} };
static const Pt G={625037715,2100627251,false};
struct Count {uint64_t adds=0, inv=0, muls=0, sqrts=0;} cost;
static uint64_t modadd(uint64_t a,uint64_t b,uint64_t m){uint64_t s=a+b;return s>=m?s-m:s;}
static uint64_t modsub(uint64_t a,uint64_t b,uint64_t m){return a>=b?a-b:a+m-b;}
static uint64_t modmul(uint64_t a,uint64_t b,uint64_t m){cost.muls++;return (a*b)%m;}
static uint64_t modpow(uint64_t a,uint64_t e,uint64_t m){uint64_t r=1;while(e){if(e&1)r=modmul(r,a,m);a=modmul(a,a,m);e>>=1;}return r;}
static uint64_t inverse(uint64_t x,uint64_t modulus){
 cost.inv++; if(x==0)throw runtime_error("division by zero");
 int64_t t=0,u=1,r=(int64_t)modulus,s=(int64_t)x;
 while(s){int64_t q=r/s;int64_t z=t-q*u;t=u;u=z;z=r-q*s;r=s;s=z;}
 if(r!=1)throw runtime_error("nonunit"); if(t<0)t+=modulus;return (uint64_t)t;
}
static Pt neg(Pt Q){if(!Q.inf && Q.y)Q.y=P-Q.y;return Q;}
static uint64_t key(Pt Q){return Q.inf?UINT64_MAX:((uint64_t)Q.x<<31)|Q.y;}
static Pt add(Pt Q,Pt R){
 cost.adds++;
 if(Q.inf)return R;if(R.inf)return Q;
 uint64_t s=0;
 if(Q.x==R.x){if((Q.y+uint64_t(R.y))%P==0)return Pt{};
   s=modmul(modadd(modmul(3,modmul(Q.x,Q.x,P),P),A,P),inverse((2ULL*Q.y)%P,P),P);
 }else s=modmul(modsub(R.y,Q.y,P),inverse(modsub(R.x,Q.x,P),P),P);
 uint64_t x=modsub(modsub(modmul(s,s,P),Q.x,P),R.x,P);
 uint64_t y=modsub(modmul(s,modsub(Q.x,x,P),P),Q.y,P);
 return {(uint32_t)x,(uint32_t)y,false};
}
static Pt scalar(Pt Q,uint64_t k){Pt R;while(k){if(k&1)R=add(R,Q);Q=add(Q,Q);k>>=1;}return R;}
static bool oncurve(Pt Q){if(Q.inf)return true;uint64_t x=Q.x;uint64_t f=(modmul(modmul(x,x,P),x,P)+modmul(A,x,P)+BCOEF)%P;return modmul(Q.y,Q.y,P)==f;}
static vector<Pt> factorbase(size_t n){vector<Pt> fb;for(uint64_t x=0;x<P&&fb.size()<n;x++){
 uint64_t f=(modmul(modmul(x,x,P),x,P)+modmul(A,x,P)+BCOEF)%P;
 if(f==0)continue;
 if(modpow(f,(P-1)/2,P)!=1)continue;
 cost.sqrts++; uint64_t y=modpow(f,(P+1)/4,P);
 if(y*y%P!=f)throw runtime_error("sqrt invalid");
 fb.push_back({(uint32_t)x,(uint32_t)min(y,P-y),false});
 }return fb;}
struct Pair {uint16_t i,j;int8_t si,sj;};
static unordered_map<uint64_t,Pair> buildpairs(const vector<Pt>& fb){
 unordered_map<uint64_t,Pair> ht;ht.reserve(2*fb.size()*fb.size());
 for(size_t i=0;i<fb.size();i++)for(size_t j=i;j<fb.size();j++)for(int si:{-1,1})for(int sj:{-1,1}){
   Pt x=add(si==1?fb[i]:neg(fb[i]),sj==1?fb[j]:neg(fb[j]));
   // Keep a single valid representation, including infinity. Hash collisions are resolved by unordered_map.
   ht.emplace(key(x),Pair{(uint16_t)i,(uint16_t)j,(int8_t)si,(int8_t)sj});
 }
 return ht;
}
struct Echelon {
 size_t width;vector<vector<uint32_t>> piv;size_t rank=0, equations=0, dependent=0;
 explicit Echelon(size_t n):width(n),piv(n){}
 bool submit(uint64_t scalar_a,const Pair&pair,size_t k,int sign_k){
  vector<uint32_t> row(width+1,0);
  auto bump=[&](size_t pos,int sign){row[pos]=(uint32_t)(sign==1?modadd(row[pos],1,N):modsub(row[pos],1,N));};
  bump(pair.i,pair.si);bump(pair.j,pair.sj);bump(k,sign_k);row[width]=(uint32_t)scalar_a;
  equations++;
  for(size_t j=0;j<width;j++){
   uint64_t c=row[j]; if(!c)continue;
   if(piv[j].empty()){
    uint64_t scale=inverse(c,N);
    for(size_t t=j;t<=width;t++)if(row[t])row[t]=(uint32_t)modmul(row[t],scale,N);
    piv[j]=move(row);rank++;return true;
   }
   const auto&pr=piv[j];
   for(size_t t=j;t<=width;t++){
    if(pr[t])row[t]=(uint32_t)modsub(row[t],modmul(c,pr[t],N),N);
   }
  }
  if(row[width])throw runtime_error("contradictory group relation");dependent++;return false;
 }
 vector<uint32_t> solve()const{
  if(rank<width)throw runtime_error("incomplete linear algebra");
  vector<uint32_t> logs(width);
  for(int i=(int)width-1;i>=0;i--){
   uint64_t v=piv[i][width];
   for(size_t j=i+1;j<width;j++)if(piv[i][j])v=modsub(v,modmul(piv[i][j],logs[j],N),N);
   logs[i]=(uint32_t)v;
  }return logs;
 }
};
struct Relation{Pair pair;size_t k;int sign;};
static bool decompose(Pt R,const vector<Pt>&fb,const unordered_map<uint64_t,Pair>&ht,Relation &res){
 for(size_t k=0;k<fb.size();k++){
  Pt delta=add(R,neg(fb[k]));
  auto it=ht.find(key(delta));
  if(it!=ht.end()){res={it->second,k,1};return true;}
  delta=add(R,fb[k]);it=ht.find(key(delta));
  if(it!=ht.end()){res={it->second,k,-1};return true;}
 }
 return false;
}
static uint64_t recover(const Relation &r,const vector<uint32_t>&logs){
 uint64_t v=0;
 auto upd=[&](size_t i,int sign){v=(sign==1?modadd(v,logs[i],N):modsub(v,logs[i],N));};
 upd(r.pair.i,r.pair.si);upd(r.pair.j,r.pair.sj);upd(r.k,r.sign);return v;
}
static uint64_t bsgs(Pt Q){
 uint64_t m=uint64_t(sqrtl((long double)N))+1;
 unordered_map<uint64_t,uint32_t> baby;baby.reserve(m*2);Pt cur;
 for(uint64_t j=0;j<m;j++){baby.emplace(key(cur),(uint32_t)j);cur=add(cur,G);}
 Pt step=neg(scalar(G,m));cur=Q;
 for(uint64_t i=0;i<=m;i++){
  auto it=baby.find(key(cur));if(it!=baby.end()){
   uint64_t v=(i*m+it->second)%N;
   if(scalar(G,v)==Q)return v;throw runtime_error("BSGS invalid");
  }cur=add(cur,step);
 }
 throw runtime_error("BSGS failure");
}
struct RhoState {Pt X;uint64_t alpha,beta;};
static uint64_t rho16(Pt Q,uint64_t seed){
 mt19937_64 gen(seed^0x9e3779b97f4a7c15ULL);
 uniform_int_distribution<uint64_t> dist(1,N-1);
 vector<RhoState> move;
 for(int i=0;i<16;i++){
  uint64_t a=dist(gen), b=dist(gen);
  move.push_back({add(scalar(G,a),scalar(Q,b)),a,b});
 }
 auto step=[&](RhoState& st){
  size_t i=st.X.inf?0:((uint64_t(st.X.x) ^ (uint64_t(st.X.y)<<1))&15);
  st.X=add(st.X,move[i].X);
  st.alpha=modadd(st.alpha,move[i].alpha,N);
  st.beta=modadd(st.beta,move[i].beta,N);
 };
 for(int attempt=0;attempt<8;attempt++){
  uint64_t a=dist(gen),b=dist(gen);
  RhoState tort{add(scalar(G,a),scalar(Q,b)),a,b},hare=tort;
  for(uint64_t steps=0;steps<5'000'000;steps++){
   step(tort);step(hare);step(hare);
   if(tort.X==hare.X){
    uint64_t denominator=modsub(hare.beta,tort.beta,N);
    if(denominator==0)break;
    uint64_t num=modsub(tort.alpha,hare.alpha,N);
    uint64_t d=modmul(num,inverse(denominator,N),N);
    if(scalar(G,d)==Q)return d;
    throw runtime_error("rho verified incorrect scalar");
   }
  }
 }
 throw runtime_error("rho retries exhausted");
}
int main(int argc,char**argv){
 try{
  size_t base_size=argc>1?stoul(argv[1]):384;
  uint64_t seed=argc>2?stoull(argv[2]):20261114;
  uint64_t limit=argc>3?stoull(argv[3]):1000000;
  if(base_size<8 || base_size>5000)throw runtime_error("bad size");
  if(!oncurve(G) || !scalar(G,N).inf)throw runtime_error("curve certificate failure");
  mt19937_64 rng(seed);uniform_int_distribution<uint64_t> uni(1,N-1);
  uint64_t secret=uni(rng);Pt Q=scalar(G,secret);
  cost={};auto begin=chrono::steady_clock::now();auto fb=factorbase(base_size);auto table=buildpairs(fb);
  Echelon linear(base_size);uint64_t trials=0, positive=0;
  Relation relation{};
  while(linear.rank<base_size&&trials<limit){
   uint64_t a=uni(rng);Pt R=scalar(G,a);trials++;
   if(decompose(R,fb,table,relation)){
    positive++; linear.submit(a,relation.pair,relation.k,relation.sign);
   }
  }
  auto after_rel=chrono::steady_clock::now();
  if(linear.rank<base_size){
   cout<<"{\"status\":\"censored\",\"rank\":"<<linear.rank<<",\"target\":["<<Q.x<<","<<Q.y<<"]"
  <<",\"factor_base\":"<<base_size<<",\"trials\":"<<trials<<",\"group_adds\":"<<cost.adds<<"}\n";return 2;
  }
  auto logs=linear.solve();
  for(size_t i=0;i<base_size;i++)if(!(scalar(G,logs[i])==fb[i]))throw runtime_error("factor base log verification fail");
  auto after_linalg=chrono::steady_clock::now();
  uint64_t shifts=0, answer=0;Pt shifted=Q;
  while(shifts<N){
   if(decompose(shifted,fb,table,relation)){
    answer=modsub(recover(relation,logs),shifts%N,N);
    if(scalar(G,answer)==Q)break;
    throw runtime_error("target log verification fail");
   }
   shifted=add(shifted,G);shifts++;
  }
  if(shifts==N)throw runtime_error("target failed to decompose");
  auto end=chrono::steady_clock::now();
  uint64_t ic_adds=cost.adds,ic_invs=cost.inv,ic_muls=cost.muls;
  auto baseline_start=chrono::steady_clock::now();cost={};uint64_t check=bsgs(Q);auto baseline_end=chrono::steady_clock::now();
  if(check!=secret || answer!=secret)throw runtime_error("wrong logarithm");
  uint64_t bsgs_adds=cost.adds,bsgs_invs=cost.inv,bsgs_muls=cost.muls;
  auto rho_start=chrono::steady_clock::now();cost={};uint64_t rhoanswer=rho16(Q,seed);auto rho_end=chrono::steady_clock::now();
  if(rhoanswer!=secret)throw runtime_error("wrong rho logarithm");
  uint64_t rho_adds=cost.adds,rho_invs=cost.inv,rho_muls=cost.muls;
  cout<<"{\"status\":\"complete\",\"p\":"<<P<<",\"a\":"<<A<<",\"b\":"<<BCOEF
  <<",\"subgroup_order\":"<<N<<",\"generator\":["<<G.x<<","<<G.y<<"]"
  <<",\"target\":["<<Q.x<<","<<Q.y<<"]"
  <<",\"factor_base\":"<<base_size<<",\"pair_table_size\":"<<table.size()
  <<",\"trials\":"<<trials<<",\"positive_relations\":"<<positive
  <<",\"independent_rank\":"<<linear.rank<<",\"dependent_relations\":"<<linear.dependent
  <<",\"individual_target_shifts\":"<<shifts<<",\"secret\":"<<secret
  <<",\"verified_log\":"<<answer<<",\"ic_additions\":"<<ic_adds<<",\"ic_inversions\":"<<ic_invs
  <<",\"ic_field_multiplications\":"<<ic_muls<<",\"bsgs_additions\":"<<bsgs_adds
  <<",\"bsgs_inversions\":"<<bsgs_invs<<",\"bsgs_field_multiplications\":"<<bsgs_muls
  <<",\"relation_ms\":"<<chrono::duration_cast<chrono::milliseconds>(after_rel-begin).count()
  <<",\"linalg_and_check_ms\":"<<chrono::duration_cast<chrono::milliseconds>(after_linalg-after_rel).count()
  <<",\"ic_ms\":"<<chrono::duration_cast<chrono::milliseconds>(end-begin).count()
  <<",\"bsgs_ms\":"<<chrono::duration_cast<chrono::milliseconds>(baseline_end-baseline_start).count()
  <<",\"rho_additions\":"<<rho_adds<<",\"rho_inversions\":"<<rho_invs<<",\"rho_field_multiplications\":"<<rho_muls
  <<",\"rho_ms\":"<<chrono::duration_cast<chrono::milliseconds>(rho_end-rho_start).count()
  <<",\"seed\":"<<seed<<"}\n";
 }catch(const exception&e){cerr<<"ERROR "<<e.what()<<"\n";return 1;}
}
