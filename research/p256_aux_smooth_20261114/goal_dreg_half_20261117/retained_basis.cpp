// Exact retained-point ideals, not a complete relation-regularity result.
#define main frozen_auxiliary_main
#include "../aux_base.cpp"
#undef main

using Vars=std::vector<std::string>;
using Assignments=std::vector<std::map<std::string,Z>>;
unsigned md(const Monomial& m) {
    unsigned d=0;
    for(const auto& [v,e]:m) { (void)v; d+=e; }
    return d;
}
unsigned ex(const Monomial& m,const std::string& v) {
    auto i=m.find(v); return i==m.end()?0:i->second;
}
struct Grevlex {
    Vars vars;
    bool operator()(const Monomial& a,const Monomial& b) const {
        if(md(a)!=md(b)) return md(a)<md(b);
        for(auto i=vars.rbegin();i!=vars.rend();++i)
            if(ex(a,*i)!=ex(b,*i)) return ex(a,*i)>ex(b,*i);
        return false;
    }
};
std::pair<Monomial,Z> lt(const Poly& p,const Vars& vars) {
    require(!p.terms.empty(),"zero leading term");
    std::pair<Monomial,Z> result=*p.terms.begin(); Grevlex less{vars};
    for(const auto& entry:p.terms) if(less(result.first,entry.first)) result=entry;
    return result;
}
Poly mt(const Monomial& m,const Z& c=1) {
    Poly p; if(mod(c)!=0) p.terms[m]=mod(c); return p;
}
bool divmono(const Monomial& a,const Monomial& b) {
    for(const auto& [v,e]:a) if(ex(b,v)<e) return false;
    return true;
}
Monomial mq(Monomial b,const Monomial& a) {
    require(divmono(a,b),"invalid monomial quotient");
    for(const auto& [v,e]:a) { b[v]-=e; if(!b[v]) b.erase(v); }
    return b;
}
bool coprime(const Monomial& a,const Monomial& b) {
    for(const auto& [v,e]:a) if(e && ex(b,v)) return false;
    return true;
}
struct Basis { std::vector<Monomial> standard; std::vector<Poly> generators; };
struct Row { size_t pivot; std::vector<Z> values,coefficients; };
Basis buchberger_moeller(const Vars& vars,const Assignments& points) {
    require(!points.empty(),"empty point set");
    std::set<std::string> distinct;
    for(const auto& p:points) {
        std::string key;
        for(const auto& v:vars) key+=p.at(v).get_str()+",";
        require(distinct.insert(key).second,"repeated input point");
    }
    const size_t N=points.size(); Basis out; std::vector<Row> rows;
    std::set<Monomial,Grevlex> todo(Grevlex{vars}); todo.insert(Monomial{});
    size_t processed=0;
    while(!todo.empty()) {
        require(++processed<=10000,"BM monomial cap; no partial certificate");
        Monomial m=*todo.begin(); todo.erase(todo.begin()); bool redundant=false;
        for(const auto& g:out.generators) if(divmono(lt(g,vars).first,m)) { redundant=true; break; }
        if(redundant) continue;
        std::vector<Z> value,coeff(N);
        for(const auto& p:points) value.push_back(mt(m).eval(p));
        for(const auto& row:rows) {
            Z factor=value[row.pivot]; if(factor==0) continue;
            for(size_t k=0;k<N;++k) {
                value[k]=mod(value[k]-factor*row.values[k]);
                coeff[k]=mod(coeff[k]+factor*row.coefficients[k]);
            }
        }
        size_t pivot=0; while(pivot<N && value[pivot]==0) ++pivot;
        if(pivot==N) {
            Poly g=mt(m);
            for(size_t k=0;k<out.standard.size();++k) g=g-mt(out.standard[k],coeff[k]);
            require(lt(g,vars)==std::make_pair(m,Z(1)),"BM leading term");
            out.generators.push_back(std::move(g));
        } else {
            require(out.standard.size()<N,"BM rank overflow");
            Z scale=inv(value[pivot]);
            for(size_t k=0;k<N;++k) { value[k]=mod(value[k]*scale); coeff[k]=mod(-coeff[k]*scale); }
            coeff[out.standard.size()]=scale;
            Row row{pivot,std::move(value),std::move(coeff)};
            rows.insert(std::lower_bound(rows.begin(),rows.end(),pivot,[](const Row& r,size_t p){return r.pivot<p;}),std::move(row));
            out.standard.push_back(m);
            for(const auto& v:vars) { Monomial next=m; ++next[v]; todo.insert(std::move(next)); }
        }
    }
    require(out.standard.size()==N,"BM did not span point functions"); return out;
}
Poly homogenize(const Poly& f) {
    Poly g;
    for(const auto& [m,c]:f.terms) {
        require(!m.contains("H"),"reserved H in affine input");
        Monomial n=m; if(f.degree()>md(m)) n["H"]=f.degree()-md(m); g.terms[n]=c;
    }
    return g;
}
struct Reduction { std::vector<Poly> multipliers; Poly remainder; size_t steps=0; };
Reduction normal_form(Poly f,const std::vector<Poly>& gs,const Vars& vars) {
    const Poly original=f; Reduction result{std::vector<Poly>(gs.size()),Poly(),0};
    while(!f.terms.empty()) {
        require(++result.steps<=1000000,"S-pair reduction cap; no partial certificate");
        auto [m,c]=lt(f,vars); bool reduced=false;
        for(size_t j=0;j<gs.size();++j) {
            auto [n,a]=lt(gs[j],vars); require(a==1,"nonmonic generator");
            if(divmono(n,m)) {
                Poly q=mt(mq(m,n),c); result.multipliers[j]+=q; f=f-q*gs[j]; reduced=true; break;
            }
        }
        if(!reduced) { Poly t=mt(m,c); result.remainder+=t; f=f-t; }
    }
    Poly reconstruction=result.remainder;
    for(size_t j=0;j<gs.size();++j) reconstruction+=result.multipliers[j]*gs[j];
    require((original-reconstruction).terms.empty(),"division coefficient identity"); return result;
}
struct Checks { size_t pairs=0,product_pairs=0,steps=0; };
Checks check_groebner(const std::vector<Poly>& gs,const Vars& vars) {
    Checks out;
    for(size_t i=0;i<gs.size();++i) for(size_t j=i+1;j<gs.size();++j) {
        const auto [a,ac]=lt(gs[i],vars); const auto [b,bc]=lt(gs[j],vars);
        require(ac==1 && bc==1,"nonmonic basis");
        ++out.pairs;
        if(coprime(a,b)) { ++out.product_pairs; continue; }
        Monomial lcm=a;
        for(const auto& [v,e]:b) lcm[v]=std::max(ex(lcm,v),e);
        Poly S=mt(mq(lcm,a))*gs[i]-mt(mq(lcm,b))*gs[j];
        auto r=normal_form(S,gs,vars); out.steps+=r.steps;
        require(r.remainder.terms.empty(),"nonzero S-pair remainder");
    }
    return out;
}
size_t independent_evaluation_rank(const Vars& vars,const Assignments& points,const std::vector<Monomial>& standard) {
    // Independent Gauss-Jordan pass, not the BM's incremental row routine.
    std::vector<std::vector<Z>> matrix(points.size(),std::vector<Z>(standard.size()));
    for(size_t i=0;i<points.size();++i) for(size_t j=0;j<standard.size();++j) matrix[i][j]=mt(standard[j]).eval(points[i]);
    size_t rank=0;
    for(size_t col=0;col<standard.size() && rank<points.size();++col) {
        size_t pivot=rank; while(pivot<points.size() && matrix[pivot][col]==0) ++pivot;
        if(pivot==points.size()) continue;
        std::swap(matrix[pivot],matrix[rank]); Z scale=inv(matrix[rank][col]);
        for(size_t j=col;j<standard.size();++j) matrix[rank][j]=mod(matrix[rank][j]*scale);
        for(size_t i=0;i<points.size();++i) if(i!=rank && matrix[i][col]!=0) {
            Z factor=matrix[i][col];
            for(size_t j=col;j<standard.size();++j) matrix[i][j]=mod(matrix[i][j]-factor*matrix[rank][j]);
        }
        ++rank;
    }
    (void)vars; return rank;
}
std::string check_basis(const Basis& basis,const Vars& vars,const Assignments& points,unsigned asserted_D) {
    require(basis.standard.size()==points.size() && !points.empty(),"point/footprint count");
    std::set<Monomial> footprint(basis.standard.begin(),basis.standard.end());
    require(footprint.size()==points.size(),"duplicate footprint monomial");
    unsigned top=0; std::vector<size_t> hilbert;
    std::vector<Poly> hg; Vars hvars=vars; hvars.push_back("H");
    for(const auto& g:basis.generators) {
        for(const auto& p:points) require(g.eval(p)==0,"generator does not vanish");
        auto [m,c]=lt(g,vars); require(c==1 && md(m)==g.degree(),"degree-compatible leading term");
        Poly h=homogenize(g); require(lt(h,hvars)==std::make_pair(m,Z(1)),"homogeneous leading term contains H"); hg.push_back(std::move(h));
    }
    for(const auto& m:basis.standard) {
        top=std::max(top,md(m));
        if(hilbert.size()<=md(m)) hilbert.resize(md(m)+1);
        ++hilbert[md(m)];
        for(const auto& g:basis.generators) require(!divmono(lt(g,vars).first,m),"footprint reducible");
        for(const auto& v:vars) {
            if(ex(m,v)>0) require(footprint.contains(mq(m,Monomial{{v,1}})),"footprint not divisor closed");
            Monomial border=m; ++border[v];
            bool covered=footprint.contains(border);
            for(const auto& g:basis.generators) covered=covered || divmono(lt(g,vars).first,border);
            require(covered,"uncovered standard border");
        }
    }
    require(asserted_D==top+1,"incorrect exact regularity assertion");
    const auto affine=check_groebner(basis.generators,vars),homogeneous=check_groebner(hg,hvars);
    require(independent_evaluation_rank(vars,points,basis.standard)==points.size(),"singular evaluation map");
    std::vector<std::string> polys,hpolys,ms,hs,ps,vs;
    for(const auto& p:basis.generators) polys.push_back(p.json());
    for(const auto& p:hg) hpolys.push_back(p.json());
    for(const auto& m:basis.standard) ms.push_back(mt(m).json());
    for(size_t n:hilbert) hs.push_back(num(n));
    for(const auto& p:points) { Object q; for(const auto& [v,c]:p) q[v]=dec(c); ps.push_back(object(q)); }
    for(const auto& v:vars) vs.push_back(jquote(v));
    return object({{"variables",array(vs)},{"points",array(ps)},{"affine_generators",array(polys)},
      {"homogeneous_generators",array(hpolys)},{"standard_monomials",array(ms)},
      {"hilbert_numerator_coefficients",array(hs)},{"hilbert_denominator",jquote("1-t")},
      {"ideal_module_regularity",num(top+1)},{"quotient_module_regularity",num(top)},
      {"evaluation_rank",num(points.size())},{"affine_s_pairs",num(affine.pairs)},
      {"product_criterion_pairs",num(affine.product_pairs)},{"homogeneous_s_pairs",num(homogeneous.pairs)},
      {"division_steps",num(affine.steps+homogeneous.steps)},
      {"scope",jquote("exact retained membership ideal; full relation regularity unmeasured")},
      {"standing",jquote("unsigned producer certificate; check all S-pairs/rank/H-free footprint")}});
}
int main(int argc,char** argv) {
    try {
        require(argc==3 && std::string(argv[1])=="selftest","usage: retained_basis selftest FRESH_DIR");
        std::filesystem::path out=argv[2]; require(!std::filesystem::exists(out),"fresh output required");
        std::vector<std::string> rows; size_t corruptions=0;
        for(unsigned d:{2U,4U,5U}) {
            Base b=build(d); crosscheck(b);
            if(d==5) require(b.fb_id=="FB1h8bc0ab2ed2af","existing depth5 identity changed");
            write(out/("depth"+num(d)+"-factor-base.json"),b.dump);
            for(bool signed_points:{false,true}) for(bool structured:{false,true}) {
                Vars vars{"x"}; if(signed_points) vars.push_back("y");
                if(structured) for(unsigned k=1;k<d;++k) vars.push_back("f"+num(k));
                Assignments points;
                for(const auto& column:b.columns) for(unsigned sign=0;sign<(signed_points?2U:1U);++sign) {
                    std::map<std::string,Z> p{{"x",column.target.x}};
                    if(signed_points) p["y"]=sign?mod(-column.target.y):column.target.y;
                    Point q=column.auxiliary;
                    for(unsigned k=0;k<d;++k) {
                        require(q.x!=b.chain[k].r,"selected chain pole"); q=b.chain[k].apply(q);
                        if(structured && k+1<d) p["f"+num(k+1)]=q.x;
                    }
                    require(q==b.terminal,"selected terminal mismatch"); points.push_back(std::move(p));
                }
                Counters before=counters; Basis basis=buchberger_moeller(vars,points);
                Counters after_construction=counters;
                unsigned D=0; for(const auto& m:basis.standard) D=std::max(D,md(m)+1);
                const auto cert=check_basis(basis,vars,points,D);
                Counters after_verification=counters;
                auto reject=[&](const auto& test) {
                    bool rejected=false; try {test();} catch(const std::exception&) {rejected=true;}
                    require(rejected,"corrupted certificate accepted"); ++corruptions;
                };
                reject([&]{check_basis(basis,vars,points,D+1);});
                Basis broken=basis; broken.generators.at(0)+=Poly(1);
                reject([&]{check_basis(broken,vars,points,D);});
                Assignments repeated=points; repeated.push_back(points.front());
                reject([&]{buchberger_moeller(vars,repeated);});
                std::string cell="depth"+num(d)+(signed_points?"-signed":"-abscissa")+(structured?"-structured":"-dense");
                write(out/(cell+".json"),cert);
                rows.push_back(object({{"cell",jquote(cell)},{"factor_base",jquote(b.fb_id)},
                  {"points",num(points.size())},{"affine_variables",num(vars.size())},
                  {"generators",num(basis.generators.size())},{"ideal_regularity",num(D)},
                  {"certificate_sha256",jquote(hash(cert+"\n"))},
                  {"construction_modular_reduction_API_calls",num(after_construction.reductions-before.reductions)},
                  {"construction_inversion_API_calls",num(after_construction.inversions-before.inversions)},
                  {"verification_modular_reduction_API_calls",num(after_verification.reductions-after_construction.reductions)},
                  {"modular_reduction_API_calls",num(counters.reductions-before.reductions)},
                  {"inversion_API_calls",num(counters.inversions-before.inversions)},
                  {"counter_scope",jquote("GMP API calls; total includes construction, verification and corrupt controls, not field-equivalent solve cost")}}));
            }
        }
        write(out/"summary.json",object({{"cells",array(rows)},{"corrupt_certificates_rejected",num(corruptions)},
          {"full_relation_regularity","null"},{"goal_achieved","false"},
          {"relation_rows_reported","0"},{"timing_claim","false"}}));
        std::cout<<"Retained membership certificates pass; full relation goal remains open.\n"; return 0;
    } catch(const std::exception& e) { std::cerr<<"retained_basis: "<<e.what()<<'\n'; return 1; }
}
