// Serialization/witness integration only. No Groebner or relation search.
#include "frozen_linear_prefix.inc"
struct CachedBasis { std::vector<std::string> vars; size_t points=0; std::vector<Poly> gs; };
CachedBasis read_cached(const std::filesystem::path& file) {
    std::istringstream in(read(file)); std::string tag; size_t n=0; CachedBasis b;
    in>>tag>>n; require(tag=="VARS" && n>=2 && n<=20,"cached variables header");
    for(size_t i=0;i<n;++i) { std::string v; in>>v; b.vars.push_back(v); }
    in>>tag>>b.points; require(tag=="POINTS" && b.points>0 && b.points<=1000,"cached points header");
    in>>tag>>n; require(tag=="GENERATORS" && n>0 && n<=1000,"cached generator header");
    for(size_t i=0;i<n;++i) {
        size_t terms=0; in>>tag>>terms; require(tag=="POLYNOMIAL" && terms>0 && terms<=10000,"cached polynomial header"); Poly p;
        for(size_t j=0;j<terms;++j) {
            std::string cs; size_t powers=0; in>>tag>>cs>>powers;
            require(tag=="TERM" && !cs.empty() && cs.find_first_not_of("0123456789")==std::string::npos && powers<=20,"cached term");
            Z c(cs); require(c>0 && c<P && c.get_str()==cs,"canonical cached coefficient"); Monomial m;
            for(size_t k=0;k<powers;++k) {
                std::string v; unsigned e=0; in>>v>>e;
                require(std::find(b.vars.begin(),b.vars.end(),v)!=b.vars.end() && e>0 && e<=1000 && !m.contains(v),"cached exponent"); m[v]=e;
            }
            require(p.terms.emplace(m,c).second,"duplicate cached term");
        }
        b.gs.push_back(std::move(p));
    }
    in>>tag; require(tag=="END","cached terminator");
    require(!(in>>tag),"trailing cached input"); return b;
}
Poly point_rename(const Poly& p,const std::string& prefix) {
    Poly out;
    for(const auto& [m,c]:p.terms) {
        Monomial renamed;
        for(const auto& [v,e]:m) {
            std::string name;
            if(v=="x" || v=="y") name=prefix+"_"+v;
            else { require(v.size()>1 && v[0]=='f',"unknown point coordinate"); name=prefix+"_fibre_"+v.substr(1); }
            require(renamed.emplace(name,e).second,"coordinate rename collision");
        }
        out.terms[renamed]=c;
    }
    return out;
}
System bridged_system(const Base& b,unsigned m,bool structured,const CachedBasis& cache,const Point& target,const std::vector<size_t>& indices) {
    std::vector<std::string> expected{"x","y"};
    if(structured) for(unsigned k=1;k<b.depth;++k) expected.push_back("f"+num(k));
    require(cache.vars==expected && cache.points==2*b.columns.size(),"wrong membership cache identity");
    System s=relation_system(b,m,"dense",target,indices); std::set<std::string> replaced;
    for(unsigned j=0;j<m;++j) { replaced.insert("p"+num(j)+"_target_curve"); replaced.insert("p"+num(j)+"_membership"); }
    size_t before=s.equations.size();
    std::erase_if(s.equations,[&](const auto& e){return replaced.contains(e.first);});
    require(before-s.equations.size()==2*m,"unexpected point equation replacement");
    for(unsigned j=0;j<m;++j) {
        std::string prefix="p"+num(j);
        if(structured) {
            Point q=b.columns.at(indices[j]/2).auxiliary;
            for(unsigned k=0;k<b.depth;++k) {
                q=b.chain[k].apply(q);
                if(k+1<b.depth) s.variable(prefix+"_fibre_"+num(k+1),q.x);
            }
            require(q==b.terminal,"cached lift terminal");
        }
        for(size_t k=0;k<cache.gs.size();++k) s.eq(prefix+"_retained_gb_"+num(k),point_rename(cache.gs[k],prefix));
    }
    return s;
}
int main(int argc,char** argv) {
    try {
        require(argc==4 && std::string(argv[1])=="export","usage: retained_bridge export CACHED_TEXT_DIR FRESH_DIR");
        std::filesystem::path cache_dir=argv[2],out=argv[3]; require(!std::filesystem::exists(out),"fresh output required");
        std::vector<std::string> cells; size_t replays=0,planted_checks=0;
        for(unsigned d:{4U,5U}) {
            Base b=build(d); crosscheck(b); auto domain=signed_points(b);
            write(out/("depth"+num(d)+"-base-certificate.json"),b.certificate);
            CachedBasis dense=read_cached(cache_dir/("depth"+num(d)+"-signed-dense.txt"));
            CachedBasis structured=read_cached(cache_dir/("depth"+num(d)+"-signed-structured.txt"));
            for(unsigned m=2;m<=7;++m) {
                auto indices=planted_indices(b,m); Point planted=replay(domain,indices),natural=natural_target(b,m);
                OpenSSLCheck independent(TARGET); Point acc;
                for(size_t i:indices) { Point next=TARGET.add(acc,domain.at(i)); independent.add(acc,domain.at(i),next); acc=next; }
                require(acc==planted,"planted target replay"); ++replays;
                for(const std::string kind:{"planted","unplanted"}) for(bool chain:{false,true}) {
                    const std::string arm=chain?"RB-chain":"RB-dense"; Point target=kind=="planted"?planted:natural;
                    Counters setup_before=counters;
                    System original=bridged_system(b,m,chain,chain?structured:dense,target,indices);
                    if(kind=="planted") { original.check(); ++planted_checks; }
                    Reduced reduced=reduce_linear(original);
                    if(kind=="planted") check_reconstruction(original,reduced);
                    Counters setup_after=counters;
                    size_t original_terms=0,residual_terms=0;
                    unsigned original_degree=0,residual_degree=0;
                    for(const auto& e:original.equations) { original_terms+=e.second.terms.size(); original_degree=std::max(original_degree,e.second.degree()); }
                    for(const auto& e:reduced.residual.equations) { residual_terms+=e.second.terms.size(); residual_degree=std::max(residual_degree,e.second.degree()); }
                    const auto raw=system_json(b,m,arm,kind,target,original)+"\n";
                    const auto residual=system_json(b,m,arm,kind,target,reduced.residual)+"\n";
                    const auto pivots=array(reduced.certificates)+"\n";
                    const std::string stem="depth"+num(d)+"-m"+num(m)+"-"+arm+"-"+kind;
                    if(m==2) {
                        write(out/(stem+"-original.json"),raw.substr(0,raw.size()-1));
                        write(out/(stem+"-residual.json"),residual.substr(0,residual.size()-1));
                        write(out/(stem+"-pivots.json"),pivots.substr(0,pivots.size()-1));
                    }
                    cells.push_back(object({{"cell",jquote(stem)},{"factor_base",jquote(b.fb_id)},
                      {"target",point_json(target)},{"depth",num(d)},{"m",num(m)},{"arm",jquote(arm)},
                      {"kind",jquote(kind)},{"original_variables",num(original.witness.size())},
                      {"residual_variables",num(reduced.residual.witness.size())},{"original_equations",num(original.equations.size())},
                      {"residual_equations",num(reduced.residual.equations.size())},
                      {"original_terms",num(original_terms)},{"residual_terms",num(residual_terms)},
                      {"original_input_degree",num(original_degree)},{"residual_input_degree",num(residual_degree)},
                      {"setup_validation_modular_reduction_API_calls",num(setup_after.reductions-setup_before.reductions)},
                      {"setup_validation_inversion_API_calls",num(setup_after.inversions-setup_before.inversions)},
                      {"counter_scope",jquote("GMP construction/preprocessing/witness API calls, not field-equivalent solving cost; common base build and planted OpenSSL replay separate")},
                      {"original_sha256",jquote(hash(raw))},{"residual_sha256",jquote(hash(residual))},
                      {"pivot_certificate_sha256",jquote(hash(pivots))},{"materialized",m==2?"true":"false"},
                      {"full_relation_regularity","null"},{"solver_status",jquote("not-run")}}));
                }
            }
        }
        write(out/"manifest.json",object({{"cells",array(cells)},{"planted_group_replays",num(replays)},
          {"planted_system_witness_checks",num(planted_checks)},{"complete_relation_degree_certificates","0"},
          {"unplanted_relations_found","null"},{"goal_achieved","false"}}));
        std::cout<<"48 matched exports; 12 exact planted group replays; no full-system degree or solve.\n"; return 0;
    } catch(const std::exception& e) { std::cerr<<"retained_bridge: "<<e.what()<<'\n'; return 1; }
}
