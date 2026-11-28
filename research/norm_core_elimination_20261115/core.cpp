// Unit-coefficient graph elimination for the shared norm/remainder system.
// Reuses the parent's arithmetic, extraction, backend caps and cold recovery.
#define NORM_ALGEBRAIC_NO_MAIN
#include "../shared_norm_algebraic_20261115/norm_algebraic.cpp"

namespace core {
using namespace algebraic;
struct Generator {
    algebraic::Generator full;
    System system;
    std::vector<Polynomial> images;
    std::vector<std::size_t> kept;
    unsigned degree=0;
    std::size_t zero_equations=0;

    Polynomial constant(U x)const{return Polynomial::constant(system.names.size(),system.p,x);}
    Polynomial substitute(const Polynomial&f)const {
        auto out=constant(0);
        for(const auto&term:f.terms) {
            auto product=constant(term.second);
            for(std::size_t i=0;i<term.first.size();++i)if(term.first[i])
                product=product.times(images.at(i).power(term.first[i]));
            out=out.plus(product);
        }
        return out;
    }
    Generator(coeff::Config cfg,unsigned m):full(cfg,m,true) {
        const reference::Curve curve{cfg.field.p,cfg.a,cfg.b};
        if(!canonical::prime(cfg.field.p)||cfg.rx<0||cfg.rx>=cfg.field.p
           ||cfg.ry<0||cfg.ry>=cfg.field.p||!curve.contains({cfg.rx,cfg.ry,false}))
            throw std::invalid_argument("core encoding requires a canonical curve target");
        const auto oldn=full.system.names.size();
        std::vector<bool> removed(oldn,false);
        removed.at(full.a[0])=true;
        for(auto id:full.u)removed.at(id)=true;
        unsigned exponent=2;
        for(int j=0;j<cfg.t;++j,exponent*=2) {
            if(exponent<m) {
                for(auto id:full.q[j])removed.at(id)=true;
                for(auto id:full.r[j])removed.at(id)=true;
            }else if(exponent/2<m) {
                const unsigned bound=exponent-m;
                for(std::size_t i=bound+1;i<full.q[j].size();++i)removed.at(full.q[j][i])=true;
            }
        }
        system.p=full.system.p;
        for(std::size_t i=0;i<oldn;++i)if(!removed[i]) {
            system.variables.emplace(full.system.names[i],system.names.size());
            system.names.push_back(full.system.names[i]);kept.push_back(i);
        }
        images.assign(oldn,constant(0));
        for(std::size_t i=0;i<kept.size();++i)
            images[kept[i]]=Polynomial::variable(kept.size(),system.p,i);
        exponent=2;
        for(int j=0;j<cfg.t;++j,exponent*=2)if(exponent<m&&!full.r[j].empty())
            images.at(full.r[j].at(exponent))=constant(1);

        const auto A=full.polynomial(full.a,true),C=full.polynomial(full.c);
        // a0 currently maps to zero, leaving only the known higher A terms.
        images[full.a[0]]=substitute(full.evaluate(C,cfg.rx)).times(constant(cfg.ry))
            .plus(substitute(full.evaluate(A,cfg.rx)),true);
        const P f{full.constant(cfg.b),full.constant(cfg.a),full.constant(0),full.constant(1)};
        const auto norm=full.subtract(full.multiply(A,A),full.multiply(f,full.multiply(C,C)));
        auto next=constant(1); // leading monic coefficient of U
        for(unsigned j=m;j-->0;) {
            next=substitute(norm.at(j+1)).plus(next.times(constant(cfg.rx)));
            images.at(full.u.at(j))=next;
        }
        if(!substitute(norm[0]).plus(images[full.u[0]].times(constant(cfg.rx))).terms.empty())
            throw std::runtime_error("synthetic division remainder is not identically zero");
        for(std::size_t i=0;i<full.system.equations.size();++i) {
            auto transformed=substitute(full.system.equations[i]);
            if(i<m+2&&!transformed.terms.empty())
                throw std::runtime_error("eliminated norm/target equation did not cancel");
            if(transformed.terms.empty()){++zero_equations;continue;}
            degree=std::max(degree,transformed.largest_degree());
            system.equations.push_back(std::move(transformed));
        }
        if(degree>3)throw std::runtime_error("core input degree exceeded three");
    }
    std::vector<U> lift(const std::vector<U>&assignment)const {
        std::vector<U> lifted;for(const auto&image:images)lifted.push_back(image.evaluate(assignment));
        return lifted;
    }
    std::string decode(const std::vector<U>&assignment)const {
        const auto lifted=lift(assignment);
        for(const auto&equation:full.system.equations)if(equation.evaluate(lifted)!=0)
            throw std::runtime_error("lifted original equation failed");
        return algebraic::decode(full,lifted);
    }
};
struct Oracle:algebraic::Oracle {
    using algebraic::Oracle::Oracle;
    OracleResult solve(coeff::Config cfg,unsigned m=3) {
        const auto directory=root/("call-"+std::to_string(++calls));
        if(!std::filesystem::create_directory(directory))throw std::runtime_error("reused core call path");
        Generator gen(cfg,m);
        {
            std::ofstream receipt(directory/"encoding.tsv");
            receipt<<"full_variables\t"<<gen.full.system.names.size()<<"\ncore_variables\t"<<gen.system.names.size()
                   <<"\ninput_degree\t"<<gen.degree<<"\nzero_equations\t"<<gen.zero_equations<<'\n';
        }
        const auto input=directory/"input.ms",gb=directory/"basis.gb";
        canonical::write_system(input,gen.system,false);
        const int exit=backend(exe,input,gb,directory);
        {std::ofstream receipt(directory/"exit.txt");receipt<<exit<<'\n';}
        if(exit!=0)throw std::runtime_error("backend unknown exit "+std::to_string(exit));
        const auto basis=quotient::read_basis(gb,gen.system);
        OracleResult out;out.extraction=quotient::Extractor(gen.system,basis,128).run();
        for(const auto&assignment:out.extraction.points)
            if(!out.models.insert(gen.decode(assignment)).second)throw std::runtime_error("duplicate core witness");
        std::ofstream models(directory/"models.tsv");
        for(const auto&model:out.models)models<<"model\t"<<model<<'\n';
        models<<"summary\t"<<out.models.size()<<'\t'<<out.extraction.dimension<<'\t'
              <<out.extraction.normal_form_reductions<<'\t'<<out.extraction.eigenspace_splits<<'\n';
        return out;
    }
};
void selftest() {
    const std::vector<std::pair<unsigned,std::size_t>> sizes{{3,15},{5,18},{7,22}};
    for(const auto&shape:sizes) {
        const Generator gen({{17},2,2,4,1,0,6},shape.first);
        if(gen.system.names.size()!=shape.second||gen.degree>3)throw std::runtime_error("core shape mismatch");
        // Graph substitution must round-trip every free coordinate, even away
        // from the solution set; this tests accidental old/new indexing leaks.
        std::vector<U> assignment(gen.kept.size());
        for(std::size_t i=0;i<assignment.size();++i)assignment[i]=(i*7+3)%17;
        const auto lifted=gen.lift(assignment);
        for(std::size_t i=0;i<assignment.size();++i)if(lifted[gen.kept[i]]!=assignment[i])
            throw std::runtime_error("graph round-trip mismatch");
    }
    unsigned rejected=0;
    try{Generator bad({{17},2,2,4,0,0,6},3);}catch(const std::invalid_argument&){++rejected;}
    try{Generator bad({{17},2,2,4,1,0,0},3);}catch(const std::invalid_argument&){++rejected;}
    if(rejected!=2)throw std::runtime_error("core admission controls failed");
    std::cout<<"PASS core shape, symbolic cancellation, graph round-trip and admission controls\n";
}
} // namespace core

