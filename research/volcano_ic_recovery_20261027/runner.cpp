#include "bounded.hpp"
#include <cerrno>
#include <filesystem>
#include <fstream>
#include <iostream>
#include <set>
#include <sys/random.h>
#include <sys/resource.h>
#include <sys/stat.h>
#include <time.h>
using namespace vic;
static U integer(const json& v) {
    require(v.is_string(),"integers must be decimal strings");auto s=v.get<std::string>();
    require(!s.empty()&&std::all_of(s.begin(),s.end(),[](char c){return c>='0'&&c<='9';}),"invalid decimal integer");
    std::size_t used=0;U n=std::stoull(s,&used);require(used==s.size()&&std::to_string(n)==s,"noncanonical integer");return n;
}
static void keys(const json& j,const std::set<std::string>& allowed) {
    require(j.is_object(),"expected object");for(auto i=j.begin();i!=j.end();++i)require(allowed.count(i.key())!=0,"unrecognized field (witness inputs are forbidden)");
    for(const auto& k:allowed)require(j.contains(k),"missing required field");
}
static Point read_point(const json& p) {
    if(p.is_null())return {};
    require(p.is_array()&&p.size()==2,"point encoding");return {integer(p[0]),integer(p[1]),false};
}
static json point_wire(Point p) {return p.infinity?json(nullptr):json::array({std::to_string(p.x),std::to_string(p.y)});}
static json load(const std::string& p){std::ifstream f(p);require(bool(f),"input open failed");json j;f>>j;return j;}
static U wall() {return U(std::chrono::duration_cast<std::chrono::nanoseconds>(std::chrono::steady_clock::now().time_since_epoch()).count());}
static U cpu() {timespec t{};require(clock_gettime(CLOCK_PROCESS_CPUTIME_ID,&t)==0,"CPU clock failed");return U(t.tv_sec)*1000000000+U(t.tv_nsec);}
static long rss() {rusage u{};require(getrusage(RUSAGE_SELF,&u)==0,"rusage failed");return u.ru_maxrss;}
class Journal {
    std::ofstream out;
public:
    explicit Journal(const std::string& p):out(p) {require(bool(out),"journal open failed");}
    void write(const json& v){out<<v.dump()<<'\n';out.flush();require(bool(out),"journal write failed");}
};
class Random {
    Journal& log;U index=0;
public:
    explicit Random(Journal& j):log(j){}
    U below(U bound,const char* scope) {
        require(bound>0,"zero random bound");U threshold=U(-bound)%bound;
        for(;;){U value=0;std::size_t at=0;while(at<sizeof(value)){ssize_t n=getrandom(reinterpret_cast<char*>(&value)+at,sizeof(value)-at,0);if(n<0&&errno==EINTR)continue;require(n>0,"getrandom failed");at+=std::size_t(n);}
            bool accept=value>=threshold;log.write({{"kind","random_word"},{"index",index++},{"scope",scope},{"bound",std::to_string(bound)},{"threshold",std::to_string(threshold)},{"word",std::to_string(value)},{"accepted",accept}});
            if(accept)return value%bound;
        }
    }
};
class Phase {
    Curve& e;RowSpace& matrix;Journal& journal;json& phases;std::string name;U start=wall(),process=cpu();bool done=false;
public:
    Phase(Curve& curve,RowSpace& rows,Journal& j,json& p,std::string n):e(curve),matrix(rows),journal(j),phases(p),name(std::move(n)) {
    }
    void finish(bool completed=true) {
        if(done)return;
        done=true;json row={{"kind","phase"},{"name",name},{"completed",completed},
          {"wall_ns",wall()-start},{"process_cpu_ns",cpu()-process},{"cumulative_process_peak_rss_KiB",rss()},
          {"Fp",e.f.count.wire()},{"scalar_field",matrix.take_counts().wire()}};
        journal.write(row);phases.push_back(row);e.f.count={};
    }
    ~Phase(){if(!done){try{finish(false);}catch(...){}}}
};
int main(int argc,char** argv) {
    // A plan contains counts/bytes only; it does not allocate the production table.
    try {
        require(argc==5||argc==7,"usage: runner plan INPUT CONFIG --explicit | runner run INPUT CONFIG NEW_PRIVATE_DIRECTORY --private-output --explicit");
        std::string mode=argv[1];require(mode=="plan"||mode=="run","unknown mode");
        require((mode=="plan"&&argc==5)||(mode=="run"&&argc==7),"argument count");
        require(std::string(argv[argc-1])=="--explicit","require explicit mode marker");
        json input=load(argv[2]),config=load(argv[3]);
        keys(input,{"schema","instance_id","p","a","b","r","P","Q","factor_base"});
        require(input.at("schema")=="volcano.ic.public-instance/v1","input schema");
        keys(config,{"schema","radius","memory_limit_bytes","max_setup_trials","deadline_seconds"});
        require(config.at("schema")=="volcano.ic.bounded-config/v1","config schema");
        U rad=integer(config.at("radius")),memory=integer(config.at("memory_limit_bytes")),max_trials=integer(config.at("max_setup_trials")),seconds=integer(config.at("deadline_seconds"));
        require(rad<=31&&seconds>0&&seconds<=3600&&max_trials>0&&max_trials<=100000,"configuration bounds");
        auto sizes=layout(unsigned(rad));require(sizes.total_bytes<=memory,"table exceeds memory budget");
        require(memory<=(U(24)<<30),"memory budget exceeds frozen 24 GiB maximum");
        U p=integer(input.at("p")),r=integer(input.at("r"));
        auto shape=json{{"schema","volcano.ic.bounded-plan/v1"},{"instance_id",input.at("instance_id")},{"radix",sizes.radix},{"side_entries",sizes.side_entries},
          {"slots",sizes.slots},{"payload_bytes",sizes.total_bytes},{"memory_excludes", "allocator overhead, batch buffers, input and journals; reserve headroom"},
          {"coefficient_vector_count",std::to_string(sizes.side_entries)+" squared"},{"unique_support_or_success_probability_measured",false},
          {"subgroup_order",std::to_string(r)},{"method","relation/linear-algebra pipeline with generic bounded-coefficient MITM PDP"},{"algebraic_PDP_solver",false}};
        require(input.at("factor_base").is_array()&&input.at("factor_base").size()==12,"12 base points required");
        Point P=read_point(input.at("P")),Q=read_point(input.at("Q"));std::array<Point,12> base{};
        for(int i=0;i<12;++i)base[i]=read_point(input.at("factor_base")[i]);
        if(mode=="plan"){std::cout<<shape.dump(2)<<'\n';return 0;}
        require(std::string(argv[5])=="--private-output","private output marker required");
        umask(0077);std::string dir=argv[4];require(std::filesystem::create_directory(dir),"output directory must not exist");
        Journal log(dir+"/records.private.jsonl"),random_log(dir+"/randomness.private.jsonl");Random rng(random_log);
        U epoch=wall();Curve e(p,integer(input.at("a")),integer(input.at("b")));RowSpace matrix(r);json phases=json::array();U attempts=0,found=0;bool recovered=false;std::string status="not_started";
        auto deadline=[&](U progress){require((wall()-epoch)/1000000000<seconds,"original deadline exceeded");if(progress%1048576==0)log.write({{"kind","progress"},{"entries",progress},{"wall_since_epoch_ns",wall()-epoch},{"setup_trials",attempts},{"setup_rank",matrix.rank()}});};
        try {
            {Phase phase(e,matrix,log,phases,"input_validation");
                require(!P.infinity&&!Q.infinity&&e.on(P)&&e.on(Q)&&e.scalar(r,P).infinity&&e.scalar(r,Q).infinity,"invalid P/Q subgroup");
                std::set<U> xs;for(Point b:base)require(!b.infinity&&e.on(b)&&e.scalar(r,b).infinity&&xs.insert(b.x).second,"invalid or repeated sign orbit in base");phase.finish();}
            log.write({{"kind","identity"},{"input",input},{"config",config},{"plan",shape},{"no_factor_base_logs_or_target_witness_inputs",true}});
            Decomposer d(e,base,unsigned(rad),memory,deadline);
            {Phase phase(e,matrix,log,phases,"reusable_table_construction");d.build();phase.finish();}
            while(attempts<max_trials&&matrix.rank()<12) {
                deadline(1);++attempts;U t;Point R;
                {Phase phase(e,matrix,log,phases,"setup_probe_generation");t=rng.below(r-1,"setup_scalar")+1;R=e.scalar(t,P);phase.finish();}
                Match m;
                {Phase phase(e,matrix,log,phases,"setup_relation_search");m=d.find(R,[&](U n){return rng.below(n,"setup_decomposition_choice");});phase.finish();}
                bool increased=false;
                if(m.found) {
                    ++found;{Phase phase(e,matrix,log,phases,"setup_relation_check");require(evaluate(e,base,m.coefficient)==R,"invalid setup relation");phase.finish();}
                    {Phase phase(e,matrix,log,phases,"setup_linear_algebra");increased=matrix.insert(m.coefficient,t);phase.finish();}
                }
                log.write({{"kind","setup"},{"trial",attempts},{"t",std::to_string(t)},{"R",point_wire(R)},{"found",m.found},{"coefficients",m.coefficient},{"right_examined",m.right_examined},{"rank",matrix.rank()},{"rank_increased",increased}});
            }
            if(matrix.rank()!=12)status="insufficient_setup_rank";
            else {
                {Phase phase(e,matrix,log,phases,"factor_base_log_verification");auto values=matrix.logs();for(int i=0;i<12;++i)require(e.scalar(values[i],P)==base[i],"factor-base log equation failed");log.write({{"kind","factor_base_logs_private"},{"logs",values}});phase.finish();}
                Match m;{Phase phase(e,matrix,log,phases,"target_decomposition");m=d.find(Q,[&](U n){return rng.below(n,"target_decomposition_choice");});phase.finish();}
                status="target_not_decomposed";
                if(m.found) {U k;{Phase phase(e,matrix,log,phases,"target_recovery_and_check");require(evaluate(e,base,m.coefficient)==Q,"invalid target decomposition");k=matrix.recover(m.coefficient);require(e.scalar(k,P)==Q,"target equation failed");phase.finish();}
                    log.write({{"kind","target_recovery_private"},{"coefficients",m.coefficient},{"k",std::to_string(k)},{"right_examined",m.right_examined},{"public_equation_checked",true}});recovered=true;status="recovered_requires_independent_replay";}
            }
        } catch(const std::exception& ex) {status="failed_or_censored";log.write({{"kind","failure"},{"message",ex.what()},{"setup_trials",attempts},{"rank",matrix.rank()}});}
        json summary={{"schema","volcano.ic.bounded-summary/v1"},{"instance_id",input.at("instance_id")},{"status",status},{"setup_trials",attempts},{"relations_found",found},{"rank",matrix.rank()},
            {"native_target_equation_passed",recovered},{"independent_replay_passed",nullptr},{"independent_replay_status","not_run_by_collector"},{"original_full_graph_requirement_complete",false},{"contains_scalar_witnesses",false},{"contains_raw_randomness",false},
            {"phases",phases},{"epoch_wall_ns",wall()-epoch},{"cumulative_process_peak_rss_KiB",rss()},{"timing_class","non-isolated diagnostic; not a speedup comparison"},
            {"Montgomery_counters",false},{"counter_definition","actual standard-residue calls; multiply and square are disjoint; inverse overlaps its executed multiply/square components"},
            {"method",shape.at("method")},{"algebraic_PDP_solver",false}};
        std::ofstream summary_file(dir+"/summary.json");summary_file<<summary.dump(2)<<'\n';require(bool(summary_file),"summary write failed");std::cout<<summary.dump()<<'\n';return recovered?0:2;
    } catch(const std::exception& ex){std::cerr<<"BOUNDED_IC_ERROR "<<ex.what()<<'\n';return 1;}
}
