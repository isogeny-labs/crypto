// Exact HKY closure V_{F,c} in R = F_p[z1,z2]/(z1^K-1, z2^K-1), deg z^k = popcount(k).
// Usage: closure p t cmax tlimit_sec  < coeffs (9 ints c[e1][e2], e in {0,1,2}, row-major)
// Generator f = sum c[e1][e2] z1^e1 z2^e2. Variables y_{i,j} = z_i^{2^j}.
// Output per c: c dimV dimLow(deg<=c-1) unit(0/1) secs
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <string.h>
#include <time.h>
typedef uint64_t u64;
static u64 P; static int T, K;
static u64 pw(u64 a,u64 e){u64 r=1;a%=P;while(e){if(e&1)r=r*a%P;a=a*a%P;e>>=1;}return r;}
static double now(){struct timespec s;clock_gettime(CLOCK_MONOTONIC,&s);return s.tv_sec+1e-9*s.tv_nsec;}
typedef struct {int n; int *c; uint32_t *v;} Row;
int main(int argc,char**argv){
  P=strtoull(argv[1],0,10); T=atoi(argv[2]); K=1<<T; int cmax=atoi(argv[3]); double tl=atof(argv[4]);
  long long cf[9]; for(int i=0;i<9;i++) if(scanf("%lld",&cf[i])!=1) return 2;
  double t0=now(); int prevdim=-1; int ffd=-1, dF=-1, censored=0;
  int NM=K*K; int *deg=malloc(sizeof(int)*NM);
  for(int k1=0;k1<K;k1++)for(int k2=0;k2<K;k2++) deg[k1*K+k2]=__builtin_popcount(k1)+__builtin_popcount(k2);
  for(int c=2;c<=cmax;c++){
    // column order: degree descending
    int *col=malloc(sizeof(int)*NM); int n=0;
    for(int d=c;d>=0;d--) for(int m=0;m<NM;m++) if(deg[m]==d) col[m]=n++;
    for(int m=0;m<NM;m++) if(deg[m]>c) col[m]=-1;
    int *mon=malloc(sizeof(int)*n); for(int m=0;m<NM;m++) if(col[m]>=0) mon[col[m]]=m;
    int *piv=malloc(sizeof(int)*n); for(int i=0;i<n;i++)piv[i]=-1;
    Row *rows=malloc(sizeof(Row)*(n+1)); int nr=0;
    int *queue=malloc(sizeof(int)*(n+1)); int qh=0,qt=0;
    u64 *acc=calloc(n,sizeof(u64));
    int unit=0; int stopped=0;
    // insert function inline via loop over candidates
    // candidate buffer: monomial list
    int cand_n; int cand_m[64*1024]; u64 cand_v[64*1024];
    // first candidate: f
    int phase=0; int qi=-1, vj=0;
    while(1){
      // produce candidate into acc
      int lead=n;
      if(phase==0){ for(int e1=0;e1<3;e1++)for(int e2=0;e2<3;e2++){ long long x=cf[e1*3+e2]; if(!x)continue; u64 xv=((x%(long long)P)+P)%P;
          int m=(e1%K)*K+(e2%K); int cc=col[m]; acc[cc]=(acc[cc]+xv)%P; if(cc<lead)lead=cc;} phase=1; }
      else {
        if(vj==0){ if(qh==qt) break; qi=queue[qh++]; }
        int i=vj/T, j=vj%T; int sh=1<<j; Row *r=&rows[qi];
        for(int s=0;s<r->n;s++){ int m=mon[r->c[s]]; int k1=m/K,k2=m%K; if(i==0)k1=(k1+sh)&(K-1); else k2=(k2+sh)&(K-1);
          int cc=col[k1*K+k2]; acc[cc]=(acc[cc]+r->v[s])%P; if(cc<lead)lead=cc; }
        vj++; if(vj==2*T) vj=0;
      }
      // reduce
      int newp=-1;
      for(int cc=lead;cc<n;cc++){ if(!acc[cc])continue; int pr=piv[cc];
        if(pr<0){ newp=cc; break; }
        u64 a=acc[cc]; Row *r=&rows[pr];
        for(int s=0;s<r->n;s++){ int q=r->c[s]; acc[q]=(acc[q]+(P-a)*r->v[s])%P; }
      }
      if(newp<0) continue;
      // extract row normalized
      u64 inv=pw(acc[newp],P-2); int cnt=0; for(int cc=newp;cc<n;cc++) if(acc[cc])cnt++;
      Row R; R.n=cnt; R.c=malloc(sizeof(int)*cnt); R.v=malloc(sizeof(uint32_t)*cnt); int s=0;
      for(int cc=newp;cc<n;cc++) if(acc[cc]){ R.c[s]=cc; R.v[s]=acc[cc]*inv%P; s++; acc[cc]=0; }
      rows[nr]=R; piv[newp]=nr;
      if(deg[mon[newp]]==0){ unit=1; nr++; break; }
      if(deg[mon[newp]]<=c-1) queue[qt++]=nr;
      nr++;
      if(now()-t0>tl){ stopped=1; break; }
    }
    int low=0; for(int r=0;r<nr;r++) if(deg[mon[rows[r].c[0]]]<=c-1) low++;
    printf("c=%d ncols=%d dimV=%d dimLow=%d unit=%d stopped=%d secs=%.2f\n",c,n,nr,low,unit,stopped,now()-t0); fflush(stdout);
    if(ffd<0 && prevdim>=0 && low>prevdim && !stopped) ffd=c;
    prevdim=nr;
    for(int r=0;r<nr;r++){free(rows[r].c);free(rows[r].v);} free(rows);free(queue);free(acc);free(piv);free(mon);free(col);
    if(stopped){censored=1; printf("RESULT dF>=%d ffd=%d censored=1\n",c,ffd); return 0;}
    if(unit){dF=c; printf("RESULT dF=%d ffd=%d censored=0\n",dF,ffd); return 0;}
  }
  printf("RESULT dF>%d ffd=%d censored=1\n",cmax,ffd); return 0;
}
