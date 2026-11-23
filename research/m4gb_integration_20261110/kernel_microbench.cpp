#include <cstdint>
#include <cstdio>
#include <cstring>
#include <chrono>
#include <vector>
#include <random>
#ifndef P
#define P 31
#endif
typedef uint8_t e8; typedef uint16_t w16;
static uint8_t addmul[P][P][P];
__attribute__((noinline)) void k_table(e8* l, e8 c, const e8* r, int n){ for(int i=0;i<n;++i) l[i]=addmul[c][r[i]][l[i]]; }
__attribute__((noinline)) void k_mod(e8* l, e8 c, const e8* r, int n){ const w16 cc=c; for(int i=0;i<n;++i) l[i]=(e8)(((w16)l[i]+cc*(w16)r[i])%(w16)P); }
__attribute__((noinline)) void k_magic(e8* l, e8 c, const e8* r, int n){ const w16 cc=c; const w16 M=(w16)((65536+P-1)/P); for(int i=0;i<n;++i){ w16 t=(w16)l[i]+cc*(w16)r[i]; w16 q=(w16)(((uint32_t)t*M)>>16); l[i]=(e8)(t-q*P);} }
__attribute__((noinline)) void k_xor(e8* l, e8 c, const e8* r, int n){ for(int i=0;i<n;++i) l[i]^=r[i]; }
int main(int argc,char**argv){ int n=argc>1?atoi(argv[1]):4096; int reps=argc>2?atoi(argv[2]):200000;
 for(int c=0;c<P;++c)for(int r=0;r<P;++r)for(int l=0;l<P;++l) addmul[c][r][l]=(uint8_t)((l+c*r)%P);
 std::vector<e8> L(n),R(n),L0(n); std::mt19937 g(1); for(int i=0;i<n;++i){L0[i]=g()%P;R[i]=g()%P;}
 struct K{const char*nm; void(*f)(e8*,e8,const e8*,int);} ks[]={{"table",k_table},{"mod",k_mod},{"magic",k_magic},{"xor(ref)",k_xor}};
 for(auto&k:ks){ L=L0; auto t0=std::chrono::steady_clock::now(); for(int j=0;j<reps;++j) k.f(L.data(),(e8)(3+(j%(P-4))),R.data(),n); double s=std::chrono::duration<double>(std::chrono::steady_clock::now()-t0).count();
  unsigned long chk=0; for(auto v:L) chk=chk*131+v; printf("%-9s %7.3f ns/elem  chk=%lu\n",k.nm,s*1e9/((double)n*reps),chk);} }
