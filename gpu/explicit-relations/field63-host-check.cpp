#include "host.hpp"
#include <cassert>
#include <iostream>
using namespace explicit_relations;
using W=unsigned __int128;
Point refadd(Point a,Point b,U p,U ca) {
 if (a.inf) return b;
 if (b.inf) return a;
 if(a.x==b.x&&(a.y+b.y)%p==0)return {};
 U den=a.x==b.x?2*a.y%p:(b.x+p-a.x)%p;
 U num=a.x==b.x?U((3*W(modmul(a.x,a.x,p))+ca)%p):(b.y+p-a.y)%p;
 U slope=modmul(num,modpow(den,p-2,p),p);
 U x=U((W(modmul(slope,slope,p))+2*W(p)-a.x-b.x)%p);
 return {x,(modmul(slope,(a.x+p-x)%p,p)+p-a.y)%p,false};
}
int main(){
 U seed=0x123456789abcdef;
 auto next=[&](){seed^=seed<<13;seed^=seed>>7;seed^=seed<<17;return seed;};
 for(U p:{(U(1)<<62)-57,(U(1)<<63)-25}){
  while(!prime(p))p-=2;
  auto f=fp64_ctx_new(p);
  for(unsigned i=0;i<20000;++i){U a=next()%p,b=next()%p;U x=fp64_to_mont(f,a),y=fp64_to_mont(f,b);
   assert(fp64_from_mont(f,fp64_mul(f,x,y))==modmul(a,b,p));
   assert(fp64_from_mont(f,fp64_add(f,x,y))==U((W(a)+b)%p));
   assert(fp64_from_mont(f,fp64_sub(f,x,y))==U((W(a)+p-b)%p));
  }
  Curve c=curve(p,p-3,7);Point q;U x=0;while(!c.lift(x++,0,q)){};Point a;
  for(unsigned i=0;i<512;++i){Point got=c.affine(c.mixed(c.jac(a),q));Point want=refadd(a,q,p,p-3);assert(equal(got,want));assert(c.on_curve(got));a=got;}
 }
 for(U m:{(U(1)<<63)+29,UINT64_MAX-58,UINT64_MAX}){
  for(U a:{U(0),U(1),m/2,m-2,m-1})for(U b:{U(0),U(1),m/2,m-2,m-1})assert(hash_mul64(a,b,m)==modmul(a,b,m));
  for(unsigned i=0;i<20000;++i){U a=next()%m,b=next()%m;assert(hash_mul64(a,b,m)==modmul(a,b,m));}
 }
 U hp=6917529027641081803ULL;while(!prime(hp))hp-=2;
 assert(hp==6917529027641081737ULL);
 Curve c=curve(hp,0,7);HostBackend host(c,10000,257);U s=UINT64_MAX-58;
 for(U alpha:{U(1),U(17),s-1})for(U beta:{U(0),s/2,s-1}){
  hash_parameters(c,s,alpha,beta,257,10000);
  for(auto q:host.hash(s,alpha,beta,257)){U z=2*q.x+(q.y&1);assert(U((W(modmul(alpha,z,s))+beta)%s)<257);assert(c.on_curve(q));}
 }
 std::cout<<"PASS field63 Montgomery, full-u64 hash, affine and inventory host fixtures\n";
}
