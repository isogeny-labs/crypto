/* vclose.c -- skeptic's own degree-D closure (VERIFY of closure-falls-crosscheck, 2026-11-12).
 * Written without reading fallclose.c or iclose.c internals.
 * W = least subspace of B_{<=D} (multilinear GF(2) polys) containing F, closed under f -> x_i f
 * for f in W with deg f < D.  Method: rounds; each round stacks the current reduced basis with a
 * block of new products and re-runs a full M4RI reduced echelon form (mzd_echelonize_m4ri, full=1).
 * Columns: degree descending; inside a degree, ordered by an xorshift hash of the mask with SEED.
 * Low rows (pivot degree < D) have support of degree < D only.  Products are formed for each
 * low pivot column the first time it appears (leading-monomial sets only grow, so this spans x_i*W_low).
 * Usage: vclose D SYSFILE SEED BLOCK FULL   (FULL=0: stop when 1 in W)
 * Output: one JSON line. */
#include <m4ri/m4ri.h>
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <time.h>
#include <sys/resource.h>
typedef uint64_t u64;
static int N, D; static long NC; static u64 *cmask; static int *cdeg; static u64 seed;
static u64 hsh(u64 x){ x ^= seed; x ^= x>>33; x*=0xff51afd7ed558ccdULL; x^=x>>33; x*=0xc4ceb9fe1a85ec53ULL; x^=x>>33; return x; }
/* open-addressing map mask -> column */
static u64 *hk; static long *hv; static long HS;
static void hput(u64 k,long v){ long i=hsh(k)&(HS-1); while(hv[i]>=0) i=(i+1)&(HS-1); hk[i]=k; hv[i]=v; }
static long hget(u64 k){ long i=hsh(k)&(HS-1); while(hv[i]>=0){ if(hk[i]==k) return hv[i]; i=(i+1)&(HS-1);} fprintf(stderr,"missing mono\n"); exit(3); }
static int cmpcol(const void*a,const void*b){ u64 x=*(const u64*)a,y=*(const u64*)b; int dx=__builtin_popcountll(x),dy=__builtin_popcountll(y);
  if(dx!=dy) return dy-dx;
  u64 hx=hsh(x^0x9e3779b97f4a7c15ULL),hy=hsh(y^0x9e3779b97f4a7c15ULL); return hx<hy?-1:hx>hy?1:(x<y?-1:x>y); }
