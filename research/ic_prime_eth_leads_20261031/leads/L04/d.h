struct Cfg{ int cap; bool filt; int budget_s; };
static u64 g_rng(u64&st){ st+=0x9E3779B97F4A7C15ULL; u64 z=st; z=(z^(z>>30))*0xBF58476D1CE4E5B9ULL; z=(z^(z>>27))*0x94D049BB133111EBULL; return z^(z>>31);}
static bool isBD(const Inst&I,const Cfg&C,u32 mask,u64&st){
  for(int r=0;r<24;r++){ u32 a=(u32)g_rng(st)&mask; if(decide(I,C.cap,C.filt,mask,a)==0) return false; }
  u32 a=0; do{ if(decide(I,C.cap,C.filt,mask,a)==0) return false; a=(a-mask)&mask; }while(a);
  return true;
}
static u32 swp(u32 S,int s){ u32 W=(1u<<s)-1; u32 b0=S&W,b1=(S>>s)&W; return (S&~(W|(W<<s)))|(b1)|(b0<<s); }
static u32 canon(u32 S,int s){ u32 t=swp(S,s); return t<S?t:S; }
struct Res{ int bstar,ub; std::vector<long> lv; std::string prof; double sec; bool timeout; long sound_bad; std::string snd; };
static Res search(const Inst&I,const Cfg&C,u64 seed){
  Res R; R.timeout=false; R.bstar=-1; R.sound_bad=0; u64 st=seed; int n=I.m*I.s, s=I.s;
  auto t0=std::chrono::steady_clock::now();
  auto el=[&](){return std::chrono::duration<double>(std::chrono::steady_clock::now()-t0).count();};
  // soundness
  { long bad=0; std::string sn;
    u32 all=(n==32)?~0u:((1u<<n)-1);
    if(I.sat){ int pl=0; u32 pa=0; for(int i=0;i<I.m;i++) pa|=I.planted[i]<<(i*s);
      if(decide(I,C.cap,C.filt,all,pa)==1) bad++;
      for(int r=0;r<300;r++){ u32 mk=(u32)g_rng(st)&all; if(decide(I,C.cap,C.filt,mk,pa&mk)==1) bad++; } (void)pl; }
    else { int N=(n<=16)?(1<<n):3000; for(int r=0;r<N;r++){ u32 a=(n<=16)?(u32)r:((u32)g_rng(st)&all); if(decide(I,C.cap,C.filt,all,a)!=1) bad++; } }
    if(n<=16){ long solved=0; for(u32 a=0;a<(1u<<n);a++) if(decide(I,C.cap,C.filt,all,a)==2) solved++;
      if((solved>0)!=I.sat) bad+=1000; sn="nsol="+std::to_string(solved); }
    R.sound_bad=bad; R.snd=sn; }
  // upper bound check: first m-1 windows fully fixed
  { u32 ubm=(1u<<((I.m-1)*s))-1; R.ub=isBD(I,C,ubm,st)?(I.m-1)*s:-1; }
  // upward exhaustive search (backdoors are upward closed): first k with a backdoor
  u32 lim=(n==32)?0:(1u<<n); int found=-1; u32 first=0;
  for(int k=0;k<=n&&found<0&&!R.timeout;k++){
    long cnt=0;
    if(k==0){ if(isBD(I,C,0,st)){found=0;first=0;} cnt=1; }
    else { u32 c=(1u<<k)-1;
      while(c<lim){ if(c==canon(c,s)){ cnt++; if(isBD(I,C,c,st)){found=k;first=c;break;} }
        if((cnt&1023)==0 && el()>C.budget_s){R.timeout=true;break;}
        u32 t=c&-c, r=c+t; c=(((r^c)>>2)/t)|r; } }
    R.lv.push_back(cnt);
  }
  if(R.timeout||found<0){ R.bstar=R.timeout?-2:-1; R.sec=el(); return R; }
  R.bstar=found;
  // profile: collect up to 200 verified minimal backdoors
  std::vector<std::pair<std::string,long>> pr; long nb=0; double tl=el()+C.budget_s/2.0;
  { int k=found; u32 c=(k==0)?0:((1u<<k)-1);
    while(nb<200 && el()<tl){
      if(c==canon(c,s) && isBD(I,C,c,st)){ nb++; std::string key; for(int i=0;i<I.m;i++){ key+=std::to_string(__builtin_popcount((c>>(i*s))&((1u<<s)-1))); key+=(i+1<I.m?",":""); }
        bool f=false; for(auto&q:pr) if(q.first==key){q.second++;f=true;break;} if(!f) pr.push_back({key,1}); }
      if(k==0||c>=lim) break; u32 t=c&-c, r=c+t; c=(((r^c)>>2)/t)|r; if(c>=lim) break; } }
  R.sec=el();
  std::sort(pr.begin(),pr.end(),[](auto&a,auto&b){return a.second>b.second;});
  for(size_t i=0;i<pr.size()&&i<4;i++) R.prof+=pr[i].first+"x"+std::to_string(pr[i].second)+(i+1<pr.size()&&i<3?";":"");
  R.prof+=" nback>="+std::to_string(nb); return R;
}
