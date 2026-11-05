// S_4 membership toy: pair table vs no-table vs 2-torsion quotient. Single thread.
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <string.h>
typedef uint64_t u64; typedef int64_t i64;
static u64 P, A2, A4; // curve y^2 = x^3 + A2 x^2 + A4 x  (= x(x-al)(x-be))
static u64 cI=0, cM=0;
static u64 rs=88172645463325252ULL; static u64 rnd(){rs^=rs<<13;rs^=rs>>7;rs^=rs<<17;return rs;}
static inline u64 mm(u64 a,u64 b){cM++;return (unsigned __int128)a*b%P;}
static inline u64 ad(u64 a,u64 b){u64 c=a+b;return c>=P?c-P:c;}
static inline u64 sb(u64 a,u64 b){return a>=b?a-b:a+P-b;}
static u64 inv(u64 a){cI++;i64 t=0,nt=1,r=P,nr=a;while(nr){i64 q=r/nr,tmp=t-q*nt;t=nt;nt=tmp;tmp=r-q*nr;r=nr;nr=tmp;}return t<0?t+P:t;}
static u64 pw(u64 a,u64 e){u64 r=1;while(e){if(e&1)r=(unsigned __int128)r*a%P;a=(unsigned __int128)a*a%P;e>>=1;}return r;}
typedef struct{u64 x,y;int inf;}pt;
static pt add(pt a,pt b){ // full affine add, counts ops
  pt r={0,0,1}; if(a.inf)return b; if(b.inf)return a; u64 l;
  if(a.x==b.x){ if((a.y+b.y)%P==0) return r; // O
    u64 num=ad(ad(mm(3,mm(a.x,a.x)),mm(2*A2%P,a.x)),A4); l=mm(num,inv(ad(a.y,a.y)));}
  else l=mm(sb(b.y,a.y),inv(sb(b.x,a.x)));
  r.inf=0; r.x=sb(sb(sb(mm(l,l),A2),a.x),b.x); r.y=sb(mm(l,sb(a.x,r.x)),a.y); return r;}
static pt neg(pt a){if(!a.inf)a.y=a.y?P-a.y:0;return a;}
// x of a+b and a-b sharing one inversion (generic case); returns flags for O
static int xpm(pt a,pt b,u64*xs,u64*xd){ // returns bit0: sum is O, bit1: diff is O
  if(a.inf){ if(b.inf) return 3; *xs=*xd=b.x; return 0;} if(b.inf){*xs=*xd=a.x;return 0;}
  if(a.x==b.x){ pt s=add(a,b),d=add(a,neg(b)); int f=0; if(s.inf)f|=1;else *xs=s.x; if(d.inf)f|=2;else *xd=d.x; return f;}
  u64 iv=inv(sb(b.x,a.x)); u64 l1=mm(sb(b.y,a.y),iv), l2=mm(P-ad(b.y,a.y)==P?0:P-ad(b.y,a.y),iv);
  // diff: a + (-b): slope (-b.y - a.y)/(b.x-a.x)
  *xs=sb(sb(sb(mm(l1,l1),A2),a.x),b.x); *xd=sb(sb(sb(mm(l2,l2),A2),a.x),b.x); return 0;}
