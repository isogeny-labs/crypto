/* Smoke workload for tools/test_profile.sh: two hot functions with different
   per-instruction cost (a GF(2^64)-style shift-and-xor multiply and a dependent
   64-bit multiply chain), so instruction share and time share differ. */
#include <stdio.h>
#include <stdint.h>
__attribute__((noinline)) uint64_t gf2_mul(uint64_t a, uint64_t b){uint64_t r=0;for(int i=0;i<64;i++){if(b&1)r^=a;b>>=1;a=(a<<1)^((a>>63)?0x1b:0);}return r;}
__attribute__((noinline)) uint64_t sq_chain(uint64_t x,int n){for(int i=0;i<n;i++)x=gf2_mul(x,x);return x;}
__attribute__((noinline)) uint64_t mix(uint64_t x,int n){uint64_t s=0;for(int i=0;i<n;i++){s+=x*0x9e3779b97f4a7c15ULL;x^=s>>17;}return s;}
int main(){uint64_t acc=1;for(int k=0;k<4000;k++){acc^=sq_chain(acc+k,300);acc+=mix(acc,20000);}printf("%llu\n",(unsigned long long)acc);return 0;}
