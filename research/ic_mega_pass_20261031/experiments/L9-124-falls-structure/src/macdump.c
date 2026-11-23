/* macdump (L9): copy of L1 macclose + argv[1]=maxround, argv[2]=dump prefix: after each round dumps low-block pivot rows (packed bits) and low monomial list.
 * macclose: independent Boolean degree-D Macaulay closure over GF(2).
 * Columns: degree-D monomials first (lex), ..., constant last. Pivot = lowest set bit.
 * Monomial->column by open-addressing hash. Round-based closure with per-round logs.
 * stdin: "N D neq" then per equation "t m1 .. mt" (monomial bitmasks).
 * stdout: JSON. stderr: per-round log lines. */
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <string.h>
#include <time.h>
typedef uint64_t u64;
static int N, D; static long nc, W, lowstart; /* columns >= lowstart have degree <= D-1 */
static u64 *mon; static int *cdeg;
static u64 *hk; static long *hv; static long hsz;
static u64 hh(u64 x){ x^=x>>31; x*=0x9E3779B97F4A7C15ULL; x^=x>>29; return x; }
static void hput(u64 m,long c){ long i=hh(m)&(hsz-1); while(hv[i]>=0) i=(i+1)&(hsz-1); hk[i]=m; hv[i]=c; }
static long hget(u64 m){ long i=hh(m)&(hsz-1); while(hv[i]>=0){ if(hk[i]==m) return hv[i]; i=(i+1)&(hsz-1);} return -1; }
static long cnt=0;
static void gen(int e,int start,int left,u64 m){ /* lex enumeration of e-subsets */
  if(left==0){ mon[cnt]=m; cdeg[cnt]=e; cnt++; return; }
  for(int v=start; v<=N-left; v++) gen(e,v+1,left-1,m|(1ULL<<v)); }
static u64 **piv; static long rank=0, lowrank=0; static long bydeg[8];
static int one=0;
static long *newlow; static long nnew=0;
/* reduce row (dense, W words) ; returns pivot col inserted or -1 */
static long insert(u64 *r){
  long w=0;
  for(;;){
    while(w<W && !r[w]) w++;
    if(w==W) return -1;
    long p=w*64+__builtin_ctzll(r[w]);
    if(piv[p]){ u64 *q=piv[p]; for(long i=w;i<W;i++) r[i]^=q[i]; continue; }
    piv[p]=malloc(W*8); memcpy(piv[p],r,W*8); rank++; bydeg[cdeg[p]]++;
    if(p>=lowstart){ lowrank++; newlow[nnew++]=p; }
    if(p==nc-1) one=1;
    return p;
  }
}
static inline void tog(u64 *r,u64 m){ long c=hget(m); if(c<0){fprintf(stderr,"bad mono\n");exit(7);} r[c>>6]^=1ULL<<(c&63); }

static char *pref=0; static int maxround=99;
static void dumpround(int rd){
  if(!pref) return; char fn[1024]; long nl=nc-lowstart, WL=(nl+63)/64;
  if(rd==0){ sprintf(fn,"%s_monos.txt",pref); FILE*f=fopen(fn,"w"); for(long c=lowstart;c<nc;c++) fprintf(f,"%llu\n",(unsigned long long)mon[c]); fclose(f);}
  sprintf(fn,"%s_r%d.bin",pref,rd); FILE*f=fopen(fn,"wb"); long cntr=0; for(long c=lowstart;c<nc;c++) if(piv[c]) cntr++;
  fwrite(&nl,8,1,f); fwrite(&cntr,8,1,f); u64 *o=malloc(WL*8);
  for(long c=lowstart;c<nc;c++) if(piv[c]){ memset(o,0,WL*8); u64*g=piv[c]; for(long j=lowstart;j<nc;j++) if((g[j>>6]>>(j&63))&1){ long q=j-lowstart; o[q>>6]|=1ULL<<(q&63);} fwrite(o,8,WL,f);} 
  fclose(f); free(o); }
int main(int argc,char**argv){ if(argc>1) maxround=atoi(argv[1]); if(argc>2) pref=argv[2];

  int neq; if(scanf("%d %d %d",&N,&D,&neq)!=3) return 3;
  nc=0; { u64 b=1; long s=0; for(int e=0;e<=D;e++){ /* C(N,e) */ u64 c=1; for(int i=0;i<e;i++) c=c*(N-i)/(i+1); s+=c; (void)b;} nc=s; }
  W=(nc+63)/64; mon=malloc(nc*8); cdeg=malloc(nc*sizeof(int));
  for(int e=D;e>=0;e--){ if(e==D-1) lowstart=cnt; if(e==0){ mon[cnt]=0; cdeg[cnt]=0; cnt++; } else gen(e,0,e,0); }
  if(D==0) lowstart=0;
  if(cnt!=nc){fprintf(stderr,"count bug\n");return 2;}
  hsz=1; while(hsz<4*nc) hsz<<=1; hk=malloc(hsz*8); hv=malloc(hsz*sizeof(long)); for(long i=0;i<hsz;i++) hv[i]=-1;
  for(long c=0;c<nc;c++) hput(mon[c],c);
  piv=calloc(nc,sizeof(u64*)); newlow=malloc(nc*sizeof(long));
  long *cur=malloc(nc*sizeof(long)); long ncur=0;
  u64 *r=malloc(W*8); clock_t t0=clock();
  for(int i=0;i<neq;i++){ int t; if(scanf("%d",&t)!=1) return 3; memset(r,0,W*8); int dg=0;
    for(int j=0;j<t;j++){ unsigned long long m; if(scanf("%llu",&m)!=1) return 3; tog(r,m); int e=__builtin_popcountll(m); if(e>dg) dg=e; }
    if(dg<=D) insert(r); }
  int round=0; long nprod=0;
  fprintf(stderr,"round %d rank %ld low %ld bydeg",round,rank,lowrank); for(int e=0;e<=D;e++) fprintf(stderr," %ld",bydeg[e]); fprintf(stderr,"\n");
  dumpround(0);
  while(nnew && !one && round<maxround){
    memcpy(cur,newlow,nnew*sizeof(long)); ncur=nnew; nnew=0; round++;
    for(long a=0;a<ncur && !one;a++){
      u64 *g=piv[cur[a]];
      for(int v=0; v<N && !one; v++){
        memset(r,0,W*8); u64 bv=1ULL<<v;
        for(long w=lowstart>>6; w<W; w++){ u64 x=g[w]; while(x){ long c=w*64+__builtin_ctzll(x); x&=x-1; tog(r,mon[c]|bv);} }
        nprod++; insert(r);
      }
    }
    fprintf(stderr,"round %d rank %ld low %ld bydeg",round,rank,lowrank); for(int e=0;e<=D;e++) fprintf(stderr," %ld",bydeg[e]); fprintf(stderr," new_low %ld cpu %.1f\n",nnew,(double)(clock()-t0)/CLOCKS_PER_SEC); dumpround(round);
  }
  long dim=rank, low=lowrank; if(one){ dim=nc; low=nc-lowstart; }
  printf("{\"dim\":%ld,\"low\":%ld,\"one\":%d,\"ncols\":%ld,\"rounds\":%d,\"nprod\":%ld,\"bydeg\":[",dim,low,one,nc,round,nprod);
  for(int e=0;e<=D;e++) printf("%s%ld",e?",":"",bydeg[e]);
  printf("],\"cpu\":%.2f}\n",(double)(clock()-t0)/CLOCKS_PER_SEC); return 0; }
