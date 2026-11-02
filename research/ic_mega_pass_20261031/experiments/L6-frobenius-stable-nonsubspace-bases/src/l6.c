// L6 core: GF(2^n) Koblitz y^2+xy=x^3+x^2+1 ; yields, ground truth, N1 lift test.
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <string.h>
typedef uint32_t fe;
static int n; static uint64_t F; static fe MASK; static uint64_t R_ORD; static fe A=1,B=1;
static fe mul(fe a, fe b){ uint64_t r=0; for(int i=0;i<n;i++) if(b>>i&1) r^=(uint64_t)a<<i;
  for(int i=2*n-2;i>=n;i--) if(r>>i&1) r^=F<<(i-n); return (fe)r; }
static fe sq(fe a){return mul(a,a);}
static fe inv(fe a){ fe r=1,b=a; uint64_t e=(1ull<<n)-2; while(e){ if(e&1) r=mul(r,b); b=sq(b); e>>=1;} return r; }
static int tr(fe a){ fe t=a,s=a; for(int i=1;i<n;i++){t=sq(t); s^=t;} return s&1; }
static fe htr(fe c){ fe s=c,t=c; for(int i=1;i<=(n-1)/2;i++){ t=sq(sq(t)); s^=t;} return s; }
typedef struct{fe x,y; int inf;} pt;
static pt add(pt P, pt Q){ pt R={0,0,0};
  if(P.inf) return Q; if(Q.inf) return P;
  if(P.x==Q.x){ if(P.y!=Q.y || P.x==0){R.inf=1;return R;}
    fe l=P.x^mul(P.y,inv(P.x)); R.x=sq(l)^l^A; R.y=sq(P.x)^mul(l^1,R.x); return R; }
  fe l=mul(P.y^Q.y,inv(P.x^Q.x)); R.x=sq(l)^l^P.x^Q.x^A; R.y=mul(l,P.x^R.x)^R.x^P.y; return R; }
static pt neg(pt P){ if(!P.inf) P.y^=P.x; return P; }
static pt smul(uint64_t k, pt P){ pt R={0,0,1}; while(k){ if(k&1) R=add(R,P); P=add(P,P); k>>=1;} return R; }
static int lift_x(fe x, pt *P){ if(!x) return 0; fe c=x^A^mul(B,inv(sq(x))); if(tr(c)) return 0;
  fe z=htr(c); P->x=x; P->y=mul(x,z); P->inf=0; return 1; }
static uint64_t key(pt P){ return P.inf? ~0ull : ((uint64_t)P.x<<32)|P.y; }
static int cmpu(const void*a,const void*b){uint64_t x=*(uint64_t*)a,y=*(uint64_t*)b; return x<y?-1:x>y;}
static uint64_t rng=0x9e3779b97f4a7c15ull; static uint64_t rnd(){ rng^=rng<<13; rng^=rng>>7; rng^=rng<<17; return rng; }
static long cntkey(uint64_t*a,long m,uint64_t k){ long lo=0,hi=m; while(lo<hi){long md=(lo+hi)/2; if(a[md]<k) lo=md+1; else hi=md;}
  long s=lo; hi=m; while(lo<hi){long md=(lo+hi)/2; if(a[md]<=k) lo=md+1; else hi=md;} return lo-s; }
