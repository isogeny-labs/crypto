// Cached exact equations, complete m2 reference, and finite duality controls.
#include <bit>
#include "complete_law_prefix.inc"
Poly read_poly(std::istream& in) {
    size_t count;in>>count;Poly p;
    for(size_t i=0;i<count;++i){std::string c;size_t k;in>>c>>k;Monomial m;for(size_t j=0;j<k;++j){std::string v;unsigned e;in>>v>>e;require(e>0 && !m.contains(v),"cache monomial");m[v]=e;}require(p.terms.emplace(m,Z(c)).second,"cache duplicate term");}
    require(bool(in),"truncated polynomial");return p;
}
Poly hom(const Poly& p){Poly q;for(const auto& [m,c]:p.terms){Monomial n=m;unsigned d=0;for(const auto& [v,e]:m){require(v!="H","reserved H");d+=e;}if(d<p.degree())n["H"]=p.degree()-d;q.terms[n]=c;}return q;}
struct Cell {unsigned m;std::string kind,sha;Point target;Line line;std::vector<size_t> indices;};
size_t rank_matrix(std::vector<std::vector<Z>> a) {
    if(a.empty())return 0;
    size_t rank=0;
    for(size_t col=0;col<a[0].size() && rank<a.size();++col){size_t pivot=rank;while(pivot<a.size() && a[pivot][col]==0)++pivot;if(pivot==a.size())continue;std::swap(a[pivot],a[rank]);Z scale=inv(a[rank][col]);for(size_t j=col;j<a[0].size();++j)a[rank][j]=mod(a[rank][j]*scale);
        for(size_t i=0;i<a.size();++i)if(i!=rank && a[i][col]!=0){Z k=a[i][col];for(size_t j=col;j<a[0].size();++j)a[i][j]=mod(a[i][j]-k*a[rank][j]);}
        ++rank;
    }return rank;
}
std::string duality_units() {
    // A different calculation from the derived series: actual evaluation ranks.
    int ranks[512][5]{};
    for(unsigned mask=0;mask<512;++mask)for(int d=0;d<=4;++d){std::vector<std::vector<Z>> a;
        for(unsigned i=0;i<9;++i)if(mask&(1U<<i)){std::vector<Z> row;for(unsigned u=0;u<=2;++u)for(unsigned v=0;v<=2;++v)if(u+v<=static_cast<unsigned>(d))row.push_back(power(Z(i/3),Z(u))*power(Z(i%3),Z(v)));a.push_back(std::move(row));}
        ranks[mask][d]=static_cast<int>(rank_matrix(a));
    }
    auto hf=[&](unsigned mask,int d){return d<0?0:ranks[mask][std::min(d,4)];};
    size_t units=0,duality_checks=0,wrong_shift_rejections=0;std::vector<std::string> distribution;
    for(int e:{4,5}){std::map<int,size_t> Ds;
        for(unsigned Zmask=0;Zmask<512;++Zmask){unsigned W=511U^Zmask;int size=std::popcount(Zmask),rC=-1,endT=-1;
            for(int d=0;d<=4;++d)if(hf(Zmask,d)>hf(Zmask,d-1))rC=d;
            bool wrong_shift_detected=false;
            for(int d=0;d<=4+e+2;++d){int ann=hf(511,d)-hf(W,d),pred=size-hf(Zmask,4-d-1);require(ann==pred,"filtered duality shift fails");++duality_checks;
                if(ann!=size-hf(Zmask,4-d))wrong_shift_detected=true;
                int Q=hf(511,d)-hf(W,d-e),torsion=Q-hf(Zmask,d);require(torsion>=0,"negative torsion dimension");if(torsion)endT=d;
            }
            if(wrong_shift_detected)++wrong_shift_rejections;
            int D=1+std::max(rC,endT);require(D<=4+e,"upper bound grid counterexample");
            if(std::popcount(W)>=2)require(D>=4,"lower bound grid counterexample");
            ++Ds[D];++units;
        }
        Object values;for(const auto& [D,count]:Ds)values[num(static_cast<size_t>(D))]=num(count);
        distribution.push_back(object({{"e",num(static_cast<size_t>(e))},{"ideal_regularity_counts",object(values)}}));
    }
    require(wrong_shift_rejections>0,"shift mutation undetected");
    return object({{"grid_points","9"},{"accepted_subsets","512"},{"units",num(units)},
      {"duality_dimension_checks",num(duality_checks)},{"wrong_shift_detected_units",num(wrong_shift_rejections)},
      {"distributions",array(distribution)},{"scope",jquote("finite3x3 grid algebra regression, not an elliptic experiment or proof of the general lemma")}});
}
int main(int argc,char** argv) {
    try {
        require(argc==3,"usage: bound_assembly CACHE FRESH_OUT");std::filesystem::path out=argv[2];require(!std::filesystem::exists(out),"fresh output");
        std::istringstream in(read(argv[1]));std::string tag;size_t n,k;in>>tag>>k;require(tag=="VARS" && k==9,"vars");std::vector<std::string> vars;for(size_t i=0;i<k;++i){std::string v;in>>v;vars.push_back(v);}
        in>>tag>>n;require(tag=="POINTS" && n==288,"points");std::vector<std::map<std::string,Z>> assignments;std::vector<Point> domain;
        for(size_t i=0;i<n;++i){std::map<std::string,Z> p;for(const auto& v:vars){std::string s;in>>s;p[v]=Z(s);}p["H"]=1;require(TARGET.on(point(p.at("x"),p.at("y"))),"target point");domain.push_back(point(p.at("x"),p.at("y")));assignments.push_back(std::move(p));}
        std::vector<Poly> chain,dense;in>>tag>>k;require(tag=="CHAIN" && k==51,"chain generator count");for(size_t i=0;i<k;++i)chain.push_back(read_poly(in));
        in>>tag>>k;require(tag=="DENSE" && k==2,"dense generator count");for(size_t i=0;i<k;++i)dense.push_back(hom(read_poly(in)));
        in>>tag>>k;require(tag=="CELLS" && k==12,"cells");std::vector<Cell> cells;
        for(size_t i=0;i<k;++i){Cell c;unsigned inf;std::string tx,ty,lx,ly,lz,param;in>>c.m>>c.kind>>inf>>tx>>ty>>lx>>ly>>lz>>param>>c.sha;require(c.m>=2 && c.m<=7 && inf<=1,"cell header");c.target=inf?Point{}:point(Z(tx),Z(ty));c.line={Z(lx),Z(ly),Z(lz),Z(param),P-1,inf==1};for(unsigned j=0;j<c.m;++j){size_t index;in>>index;require(index<n,"witness index");c.indices.push_back(index);}cells.push_back(std::move(c));}
        in>>tag;require(tag=="END" && !(in>>tag),"cache terminator");
        const auto before=counters;OpenSSLCheck ssl(TARGET);size_t equation_checks=0,replays=0;std::vector<std::string> results;
        for(const auto& c:cells) {
            const auto& l=c.line;Z discriminant;
            if(c.target.infinity)discriminant=mod(l.parameter*l.parameter*l.parameter-3*l.parameter+B);
            else{Z a=mod(3*c.target.x-l.parameter*l.parameter),b=mod(3*c.target.x*c.target.x-3-2*l.parameter*c.target.y);discriminant=mod(a*a-4*b);}
            require(power(discriminant,(P-1)/2)==P-1,"line nonsquare certificate");(void)line_json(l,c.target);
            Point sum;std::map<std::string,Z> witness{{"H",1}};
            for(size_t i=0;i<c.indices.size();++i){size_t index=c.indices[i];for(const auto& g:chain){require(g.eval(assignments[index])==0,"chain witness equation");++equation_checks;}for(const auto& g:dense){require(g.eval(assignments[index])==0,"dense witness equation");++equation_checks;}
                auto next=TARGET.add(sum,domain[index]);ssl.add(sum,domain[index],next);sum=next;witness["x"+num(i)]=domain[index].x;witness["y"+num(i)]=domain[index].y;
            }
            nodes.clear();C H=C::var("H");std::vector<Proj<C>> leaves;for(unsigned i=0;i<c.m;++i)leaves.push_back({C::var("x"+num(i)),C::var("y"+num(i)),H});auto symbolic=balanced(leaves,0,leaves.size());C f=C(l.x)*symbolic.x+C(l.y)*symbolic.y+C(l.z)*symbolic.z;
            require(hash(circuit_json(symbolic,f)+"\n")==c.sha,"frozen circuit differs");auto values=evaluate_circuit(witness);require(decode(Proj<F>{values[symbolic.x.id],values[symbolic.y.id],values[symbolic.z.id]})==sum,"circuit/reference sum");require((values[f.id].z==0)==(sum==c.target),"line/reference classification");
            if(c.kind=="planted"){require(sum==c.target,"reported relation failed replay");++replays;}
            unsigned e=nodes[f.id].degree,lower=97*c.m,upper=5*c.m+e;require(2*upper<=lower,"numeric twofold inequality");
            results.push_back(object({{"m",num(c.m)},{"kind",jquote(c.kind)},{"dense_D_lower",num(lower)},
              {"chain_D_upper",num(upper)},{"numeric_twofold_inequality","true"},{"group_polynomial_degree",num(e)},
              {"affine_variables_dense",num(2*c.m)},{"affine_variables_chain",num(9*c.m)},
              {"membership_equations_dense",num(2*c.m)},{"membership_equations_chain",num(51*c.m)},
              {"group_equations","1"},{"homogenizing_variables","1"},{"free_group_witness_variables","0"},
              {"source",jquote("FULL_BOUND_PROOF.md applied to exact point certificates; independent general-proof review pending")}}));
        }
        std::vector<std::string> reference;size_t all_replayed=0;
        std::vector<Cell> pairs;for(const auto& c:cells)if(c.m==2)pairs.push_back(c);require(pairs.size()==2,"m2 cells");
        std::vector<std::vector<std::string>> relations(2);size_t checked=0;
        for(size_t i=0;i<n;++i)for(size_t j=0;j<n;++j){Point expected=TARGET.add(domain[i],domain[j]);ssl.add(domain[i],domain[j],expected);auto p=rcb(encode(domain[i]),encode(domain[j]));require(decode(p)==expected,"complete m2 law discrepancy");
            for(size_t c=0;c<2;++c){bool accepted=line_eval(pairs[c].line,p)==0;require(accepted==(expected==pairs[c].target),"m2 false positive/negative");if(accepted){relations[c].push_back(array({num(i),num(j)}));++all_replayed;}}
            ++checked;
        }
        for(size_t c=0;c<2;++c)reference.push_back(object({{"kind",jquote(pairs[c].kind)},{"tuples_checked",num(checked)},
          {"relations",array(relations[c])},{"false_positives","0"},{"false_negatives","0"},
          {"ordered_tuple_semantics","true"},{"rank_or_independence_claim","false"}}));
        write(out/"finite-duality-units.json",duality_units());
        write(out/"complete-m2-reference.json",array(reference));
        write(out/"derived-full-bounds.json",object({{"factor_base",jquote("FB1h16882188194e")},
          {"definition",jquote("D=reg_S(J), actual unsaturated generator homogenization; no preprocessing")},
          {"cells",array(results)},{"exact_full_D_values_measured","false"},{"affine_F4_or_runtime_gain","false"},
          {"proof_review_status",jquote("producer derivation plus finite regression; not independently signed or Lean verified")}}));
        write(out/"manifest.json",object({{"cache_sha256",jquote(hash(read(argv[1])))},{"matched_cells",num(cells.size())},
          {"cached_membership_equation_witness_checks",num(equation_checks)},{"planted_relations_replayed",num(replays)},
          {"complete_m2_tuples_per_target",num(checked)},{"complete_m2_relations_replayed",num(all_replayed)},
          {"field_modular_reduction_API_calls",num(counters.reductions-before.reductions)},
          {"counter_scope",jquote("native setup, equation/reference replay and grid checks, not field-equivalent solver cost")},
          {"goal_achieved","false"},{"remaining",jquote("independent review of full-bound proof; affine F4 degree/runtime and larger solve obligations")}}));
        std::cout<<"12paired full-bound applications; complete m2 reference="<<checked<<" tuples/target; independent proof review pending.\n";
    }catch(const std::exception& e){std::cerr<<"bound_assembly: "<<e.what()<<'\n';return 1;}
}
