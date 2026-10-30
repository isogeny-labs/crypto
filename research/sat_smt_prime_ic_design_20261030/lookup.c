// ns per hash lookup and ns per direct-subtraction candidate (x(R-F_i), x(R+F_i), 2 lookups), p ~ 2^62.
// Synthetic keys/coordinates: measures arithmetic + memory cost, not curve correctness.
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <time.h>
typedef uint64_t u64; typedef unsigned __int128 u128;
static const u64 P = 4611686018427387847ULL; // odd 62-bit modulus (primality not needed for timing)
static u64 PINV; // -P^-1 mod 2^64
static inline u64 mont(u128 t){ u64 m=(u64)t*PINV; u128 r=(t+(u128)m*P)>>64; u64 x=(u64)r; return x>=P?x-P:x; }
static inline u64 mmul(u64 a,u64 b){ return mont((u128)a*b); }
static inline u64 msub(u64 a,u64 b){ return a>=b?a-b:a+P-b; }
static inline u64 madd(u64 a,u64 b){ u64 s=a+b; return s>=P?s-P:s; }
static double now(){ struct timespec t; clock_gettime(CLOCK_MONOTONIC,&t); return t.tv_sec*1e9+t.tv_nsec; }
static u64 rs=88172645463325252ULL; static inline u64 rnd(){ rs^=rs<<13; rs^=rs>>7; rs^=rs<<17; return rs%P; }
static u64 *tab; static u64 mask;
static inline u64 h(u64 k){ return (k*0x9E3779B97F4A7C15ULL)>>20; }
static inline int look(u64 k){ u64 i=h(k)&mask; while(tab[i]){ if(tab[i]==k) return 1; i=(i+1)&mask; } return 0; }
int main(){
  u64 inv=1; for(int i=0;i<6;i++) inv*=2-P*inv; PINV=-inv;
  int sizes[]={16,20,24};
  for(int si=0; si<3; si++){
    int lg=sizes[si]; u64 n=1ULL<<lg; u64 cap=n*2; mask=cap-1;
    tab=calloc(cap,8); for(u64 i=0;i<n;i++){ u64 k=rnd()|1; u64 j=h(k)&mask; while(tab[j]) j=(j+1)&mask; tab[j]=k; }
    // pure lookup, random queries (almost all misses)
    long Q=20000000; u64 *qs=malloc(Q*8); for(long i=0;i<Q;i++) qs[i]=rnd()|1;
    double t0=now(); long hits=0; for(long i=0;i<Q;i++) hits+=look(qs[i]); double t1=now();
    printf("table 2^%d entries (%.0f MB): lookup %.1f ns (hits=%ld)\n",lg,cap*8/1e6,(t1-t0)/Q,hits);
    // direct subtraction: batch of 1024 candidates, one batched inversion, two x-coords each
    enum{BT=1024}; static u64 xi[BT],yi[BT],dx[BT],pre[BT],iv[BT];
    u64 xR=rnd(), yR=rnd(); long cand=0; double acc=0; long rep=20000; hits=0;
    for(int i=0;i<BT;i++){ xi[i]=rnd(); yi[i]=rnd(); }
    t0=now();
    for(long r=0;r<rep;r++){
      xi[r&(BT-1)]^=r; // keep compiler honest
      u64 run=1; for(int i=0;i<BT;i++){ dx[i]=msub(xR,xi[i]); if(!dx[i]) dx[i]=1; pre[i]=run; run=mmul(run,dx[i]); }
      u64 ir=run; for(int e=0;e<62;e++){ ir=mmul(ir,ir); } // stand-in for the single inversion (62 squarings)
      for(int i=BT-1;i>=0;i--){ u64 d=mmul(ir,pre[i]); ir=mmul(ir,dx[i]);
        u64 l1=mmul(msub(yR,yi[i]),d), l2=mmul(madd(yR,yi[i]),d);
        u64 x1=msub(msub(mmul(l1,l1),xR),xi[i]), x2=msub(msub(mmul(l2,l2),xR),xi[i]);
        hits+=look(x1|1)+look(x2|1); }
      cand+=BT;
    }
    t1=now(); printf("   direct-subtraction candidate (both signs, 2 lookups): %.1f ns/candidate (hits=%ld)\n",(t1-t0)/cand,hits);
    free(tab); free(qs); (void)acc;
  }
  return 0;
}
