// Exact point-ideal certificate by terminating division plus evaluation rank.
// No claim that an unexecuted S-pair loop passed.
#include "retained_prefix.inc"
std::string certify_dimension(const Basis& b,const Vars& vars,const Assignments& points,unsigned D) {
    const size_t n=points.size();require(n>0 && b.standard.size()==n,"dimension count");
    std::set<std::string> distinct;
    for(const auto& p:points){std::string s;for(const auto& v:vars)s+=p.at(v).get_str()+",";require(distinct.insert(s).second,"repeated point");}
    std::set<Monomial> footprint(b.standard.begin(),b.standard.end());require(footprint.size()==n,"repeated monomial");
    unsigned top=0;for(const auto& m:b.standard)top=std::max(top,md(m));require(D==top+1,"wrong regularity");
    Vars hvars=vars;hvars.push_back("H");std::vector<Poly> hg;std::vector<Monomial> leads;
    for(const auto& g:b.generators) {
        auto [m,c]=lt(g,vars);require(c==1 && md(m)==g.degree(),"nonmonic or nongraded leading term");
        for(const auto& p:points)require(g.eval(p)==0,"nonvanishing polynomial");
        Poly h=homogenize(g);require(lt(h,hvars)==std::make_pair(m,Z(1)),"leading term contains H");
        leads.push_back(m);hg.push_back(std::move(h));
    }
    size_t border_checks=0;
    for(const auto& m:b.standard) {
        for(const auto& lead:leads)require(!divmono(lead,m),"reducible footprint");
        for(const auto& v:vars) {
            if(ex(m,v))require(footprint.contains(mq(m,{{v,1}})),"not divisor closed");
            Monomial next=m;++next[v];bool covered=footprint.contains(next);
            for(const auto& lead:leads)covered=covered || divmono(lead,next);
            require(covered,"incomplete border");++border_checks;
        }
    }
    // All homogeneous divisions terminate and span by the footprint over K[H].
    // N independent point evaluations then prove freeness/exactness, not just
    // an upper bound on a guessed leading ideal. See protocol for the argument.
    size_t rank=independent_evaluation_rank(vars,points,b.standard);require(rank==n,"evaluation rank");
    std::vector<size_t> h(top+1);std::vector<std::string> ms,hs,vs,gs,hgs,ps;
    for(const auto& m:b.standard){++h[md(m)];ms.push_back(mt(m).json());}
    for(size_t x:h)hs.push_back(num(x));
    for(const auto& v:vars)vs.push_back(jquote(v));
    for(const auto& g:b.generators)gs.push_back(g.json());
    for(const auto& g:hg)hgs.push_back(g.json());
    for(const auto& p:points){Object row;for(const auto& [v,c]:p)row[v]=dec(c);ps.push_back(object(row));}
    return object({{"variables",array(vs)},{"points",array(ps)},{"standard_monomials",array(ms)},
      {"affine_generators",array(gs)},{"homogeneous_generators",array(hgs)},
      {"evaluation_rank",num(rank)},{"border_checks",num(border_checks)},
      {"hilbert_numerator_coefficients",array(hs)},{"hilbert_denominator",jquote("1-t")},
      {"quotient_module_regularity",num(top)},{"ideal_module_regularity",num(D)},
      {"H_free_leading_terms","true"},{"explicit_S_pairs_checked","false"},
      {"certificate_method",jquote("terminating graded border division + complete footprint + independent evaluation rank")},
      {"standing",jquote("unsigned producer; not Lean or independent signed verification")}});
}
int main(int argc,char** argv) {
    try {
        require(argc==3,"usage: retained_depth8 CACHE FRESH_OUT");std::filesystem::path out=argv[2];require(!std::filesystem::exists(out),"fresh output");
        std::istringstream in(read(argv[1]));std::string tag;size_t k,n;
        in>>tag>>k;require(tag=="VARS" && k==9,"vars");Vars vars;for(size_t i=0;i<k;++i){std::string v;in>>v;vars.push_back(v);}
        require(vars==Vars({"x","y","f1","f2","f3","f4","f5","f6","f7"}),"variable names");
        in>>tag>>n;require(tag=="POINTS" && n==288,"points");Assignments points;
        for(size_t i=0;i<n;++i){std::map<std::string,Z> p;for(const auto& v:vars){std::string s;in>>s;Z z(s);require(z>=0 && z<P,"canonical coordinate");p[v]=z;}points.push_back(std::move(p));}
        in>>tag>>k;require(tag=="STEPS" && k==8,"steps");std::vector<std::pair<Z,Z>> steps;
        for(size_t i=0;i<k;++i){std::string r,d;in>>r>>d;steps.emplace_back(Z(r),Z(d));}
        std::string terminal;in>>tag>>terminal;require(tag=="TERMINAL","terminal");Z terminal_x(terminal);
        in>>tag;require(tag=="END" && !(in>>tag),"cache trailing data");
        for(const auto& p:points) {
            require(TARGET.on(point(p.at("x"),p.at("y"))) && p.at("y")!=0,"target point");Z x=p.at("x");
            for(size_t i=0;i<8;++i){auto [r,d]=steps[i];require(mod(x-r)!=0 && mod(d)!=0,"chain pole/delta");x=mod(x+d*inv(x-r));if(i<7)require(x==p.at("f"+num(i+1)),"unique chain reconstruction");}
            require(x==terminal_x,"chain endpoint");
        }
        const auto before=counters;Basis b=buchberger_moeller(vars,points);unsigned D=0;for(const auto& m:b.standard)D=std::max(D,md(m)+1);
        const auto after_bm=counters;const std::string cert=certify_dimension(b,vars,points,D);const auto after_check=counters;
        size_t corruptions=0;auto reject=[&](const auto& f){bool rejected=false;try{f();}catch(const std::exception&){rejected=true;}require(rejected,"corrupt certificate accepted");++corruptions;};
        reject([&]{certify_dimension(b,vars,points,D+1);});
        Basis broken=b;broken.generators[0]+=Poly(1);reject([&]{certify_dimension(broken,vars,points,D);});
        broken=b;broken.generators.erase(broken.generators.begin());reject([&]{certify_dimension(broken,vars,points,D);});
        auto repeated=points;repeated.back()=repeated.front();reject([&]{certify_dimension(b,vars,repeated,D);});
        write(out/"certificate.json",cert);
        write(out/"manifest.json",object({{"factor_base",jquote("FB1h16882188194e")},{"signed_points",num(n)},
          {"affine_variables",num(vars.size())},{"generators",num(b.generators.size())},
          {"quotient_module_regularity",num(D-1)},{"ideal_module_regularity",num(D)},
          {"cache_sha256",jquote(hash(read(argv[1])))},{"certificate_sha256",jquote(hash(cert+"\n"))},
          {"corrupt_certificates_rejected",num(corruptions)},
          {"BM_modular_reduction_API_calls",num(after_bm.reductions-before.reductions)},
          {"verification_modular_reduction_API_calls",num(after_check.reductions-after_bm.reductions)},
          {"counter_scope",jquote("GMP API calls; construction/check separate; no solve-time equivalence")},
          {"full_system_degree_bound_reviewed","false"},{"goal_achieved","false"}}));
        std::cout<<"Retained depth8 point ideal D="<<D<<"; full-bound proof review pending.\n";
    }catch(const std::exception& e){std::cerr<<"retained_depth8: "<<e.what()<<'\n';return 1;}
}