static pt gen(){ for(fe x=2;;x++){ pt P; if(lift_x(x,&P)){ pt G=add(P,P); if(!G.inf && smul(R_ORD,G).inf) return G; } } }
static uint8_t *bm;
int main(int argc,char**argv){
  n=atoi(argv[2]); F=strtoull(argv[3],0,10); R_ORD=strtoull(argv[4],0,10); MASK=(fe)((1ull<<n)-1);
  bm=calloc((1ull<<n)/8+1,1);
  // read base x-values
  FILE*fp=fopen(argv[5],"r"); long cap=1<<20, nb=0; fe*xs=malloc(cap*4); unsigned v; while(fscanf(fp,"%u",&v)==1){ xs[nb++]=v; bm[v>>3]|=1<<(v&7);} fclose(fp);
  pt G=gen();
  if(!strcmp(argv[1],"selftest")){ // S3 on triples P1+P2+P3=O ; group order
    int ok=0,bad=0; for(int t=0;t<2000;t++){ pt P=smul(rnd()%R_ORD,G),Q=smul(rnd()%R_ORD,G); pt S=neg(add(P,Q)); if(P.inf||Q.inf||S.inf) continue;
      fe x1=P.x,x2=Q.x,x3=S.x; fe s=sq(mul(x1,x2)^mul(x1,x3)^mul(x2,x3))^mul(mul(x1,x2),x3)^B; if(s) bad++; else ok++; }
    printf("{\"s3_ok\":%d,\"s3_bad\":%d,\"rG_inf\":%d,\"G\":[%u,%u]}\n",ok,bad,smul(R_ORD,G).inf,G.x,G.y); return 0; }
  // build B = subgroup points with x in base
  pt*Bp=malloc(sizeof(pt)*2*nb); long m=0, oncurve=0, subx=0;
  for(long i=0;i<nb;i++){ pt P; if(!lift_x(xs[i],&P)) continue; oncurve++; if(!smul(R_ORD,P).inf) continue; subx++; Bp[m++]=P; Bp[m++]=neg(P); }
  if(!strcmp(argv[1],"yields")){
    int do3=atoi(argv[6]); int ntg=atoi(argv[7]);
    long N2=0; for(long i=0;i<m;i++) for(long j=0;j<m;j++){ pt S=add(Bp[i],Bp[j]); if(!S.inf && (bm[S.x>>3]>>(S.x&7)&1)) N2++; }
    double N3=-1; if(do3){ uint64_t*ps=malloc(sizeof(uint64_t)*m*m); long c=0; for(long i=0;i<m;i++) for(long j=0;j<m;j++) ps[c++]=key(add(Bp[i],Bp[j]));
      qsort(ps,c,8,cmpu); N3=0; for(long i=0;i<m;i++) for(long j=0;j<m;j++) N3+=cntkey(ps,c,key(add(Bp[i],neg(Bp[j])))); free(ps);
      // N3 counts (P1,P2,P3,Q): P1+P2 = Q-P3 i.e. P1+P2+P3 = Q in B
    }
    printf("{\"nb\":%ld,\"oncurve_x\":%ld,\"subgroup_x\":%ld,\"B_points\":%ld,\"N2\":%ld,\"N3\":%.0f,\"targets\":[",nb,oncurve,subx,m,N2,N3);
    rng=12345; for(int t=0;t<ntg;t++){ pt R=smul(1+rnd()%(R_ORD-1),G); long dec=0; for(long i=0;i<m;i++){ pt Q=add(R,neg(Bp[i])); if(!Q.inf && (bm[Q.x>>3]>>(Q.x&7)&1)) dec++; }
      printf("%s[%u,%u,%ld]",t?",":"",R.x,R.y,dec); }
    printf("]}\n"); return 0; }
  if(!strcmp(argv[1],"n1")){ // argv: n1 n f r basefile(cyc2 x list) expfile(pairs a b per line, same order) ntargets onbfile
    int ntg=atoi(argv[7]); int p=2*n+1; uint64_t PM=(1ull<<p)-1;
    int *ea=malloc(nb*4),*eb=malloc(nb*4); FILE*fe_=fopen(argv[6],"r"); for(long i=0;i<nb;i++) if(fscanf(fe_,"%d %d",&ea[i],&eb[i])!=2) return 2; fclose(fe_);
    fe onb[64]; FILE*fo=fopen(argv[8],"r"); for(int i=0;i<n;i++) fscanf(fo,"%u",&onb[i]); fclose(fo);
    // ONB coords of x: solve via precomputed inverse matrix: brute: build matrix rows -> gaussian each time (n small)
    #define CM(a,b) ({ uint64_t _r=0; for(int _i=0;_i<p;_i++) if((b)>>_i&1) _r^= (((a)<<_i)|((a)>>(p-_i)))&PM; _r; })
    uint64_t J=PM, e0=1;
    long symb=0,L2sym=0,L2asym=0; long sols=0, sols_oc=0, L1=0,L2=0,L3=0, L2oc=0, L3oc=0;
    rng=777; for(int t=0;t<ntg;t++){ pt R=smul(1+rnd()%(R_ORD-1),G); fe xr=R.x;
      // ONB coords of xr by gaussian elimination
      uint32_t coord=0; for(int bb_=0;bb_<n;bb_++) if(xr>>bb_&1) coord^=onb[bb_];
      uint64_t XR_onb=0; for(int i=0;i<n;i++) if(coord>>i&1){ long kk=1; for(int s=0;s<i;s++) kk=kk*2%p; XR_onb^=(1ull<<kk)|(1ull<<((p-kk)%p)); }
      uint64_t XR_pb=xr;
      fe xr2=sq(xr);
      for(long i=0;i<nb;i++){ fe x1=xs[i]; fe Aq=sq(x1)^xr2, Bq=mul(x1,xr), Cq=mul(sq(x1),xr2)^B; // Aq x2^2 + Bq x2 + Cq = 0
        fe roots[2]; int nr=0;
        if(Bq==0){ if(Aq==0){ if(Cq==0){ fprintf(stderr,"degenerate\n"); } } else { fe c=mul(Cq,inv(Aq)); fe r=c; for(int s=0;s<n-1;s++) r=sq(r); roots[nr++]=r; } }
        else if(Aq==0){ roots[nr++]=mul(Cq,inv(Bq)); }
        else { fe s=mul(Bq,inv(Aq)); fe c=mul(mul(Aq,Cq),inv(sq(Bq))); if(!tr(c)){ fe tt=htr(c); roots[nr++]=mul(s,tt); roots[nr++]=mul(s,tt^1);} }
        for(int q=0;q<nr;q++){ fe x2=roots[q]; if(!(bm[x2>>3]>>(x2&7)&1)) continue;
          long j=-1; for(long u=0;u<nb;u++) if(xs[u]==x2){j=u;break;}
          fe chk=sq(mul(x1,x2)^mul(x1,xr)^mul(x2,xr))^mul(mul(x1,x2),xr)^B; if(chk){fprintf(stderr,"root bug\n");return 4;}
          sols++; int sy=(ea[i]+eb[i]==p)&&(ea[j]+eb[j]==p); symb+=sy; pt P1,P2; int oc=lift_x(x1,&P1)&&lift_x(x2,&P2); sols_oc+=oc;
          uint64_t X1=(1ull<<ea[i])|(1ull<<eb[i]), X2=(1ull<<ea[j])|(1ull<<eb[j]);
          for(int conv=1;conv<=3;conv++){ uint64_t bb= conv==1? e0 : (e0^J); uint64_t X3= conv==3? XR_pb : XR_onb;
            uint64_t X12=CM(X1,X2); uint64_t t1=X12^CM(X1,X3)^CM(X2,X3); uint64_t s3=CM(t1,t1)^CM(X12,X3)^bb;
            if(!s3){ if(conv==1)L1++; if(conv==2){L2++;L2oc+=oc; if(sy)L2sym++; else L2asym++;} if(conv==3){L3++;L3oc+=oc;} } }
        } } }
    printf("{\"targets\":%d,\"s3_solutions_ordered\":%ld,\"s3_solutions_oncurve\":%ld,\"exact_L1\":%ld,\"exact_L2\":%ld,\"exact_L3\":%ld,\"exact_L2_oncurve\":%ld,\"exact_L3_oncurve\":%ld,\"both_symmetric\":%ld,\"exact_L2_symmetric\":%ld,\"exact_L2_asymmetric\":%ld}\n",ntg,sols,sols_oc,L1,L2,L3,L2oc,L3oc,symb,L2sym,L2asym);
    return 0; }
  return 1; }
