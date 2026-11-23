#include "a.h"
#include "b.h"
#include "c.h"
#include "d.h"
int main(int argc,char**argv){
  if(argc<8){ fprintf(stderr,"usage: m s cap filt nper ncurves seed [threads budget]\n"); return 1; }
  int m=atoi(argv[1]),s=atoi(argv[2]); Cfg C; C.cap=atoi(argv[3]); C.filt=atoi(argv[4]); int nper=atoi(argv[5]),nc=atoi(argv[6]);
  rs^=strtoull(argv[7],0,10)*0x9E3779B97F4A7C15ULL; int nt=argc>8?atoi(argv[8]):4; C.budget_s=argc>9?atoi(argv[9]):300;
  std::vector<Inst> insts; std::vector<int> curve;
  for(int c=0;c<nc;c++){ size_t b=insts.size(); gen_inst(m,s,nper,insts); for(size_t i=b;i<insts.size();i++) curve.push_back(c); }
  { // S3 self-test against EC addition
    long bad=0,tot=0; for(auto&I:insts){ std::vector<Pt> W=wpts(I); for(int r=0;r<200;r++){ Pt a=W[rnd()%W.size()],b=W[rnd()%W.size()],c=padd(I,a,b); if(c.inf) continue; tot++;
      if(!s3zero(I,a.x,b.x,c.x)) bad++; u64 o[2]; int k=roots(I,a.x,b.x,o); bool f=(k<0); for(int t=0;t<k;t++) if(o[t]==c.x) f=true; if(!f) bad++; } }
    printf("SELFTEST s3 vs EC: %ld checks, %ld bad\n",tot,bad); }
  printf("CELL m=%d s=%d cap=%d filt=%d insts=%zu\n",m,s,C.cap,(int)C.filt,insts.size());
  std::vector<Res> res(insts.size()); std::atomic<size_t> nx(0); std::mutex mu;
  auto work=[&](){ for(;;){ size_t i=nx++; if(i>=insts.size()) return; Res r=search(insts[i],C,0xABCDEF+i*7919);
      std::lock_guard<std::mutex> g(mu); res[i]=r;
      printf("INST m=%d s=%d cap=%d filt=%d curve=%d p=%llu %s ub=%d bstar=%d off=%d sound_bad=%ld %s sec=%.2f lv=",m,s,C.cap,(int)C.filt,curve[i],(unsigned long long)insts[i].p,insts[i].sat?"SAT":"UNSAT",r.ub,r.bstar,(m-1)*s-r.bstar,r.sound_bad,r.snd.c_str(),r.sec);
      for(size_t q=0;q<r.lv.size();q++) printf("%ld%s",r.lv[q],q+1<r.lv.size()?",":""); printf(" prof=%s\n",r.prof.c_str()); fflush(stdout); } };
  std::vector<std::thread> th; for(int t=0;t<nt;t++) th.emplace_back(work); for(auto&t:th) t.join();
  return 0;
}
