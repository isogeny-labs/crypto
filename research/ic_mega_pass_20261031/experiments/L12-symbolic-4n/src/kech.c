/* kech: dense echelon over GF(2^m), m<=63, fixed column order (col 0 = highest pivot priority).
 * stdin : m modulus ncols split nrows ; rows: t (col coeff)*t   [coeff as unsigned decimal]
 * stdout: rank, then the RREF basis of (row space ∩ span(cols>=split)): nout, rows "t (col coeff)*t".
 * Used for the K-tensored closure of L12 (generic Frobenius model). */
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <string.h>
#include <arm_neon.h>
typedef uint64_t u64;
static int m; static u64 md; static u64 RT[8][256];
static inline u64 mul(u64 a,u64 b){
  poly128_t p=vmull_p64((poly64_t)a,(poly64_t)b); __uint128_t q=(__uint128_t)p;
  u64 lo=(u64)q, hi=(u64)(q>>64);
  u64 h=(lo>>m)|(m<64?(hi<<(64-m)):0); u64 l=lo&((m==64)?~0ULL:((1ULL<<m)-1));
  for(int k=0;k<8&&h;k++){ l^=RT[k][h&255]; h>>=8; }
  return l;
}
static u64 slowred(u64 hi_coef_shift){ return 0; }
static u64 inv(u64 a){ u64 r=1,b=a; /* a^(2^m-2) */ for(int i=1;i<m;i++){ b=mul(b,b); r=mul(r,b);} return r; }
int main(){
  long nc,split,nr; if(scanf("%d %llu %ld %ld %ld",&m,(unsigned long long*)&md,&nc,&split,&nr)!=5) return 3;
  /* RT[k][x] = (x << 8k) * X^m mod md, built by bitwise reduction */
  for(int k=0;k<8;k++) for(int x=0;x<256;x++){
    /* compute X^(m+8k+bit) mod md for set bits */
    u64 acc=0;
    for(int bit=0;bit<8;bit++) if(x>>bit&1){
      int e=m+8*k+bit; u64 v=1; /* X^e mod md via repeated shift */
      for(int i=0;i<e;i++){ v<<=1; if(v>>m&1) v^=md; }
      acc^=v; }
    RT[k][x]=acc; }
  u64 **piv=calloc(nc,sizeof(u64*)); long rank=0;
  u64 *r=malloc(nc*8);
  for(long i=0;i<nr;i++){
    int t; if(scanf("%d",&t)!=1) return 4; memset(r,0,nc*8); long first=nc;
    for(int j=0;j<t;j++){ long c; unsigned long long v; if(scanf("%ld %llu",&c,&v)!=2) return 5; r[c]^=v; if(c<first) first=c; }
    for(long c=first;c<nc;c++){
      if(!r[c]) continue;
      if(piv[c]){ u64 f=r[c], *q=piv[c]; for(long j=c;j<nc;j++) if(q[j]) r[j]^=mul(f,q[j]); continue; }
      u64 iv=inv(r[c]); for(long j=c;j<nc;j++) if(r[j]) r[j]=mul(iv,r[j]);
      piv[c]=malloc(nc*8); memcpy(piv[c],r,nc*8); rank++; break;
    }
  }
  /* RREF of block >= split */
  long nout=0;
  for(long c=nc-1;c>=split;c--) if(piv[c]){ nout++;
    for(long c2=split;c2<c;c2++) if(piv[c2]&&piv[c2][c]){ u64 f=piv[c2][c],*q=piv[c]; for(long j=c;j<nc;j++) if(q[j]) piv[c2][j]^=mul(f,q[j]); } }
  printf("%ld\n%ld\n",rank,nout);
  for(long c=split;c<nc;c++) if(piv[c]){ int t=0; for(long j=c;j<nc;j++) if(piv[c][j]) t++; printf("%d",t);
    for(long j=c;j<nc;j++) if(piv[c][j]) printf(" %ld %llu",j,(unsigned long long)piv[c][j]); printf("\n"); }
  return 0;
}
