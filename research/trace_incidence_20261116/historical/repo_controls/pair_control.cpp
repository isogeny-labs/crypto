// Portable algorithm-level control. NOT an exact build of upstream prime_fast.rs.
// Single-threaded, batched inversion and x-only streamed pair search.
// Uses no planted witness or secret logarithm. Build: g++ -O3 -std=c++17.
#include <algorithm>
#include <cassert>
#include <chrono>
#include <cstdint>
#include <ctime>
#include <fstream>
#include <iostream>
#include <iterator>
#include <sstream>
#include <string>
#include <sys/resource.h>
#include <unordered_map>
#include <utility>
#include <vector>
using U=uint64_t;
using Clock=std::chrono::steady_clock;
struct Counts {U mul=0,inv=0,group=0;};
struct Field {
 U p; Counts counts{};
 U mul(U x,U y){++counts.mul;return (U)((__uint128_t)x*y%p);}
 U add(U x,U y){return (U)(((__uint128_t)x+y)%p);}
 U sub(U x,U y){return x>=y?x-y:p-(y-x);}
 U neg(U x){return x?p-x:0;}
 U inv(U x){
  assert(x);++counts.inv;
  __int128 u=0,v=1; U aa=p,bb=x;
  while(bb){U q=aa/bb,rr=aa%bb;aa=bb;bb=rr;auto w=u-(__int128)q*v;u=v;v=w;}
  assert(aa==1);u%=p;if(u<0)u+=p;return (U)u;
 }
};
struct Pt {U x=0,y=0;bool inf=true;};
bool equal(Pt u,Pt v){return (u.inf&&v.inf)||(!u.inf&&!v.inf&&u.x==v.x&&u.y==v.y);}
Pt neg(Field &f,Pt q){if(!q.inf)q.y=f.neg(q.y);return q;}
Pt add(Field &f,U a,Pt u,Pt v){
 ++f.counts.group;
 if(u.inf)return v;
 if(v.inf)return u;
 if(u.x==v.x&&f.add(u.y,v.y)==0)return {};
 U num,den;
 if(u.x==v.x){num=f.add(f.mul(3,f.mul(u.x,u.x)),a);den=f.add(u.y,u.y);}
 else{num=f.sub(v.y,u.y);den=f.sub(v.x,u.x);}
 U slope=f.mul(num,f.inv(den));
 U x=f.sub(f.sub(f.mul(slope,slope),u.x),v.x);
 U y=f.sub(f.mul(slope,f.sub(u.x,x)),u.y);return {x,y,false};
}
Pt scalar(Field &f,U a,U n,Pt q){
 Pt out;
 while(n){if(n&1)out=add(f,a,out,q);q=add(f,a,q,q);n>>=1;}return out;
}
struct Entry {U x,y,alpha,beta;};
struct Witness {size_t i=0,j=0,k=0;int sj=1,sk=1;};
struct Recovery {U A=0,B=0,k=0;bool geo=false,label=false,informative=false,verified=false;};
Recovery verify(Field &f,U a,U r,Pt P,Pt Q,const std::vector<Entry> pools[3],Witness w){
 Recovery out;const size_t ids[3]={w.i,w.j,w.k};const int signs[3]={1,w.sj,w.sk};
 Pt acc;
 Field scalars{r};
 for(int h=0;h<3;++h){const auto&e=pools[h][ids[h]];Pt q{e.x,e.y,false};
  acc=add(f,a,acc,signs[h]==1?q:neg(f,q));
  out.A=scalars.add(out.A,signs[h]==1?e.alpha:scalars.neg(e.alpha));
  out.B=scalars.add(out.B,signs[h]==1?e.beta:scalars.neg(e.beta));}
 out.geo=acc.inf;
 out.label=add(f,a,scalar(f,a,out.A,P),scalar(f,a,out.B,Q)).inf;
 out.informative=out.geo&&out.label&&out.B!=0;
 if(out.informative){out.k=scalars.mul(scalars.neg(out.A),scalars.inv(out.B));
  out.verified=equal(scalar(f,a,out.k,P),Q);}
 return out;
}
double cpu(){return double(std::clock())/CLOCKS_PER_SEC;}
double wall(){return std::chrono::duration<double>(Clock::now().time_since_epoch()).count();}
int main(int argc,char**argv){
 unsigned repetitions=1;
 if(argc==3&&std::string(argv[1])=="--repeats")repetitions=std::stoul(argv[2]);
 if(!repetitions)return 2;
 std::string serialized((std::istreambuf_iterator<char>(std::cin)),std::istreambuf_iterator<char>());
 const double batch_start_cpu=cpu(),batch_start_wall=wall();
 double sum_pre_cpu=0,sum_search_cpu=0,sum_rec_cpu=0,sum_cpu=0;
 double sum_pre_wall=0,sum_search_wall=0,sum_rec_wall=0,sum_wall=0;
 for(unsigned repetition=0;repetition<repetitions;++repetition){
 const double t0=cpu(),w0=wall();
 std::istringstream input(serialized);
 U p,a,b,r,px,py,qx,qy;size_t sizes[3];
 if(!(input>>p>>a>>b>>r>>px>>py>>qx>>qy>>sizes[0]>>sizes[1]>>sizes[2]))return 2;
 if(p>uint64_t(1)<<62||r>uint64_t(1)<<62)return 3;
 std::vector<Entry> pools[3];
 for(int h=0;h<3;++h){pools[h].resize(sizes[h]);for(auto&e:pools[h])input>>e.x>>e.y>>e.alpha>>e.beta;}
 if(!input)return 2;
 Field f{p};Pt P{px,py,false},Q{qx,qy,false};
 std::unordered_map<U,std::vector<size_t>> index;index.reserve(sizes[2]*2+1);
 for(size_t k=0;k<sizes[2];++k)index[pools[2][k].x].push_back(k);
 std::vector<U> den(sizes[1]),prefix(sizes[1]),inverse(sizes[1]);
 std::vector<size_t> ordinary;ordinary.reserve(sizes[1]);
 const double tp=cpu(),wp=wall();
 U positions=0,signed_pairs=0,lookups=0,hits=0,candidates=0,uninformative=0,exceptions=0,infinity=0,batch_rows=0,batch_inputs=0;
 U search_mul=0,search_inv=0,verify_mul=0,verify_inv=0,verify_group=0;
 double recovery_cpu=0,recovery_wall=0;
 bool found=false;Witness witness;Recovery recovery;
 for(size_t i=0;i<sizes[0]&&!found;++i){
  const auto&e0=pools[0][i];ordinary.clear();
  for(size_t j=0;j<sizes[1];++j)if(pools[1][j].x!=e0.x){
   den[j]=f.sub(pools[1][j].x,e0.x);ordinary.push_back(j);}
  if(!ordinary.empty()){
   ++batch_rows;batch_inputs+=ordinary.size();
   prefix[0]=den[ordinary[0]];
   for(size_t v=1;v<ordinary.size();++v)prefix[v]=f.mul(prefix[v-1],den[ordinary[v]]);
   U acc=f.inv(prefix[ordinary.size()-1]);
   for(size_t v=ordinary.size()-1;v>0;--v){inverse[ordinary[v]]=f.mul(prefix[v-1],acc);acc=f.mul(acc,den[ordinary[v]]);}
   inverse[ordinary[0]]=acc;
  }
  for(size_t j=0;j<sizes[1]&&!found;++j){
   const auto&e1=pools[1][j];++positions;
   if(e0.x==e1.x)++exceptions;
   for(int sj:{1,-1}){
    ++signed_pairs;U xx,yy=0,slope=0;bool have_y=false;
    if(e0.x==e1.x){
     Pt q=add(f,a,{e0.x,e0.y,false},{e1.x,sj==1?e1.y:f.neg(e1.y),false});
     if(q.inf){++infinity;continue;}xx=q.x;yy=q.y;have_y=true;
    }else{
     U num=f.sub(sj==1?e1.y:f.neg(e1.y),e0.y);
     slope=f.mul(num,inverse[j]);xx=f.sub(f.sub(f.mul(slope,slope),e0.x),e1.x);
    }
    ++lookups;auto hit=index.find(xx);if(hit==index.end())continue;++hits;
    if(!have_y)yy=f.sub(f.mul(slope,f.sub(e0.x,xx)),e0.y);
    for(size_t k:hit->second){
     const U target_y=pools[2][k].y;
     for(int sk:{1,-1}){
      if(f.add(yy,sk==1?target_y:f.neg(target_y)))continue;
      ++candidates;Witness w{i,j,k,sj,sk};
      Counts before=f.counts;double tr=cpu(),wr=wall();
      Recovery test=verify(f,a,r,P,Q,pools,w);
      recovery_cpu+=cpu()-tr;recovery_wall+=wall()-wr;
      verify_mul+=f.counts.mul-before.mul;verify_inv+=f.counts.inv-before.inv;verify_group+=f.counts.group-before.group;
      if(!test.geo)return 4;
      if(test.verified){found=true;witness=w;recovery=test;break;}++uninformative;
     }if(found)break;
    }if(found)break;
   }
  }
 }
 const double te=cpu(),we=wall();
 sum_pre_cpu+=tp-t0;sum_search_cpu+=te-tp-recovery_cpu;sum_rec_cpu+=recovery_cpu;sum_cpu+=te-t0;
 sum_pre_wall+=wp-w0;sum_search_wall+=we-wp-recovery_wall;sum_rec_wall+=recovery_wall;sum_wall+=we-w0;
 if(repetition+1<repetitions)continue;
 search_mul=f.counts.mul-verify_mul;search_inv=f.counts.inv-verify_inv;
 struct rusage usage;getrusage(RUSAGE_SELF,&usage);
 std::cout.precision(17);
 std::cout<<"{\"algorithm\":\"streamed-batched-pair-enumeration-cpp\",\"status\":\""<<(found?"relation":"none")<<"\",\"witness\":";
 if(found)std::cout<<"[{\"pool\":0,\"index\":"<<witness.i<<",\"sign\":1},{\"pool\":1,\"index\":"<<witness.j<<",\"sign\":"<<witness.sj<<"},{\"pool\":2,\"index\":"<<witness.k<<",\"sign\":"<<witness.sk<<"}]";
 else std::cout<<"null";
 std::cout<<",\"recovery\":";
 if(found)std::cout<<"{\"A\":"<<recovery.A<<",\"B\":"<<recovery.B<<",\"k\":"<<recovery.k<<",\"geometric_zero\":true,\"label_relation_zero\":true,\"informative\":true,\"recovered_scalar_verified\":true}";
 else std::cout<<"null";
 std::cout<<",\"counters\":{\"pair_positions_scanned\":"<<positions<<",\"signed_pair_candidates\":"<<signed_pairs<<",\"x_lookups\":"<<lookups<<",\"x_hits\":"<<hits<<",\"candidate_witnesses\":"<<candidates<<",\"uninformative_relations\":"<<uninformative<<",\"exceptional_denominators\":"<<exceptions<<",\"pair_sums_at_infinity\":"<<infinity<<",\"batch_rows\":"<<batch_rows<<",\"batch_inputs\":"<<batch_inputs<<",\"fp_multiplications_explicit_search\":"<<search_mul<<",\"modular_inversions_search\":"<<search_inv<<",\"witness_fp_multiplications\":"<<verify_mul<<",\"witness_fp_inversions\":"<<verify_inv<<",\"witness_group_additions\":"<<verify_group<<"}";
 U vm_hwm=0;
 std::ifstream status("/proc/self/status");std::string key,rest;
 while(status>>key){if(key=="VmHWM:"){status>>vm_hwm;break;}std::getline(status,rest);}
 std::cout<<",\"phase_cpu_seconds\":{\"preprocessing\":"<<sum_pre_cpu/repetitions<<",\"search\":"<<sum_search_cpu/repetitions<<",\"witness_recovery\":"<<sum_rec_cpu/repetitions<<"},\"phase_wall_seconds\":{\"preprocessing\":"<<sum_pre_wall/repetitions<<",\"search\":"<<sum_search_wall/repetitions<<",\"witness_recovery\":"<<sum_rec_wall/repetitions<<"},\"total_cpu_seconds\":"<<sum_cpu/repetitions<<",\"total_wall_seconds\":"<<sum_wall/repetitions<<",\"peak_rss_kib\":"<<vm_hwm<<",\"getrusage_peak_rss_including_pre_exec_kib\":"<<usage.ru_maxrss;
 size_t payload=0;for(auto&v:pools)payload+=v.capacity()*sizeof(Entry);
 payload+=(den.capacity()+prefix.capacity()+inverse.capacity())*sizeof(U)+ordinary.capacity()*sizeof(size_t);
 std::cout<<",\"vector_payload_bytes_including_inputs\":"<<payload<<",\"memory_note\":\"VmHWM includes current executable; vector payload excludes unordered-map nodes/buckets; total auxiliary space O(B)\",\"batch_repetitions\":"<<repetitions<<",\"batch_cpu_seconds\":"<<cpu()-batch_start_cpu<<",\"batch_wall_seconds\":"<<wall()-batch_start_wall<<",\"repeat_scope\":\"every call reparses public input, builds index and batches, searches and recovers witness; stdout only final result\"}\n";
 }
}
