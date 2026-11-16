#include "a.h"
#include "b.h"
#include "c.h"
int main(int argc,char**argv){
  long viol=0,tot=0,dec=0;
  for(int m=3;m<=4;m++)for(int s=4;s<=6;s++)for(int cap=2;cap<=8;cap*=2)for(int f=0;f<2;f++){
    std::vector<Inst> v; gen_inst(m,s,2,v); int n=m*s;
    for(auto&I:v) for(int r=0;r<20000;r++){
      u32 all=(1u<<n)-1; u32 S=(u32)rnd()&all, T=S&(u32)rnd(); u32 a=(u32)rnd()&all;
      int dT=decide(I,cap,f,T,a&T), dS=decide(I,cap,f,S,a&S); tot++; if(dT) dec++;
      if(dT && !dS) viol++; if(dT==1 && dS!=1) viol++; }
  }
  printf("monotonicity: %ld samples, %ld with T decided, %ld violations\n",tot,dec,viol); }
