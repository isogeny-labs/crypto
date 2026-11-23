/* enum2: enumerate x(kG), k=0..L-1, on y^2+xy=x^3+a x^2+b over F_2^n = F_2[w]/(poly).
   usage: enum2 n polymask a b h l mode(P|E) seed outfile
   mode P: G = generator of the order-l subgroup; L=l.  mode E: G of order h*l (cyclic); L=h*l.
   writes: uint32 x-values, x(O) written as 0xFFFFFFFF; prints G coords and checks L*G=O, order exact. */
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <string.h>
typedef uint64_t u64; static int n; static u64 poly, A, B;
static u64 mul(u64 x,u64 y){u64 r=0;while(y){if(y&1)r^=x;y>>=1;x<<=1;if(x>>n&1)x^=poly;}return r;}
static u64 sq(u64 x){return mul(x,x);}
static u64 inv(u64 x){ /* Fermat: x^(2^n-2) */ u64 r=1,b=x; u64 e=(1ULL<<n)-2; while(e){if(e&1)r=mul(r,b);b=mul(b,b);e>>=1;} return r;}
static int tr(u64 x){u64 t=x,s=x;for(int i=1;i<n;i++){t=sq(t);s^=t;}return (int)(s&1);}
static u64 htr(u64 c){/* half-trace, n odd: sum_{i=0}^{(n-1)/2} c^{2^{2i}} */ u64 s=c,t=c;for(int i=1;i<=(n-1)/2;i++){t=sq(sq(t));s^=t;}return s;}
typedef struct{u64 x,y;int inf;}P;
static P add(P p,P q){P r; if(p.inf)return q; if(q.inf)return p;
 u64 l; if(p.x==q.x){ if(p.y!=q.y || p.x==0){r.inf=1;r.x=r.y=0;return r;} /* doubling */
   l=p.x^mul(p.y,inv(p.x)); r.x=sq(l)^l^A; r.y=sq(p.x)^mul(l^1,r.x); r.inf=0; return r;}
 l=mul(p.y^q.y,inv(p.x^q.x)); r.x=sq(l)^l^p.x^q.x^A; r.y=mul(l,p.x^r.x)^r.x^p.y; r.inf=0; return r;}
static P smul(P p,u64 k){P r={0,0,1};while(k){if(k&1)r=add(r,p);p=add(p,p);k>>=1;}return r;}
static int oncurve(P p){ if(p.inf) return 1; u64 l=sq(p.y)^mul(p.x,p.y), r=mul(sq(p.x),p.x)^mul(A,sq(p.x))^B; return l==r;}
static u64 rs=88172645463325252ULL; static u64 rnd(){rs^=rs<<13;rs^=rs>>7;rs^=rs<<17;return rs;}
static P randpt(){ for(;;){u64 x=rnd()&((1ULL<<n)-1); if(!x)continue; u64 rhs=mul(sq(x),x)^mul(A,sq(x))^B; u64 c=mul(rhs,inv(sq(x))); if(tr(c))continue; u64 z=htr(c); P p={x,mul(x,z),0}; if(!oncurve(p)){fprintf(stderr,"bad pt\n");exit(1);} return p;}}
int main(int argc,char**argv){ if(argc<10){fprintf(stderr,"usage\n");return 1;}
 n=atoi(argv[1]); poly=strtoull(argv[2],0,10); A=strtoull(argv[3],0,10); B=strtoull(argv[4],0,10);
 u64 h=strtoull(argv[5],0,10), l=strtoull(argv[6],0,10); char mode=argv[7][0]; rs^=strtoull(argv[8],0,10)*0x9E3779B97F4A7C15ULL; if(!rs)rs=1;
 u64 L = mode=='P'? l : h*l; P G;
 for(;;){ P R=randpt(); if(mode=='P'){ G=smul(R,h); if(!G.inf) break; }
   else { G=R; if(smul(G,l).inf) continue; if(smul(G,h*l/2).inf) continue; if(h==4 && smul(G,2*l).inf) continue; break; } }
 if(!smul(G,L).inf){fprintf(stderr,"order check failed\n");return 2;}
 FILE*f=fopen(argv[9],"wb"); uint32_t *buf=malloc(4*L); P Q={0,0,1}; 
 for(u64 k=0;k<L;k++){ buf[k]= Q.inf?0xFFFFFFFFu:(uint32_t)Q.x; if(k>0 && Q.inf){fprintf(stderr,"early O at %llu\n",(unsigned long long)k);return 3;} Q=add(Q,G);} 
 if(!Q.inf){fprintf(stderr,"L*G!=O in walk\n");return 4;}
 fwrite(buf,4,L,f); fclose(f);
 printf("{\"Gx\":%llu,\"Gy\":%llu,\"L\":%llu}\n",(unsigned long long)G.x,(unsigned long long)G.y,(unsigned long long)L); return 0;}
