struct Pt{u64 x,y;bool inf;};
static Pt padd(const Inst&I,Pt a,Pt b){
  u64 p=I.p; if(a.inf) return b; if(b.inf) return a; u64 lam;
  if(a.x==b.x){ if((a.y+b.y)%p==0) return Pt{0,0,true}; lam=I.mul((3*I.mul(a.x,a.x)+I.A)%p,I.inv(2*a.y%p)); }
  else lam=I.mul((b.y+p-a.y)%p,I.inv((b.x+p-a.x)%p));
  u64 x3=(I.mul(lam,lam)+2*p-a.x-b.x)%p; u64 y3=(I.mul(lam,(a.x+p-x3)%p)+p-a.y)%p; return Pt{x3,y3,false};
}
static Pt pneg(const Inst&I,Pt a){ if(a.inf) return a; return Pt{a.x,(I.p-a.y)%I.p,false}; }
static bool isprime(u64 n){ if(n<2) return false; for(u64 d=2;d*d<=n;d++) if(n%d==0) return false; return true; }
static u64 genprime(int bits){ for(;;){ u64 c=(1ULL<<(bits-1))|(rnd()&((1ULL<<(bits-1))-1)); c|=3; if((c&3)==3 && isprime(c)) return c; } }
static std::vector<Pt> wpts(const Inst&I){
  std::vector<Pt> w; for(u32 x=0;x<(1u<<I.s);x++){ u64 y; if(I.sq(I.f(x),y)){ w.push_back(Pt{x,y,false}); if(y) w.push_back(Pt{x,I.p-y,false}); } } return w; }
// independent EC brute force: is R = +-P1+-P2..+-Pm (chain-representable) with x_i<2^s ?
static bool ecsat(const Inst&I){
  std::vector<Pt> W=wpts(I); Pt R{I.xR,I.yR,false};
  if(I.m==3){
    std::unordered_set<u64> X2; for(auto&a:W)for(auto&b:W){Pt c=padd(I,a,b); if(!c.inf) X2.insert(c.x);}
    for(auto&c:W){ Pt q=padd(I,R,pneg(I,c)); if(!q.inf && X2.count(q.x)) return true; }
    return false;
  }
  std::vector<Pt> S2; std::unordered_set<u64> X2;
  for(auto&a:W)for(auto&b:W){Pt c=padd(I,a,b); if(!c.inf){S2.push_back(c);X2.insert(c.x);}}
  if(X2.count(R.x) && !W.empty()) return true;
  for(auto&a:S2){ Pt q=padd(I,R,pneg(I,a)); if(!q.inf && X2.count(q.x)) return true; }
  return false;
}
static bool nonsing(const Inst&I){ u64 p=I.p; u64 d=(4*I.mul(I.mul(I.A,I.A),I.A)+27*I.mul(I.B,I.B))%p; return d!=0; }
// build instances for one curve
static void gen_inst(int m,int s,int nper,std::vector<Inst>&out){
  Inst I; I.m=m; I.s=s; I.p=genprime(m*s+2);
  do{ I.A=1+rnd()%(I.p-1); I.B=1+rnd()%(I.p-1);}while(!nonsing(I));
  I.valid.assign(1u<<s,0); int nv=0; for(u32 x=0;x<(1u<<s);x++){u64 y; if(I.sq(I.f(x),y)){I.valid[x]=1;nv++;}}
  std::vector<Pt> W=wpts(I); if(W.size()<4){ fprintf(stderr,"tiny window\n"); return; }
  int ns=0,nu=0; long tries=0;
  while((ns<nper||nu<nper) && tries++<2000000){
    Inst J=I;
    if(ns<nper && (nu>=nper || (tries&1))){
      std::vector<Pt> pts; Pt acc{0,0,true}; bool ok=true;
      for(int i=0;i<m;i++){ Pt q=W[rnd()%W.size()]; pts.push_back(q); acc=padd(I,acc,q); if(i>=1 && acc.inf) ok=false; }
      if(!ok||acc.inf) continue;
      J.xR=acc.x; J.yR=acc.y; J.sat=true; for(auto&q:pts) J.planted.push_back((u32)q.x);
      out.push_back(J); ns++;
    } else if(nu<nper){
      u64 x=rnd()%I.p,y; if(!I.sq(I.f(x),y)||y==0) continue; J.xR=x; J.yR=y; J.sat=false;
      if(ecsat(J)) continue; out.push_back(J); nu++;
    }
  }
}
