// usage: dlog p a r gx gy out.bin  -> writes r pairs (x,y) uint32 for k*G, k=0..r-1 ((0,0) for k=0)
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
typedef int64_t i64; typedef uint64_t u64;
static i64 P;
static i64 inv(i64 a){ i64 t=0,nt=1,r=P,nr=a%P; while(nr){ i64 q=r/nr,tmp=t-q*nt;t=nt;nt=tmp;tmp=r-q*nr;r=nr;nr=tmp;} if(t<0)t+=P; return t;}
int main(int c,char**v){
  P=atoll(v[1]); i64 A=atoll(v[2]); i64 r=atoll(v[3]); i64 gx=atoll(v[4]), gy=atoll(v[5]);
  FILE*f=fopen(v[6],"wb"); uint32_t z[2]={0,0}; fwrite(z,4,2,f);
  i64 x=gx,y=gy;
  for(i64 k=1;k<r;k++){
    uint32_t w[2]={(uint32_t)x,(uint32_t)y}; fwrite(w,4,2,f);
    if(k==r-1) break;
    i64 l;
    if(x==gx){ if(y!=gy){fprintf(stderr,"order hit at %lld\n",(long long)k);return 1;} l=(3*(x*x%P)+A)%P*inv(2*y%P)%P; }
    else l=((y-gy)%P+P)%P*inv(((x-gx)%P+P)%P)%P;
    i64 nx=((l*l-x-gx)%P+2*P)%P; i64 ny=((l*((x-nx)%P+P)-y)%P+P)%P; x=nx;y=ny;
  }
  fclose(f); return 0; }