static int isqrt(u64 a,u64*r){ if(a==0){*r=0;return 1;} if(pw(a,(P-1)/2)!=1)return 0; *r=pw(a,(P+1)/4); return 1;}
static u64 rhs(u64 x){return (unsigned __int128)x*(( (unsigned __int128)x*x%P + (unsigned __int128)A2*x%P + A4)%P)%P;}
static pt rndpt(){ for(;;){u64 x=rnd()%P,y; if(isqrt(rhs(x),&y)&&y){pt r={x,(rnd()&1)?y:P-y,0};return r;}}}
static uint8_t*bm_new(){return calloc(P/8+1,1);} 
#define BS(b,x) (b[(x)>>3]|=1<<((x)&7))
#define BT(b,x) ((b[(x)>>3]>>((x)&7))&1)
static pt T[4]; // O, T1,T2,T3
static int n, n0, mode; static pt *F, *F0;
static u64 popc(uint8_t*b){u64 c=0;for(u64 i=0;i<=P/8;i++)c+=__builtin_popcount(b[i]);return c;}
int main(int argc,char**argv){
  P=strtoull(argv[1],0,10); n=atoi(argv[2]); mode=atoi(argv[3]); int ntg=atoi(argv[4]),npl=atoi(argv[5]); rs^=strtoull(argv[6],0,10)*0x9E3779B97F4A7C15ULL;
  u64 al=rnd()%P, be=rnd()%P; A2=(2*P-al-be)%P; A4=(unsigned __int128)al*be%P; // x(x-al)(x-be)
  T[0].inf=1; T[1]=(pt){0,0,0}; T[2]=(pt){al,0,0}; T[3]=(pt){be,0,0};
  int nsh = mode==0?1:(mode==1?2:4); n0=n/nsh; F=malloc(sizeof(pt)*n); F0=malloc(sizeof(pt)*n0);
  uint8_t*X1=bm_new(); int k=0;
  while(k<n0){ pt q=rndpt(); int ok=1; pt cand[4]; for(int s=0;s<nsh;s++){cand[s]=add(q,T[s]); if(cand[s].inf||cand[s].y==0||BT(X1,cand[s].x))ok=0;}
    for(int s=0;s<nsh&&ok;s++)for(int t=0;t<s;t++)if(cand[s].x==cand[t].x)ok=0; if(!ok)continue;
    F0[k]=q; for(int s=0;s<nsh;s++){F[k*nsh+s]=cand[s];BS(X1,cand[s].x);} k++; }
  uint8_t*X10=bm_new(); for(int i=0;i<n0;i++)BS(X10,F0[i].x);
  // ground truth X3 over full F
  uint8_t*X3=bm_new(); cI=cM=0; int hasO=1;
  for(int i=0;i<n;i++)BS(X3,F[i].x); // Pi - Pi + Pk
  for(int i=0;i<n;i++)for(int j=i;j<n;j++){ pt S=add(F[i],F[j]),D=add(F[i],neg(F[j]));
    pt Q[2]={S,D}; for(int q=0;q<2;q++){ if(Q[q].inf) continue; for(int kk=0;kk<n;kk++){u64 xs,xd;int f=xpm(Q[q],F[kk],&xs,&xd); if(!(f&1))BS(X3,xs); if(!(f&2))BS(X3,xd);} } }
  u64 nX3=popc(X3);
  // Method A pair table on the base used by method: full F (A,B) and F0 (quotient)
  pt*G = mode==0?F:F0; int g=mode==0?n:n0; uint8_t*PX=bm_new(); cI=cM=0; u64 S=0;
  for(int i=0;i<g;i++)for(int j=i;j<g;j++){u64 xs,xd;int f=xpm(G[i],G[j],&xs,&xd); if(!(f&1)){if(!BT(PX,xs))S++;BS(PX,xs);} if(!(f&2)){if(!BT(PX,xd))S++;BS(PX,xd);} }
  u64 buildI=cI,buildM=cM; uint8_t*XG= mode==0?X1:X10;
  // targets
  u64 mism=0, pos=0, verified=0, onI_A=0,onM_A=0,onI_B=0,onM_B=0, nB=0, mismB=0, hits_rand=0;
  int ntot=ntg+npl;
  for(int t=0;t<ntot;t++){
    pt R; if(t<ntg) R=rndpt(); else { pt s=add(add(F[rnd()%n],(rnd()&1)?F[rnd()%n]:neg(F[rnd()%n])),(rnd()&1)?F[rnd()%n]:neg(F[rnd()%n])); if(s.inf){t--;continue;} R=s; }
    int truth=BT(X3,R.x);
    // method A / quotient
    cI=cM=0; int dec=0; int fs=-1,fk=-1; u64 fv=0;
    for(int s=0;s<nsh&&!dec;s++){ pt R2=add(R,T[s]); if(R2.inf) continue; if(BT(XG,R2.x)&&hasO){dec=1;fs=s;fk=-2;break;}
      for(int kk=0;kk<g;kk++){u64 xs,xd;int f=xpm(R2,G[kk],&xs,&xd); if(!(f&1)&&BT(PX,xs)){dec=1;fs=s;fk=kk;fv=xs;break;} if(!(f&2)&&BT(PX,xd)){dec=1;fs=s;fk=kk;fv=xd;break;}} }
    onI_A+=cI; onM_A+=cM; if(dec!=truth)mism++; if(dec){pos++; if(t<ntg)hits_rand++;
      // recover & verify decomposition explicitly (uncounted)
      u64 sI=cI,sM=cM; int ok=0; pt R2=add(R,T[fs]);
      if(fk==-2) ok=1; else for(int i=0;i<g&&!ok;i++)for(int j=i;j<g&&!ok;j++)for(int e2=0;e2<2&&!ok;e2++){pt Q=add(G[i],e2?neg(G[j]):G[j]); if(Q.inf||Q.x!=fv)continue;
          for(int e1=0;e1<2&&!ok;e1++)for(int e3=0;e3<2&&!ok;e3++){pt Z=add(add(e1?neg(Q):Q,e3?neg(G[fk]):G[fk]),T[fs]); if(!Z.inf&&Z.x==R.x)ok=1;}}
      verified+=ok; cI=sI;cM=sM; }
    // method B (no table) on full F, only for first 200 targets and only mode 0 (cost n^2)
    if(t<100||t>=ntg&&t<ntg+20){ cI=cM=0; int db=BT(X1,R.x);
      for(int i=0;i<n&&!db;i++)for(int j=i;j<n&&!db;j++){pt S2=add(F[i],F[j]),D2=add(F[i],neg(F[j])); pt Q[2]={S2,D2};
        for(int q=0;q<2&&!db;q++){if(Q[q].inf)continue; u64 xs,xd;int f=xpm(R,Q[q],&xs,&xd); if((!(f&1)&&BT(X1,xs))||(!(f&2)&&BT(X1,xd)))db=1;}}
      onI_B+=cI;onM_B+=cM;nB++; if(db!=truth)mismB++; }
  }
  printf("{\"p\":%llu,\"n\":%d,\"mode\":%d,\"n0\":%d,\"nX3\":%llu,\"S_pairtable\":%llu,\"build_I\":%llu,\"build_M\":%llu,"
    "\"targets\":%d,\"planted\":%d,\"mismatch_A\":%llu,\"positives\":%llu,\"verified\":%llu,\"hits_random\":%llu,"
    "\"onA_I_per\":%.2f,\"onA_M_per\":%.2f,\"B_tests\":%llu,\"mismatch_B\":%llu,\"onB_I_per\":%.2f,\"onB_M_per\":%.2f}\n",
    (unsigned long long)P,n,mode,n0,(unsigned long long)nX3,(unsigned long long)S,(unsigned long long)buildI,(unsigned long long)buildM,ntg,npl,
    (unsigned long long)mism,(unsigned long long)pos,(unsigned long long)verified,(unsigned long long)hits_rand,(double)onI_A/ntot,(double)onM_A/ntot,
    (unsigned long long)nB,(unsigned long long)mismB,nB?(double)onI_B/nB:0,nB?(double)onM_B/nB:0);
  return 0;}
