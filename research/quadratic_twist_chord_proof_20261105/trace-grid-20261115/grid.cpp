#define main preceding_fixture_main
#include "../trace-fixture-20261115/fixture.cpp"
#undef main
#include <algorithm>
#include <set>
#include <sstream>
int code(P p){return p.o?-1:((p.x.a*49+p.x.b)*49+p.y.a)*49+p.y.b;}
std::string list(const std::set<int>&v){if(v.empty())return "-";std::ostringstream o;bool first=true;for(int k:v){if(!first)o<<',';first=false;o<<k;}return o.str();}
int main(){try{
 AA=1;BB=8;delta=8;auto source=points(AA,BB);auto twist=points(18,14);
 if(source.size()!=47||twist.size()!=53)throw std::runtime_error("domain");
 std::vector<int> betas{1,7,8,9,10,11,12,13};
 // Each direction is normalized to real=1 if imaginary=0, else imaginary=1.
 for(int z=1;z<49;z++){int norm=z/7?mul(z,inv(z/7)):1;if(std::find(betas.begin(),betas.end(),norm)==betas.end())throw std::runtime_error("direction coverage");}
 std::cout<<"TG1 7 3 8 1 8 47 53 56 2632\n";
 int gid=0;for(int gamma=0;gamma<49;gamma+=7)for(int beta:betas){
  unsigned id=0;for(P q:source){std::set<int> lifts;
   // Axis branch: rational half of nonzero Q, and the complete twist image over O.
   if(!q.o){P p=scalar(q,24);if(sub(p.x.a,gamma)/7==0)lifts.insert(code(p));}
   else for(P u:twist){P p=embed(u);if(p.o||sub(p.x.a,gamma)/7==0)lifts.insert(code(p));}
   if(!q.o)for(int b=1;b<7;b++){
    F z=scale(beta,b),d=mul(delta,mul(z,z)),s=q.x.a,t=q.y.a;
    for(F l=0;l<49;l++){
     F l2=mul(l,l),g=sub(sub(sub(sub(mul(l2,l2),scale(mul(s,l2),6)),scale(mul(s,s),3)),scale(mul(t,l),8)),scale(add(AA,d),4));
     if(g)continue;
     F a0=mul(sub(l2,s),inv(2));if(sub(a0,gamma)/7)continue;
     F c=sub(mul(l,sub(a0,s)),t);P p{{a0,z},{c,mul(z,l)},false};
     if(!on(p)||!(sum(p,frob(p))==q))throw std::runtime_error("chart reconstruction");
     lifts.insert(code(p));
    }
   }
   std::cout<<gid<<' '<<gamma<<' '<<beta<<' '<<id++<<' '<<(q.o?-1:q.x.a)<<' '<<(q.o?-1:q.y.a)<<' '<<lifts.size()<<' '<<list(lifts)<<'\n';
  }gid++;
 }
 }catch(const std::exception&e){std::cerr<<e.what()<<'\n';return 101;}}
