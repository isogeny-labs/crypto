// Shared norm/remainder algebraic oracle and cold fixed-fixture IC controls.
// Backend output is untrusted until every returned relation is independently added.
#define QUOTIENT_EXTRACT_NO_MAIN
#include "quotient_extract.cpp"
#define main legacy_direct_cli_main
#include "../prime_one_shot_20261029/continuation/solver/direct_ic.cpp"
#undef main
#include <chrono>
#include <csignal>
#include <fcntl.h>
#include <functional>
#include <sys/resource.h>
#include <sys/wait.h>
#include <unistd.h>

namespace algebraic {
using canonical::Polynomial;
using canonical::System;
using canonical::U;
using P=std::vector<Polynomial>;
using reference::Point;

struct Generator {
    struct DeferredMembership {};
    coeff::Config cfg;
    unsigned m,h;
    System system;
    std::vector<std::size_t> a,c,u,cs,ct,ds,dt;
    std::vector<std::vector<std::size_t>> q,r;
    std::size_t target_inverse=0;

    std::size_t name(const std::string&v) {
        const auto id=system.names.size();system.names.push_back(v);system.variables.emplace(v,id);return id;
    }
    std::vector<std::size_t> names(const std::string&prefix,unsigned count) {
        std::vector<std::size_t> out;
        for(unsigned i=0;i<count;++i)out.push_back(name(prefix+std::to_string(i)));
        return out;
    }
    Polynomial constant(U value)const{return Polynomial::constant(system.names.size(),system.p,value);}
    Polynomial variable(std::size_t id)const{return Polynomial::variable(system.names.size(),system.p,id);}
    P polynomial(const std::vector<std::size_t>&ids,bool monic=false)const {
        P out;for(auto id:ids)out.push_back(variable(id));if(monic)out.push_back(constant(1));return out;
    }
    P multiply(const P&a0,const P&b0)const {
        P out(a0.size()+b0.size()-1,constant(0));
        for(std::size_t i=0;i<a0.size();++i)for(std::size_t j=0;j<b0.size();++j)
            out[i+j]=out[i+j].plus(a0[i].times(b0[j]));
        return out;
    }
    P subtract(P a0,const P&b0)const {
        a0.resize(std::max(a0.size(),b0.size()),constant(0));
        for(std::size_t i=0;i<b0.size();++i)a0[i]=a0[i].plus(b0[i],true);
        return a0;
    }
    P plus(P a0,const P&b0)const {
        a0.resize(std::max(a0.size(),b0.size()),constant(0));
        for(std::size_t i=0;i<b0.size();++i)a0[i]=a0[i].plus(b0[i]);
        return a0;
    }
    Polynomial evaluate(const P&f,int x)const {
        auto out=constant(0);
        for(auto i=f.rbegin();i!=f.rend();++i)out=out.times(constant(x)).plus(*i);
        return out;
    }
    void equations(const P&f) {for(const auto&v:f)if(!v.terms.empty())system.equations.push_back(v);}

