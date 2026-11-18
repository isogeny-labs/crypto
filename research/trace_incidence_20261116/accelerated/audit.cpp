#include "accelerated.hpp"
#include <nlohmann/json.hpp>
#include <filesystem>
#include <fstream>
#include <iostream>
#include <stdexcept>

using namespace paper_ec;
namespace acc=paper_ec::accelerated;
using Json=nlohmann::json;
namespace {
void need(bool b,const char* message){if(!b)throw std::runtime_error(message);}
Word neg(Word a,Word p){return a?p-a:0;}
Word mul(Word a,Word b,Word p){return Word(__uint128_t(a)*b%p);}
Word add(Word a,Word b,Word p){return Word((__uint128_t(a)+b)%p);}
Word sub(Word a,Word b,Word p){return add(a,neg(b,p),p);}
LabelledPools public_input(const Json& j) {
    const auto& c=j.at("curve");LabelledPools p;
    p.target={c.at("p"),c.at("a"),c.at("b")};p.subgroup_order=c.at("r");
    p.generator={c.at("P")[0],c.at("P")[1],false};p.public_target={c.at("Q")[0],c.at("Q")[1],false};return p;
}
bool equal(const LabelledPools& a,const LabelledPools& b) {
    for(std::size_t j=0;j<3;++j){if(a.pools[j].size()!=b.pools[j].size())return false;
        for(std::size_t i=0;i<a.pools[j].size();++i){const auto &x=a.pools[j][i],&y=b.pools[j][i];if(x.point!=y.point||x.alpha!=y.alpha||x.beta!=y.beta)return false;}}
    return true;
}
// Independent cubic summation-polynomial enumeration and schoolbook root
// product: no FLINT composition, quotient arithmetic or pair group additions.
Coefficients cubic_support(const LabelledPools& p) {
    const auto c=p.target;const auto prime=c.p;Coefficients out{1};
    for(const auto& t:p.pools[2]) {
        bool hit=false;
        for(const auto& u:p.pools[0])for(const auto& v:p.pools[1]) {
            const auto x=u.point.x,y=v.point.x,z=t.point.x;
            const auto xy=mul(x,y,prime),sum=add(x,y,prime),diff=sub(x,y,prime);
            const auto b=mul(2,add(mul(sum,add(xy,c.a,prime),prime),mul(2,c.b,prime),prime),prime);
            const auto q=sub(xy,c.a,prime);
            const auto constant=sub(mul(q,q,prime),mul(mul(4,c.b,prime),sum,prime),prime);
            const auto value=add(sub(mul(mul(diff,diff,prime),mul(z,z,prime),prime),mul(b,z,prime),prime),constant,prime);
            hit=hit||value==0;
        }
        if(hit){Coefficients next(out.size()+1);for(std::size_t i=0;i<out.size();++i){next[i]=sub(next[i],mul(t.point.x,out[i],prime),prime);next[i+1]=add(next[i+1],out[i],prime);}out=std::move(next);}
    }
    return out;
}
void check_witness(const LabelledPools& p,const IncidenceAnswer& a) {
    if(!a.informative_relation)return;
    Point sum;Word A=0,B=0;const auto r=p.subgroup_order;
    for(std::size_t j=0;j<3;++j){const auto& l=p.pools[j].at(a.positions[j]);need(a.signs[j]==1||a.signs[j]==-1,"invalid witness sign");
        sum=point_add(p.target,sum,a.signs[j]==1?l.point:point_neg(p.target,l.point));
        A=add(A,a.signs[j]==1?l.alpha:neg(l.alpha,r),r);B=add(B,a.signs[j]==1?l.beta:neg(l.beta,r),r);}
    need(sum.infinity&&B!=0&&add(A,mul(B,a.recovered_scalar,r),r)==0,"witness label/geometry mismatch");
    need(point_mul(p.target,a.recovered_scalar,p.generator)==p.public_target,"scalar recovery mismatch");
}
void exercise(const LabelledPools& p,bool independent,std::size_t& cases) {
    acc::FastContext context(p);Stats stats;
    const auto legacy=solve_incidence(p,RootBackend::KnownCoordinates,nullptr);
    const auto compose=acc::solve(p,acc::Norm::Composition,nullptr,&stats);
    const auto pairs=acc::solve(p,acc::Norm::StreamedPairs,&context);
    const auto bloom=acc::solve(p,acc::Norm::StreamedPairsBloom,&context);
    if(independent)need(legacy.incidence_gcd==cubic_support(p),"independent cubic full-support discrepancy");
    for(const auto& a:{compose,pairs,bloom}) {
        need(a.incidence_gcd==legacy.incidence_gcd,"entire incidence GCD mismatch");
        need(a.informative_relation==legacy.informative_relation&&a.coordinate_relation==legacy.coordinate_relation,"relation status discrepancy");
        if(a.informative_relation)need(a.recovered_scalar==legacy.recovered_scalar,"recovered scalar discrepancy");
        check_witness(p,a);
    }
    need(context.counts().pair_coordinates==4*p.pools[0].size()*p.pools[1].size(),"support traversal stopped early");
    const auto first=context.first_witness(p,true);check_witness(p,first);
    need(first.incidence_gcd.empty()&&first.informative_relation==legacy.informative_relation&&first.coordinate_relation==legacy.coordinate_relation,"first-witness contract mismatch");
    ++cases;
}
template<class F> void rejected(F f) {try {f();}catch(const std::invalid_argument&){return;}throw std::runtime_error("malformed input accepted");}
}
int main(int argc,char** argv) {
    try {
        need(argc==2,"usage: audit research-directory");const std::filesystem::path root=argv[1];
        std::vector<std::filesystem::path> paths;
        for(const auto& e:std::filesystem::directory_iterator(root/"historical/prime_fixtures")){
            const auto name=e.path().filename().string();
            if(name.find("_natural_s")!=std::string::npos&&name.find(".validation.")==std::string::npos&&e.path().extension()==".json")paths.push_back(e.path());
        }
        std::sort(paths.begin(),paths.end());need(paths.size()==60,"expected full 60-target panel");
        std::size_t cases=0,generated=0,rejects=0;
        for(const auto& path:paths) {
            std::ifstream input(path);Json fixture;input>>fixture;auto p=public_input(fixture);
            for(std::size_t j=0;j<3;++j)for(const auto& l:fixture.at("pools")[j])p.pools[j].push_back({{l.at("x"),l.at("y"),false},l.at("alpha"),l.at("beta")});
            exercise(p,true,cases);
            const auto B=fixture.at("B").get<std::size_t>();
            // Fixed public seeds cover rejections and misses without selecting
            // seeds for a favorable solve. Frozen points are not reused.
            for(Word seed:{Word(20261116),Word(20261116)+2654435761}) {
                acc::GenerationCounts a,b,c;acc::FastContext straus(p),batch(p,true);
                const auto baseline=acc::generate(p,B,seed,acc::Generator::LegacyAffine,a,nullptr);
                const auto single=acc::generate(p,B,seed,acc::Generator::Straus,b,&straus);
                const auto fast=acc::generate(p,B,seed,acc::Generator::BatchedWindow,c,&batch);
                need(equal(baseline,single)&&equal(single,fast),"generator changed exact labels/points/order");
                need(a.draws==b.draws&&b.draws==c.draws&&a.rejected_infinity==c.rejected_infinity&&a.rejected_coordinate==c.rejected_coordinate,"generation rejection accounting discrepancy");
                rejects+=a.rejected_infinity+a.rejected_coordinate;generated+=3*B;
                exercise(fast,B<=32,cases);
            }
        }
        std::ifstream control(root/"runpod/nonlinear-control.json");Json datum;control>>datum;auto toy=public_input(datum);
        // Many seeds in a small public subgroup force identity/collision
        // rejections and exercise different support multiplicities.
        for(Word seed=0;seed<24;++seed){acc::GenerationCounts a,b;acc::FastContext ctx(toy,true);
            const auto old=acc::generate(toy,4,seed,acc::Generator::LegacyAffine,a,nullptr);
            const auto fast=acc::generate(toy,4,seed,acc::Generator::BatchedWindow,b,&ctx);
            need(equal(old,fast)&&a.draws==b.draws,"small-group rejection order mismatch");rejects+=b.rejected_infinity+b.rejected_coordinate;exercise(fast,true,cases);
            // Unequal pool lengths include scalar and multi-block compose.
            for(const auto sizes:std::vector<std::array<std::size_t,3>>{{1,4,1},{4,4,2},{2,1,4},{3,3,3}}){auto uneven=fast;for(std::size_t j=0;j<3;++j)uneven.pools[j].resize(sizes[j]);exercise(uneven,true,cases);}
        }
        auto uninformative=toy;
        for(std::size_t j=0;j<3;++j)for(Word i=0;i<4;++i){Word a=1+4*j+i;uninformative.pools[j].push_back({point_mul(toy.target,a,toy.generator),a,0});}
        exercise(uninformative,true,cases);
        const auto noinfo=acc::solve(uninformative,acc::Norm::Composition,nullptr);
        need(noinfo.coordinate_relation&&!noinfo.informative_relation,"uninformative control failed");
        Word toy_log=0;while(point_mul(toy.target,toy_log,toy.generator)!=toy.public_target){need(++toy_log<toy.subgroup_order,"toy public log absent");}
        auto mixed=uninformative;auto& late=mixed.pools[2].back();late.alpha=sub(late.alpha,toy_log,toy.subgroup_order);late.beta=1;
        exercise(mixed,true,cases);acc::FastContext mixed_context(mixed);
        const auto late_answer=mixed_context.pairs(mixed,true);
        need(late_answer.informative_relation&&mixed_context.counts().uninformative>0,"uninformative witnesses hid later informative witness");
        auto bad=uninformative;bad.pools[1][0]=bad.pools[0][0];rejected([&]{acc::composition_gcd(bad);});
        bad=uninformative;bad.pools[0].clear();rejected([&]{acc::composition_gcd(bad);});
        bad=uninformative;bad.pools[0][0].point.y=bad.target.p;rejected([&]{acc::composition_gcd(bad);});
        bad=uninformative;bad.subgroup_order=73;rejected([&]{acc::FastContext c(bad);});
        bad=uninformative;bad.pools[0][0].alpha=bad.subgroup_order;rejected([&]{acc::composition_gcd(bad);});
        bad=uninformative;bad.pools[0][0].alpha=2;rejected([&]{acc::solve(bad,acc::Norm::Composition,nullptr);});
        rejected([&]{acc::FastContext c(bad);c.pairs(bad,true);});
        bad=uninformative;bad.public_target=point_neg(bad.target,bad.public_target);acc::FastContext wrong(bad);
        rejected([&]{wrong.pairs(uninformative,true);});
        need(rejects>0,"rejection cases missing");
        std::cout<<Json({{"schema","trace-accelerated-checks/v1"},{"standing","unverified"},{"timings_collected",false},{"status","passed"},{"full_support_cases",cases},{"exact_generator_points",generated},{"observed_rejected_draws",rejects},{"malformed_cases",8},{"controls",{"frozen native quotient/GCD","independent cubic polynomial enumeration","scalar affine public labels","Straus public labels","streamed pairs with Bloom off/on"}}}).dump(2)<<'\n';
        return 0;
    }catch(const std::exception& e){std::cerr<<e.what()<<'\n';return 1;}
}
