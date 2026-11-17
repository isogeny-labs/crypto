// Exact proof-development component. Not a complete relation regularity result.
#define main frozen_auxiliary_main
#include "../aux_base.cpp"
#undef main

using Variables=std::vector<std::string>;
unsigned exponent(const Monomial& m,const std::string& v) {
    auto it=m.find(v); return it==m.end()?0:it->second;
}
bool lex_less(const Monomial& a,const Monomial& b,const Variables& variables) {
    for(const auto& v:variables) {
        auto x=exponent(a,v), y=exponent(b,v); if(x!=y) return x<y;
    }
    return false;
}
std::pair<Monomial,Z> leading(const Poly& p,const Variables& vars) {
    require(!p.terms.empty(),"leading term of zero");
    auto result=*p.terms.begin();
    for(const auto& term:p.terms) if(lex_less(result.first,term.first,vars)) result=term;
    return result;
}
Poly term(const Monomial& m,const Z& c) { Poly p; if(mod(c)!=0) p.terms[m]=mod(c); return p; }
bool divides(const Monomial& a,const Monomial& b) {
    for(const auto& [v,e]:a) if(exponent(b,v)<e) return false;
    return true;
}
Monomial quotient(Monomial b,const Monomial& a) {
    require(divides(a,b),"nondividing monomials");
    for(const auto& [v,e]:a) { b[v]-=e; if(b[v]==0) b.erase(v); } return b;
}
bool homogeneous(const Poly& p,unsigned degree) {
    if(p.terms.empty()) return false;
    for(const auto& [m,c]:p.terms) {
        (void)c; unsigned d=0; for(const auto& [v,e]:m) { (void)v; d+=e; }
        if(d!=degree) return false;
    }
    return true;
}
struct Division { std::vector<Poly> multipliers; Poly remainder; };
Division divide(Poly p,const std::vector<Poly>& generators,const Variables& vars) {
    const Poly original=p; Division out{std::vector<Poly>(generators.size()),Poly()};
    unsigned steps=0;
    while(!p.terms.empty()) {
        require(++steps<=100000,"exact division cap; no partial certificate");
        auto [m,c]=leading(p,vars); bool reduced=false;
        for(size_t i=0;i<generators.size();++i) {
            auto [n,a]=leading(generators[i],vars);
            if(divides(n,m)) {
                Poly q=term(quotient(m,n),mod(c*inv(a)));
                out.multipliers[i]+=q; p=p-q*generators[i]; reduced=true; break;
            }
        }
        if(!reduced) { Poly t=term(m,c); out.remainder+=t; p=p-t; }
    }
    Poly recovered=out.remainder;
    for(size_t i=0;i<generators.size();++i) recovered+=out.multipliers[i]*generators[i];
    require((original-recovered).terms.empty(),"division coefficient identity"); return out;
}
unsigned certificate_checks=0;
std::string certify_tower(const std::vector<Poly>& generators,const Variables& vars,unsigned asserted_D) {
    const unsigned d=static_cast<unsigned>(generators.size());
    require(d>0 && vars.size()==d+1 && vars.back()=="H","tower ring");
    require(asserted_D==d+1,"wrong ideal regularity assertion");
    for(unsigned i=0;i<d;++i) {
        require(homogeneous(generators[i],2),"tower generator not homogeneous quadratic");
        require(leading(generators[i],vars)==std::make_pair(Monomial{{vars[i],2}},Z(1)),"tower leading square");
    }
    std::vector<std::string> pairs;
    for(unsigned i=0;i<d;++i) for(unsigned j=i+1;j<d;++j) {
        Poly S=sq(Poly::var(vars[j]))*generators[i]-sq(Poly::var(vars[i]))*generators[j];
        auto reduced=divide(S,generators,vars);
        require(reduced.remainder.terms.empty(),"nonzero S remainder");
        std::vector<std::string> qs; for(const auto& q:reduced.multipliers) qs.push_back(q.json());
        pairs.push_back(object({{"i",num(i)},{"j",num(j)},{"multipliers",array(qs)},
          {"s_polynomial",S.json()},{"remainder",reduced.remainder.json()}}));
    }
    ++certificate_checks; return array(pairs);
}
std::vector<Poly> tower_polynomials(const Base& b,Variables& vars) {
    for(unsigned i=0;i<b.depth;++i) vars.push_back("X"+num(i)); vars.push_back("H");
    Poly H=Poly::var("H"); std::vector<Poly> gs;
    for(unsigned i=0;i<b.depth;++i) {
        const auto& s=b.chain[i]; require(s.delta!=0,"zero isogeny delta");
        Poly x=Poly::var(vars[i]);
        Poly next=i+1==b.depth?Poly(b.terminal.x)*H:Poly::var(vars[i+1]);
        gs.push_back(-((next-x)*(x-Poly(s.r)*H)-Poly(s.delta)*sq(H)));
    }
    return gs;
}
std::string koszul(const std::vector<unsigned>& degrees) {
    // Minimal shifts of a certified homogeneous complete intersection.
    // This routine does not assert that arbitrary supplied polynomials are CI.
    std::map<std::pair<unsigned,unsigned>,Z> betti{{{0,0},1}};
    std::vector<Z> numerator{1}; unsigned qreg=0;
    for(unsigned degree:degrees) {
        require(degree>0,"zero generator degree"); auto old=betti;
        for(const auto& [ij,count]:old) betti[{ij.first+1,ij.second+degree}]+=count;
        std::vector<Z> updated(numerator.size()+degree-1);
        for(size_t i=0;i<numerator.size();++i) for(unsigned j=0;j<degree;++j) updated[i+j]+=numerator[i];
        numerator=std::move(updated); qreg+=degree-1;
    }
    std::vector<std::string> entries,coeffs,ds;
    unsigned from_shifts=0;
    for(const auto& [ij,count]:betti) {
        from_shifts=std::max(from_shifts,ij.second-ij.first);
        entries.push_back(object({{"homological_degree",num(ij.first)},{"internal_degree",num(ij.second)},{"rank",dec(count)}}));
    }
    require(from_shifts==qreg && numerator.size()==qreg+1,"Koszul/Hilbert degree disagreement");
    for(const auto& c:numerator) coeffs.push_back(dec(c));
    for(unsigned d:degrees) ds.push_back(num(d));
    return object({{"betti_shifts",array(entries)},{"generator_degrees",array(ds)},
      {"hilbert_numerator_coefficients",array(coeffs)},
      {"ideal_module_regularity",num(qreg+1)},{"quotient_module_regularity",num(qreg)},
      {"hilbert_denominator",jquote("1-t")}});
}
void expect_rejection(const std::string& name,const std::function<void()>& test,std::vector<std::string>& controls) {
    bool rejected=false; try { test(); } catch(const std::exception&) { rejected=true; }
    require(rejected,"corrupt certificate accepted: "+name); controls.push_back(jquote(name));
}
std::string fixture_certificate(unsigned depth,std::vector<std::string>& controls) {
    Base b=build(depth); crosscheck(b); Variables vars;
    auto generators=tower_polynomials(b,vars); std::string pairs=certify_tower(generators,vars,depth+1);
    expect_rejection("wrong-regularity-depth"+num(depth),[&]{certify_tower(generators,vars,depth);},controls);
    auto corrupt=generators; corrupt[0]+=Poly::var("H");
    expect_rejection("nonhomogeneous-depth"+num(depth),[&]{certify_tower(corrupt,vars,depth+1);},controls);
    if(depth>1) {
        corrupt=generators; corrupt[1]=generators[0];
        expect_rejection("repeated-leading-square-depth"+num(depth),[&]{certify_tower(corrupt,vars,depth+1);},controls);
    }
    auto bad_base=b; bad_base.chain[0].delta=0;
    expect_rejection("zero-delta-depth"+num(depth),[&]{Variables v; tower_polynomials(bad_base,v);},controls);
    // Exact numerator/denominator composition; all arithmetic in actual F_p.
    Poly X=Poly::var("x"), u=X,v(1);
    for(const auto& s:b.chain) {
        Poly next_u=sq(u)-Poly(s.r)*u*v+Poly(s.delta)*sq(v);
        Poly next_v=v*(u-Poly(s.r)*v); u=next_u; v=next_v;
    }
    Poly full=u-Poly(b.terminal.x)*v, product(1),retained(1),excluded(1);
    std::set<Z> roots; std::map<Z,Point> selected;
    for(const auto& c:b.columns) { selected[c.target.x]=c.target; retained=retained*(X-Poly(c.target.x)); }
    Point q=b.coset; std::vector<std::string> root_records;
    for(size_t i=0;i<(size_t(1)<<depth);++i) {
        require(!q.infinity && roots.insert(q.x).second,"repeated full-fibre root");
        const std::map<std::string,Z> assignment{{"x",q.x}};
        require(full.eval(assignment)==0 && v.eval(assignment)!=0,"full-fibre composition/pole");
        Z rhs=mod(q.x*q.x*q.x-3*q.x+B),character=power(rhs,(P-1)/2);
        require(rhs!=0,"target cubic vanishes at fibre root; Y section invalid");
        bool kept=selected.contains(q.x);
        require(character==(kept?Z(1):Z(P-1)),"retained-lift/character disagreement");
        if(!kept) excluded=excluded*(X-Poly(q.x));
        product=product*(X-Poly(q.x));
        root_records.push_back(object({{"x",dec(q.x)},{"retained",kept?"true":"false"},
          {"target_quadratic_character",kept?"1":"-1"}}));
        q=AUX.add(q,b.kernel);
    }
    require(q==b.coset && (full-product).terms.empty(),"composition is not exact full-fibre product");
    require((retained*excluded-full).terms.empty(),"retained/excluded factor identity");
    require(full.degree()==(1U<<depth) && retained.degree()==b.columns.size(),"dense degree");
    std::vector<std::string> polynomials,names;
    for(const auto& p:generators) polynomials.push_back(p.json());
    for(const auto& s:vars) names.push_back(jquote(s));
    std::vector<unsigned> tower_degrees(depth,2); auto curve_degrees=tower_degrees; curve_degrees.push_back(3);
    unsigned D=1U<<depth, selected_D=static_cast<unsigned>(b.columns.size());
    return object({{"schema",jquote("p256.membership_regularity_certificate/v1")},
      {"curve",jquote(SLUG)},{"factor_base",jquote(b.fb_id)},{"depth",num(depth)},
      {"modulus",dec(P)},{"full_abscissae",num(D)},{"retained_abscissae",num(selected_D)},
      {"lex_variables",array(names)},{"homogeneous_tower_generators",array(polynomials)},
      {"s_pair_identities",pairs},{"full_fibre_polynomial",full.json()},
      {"composition_denominator",v.json()},{"retained_polynomial",retained.json()},
      {"excluded_polynomial",excluded.json()},{"roots",array(root_records)},
      {"full_fibre_membership_CI",koszul({D})},{"retained_membership_CI",koszul({selected_D})},
      {"tower_membership_CI",koszul(tower_degrees)},
      {"full_fibre_with_target_cubic_CI",koszul({D,3})},
      {"retained_with_target_cubic_CI",koszul({selected_D,3})},
      {"tower_with_target_cubic_CI",koszul(curve_degrees)},
      {"membership_regular_section",jquote("H")},{"target_cubic_regular_section",jquote("Y; verified nonzero target RHS at every full-fibre root")},
      {"full_fibre_component_twofold",2*(depth+1)<=D?"true":"false"},
      {"retained_component_numeric_twofold",2*(depth+1)<=selected_D?"true":"false"},
      {"full_relation_regularity","null"},{"goal_achieved","false"},
      {"relations_reported","0"},{"standing",jquote("unsigned-producer; see PROOF.md for hypotheses and height argument")}});
}
int main(int argc,char** argv) {
    try {
        require(argc==3 && std::string(argv[1])=="selftest","usage: membership_certificate selftest FRESH_DIR");
        std::filesystem::path out=argv[2]; require(!std::filesystem::exists(out),"fresh output required");
        std::vector<std::string> cases,controls;
        for(unsigned d:{1U,2U,4U}) {
            std::string cert=fixture_certificate(d,controls); write(out/("depth"+num(d)+".json"),cert);
            cases.push_back(object({{"depth",num(d)},{"certificate_sha256",jquote(hash(cert))}}));
        }
        write(out/"summary.json",object({{"cases",array(cases)},{"positive_certificates",num(certificate_checks)},
          {"corrupt_certificates_rejected",array(controls)},{"goal_achieved","false"},
          {"scope",jquote("native exact component proof checks; no complete relation regularity or timing measurement")}}));
        std::cout<<"Membership certificates and corruption controls pass; complete relation goal open.\n";
        return 0;
    } catch(const std::exception& e) { std::cerr<<"membership_certificate: "<<e.what()<<'\n'; return 1; }
}
