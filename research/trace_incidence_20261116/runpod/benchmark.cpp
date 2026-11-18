// Native Runpod driver. Production kernels and frozen fixtures are unchanged.
// Check mode deliberately makes no clock or getrusage calls.
#include <bits/stdc++.h>
#include <nlohmann/json.hpp>
#include <flint/nmod_poly.h>
#include <flint/nmod_poly_factor.h>
#include <flint/ulong_extras.h>
#include <sys/resource.h>
#include <sys/wait.h>
#include <unistd.h>
#include "../native/incidence.hpp"
#include "../native/isogeny.hpp"

namespace frozen_pairs {
bool control_check_only=false;
#include "pair_control_callable.cpp"
}
namespace paper_ec {
IncidenceAnswer solve_incidence_lazy(const LabelledPools&,RootBackend,
    const std::function<PreparedDatum()>&,Stats*);
}
namespace driver {
using namespace paper_ec;
using Json=nlohmann::json;
bool check_only=false;
struct Reading { double wall=0,cpu=0; };
Reading read_clock() {
    if(check_only)return {};
    struct rusage self{},child{};
    if(getrusage(RUSAGE_SELF,&self)||getrusage(RUSAGE_CHILDREN,&child))throw std::runtime_error("getrusage");
    auto seconds=[](timeval v){return double(v.tv_sec)+1e-6*double(v.tv_usec);};
    return {std::chrono::duration<double>(std::chrono::steady_clock::now().time_since_epoch()).count(),
        seconds(self.ru_utime)+seconds(self.ru_stime)+seconds(child.ru_utime)+seconds(child.ru_stime)};
}
Json elapsed(Reading start,Reading stop) {return {{"wall_seconds",stop.wall-start.wall},{"cpu_seconds",stop.cpu-start.cpu}};}
void need(bool okay,const std::string& text){if(!okay)throw std::runtime_error(text);}
Word mul(Word a,Word b,Word p){return Word((__uint128_t(a)*b)%p);}
Word power(Word a,Word n,Word p){Word r=1;while(n){if(n&1)r=mul(r,a,p);a=mul(a,a,p);n>>=1;}return r;}
Json load(const std::string& path){std::ifstream f(path);Json j;f>>j;return j;}
Datum parse_datum(std::istream& f){
    Datum d;need(bool(f>>d.curve.p>>d.curve.a>>d.curve.b>>d.beta>>d.order>>d.twist_order),"auxiliary construction missed or malformed");
    Word inv=power(d.beta,d.curve.p-2,d.curve.p),inv2=mul(inv,inv,d.curve.p);
    d.twist={d.curve.p,mul(d.curve.a,inv2,d.curve.p),mul(d.curve.b,mul(inv,inv2,d.curve.p),d.curve.p)};
    for(auto* factors:{&d.factors,&d.twist_factors}){
        unsigned n=0;need(bool(f>>n)&&n<64,"auxiliary factor count");
        for(unsigned i=0;i<n;++i){PrimePower q;need(bool(f>>q.prime>>q.exponent),"auxiliary factor record");factors->push_back(q);}
    }
    for(auto* b:{&d.basis,&d.twist_basis}){
        for(unsigned i=0;i<2;++i){unsigned infinity=0;Point pt;Word order=0;
            need(bool(f>>infinity>>pt.x>>pt.y>>order)&&infinity<2,"auxiliary basis record");pt.infinity=bool(infinity);
            if(!i){b->first=pt;b->first_order=order;}else{b->second=pt;b->second_order=order;}
        }
    }return d;
}
Datum construct(Word p,Word seed,const std::string& script,Json& record,const Curve* fixed=nullptr){
    int channel[2];need(pipe(channel)==0,"pipe");
    const pid_t child=fork();need(child>=0,"fork");
    if(!child){
        close(channel[0]);if(dup2(channel[1],STDOUT_FILENO)<0)_exit(126);close(channel[1]);
        auto variable=[](const char* key,Word v){const auto s=std::to_string(v);if(setenv(key,s.c_str(),1))_exit(126);};
        variable("TRACE_AUX_P",p);variable("TRACE_AUX_SEED",seed);variable("TRACE_AUX_FIXED",fixed?1:0);
        if(fixed){variable("TRACE_AUX_A",fixed->a);variable("TRACE_AUX_B",fixed->b);}
        execlp("timeout","timeout","-k","1","15","gp","-q","-f",script.c_str(),static_cast<char*>(nullptr));_exit(127);
    }
    close(channel[1]);std::string text;char buffer[4096];ssize_t n=0;
    while((n=read(channel[0],buffer,sizeof buffer))>0){text.append(buffer,std::size_t(n));if(text.size()>65536){kill(child,SIGKILL);break;}}
    close(channel[0]);int status=0;need(waitpid(child,&status,0)==child,"waitpid");
    record["exit_status"]=WIFEXITED(status)?WEXITSTATUS(status):128+WTERMSIG(status);
    record["native_output"]=text;
    need(WIFEXITED(status)&&WEXITSTATUS(status)==0,"bounded PARI datum construction failed");
    std::istringstream in(text);auto d=parse_datum(in);Word tries=0;need(bool(in>>tries),"missing construction attempts");
    record["curve_attempts"]=tries;record["order"]=d.order;record["twist_order"]=d.twist_order;
    record["basis_orders"]={d.basis.first_order,d.basis.second_order,d.twist_basis.first_order,d.twist_basis.second_order};
    return d;
}
std::vector<Word> flint_roots(const Coefficients& coefficients,Word p){
    nmod_poly_t f;nmod_poly_init(f,p);
    for(std::size_t i=0;i<coefficients.size();++i)nmod_poly_set_coeff_ui(f,i,coefficients[i]);
    nmod_poly_factor_t factors;nmod_poly_factor_init(factors);nmod_poly_roots(factors,f,1);
    std::vector<Word> roots;
    for(slong i=0;i<factors->num;++i){
        auto* g=factors->p+i;need(nmod_poly_degree(g)==1,"FLINT root output not linear");
        const Word a=nmod_poly_get_coeff_ui(g,0),b=nmod_poly_get_coeff_ui(g,1);
        roots.push_back(mul(a?p-a:0,n_invmod(b,p),p));
    }
    nmod_poly_factor_clear(factors);nmod_poly_clear(f);std::sort(roots.begin(),roots.end());return roots;
}
Word evaluate(const Coefficients& f,Word x,Word p){Word r=0;for(auto i=f.rbegin();i!=f.rend();++i)r=Word((__uint128_t(r)*x+*i)%p);return r;}
LabelledPools generate(const Json& source,Word seed,Json& record){
    need(source.at("schema")=="prime-pools/v1"&&source.at("kind")=="natural","only natural public fixtures allowed");
    LabelledPools result;auto c=source.at("curve");result.target={c.at("p"),c.at("a"),c.at("b")};
    result.subgroup_order=c.at("r");result.generator={c.at("P")[0],c.at("P")[1],false};result.public_target={c.at("Q")[0],c.at("Q")[1],false};
    const std::size_t B=source.at("B");need(B>0&&B<=128&&6*B<result.subgroup_order,"pool capacity");
    need(on_curve(result.target,result.generator)&&on_curve(result.target,result.public_target),"public point equation");
    need(point_mul(result.target,result.subgroup_order,result.generator).infinity&&point_mul(result.target,result.subgroup_order,result.public_target).infinity,"public subgroup");
    std::mt19937_64 random(seed);std::uniform_int_distribution<Word> draw(0,result.subgroup_order-1);std::set<Word> used;
    Word draws=0,rejected_infinity=0,rejected_x=0;
    for(auto& pool:result.pools)while(pool.size()<B){
        need(++draws<1000000,"public generation draw limit");const Word a=draw(random),b=draw(random);
        const auto pt=point_add(result.target,point_mul(result.target,a,result.generator),point_mul(result.target,b,result.public_target));
        if(pt.infinity){++rejected_infinity;continue;}if(!used.insert(pt.x).second){++rejected_x;continue;}
        pool.push_back({pt,a,b});
    }
    record={{"draws",draws},{"accepted",3*B},{"rejected_infinity",rejected_infinity},{"rejected_coordinate",rejected_x},{"rng","mt19937_64/uniform_int_distribution"}};
    return result;
}
std::string pair_text(const LabelledPools& p){
    std::ostringstream out;out<<p.target.p<<' '<<p.target.a<<' '<<p.target.b<<' '<<p.subgroup_order<<' '
        <<p.generator.x<<' '<<p.generator.y<<' '<<p.public_target.x<<' '<<p.public_target.y;
    for(const auto& pool:p.pools)out<<' '<<pool.size();
    out<<'\n';
    for(const auto& pool:p.pools)for(const auto& q:pool)out<<q.point.x<<' '<<q.point.y<<' '<<q.alpha<<' '<<q.beta<<'\n';
    return out.str();
}
Json pairs(const LabelledPools& p){
    frozen_pairs::control_check_only=check_only;
    std::istringstream input(pair_text(p));std::ostringstream output;
    auto* previous_in=std::cin.rdbuf(input.rdbuf());auto* previous_out=std::cout.rdbuf(output.rdbuf());
    char name[]="frozen-pair-control";char* argv[]={name,nullptr};const int status=frozen_pairs::pair_control_call(1,argv);
    std::cin.rdbuf(previous_in);std::cout.rdbuf(previous_out);std::cin.clear();need(status==0,"pair control failed");return Json::parse(output.str());
}
Json incidence(const std::string& arm,const std::string& path,Word seed,const std::string& script){
    Json row={{"schema","trace-runpod-row/v1"},{"pcc_standing","unverified"},{"experiment","incidence"},{"arm",arm},{"seed",seed},{"timings_collected",!check_only}};
    const auto began=read_clock();const auto public_fixture=load(path);row["fixture_id"]=public_fixture.at("id");row["B"]=public_fixture.at("B");
    const auto p=generate(public_fixture,seed,row["generation"]);const auto generated=read_clock();
    row["p"]=p.target.p;row["r"]=p.subgroup_order;row["r_over_B_cubed"]=double(p.subgroup_order)/std::pow(double(p.pools[0].size()),3);
    double aux_cpu=0,aux_wall=0;
    try{
        if(arm=="pairs"){
            const auto result=pairs(p);row["status"]=result.at("status");row["pair_control"]=result;
            if(!result.at("recovery").is_null()){
                const Word k=result.at("recovery").at("k");need(point_mul(p.target,k,p.generator)==p.public_target,"pair public scalar check");row["recovered_scalar"]=k;
            }
        }else{
            const auto backend=(arm=="known"||arm=="known-aa")?RootBackend::KnownCoordinates:arm=="dense"?RootBackend::PaperDense:arm=="trace"?RootBackend::PaperTrace:RootBackend::Pullback;
            need(arm=="known"||arm=="known-aa"||arm=="dense"||arm=="trace"||arm=="pullback"||arm=="pullback-isogeny","unknown incidence arm");
            const auto factory=[&](){
                const auto start=read_clock();
                try {
                auto datum=construct(p.target.p,seed,script,row["auxiliary"]);
                PreparedDatum checked(datum);
                if(arm=="pullback-isogeny"){
                    Point kernel;
                    for(const auto& item:std::array<std::pair<Point,Word>,2>{{{datum.basis.first,datum.basis.first_order},{datum.basis.second,datum.basis.second_order}}})
                        if(!(item.second&1)){kernel=point_mul(datum.curve,item.second/2,item.first);break;}
                    need(!kernel.infinity,"auxiliary has no rational two-isogeny kernel");
                    const auto map=two_isogeny(datum.curve,kernel);datum=construct(p.target.p,seed,script,row["codomain_auxiliary"],&map.codomain);checked=PreparedDatum(datum);
                }
                const auto finish=read_clock();aux_wall+=finish.wall-start.wall;aux_cpu+=finish.cpu-start.cpu;return checked;
                } catch (...) {
                    const auto finish=read_clock();aux_wall+=finish.wall-start.wall;aux_cpu+=finish.cpu-start.cpu;throw;
                }
            };
            const auto answer=solve_incidence_lazy(p,backend,factory,nullptr);
            row["status"]=answer.informative_relation?"relation":answer.coordinate_relation?"uninformative":"none";
            row["gcd_degree"]=answer.incidence_gcd.size()-1;row["coordinate_relation"]=answer.coordinate_relation;
            row["informative_relation"]=answer.informative_relation;
            if(answer.informative_relation){need(point_mul(p.target,answer.recovered_scalar,p.generator)==p.public_target,"public scalar check");row["recovered_scalar"]=answer.recovered_scalar;}
            if(check_only){
                const auto reference=solve_incidence(p,RootBackend::KnownCoordinates,nullptr);
                need(reference.incidence_gcd==answer.incidence_gcd&&reference.informative_relation==answer.informative_relation,"lazy/original output mismatch");
                const auto pair_reference=pairs(p);need((pair_reference.at("status")=="relation")==answer.informative_relation,"independent frozen pair existence mismatch");
                std::vector<Word> expected;for(const auto& q:p.pools[2])if(!evaluate(answer.incidence_gcd,q.point.x,p.target.p))expected.push_back(q.point.x);
                std::sort(expected.begin(),expected.end());need(flint_roots(answer.incidence_gcd,p.target.p)==expected,"FLINT selected-root support mismatch");
            }
        }
    }catch(const std::exception& e){row["status"]="failed";row["error"]=e.what();}
    const auto finish=read_clock();
    if(!check_only){row["complete"]=elapsed(began,finish);row["generation_and_public_input"]=elapsed(began,generated);row["solve_including_lazy_auxiliary"]=elapsed(generated,finish);row["auxiliary_nested"]={{"wall_seconds",aux_wall},{"cpu_seconds",aux_cpu}};}
    return row;
}
Json root(const std::string& arm,unsigned repetitions){
    need(arm=="dense"||arm=="trace"||arm=="pullback"||arm=="dense-isogeny"||arm=="trace-isogeny"||arm=="pullback-isogeny"||arm=="flint","unknown root arm");
    const Coefficients f{26,12,7,48,4,58,78,41,32,28,1};const std::vector<Word> expected{11,19,25,34,41,64,65,68,71,72};
    const auto start=read_clock();
    for(unsigned i=0;i<repetitions;++i){
        std::vector<Word> roots;
        if(arm=="flint")roots=flint_roots(f,83);
        else{
            auto d=paper_f83_datum();PreparedDatum checked(d);
            if(arm.find("-isogeny")!=std::string::npos){const auto map=two_isogeny(d.curve,d.basis.first);d=construct_toy_datum(map.codomain,d.beta);checked=PreparedDatum(d);}
            roots=arm.rfind("pullback",0)==0?root_find_pullback(checked,f):root_find(checked,f,arm.rfind("trace",0)==0?TranslationMethod::Trace:TranslationMethod::Dense);
        }
        need(roots==expected,"paper root output mismatch");
    }
    const auto finish=read_clock();Json row={{"schema","trace-runpod-row/v1"},{"pcc_standing","unverified"},{"experiment","paper-root"},{"arm",arm},{"repetitions",repetitions},{"status","passed"},{"roots",expected},{"timings_collected",!check_only},{"construction_scope","literal datum supplied; validation charged each call; isogeny arms additionally construct and validate the complete toy codomain datum"}};
    if(!check_only)row["batch"]=elapsed(start,finish);
    return row;
}
Json translation(const std::string& arm,const std::string& path){
    need(arm=="trace"||arm=="dense","unknown translation arm");
    const auto start=read_clock();const auto f=load(path);
    const Curve curve{f.at("p"),f.at("a"),f.at("b")};Divisor input;
    input.components.push_back({f.at("u").get<Coefficients>(),f.at("v").get<Coefficients>()});input.infinity=f.at("input_infinity");
    Point q;if(!f.at("Q").is_null())q={f.at("Q")[0],f.at("Q")[1],false};
    Word beta=2;while(power(beta,(curve.p-1)/2,curve.p)!=curve.p-1)++beta;
    const auto output=translate(curve,input,q,arm=="trace"?TranslationMethod::Trace:TranslationMethod::Dense,beta);
    const auto finish=read_clock();
    std::size_t degree=output.infinity?1:0;for(const auto& component:output.components)degree+=component.u.size()-1;
    need(degree==f.at("expected_points").size(),"translated support cardinality");
    for(const auto& point:f.at("expected_points")){
        if(point.is_null()){need(output.infinity,"missing translated infinity");continue;}
        const Word x=point[0],y=point[1];std::size_t matches=0;
        for(const auto& component:output.components)if(!evaluate(component.u,x,curve.p)&&evaluate(component.v,x,curve.p)==y)++matches;
        need(matches==1,"translated collision/exception support mismatch");
    }
    Json row={{"schema","trace-runpod-row/v1"},{"pcc_standing","unverified"},{"experiment","translation"},{"fixture_id",f.at("id")},{"arm",arm},{"degree",f.at("degree")},{"case",f.at("kind")},{"status","passed"},{"timings_collected",!check_only},{"support_checked",degree}};
    if(!check_only){row["complete"]=elapsed(start,finish);row["independent_support_check"]=elapsed(finish,read_clock());}
    return row;
}
}
int main(int argc,char** argv){
    using namespace driver;
    try{
        int i=1;if(i<argc&&std::string(argv[i])=="--check-only"){check_only=true;++i;}
        need(i<argc,"root ARM REPEATS | incidence ARM FIXTURE SEED AUXILIARY_GP");const std::string kind=argv[i++];Json row;
        if(kind=="root"){need(argc-i==2,"root arguments");const std::string arm=argv[i++];const unsigned repetitions=std::stoul(argv[i++]);need(repetitions>0&&repetitions<=1000,"repetition limit");row=root(arm,repetitions);}
        else if(kind=="translation"){need(argc-i==2,"translation arguments");const std::string arm=argv[i++],path=argv[i++];row=translation(arm,path);}
        else if(kind=="datum"){
            need(check_only&&(argc-i==3||argc-i==5),"datum is a check-only entry point");
            const Word p=std::stoull(argv[i++]),seed=std::stoull(argv[i++]);const std::string script=argv[i++];
            Curve fixed{p,0,0};const bool use_fixed=argc-i==2;if(use_fixed){fixed.a=std::stoull(argv[i++]);fixed.b=std::stoull(argv[i++]);}
            Json record;const auto d=construct(p,seed,script,record,use_fixed?&fixed:nullptr);const PreparedDatum checked(d);
            row={{"schema","trace-runpod-datum-check/v1"},{"status","passed"},{"timings_collected",false},{"construction",record}};
        }
        else{need(kind=="incidence"&&argc-i==4,"incidence arguments");const std::string arm=argv[i++],path=argv[i++];const Word seed=std::stoull(argv[i++]);const std::string script=argv[i++];row=incidence(arm,path,seed,script);}
        if(!check_only){struct rusage self{},children{};getrusage(RUSAGE_SELF,&self);getrusage(RUSAGE_CHILDREN,&children);row["peak_rss_kib"]=self.ru_maxrss;row["maximum_child_rss_kib"]=children.ru_maxrss;row["memory_scope"]="fresh driver process; parent high-water RSS and maximum child RSS, not their measured concurrent sum";}
        std::cout<<row.dump()<<'\n';return row.at("status")=="failed"?3:0;
    }catch(const std::exception& e){std::cout<<Json({{"schema","trace-runpod-row/v1"},{"status","failed"},{"error",e.what()},{"timings_collected",!check_only}}).dump()<<'\n';return 2;}
}
