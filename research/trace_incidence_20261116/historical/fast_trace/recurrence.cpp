#include <cstdint>
#include <vector>
#include <algorithm>
using U=uint64_t;
using W=__uint128_t;
static U mul(U a,U b,U p){return W(a)*b%p;}
static U powmod(U x,U n,U p){U out=1;while(n){if(n&1)out=mul(out,x,p);x=mul(x,x,p);n>>=1;}return out;}
extern "C" int recurrence(const U* s,size_t count,U p,U* result,size_t capacity){
 std::vector<U>C(1,1),B(1,1);size_t L=0,shift=1;U last=1;
 for(size_t n=0;n<count;++n){
  U delta=s[n];for(size_t i=1;i<=L;++i)delta=(W(delta)+W(C[i])*s[n-i])%p;
  if(!delta){++shift;continue;}
  auto old=C;U factor=mul(delta,powmod(last,p-2,p),p);
  if(C.size()<B.size()+shift)C.resize(B.size()+shift,0);
  for(size_t j=0;j<B.size();++j){U v=mul(factor,B[j],p);C[j+shift]=(W(C[j+shift])+p-v)%p;}
  if(2*L<=n){L=n+1-L;B=old;last=delta;shift=1;}else ++shift;
 }
 if(L+1>capacity)return -1;C.resize(L+1,0);
 for(size_t i=0;i<=L;++i)result[i]=C[L-i];
 return static_cast<int>(L+1);
}