int main(int argc,char**argv){
  if(argc<6){fprintf(stderr,"usage\n");return 1;}
  D=atoi(argv[1]); seed=strtoull(argv[3],0,10)*0x9E3779B97F4A7C15ULL+1; long BLK=atol(argv[4]); int FULL=atoi(argv[5]);
  FILE*f=fopen(argv[2],"r"); int neq; if(fscanf(f,"%d %d",&N,&neq)!=2) return 2;
  /* enumerate monomials deg<=D */
  long cap=1; for(int d=1,c=1;d<=D;d++){ c=c*(N-d+1)/d; cap+=c; } (void)cap;
  long nc=0; u64 *ms=malloc(sizeof(u64)*4000000);
  /* simple generation */
  ms[nc++]=0;
  for(int a=0;a<N;a++){ ms[nc++]=1ULL<<a; }
  if(D>=2) for(int a=0;a<N;a++) for(int b=a+1;b<N;b++) ms[nc++]=(1ULL<<a)|(1ULL<<b);
  if(D>=3) for(int a=0;a<N;a++) for(int b=a+1;b<N;b++) for(int c=b+1;c<N;c++) ms[nc++]=(1ULL<<a)|(1ULL<<b)|(1ULL<<c);
  if(D>=4) for(int a=0;a<N;a++) for(int b=a+1;b<N;b++) for(int c=b+1;c<N;c++) for(int e=c+1;e<N;e++) ms[nc++]=(1ULL<<a)|(1ULL<<b)|(1ULL<<c)|(1ULL<<e);
  if(D>4){fprintf(stderr,"D<=4 only\n");return 1;}
  NC=nc; qsort(ms,nc,sizeof(u64),cmpcol); cmask=ms; cdeg=malloc(sizeof(int)*nc);
  for(long i=0;i<nc;i++) cdeg[i]=__builtin_popcountll(ms[i]);
  HS=1; while(HS<4*nc) HS<<=1; hk=malloc(sizeof(u64)*HS); hv=malloc(sizeof(long)*HS); for(long i=0;i<HS;i++) hv[i]=-1;
  for(long i=0;i<nc;i++) hput(ms[i],i);
  long firstlow=0; while(firstlow<nc && cdeg[firstlow]==D) firstlow++;
  long one_col=hget(0); if(one_col!=nc-1){fprintf(stderr,"const col not last\n");return 4;}
  mzd_t *A=mzd_init(nc+BLK+neq, nc); long r=0;
  for(int e=0;e<neq;e++){ int cnt; if(fscanf(f,"%d",&cnt)!=1) return 2; for(int t=0;t<cnt;t++){ unsigned long long m; if(fscanf(f,"%llu",&m)!=1) return 2;
      long c=hget(m); mzd_write_bit(A,r,c, mzd_read_bit(A,r,c)^1);} r++; }
  fclose(f);
  time_t t0=time(0);
  mzd_t *W=mzd_init_window(A,0,0,r,nc); long rank=mzd_echelonize_m4ri(W,1,0); mzd_free_window(W);
  long gen_rank=rank;
  char *done=calloc(nc,1); long products=0, rounds=0; int one=0;
  u64 *supp=malloc(sizeof(u64)*nc);
  long *pivc=malloc(sizeof(long)*(nc+BLK+neq));
  for(;;){
    /* pivot columns of rows 0..rank-1 */
    long c=0; for(long i=0;i<rank;i++){ while(!mzd_read_bit(A,i,c)) c++; pivc[i]=c; }
    if(rank>0 && pivc[rank-1]==one_col){ one=1; if(!FULL) break; }
    long nr=rank; int any=0;
    for(long i=0;i<rank && nr<rank+BLK;i++){
      long p=pivc[i]; if(p<firstlow || done[p]) continue;
      /* ensure whole product set of this row fits */
      if(nr+N>rank+BLK) break;
      done[p]=1; any=1;
      /* collect support (cols >= p; all of degree < D) */
      long ns=0; for(long cc=p; cc<nc; cc++) if(mzd_read_bit(A,i,cc)) supp[ns++]=cmask[cc];
      for(int x=0;x<N;x++){ u64 bit=1ULL<<x; int nz=0;
        for(long t=0;t<ns;t++){ u64 m=supp[t]|bit; long nc2=hget(m); mzd_write_bit(A,nr,nc2, mzd_read_bit(A,nr,nc2)^1); }
        /* row may cancel to zero (e.g. x*(x+1)=0); keep only nonzero rows */
        for(long cc=0; cc<nc && !nz; cc+=64){ int w=(nc-cc<64)?(int)(nc-cc):64; if(mzd_read_bits(A,nr,cc,w)) nz=1; }
        if(nz){ nr++; products++; } }
    }
    if(!any) break;
    rounds++;
    W=mzd_init_window(A,0,0,nr,nc); rank=mzd_echelonize_m4ri(W,1,0); mzd_free_window(W);
    for(long i=rank;i<nr;i++) mzd_row_clear_offset(A,i,0);  /* rows beyond rank must be zero for the next block */
    fprintf(stderr,"round %ld rank %ld products %ld t=%lds\n",rounds,rank,products,(long)(time(0)-t0));
  }
  long c=0; long dims[5]={0,0,0,0,0};
  for(long i=0;i<rank;i++){ while(!mzd_read_bit(A,i,c)) c++; dims[cdeg[c]]++; }
  long cum[5]; long s=0; for(int d=0;d<=D;d++){ s+=dims[d]; cum[d]=s; }
  struct rusage ru; getrusage(RUSAGE_SELF,&ru);
  printf("{\"tool\":\"vclose\",\"N\":%d,\"n_eq\":%d,\"D\":%d,\"seed\":%s,\"one_in\":%d,\"W_dims\":[",N,neq,D,argv[3],one);
  for(int d=0;d<=D;d++) printf("%ld%s",cum[d],d<D?",":"");
  printf("],\"ncols\":%ld,\"rank\":%ld,\"codim\":%ld,\"gen_rank\":%ld,\"products\":%ld,\"rounds\":%ld,\"sec\":%ld,\"maxrss_kb\":%ld,\"full\":%d}\n",
    nc,rank,nc-rank,gen_rank,products,rounds,(long)(time(0)-t0),ru.ru_maxrss,FULL);
  return 0;
}
