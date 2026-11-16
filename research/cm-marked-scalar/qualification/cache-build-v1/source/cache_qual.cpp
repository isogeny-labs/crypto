#include "cache_core.hpp"
#include <gmpxx.h>
#include <set>
namespace independent {
using Int=mpz_class;
static std::vector<int> recode(Int scalar,unsigned width){bool sign=scalar<0;if(sign)scalar=-scalar;std::vector<int> digits;Int original=scalar,sum=0,power=1;while(scalar!=0){int d=0;if(mpz_odd_p(scalar.get_mpz_t())){Int small=scalar%(1u<<width);d=int(small.get_si());if(d>=(1<<(width-1)))d-=1<<width;scalar-=d;}scalar/=2;sum+=d*power;power*=2;digits.push_back(sign?-d:d);}require(sum==original,"GMP signed expansion");return digits;}
static cmcache::Metrics predict(const cmcache::Split&s,const cmcache::Plan&p,bool reuse){
 cmcache::Metrics out;std::set<int> bank={1};
 // The chain program is public input; arithmetic/dependencies and counts are
 // reconstructed here, independently of point-bank operations and digit code.
 auto operation=[&](int a,int b,bool sub){require(bank.count(a)&&bank.count(b),"GMP prefix dependency");int result=sub?a-b:a+b;require(result>0,"GMP positive coefficient");if(!bank.insert(result).second)return;if(a==b&&!sub)++out.calls.doubles;else{++out.calls.adds;if(sub)++out.calls.negations;}};
 for(const auto&op:cmcache::prefix(p.radix))operation(op.left,op.right,op.subtract);
 Int k(s.k.str()),c(s.c.str());bool sign=k<0;if(sign){k=-k;c=-c;}auto kd=recode(k,p.width),cd=recode(c,2);size_t table=kd.empty()?0:(1u<<(p.width-2));
 if(table>1){operation(int(p.radix),int(p.radix),false);for(size_t j=1;j<table;++j)operation(int((2*j-1)*p.radix),int(2*p.radix),false);out.affine_entries=table;}
 out.bank_points=bank.size();out.calls.table_points=uint64_t(p.radix>1)+table+uint64_t(!cd.empty());bool started=false;
 auto term=[&](int n){require(n!=0&&bank.count(std::abs(n)),"GMP cached term");out.calls.negations+=n<0;if(started)++out.calls.adds;else started=true;};
 for(size_t pos=std::max(kd.size(),cd.size());pos>0;--pos){if(started)++out.calls.doubles;int q=pos<=kd.size()?kd[pos-1]:0,r=pos<=cd.size()?cd[pos-1]:0;if(reuse&&q&&r){int sum=q*int(p.radix)+r;if(!sum){++out.cancellations;continue;}if(bank.count(std::abs(sum))){++out.cache_hits;term(sum);continue;}}if(q)term(q*int(p.radix));if(r)term(r);}
 if(sign&&started)++out.calls.negations;
 return out;
}
static bool equal(const cmcache::Metrics&a,const cmcache::Metrics&b){return a.calls.doubles==b.calls.doubles&&a.calls.adds==b.calls.adds&&a.calls.negations==b.calls.negations&&a.calls.table_points==b.calls.table_points&&a.bank_points==b.bank_points&&a.cache_hits==b.cache_hits&&a.cancellations==b.cancellations&&a.affine_entries==b.affine_entries;}
}
int main(int argc,char**argv){try{require(argc==2,"cache-qual FIXTURES");unsigned checks=0,audits=0,positive=0,negative=0;
 for(auto&f:fixtures(argv[1])){Runtime r(f,true);verify_relation(r);
  for(long input:{0L,1L,2L,17L,-1L}){Big k(input),residue;require(BN_nnmod(residue.p,k.p,r.n.p,r.field.ctx),"cache qualifier point scalar");auto P=point(r.group),expected=point(r.group),out=point(r.group);require(EC_POINT_mul(r.group,P.get(),residue.p,nullptr,nullptr,r.field.ctx)&&EC_POINT_mul(r.group,expected.get(),nullptr,P.get(),r.mu.p,r.field.ctx),"cache qualifier reference");
   auto audit=[&](int coefficient,const EC_POINT*value){auto check=point(r.group);Big scalar(coefficient);require(EC_POINT_mul(r.group,check.get(),nullptr,P.get(),scalar.p,r.field.ctx)&&EC_POINT_cmp(r.group,check.get(),value,r.field.ctx)==0,"independent bank point label");++audits;};
   for(unsigned radix:{1u,2u,4u,8u,16u,32u,unsigned(f.marked.at("degree").integer())})for(int offset:{-64,0,64})for(unsigned width=2;width<=7;++width)for(bool reuse:{false,true}){cmcache::Plan plan{radix,width,offset,0};auto split=cmcache::decompose(r,plan);cmcache::Metrics actual;cmcache::multiply(r,out.get(),P.get(),split,plan,reuse,actual,audit);require(EC_POINT_cmp(r.group,out.get(),expected.get(),r.field.ctx)==0,"cached fixed-mu point action");require(independent::equal(actual,input?independent::predict(split,plan,reuse):cmcache::Metrics{}),"independent cached schedule");++checks;}
   if(input==1){cmcache::Plan plan{13,4,0,0};cmcache::Split split{Big(1),Big(-1)};cmcache::Metrics hit;cmcache::multiply(r,out.get(),P.get(),split,plan,true,hit,audit);Big twelve(12);require(EC_POINT_mul(r.group,expected.get(),nullptr,P.get(),twelve.p,r.field.ctx)&&EC_POINT_cmp(r.group,out.get(),expected.get(),r.field.ctx)==0&&hit.cache_hits==1,"cache-hit positive control");++positive;split.c=Big();cmcache::Metrics bad;cmcache::multiply(r,out.get(),P.get(),split,plan,true,bad);require(EC_POINT_cmp(r.group,out.get(),expected.get(),r.field.ctx)!=0,"cache wrong residual negative control");++negative;}
  }
 }
 std::cout<<"{\"fixed_mu_point_and_GMP_schedule_checks\":"<<checks<<",\"independent_bank_point_checks\":"<<audits<<",\"cache_hit_positive_controls\":"<<positive<<",\"wrong_residual_controls\":"<<negative<<",\"timing_performed\":false}\n";return 0;
}catch(const std::exception&e){std::cerr<<"cached qualification failed: "<<e.what()<<'\n';return 1;}}
