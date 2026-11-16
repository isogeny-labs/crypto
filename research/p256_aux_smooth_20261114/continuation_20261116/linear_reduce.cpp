// Native exact affine-linear elimination, with explicit reconstruction.
// Input degree, affine output basis degree and regularity remain distinct.
#define main frozen_auxiliary_cli_main
#include "../aux_base.cpp"
#undef main

Poly pow_small(Poly p,unsigned e) {
    Poly out(1);
    while(e) {
        if(e&1) out=out*p;
        e>>=1; if(e) p=p*p;
        require(out.terms.size()<=100000 && p.terms.size()<=100000,"substitution term cap; no partial output admitted");
    }
    return out;
}
Poly substitute(const Poly& p,const std::string& variable,const Poly& replacement) {
    Poly out;
    for(const auto& [mono,c]:p.terms) {
        Monomial rest=mono; auto it=rest.find(variable); unsigned e=0;
        if(it!=rest.end()) { e=it->second; rest.erase(it); }
        Poly term; term.terms[rest]=c;
        out+=term*pow_small(replacement,e);
        require(out.terms.size()<=100000,"substitution term cap; no partial output admitted");
    }
    return out;
}
bool contains(const Poly& p,const std::string& variable) {
    for(const auto& [m,c]:p.terms) { (void)c; if(m.contains(variable)) return true; }
    return false;
}
// Independent algebraic identity for substitution divisibility:
// f(x)-f(L) = (x-L)*sum c*other*sum_{i=0}^{e-1} x^{e-1-i}*L^i.
void check_substitution_identity(const Poly& before,const Poly& after,
                                  const std::string& variable,const Poly& replacement) {
    Poly quotient,x=Poly::var(variable);
    for(const auto& [m,c]:before.terms) {
        auto found=m.find(variable); if(found==m.end()) continue;
        Monomial rest=m; unsigned exponent=found->second; rest.erase(variable);
        Poly coefficient; coefficient.terms[rest]=c;
        for(unsigned i=0;i<exponent;++i)
            quotient+=coefficient*pow_small(x,exponent-1-i)*pow_small(replacement,i);
    }
    require((before-after-(x-replacement)*quotient).terms.empty(),"substitution divisibility identity failed");
}
struct Pivot { std::string variable,equation; Z coefficient; Poly expression; };
void check_pivot(const Poly& original,const Pivot& p) {
    require(p.coefficient!=0 && !contains(p.expression,p.variable),"invalid unit pivot");
    require(p.expression.degree()<=1,"non-affine replacement rejected");
    require((original-Poly(p.coefficient)*(Poly::var(p.variable)-p.expression)).terms.empty(),"pivot identity failed");
}
std::string equations_json(const System& s) {
    std::vector<std::string> es;
    for(const auto& [name,p]:s.equations) es.push_back(object({{"name",jquote(name)},{"terms",p.json()}}));
    return array(es);
}
struct Reduced {
    System residual;
    std::map<std::string,Poly> reconstruction;
    std::vector<std::string> certificates;
    size_t original_variables=0,original_equations=0;
    unsigned original_degree=0;
};
Reduced reduce_linear(const System& original) {
    Reduced out; out.residual=original;
    out.original_variables=original.witness.size(); out.original_equations=original.equations.size();
    for(const auto& [name,value]:original.witness) { (void)value; out.reconstruction[name]=Poly::var(name); }
    for(const auto& [name,p]:original.equations) { (void)name; out.original_degree=std::max(out.original_degree,p.degree()); }
    while(true) {
        auto& es=out.residual.equations;
        es.erase(std::remove_if(es.begin(),es.end(),[](const auto& e){return e.second.terms.empty();}),es.end());
        size_t index=es.size(); Pivot pivot;
        for(size_t i=0;i<es.size();++i) {
            if(es[i].second.degree()!=1) continue;
            for(const auto& [mono,c]:es[i].second.terms) if(mono.size()==1 && mono.begin()->second==1) {
                pivot.variable=mono.begin()->first; pivot.equation=es[i].first; pivot.coefficient=c;
                pivot.expression=-(es[i].second-Poly(c)*Poly::var(pivot.variable))*Poly(inv(c));
                index=i; break;
            }
            if(index!=es.size()) break;
        }
        if(index==es.size()) break;
        const Poly original_pivot=es[index].second;
        check_pivot(original_pivot,pivot);
        std::string before_hash=hash(equations_json(out.residual));
        for(auto& [name,p]:es) {
            (void)name; Poly next=substitute(p,pivot.variable,pivot.expression);
            check_substitution_identity(p,next,pivot.variable,pivot.expression); p=std::move(next);
        }
        for(auto& [name,p]:out.reconstruction) { (void)name; p=substitute(p,pivot.variable,pivot.expression); }
        require(out.residual.witness.erase(pivot.variable)==1,"pivot variable not in ring");
        // The pivot generator is now zero. All other generators are retained,
        // so no probabilistic branch or exceptional addition chart is discarded.
        out.certificates.push_back(object({{"before_sha256",jquote(before_hash)},
          {"after_sha256",jquote(hash(equations_json(out.residual)))},{"coefficient",dec(pivot.coefficient)},
          {"equation",jquote(pivot.equation)},{"expression",pivot.expression.json()},
          {"pivot_polynomial",original_pivot.json()},{"variable",jquote(pivot.variable)}}));
    }
    return out;
}
void check_reconstruction(const System& original,const Reduced& reduced) {
    reduced.residual.check(); std::map<std::string,Z> restored;
    for(const auto& [name,expression]:reduced.reconstruction) restored[name]=expression.eval(reduced.residual.witness);
    require(restored==original.witness,"reconstruction disagrees with original assignment");
    for(const auto& [name,p]:original.equations) require(p.eval(restored)==0,"restored equation failed: "+name);
}
std::string magma_poly2(const Poly& p) {
    if(p.terms.empty()) return "0";
    std::string text;
    for(const auto& [m,c]:p.terms) {
        if(!text.empty()) text+=" + ";
        text+=c.get_str();
        for(const auto& [name,e]:m) text+="*"+name+"^"+num(e);
    }
    return text;
}
std::string reduced_program(const Base& b,unsigned m,const std::string& arm,const Reduced& reduced) {
    std::string code="// Public development unit; thread control is unavailable.\nSetSeed(1);\nSetColumns(256);\n";
    code+="F := GF("+P.get_str()+");\nassert #F eq "+P.get_str()+";\n";
    std::vector<std::string> variables;
    for(const auto& [name,value]:reduced.residual.witness) { (void)value; variables.push_back(name); }
    if(!variables.empty()) {
        code+="R<";
        for(size_t i=0;i<variables.size();++i) { if(i) code+=","; code+=variables[i]; }
        code+="> := PolynomialRing(F,"+num(variables.size())+",\"grevlex\");\nI := ideal<R | ";
        if(reduced.residual.equations.empty()) code+="R!0";
        for(size_t i=0;i<reduced.residual.equations.size();++i) {
            if(i) code+=",";
            code+="R!("+magma_poly2(reduced.residual.equations[i].second)+")";
        }
        code+=" >;\nG := GroebnerBasis(I);\nV := Variety(I);\n";
        code+="printf \"AFFINE_GB_MAX_DEGREE %o\\n\", Maximum([TotalDegree(g):g in G]);\n";
    } else {
        bool consistent=true; for(const auto& [name,p]:reduced.residual.equations) { (void)name; if(!p.terms.empty()) consistent=false; }
        code+=consistent?"V := [1];\n":"V := [];\n";
    }
    code+="printf \"REDUCED_VARIABLES "+num(variables.size())+"\\n\";\n";
    code+="printf \"BEGIN_SOLUTIONS "+b.fb_id+" m="+num(m)+" kind=planted\\n\";\nfor solution in V do\n  printf \"REL\\n\";\n";
    for(unsigned j=0;j<m;++j) {
        auto coordinate=[&](const std::string& suffix) {
            const Poly& p=reduced.reconstruction.at("p"+num(j)+suffix);
            return variables.empty()?"Integers()!(F!("+magma_poly2(p)+"))":"Integers()!Evaluate(R!("+magma_poly2(p)+"),[solution[k]:k in [1.."+num(variables.size())+"]])";
        };
        code+="  printf \"P %o %o\\n\", "+coordinate("_x")+", "+coordinate("_y")+";\n";
    }
    code+="  printf \"END_REL\\n\";\nend for;\nprintf \"COUNT %o\\n\", #V;\nprintf \"END_SOLUTIONS\\n\";\nquit;\n";
    (void)arm; return code;
}
std::string summary(const Base& b,unsigned m,const std::string& arm,const Reduced& r) {
    unsigned degree=0; for(const auto& [name,p]:r.residual.equations) { (void)name; degree=std::max(degree,p.degree()); }
    return object({{"arm",jquote(arm)},{"depth",num(b.depth)},{"factor_base",jquote(b.fb_id)},{"m",num(m)},
      {"original_variables",num(r.original_variables)},{"residual_variables",num(r.residual.witness.size())},
      {"original_equations",num(r.original_equations)},{"residual_equations",num(r.residual.equations.size())},
      {"original_input_degree",num(r.original_degree)},{"residual_input_degree",num(degree)},
      {"pivots",num(r.certificates.size())},{"structured_degree_of_regularity","null"},
      {"verification_standing",jquote("producer-unverified")}});
}
int main(int argc,char** argv) {
    try {
        require(argc==3 && std::string(argv[1])=="selftest","usage: linear_reduce selftest FRESH_OUTPUT_DIRECTORY");
        std::filesystem::path out=argv[2]; require(!std::filesystem::exists(out),"output must be fresh");
        std::vector<std::string> summaries;
        for(unsigned depth:{1U,2U,4U}) {
            Base b=build(depth); auto domain=signed_points(b);
            for(unsigned m=2;m<=7;++m) for(const std::string arm:{"dense","tower","paired","oriented"}) {
                auto indices=planted_indices(b,m); Point target=replay(domain,indices);
                System original=relation_system(b,m,arm,target,indices); original.check();
                Reduced r=reduce_linear(original); check_reconstruction(original,r);
                for(unsigned j=0;j<m;++j) {
                    Z x=r.reconstruction.at("p"+num(j)+"_x").eval(r.residual.witness),
                      y=r.reconstruction.at("p"+num(j)+"_y").eval(r.residual.witness);
                    require(point(x,y)==domain.at(indices[j]),"restored point differs");
                }
                summaries.push_back(summary(b,m,arm,r));
                std::string prefix="d"+num(depth)+"-m"+num(m)+"-"+arm;
                write(out/(prefix+"-certificate.json"),object({{"schema",jquote("p256.affine_linear_elimination/v1")},
                  {"summary",summary(b,m,arm,r)},{"unit_pivots",array(r.certificates)}}));
                if(depth<=2 && m==2) write(out/(prefix+".m"),reduced_program(b,m,arm,r));
            }
        }
        // Check the exact symbolic certificate, not only one evaluated point.
        Poly x=Poly::var("x"),y=Poly::var("y"); Pivot good{"x","synthetic",Z(2),Poly(3)*y-Poly(4)};
        Poly equation=Poly(2)*(x-good.expression); check_pivot(equation,good);
        Pivot corrupt=good; corrupt.expression=corrupt.expression+Poly(1);
        rejects([&]{check_pivot(equation,corrupt);},"corrupt pivot expression");
        corrupt=good; corrupt.coefficient=0;
        rejects([&]{check_pivot(equation,corrupt);},"zero/nonunit pivot");
        Poly before=cube(x)+x*y+Poly(5),after=substitute(before,"x",good.expression);
        check_substitution_identity(before,after,"x",good.expression);
        rejects([&]{check_substitution_identity(before,after+Poly(1),"x",good.expression);},"corrupt substitution remainder");
        write(out/"summary.json",object({{"schema",jquote("p256.affine_linear_elimination_selftest/v1")},
          {"cases",array(summaries)},{"symbolic_negative_controls","3"},{"performance_experiment","false"}}));
        std::cout<<"72 exact affine-elimination/reconstruction unit cases and3 corruption controls pass; no timing claim.\n";
        return 0;
    } catch(const std::exception& e) { std::cerr<<"linear_reduce: "<<e.what()<<'\n'; return 1; }
}