    Generator(coeff::Config input,unsigned arity,DeferredMembership):cfg(input),m(arity),h((arity+1)/2) {
        if(m<3||m>7||m%2==0||cfg.t<1||cfg.t>8||cfg.field.p<5||cfg.field.p>257)
            throw std::invalid_argument("bounded shared-norm shape");
        system.p=cfg.field.p;
    }
    Generator(coeff::Config input,unsigned arity,bool pruned=false)
        :Generator(input,arity,DeferredMembership{}) {
        if(pruned&&cfg.field.mod(cfg.c)==0)
            throw std::invalid_argument("witness-pruned membership requires nonzero c");
        system.p=cfg.field.p;
        a=names("a",h);c=names("c",h-1);u=names("u0_",m);
        for(int j=1;j<=cfg.t;++j) {
            q.push_back(names("q"+std::to_string(j)+"_",m-1));
            r.push_back(j<cfg.t?names("r"+std::to_string(j)+"_",m):std::vector<std::size_t>{});
        }
        if(!pruned) {
            cs=names("bc_s",h-1);ct=names("bc_t",m);
            ds=names("bd_s",m-1);dt=names("bd_t",m);
        }
        target_inverse=name("target_inverse");
        const auto A=polynomial(a,true),C=polynomial(c),U0=polynomial(u,true);
        const P curve{constant(cfg.b),constant(cfg.a),constant(0),constant(1)};
        equations(subtract(subtract(multiply(A,A),multiply(curve,multiply(C,C))),
                           multiply(P{constant(cfg.field.mod(-cfg.rx)),constant(1)},U0)));
        equations(P{evaluate(A,cfg.rx).plus(evaluate(C,cfg.rx).times(constant(cfg.ry)),true)});
        P previous{constant(0),constant(1)};
        for(int j=0;j<cfg.t;++j) {
            const auto next=j+1==cfg.t?P{constant(cfg.c)}:polynomial(r[j]);
            equations(subtract(subtract(multiply(previous,previous),multiply(polynomial(q[j]),U0)),next));
            previous=next;
        }
        if(!pruned) {
            equations(subtract(plus(multiply(U0,polynomial(cs)),multiply(C,polynomial(ct))),P{constant(1)}));
            P derivative;
            for(unsigned i=1;i<=m;++i)derivative.push_back(U0[i].times(constant(i)));
            equations(subtract(plus(multiply(U0,polynomial(ds)),multiply(derivative,polynomial(dt))),P{constant(1)}));
        }
        equations(P{evaluate(U0,cfg.rx).times(variable(target_inverse)).plus(constant(1),true)});
    }
};

std::vector<int> roots(const coeff::Poly&f,int p) {
    nmod_poly_t poly;nmod_poly_init(poly,p);
    for(std::size_t i=0;i<f.size();++i)nmod_poly_set_coeff_ui(poly,i,f[i]);
    nmod_poly_factor_t factors;nmod_poly_factor_init(factors);
    nmod_poly_roots(factors,poly,0);
    std::vector<int> out;
    for(slong i=0;i<factors->num;++i) {
        const auto&v=factors->p[i];
        if(v.length!=2)throw std::runtime_error("nonlinear root returned");
        out.push_back(static_cast<int>((p-v.coeffs[0])*quotient::power(v.coeffs[1],p-2,p)%p));
    }
    nmod_poly_factor_clear(factors);nmod_poly_clear(poly);
    std::sort(out.begin(),out.end());return out;
}
std::string decode(const Generator&g,const std::vector<U>&assignment) {
    const auto&f=g.cfg.field;
    auto coefficients=[&](const std::vector<std::size_t>&ids,bool monic=false) {
        coeff::Poly out;for(auto id:ids)out.push_back(static_cast<int>(assignment.at(id)));
        if(monic)out.push_back(1);
        coeff::trim(out);return out;
    };
    auto A=coefficients(g.a,true),C=coefficients(g.c),U0=coefficients(g.u,true);
    const auto norm=coeff::add(f,coeff::mul(f,A,A),coeff::mul(f,coeff::Poly{g.cfg.b,g.cfg.a,0,1},coeff::mul(f,C,C)),-1);
    const auto division=coeff::divrem(f,norm,coeff::Poly{f.mod(-g.cfg.rx),1});
    if(!coeff::zero(division.second)||division.first!=U0||coeff::evaluate(f,U0,g.cfg.rx)==0)
        throw std::runtime_error("norm/target chart certificate failed");
    auto xs=roots(U0,f.p);
    if(xs.size()!=g.m)throw std::runtime_error("support is not squarefree rational");
    reference::Curve E{f.p,g.cfg.a,g.cfg.b};Point sum;
    std::vector<std::pair<int,int>> points;
    for(int x:xs) {
        const int den=coeff::evaluate(f,C,x);
        if(den==0)throw std::runtime_error("zero norm denominator");
        const int y=f.mod(-coeff::evaluate(f,A,x)*f.inv(den));
        Point point{x,y,false};
        if(!E.contains(point)||x==g.cfg.rx||E.powmod(x,1U<<g.cfg.t)!=g.cfg.c)
            throw std::runtime_error("invalid recovered norm point");
        sum=E.sum(sum,point);points.emplace_back(x,y);
    }
    if(!reference::equal(sum,{g.cfg.rx,g.cfg.ry,false}))throw std::runtime_error("norm point sum failed");
    return coeff::key(points);
}

int backend(const std::filesystem::path&exe,const std::filesystem::path&input,
            const std::filesystem::path&output,const std::filesystem::path&directory) {
    const auto pid=fork();
    if(pid<0)throw std::runtime_error("backend fork failed");
    if(pid==0) {
        setpgid(0,0);
        rlimit memory{2ULL*1024*1024*1024,2ULL*1024*1024*1024};setrlimit(RLIMIT_AS,&memory);
        rlimit cpu{20,20};setrlimit(RLIMIT_CPU,&cpu);
        const int out=open((directory/"backend.stdout").c_str(),O_CREAT|O_EXCL|O_WRONLY,0600);
        const int err=open((directory/"backend.stderr").c_str(),O_CREAT|O_EXCL|O_WRONLY,0600);
        if(out<0||err<0)_exit(125);
        dup2(out,STDOUT_FILENO);dup2(err,STDERR_FILENO);close(out);close(err);
        execl(exe.c_str(),exe.c_str(),"-t","1","-g","2","-f",input.c_str(),"-o",output.c_str(),nullptr);
        _exit(127);
    }
    setpgid(pid,pid);
    int status=0;
    const auto start=std::chrono::steady_clock::now();
    while(waitpid(pid,&status,WNOHANG)==0) {
        if(std::chrono::steady_clock::now()-start>std::chrono::seconds(20)) {
            kill(-pid,SIGKILL);waitpid(pid,&status,0);return 124;
        }
        usleep(10000);
    }
    if(!WIFEXITED(status))return 128+(WIFSIGNALED(status)?WTERMSIG(status):0);
    return WEXITSTATUS(status);
}
struct OracleResult {std::set<std::string> models;quotient::Result extraction;};
struct Oracle {
    std::filesystem::path exe,root;
    std::size_t calls=0;
    bool pruned=false;
    Oracle(std::filesystem::path binary,std::filesystem::path directory,bool prune_witnesses=false)
        :exe(std::filesystem::absolute(binary)),root(std::filesystem::absolute(directory)),pruned(prune_witnesses) {
        if(!std::filesystem::is_regular_file(exe))throw std::invalid_argument("missing backend binary");
        if(!std::filesystem::create_directory(root))throw std::invalid_argument("refusing reused output directory");
    }
    OracleResult solve(coeff::Config cfg,unsigned m=3) {
        const auto directory=root/("call-"+std::to_string(++calls));
        if(!std::filesystem::create_directory(directory))throw std::runtime_error("reused backend call path");
        Generator gen(cfg,m,pruned);
        const auto in=directory/"input.ms",gb=directory/"basis.gb";
        canonical::write_system(in,gen.system,false);
        const int exit=backend(exe,in,gb,directory);
        {std::ofstream receipt(directory/"exit.txt");receipt<<exit<<'\n';}
        if(exit!=0)throw std::runtime_error("backend unknown exit "+std::to_string(exit));
        auto basis=quotient::read_basis(gb,gen.system);
        OracleResult out;out.extraction=quotient::Extractor(gen.system,basis,128).run();
        for(const auto&assignment:out.extraction.points) {
            auto model=decode(gen,assignment);
            if(!out.models.insert(model).second)throw std::runtime_error("duplicate norm witness");
        }
        std::ofstream models(directory/"models.tsv");
        for(const auto&model:out.models)models<<"model\t"<<model<<'\n';
        models<<"summary\t"<<out.models.size()<<'\t'<<out.extraction.dimension<<'\t'
              <<out.extraction.normal_form_reductions<<'\t'<<out.extraction.eigenspace_splits<<'\n';
        return out;
    }
};

std::set<std::string> point_reference(coeff::Config cfg,unsigned m=3) {
    reference::Curve E{cfg.field.p,cfg.a,cfg.b};
    std::vector<Point> base;
    for(const auto&point:E.points())
        if(point.x!=cfg.rx&&E.powmod(point.x,1U<<cfg.t)==cfg.c)base.push_back(point);
    std::set<std::string> out;
    std::vector<std::pair<int,int>> chosen;
    std::set<int> used;
    std::function<void(std::size_t,Point)> visit=[&](std::size_t start,Point sum) {
        if(chosen.size()==m) {
            if(reference::equal(sum,{cfg.rx,cfg.ry,false}))out.insert(coeff::key(chosen));
            return;
        }
        for(std::size_t i=start;i<base.size();++i)if(!used.count(base[i].x)) {
            used.insert(base[i].x);chosen.emplace_back(base[i].x,base[i].y);
            visit(i+1,E.sum(sum,base[i]));chosen.pop_back();used.erase(base[i].x);
        }
    };
    visit(0,{});return out;
}
Point multiply(const reference::Curve&E,Point point,int k) {
    Point out;
    while(k){if(k&1)out=E.sum(out,point);k>>=1;if(k)point=E.sum(point,point);}
    return out;
}
struct ColdResult {int scalar;std::size_t attempts,calls,misses,rank;};
template<class OracleType>
ColdResult cold_collect(const reference::Curve&E,Point P,Point Q,std::uint64_t seed,
    int scalar_order,const std::vector<Point>&base,int membership_depth,int membership_terminal,OracleType&oracle) {
    if(scalar_order<3||scalar_order>257||!reference::isprime(scalar_order)||base.empty()
       ||P.infinity||Q.infinity||!E.contains(P)||!E.contains(Q)
       ||!multiply(E,P,scalar_order).infinity||!multiply(E,Q,scalar_order).infinity)
        throw std::invalid_argument("prime-order collector admission");
    for(const auto&point:base)if(point.infinity||!E.contains(point)||!multiply(E,point,scalar_order).infinity)
        throw std::invalid_argument("collector base subgroup admission");
    std::mt19937_64 rng(seed);
    std::ofstream attempts_log(oracle.root/"cold-attempts.tsv");
    std::ofstream relations_log(oracle.root/"cold-relations.tsv");
    std::ofstream kernels_log(oracle.root/"cold-kernels.tsv");
    if(!attempts_log||!relations_log||!kernels_log)throw std::runtime_error("cold trace open failed");
    attempts_log<<"attempt\tbatch\ta\tb\trx\try\tstatus\tbackend_call\n";
    relations_log<<"attempt\tbatch\ta\tb\trx\try\trow\tmodel\n";
    kernels_log<<"batch\trank\tw\talpha\tbeta\tstatus\n";
    auto vector_text=[](const std::vector<int>&values) {
        std::ostringstream out;
        for(std::size_t i=0;i<values.size();++i){if(i)out<<',';out<<values[i];}
        return out.str();
    };
    auto sample=[&]() {
        const auto order=static_cast<std::uint64_t>(scalar_order);
        const auto threshold=(std::uint64_t(0)-order)%order;
        std::uint64_t value;do{value=rng();}while(value<threshold);
        return static_cast<int>(value%order);
    };
    std::size_t attempts=0,misses=0,calls=0;
    std::uint64_t arithmetic_ops=0;
    for(unsigned batch=0;batch<64;++batch) {
        std::vector<std::vector<int>> C;
        std::vector<int> aa,bb;
        while(C.size()<base.size()+1) {
            if(++attempts>4096)throw std::runtime_error("cold solve attempt cap");
            const int a=sample(),b=sample();
            const Point R=E.sum(multiply(E,P,a),multiply(E,Q,b));
            if(R.infinity){
                ++misses;attempts_log<<attempts<<'\t'<<batch<<'\t'<<a<<'\t'<<b<<"\t-\t-\tidentity\t-\n";
                attempts_log.flush();continue;
            }
            // Only public curve, domain and R enter the oracle, never a,b,Q.
            OracleResult answer;++calls;
            try{answer=oracle.solve({{E.p},E.a,E.b,membership_depth,membership_terminal,R.x,R.y});}
            catch(const std::exception&){
                attempts_log<<attempts<<'\t'<<batch<<'\t'<<a<<'\t'<<b<<'\t'<<R.x<<'\t'<<R.y<<"\toracle_unknown\t"<<oracle.calls<<'\n';
                attempts_log.flush();throw;
            }
            if(answer.models.empty()){
                ++misses;attempts_log<<attempts<<'\t'<<batch<<'\t'<<a<<'\t'<<b<<'\t'<<R.x<<'\t'<<R.y<<"\tno_relation\t"<<oracle.calls<<'\n';
                attempts_log.flush();continue;
            }
            std::vector<int> row(base.size());Point check;
            std::stringstream input(*answer.models.begin());std::string token;
            while(std::getline(input,token,';')) {
                const auto cut=token.find(':');
                Point point{std::stoi(token.substr(0,cut)),std::stoi(token.substr(cut+1)),false};
                const auto found=std::find_if(base.begin(),base.end(),[&](Point v){return v.x==point.x;});
                if(found==base.end()||row[found-base.begin()]!=0)throw std::runtime_error("point absent/repeated in base");
                if(point.y!=found->y&&point.y!=E.p-found->y)throw std::runtime_error("noncanonical relation sign");
                row[found-base.begin()]=point.y==found->y?1:scalar_order-1;
                check=E.sum(check,point);
            }
            if(!reference::equal(check,R))throw std::runtime_error("cold relation replay failed");
            attempts_log<<attempts<<'\t'<<batch<<'\t'<<a<<'\t'<<b<<'\t'<<R.x<<'\t'<<R.y<<"\taccepted\t"<<oracle.calls<<'\n';
            relations_log<<attempts<<'\t'<<batch<<'\t'<<a<<'\t'<<b<<'\t'<<R.x<<'\t'<<R.y<<'\t'<<vector_text(row)<<'\t'<<*answer.models.begin()<<'\n';
            attempts_log.flush();relations_log.flush();
            C.push_back(row);aa.push_back(a);bb.push_back(b);
        }
        const auto kernel=direct::left_null(C,{&arithmetic_ops,scalar_order});
        const auto recovered=direct::recover(kernel.w,aa,bb,{&arithmetic_ops,scalar_order});
        kernels_log<<batch<<'\t'<<kernel.rank<<'\t'<<vector_text(kernel.w)<<'\t'<<recovered.alpha<<'\t'<<recovered.beta<<'\t'<<(recovered.scalar?"return":"retry")<<'\n';
        kernels_log.flush();
        if(!recovered.scalar)continue;
        if(!reference::equal(multiply(E,P,*recovered.scalar),Q))throw std::runtime_error("final scalar replay failed");
        return {*recovered.scalar,attempts,calls,misses,kernel.rank};
    }
    throw std::runtime_error("cold solve batch cap");
}
template<class OracleType>
ColdResult cold_solve(Point Q,std::uint64_t seed,OracleType&oracle) {
    const reference::Curve E{17,2,2};const Point P{5,1,false};
    if(Q.infinity||!E.contains(Q)||Q.x<0||Q.x>=17||Q.y<0||Q.y>=17)
        throw std::invalid_argument("canonical nonzero public target required");
    // Prime19 and Hasse's interval[10,26] admit exactly this whole group.
    if(!reference::isprime(19)||P.infinity||!E.contains(P)
       ||!multiply(E,P,19).infinity||!multiply(E,Q,19).infinity
       ||((10+18)/19)!=26/19)
        throw std::runtime_error("fixture group admission failed");
    std::vector<Point> base;
    coeff::Poly membership_polynomial(9,0);membership_polynomial[0]=16;membership_polynomial[8]=1;
    const auto abscissae=roots(membership_polynomial,17);
    for(int x:abscissae) {
        const int rhs=E.residue(x*x*x+2*x+2);
        for(int y:roots(coeff::Poly{E.residue(-rhs),0,1},17))if(y<=8) {
            Point point{x,y,false};
            if(!E.contains(point)||!multiply(E,point,19).infinity)
                throw std::runtime_error("algebraic factor-base point admission");
            base.push_back(point);
        }
    }
    if(base.size()!=3)throw std::runtime_error("base dimension mismatch");
    {
        std::ofstream setup(oracle.root/"cold-setup.tsv");
        setup<<"order_admission\tprime_order19_and_Hasse_10_26\n";
        setup<<"abscissa_roots\t"<<abscissae.size()<<'\n';
        for(Point point:base)setup<<"base\t"<<point.x<<'\t'<<point.y<<'\n';
    }
    return cold_collect(E,P,Q,seed,19,base,3,1,oracle);
}
} // namespace algebraic

