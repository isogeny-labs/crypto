#include <algorithm>
#include <array>
#include <cassert>
#include <cmath>
#include <cstdint>
#include <filesystem>
#include <fstream>
#include <iostream>
#include <map>
#include <set>
#include <sstream>
#include <stdexcept>
#include <string>
#include <unordered_map>
#include <vector>

// Exact small-field diagnostics. No curve secret is an input to either oracle.
using U=uint64_t;
struct Rng { U s; U next(){U z=(s+=0x9e3779b97f4a7c15ULL);z=(z^(z>>30))*0xbf58476d1ce4e5b9ULL;z=(z^(z>>27))*0x94d049bb133111ebULL;return z^(z>>31);} U range(U n){if(!n)throw std::runtime_error("empty range"); U z=~U(0)-(~U(0)%n),v;do{v=next();}while(v>=z);return v%n;} };
struct Cost { U ga=0,fm=0,fi=0; };
struct F {
 U p,d,q; bool ext; mutable Cost cost;
 F(U p_,bool e):p(p_),d(0),q(e?p_*p_:p_),ext(e){if(e){d=2;while(pow(d,(p-1)/2)==1)++d;}cost={};}
 U add(U a,U b)const{return ((a%p+b%p)%p)+p*((a/p+b/p)%p);}
 U neg(U a)const{return ((p-a%p)%p)+p*((p-a/p)%p);}
 U sub(U a,U b)const{return add(a,neg(b));}
 U mul(U a,U b)const{++cost.fm;U x=a%p,y=a/p,z=b%p,w=b/p;return (x*z+d*y*w)%p+p*((x*w+y*z)%p);}
 U pow(U a,U n)const{U r=1;while(n){if(n&1)r=mul(r,a);a=mul(a,a);n>>=1;}return r;}
 U inv(U a)const{if(!a)throw std::runtime_error("zero inverse");++cost.fi;return pow(a,q-2);}
 U div(U a,U b)const{return mul(a,inv(b));}
 void reset()const{cost={};}
};
struct P { U x=0,y=0;bool inf=true;auto operator<=>(const P&)const=default; };
struct Curve {
 F f;U a,b;std::unordered_map<U,std::vector<U>> roots;
 Curve(U p,bool ext,U aa,U bb):f(p,ext),a(aa),b(bb){for(U y=0;y<f.q;++y)roots[f.mul(y,y)].push_back(y);}
 U rhs(U x)const{return f.add(f.add(f.mul(f.mul(x,x),x),f.mul(a,x)),b);}
 bool on(P x)const{return x.inf||f.mul(x.y,x.y)==rhs(x.x);}
 P neg(P x)const{if(!x.inf)x.y=f.neg(x.y);return x;}
 P add(P x,P y)const{
  ++f.cost.ga;if(x.inf)return y;if(y.inf)return x;
  if(x.x==y.x&&f.add(x.y,y.y)==0)return {};
  U l;if(x.x==y.x){l=f.div(f.add(f.mul(3,f.mul(x.x,x.x)),a),f.mul(2,x.y));}
  else l=f.div(f.sub(y.y,x.y),f.sub(y.x,x.x));
  U xx=f.sub(f.sub(f.mul(l,l),x.x),y.x);return {xx,f.sub(f.mul(l,f.sub(x.x,xx)),x.y),false};
 }
 P mul(U n,P x)const{P z;while(n){if(n&1)z=add(z,x);x=add(x,x);n>>=1;}return z;}
 U key(P x)const{return x.inf?f.q*f.q:x.x*f.q+x.y;}
 std::vector<P> points()const{std::vector<P>r;for(U x=0;x<f.q;++x){auto it=roots.find(rhs(x));if(it!=roots.end())for(U y:it->second)r.push_back({x,y,false});}return r;}
 bool nonsingular()const{return f.add(f.mul(4%f.p,f.mul(f.mul(a,a),a)),f.mul(27%f.p,f.mul(b,b)))!=0;}
};
U largest_prime(U n){U best=1;for(U d=2;d*d<=n;++d)if(n%d==0){best=d;while(n%d==0)n/=d;}return std::max(best,n);}
bool prime(U n){return n>1&&largest_prime(n)==n;}
U modpow(U a,U n,U p){U r=1;while(n){if(n&1)r=r*a%p;a=a*a%p;n>>=1;}return r;}
std::string pj(P p){if(p.inf)return "null";return "["+std::to_string(p.x)+","+std::to_string(p.y)+"]";}
std::string cj(Cost c){return "{\"group_calls\":"+std::to_string(c.ga)+",\"field_mul_calls\":"+std::to_string(c.fm)+",\"field_inv_calls\":"+std::to_string(c.fi)+"}";}
struct Fixture {Curve c;U n,r,h;P g,t;Cost construction;U draws;};
static std::ostream* candidate_log=nullptr;
Fixture fixture(U p,bool ext,U seed){
 Rng rng{20261018+seed*0x100000001b3ULL+p};Cost total;
 for(U draw=1;draw<=512;++draw){
  U q=ext?p*p:p;U a=rng.range(q),e=rng.range(q);F f(p,ext);U b=ext?f.neg(f.add(f.mul(f.mul(e,e),e),f.mul(a,e))):rng.range(p);
  Curve c(p,ext,a,b);if(!c.nonsingular()){if(candidate_log)*candidate_log<<"{\"p\":"<<p<<",\"seed\":"<<seed<<",\"draw\":"<<draw<<",\"a\":"<<a<<",\"b\":"<<b<<",\"accepted\":false,\"reason\":\"singular\"}\n";total.fm+=c.f.cost.fm;continue;}auto pts=c.points();U n=pts.size()+1,r=largest_prime(n),h=n/r;
  if((!ext&&!prime(n))||(ext&&(r<p||h%2))){if(candidate_log)*candidate_log<<"{\"p\":"<<p<<",\"seed\":"<<seed<<",\"draw\":"<<draw<<",\"a\":"<<a<<",\"b\":"<<b<<",\"order\":"<<n<<",\"r\":"<<r<<",\"cofactor\":"<<h<<",\"accepted\":false,\"reason\":\"subgroup_condition\"}\n";total.fm+=c.f.cost.fm;continue;}
  for(P x:pts){P g=c.mul(h,x);if(!g.inf&&c.mul(r,g).inf){if(candidate_log)*candidate_log<<"{\"p\":"<<p<<",\"seed\":"<<seed<<",\"draw\":"<<draw<<",\"a\":"<<a<<",\"b\":"<<b<<",\"order\":"<<n<<",\"r\":"<<r<<",\"cofactor\":"<<h<<",\"accepted\":true}\n";total.ga+=c.f.cost.ga;total.fm+=c.f.cost.fm;total.fi+=c.f.cost.fi;return {std::move(c),n,r,h,g,ext?P{e,0,false}:P{},total,draw};}}
  throw std::runtime_error("no subgroup generator");
 }throw std::runtime_error("512 curve candidate cap");
}
struct Rho {bool ok=false;U k=0,restarts=0,steps=0,table_entries=0;Cost setup,online,verification;};
Rho rho(const Curve&c,P g,P q,U r,U seed){
 struct S{P x;U a,b;};Rng rng{seed};std::array<S,16>inc;Rho out;c.f.reset();
 for(auto &s:inc){s.a=rng.range(r);s.b=rng.range(r);s.x=c.add(c.mul(s.a,g),c.mul(s.b,q));}out.setup=c.f.cost;c.f.reset();
 auto canon=[&](S s){if(!s.x.inf&&c.key(c.neg(s.x))<c.key(s.x)){s.x=c.neg(s.x);s.a=(r-s.a)%r;s.b=(r-s.b)%r;}return s;};
 auto step=[&](S s){U j=(c.key(s.x)*0x9e3779b97f4a7c15ULL)>>60;auto z=inc[j];s={c.add(s.x,z.x),(s.a+z.a)%r,(s.b+z.b)%r};++out.steps;return s;};
 for(U attempt=0;attempt<32;++attempt){out.restarts=attempt;U a=rng.range(r),b=rng.range(r);S x={c.add(c.mul(a,g),c.mul(b,q)),a,b};std::unordered_map<U,S>seen;
  for(U i=0;i<100000;++i){S z=canon(x);auto it=seen.find(c.key(z.x));if(it!=seen.end()){S y=it->second;U den=(y.b+r-z.b)%r;if(den){out.k=((z.a+r-y.a)%r)*modpow(den,r-2,r)%r;out.ok=true;}break;}seen.emplace(c.key(z.x),z);out.table_entries=std::max(out.table_entries,U(seen.size()));x=step(x);}
  if(out.ok)break;
 }
 out.online=c.f.cost;c.f.reset();if(out.ok&&c.mul(out.k,g)!=q)throw std::runtime_error("rho wrong logarithm");out.verification=c.f.cost;return out;
}
struct Pair {size_t i,j;};
using Table=std::unordered_map<U,std::vector<Pair>>;
Table pairtable(const Curve &c,const std::vector<P>&base){Table t;for(size_t i=0;i<base.size();++i)for(size_t j=0;j<base.size();++j)t[c.key(c.add(base[i],base[j]))].push_back({i,j});return t;}
struct Solve4 {bool found=false;std::array<size_t,4>i{};};
Solve4 solve4(const Curve&c,const std::vector<P>&base,const Table&t,P target){for(size_t k=0;k<base.size();++k)for(size_t l=0;l<base.size();++l){P need=c.add(target,c.neg(c.add(base[k],base[l])));auto it=t.find(c.key(need));if(it!=t.end()){auto q=it->second.front();return {true,{q.i,q.j,k,l}};}}return {};}
Solve4 reference4(const Curve&c,const std::vector<P>&b,P q){for(size_t i=0;i<b.size();++i)for(size_t j=0;j<b.size();++j)for(size_t k=0;k<b.size();++k)for(size_t l=0;l<b.size();++l)if(c.add(c.add(c.add(b[i],b[j]),b[k]),b[l])==q)return {true,{i,j,k,l}};return {};}
U s3(const Curve&c,U x,U y,U z){auto &f=c.f;U xy=f.mul(x,y);return f.sub(f.sub(f.add(f.mul(f.mul(f.sub(x,y),f.sub(x,y)),f.mul(z,z)),f.mul(f.sub(xy,c.a),f.sub(xy,c.a))),f.mul(2,f.mul(f.add(f.mul(f.add(x,y),f.add(xy,c.a)),f.mul(2,c.b)),z))),f.mul(4,f.mul(c.b,f.add(x,y))));}
std::string check4(const Curve&c,const std::vector<P>&base,Solve4 s,P target,U B){std::array<P,4>p;for(size_t j=0;j<4;++j){p[j]=base[s.i[j]];if(!c.on(p[j])||p[j].x>=B)throw std::runtime_error("base constraint failure");}P u=c.add(p[0],p[1]),v=c.add(p[2],p[3]);if(c.add(u,v)!=target)throw std::runtime_error("certificate sum failure");
 if(u.inf){if(s3(c,p[2].x,p[3].x,target.x))throw std::runtime_error("left infinity S3 mismatch");return "left_cancel";}
 if(v.inf){if(s3(c,p[0].x,p[1].x,target.x))throw std::runtime_error("right infinity S3 mismatch");return "right_cancel";}
 if(s3(c,p[0].x,p[1].x,u.x)||s3(c,p[2].x,p[3].x,v.x)||s3(c,u.x,v.x,target.x))throw std::runtime_error("chained S3 mismatch");
 return "finite";
}
void system(std::ostream&o,const Fixture&z,U B,U curve){o<<"{\"curve_id\":"<<curve<<",\"field\":"<<z.c.f.p<<",\"a\":"<<z.c.a<<",\"b\":"<<z.c.b<<",\"B\":"<<B<<",\"rational_lift_constraints\":\"yi^2=xi^3+a*xi+b for i=1..4 over the recorded Fp\",\"variables\":[\"y1\",\"y2\",\"y3\",\"y4\",\"x1\",\"x2\",\"x3\",\"x4\",\"u\",\"v\"],\"target_parameter\":\"xR\",\"S3\":\"(x-y)^2*z^2-2*((x+y)*(x*y+a)+2*b)*z+(x*y-a)^2-4*b*(x+y)\",\"membership\":\"prod(t=0..B-1,xi-t)=0 for i=1..4\",\"finite_branch\":[\"S3(x1,x2,u)=0\",\"S3(x3,x4,v)=0\",\"S3(u,v,xR)=0\"],\"exceptional_branches\":[[\"x1=x2; choose opposite point signs\",\"S3(x3,x4,xR)=0\"],[\"x3=x4; choose opposite point signs\",\"S3(x1,x2,xR)=0\"]],\"certification\":\"Every hit requires actual Fp rational point lifts and exact four-point sum; S3 roots alone are not accepted\"}\n";}
void tests(){
 if(Curve(17,true,14,2).nonsingular())throw std::runtime_error("packed prime-subfield discriminant regression");
 for(U p:{5ULL,7ULL,11ULL})for(bool ext:{false,true}){F f(p,ext);for(U a=1;a<f.q;++a)if(f.mul(a,f.inv(a))!=1)throw std::runtime_error("field inverse");Curve c(p,ext,1,1);if(!c.nonsingular())continue;auto pts=c.points();pts.push_back({});for(P a:pts){if(!c.on(a)||!c.add(a,c.neg(a)).inf)throw std::runtime_error("group inverse");for(P b:pts)if(!c.on(c.add(a,b)))throw std::runtime_error("group closure");}size_t lim=std::min<size_t>(pts.size(),15);for(size_t i=0;i<lim;++i)for(size_t j=0;j<lim;++j)for(size_t k=0;k<lim;++k)if(c.add(c.add(pts[i],pts[j]),pts[k])!=c.add(pts[i],c.add(pts[j],pts[k])))throw std::runtime_error("associativity");}
 Curve c(101,false,2,3);auto pts=c.points();std::vector<P>b(pts.begin(),pts.begin()+6);auto t=pairtable(c,b);for(P q:pts){auto a=solve4(c,b,t,q),z=reference4(c,b,q);if(a.found!=z.found)throw std::runtime_error("MITM reference mismatch");if(a.found)check4(c,b,a,q,101);}
 auto z=fixture(509,false,123);P q=z.c.mul(17%z.r,z.g);auto r=rho(z.c,z.g,q,z.r,500);if(!r.ok||r.k!=17%z.r)throw std::runtime_error("rho positive control");std::cout<<"PASS field inverses, group closure/inverse/associativity, all-target MITM/reference, chained S3, rho positive control\n";
}
void run(std::filesystem::path dir){std::filesystem::create_directories(dir);std::ofstream out(dir/"results.jsonl"),cert(dir/"certificates.jsonl"),sys(dir/"systems.jsonl"),sum(dir/"summary.csv");sum<<"item,p,seed,curve,r,B,base_points,targets,hits,reference_agreement,pair_setup_ga,online_ga,reference_ga,rho_success,rho_setup_ga,rho_online_ga\n";U curveid=0,total=0;
 std::ofstream draws(dir/"candidate-draws.jsonl");candidate_log=&draws;
 for(U p:{17ULL,29ULL,43ULL,61ULL})for(U seed=1;seed<=6;++seed){++curveid;auto z=fixture(p,true,seed);auto &c=z.c;Rng rng{20261018+seed+p};U k=1+rng.range(z.r-1);P q=c.mul(k,z.g);c.f.reset();std::set<P>bp;U torsion_checks=0;
  for(U x=0;x<p;++x){auto it=c.roots.find(c.rhs(x));if(it==c.roots.end())continue;for(U y:it->second){P a{x,y,false};P u=c.mul(z.h,a),v=c.mul(z.h,c.add(a,z.t));if(u!=v)throw std::runtime_error("even cofactor failed to kill torsion");++torsion_checks;if(!u.inf)bp.insert(u);}}
  std::vector<P>base(bp.begin(),bp.end());Cost bc=c.f.cost;c.f.reset();auto tab=pairtable(c,base);Cost tc=c.f.cost;c.f.reset();bool found=false;Pair ij{};auto it=tab.find(c.key(q));if(it!=tab.end()){found=true;ij=it->second.front();}Cost online=c.f.cost;c.f.reset();if(found&&c.add(base[ij.i],base[ij.j])!=q)throw std::runtime_error("Fp2 oracle certificate");Cost vc=c.f.cost;auto rh=rho(c,z.g,q,z.r,seed*2017+p);
  out<<"{\"rho_peak_table_entries\":"<<rh.table_entries<<",\"item\":1,\"p\":"<<p<<",\"seed\":"<<seed<<",\"curve_id\":"<<curveid<<",\"nonresidue\":"<<c.f.d<<",\"a\":"<<c.a<<",\"b\":"<<c.b<<",\"order\":"<<z.n<<",\"r\":"<<z.r<<",\"cofactor\":"<<z.h<<",\"generator\":"<<pj(z.g)<<",\"target\":"<<pj(q)<<",\"known_answer_k\":"<<k<<",\"torsion\":"<<pj(z.t)<<",\"torsion_projection_checks\":"<<torsion_checks<<",\"base_points\":"<<base.size()<<",\"two_sum_found\":"<<found<<",\"curve_draws\":"<<z.draws<<",\"construction\":"<<cj(z.construction)<<",\"base_construction\":"<<cj(bc)<<",\"pair_table_setup\":"<<cj(tc)<<",\"table_entries\":"<<tab.size()<<",\"table_pairs\":"<<base.size()*base.size()<<",\"online\":"<<cj(online)<<",\"verification\":"<<cj(vc)<<",\"rho_ok\":"<<rh.ok<<",\"rho_restarts\":"<<rh.restarts<<",\"rho_steps\":"<<rh.steps<<",\"rho_setup\":"<<cj(rh.setup)<<",\"rho_online\":"<<cj(rh.online)<<",\"rho_verification\":"<<cj(rh.verification)<<"}\n";
  if(found)cert<<"{\"item\":1,\"curve_id\":"<<curveid<<",\"target\":"<<pj(q)<<",\"points\":["<<pj(base[ij.i])<<","<<pj(base[ij.j])<<"],\"verified\":true}\n";
  sum<<"1,"<<p<<','<<seed<<','<<curveid<<','<<z.r<<",0,"<<base.size()<<",1,"<<found<<",1,"<<tc.ga<<','<<online.ga<<",0,"<<rh.ok<<','<<rh.setup.ga<<','<<rh.online.ga<<'\n';++total;
 }
 std::cout<<"Review-driven v2 replication completed "<<total<<" Fp2 fixture records; original prime-field arms remain unchanged.\n";
 return;
 for(U p:{251ULL,509ULL,1021ULL,2039ULL,4093ULL})for(U seed=1;seed<=3;++seed){++curveid;auto z=fixture(p,false,seed);auto&c=z.c;U B=1;while(B*B*B*B*B<p)++B;c.f.reset();std::vector<P>base;for(U x=0;x<B;++x){auto it=c.roots.find(c.rhs(x));if(it!=c.roots.end())for(U y:it->second)base.push_back({x,y,false});}Cost bc=c.f.cost;c.f.reset();auto tab=pairtable(c,base);Cost tc=c.f.cost;system(sys,z,B,curveid);Rng rng{20261018+p+seed};U hits=0,og=0,rg=0,agree=0;U rho_ok=0,rho_setup=0,rho_online=0;
  for(U targetid=0;targetid<33;++targetid){bool planted=targetid==32;P q;U k=0;if(planted){if(base.empty()){out<<"{\"item\":14,\"curve_id\":"<<curveid<<",\"target_id\":32,\"status\":\"empty_base_no_planted_control\"}\n";continue;}for(int j=0;j<4;++j)q=c.add(q,base[rng.range(base.size())]);if(q.inf){q=c.mul(4,base[0]);if(q.inf)throw std::runtime_error("planted infinity");}}else{k=1+rng.range(z.r-1);q=c.mul(k,z.g);}
   c.f.reset();auto a=solve4(c,base,tab,q);Cost ac=c.f.cost;c.f.reset();auto b=reference4(c,base,q);Cost rc=c.f.cost;bool eq=a.found==b.found;if(!eq)throw std::runtime_error("independent oracle mismatch");++agree;if(a.found&&!planted)++hits;og+=ac.ga;rg+=rc.ga;c.f.reset();std::string branch="none";if(a.found)branch=check4(c,base,a,q,B);Cost vc=c.f.cost;if(planted&&!a.found)throw std::runtime_error("positive control missed");
   if(a.found){cert<<"{\"item\":14,\"curve_id\":"<<curveid<<",\"target_id\":"<<targetid<<",\"target\":"<<pj(q)<<",\"branch\":\""<<branch<<"\",\"points\":[";for(int j=0;j<4;++j){if(j)cert<<',';cert<<pj(base[a.i[j]]);}cert<<"],\"verified\":true}\n";}
   Rho rh;if(!planted){rh=rho(c,z.g,q,z.r,seed*100000+p+targetid);rho_ok+=rh.ok;rho_setup+=rh.setup.ga;rho_online+=rh.online.ga;}
   out<<"{\"rho_peak_table_entries\":"<<rh.table_entries<<",\"item\":14,\"p\":"<<p<<",\"seed\":"<<seed<<",\"curve_id\":"<<curveid<<",\"a\":"<<c.a<<",\"b\":"<<c.b<<",\"order\":"<<z.n<<",\"r\":"<<z.r<<",\"generator\":"<<pj(z.g)<<",\"target\":"<<pj(q)<<",\"known_answer_k\":"<<(planted?"null":std::to_string(k))<<",\"target_id\":"<<targetid<<",\"planted\":"<<planted<<",\"arity\":4,\"B\":"<<B<<",\"root_delta\":"<<std::log(double(B))/std::log(double(p))<<",\"base_points\":"<<base.size()<<",\"found\":"<<a.found<<",\"reference_agreement\":"<<eq<<",\"curve_draws\":"<<z.draws<<",\"construction\":"<<cj(z.construction)<<",\"base_construction\":"<<cj(bc)<<",\"pair_table_setup\":"<<cj(tc)<<",\"pair_table_entries\":"<<tab.size()<<",\"pair_table_pairs\":"<<base.size()*base.size()<<",\"online\":"<<cj(ac)<<",\"exhaustive_reference\":"<<cj(rc)<<",\"verification\":"<<cj(vc)<<",\"rho_ok\":"<<rh.ok<<",\"rho_setup\":"<<cj(rh.setup)<<",\"rho_online\":"<<cj(rh.online)<<"}\n";++total;
  }
  sum<<"14,"<<p<<','<<seed<<','<<curveid<<','<<z.r<<','<<B<<','<<base.size()<<",32,"<<hits<<','<<agree<<','<<tc.ga<<','<<og<<','<<rg<<','<<rho_ok<<','<<rho_setup<<','<<rho_online<<'\n';
 }
 std::cout<<"Completed "<<total<<" target records; all emitted certificates and independent-oracle comparisons passed.\n";
}
int main(int argc,char**argv){try{if(argc==2&&std::string(argv[1])=="--test"){tests();return 0;}if(argc!=2)throw std::runtime_error("usage: diagnostic --test|OUTPUT_DIRECTORY");run(argv[1]);return 0;}catch(std::exception&e){std::cerr<<"FAIL: "<<e.what()<<'\n';return 1;}}
