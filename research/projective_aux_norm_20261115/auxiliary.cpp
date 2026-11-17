// Auxiliary elliptic fibre and implicit projective norm membership, native only.
#define NORM_ALGEBRAIC_NO_MAIN
#include "../shared_norm_algebraic_20261115/norm_algebraic.cpp"
namespace auxiliary {
using namespace algebraic;
void check(bool condition,const std::string&message){if(!condition)throw std::runtime_error(message);}
struct Step {
    reference::Curve domain,codomain;int xi,tau;
    Point map(Point P)const{
        check(tau!=0,"nonunit Velu parameter");
        check(P.infinity||(P.x>=0&&P.x<domain.p&&P.y>=0&&P.y<domain.p),"canonical isogeny point");
        check(domain.contains(P),"off-curve isogeny input");if(P.infinity||P.x==xi)return {};
        const auto inv=domain.inverse(P.x-xi);
        Point image{domain.residue(P.x+tau*inv),domain.residue(P.y*(1-tau*inv*inv)),false};
        check(codomain.contains(image),"isogeny codomain mismatch");return image;
    }
};
struct Tower {
    reference::Curve initial{19,2,2};Point generator{16,11,false},coset{12,5,false};
    std::vector<Step> steps;int terminal=0;
    Tower(reference::Curve curve={19,2,2},Point subgroup={16,11,false},Point representative={12,5,false})
        :initial(curve),generator(subgroup),coset(representative){
        check(initial.contains(generator)&&initial.contains(coset),"auxiliary point admission");
        check(multiply(initial,generator,8).infinity&&!multiply(initial,generator,4).infinity,"auxiliary exact order8");
        auto E=initial;auto T=generator,G=coset;
        for(int order=8;order>1;order/=2){
            const auto kernel=multiply(E,T,order/2);check(!kernel.infinity&&kernel.y==0,"order2 kernel");
            const int xi=kernel.x,tau=E.residue(3*xi*xi+E.a);check(tau!=0,"nonunit Velu parameter");
            const reference::Curve codomain{E.p,E.residue(E.a-5*tau),E.residue(E.b-7*xi*tau)};
            check(E.residue(4*codomain.a*codomain.a*codomain.a+27*codomain.b*codomain.b)!=0,"singular codomain");
            Step step{E,codomain,xi,tau};T=step.map(T);G=step.map(G);steps.push_back(step);E=codomain;
        }
        check(T.infinity&&!G.infinity&&!multiply(E,G,2).infinity,"auxiliary terminal/trace admission");terminal=G.x;
    }
    std::optional<int> evaluate(int x)const{
        if(x<0||x>=initial.p)return std::nullopt;
        int P=x,Q=1;
        for(const auto&step:steps){
            const int n=initial.residue(P*P-step.xi*P*Q+step.tau*Q*Q);
            const int d=initial.residue(P*Q-step.xi*Q*Q);P=n;Q=d;
        }
        if(Q==0)return std::nullopt;
        return initial.residue(P*initial.inverse(Q));
    }
    bool member(int x)const{const auto value=evaluate(x);return value&&*value==terminal;}
    std::set<int> fibre()const{
        std::set<int> out;Point H;
        for(int i=0;i<8;++i){const auto P=initial.sum(coset,H);check(!P.infinity,"coset infinity");
            check(out.insert(P.x).second&&member(P.x),"coset duplicate/membership");H=initial.sum(H,generator);}
        check(H.infinity&&out.size()==8,"coset fibre size");return out;
    }
};
struct Generator:algebraic::Generator {
    const Tower&tower;P A,C,U0;
    std::vector<std::vector<std::size_t>> qp,qd,rp,rd;
    unsigned max_degree=0;
    P scaled(P value,int scalar)const{for(auto&v:value)v=v.times(constant(cfg.field.mod(scalar)));return value;}
    P coefficients(const std::vector<std::size_t>&ids)const{return ids.empty()?P{constant(0)}:polynomial(ids);}
    Generator(coeff::Config input,unsigned arity,const Tower&public_tower)
        :algebraic::Generator(input,arity,DeferredMembership{}),tower(public_tower){
        const reference::Curve E{cfg.field.p,cfg.a,cfg.b};
        check(cfg.field.p==19&&cfg.t==3&&cfg.c==tower.terminal,"bounded auxiliary fixture");
        check(cfg.rx>=0&&cfg.rx<19&&cfg.ry>=0&&cfg.ry<19&&E.contains({cfg.rx,cfg.ry,false}),"canonical main target");
        for(unsigned i=1;i<h;++i)a.push_back(name("a"+std::to_string(i)));
        c=names("c",h-1);target_inverse=name("target_inverse");
        for(unsigned j=1;j<=3;++j){
            const bool terminal=j==3,public_next=(1U<<j)<m&&!terminal;
            const bool public_previous=(1U<<(j-1))<m;
            const unsigned dn=public_previous?(1U<<j):2*m-2;
            const unsigned dd=public_previous?(1U<<j)-1:2*m-2;
            qp.push_back(public_next?std::vector<std::size_t>{}:names("qn"+std::to_string(j)+"_",dn>=m?dn-m+1:0));
            qd.push_back(public_next||terminal?std::vector<std::size_t>{}:names("qd"+std::to_string(j)+"_",dd>=m?dd-m+1:0));
            rp.push_back(public_next||terminal?std::vector<std::size_t>{}:names("rn"+std::to_string(j)+"_",m));
            rd.push_back(public_next||terminal?std::vector<std::size_t>{}:names("rd"+std::to_string(j)+"_",m));
        }
        A={constant(0)};for(auto id:a)A.push_back(variable(id));A.push_back(constant(1));C=polynomial(c);
        A[0]=evaluate(C,cfg.rx).times(constant(cfg.ry)).plus(evaluate(A,cfg.rx),true);
        const P f{constant(cfg.b),constant(cfg.a),constant(0),constant(1)};
        const auto norm=subtract(multiply(A,A),multiply(f,multiply(C,C)));
        U0.assign(m+1,constant(0));U0[m]=constant(1);
        for(unsigned j=m;j-->0;)U0[j]=norm.at(j+1).plus(U0[j+1].times(constant(cfg.rx)));
        for(const auto&v:subtract(norm,multiply(P{constant(cfg.field.mod(-cfg.rx)),constant(1)},U0)))
            check(v.terms.empty(),"symbolic norm division failed");
        equations(P{evaluate(U0,cfg.rx).times(variable(target_inverse)).plus(constant(1),true)});
        P previousP{constant(0),constant(1)},previousQ{constant(1)};
        for(unsigned j=0;j<3;++j){const auto&step=tower.steps.at(j);
            const auto PQ=multiply(previousP,previousQ),QQ=multiply(previousQ,previousQ);
            const auto N=plus(subtract(multiply(previousP,previousP),scaled(PQ,step.xi)),scaled(QQ,step.tau));
            const auto D=subtract(PQ,scaled(QQ,step.xi));
            if(j==2)equations(subtract(subtract(N,scaled(D,tower.terminal)),multiply(coefficients(qp[j]),U0)));
            else if((1U<<(j+1))<m){previousP=N;previousQ=D;}
            else{
                const auto nextP=polynomial(rp[j]),nextQ=polynomial(rd[j]);
                equations(subtract(subtract(N,multiply(coefficients(qp[j]),U0)),nextP));
                equations(subtract(subtract(D,multiply(coefficients(qd[j]),U0)),nextQ));previousP=nextP;previousQ=nextQ;
            }
        }
        for(const auto&eq:system.equations)max_degree=std::max(max_degree,eq.largest_degree());
        check(max_degree<=3,"auxiliary input degree cap");
    }
    std::string decode(const std::vector<canonical::U>&assignment)const{
        for(const auto&eq:system.equations)check(eq.evaluate(assignment)==0,"auxiliary original equation failed");
        auto values=[&](const P&polys){coeff::Poly out;for(const auto&p:polys)out.push_back(p.evaluate(assignment));coeff::trim(out);return out;};
        const auto av=values(A),cv=values(C),uv=values(U0);const auto xs=roots(uv,cfg.field.p);
        check(xs.size()==m,"auxiliary support rationality/squarefreeness");
        const reference::Curve E{cfg.field.p,cfg.a,cfg.b};Point sum;std::vector<std::pair<int,int>> points;
        for(int x:xs){const int den=coeff::evaluate(cfg.field,cv,x);check(den!=0,"auxiliary norm denominator");
            const Point P{x,cfg.field.mod(-coeff::evaluate(cfg.field,av,x)*cfg.field.inv(den)),false};
            check(E.contains(P)&&x!=cfg.rx&&tower.member(x),"auxiliary recovered point");sum=E.sum(sum,P);points.emplace_back(P.x,P.y);}
        check(reference::equal(sum,{cfg.rx,cfg.ry,false}),"auxiliary model group sum");return coeff::key(points);
    }
};
struct Eliminated {
    Generator full;System system;std::vector<Polynomial> images;
    unsigned max_degree=0;std::size_t zero_equations=0;
    Polynomial constant(U value)const{return Polynomial::constant(system.names.size(),system.p,value);}
    Polynomial substitute(const Polynomial&f)const{
        auto out=constant(0);
        for(const auto&term:f.terms){auto product=constant(term.second);
            for(std::size_t i=0;i<term.first.size();++i)if(term.first[i])product=product.times(images.at(i).power(term.first[i]));
            out=out.plus(product);
        }return out;
    }
    Eliminated(coeff::Config cfg,unsigned arity,const Tower&tower):full(cfg,arity,tower){
        check(arity==3&&full.qp[1].size()==2&&full.qd[1].size()==1&&full.rd[1].size()==3,"first-wrap graph shape");
        std::vector<bool> removed(full.system.names.size(),false);
        for(auto id:full.qp[1])removed.at(id)=true;
        for(auto id:full.qd[1])removed.at(id)=true;
        for(auto id:full.rd[1])removed.at(id)=true;
        system.p=full.system.p;std::vector<std::size_t> kept;
        for(std::size_t i=0;i<removed.size();++i)if(!removed[i]){
            system.variables.emplace(full.system.names[i],system.names.size());system.names.push_back(full.system.names[i]);kept.push_back(i);}
        images.assign(removed.size(),constant(0));
        for(std::size_t i=0;i<kept.size();++i)images[kept[i]]=Polynomial::variable(kept.size(),system.p,i);
        coeff::Poly P{0,1},Q{1};
        for(std::size_t j=0;j<2;++j){const auto&s=tower.steps[j];const auto&f=cfg.field;
            const auto PQ=coeff::mul(f,P,Q),QQ=coeff::mul(f,Q,Q);
            const auto N=coeff::add(f,coeff::add(f,coeff::mul(f,P,P),coeff::scale(f,PQ,s.xi),-1),coeff::scale(f,QQ,s.tau));
            const auto D=coeff::add(f,PQ,coeff::scale(f,QQ,s.xi),-1);P=N;Q=D;
        }
        check(P.size()==5&&P.back()==1&&Q.size()==4&&Q.back()==1,"public first-wrap degree/monicity");
        images[full.qp[1][1]]=constant(1);images[full.qd[1][0]]=constant(1);
        images[full.qp[1][0]]=constant(P[3]).plus(substitute(full.U0[2]),true);
        for(std::size_t k=0;k<3;++k)images[full.rd[1][k]]=constant(Q[k]).plus(substitute(full.U0[k]),true);
        for(const auto&eq:full.system.equations){auto projected=substitute(eq);
            if(projected.terms.empty()){++zero_equations;continue;}
            max_degree=std::max(max_degree,projected.largest_degree());system.equations.push_back(std::move(projected));}
        check(system.names.size()==8&&max_degree<=4&&zero_equations==6,"projected graph shape/cancellation");
    }
    std::string decode(const std::vector<U>&assignment)const{
        for(const auto&eq:system.equations)check(eq.evaluate(assignment)==0,"projected original equation failed");
        std::vector<U> lifted;for(const auto&image:images)lifted.push_back(image.evaluate(assignment));
        return full.decode(lifted);
    }
};
std::set<std::string> reference_models(const Tower&tower,Point R){
    const reference::Curve E{19,0,2};std::vector<Point> base;
    for(const auto&P:E.points())if(P.x!=R.x&&tower.member(P.x))base.push_back(P);
    std::set<std::string> out;
    for(std::size_t i=0;i<base.size();++i)for(std::size_t j=i+1;j<base.size();++j)for(std::size_t k=j+1;k<base.size();++k){
        if(base[i].x==base[j].x||base[i].x==base[k].x||base[j].x==base[k].x)continue;
        if(reference::equal(E.sum(E.sum(base[i],base[j]),base[k]),R))
            out.insert(coeff::key({{base[i].x,base[i].y},{base[j].x,base[j].y},{base[k].x,base[k].y}}));
    }return out;
}
void selftest(const Tower&tower,std::ostream&log){
    check(tower.steps.size()==3&&tower.terminal==15,"auxiliary terminal constants");
    const std::vector<std::pair<int,int>> expected{{14,1},{16,5},{17,3}};
    coeff::Field f{19};coeff::Poly P{0,1},Q{1};
    for(std::size_t j=0;j<3;++j){const auto&s=tower.steps[j];check(std::make_pair(s.xi,s.tau)==expected[j],"auxiliary step constants");
        for(const auto&point:s.domain.points())check(s.codomain.contains(s.map(point)),"full point-map check");
        check(s.map({}).infinity&&s.map({s.xi,0,false}).infinity,"kernel infinity check");
        const auto PQ=coeff::mul(f,P,Q),QQ=coeff::mul(f,Q,Q);
        const auto N=coeff::add(f,coeff::add(f,coeff::mul(f,P,P),coeff::scale(f,PQ,s.xi),-1),coeff::scale(f,QQ,s.tau));
        const auto D=coeff::add(f,PQ,coeff::scale(f,QQ,s.xi),-1);P=N;Q=D;
        log<<"step\t"<<j<<'\t'<<s.xi<<'\t'<<s.tau<<'\t'<<s.codomain.a<<'\t'<<s.codomain.b<<'\n';
    }
    const auto fibre=tower.fibre();coeff::Poly product{1};
    for(int x:fibre){product=coeff::mul(f,product,{f.mod(-x),1});log<<"fibre\t"<<x<<'\n';}
    check(coeff::add(f,P,coeff::scale(f,Q,tower.terminal),-1)==product,"full composed fibre identity");
    check(tower.evaluate(14)==std::nullopt&&!tower.evaluate(-1)&&!tower.evaluate(19),"pole/canonical admission");
    bool rejected=false;auto invalid=tower.steps[0];invalid.tau=0;
    try{invalid.map(tower.coset);}catch(const std::runtime_error&){rejected=true;}
    check(rejected,"zero tau accepted");rejected=false;
    try{tower.steps[0].map({0,0,false});}catch(const std::runtime_error&){rejected=true;}
    check(rejected,"invalid auxiliary point accepted");
    const Generator gen({{19},0,2,3,15,4,3},3,tower);
    check(gen.system.names.size()==14&&gen.max_degree<=3,"auxiliary shape");
    log<<"shape\t"<<gen.system.names.size()<<'\t'<<gen.max_degree<<'\n';
    const Eliminated projected({{19},0,2,3,15,4,3},3,tower);
    log<<"projected_shape\t"<<projected.system.names.size()<<'\t'<<projected.max_degree<<'\t'<<projected.zero_equations<<'\n';
}
} // namespace auxiliary
#ifndef AUXILIARY_NO_MAIN
int main(int argc,char**argv){
    try{
        const auxiliary::Tower tower;
        if(argc==2&&std::string(argv[1])=="--selftest"){auxiliary::selftest(tower,std::cout);return 0;}
        if(argc==5&&(std::string(argv[1])=="--qualify"||std::string(argv[1])=="--qualify-eliminated")){
            const bool eliminated=std::string(argv[1])=="--qualify-eliminated";
            const auto root=std::filesystem::absolute(argv[3]);auxiliary::check(std::filesystem::create_directory(root),"reused auxiliary root");
            std::ofstream log(argv[4]);log<<"rx\try\treference\tmodels\tquotient\tstatus\n";
            const reference::Curve E{19,0,2};const auto targets=E.points();auxiliary::check(targets.size()==12,"main curve census");
            std::size_t id=0;
            for(const auto&R:targets){const auto dir=root/("call-"+std::to_string(++id));std::filesystem::create_directory(dir);
                const auto expected=auxiliary::reference_models(tower,R);const auto input=dir/"input.ms",gb=dir/"basis.gb";
                auto solve=[&](const auto&gen){canonical::write_system(input,gen.system,false);
                const int status=algebraic::backend(std::filesystem::absolute(argv[2]),input,gb,dir);
                {std::ofstream out(dir/"exit.txt");out<<status<<'\n';}
                if(status!=0){log<<R.x<<'\t'<<R.y<<'\t'<<expected.size()<<"\t-\t-\tunknown exit"<<status<<'\n';log.flush();return;}
                quotient::Result result;std::set<std::string> models;bool duplicate=false;
                try{
                    result=quotient::Extractor(gen.system,quotient::read_basis(gb,gen.system),128).run();
                    for(const auto&point:result.points)if(!models.insert(gen.decode(point)).second)duplicate=true;
                }catch(const std::exception&e){
                    std::ofstream error(dir/"extraction-unknown.txt");error<<e.what()<<'\n';
                    log<<R.x<<'\t'<<R.y<<'\t'<<expected.size()<<"\t-\t-\tunknown extraction: "<<e.what()<<'\n';log.flush();return;
                }
                auxiliary::check(!duplicate,"duplicate auxiliary witness");
                std::ofstream out(dir/"models.tsv");for(const auto&model:models)out<<model<<'\n';
                auxiliary::check(models==expected,"complete auxiliary model mismatch");
                log<<R.x<<'\t'<<R.y<<'\t'<<expected.size()<<'\t'<<models.size()<<'\t'<<result.dimension<<"\tmatch\n";log.flush();
                };
                if(eliminated){const auxiliary::Eliminated gen({{19},0,2,3,15,R.x,R.y},3,tower);solve(gen);}
                else{const auxiliary::Generator gen({{19},0,2,3,15,R.x,R.y},3,tower);solve(gen);}
            }return 0;
        }
        throw std::invalid_argument("usage: auxiliary --selftest | --qualify[-eliminated] BACKEND NEW_ROOT TSV");
    }catch(const std::exception&e){std::cerr<<"UNKNOWN: "<<e.what()<<'\n';return 2;}
}
#endif