#ifndef NORM_CORE_NO_MAIN
int main(int argc,char**argv) {
    try {
        using namespace algebraic;
        if(argc==2&&std::string(argv[1])=="--selftest"){core::selftest();return 0;}
        // Bounded fixtures below are the same frozen outer harness controls as
        // the parent. Production arithmetic and cold recovery are reused above.
        if(argc==5&&std::string(argv[1])=="--qualify-models") {
            core::Oracle oracle(argv[2],argv[3]);std::ofstream log(argv[4]);
            log<<"t\tc\trx\try\tmodels\tquotient_dimension\tstatus\n";
            reference::Curve E{17,2,2};std::size_t cases=0,models=0;
            for(const auto&domain:std::vector<std::pair<int,int>>{{1,1},{2,1},{3,1},{3,16},{4,1}})
                for(const auto&R:E.points()) {
                    const coeff::Config cfg{{17},2,2,domain.first,domain.second,R.x,R.y};
                    const auto out=oracle.solve(cfg);
                    if(out.models!=point_reference(cfg))throw std::runtime_error("core complete-model mismatch");
                    log<<cfg.t<<'\t'<<cfg.c<<'\t'<<R.x<<'\t'<<R.y<<'\t'<<out.models.size()<<'\t'<<out.extraction.dimension<<"\tmatch\n";log.flush();
                    ++cases;models+=out.models.size();
                }
            std::cout<<"PASS core complete model cases="<<cases<<" models="<<models<<" backend_calls="<<oracle.calls<<'\n';return 0;
        }
        if(argc==5&&std::string(argv[1])=="--qualify-arity") {
            core::Oracle oracle(argv[2],argv[3]);std::ofstream log(argv[4]);
            log<<"m\tt\tc\treference_models\tcandidate_models\tquotient_dimension\tstatus\n";
            for(const auto&cell:std::vector<std::array<int,3>>{{5,3,16},{5,4,1},{7,4,1}}) {
                const coeff::Config cfg{{17},2,2,cell[1],cell[2],0,6};const auto expected=point_reference(cfg,cell[0]);
                try {
                    const auto out=oracle.solve(cfg,cell[0]);
                    if(out.models!=expected)throw std::runtime_error("core arity model mismatch");
                    log<<cell[0]<<'\t'<<cell[1]<<'\t'<<cell[2]<<'\t'<<expected.size()<<'\t'<<out.models.size()<<'\t'<<out.extraction.dimension<<"\tmatch\n";
                }catch(const std::exception&e) {
                    log<<cell[0]<<'\t'<<cell[1]<<'\t'<<cell[2]<<'\t'<<expected.size()<<"\t-\t-\tunknown: "<<e.what()<<'\n';log.flush();
                    if(std::string(e.what())=="core arity model mismatch")throw;
                }
                log.flush();
            }
            std::cout<<"Completed three capped core arity controls; see TSV\n";return 0;
        }
        if(argc==5&&std::string(argv[1])=="--qualify-cold") {
            const auto root=std::filesystem::absolute(argv[3]);
            if(!std::filesystem::create_directory(root))throw std::runtime_error("reused core cold path");
            std::ofstream log(argv[4]);log<<"target_x\ttarget_y\trecovered_scalar\tattempts\tbackend_calls\tmisses\trank\n";
            const reference::Curve E{17,2,2};const Point P{5,1,false};
            for(int k=1;k<19;++k) {
                const auto Q=multiply(E,P,k);
                core::Oracle oracle(argv[2],root/("target-x"+std::to_string(Q.x)+"-y"+std::to_string(Q.y)));
                const auto out=cold_solve(Q,1234,oracle);
                if(out.scalar!=k)throw std::runtime_error("core cold scalar mismatch");
                log<<Q.x<<'\t'<<Q.y<<'\t'<<out.scalar<<'\t'<<out.attempts<<'\t'<<out.calls<<'\t'<<out.misses<<'\t'<<out.rank<<'\n';log.flush();
            }
            std::cout<<"PASS core cold point-only targets=18\n";return 0;
        }
        if(argc==7&&std::string(argv[1])=="--solve") {
            core::Oracle oracle(argv[2],argv[3]);
            const auto out=cold_solve({reference::read_int(argv[4]),reference::read_int(argv[5]),false},std::stoull(argv[6]),oracle);
            std::cout<<"{\"scalar\":"<<out.scalar<<",\"attempts\":"<<out.attempts<<",\"backend_calls\":"<<out.calls<<"}\n";return 0;
        }
        throw std::invalid_argument("usage: core --selftest | --qualify-{models,arity,cold} MSOLVE OUTDIR TSV | --solve MSOLVE OUTDIR Qx Qy SEED");
    }catch(const std::exception&e){std::cerr<<"UNKNOWN: "<<e.what()<<'\n';return 2;}
}
#endif
