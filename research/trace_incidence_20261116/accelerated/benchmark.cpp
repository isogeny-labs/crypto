// Complete public-target cost. Check mode does not read a clock or getrusage.
#include <bits/stdc++.h>
#include <nlohmann/json.hpp>
#include <sys/resource.h>
#include "accelerated.hpp"
namespace frozen_pairs {
bool control_check_only=false;
#include "../runpod/pair_control_callable.cpp"
}
using namespace paper_ec;
namespace acc=paper_ec::accelerated;
using Json=nlohmann::json;
namespace {
bool check_only=false;
struct Reading {double cpu=0,wall=0;};
Reading reading() {
    if(check_only)return {};
    struct rusage u{};if(getrusage(RUSAGE_SELF,&u))throw std::runtime_error("getrusage");
    return {double(u.ru_utime.tv_sec+u.ru_stime.tv_sec)+1e-6*double(u.ru_utime.tv_usec+u.ru_stime.tv_usec),std::chrono::duration<double>(std::chrono::steady_clock::now().time_since_epoch()).count()};
}
Json elapsed(Reading a,Reading b){return {{"cpu_seconds",b.cpu-a.cpu},{"wall_seconds",b.wall-a.wall}};}
void need(bool b,const char* s){if(!b)throw std::runtime_error(s);}
Json frozen(const LabelledPools& p) {
    std::ostringstream text;text<<p.target.p<<' '<<p.target.a<<' '<<p.target.b<<' '<<p.subgroup_order<<' '<<p.generator.x<<' '<<p.generator.y<<' '<<p.public_target.x<<' '<<p.public_target.y;
    for(const auto& pool:p.pools)text<<' '<<pool.size();
    text<<'\n';
    for(const auto& pool:p.pools)for(const auto& l:pool)text<<l.point.x<<' '<<l.point.y<<' '<<l.alpha<<' '<<l.beta<<'\n';
    std::istringstream in(text.str());std::ostringstream out;
    auto* oldin=std::cin.rdbuf(in.rdbuf());auto* oldout=std::cout.rdbuf(out.rdbuf());
    frozen_pairs::control_check_only=check_only;
    char name[]="frozen-pair-control";char* argv[]={name,nullptr};int code;
    try {code=frozen_pairs::pair_control_call(1,argv);}catch(...){std::cin.rdbuf(oldin);std::cout.rdbuf(oldout);std::cin.clear();throw;}
    std::cin.rdbuf(oldin);std::cout.rdbuf(oldout);std::cin.clear();need(code==0,"frozen pairs failed");return Json::parse(out.str());
}
}
int main(int argc,char** argv) {
    try {
        std::vector<std::string> args(argv+1,argv+argc);if(!args.empty()&&args.front()=="--check-only"){check_only=true;args.erase(args.begin());}
        need(args.size()==4,"usage: benchmark [--check-only] GENERATOR NORM PUBLIC_FIXTURE SEED");
        const auto began=reading();
        const auto gen=args[0]=="affine"?acc::Generator::LegacyAffine:args[0]=="straus"?acc::Generator::Straus:acc::Generator::BatchedWindow;
        need(args[0]=="affine"||args[0]=="straus"||args[0]=="window","unknown generator");
        const auto& arm=args[1];need(arm=="horner"||arm=="compose"||arm=="support"||arm=="support-bloom"||arm=="first"||arm=="first-bloom"||arm=="frozen-pairs","unknown norm");
        need(args[2].find(".validation.")==std::string::npos,"secret validation input rejected");
        std::ifstream input(args[2]);Json j;input>>j;need(j.at("schema")=="prime-pools/v1"&&j.at("kind")=="natural","only natural public fixtures");
        const auto& c=j.at("curve");LabelledPools public_input;
        public_input.target={c.at("p"),c.at("a"),c.at("b")};public_input.subgroup_order=c.at("r");
        public_input.generator={c.at("P")[0],c.at("P")[1],false};public_input.public_target={c.at("Q")[0],c.at("Q")[1],false};
        const auto B=j.at("B").get<std::size_t>();need(B<=128,"registered B ceiling");
        std::size_t used=0;const Word seed=std::stoull(args[3],&used);need(used==args[3].size(),"malformed seed");
        std::unique_ptr<acc::FastContext> ctx;
        if(gen!=acc::Generator::LegacyAffine||arm=="support"||arm=="support-bloom"||arm=="first"||arm=="first-bloom")ctx=std::make_unique<acc::FastContext>(public_input,gen==acc::Generator::BatchedWindow);
        const auto setup_done=reading();acc::GenerationCounts generation;
        const auto p=acc::generate(public_input,B,seed,gen,generation,ctx.get());const auto generated=reading();
        Stats stats;IncidenceAnswer answer;Json old;
        const bool full_support=arm!="first"&&arm!="first-bloom"&&arm!="frozen-pairs";
        if(arm=="frozen-pairs") {
            acc::validate(p);old=frozen(p);answer.informative_relation=old.at("status")=="relation";
            if(answer.informative_relation)answer.recovered_scalar=old.at("recovery").at("k");
        } else if(arm=="horner")answer=solve_incidence(p,RootBackend::KnownCoordinates,nullptr,&stats);
        else if(arm=="first"||arm=="first-bloom")answer=ctx->first_witness(p,arm=="first-bloom");
        else answer=acc::solve(p,arm=="compose"?acc::Norm::Composition:arm=="support"?acc::Norm::StreamedPairs:acc::Norm::StreamedPairsBloom,ctx.get(),&stats);
        if(answer.informative_relation)need(point_mul(p.target,answer.recovered_scalar,p.generator)==p.public_target,"original-curve scalar check failed");
        const auto finished=reading();
        Json row={{"schema","trace-runpod-row/v1"},{"extension_schema","trace-accelerated-row/v1"},{"pcc_standing","unverified"},{"experiment","incidence"},{"arm",args[0]+"/"+arm},{"generator",args[0]},{"norm",arm},{"fixture_id",j.at("id")},{"B",B},{"p",p.target.p},{"r",p.subgroup_order},{"seed",seed},{"timings_collected",!check_only},{"status",answer.informative_relation?"relation":"none"},{"full_support",full_support},{"generation",{{"draws",generation.draws},{"accepted",3*B},{"rejected_infinity",generation.rejected_infinity},{"rejected_coordinate",generation.rejected_coordinate},{"rng","mt19937_64/uniform_int_distribution"}}},{"phases",{{"public_setup_and_tables",elapsed(began,setup_done)},{"pool_generation",elapsed(setup_done,generated)},{"incidence_and_recovery",elapsed(generated,finished)}}},{"complete",elapsed(began,finished)},{"counters",stats.counts}};
        if(full_support){row["gcd_coefficients"]=answer.incidence_gcd;row["gcd_degree"]=answer.incidence_gcd.size()-1;}
        if(answer.informative_relation)row["recovered_scalar"]=answer.recovered_scalar;
        if(!old.is_null())row["frozen_pair_control"]=old;
        if(ctx){const auto s=ctx->counts();row["fast_counters"]={{"batch_inversions",s.batch_inversions},{"exceptional_doubling_inversions",s.exceptional_doubling_inversions},{"ordinary_additions",s.ordinary_additions},{"table_entries",s.table_entries},{"combinations",s.combinations},{"pair_coordinates",s.pair_coordinates},{"exact_lookups",s.exact_lookups},{"uninformative",s.uninformative}};}
        // Correctness comparisons are only in no-timer mode, outside the
        // measured complete interval. The paid worker runs the audit first.
        if(check_only){const auto reference=solve_incidence(p,RootBackend::KnownCoordinates,nullptr);
            need(reference.informative_relation==answer.informative_relation,"reference existence mismatch");
            if(full_support)need(reference.incidence_gcd==answer.incidence_gcd,"reference complete GCD mismatch");
            if(answer.informative_relation)need(reference.recovered_scalar==answer.recovered_scalar,"reference scalar mismatch");}
        else{struct rusage u{};need(getrusage(RUSAGE_SELF,&u)==0,"RSS read failed");row["peak_rss_kib"]=u.ru_maxrss;}
        std::cout<<row.dump()<<'\n';return 0;
    }catch(const std::exception& e){std::cerr<<e.what()<<'\n';return 1;}
}
