// Exact planted m2 certificate on the frozen equations; no relation search.
#include "retained_depth8_prefix.inc"

using IntegerPolynomial=std::vector<Z>;
IntegerPolynomial iconvolve(const IntegerPolynomial& a,const IntegerPolynomial& b) {
    IntegerPolynomial c(a.size()+b.size()-1);
    for(size_t i=0;i<a.size();++i)for(size_t j=0;j<b.size();++j)c[i+j]+=a[i]*b[j];
    return c;
}
void itrim(IntegerPolynomial& p){while(!p.empty() && p.back()==0)p.pop_back();}
std::string integer_json(const IntegerPolynomial& p) {
    std::vector<std::string> v;for(const auto& z:p)v.push_back(dec(z));return array(v);
}
IntegerPolynomial numerator(const Basis& b) {
    unsigned top=0;for(const auto& m:b.standard)top=std::max(top,md(m));
    IntegerPolynomial h(top+1);for(const auto& m:b.standard)++h[md(m)];return h;
}
Monomial read_monomial(std::istream& in,const Vars& vars) {
    size_t count;in>>count;require(count<=vars.size(),"monomial width");Monomial m;
    for(size_t i=0;i<count;++i){std::string v;unsigned e;in>>v>>e;require(e>0 && e<1000 && !m.contains(v) && std::find(vars.begin(),vars.end(),v)!=vars.end(),"monomial power");m[v]=e;}
    require(bool(in),"truncated monomial");return m;
}
Poly read_polynomial(std::istream& in,const Vars& vars) {
    size_t count;in>>count;require(count>0 && count<10000,"polynomial size");Poly p;
    for(size_t i=0;i<count;++i){std::string s;in>>s;Z c(s);require(c>0 && c<P,"coefficient");auto m=read_monomial(in,vars);require(p.terms.emplace(m,c).second,"repeated term");}
    return p;
}
struct Indicators {std::array<std::vector<Z>,2> coefficients;};
Indicators interpolate(const Basis& basis,const Assignments& points,const std::array<size_t,2>& selected) {
    constexpr size_t n=288;
    require(points.size()==n && basis.standard.size()==n,"fixed288 square evaluation matrix");
    std::vector<std::vector<Z>> a(n,std::vector<Z>(n+2));
    for(size_t i=0;i<n;++i){for(size_t j=0;j<n;++j)a[i][j]=mt(basis.standard[j]).eval(points[i]);for(size_t j=0;j<2;++j)a[i][n+j]=i==selected[j]?1:0;}
    // Solve E*c=delta for two right-hand sides, with full pivot checks.
    for(size_t col=0;col<n;++col) {
        size_t pivot=col;while(pivot<n && a[pivot][col]==0)++pivot;require(pivot<n,"singular interpolation");
        std::swap(a[col],a[pivot]);Z scale=inv(a[col][col]);
        for(size_t j=col;j<n+2;++j)a[col][j]=mod(a[col][j]*scale);
        for(size_t i=0;i<n;++i)if(i!=col && a[i][col]!=0){Z factor=a[i][col];for(size_t j=col;j<n+2;++j)a[i][j]=mod(a[i][j]-factor*a[col][j]);}
    }
    Indicators out;
    for(size_t j=0;j<2;++j){out.coefficients[j].resize(n);for(size_t i=0;i<n;++i)out.coefficients[j][i]=a[i][n+j];}
    return out;
}
void verify_indicators(const Indicators& indicators,const Basis& basis,const Assignments& points,const std::array<size_t,2>& selected) {
    for(size_t j=0;j<2;++j){require(indicators.coefficients[j].size()==basis.standard.size(),"indicator width");Poly polynomial;
        for(size_t i=0;i<basis.standard.size();++i)polynomial+=mt(basis.standard[i],indicators.coefficients[j][i]);
        for(size_t i=0;i<points.size();++i)require(polynomial.eval(points[i])==(i==selected[j]?Z(1):Z(0)),"indicator does not evaluate to delta");
    }
}
struct TensorRow {unsigned degree;std::array<Z,2> c;size_t left,right;};
std::vector<TensorRow> tensor_rows(const Basis& basis,const Indicators& indicators) {
    std::vector<TensorRow> rows;const size_t n=basis.standard.size();rows.reserve(n*n);
    for(size_t i=0;i<n;++i)for(size_t j=0;j<n;++j)rows.push_back({md(basis.standard[i])+md(basis.standard[j]),
      {mod(indicators.coefficients[0][i]*indicators.coefficients[1][j]),mod(indicators.coefficients[1][i]*indicators.coefficients[0][j])},i,j});
    return rows;
}
Z det(const std::array<Z,2>& a,const std::array<Z,2>& b){return mod(a[0]*b[1]-a[1]*b[0]);}
struct RankProof {unsigned cutoff,rank;std::vector<size_t> witnesses;};
RankProof tail_rank(const std::vector<TensorRow>& rows,unsigned cutoff) {
    RankProof proof{cutoff,0,{}};
    for(size_t i=0;i<rows.size();++i)if(rows[i].degree>cutoff && (rows[i].c[0]!=0 || rows[i].c[1]!=0)) {
        if(proof.rank==0){proof.rank=1;proof.witnesses.push_back(i);}
        else if(det(rows[proof.witnesses[0]].c,rows[i].c)!=0){proof.rank=2;proof.witnesses.push_back(i);break;}
    }
    return proof;
}
void verify_rank(const std::vector<TensorRow>& rows,const RankProof& proof) {
    require(proof.rank<=2 && proof.witnesses.size()==proof.rank,"rank witness count");
    for(size_t i:proof.witnesses)require(i<rows.size() && rows[i].degree>proof.cutoff && (rows[i].c[0]!=0 || rows[i].c[1]!=0),"rank witness invalid");
    if(proof.rank==2){require(det(rows[proof.witnesses[0]].c,rows[proof.witnesses[1]].c)!=0,"rank2 determinant");return;}
    for(const auto& row:rows)if(row.degree>proof.cutoff) {
        if(proof.rank==0)require(row.c[0]==0 && row.c[1]==0,"claimed zero rank");
        else require(det(rows[proof.witnesses[0]].c,row.c)==0,"claimed rank1 not an upper bound");
    }
}
struct AnnGenerator {unsigned degree;std::array<Z,2> combination;};
std::vector<AnnGenerator> ann_basis(const std::vector<TensorRow>& rows,const std::vector<RankProof>& ranks) {
    std::vector<AnnGenerator> out;
    for(const auto& rank:ranks) {
        const size_t dimension=2-rank.rank;if(out.size()==dimension)continue;
        std::vector<std::array<Z,2>> choices;
        if(rank.rank==1){const auto& v=rows[rank.witnesses[0]].c;choices.push_back({v[1],mod(-v[0])});}
        else if(rank.rank==0)choices={std::array<Z,2>{Z(1),Z(0)},std::array<Z,2>{Z(0),Z(1)}};
        for(const auto& c:choices)if(out.empty() || det(out[0].combination,c)!=0) {
            out.push_back({rank.cutoff,c});if(out.size()==dimension)break;
        }
        require(out.size()==dimension,"annihilator extension failed");
    }
    require(out.size()==2 && det(out[0].combination,out[1].combination)!=0,"annihilator basis not independent");return out;
}
void verify_ann_basis(const std::vector<TensorRow>& rows,const std::vector<RankProof>& ranks,const std::vector<AnnGenerator>& gs) {
    require(gs.size()==2 && det(gs[0].combination,gs[1].combination)!=0,"annihilator independence");
    for(const auto& g:gs){bool has_top=false;for(const auto& row:rows){Z coefficient=mod(g.combination[0]*row.c[0]+g.combination[1]*row.c[1]);if(row.degree>g.degree)require(coefficient==0,"annihilator generator exceeds degree");if(row.degree==g.degree && coefficient!=0)has_top=true;}require(has_top,"wrong annihilator generator degree");}
    for(const auto& rank:ranks){verify_rank(rows,rank);size_t count=0;for(const auto& g:gs)if(g.degree<=rank.cutoff)++count;require(count==2-rank.rank,"annihilator filtration not spanned");}
}
struct FullHilbert {IntegerPolynomial numerator,torsion;int end_torsion;unsigned quotient_D,ideal_D;};
FullHilbert full_hilbert(const IntegerPolynomial& input,const IntegerPolynomial& ann,unsigned e) {
    IntegerPolynomial q(std::max(input.size()+e,ann.size()+e));
    for(size_t i=0;i<input.size();++i){q[i]+=input[i];q[i+e]-=input[i];}
    for(size_t i=0;i<ann.size();++i)q[i+e]+=ann[i];
    itrim(q);
    require(q.size()>=2,"unexpected numerator");IntegerPolynomial difference=q;difference[0]-=1;difference[1]-=1;
    IntegerPolynomial torsion(difference.size());Z prefix=0;int end=-1;
    for(size_t i=0;i<difference.size();++i){prefix+=difference[i];require(prefix>=0,"negative torsion Hilbert dimension");torsion[i]=prefix;if(prefix!=0)end=static_cast<int>(i);}
    require(prefix==0 && end>=0,"not the fixed nonzero finite H torsion case");
    itrim(torsion);unsigned quotient=static_cast<unsigned>(std::max(1,end));
    return {q,torsion,end,quotient,quotient+1};
}
std::string row_json(const TensorRow& row,size_t index) {
    return object({{"row",num(index)},{"left_standard",num(row.left)},{"right_standard",num(row.right)},
      {"degree",num(row.degree)},{"coefficients",array({dec(row.c[0]),dec(row.c[1])})}});
}
std::string analyze(const std::string& arm,const Basis& basis,const Vars& vars,const Assignments& points,
                    const std::array<size_t,2>& selected,const std::string& point_certificate_sha,unsigned e,size_t& corruptions) {
    std::cout<<arm<<": interpolate\n"<<std::flush;
    auto indicator=interpolate(basis,points,selected);verify_indicators(indicator,basis,points,selected);
    auto reject=[&](const auto& f){bool rejected=false;try{f();}catch(const std::exception&){rejected=true;}require(rejected,"corrupt certificate accepted");++corruptions;};
    auto broken=indicator;broken.coefficients[0][0]=mod(broken.coefficients[0][0]+1);
    reject([&]{verify_indicators(broken,basis,points,selected);});
    std::cout<<arm<<": tensor filtration\n"<<std::flush;
    auto rows=tensor_rows(basis,indicator);const auto block_h=numerator(basis);const auto input_h=iconvolve(block_h,block_h);
    std::vector<RankProof> ranks;for(unsigned cutoff=0;cutoff<input_h.size();++cutoff){auto rank=tail_rank(rows,cutoff);verify_rank(rows,rank);ranks.push_back(std::move(rank));}
    const auto gs=ann_basis(rows,ranks);verify_ann_basis(rows,ranks,gs);
    RankProof wrong=ranks.front();require(wrong.rank==2,"degree0 annihilator is unexpectedly nonzero");
    wrong.rank=1;wrong.witnesses.resize(1);reject([&]{verify_rank(rows,wrong);});
    IntegerPolynomial ann(input_h.size());for(const auto& g:gs)++ann[g.degree];itrim(ann);
    const auto full=full_hilbert(input_h,ann,e);reject([&]{require(full.ideal_D==full.quotient_D+2,"wrong ideal/quotient shift");});
    bool duality_matched=false;
    if(arm=="dense") {
        // Checked only AFTER the independent tensor/rank calculation.
        const size_t r=input_h.size()-1;IntegerPolynomial expected(r+1);expected[r-1]=1;expected[r]=1;
        require(ann==expected,"independent strict-CI duality prediction disagrees");duality_matched=true;
    }
    std::string stream;for(size_t i=0;i<rows.size();++i)stream+=num(i)+"|"+num(rows[i].degree)+"|"+rows[i].c[0].get_str()+"|"+rows[i].c[1].get_str()+"\n";
    std::vector<std::string> coefficients,proofs,generators;
    for(const auto& c:indicator.coefficients)coefficients.push_back(integer_json(c));
    for(const auto& proof:ranks){std::vector<std::string> witnesses;for(size_t i:proof.witnesses)witnesses.push_back(row_json(rows[i],i));proofs.push_back(object({{"cutoff",num(proof.cutoff)},{"tail_rank",num(proof.rank)},
      {"annihilator_dimension",num(2-proof.rank)},{"witnesses",array(witnesses)},
      {"low_rank_upper_bound_checked",proof.rank<2?"true":"false"}}));}
    for(const auto& g:gs)generators.push_back(object({{"degree",num(g.degree)},
      {"accepted_point_combination",array({dec(g.combination[0]),dec(g.combination[1])})}}));
    std::vector<std::string> names;for(const auto& v:vars)names.push_back(jquote(v));
    return object({{"arm",jquote(arm)},{"point_variables",array(names)},
      {"point_certificate_sha256",jquote(point_certificate_sha)},
      {"indicator_point_indices",array({num(selected[0]),num(selected[1])})},
      {"indicator_coefficients_in_standard_basis",array(coefficients)},
      {"tensor_rows",num(rows.size())},{"tensor_stream_sha256",jquote(hash(stream))},
      {"tensor_stream_encoding",jquote("row|total_degree|coefficient0_decimal|coefficient1_decimal followed by LF")},
      {"tail_rank_certificates",array(proofs)},{"annihilator_free_K_H_generators",array(generators)},
      {"point_Hilbert_numerator",integer_json(block_h)},{"input_Hilbert_numerator",integer_json(input_h)},
      {"annihilator_Hilbert_numerator",integer_json(ann)},{"complete_quotient_Hilbert_numerator",integer_json(full.numerator)},
      {"complete_quotient_Hilbert_denominator",jquote("1-t")},{"accepted_Hilbert_numerator",array({"1","1"})},
      {"finite_H_torsion_Hilbert_polynomial",integer_json(full.torsion)},
      {"H_torsion_last_degree",num(static_cast<size_t>(full.end_torsion))},
      {"full_quotient_regularity",num(full.quotient_D)},{"full_ideal_regularity",num(full.ideal_D)},
      {"independent_strict_CI_duality_prediction_matched",duality_matched?"true":"false"},
      {"standing",jquote("unsigned producer algebraic certificate; not independent signed/Lean admission")}});
}
int main(int argc,char** argv) {
    try {
        require(argc==3,"usage: ann_m2 CACHE FRESH_OUT");std::filesystem::path out=argv[2];require(!std::filesystem::exists(out),"fresh output");
        std::istringstream in(read(argv[1]));std::string tag;size_t n,k;Vars vars;
        in>>tag>>k;require(tag=="VARS" && k==9,"variables");for(size_t i=0;i<k;++i){std::string v;in>>v;vars.push_back(v);}
        in>>tag>>n;require(tag=="POINTS" && n==288,"point count");Assignments points;
        for(size_t i=0;i<n;++i){std::map<std::string,Z> p;for(const auto& v:vars){std::string c;in>>c;Z z(c);require(z>=0 && z<P,"coordinate");p[v]=z;}points.push_back(std::move(p));}
        Basis chain;in>>tag>>k;require(tag=="CHAIN_STANDARD" && k==n,"standard count");for(size_t i=0;i<k;++i)chain.standard.push_back(read_monomial(in,vars));
        in>>tag>>k;require(tag=="CHAIN_GENERATORS" && k==51,"generator count");for(size_t i=0;i<k;++i)chain.generators.push_back(read_polynomial(in,vars));
        std::string chain_sha,circuit_sha,line_sha;in>>tag>>chain_sha;require(tag=="CHAIN_SHA" && chain_sha.size()==64,"chain hash");
        in>>tag>>k;require(tag=="RELATIONS" && k==2,"accepted reference count");std::vector<std::array<size_t,2>> accepted(k);
        for(auto& ij:accepted){in>>ij[0]>>ij[1];require(ij[0]<n && ij[1]<n,"relation index");}
        unsigned inf,e,dense_r;std::string tx,ty;in>>tag>>inf>>tx>>ty;require(tag=="TARGET" && inf==0,"finite fixed target");Point target=point(Z(tx),Z(ty));
        in>>tag>>e;require(tag=="GROUP_DEGREE" && e==4,"group degree");in>>tag>>dense_r;require(tag=="DENSE_R" && dense_r==97,"dense CI degree");
        in>>tag>>circuit_sha;require(tag=="CIRCUIT_SHA" && circuit_sha.size()==64,"circuit hash");in>>tag>>line_sha;require(tag=="LINE_SHA" && line_sha.size()==64,"line hash");in>>tag;require(tag=="END" && !(in>>tag),"cache terminator");
        const auto before=counters;OpenSSLCheck ssl(TARGET);
        auto verify_reference=[&](const auto& refs){require(refs.size()==2 && refs[0][0]==193 && refs[0][1]==262 && refs[1][0]==262 && refs[1][1]==193,"complete frozen reference changed");};verify_reference(accepted);
        for(const auto& ij:accepted){const auto& a=points[ij[0]];const auto& b=points[ij[1]];Point u=point(a.at("x"),a.at("y")),v=point(b.at("x"),b.at("y"));require(TARGET.add(u,v)==target,"reported relation failed");ssl.add(u,v,target);}
        size_t corruptions=0;auto missing=accepted;missing.pop_back();try{verify_reference(missing);}catch(const std::exception&){++corruptions;}require(corruptions==1,"missing tuple accepted");
        Z separator=mod(points[193].at("x")-points[262].at("x"));require(separator!=0,"accepted two-point linear separation");Z separator_inverse=inv(separator);require(mod(separator*separator_inverse)==1,"separator inverse");
        std::cout<<"check cached chain ideal\n"<<std::flush;
        const auto chain_cert=certify_dimension(chain,vars,points,6);require(hash(chain_cert+"\n")==chain_sha,"cached chain ideal changed");
        const auto after_chain=counters;Vars dense_vars{"x","y"};Assignments dense_points;
        for(const auto& p:points)dense_points.push_back({{"x",p.at("x")},{"y",p.at("y")}});
        std::cout<<"construct dense interpolation basis\n"<<std::flush;
        const auto dense=buchberger_moeller(dense_vars,dense_points);const auto after_bm=counters;
        const auto dense_cert=certify_dimension(dense,dense_vars,dense_points,dense_r+1);const auto after_dense=counters;
        write(out/"dense-membership.json",dense_cert);
        const std::array<size_t,2> selected{193,262};
        const std::string dense_ann=analyze("dense",dense,dense_vars,dense_points,selected,hash(dense_cert+"\n"),e,corruptions);
        const auto after_dense_ann=counters;
        const std::string chain_ann=analyze("chain",chain,vars,points,selected,chain_sha,e,corruptions);
        write(out/"dense-annihilator.json",dense_ann);write(out/"chain-annihilator.json",chain_ann);
        write(out/"manifest.json",object({{"factor_base",jquote("FB1h16882188194e")},{"m","2"},{"kind",jquote("fixed planted")},
          {"cache_sha256",jquote(hash(read(argv[1])))},{"group_circuit_sha256",jquote(circuit_sha)},{"target_line_sha256",jquote(line_sha)},
          {"reported_relations",array({array({"193","262"}),array({"262","193"})})},{"group_replays","2"},
          {"accepted_two_point_linear_separator",object({{"coordinate",jquote("x of first input")},{"difference",dec(separator)},{"inverse",dec(separator_inverse)}})},
          {"dense_membership_certificate_sha256",jquote(hash(dense_cert+"\n"))},
          {"dense_annihilator_certificate_sha256",jquote(hash(dense_ann+"\n"))},{"chain_annihilator_certificate_sha256",jquote(hash(chain_ann+"\n"))},
          {"corrupt_certificates_rejected",num(corruptions)},
          {"chain_input_verification_mod_API_calls",num(after_chain.reductions-before.reductions)},
          {"dense_BM_mod_API_calls",num(after_bm.reductions-after_chain.reductions)},
          {"dense_input_verification_mod_API_calls",num(after_dense.reductions-after_bm.reductions)},
          {"dense_annihilator_mod_API_calls",num(after_dense_ann.reductions-after_dense.reductions)},
          {"chain_annihilator_mod_API_calls",num(counters.reductions-after_dense_ann.reductions)},
          {"counter_scope",jquote("construction/certificate GMP API calls, no field-equivalent solve or runtime claim")},
          {"affine_variables_dense","4"},{"affine_variables_chain","18"},{"free_group_witness_variables","0"},
          {"equations_or_preprocessing_changed","false"},{"independent_signed_or_Lean_admission","false"},
          {"higher_m_degree_validation",jquote("preserved and open")},{"goal_achieved","false"}}));
        std::cout<<"Exact planted m2 annihilator certificates completed; unsigned.\n";
    }catch(const std::exception& error){std::cerr<<"ann_m2: "<<error.what()<<'\n';return 1;}
}
