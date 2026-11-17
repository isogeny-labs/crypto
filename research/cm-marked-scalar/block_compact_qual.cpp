#include "block_compact.hpp"
#define main cm_original_block_qualification_main
#include "block_qual.cpp"
#undef main
namespace compactcheck {
static cmblock::Program expanded(const cmcompact::Program&p){
 cmblock::Program out;out.doubles=p.doubles;out.adds=p.adds;out.negations=p.negations;out.transitions=p.transitions;out.table=p.table;out.outer_sign=p.outer_sign;out.has_residual=p.has_residual;
 for(const auto&step:p.steps){cmblock::Step s;s.bits=step.bits;for(int term:step.terms)s.terms.push_back(term);out.steps.push_back(std::move(s));}return out;
}
static bool same(const cmblock::Program&a,const cmblock::Program&b){
 if(std::tie(a.doubles,a.adds,a.negations,a.transitions,a.table,a.outer_sign,a.has_residual)!=std::tie(b.doubles,b.adds,b.negations,b.transitions,b.table,b.outer_sign,b.has_residual)||a.steps.size()!=b.steps.size())return false;
 for(size_t i=0;i<a.steps.size();++i)if(a.steps[i].bits!=b.steps[i].bits||a.steps[i].terms!=b.steps[i].terms)return false;
 return true;
}
static void check(const cmcache::Split&s,const cmcache::Plan&p){auto old=cmblock::prepare(s,p);auto compact=expanded(cmcompact::prepare(s,p));require(same(old,compact),"compact exact old program/ties/metadata");blockcheck::program(s,p,compact);}
static void run(const std::string&directory){
 uint64_t integers=0,small=0,points=0,labels=0,rejections=0,boundary=0;
 for(unsigned radix:{1u,2u,4u,7u,8u,11u,13u,16u,19u,32u})for(unsigned width=2;width<=7;++width)
  for(long k:{-8L,-1L,0L,1L,2L,8L})for(long c:{-8L,-1L,0L,1L,8L}){
   cmcache::Plan p{radix,width,0,0};cmcache::Split s{Big(k),Big(c)};check(s,p);++integers;
   auto in=blockcheck::inputs(s,p);auto count=blockcheck::program(s,p,expanded(cmcompact::prepare(s,p)));
   require(blockcheck::key(count)==blockcheck::key(blockcheck::exhaustive(in,p,std::max(in.q.size(),in.c.size()),false)),"compact exhaustive GMP minimum");++small;
  }
 for(auto&f:fixtures(directory)){
  Runtime r(f,true);verify_relation(r);
  for(long input:{0L,1L,2L,17L,-1L}){
   Big scalar(input),residue;require(BN_nnmod(residue.p,scalar.p,r.n.p,r.field.ctx),"compact input residue");
   auto P=point(r.group),actual=point(r.group),expected=point(r.group);
   require(EC_POINT_mul(r.group,P.get(),residue.p,nullptr,nullptr,r.field.ctx)&&EC_POINT_mul(r.group,expected.get(),nullptr,P.get(),r.mu.p,r.field.ctx),"compact point reference");
   auto audit=[&](int coefficient,const EC_POINT*value){Big n(coefficient);auto test=point(r.group);require(EC_POINT_mul(r.group,test.get(),nullptr,P.get(),n.p,r.field.ctx)&&EC_POINT_cmp(r.group,test.get(),value,r.field.ctx)==0,"compact independently labelled bank point");++labels;};
   for(unsigned radix:{1u,2u,4u,8u,16u,32u,unsigned(f.marked.at("degree").integer())})
    for(unsigned width=2;width<=7;++width)for(int offset:{-64,0,64}){
     cmcache::Plan p{radix,width,offset,0};auto s=cmcache::decompose(r,p);check(s,p);++integers;
     auto program=cmcompact::prepare(s,p);cmblock::Metrics m;unsigned callbacks=0;
     cmcompact::execute_prepared(r,actual.get(),P.get(),s,p,program,m,audit,[&](){++callbacks;});
     require(EC_POINT_cmp(r.group,actual.get(),expected.get(),r.field.ctx)==0,"compact original mu point action");
     if(input)require(blockcheck::full_counts(s,p,expanded(program),m)&&callbacks==1,"compact full counters and setup boundary");
     else require(m.bank.calls.doubles==0&&m.bank.calls.adds==0&&m.bank.bank_points==0&&callbacks==0,"compact identity skips setup");
     ++points;
    }
  }
 }
 // Explicit public bounds, and both signs at the largest accepted scalar.
 for(bool minus:{false,true}){Big k;require(BN_set_bit(k.p,2047),"compact boundary scalar");if(minus)BN_set_negative(k.p,1);cmcache::Split s{k,Big(0L)};check(s,{32,7,0,0});++boundary;}
 for(auto p:std::vector<cmcache::Plan>{{0,4,0,0},{33,4,0,0},{3,4,0,0},{1,1,0,0},{1,8,0,0}}){
  bool old=false,next=false;cmcache::Split s{Big(17L),Big(-1L)};try{cmblock::prepare(s,p);}catch(const std::exception&){old=true;}try{cmcompact::prepare(s,p);}catch(const std::exception&){next=true;}require(old&&next,"compact same invalid plan rejection");++rejections;
 }
 {Big k;require(BN_set_bit(k.p,2048),"compact oversized scalar");cmcache::Split s{k,Big(0L)};bool old=false,next=false;try{cmblock::prepare(s,{1,2,0,0});}catch(const std::exception&){old=true;}try{cmcompact::prepare(s,{1,2,0,0});}catch(const std::exception&){next=true;}require(old&&next,"compact oversized rejection");++rejections;}
 {cmcompact::Terms terms;terms.push_back(1);terms.push_back(-1);bool bad=false;try{terms.push_back(2);}catch(const std::exception&){bad=true;}require(bad,"compact inline term overflow rejection");++rejections;}
 std::cout<<"{\"exact_program_and_GMP_checks\":"<<integers<<",\"independent_small_exhaustive_cases\":"<<small<<",\"original_mu_point_and_full_count_checks\":"<<points<<",\"independent_bank_labels\":"<<labels<<",\"largest_scalar_boundary_cases\":"<<boundary<<",\"domain_and_inline_overflow_rejections\":"<<rejections<<",\"timing_performed\":false,\"allocation_measurement_performed\":false}\n";
}
}
int main(int argc,char**argv){try{require(argc==2,"block-compact-qual FIXTURES");compactcheck::run(argv[1]);return 0;}catch(const std::exception&e){std::cerr<<"compact qualification failed: "<<e.what()<<'\n';return 1;}}