#ifndef NORM_ALGEBRAIC_NO_MAIN
int main(int argc,char**argv) {
    try {
        using namespace algebraic;
        if(argc==2&&std::string(argv[1])=="--selftest") {
            quotient::selftest();direct::unit_linear_checks();
            for(unsigned m:{3U,5U,7U}) {
                const coeff::Config cfg{{17},2,2,4,1,0,6};
                const Generator original(cfg,m),pruned(cfg,m,true);
                if(original.system.names.size()-pruned.system.names.size()!=(7*m-3)/2)
                    throw std::runtime_error("witness-pruned variable count");
            }
            bool rejected=false;
            try{Generator bad({{17},2,2,4,0,0,6},3,true);}
            catch(const std::invalid_argument&){rejected=true;}
            if(!rejected)throw std::runtime_error("zero-c pruned input accepted");
            std::cout<<"PASS quotient and rank-free linear controls\n";return 0;
        }
        if(argc==3&&std::string(argv[1])=="--retained") {
            Generator gen({{17},2,2,4,1,0,6},3);
            const auto s=canonical::read_system("research/prime_one_shot_20261029/continuation/canonical/outputs/m3_t4_remainder.ms");
            if(gen.system.names!=s.names)throw std::runtime_error("generator name mismatch");
            if(gen.system.equations.size()!=s.equations.size())throw std::runtime_error("generator equation count");
            for(std::size_t i=0;i<s.equations.size();++i)
                if(s.equations[i].terms!=gen.system.equations[i].terms)throw std::runtime_error("generator coefficient map mismatch");
            const auto basis=quotient::read_basis(argv[2],s);
            const auto points=quotient::Extractor(s,basis,128).run();
            std::set<std::string> models;
            for(const auto&assignment:points.points)models.insert(decode(gen,assignment));
            if(models!=point_reference(gen.cfg))throw std::runtime_error("retained model-set mismatch");
            std::cout<<"PASS retained models="<<models.size()<<" dimension="<<points.dimension<<" generator_identical=true\n";
            return 0;
        }
        if(argc==5&&(std::string(argv[1])=="--qualify-models"||std::string(argv[1])=="--qualify-pruned-models")) {
            Oracle oracle(argv[2],argv[3],std::string(argv[1])=="--qualify-pruned-models");
            std::ofstream log(argv[4]);
            log<<"t\tc\trx\try\tmodels\tquotient_dimension\tstatus\n";
            reference::Curve E{17,2,2};std::size_t cases=0,models=0;
            for(const auto&domain:std::vector<std::pair<int,int>>{{1,1},{2,1},{3,1},{3,16},{4,1}})
                for(const auto&R:E.points()) {
                    coeff::Config cfg{{17},2,2,domain.first,domain.second,R.x,R.y};
                    auto out=oracle.solve(cfg);
                    if(out.models!=point_reference(cfg))throw std::runtime_error("fresh complete model-set mismatch");
                    log<<cfg.t<<'\t'<<cfg.c<<'\t'<<R.x<<'\t'<<R.y<<'\t'<<out.models.size()<<'\t'<<out.extraction.dimension<<"\tmatch\n";log.flush();
                    ++cases;models+=out.models.size();
                }
            std::cout<<"PASS complete model controls="<<cases<<" models="<<models<<" backend_calls="<<oracle.calls<<'\n';return 0;
        }
        if(argc==5&&(std::string(argv[1])=="--qualify-cold"||std::string(argv[1])=="--qualify-pruned-cold")) {
            const auto root=std::filesystem::absolute(argv[3]);
            if(!std::filesystem::create_directory(root))throw std::runtime_error("reused cold qualification path");
            std::ofstream log(argv[4]);log<<"target_x\ttarget_y\trecovered_scalar\tattempts\tbackend_calls\tmisses\trank\n";
            const reference::Curve E{17,2,2};const Point P{5,1,false};
            for(int k=1;k<19;++k) {
                // Known scalar is only in this harness, outside cold_solve.
                const auto Q=multiply(E,P,k);
                Oracle oracle(argv[2],root/("target-x"+std::to_string(Q.x)+"-y"+std::to_string(Q.y)),std::string(argv[1])=="--qualify-pruned-cold");
                const auto out=cold_solve(Q,1234,oracle);
                if(out.scalar!=k)throw std::runtime_error("cold scalar mismatch");
                log<<Q.x<<'\t'<<Q.y<<'\t'<<out.scalar<<'\t'<<out.attempts<<'\t'<<out.calls<<'\t'<<out.misses<<'\t'<<out.rank<<'\n';log.flush();
            }
            std::cout<<"PASS cold point-only IC targets=18\n";return 0;
        }
        if(argc==5&&(std::string(argv[1])=="--qualify-arity"||std::string(argv[1])=="--qualify-pruned-arity")) {
            Oracle oracle(argv[2],argv[3],std::string(argv[1])=="--qualify-pruned-arity");std::ofstream log(argv[4]);
            log<<"m\tt\tc\treference_models\tcandidate_models\tquotient_dimension\tstatus\n";
            const std::vector<std::array<int,3>> cells{{5,3,16},{5,4,1},{7,4,1}};
            for(const auto&cell:cells) {
                const coeff::Config cfg{{17},2,2,cell[1],cell[2],0,6};
                const auto reference=point_reference(cfg,cell[0]);
                try {
                    const auto out=oracle.solve(cfg,cell[0]);
                    if(out.models!=reference)throw std::runtime_error("arity complete-model mismatch");
                    log<<cell[0]<<'\t'<<cell[1]<<'\t'<<cell[2]<<'\t'<<reference.size()<<'\t'<<out.models.size()<<'\t'<<out.extraction.dimension<<"\tmatch\n";
                }catch(const std::exception&e) {
                    // Controls preserve an unknown result, never infer UNSAT.
                    log<<cell[0]<<'\t'<<cell[1]<<'\t'<<cell[2]<<'\t'<<reference.size()<<"\t-\t-\tunknown: "<<e.what()<<'\n';
                    log.flush();
                    if(std::string(e.what())=="arity complete-model mismatch")throw;
                }
                log.flush();
            }
            std::cout<<"Completed three capped arity controls; see TSV for match/unknown\n";return 0;
        }
        if(argc==7&&(std::string(argv[1])=="--solve"||std::string(argv[1])=="--solve-pruned")) {
            Oracle oracle(argv[2],argv[3],std::string(argv[1])=="--solve-pruned");
            const auto out=cold_solve({reference::read_int(argv[4]),reference::read_int(argv[5]),false},std::stoull(argv[6]),oracle);
            std::cout<<"{\"scalar\":"<<out.scalar<<",\"attempts\":"<<out.attempts<<",\"backend_calls\":"<<out.calls<<",\"misses\":"<<out.misses<<",\"rank\":"<<out.rank<<"}\n";return 0;
        }
        throw std::invalid_argument("usage: norm_algebraic --selftest | --retained BASIS | --qualify-models MSOLVE OUTDIR TSV | --qualify-cold MSOLVE OUTDIR TSV | --qualify-arity MSOLVE OUTDIR TSV | --qualify-pruned-{models,cold,arity} MSOLVE OUTDIR TSV | --solve[-pruned] MSOLVE OUTDIR Qx Qy SEED");
    }catch(const std::exception&e){std::cerr<<"UNKNOWN: "<<e.what()<<'\n';return 2;}
}
#endif
