// Finite weighted pairing plus explicit hypotheses for all frozen arities.
#include "ann_m2_prefix.inc"
constexpr size_t FN=288;
struct ArithmeticNode {std::string op,name;Z value;size_t a,b;unsigned saved_degree;};
struct ArithmeticGraph {std::vector<ArithmeticNode> nodes;std::array<size_t,4> output;};
struct FixedCell {unsigned m,e;std::string kind,circuit_sha,line_sha;Point target;Z lx,ly,lz,slope;ArithmeticGraph graph;};
struct GraphValue {Z value;int degree;};
std::vector<GraphValue> graph_values(const ArithmeticGraph& graph,const std::map<std::string,Z>& input) {
    std::vector<GraphValue> out;out.reserve(graph.nodes.size());
    for(const auto& node:graph.nodes) {
        if(node.op=="constant"){out.push_back({node.value,node.value==0?-1:0});continue;}
        if(node.op=="input"){require(input.contains(node.name),"unaccounted circuit input");out.push_back({input.at(node.name),1});continue;}
        require(node.a<out.size() && node.b<out.size(),"DAG dependency");const auto& a=out[node.a];const auto& b=out[node.b];
        Z value;int degree;
        if(node.op=="multiply"){value=mod(a.value*b.value);degree=(a.degree<0 || b.degree<0)?-1:a.degree+b.degree;}
        else {
            require(node.op=="add" || node.op=="subtract","unknown circuit operation");
            value=node.op=="add"?mod(a.value+b.value):mod(a.value-b.value);
            require(a.degree<0 || b.degree<0 || a.degree==b.degree,"inhomogeneous arithmetic node");degree=std::max(a.degree,b.degree);
            if(degree==0 && value==0)degree=-1;
        }
        require(degree<0 || static_cast<unsigned>(degree)<=node.saved_degree,"stored degree not an upper bound");out.push_back({value,degree});
    }return out;
}
Poly cut_line_form(const ArithmeticGraph& graph,size_t id,std::map<size_t,Poly>& memo,size_t& visited) {
    if(memo.contains(id))return memo.at(id);
    require(++visited<=64,"line-form cut unexpectedly large");
    for(size_t i=0;i<3;++i)if(id==graph.output[i])return Poly::var("OUT"+num(i));
    require(id<graph.nodes.size(),"line-form output");const auto& n=graph.nodes[id];Poly p;
    if(n.op=="constant")p=Poly(n.value);
    else {
        require(n.op!="input" && n.a<id && n.b<id,"relation has free inputs outside group outputs");
        Poly a=cut_line_form(graph,n.a,memo,visited),b=cut_line_form(graph,n.b,memo,visited);
        if(n.op=="add")p=a+b;else if(n.op=="subtract")p=a-b;else{require(n.op=="multiply","line op");p=a*b;}
    }
    memo[id]=p;return p;
}
Point decode_sum(const std::vector<GraphValue>& values,const std::array<size_t,4>& indices) {
    const Z X=values.at(indices[0]).value,Y=values.at(indices[1]).value,Zc=values.at(indices[2]).value;
    require(X!=0 || Y!=0 || Zc!=0,"zero projective sum");
    if(Zc==0){require(X==0,"invalid infinity");return {};}
    Z iz=inv(Zc);Point p=point(X*iz,Y*iz);require(TARGET.on(p),"off-curve circuit sum");return p;
}
std::vector<Z> transpose_functional(const std::vector<std::vector<Z>>& E,size_t top) {
    require(E.size()==FN && top<FN,"functional size");std::vector<std::vector<Z>> a(FN,std::vector<Z>(FN+1));
    for(size_t i=0;i<FN;++i){require(E[i].size()==FN,"E row");for(size_t j=0;j<FN;++j)a[i][j]=E[j][i];a[i][FN]=i==top?1:0;}
    for(size_t col=0;col<FN;++col){size_t pivot=col;while(pivot<FN && a[pivot][col]==0)++pivot;require(pivot<FN,"singular transpose solve");std::swap(a[col],a[pivot]);Z scale=inv(a[col][col]);
        for(size_t j=col;j<=FN;++j)a[col][j]=mod(a[col][j]*scale);
        for(size_t i=0;i<FN;++i)if(i!=col && a[i][col]!=0){Z factor=a[i][col];for(size_t j=col;j<=FN;++j)a[i][j]=mod(a[i][j]-factor*a[col][j]);}
    }
    std::vector<Z> weights(FN);for(size_t i=0;i<FN;++i)weights[i]=a[i][FN];return weights;
}
void check_functional(const std::vector<std::vector<Z>>& E,const std::vector<Z>& weights,size_t top) {
    require(weights.size()==FN,"weight count");
    for(size_t i=0;i<FN;++i)require(weights[i]!=0,"degenerate weighted point pairing");
    for(size_t j=0;j<FN;++j){Z sum=0;for(size_t i=0;i<FN;++i)sum+=weights[i]*E[i][j];require(mod(sum)==(j==top?Z(1):Z(0)),"functional column identity");}
}
void complementary_dimensions(const IntegerPolynomial& h,unsigned r) {
    require(h.size()==r+1,"point numerator degree");Z sum=0;for(const auto& x:h){require(x>=0,"negative point h");sum+=x;}require(sum==FN,"point count");
    for(size_t i=0;i<h.size();++i)require(h[i]==h[h.size()-1-i],"asymmetric point Hilbert numerator");
    for(unsigned cutoff=0;cutoff<=r;++cutoff){Z left=0,right=0;for(unsigned i=0;i<=cutoff;++i)left+=h[i];for(unsigned i=0;i<r-cutoff;++i)right+=h[i];require(left+right==sum,"complementary filtration dimensions");}
}
std::string verify_no_two(std::istream& in) {
    std::string tag;size_t k;in>>tag>>k;require(tag=="NO_TWO" && k==4,"no2 header");const Vars vars{"x"};
    Poly cubic=read_polynomial(in,vars),residue=read_polynomial(in,vars),u=read_polynomial(in,vars),v=read_polynomial(in,vars),x=Poly::var("x");
    require((cubic-(x*x*x-Poly(3)*x+Poly(B))).terms.empty(),"wrong no2 curve");Poly xp(1);
    for(int i=static_cast<int>(mpz_sizeinbase(P.get_mpz_t(),2))-1;i>=0;--i){xp=normal_form(sq(xp),{cubic},vars).remainder;if(mpz_tstbit(P.get_mpz_t(),static_cast<mp_bitcnt_t>(i)))xp=normal_form(xp*x,{cubic},vars).remainder;}
    require((xp-residue).terms.empty() && (u*cubic+v*(residue-x)-Poly(1)).terms.empty(),"no2 residue/Bezout failed");
    require(mod(4*TARGET.a*TARGET.a*TARGET.a+27*B*B)!=0,"singular target");return hash(cubic.json()+residue.json()+u.json()+v.json());
}
std::string verify_cell(const FixedCell& c,const Assignments& points,OpenSSLCheck& ssl,size_t& group_checks) {
    require(c.m>=2 && c.m<=7 && !c.target.infinity && TARGET.on(c.target),"fixed finite target");
    require(c.ly==1 && c.lx==mod(-c.slope) && c.lz==mod(c.slope*c.target.x-c.target.y),"line coefficients");
    Z qa=mod(3*c.target.x-c.slope*c.slope),qb=mod(3*c.target.x*c.target.x-3-2*c.slope*c.target.y),discriminant=mod(qa*qa-4*qb);
    require(power(discriminant,(P-1)/2)==P-1,"target line not isolating");
    Poly u=Poly::var("u"),xx=u+Poly(c.target.x),yy=Poly(c.slope)*u+Poly(c.target.y);
    require((xx*xx*xx-Poly(3)*xx+Poly(B)-sq(yy)-u*(sq(u)+Poly(qa)*u+Poly(qb))).terms.empty(),"line substitution identity");
    std::map<std::string,Z> pure{{"H",0}};for(unsigned i=0;i<c.m;++i){pure["x"+num(i)]=0;pure["y"+num(i)]=1;}
    std::set<std::string> inputs;
    for(const auto& node:c.graph.nodes)if(node.op=="input")inputs.insert(node.name);
    require(inputs.size()==pure.size(),"missing or extra group inputs");
    for(const auto& [name,value]:pure){(void)value;require(inputs.contains(name),"missing variable group column");}
    const auto pv=graph_values(c.graph,pure);for(size_t id:c.graph.output)require(id<pv.size(),"DAG outputs");
    require(pv[c.graph.output[3]].degree==static_cast<int>(c.e) && pv[c.graph.output[3]].value==1,"actual group equation degree");
    for(size_t i=0;i<3;++i)require(pv[c.graph.output[i]].degree==static_cast<int>(c.e),"group output homogeneous degree");
    std::map<size_t,Poly> memo;size_t visits=0;Poly line=cut_line_form(c.graph,c.graph.output[3],memo,visits);
    require((line-(Poly(c.lx)*Poly::var("OUT0")+Poly(c.ly)*Poly::var("OUT1")+Poly(c.lz)*Poly::var("OUT2"))).terms.empty(),"raw relation is not exactly target line of sum");
    Point p0=point(points[0].at("x"),points[0].at("y")),prefix;
    for(unsigned i=0;i<c.m-1;++i){Point next=TARGET.add(prefix,p0);ssl.add(prefix,p0,next);++group_checks;prefix=next;}
    std::vector<std::array<Z,2>> rows;std::vector<std::string> witnesses;
    for(size_t last=0;last<4;++last){Point q=point(points[last].at("x"),points[last].at("y")),expected=TARGET.add(prefix,q);ssl.add(prefix,q,expected);++group_checks;
        std::map<std::string,Z> assignment{{"H",1}};std::vector<std::string> tuple;
        for(unsigned i=0;i<c.m;++i){size_t index=i+1==c.m?last:0;assignment["x"+num(i)]=points[index].at("x");assignment["y"+num(i)]=points[index].at("y");tuple.push_back(num(index));}
        auto values=graph_values(c.graph,assignment);require(decode_sum(values,c.graph.output)==expected,"DAG/group witness mismatch");
        Z X=values[c.graph.output[0]].value,Y=values[c.graph.output[1]].value,Zc=values[c.graph.output[2]].value;
        require(values[c.graph.output[3]].value==mod(c.lx*X+c.ly*Y+c.lz*Zc),"line evaluation mismatch");
        rows.push_back({mod(X-c.target.x*Zc),mod(Y-c.target.y*Zc)});
        witnesses.push_back(object({{"input_indices",array(tuple)},{"sum",point_json(expected)},
          {"projective_sum",array({dec(X),dec(Y),dec(Zc)})},{"cross_forms",array({dec(rows.back()[0]),dec(rows.back()[1])})}}));
    }
    size_t wi=0,wj=0;Z determinant=0;
    for(size_t i=0;i<4 && determinant==0;++i)for(size_t j=i+1;j<4;++j){Z d=det(rows[i],rows[j]);if(d!=0){wi=i;wj=j;determinant=d;break;}}
    require(determinant!=0,"two target-cross forms not independently witnessed");Z inverse=inv(determinant);require(mod(determinant*inverse)==1,"independence determinant inverse");
    return object({{"m",num(c.m)},{"kind",jquote(c.kind)},{"group_degree",num(c.e)},
      {"circuit_sha256",jquote(c.circuit_sha)},{"line_sha256",jquote(c.line_sha)},
      {"target",point_json(c.target)},{"isolating_line_discriminant",dec(discriminant)},
      {"homogeneous_DAG_checked","true"},{"pure_Y_degree_witness","true"},{"free_inputs",num(2*c.m+1)},
      {"fixed_tuple_witnesses",array(witnesses)},{"independence_rows",array({num(wi),num(wj)})},
      {"independence_determinant",dec(determinant)},{"determinant_inverse",dec(inverse)}});
}
int main(int argc,char** argv) {
    try {
        require(argc==3,"usage: frobenius CACHE FRESH_OUT");std::filesystem::path out=argv[2];require(!std::filesystem::exists(out),"fresh output");
        std::istringstream in(read(argv[1]));std::string tag;size_t count;Vars vars;in>>tag>>count;require(tag=="VARS" && count==2,"variables");
        for(size_t i=0;i<count;++i){std::string v;in>>v;vars.push_back(v);}require(vars==Vars({"x","y"}),"dense variables");
        in>>tag>>count;require(tag=="POINTS" && count==FN,"points");Assignments points;
        for(size_t i=0;i<FN;++i){std::map<std::string,Z> p;for(const auto& name:vars){std::string s;in>>s;p[name]=Z(s);require(p[name]>=0 && p[name]<P,"point coordinate");}require(TARGET.on(point(p["x"],p["y"])),"target point");points.push_back(std::move(p));}
        Basis basis;in>>tag>>count;require(tag=="STANDARD" && count==FN,"footprint");for(size_t i=0;i<count;++i)basis.standard.push_back(read_monomial(in,vars));
        in>>tag>>count;require(tag=="GENERATORS" && count>0 && count<100,"generators");for(size_t i=0;i<count;++i)basis.generators.push_back(read_polynomial(in,vars));
        std::string dense_sha;in>>tag>>dense_sha;require(tag=="CERT_SHA" && dense_sha.size()==64,"point hash");
        in>>tag>>count;require(tag=="CHAIN_H" && count==6,"chain h");IntegerPolynomial chain_h(count);for(auto& z:chain_h){std::string s;in>>s;z=Z(s);require(z>=0,"chain h coefficient");}
        const auto before=counters;const std::string no2=verify_no_two(in);
        in>>tag>>count;require(tag=="CELLS" && count==12,"cell count");std::vector<FixedCell> cells;
        for(size_t i=0;i<count;++i){FixedCell c;std::string x,y,lx,ly,lz,s;in>>c.m>>c.kind>>c.e>>x>>y>>lx>>ly>>lz>>s>>c.circuit_sha>>c.line_sha;c.target=point(Z(x),Z(y));c.lx=Z(lx);c.ly=Z(ly);c.lz=Z(lz);c.slope=Z(s);
            size_t n;in>>tag>>n;require(tag=="DAG" && n>0 && n<=10000,"DAG count");for(auto& id:c.graph.output)in>>id;
            for(size_t j=0;j<n;++j){ArithmeticNode node;std::string value;in>>node.op>>node.name>>value>>node.a>>node.b>>node.saved_degree;node.value=Z(value);require(node.value>=0 && node.value<P,"DAG coefficient");c.graph.nodes.push_back(std::move(node));}
            require(bool(in),"truncated cell");cells.push_back(std::move(c));
        }
        in>>tag;require(tag=="END" && !(in>>tag),"cache terminator");
        std::cout<<"verify dense point certificate\n"<<std::flush;
        const auto point_cert=certify_dimension(basis,vars,points,98);require(hash(point_cert+"\n")==dense_sha,"dense certificate changed");
        const auto after_point=counters;const auto h=numerator(basis);complementary_dimensions(h,97);size_t top=FN;
        for(size_t i=0;i<FN;++i)if(md(basis.standard[i])==97){require(top==FN,"more than one socle basis element");top=i;}
        require(top<FN,"missing socle basis element");
        std::vector<std::vector<Z>> E(FN,std::vector<Z>(FN));std::string evaluation_stream;
        for(size_t i=0;i<FN;++i)for(size_t j=0;j<FN;++j){E[i][j]=mt(basis.standard[j]).eval(points[i]);evaluation_stream+=E[i][j].get_str()+"\n";}
        std::cout<<"solve weighted functional\n"<<std::flush;
        auto weights=transpose_functional(E,top);check_functional(E,weights,top);const auto after_solve=counters;
        Z product=1;for(const auto& w:weights)product=mod(product*w);require(product!=0,"weight product zero");Z inverse=inv(product);require(mod(product*inverse)==1,"weight nonzero certificate");
        std::vector<std::vector<Z>> weighted=E;for(size_t i=0;i<FN;++i)for(size_t j=0;j<FN;++j)weighted[i][j]=mod(weights[i]*E[i][j]);
        size_t pairs=0;
        for(size_t i=0;i<FN;++i)for(size_t j=i;j<FN;++j)if(md(basis.standard[i])+md(basis.standard[j])<97){Z sum=0;for(size_t p=0;p<FN;++p)sum+=weighted[p][i]*E[p][j];require(mod(sum)==0,"low-degree pairing not zero");++pairs;}
        const auto after_pairs=counters;size_t corruptions=0;auto reject=[&](const auto& f){bool rejected=false;try{f();}catch(const std::exception&){rejected=true;}require(rejected,"corrupt certificate accepted");++corruptions;};
        auto bad_weights=weights;bad_weights[0]=mod(bad_weights[0]+1);reject([&]{check_functional(E,bad_weights,top);});
        auto bad_h=h;bad_h[0]+=1;reject([&]{complementary_dimensions(bad_h,97);});
        OpenSSLCheck ssl(TARGET);size_t group_checks=0;std::vector<std::string> hypotheses,bounds;
        for(const auto& c:cells){hypotheses.push_back(verify_cell(c,points,ssl,group_checks));IntegerPolynomial dense_tensor{1},chain_tensor{1};
            for(unsigned j=0;j<c.m;++j){dense_tensor=iconvolve(dense_tensor,h);chain_tensor=iconvolve(chain_tensor,chain_h);}
            unsigned rd=static_cast<unsigned>(dense_tensor.size()-1),rc=static_cast<unsigned>(chain_tensor.size()-1),upper=rc+c.e;
            require(rd==97*c.m && rc==5*c.m && 2*upper<=rd,"twofold bound arithmetic");
            bounds.push_back(object({{"m",num(c.m)},{"kind",jquote(c.kind)},{"dense_D_lower",num(rd)},
              {"chain_D_upper",num(upper)},{"group_degree",num(c.e)},{"extra_affine_variables",num(7*c.m)},
              {"numeric_twofold_inequality","true"},{"status",jquote("derived full-system bounds under the written tensorization/H-torsion proof; unsigned")}}));
        }
        write(out/"weighted-pairing.json",object({{"factor_base",jquote("FB1h16882188194e")},{"points",num(FN)},
          {"point_certificate_sha256",jquote(dense_sha)},{"quotient_regularity","97"},{"unique_top_standard_index",num(top)},
          {"weights",integer_json(weights)},{"weight_product",dec(product)},{"weight_product_inverse",dec(inverse)},
          {"evaluation_stream_sha256",jquote(hash(evaluation_stream))},{"evaluation_stream_order",jquote("point-major/basis-minor decimal entries each followed by LF")},
          {"functional_column_equations_checked",num(FN)},{"unordered_low_degree_products_checked",num(pairs)},
          {"all_ordered_low_degree_products_follow_by_commutativity","true"},{"complementary_dimension_cutoffs","98"},
          {"point_Hilbert_numerator",integer_json(h)},{"corrupt_controls_rejected",num(corruptions)},
          {"standing",jquote("unsigned finite algebraic certificate; no independent signature or Lean theorem")}}));
        write(out/"cell-hypotheses.json",array(hypotheses));write(out/"full-degree-bounds.json",array(bounds));
        write(out/"manifest.json",object({{"cache_sha256",jquote(hash(read(argv[1])))},{"no_two_torsion_coefficient_hash",jquote(no2)},
          {"cell_hypotheses_checked",num(cells.size())},{"GMP_OpenSSL_group_checks",num(group_checks)},
          {"point_check_mod_API_calls",num(after_point.reductions-before.reductions)},
          {"functional_solve_mod_API_calls",num(after_solve.reductions-after_point.reductions)},
          {"weighted_pair_check_mod_API_calls",num(after_pairs.reductions-after_solve.reductions)},
          {"hypothesis_check_mod_API_calls",num(counters.reductions-after_pairs.reductions)},
          {"counter_scope",jquote("native certificate API calls, not field-equivalent solving work or a runtime comparison")},
          {"independent_signed_Lean_admission","false"},{"full_higher_m_relation_search","false"},{"goal_achieved","false"}}));
        std::cout<<"Finite pairing and all12degree-bound hypotheses pass; unsigned.\n";
    }catch(const std::exception& error){std::cerr<<"frobenius: "<<error.what()<<'\n';return 1;}
}
