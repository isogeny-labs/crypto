#define main retained_audit_main
#include "../accelerated/audit.cpp"
#undef main
#include "canonical.hpp"
#include <random>
int main(int argc,char**argv){try{
 need(argc==2,"test-canonical STUDY");std::uint64_t products=0,invalid=0;std::mt19937_64 random(20261123);
 for(const Word prime:std::vector<Word>{5,7,97,4099,16777259,2147483647,4294967291ULL}){
  canonical_pairs::Field<true> f(prime);for(unsigned i=0;i<20000;++i){const Word a=i<3?prime-1-i:random()%prime,b=i<3?prime-1:random()%prime;need(f.mul(a,b)==Word(__uint128_t(a)*b%prime),"Barrett product mismatch");++products;}
 }
 canonical_pairs::Field<false> wide(2305843009213693951ULL);for(unsigned i=0;i<20000;++i){const Word a=random()%wide.p,b=random()%wide.p;need(wide.mul(a,b)==Word(__uint128_t(a)*b%wide.p),"nmod wide product mismatch");++products;}
 std::ifstream input(std::string(argv[1])+"/runpod/nonlinear-control.json");Json j;input>>j;auto base=public_input(j);canonical_pairs::Selector selector(base,true);acc::GenerationCounts generation;auto p=acc::generate(base,4,20261123,acc::Generator::BatchedWindow,generation,&selector.context());
 const auto oracle=selector.context().first_witness(p,true);for(bool barrett:{false,true}){const auto a=selector.search(p,barrett);need(a.informative_relation==oracle.informative_relation,"canonical witness status");if(a.informative_relation)need(a.recovered_scalar==oracle.recovered_scalar,"canonical scalar");}
 auto reject=[&](const LabelledPools&bad){rejected([&]{selector.search(bad,true);});++invalid;};
 auto bad=p;bad.pools[0].clear();reject(bad);bad=p;bad.pools[1][0]=bad.pools[0][0];reject(bad);bad=p;bad.pools[0][0].alpha=base.subgroup_order;reject(bad);bad=p;bad.pools[0][0].beta=base.subgroup_order;reject(bad);bad=p;bad.pools[0][0].point.infinity=true;reject(bad);bad=p;bad.pools[0][0].point.y=base.target.p;reject(bad);bad=p;bad.public_target=point_neg(base.target,base.public_target);reject(bad);bad=p;bad.subgroup_order+=2;reject(bad);bad=p;bad.target.a=(bad.target.a+1)%bad.target.p;reject(bad);
 auto uninformative=base;for(std::size_t pool=0;pool<3;++pool)for(Word i=0;i<4;++i){Word a=1+4*pool+i;uninformative.pools[pool].push_back({point_mul(base.target,a,base.generator),a,0});}
 const auto empty=selector.search(uninformative,true);need(empty.coordinate_relation&&!empty.informative_relation,"uninformative witness contract");bad=uninformative;bad.pools[0][0].alpha=2;reject(bad);
 std::cout<<Json({{"field_products",products},{"malformed_cases",invalid},{"status","passed"},{"standing","unsigned independent-formula producer checks"}}).dump()<<'\n';
}catch(const std::exception&e){std::cerr<<e.what()<<'\n';return 1;}}
