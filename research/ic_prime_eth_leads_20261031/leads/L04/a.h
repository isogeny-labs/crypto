#include <cstdio>
#include <cstdlib>
#include <cstdint>
#include <cstring>
#include <string>
#include <vector>
#include <algorithm>
#include <unordered_set>
#include <thread>
#include <mutex>
#include <atomic>
#include <chrono>
typedef uint64_t u64; typedef uint32_t u32;
struct Inst{
  u64 p,A,B,xR,yR; int m,s; bool sat; std::vector<u32> planted; std::vector<char> valid;
  u64 mul(u64 a,u64 b)const{return a*b%p;}
  u64 pw(u64 a,u64 e)const{u64 r=1;a%=p;while(e){if(e&1)r=r*a%p;a=a*a%p;e>>=1;}return r;}
  u64 inv(u64 a)const{return pw(a,p-2);}
  bool sq(u64 a,u64&r)const{ if(a==0){r=0;return true;} r=pw(a,(p+1)/4); return r*r%p==a; }
  u64 f(u64 x)const{return (mul(mul(x,x),x)+mul(A,x)+B)%p;}
};
static u64 rs=88172645463325252ULL;
static u64 rnd(){ u64 z=(rs+=0x9E3779B97F4A7C15ULL); z=(z^(z>>30))*0xBF58476D1CE4E5B9ULL; z=(z^(z>>27))*0x94D049BB133111EBULL; return z^(z>>31);}
// roots of S3(x1,x2,z)=0 in z. returns count (0..2) or -1 if identically zero (no info)
static int roots_raw(const Inst&I,u64 x1,u64 x2,u64*out){
  u64 p=I.p; u64 s=(x1+x2)%p, pr=I.mul(x1,x2);
  u64 d=(x1+p-x2)%p, d2=I.mul(d,d);
  u64 c=(I.mul(s,(pr+I.A)%p)+2*I.B)%p;
  u64 q=I.mul((pr+p-I.A)%p,(pr+p-I.A)%p); q=(q+p-I.mul(4*I.B%p,s))%p;
  if(d2==0){ if(c==0) return q==0?-1:0; out[0]=I.mul(q,I.inv(2*c%p)); return 1; }
  u64 D=(I.mul(c,c)+p-I.mul(d2,q))%p, r;
  if(!I.sq(D,r)) return 0;
  u64 iv=I.inv(d2); out[0]=I.mul((c+r)%p,iv);
  if(r==0) return 1;
  out[1]=I.mul((c+p-r)%p,iv); return 2;
}
static bool s3zero(const Inst&I,u64 x1,u64 x2,u64 x3){
  u64 p=I.p; u64 s=(x1+x2)%p, pr=I.mul(x1,x2);
  u64 d=(x1+p-x2)%p, d2=I.mul(d,d);
  u64 c=(I.mul(s,(pr+I.A)%p)+2*I.B)%p;
  u64 q=I.mul((pr+p-I.A)%p,(pr+p-I.A)%p); q=(q+p-I.mul(4*I.B%p,s))%p;
  u64 v=(I.mul(d2,I.mul(x3,x3))+p-I.mul(2*c%p,x3)+q)%p; return v==0;
}

struct CEnt{ u64 k; int n; u64 r[2]; };
static int roots(const Inst&I,u64 x1,u64 x2,u64*out){
  static thread_local std::vector<CEnt> tab; static thread_local const Inst* cur=0; static thread_local u64 curp=0,curA=0,curB=0;
  if(tab.empty()) tab.assign(1<<18,CEnt{~0ULL,0,{0,0}});
  if(cur!=&I||curp!=I.p||curA!=I.A||curB!=I.B){ for(auto&e:tab) e.k=~0ULL; cur=&I; curp=I.p; curA=I.A; curB=I.B; }
  if(x1>x2){u64 t=x1;x1=x2;x2=t;}
  u64 key=(x1<<32)|x2; u64 h=(key*0x9E3779B97F4A7C15ULL)>>46; CEnt&e=tab[h];
  if(e.k==key){ out[0]=e.r[0]; out[1]=e.r[1]; return e.n; }
  int n=roots_raw(I,x1,x2,out); e.k=key; e.n=n; e.r[0]=out[0]; e.r[1]=out[1]; return n;
}
