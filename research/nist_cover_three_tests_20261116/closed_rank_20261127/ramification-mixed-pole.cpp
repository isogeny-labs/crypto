#include "ramification-arithmetic.hpp"
#include "ramification-support.hpp"
#include "ramification-pole-engine.hpp"

struct PoleTarget {std::string name;SCRow reference;audit::Point target,half;};
void emit_pole_target(const PoleTarget& item) {
    std::cout<<"{\"divisor\":\""<<item.name<<"\",\"reference\":";ec_emit_sparse(item.reference);
    std::cout<<",\"target\":";ramification::emit_ap(item.target);
    std::cout<<",\"half\":";ramification::emit_ap(item.half);std::cout<<'}';
}
void require_pole_families(const std::vector<int>& families) {
    require(families==std::vector<int>({0,1,2,3}),"all four result families present exactly once in order");
}
void pole_controls() {
    shifted::controls();
    audit::SSL ssl;auto generator=EC_GROUP_get0_generator(ssl.group.get());audit::BN x(BN_new()),y(BN_new());
    require(EC_POINT_get_affine_coordinates(ssl.group.get(),generator,x.get(),y.get(),ssl.context.get())==1,"auxiliary fixture generator");
    auto integer=[](const audit::BN& n){char* t=BN_bn2dec(n.get());require(t,"fixture coordinate");audit::Big v(t);OPENSSL_free(t);return v;};
    const audit::Point g{false,integer(x),integer(y)};
    const std::vector<audit::Point> symbols{g,audit::plus(g,g)};
    auto full=symbols;full.push_back(norm_test::multiply(g,3));
    const ECSparse reference{{2,2}};const auto target=norm_test::multiply(g,6),half=norm_test::multiply(g,3);
    auto table=shifted::enumerate(symbols,half);shifted::Counts a,b;
    const auto first=shifted::grouped(table,reference,a),second=shifted::merged(table,reference,b);
    require(first==second&&!first.empty(),"nonempty auxiliary-reference joins agree");
    shifted::Found brute;shifted::Word word{};
    std::function<void(size_t,int,audit::Point)> visit=[&](size_t slot,int minimum,audit::Point sum) {
        if(slot==6){if(sum==target)shifted::keep(brute,word,reference);return;}
        for(int id=minimum;id<5;++id){word[slot]=id;visit(slot+1,id,audit::plus(sum,shifted::symbol(id,symbols)));}
    };
    visit(0,0,{});require(first==brute,"auxiliary-reference joins match exhaustive small words");
    shifted::verify(first,reference,full,target);
    SCMat rows{{-2,1,0}};
    for(const auto& [row,w]:first){(void)w;rows.push_back(norm_test::dense(row,3));}
    SCMat aux;for(const auto& row:rows)aux.push_back(SCRow{row[2]});
    require(sc_rank(rows)==2&&sc_rank(aux)==1,"auxiliary fixture full2aux1closed1");
    const SCRow unit{1,0,0};auto numerical=rows;numerical.push_back(unit);
    require(sc_rank(numerical)==3&&!norm_test::row_sum(unit,full).zero&&!shifted::named_sum(unit,full,{},ssl),
            "isolated numerical old-unit fixture closed2 fails groups");
    auto bad=norm_test::dense(reference,3);++bad[2];
    norm_test::rejects_norm([&]{require(norm_test::row_sum(bad,full)==target&&shifted::named_sum(bad,full,target,ssl),"changed auxiliary reference coefficient");});
    auto missing=first;missing.erase(missing.begin());norm_test::rejects_norm([&]{require(missing==brute,"missing auxiliary fixture candidate");});
}
using ComponentWeights=std::array<int,3>;
const std::array<ComponentWeights,4> pole_component_weights{{{2,0,0},{0,2,0},{0,0,1},{1,1,0}}};
void require_degree_two_inventory(const std::vector<ComponentWeights>& provided,const std::array<int,3>& degree) {
    std::set<ComponentWeights> expected;
    for(int a=0;a<=2;++a)for(int b=0;b<=2;++b)for(int c=0;c<=2;++c)
        if(a*degree[0]+b*degree[1]+c*degree[2]==2)expected.insert({a,b,c});
    const std::set<ComponentWeights> actual(provided.begin(),provided.end());
    require(provided.size()==actual.size()&&actual==expected&&actual.size()==4,
            "complete distinct degree-two ramification-supported inventory");
}
void emit_component_inventory() {
    std::cout<<'[';for(size_t i=0;i<pole_component_weights.size();++i) {
        if(i)std::cout<<',';
        std::cout<<'[';for(size_t j=0;j<3;++j)std::cout<<(j?",":"")<<pole_component_weights[i][j];std::cout<<']';
    }std::cout<<']';
}
void require_mixed_mode(const std::string& mode) {
    require(mode=="plan"||mode=="3"||mode=="combine","only plan,newfamily3,combine are executable");
}
void mixed_pole_controls() {
    pole_controls();
    for(const std::string mode:{"0","1","2"})norm_test::rejects_norm([&]{require_mixed_mode(mode);});
    audit::SSL ssl;auto generator=EC_GROUP_get0_generator(ssl.group.get());audit::BN x(BN_new()),y(BN_new());
    require(EC_POINT_get_affine_coordinates(ssl.group.get(),generator,x.get(),y.get(),ssl.context.get())==1,"two-aux fixture generator");
    auto integer=[](const audit::BN& n){char* t=BN_bn2dec(n.get());require(t,"fixture coordinate");audit::Big v(t);OPENSSL_free(t);return v;};
    const audit::Point g{false,integer(x),integer(y)};
    const std::vector<audit::Point> symbols{g,norm_test::multiply(g,2)};
    auto full=symbols;full.push_back(norm_test::multiply(g,3));full.push_back(norm_test::multiply(g,4));
    const ECSparse reference{{0,1},{1,3},{2,1},{3,-1}};
    const auto target=norm_test::multiply(g,6),half=norm_test::multiply(g,3);
    auto table=shifted::enumerate(symbols,half);shifted::Counts a,b;
    const auto first=shifted::grouped(table,reference,a),second=shifted::merged(table,reference,b);
    require(!first.empty()&&first==second,"signed two-aux joins agree");
    shifted::Found brute;shifted::Word word{};
    std::function<void(size_t,int,audit::Point)> visit=[&](size_t slot,int minimum,audit::Point sum) {
        if(slot==6){if(sum==target)shifted::keep(brute,word,reference);return;}
        for(int id=minimum;id<5;++id){word[slot]=id;visit(slot+1,id,audit::plus(sum,shifted::symbol(id,symbols)));}
    };visit(0,0,{});
    require(first==brute,"signed two-aux joins equal exhaustive words");shifted::verify(first,reference,full,target);
    SCMat rows{{-2,1,0,0}};
    for(const auto& [row,witness]:first){(void)witness;rows.push_back(norm_test::dense(row,4));}
    SCMat auxiliary;for(const auto& row:rows)auxiliary.push_back(SCRow{row[2],row[3]});
    require(sc_rank(rows)==2&&sc_rank(auxiliary)==1,"two-aux full2aux1closed1");
    auto numerical=rows;const SCRow unit{1,0,0,0};numerical.push_back(unit);
    require(sc_rank(numerical)==3&&!norm_test::row_sum(unit,full).zero&&!shifted::named_sum(unit,full,{},ssl),
            "two-aux numerical old-unit closed2 is not an actual row");
    for(size_t column:{size_t(2),size_t(3)}) {
        auto omitted=norm_test::dense(reference,4);omitted[column]=0;
        auto flipped=norm_test::dense(reference,4);flipped[column]=-flipped[column];
        norm_test::rejects_norm([&]{require(norm_test::row_sum(omitted,full)==target&&shifted::named_sum(omitted,full,target,ssl),"omitted signed auxiliary");});
        norm_test::rejects_norm([&]{require(norm_test::row_sum(flipped,full)==target&&shifted::named_sum(flipped,full,target,ssl),"changed signed auxiliary");});
    }
}
int main(int argc,char** argv)try {
    require(argc==10||argc==14,"QUARTIC BACKMAP BASELINE TANGENTS SCAN EXPANDED RAMIFICATION SHIFTED MODE [FOUR_RESULTS]");
    const std::string mode=argv[9];require_mixed_mode(mode);
    require((mode=="combine"&&argc==14)||(mode!="combine"&&argc==10),"complete mode-specific arguments");
    const auto model=read_json(argv[1]),back=read_json(argv[2]),baseline=read_json(argv[3]),
               tangents=read_json(argv[4]),scan=read_json(argv[5]),expanded=read_json(argv[6]);
    ss_private(baseline,tangents);
    require(expanded.at("controls_pass").scalar=="true"&&expanded.at("closed_rank").integer()==6&&
            expanded.at("expanded_curve_pool").integer()==1434&&expanded.at("selected_sections").integer()==128,
            "successful frozen expansion dependency");
    const auto curve=read_bp(model.at("terms"),true);
    std::array<BP,4> map;size_t position=0;
    for(const auto* name:{"X_numerator","X_denominator","Y_numerator","Y_denominator"})map[position++]=read_bp(back.at(name));
    require(global_identity(curve,map),"actual expanded-conic map identity");
    std::vector<F> leading(5);
    for(const auto& [e,a]:curve.t)if(e[0]+e[1]==4)leading[size_t(e[1])]=a;
    const Poly infinity(std::move(leading)),z{F(0),F(1)};
    require(gcd(infinity,wide_power(z,prime(),infinity)-z).degree()==0,"no omitted rational source infinity");
    std::vector<::Point> base;
    std::vector<audit::Point> gmp_base,original_base;
    std::set<std::string> column_x;
    for(const auto& q:tangents.at("projected_base").arr) {
        const auto p=point(q);valid(p);
        require(!p.zero&&BN_cmp(p.y.p.at(0).n.get(),(-p.y.p.at(0)).n.get())<=0&&
                column_x.insert(decimal(p.x.p.at(0))).second,"original429canonical image columns");
        original_base.push_back(ec_gmp(p));
        if(base.size()<225){base.push_back(p);gmp_base.push_back(ec_gmp(p));}
    }
    require(original_base.size()==429,"original full image width");
    for(size_t i=0;i<225;++i)require(base[i]==point(baseline.at("projected_base").arr.at(i)),"old225prefix unchanged");
    require(expanded.at("selected_images").arr.size()==256&&expanded.at("fibres").arr.size()==256,
            "all256new image/fibre records");
    for(const auto& q:expanded.at("selected_images").arr) {
        const auto p=point(q);valid(p);
        require(!p.zero&&BN_cmp(p.y.p.at(0).n.get(),(-p.y.p.at(0)).n.get())<=0&&
                column_x.insert(decimal(p.x.p.at(0))).second,"new canonical auxiliary image column");
        base.push_back(p);gmp_base.push_back(ec_gmp(p));
    }
    require(base.size()==481,"expanded geometric image width");
    std::vector<CPoint> points;
    std::map<std::string,int> coordinates;
    auto append=[&](const Json& q,int index,int sign) {
        const auto x=coefficient(q.at("x")),y=coefficient(q.at("y"));
        const auto actual=fs_image(curve,map,x,y).value;
        require(actual==point(q.at("image"))&&coordinates.emplace(sp_key(x,y),int(points.size())).second,
                "fresh distinct expanded conic-pool image");
        if(index<0)require(index==-1&&sign==0&&actual.zero,"identity pool label");
        else {
            require(size_t(index)<base.size()&&(sign==1||sign==-1),"signed pool label bounds");
            auto expected=base.at(size_t(index));if(sign<0)expected.y=-expected.y;
            require(actual==expected,"pool exact canonical image/sign");
        }
        const auto fx=fs_eval(curve.partial(0),x,y),fy=fs_eval(curve.partial(1),x,y);
        require(fx!=F(0)||fy!=F(0),"smooth expanded pool point");
        points.push_back({{x,y,actual,index,sign},{F(1),x,y,x*x,x*y,y*y},
                          {F(0),fy,-fx,F(2)*x*fy,y*fy-x*fx,-F(2)*y*fx}});
    };
    for(const auto& q:scan.at("points").arr) {
        require(q.at("index").integer()<225,"old scan uses only original image domain");
        append(q,q.at("index").integer(),q.at("sign").integer());
    }
    require(points.size()==652,"unchanged old pool prefix");
    for(size_t i=0;i<256;++i) {
        const auto& fibre=expanded.at("fibres").arr.at(i);
        require(fibre.at("target").integer()==int(i),"ordered frozen fibre inventory");
        for(const auto& q:fibre.at("points").arr) {
            const auto claimed=point(q.at("image"));
            require(!claimed.zero&&claimed.x==base[225+i].x,"exact new fibre image x");
            append(q,int(225+i),claimed.y==base[225+i].y?1:-1);
        }
    }
    require(points.size()==1434&&points.size()<=3000,"complete bounded actual expanded pool");
    for(size_t i=0;i<226;++i) {
        const auto& q=baseline.at("pool").at("points").arr.at(i);
        require(decimal(points[i].value.x)==q.at("x").scalar&&decimal(points[i].value.y)==q.at("y").scalar&&
                points[i].value.image==point(baseline.at("images").arr.at(i).at("point")),"original226bindings");
    }
    std::vector<Anchors> known;
    std::vector<BP> lines;
    for(const auto& fibre:baseline.at("pool").at("split_fibers").arr) {
        Anchors ids{};
        require(fibre.at("point_indices").arr.size()==4,"original split degree4");
        for(size_t i=0;i<4;++i)ids[i]=fibre.at("point_indices").arr[i].integer();
        const auto& a=points.at(size_t(ids[0])).value;
        const auto& b=points.at(size_t(ids[1])).value;
        BP line;line.term({0,0},b.y*a.x-b.x*a.y);line.term({1,0},a.y-b.y);line.term({0,1},b.x-a.x);
        for(int id:ids)require(fs_eval(line,points.at(size_t(id)).value.x,points.at(size_t(id)).value.y)==F(0),
                               "original split line incidence");
        known.push_back(ids);lines.push_back(std::move(line));
    }
    require(known.size()==7,"original seven complete sections");
    SCRow reference(481);::Point kappa;
    for(int id:known[0]) {
        const auto& p=points.at(size_t(id)).value;kappa=add(kappa,p.image);
        if(p.index>=0)reference[size_t(p.index)]+=p.sign;
    }
    audit::SSL ssl;
    auto check_row=[&](const SCRow& row) {
        require(norm_test::row_sum(row,gmp_base).zero&&ec_ssl_zero(ec_sparse(row),gmp_base,ssl),
                "independent GMP/named conic or seed row sum");
    };
    std::vector<SCRow> line_rows;
    for(const auto& ids:known) {
        SCRow row(481);
        for(int id:ids){const auto& p=points.at(size_t(id)).value;if(p.index>=0)row[size_t(p.index)]+=p.sign;}
        for(size_t i=0;i<row.size();++i)row[i]-=reference[i];
        check_row(row);line_rows.push_back(std::move(row));
    }
    auto matrix=sc_read_matrix(tangents.at("matrix"));
    for(auto& row:matrix) {
        require(norm_test::row_sum(row,original_base).zero&&ec_ssl_zero(ec_sparse(row),original_base,ssl),
                "original108row sums in both implementations");
        row.resize(685);
    }
    std::set<ECSparse> seed_rows,allowed_rows;
    allowed_rows.insert(ECSparse{});
    for(const auto& row:baseline.at("rows").arr) {
        const auto sparse=ec_read_sparse(row,225);
        allowed_rows.insert(sparse);allowed_rows.insert(norm_test::opposite(sparse));
    }
    for(const auto& saved:expanded.at("seed_sections").arr) {
        const int i=saved.at("i").integer(),j=saved.at("j").integer();
        require(i>=0&&j>=0&&i<652&&j<652,"saved seed origins in old pool");
        const bool tangent=saved.at("tangent").scalar=="true";
        require(tangent?i==j:i<j,"saved seed secant/tangent order");
        const auto& a=points[size_t(i)].value;const auto& b=points[size_t(j)].value;
        const auto dx=coefficient(saved.at("direction").arr[0]),dy=coefficient(saved.at("direction").arr[1]);
        require(tangent?(dx==fs_eval(curve.partial(1),a.x,a.y)&&dy==-fs_eval(curve.partial(0),a.x,a.y)):
                        (dx==b.x-a.x&&dy==b.y-a.y),"saved seed direction binding");
        const auto restriction=ss_restrict(curve,a.x,a.y,dx,dy);
        require(restriction==read_poly(saved.at("restriction")),"actual saved seed restriction");
        const auto roots=ss_quadratic(restriction.exact(tangent?Poly{F(0),F(0),F(1)}:Poly{F(0),F(-1),F(1)}));
        require(roots.size()==2,"saved split residual factor product");
        std::array<std::string,2> computed{sp_key(a.x+dx*roots[0],a.y+dy*roots[0]),sp_key(a.x+dx*roots[1],a.y+dy*roots[1])},recorded;
        std::array<int,4> ids{i,j,0,0};
        for(size_t k=0;k<2;++k) {
            const auto& q=saved.at("residual_points").arr.at(k);
            recorded[k]=q.at("x").scalar+":"+q.at("y").scalar;
            ids[2+k]=coordinates.at(recorded[k]);
            require(points[size_t(ids[2+k])].value.image==point(q.at("image")),"saved residual exact image");
        }
        std::sort(computed.begin(),computed.end());std::sort(recorded.begin(),recorded.end());
        require(computed==recorded,"saved residual geometry and factors agree");
        SCRow row(481);
        for(int id:ids){const auto& p=points[size_t(id)].value;if(p.index>=0)row[size_t(p.index)]+=p.sign;}
        for(size_t k=0;k<row.size();++k)row[k]-=reference[k];
        const auto sparse=ec_sparse(row);
        require(sparse==ec_read_sparse(saved.at("row"),481)&&seed_rows.insert(sparse).second,"saved seed row exact binding");
        check_row(row);matrix.push_back(ec_embed(row));allowed_rows.insert(sparse);
        require(!tangent,"declared mixed-image family uses split secant seeds");
        std::set<int> distinct(ids.begin(),ids.end());require(distinct.size()==4,"four distinct known seed support points");
        BP line;line.term({0,0},dy*a.x-dx*a.y);line.term({1,0},-dy);line.term({0,1},dx);
        for(int id:ids)require(fs_eval(line,points[size_t(id)].value.x,points[size_t(id)].value.y)==F(0),"known seed line incidence");
        known.push_back(ids);lines.push_back(std::move(line));line_rows.push_back(row);
    }
    require(seed_rows.size()==128&&matrix.size()==236,"all128actual seed rows");
    std::set<ECSparse> norm_rows;
    for(const auto& row:expanded.at("norm_rows").arr) {
        const auto sparse=ec_read_sparse(row.at("row"),481);
        require(allowed_rows.count(sparse)&&norm_rows.insert(sparse).second,"saved norm row belongs to seed/baseline span");
    }
    require(norm_rows.size()==135,"all saved necessary norm rows bound");
    for(const auto& row:seed_rows)require(norm_rows.count(row),"all seeds in saved necessary norm inventory");
    require(sc_rank(matrix)==236&&ec_aux_rank(matrix)==230,"starting full236aux230closed6");


    const auto ram=read_json(argv[7]),shifted_prior=read_json(argv[8]);
    require(ram.at("controls_pass").scalar=="true"&&ram.at("full_rank").integer()==237&&
            ram.at("auxiliary_rank").integer()==231&&ram.at("closed_rank").integer()==6&&
            ram.at("branches").arr.size()==3&&ram.at("new_auxiliary_columns").integer()==3,
            "completed exact ramification certificate dependency");
    require(shifted_prior.at("controls_pass").scalar=="true"&&shifted_prior.at("closed_rank").integer()==6&&
            shifted_prior.at("distinct_rows_including_zero").integer()==1,"completed shifted-six dependency");
    const auto line=read_bp(back.at("L"));const auto ell=ramification::linear_coefficients(line);
    require(ell[1]!=F(0),"retained ramification-line parameterization");
    const F intercept=-ell[2]/ell[1],slope=-ell[0]/ell[1];
    const auto restriction=ss_restrict(curve,F(0),intercept,F(1),slope);
    require(restriction==read_poly(ram.at("affine_restriction"))&&restriction.degree()==4&&
            restriction.monic()==read_poly(ram.at("monic_restriction")),"exact ramification support restriction binding");
    std::vector<Poly> factors;std::array<audit::Point,3> traces;std::array<int,3> component_degrees{};
    for(size_t i=0;i<3;++i) {
        const auto& saved=ram.at("branches").arr[i];const auto factor=read_poly(saved.at("factor"));
        const int degree=i<2?1:2;component_degrees[i]=saved.at("source_degree").integer();
        require(factor.degree()==degree&&saved.at("source_degree").integer()==degree&&saved.at("image_degree").integer()==degree&&
                saved.at("multiplicity").integer()==1&&saved.at("conjugate_checked").scalar=="true","ordered simple ramification degrees1/1/2");
        irreducible(factor);factors.push_back(factor);residue_modulus=factor;
        const K x(read_poly(saved.at("source_x_coefficients"))),y(read_poly(saved.at("source_y_coefficients")));
        require(x==K(Poly{F(0),F(1)})&&y==K(intercept)+K(slope)*x&&
                evaluate(homogenize(curve,4),{R{x},R{y},R{K(1)}}).c.empty(),"actual closed ramification source support");
        const auto& image=saved.at("local").at("image");
        require(image.at("infinity").scalar=="false","finite retained branch image");
        const ::Point endpoint(K(read_poly(image.at("x_coefficients"))),K(read_poly(image.at("y_coefficients"))));valid(endpoint);
        if(degree==1) {
            const auto fresh=ramification::local_map(curve,map,line,x,y);
            require(fresh.image==endpoint&&fresh.index==2&&fresh.line_order==1,"fresh rational ramification pole map and removable-value resolution");
        } else require(!(ramification::frobenius(endpoint)==endpoint),"quadratic image exact field degree");
        auto bad=endpoint;bad.y=bad.y==K(0)?K(1):K(0);norm_test::rejects_norm([&]{valid(bad);});
        ::Point trace,current=endpoint;
        for(int j=0;j<degree;++j){trace=add(trace,current);current=ramification::frobenius(current);}
        require(current==endpoint&&(trace.zero||(trace.x.p.degree()<=0&&trace.y.p.degree()<=0)),"full source-degree rational pole trace");
        traces[i]=ec_gmp(trace);require(!traces[i].zero&&traces[i]==audit::read_point(saved.at("rational_trace")),"saved rational pole trace binding");
        (void)ssl.make(traces[i]);
    }
    residue_modulus=Poly{F(0),F(1)};verify_factor_product(factors,restriction.monic());
    const auto kappa_gmp=ec_gmp(kappa);
    require(audit::plus(audit::plus(traces[0],traces[1]),traces[2])==kappa_gmp,"complete ramification trace sum");
    norm_test::rejects_norm([&]{require(audit::plus(audit::plus(audit::negative(traces[0]),traces[1]),traces[2])==kappa_gmp,"mutated trace sign");});
    norm_test::rejects_norm([&]{require(audit::plus(audit::plus(traces[0],traces[1]),norm_test::multiply(traces[2],2))==kappa_gmp,"quadratic trace weighted twice");});
    std::vector<audit::Point> full_base=std::move(original_base);
    full_base.insert(full_base.end(),gmp_base.begin()+225,gmp_base.end());
    require(full_base.size()==685,"all retained images present");
    std::array<int,3> signs{};
    for(size_t i=0;i<3;++i) {
        auto canonical=traces[i];signs[i]=1;
        if(canonical.y>audit::mod(-canonical.y)){canonical=audit::negative(canonical);signs[i]=-1;}
        require(std::find(full_base.begin(),full_base.end(),canonical)==full_base.end(),"fresh distinct ramification trace image");
        const auto& saved=ram.at("branches").arr[i];
        require(saved.at("trace_column").integer()==int(685+i)&&saved.at("trace_sign").integer()==signs[i]&&
                canonical==audit::read_point(ram.at("fresh_auxiliary_images").arr.at(i)),"exact signed trace-column binding");
        full_base.push_back(canonical);
    }
    auto hword=ec_embed(reference);hword.resize(688);
    SCRow ram_row=hword;for(auto& c:ram_row)c=-c;
    for(size_t i=0;i<3;++i)ram_row[685+i]+=signs[i];
    require(ec_sparse(ram_row)==ec_read_sparse(ram.at("principal_row"),688)&&norm_test::row_sum(ram_row,full_base).zero&&
            ec_ssl_zero(ec_sparse(ram_row),full_base,ssl),"actual Ram-H row bound in both groups");
    for(auto& row:matrix)row.resize(688);
    matrix.push_back(ram_row);
    require(sc_rank(matrix)==237&&ec_aux_rank(matrix)==231,"actual full688matrix237/231/6 retained");
    std::array<PoleTarget,4> targets;
    const SCBig half_scalar=(sc_n+1)/2;
    for(size_t i=0;i<4;++i) {
        auto& item=targets[i];item.name=i==0?"H+2B1":i==1?"H+2B2":i==2?"H+Q":"H+B1+B2";item.reference=hword;
        if(i<3) {
            const int multiplicity=i<2?2:1;item.reference[685+i]+=multiplicity*signs[i];
            item.target=audit::plus(kappa_gmp,norm_test::multiply(traces[i],multiplicity));
        } else {
            item.reference[685]+=signs[0];item.reference[686]+=signs[1];
            item.target=audit::plus(kappa_gmp,audit::plus(traces[0],traces[1]));
        }
        item.half=norm_test::multiply(item.target,half_scalar);
        require(!item.target.zero&&norm_test::row_sum(item.reference,full_base)==item.target&&
                shifted::named_sum(item.reference,full_base,item.target,ssl)&&audit::plus(item.half,item.half)==item.target,
                "actual full-width pole reference and half-target");
        auto named_target=ssl.make(item.target),named_half=ssl.make({});auto scalar=audit::bn(half_scalar);
        require(EC_POINT_mul(ssl.group.get(),named_half.get(),nullptr,named_target.get(),scalar.get(),ssl.context.get())==1&&
                ssl.encode(named_half.get())==audit::key(item.half),"named OpenSSL half-target");
    }
    require(targets[3].target==audit::plus(norm_test::multiply(kappa_gmp,2),audit::negative(traces[2])),
            "mixed target also equals2kappa-minus-quadratic-trace");
    std::vector<ComponentWeights> inventory(pole_component_weights.begin(),pole_component_weights.end());
    require_degree_two_inventory(inventory,component_degrees);
    auto missing_inventory=inventory;missing_inventory.pop_back();
    norm_test::rejects_norm([&]{require_degree_two_inventory(missing_inventory,component_degrees);});
    auto duplicate_inventory=inventory;duplicate_inventory.push_back(inventory[0]);
    norm_test::rejects_norm([&]{require_degree_two_inventory(duplicate_inventory,component_degrees);});
    const auto old_shifted=audit::read_point(shifted_prior.at("target"));
    require(old_shifted==norm_test::multiply(audit::read_point(shifted_prior.at("rho")),2),"old2rho target bound");
    const std::array<audit::Point,5> previous{{{},kappa_gmp,norm_test::multiply(kappa_gmp,2),norm_test::multiply(kappa_gmp,3),old_shifted}};
    for(size_t i=0;i<4;++i) {
        for(const auto& old:previous)require(!(targets[i].target==old)&&!(targets[i].target==audit::negative(old)),"new pole target overlaps completed signed bound; reuse required");
        for(size_t j=0;j<i;++j)require(!(targets[i].target==targets[j].target)&&!(targets[i].target==audit::negative(targets[j].target)),"pole targets distinct up to sign");
    }
    auto bad_reference=targets[3].reference;++bad_reference[686];
    norm_test::rejects_norm([&]{require(norm_test::row_sum(bad_reference,full_base)==targets[3].target&&
                shifted::named_sum(bad_reference,full_base,targets[3].target,ssl),"changed actual pole reference coefficient");});
    mixed_pole_controls();
    std::cerr<<"REBOUND 1434 points,688 columns,237/231/6; four signed-distinct targets;onlynewfamily3enumerated; controls pass\n";
    auto read_witnesses=[&](const Json& record,size_t family) {
        require(record.at("controls_pass").scalar=="true"&&record.at("family").integer()==int(family)&&
                record.at("unordered_triples").integer()==int(shifted::expected_count(451))&&
                audit::read_point(record.at("target"))==targets[family].target&&
                record.at("independent_triple_streams_match").scalar=="true"&&record.at("independent_join_sets_match").scalar=="true",
                "complete matching family receipt");
        shifted::Found found;
        for(const auto& saved:record.at("rows").arr) {
            shifted::Witness witness;require(saved.at("labels").arr.size()==6,"complete six-label witness");
            for(size_t i=0;i<6;++i){witness.labels[i]=saved.at("labels").arr[i].integer();require(witness.labels[i]>=0&&witness.labels[i]<=450,"old signed witness label");}
            require(std::is_sorted(witness.labels.begin(),witness.labels.end()),"canonical sorted six-word witness");
            witness.sign=saved.at("overall_sign").integer();
            const auto row=ec_read_sparse(saved.at("row"),688);
            require(found.emplace(row,witness).second,"distinct candidate rows in family receipt");
        }
        require(found.size()==size_t(record.at("distinct_rows_including_zero").integer()),"all family rows retained");
        shifted::verify(found,ec_sparse(targets[family].reference),full_base,targets[family].target);
        return found;
    };
    std::vector<audit::Point> old_symbols(full_base.begin(),full_base.begin()+225);
    std::array<shifted::Found,4> family_rows;
    shifted::Counts grouped_counts,merged_counts;
    std::string stream_hash;size_t triples=0,selected=4;
    if(mode=="plan") {
        std::cout<<"{\"schema\":\"p192-ramification-mixed-pole-plan-v1\",\"controls_pass\":true,\"targets_new_up_to_sign\":true,\"targets\":[";
        for(size_t i=0;i<4;++i){if(i)std::cout<<',';emit_pole_target(targets[i]);}
        std::cout<<"],\"degree_two_support_inventory\":";emit_component_inventory();
        std::cout<<",\"full_rank\":237,\"auxiliary_rank\":231,\"closed_rank\":6,\"columns\":688,\"auxiliary_columns\":463,\"pool_size\":1434}\n";
        return 0;
    }
    if(mode=="combine") {
        std::array<Json,4> saved;
        for(size_t i=0;i<4;++i){saved[i]=read_json(argv[10+i]);family_rows[i]=read_witnesses(saved[i],i);}
        std::vector<int> ids;for(const auto& record:saved)ids.push_back(record.at("family").integer());
        require_pole_families(ids);ids.pop_back();
        norm_test::rejects_norm([&]{require_pole_families(ids);});
    } else {
        require(mode=="3","only the new mixed family3 may enumerate");selected=3;
        auto table=shifted::enumerate(old_symbols,targets[selected].half);triples=table.rows.size();stream_hash=table.digest;
        family_rows[selected]=shifted::grouped(table,ec_sparse(targets[selected].reference),grouped_counts);
        const auto checked=shifted::merged(table,ec_sparse(targets[selected].reference),merged_counts);
        require(family_rows[selected]==checked&&grouped_counts.opposite_pairs==merged_counts.opposite_pairs&&
                grouped_counts.identity_pairs==merged_counts.identity_pairs,"both complete joins/counts/minimal witnesses agree");
        shifted::verify(family_rows[selected],ec_sparse(targets[selected].reference),full_base,targets[selected].target);
    }
    MIBasis candidates(matrix);SCMat candidate_matrix=matrix;size_t independent=0,total_rows=0;
    for(size_t i=0;i<4;++i)for(const auto& [row,witness]:family_rows[i]) {
        (void)witness;const auto dense=norm_test::dense(row,688);candidate_matrix.push_back(dense);++total_rows;
        require(candidate_matrix.size()<=4096,"candidate row cap");
        if(!candidates.representation(dense).first){candidates.append(dense,"necessary:"+targets[i].name+":"+std::to_string(independent));++independent;}
    }
    const auto full=sc_rank(candidates.source),auxiliary=ec_aux_rank(candidates.source);
    require(full==237+independent&&full>=auxiliary+6,"candidate closure retains actual baseline");
    if(candidate_matrix.size()<=256)require(sc_rank(candidate_matrix)==full&&ec_aux_rank(candidate_matrix)==auxiliary,"direct uncompressed candidate ranks agree");
    std::cout<<"{\"schema\":\"p192-ramification-mixed-pole-v1\",\"phase\":\""<<mode<<"\",\"controls_pass\":true,\"targets_new_up_to_sign\":true,"
             <<"\"pool_size\":1434,\"matrix_columns\":688,\"active_auxiliary_columns\":463,\"actual_full_rank\":237,\"actual_auxiliary_rank\":231,\"actual_closed_rank\":6,"
             <<"\"full_rank\":"<<full<<",\"auxiliary_rank\":"<<auxiliary<<",\"closed_rank\":"<<full-auxiliary<<",\"candidate_gain\":"<<full-auxiliary-6
             <<",\"independent_added_rows\":"<<independent<<",\"distinct_candidate_rows\":"<<total_rows<<",\"rr_dimension\":4,\"targets\":[";
    for(size_t i=0;i<4;++i){if(i)std::cout<<',';emit_pole_target(targets[i]);}
    if(selected<4) {
        std::cout<<"],\"family\":"<<selected<<",\"divisor\":\""<<targets[selected].name<<"\",\"target\":";ramification::emit_ap(targets[selected].target);
        std::cout<<",\"unordered_triples\":"<<triples<<",\"triple_stream_sha256\":\""<<stream_hash
                 <<"\",\"packed_record_bytes\":32,\"table_payload_bytes\":"<<triples*sizeof(shifted::Packed)<<",\"join_index_payload_bytes\":"<<merged_counts.index_payload
                 <<",\"independent_triple_streams_match\":true,\"independent_join_sets_match\":true,\"opposite_orientation_pairs\":"<<grouped_counts.opposite_pairs
                 <<",\"identity_pairs\":"<<grouped_counts.identity_pairs<<",\"distinct_rows_including_zero\":"<<family_rows[selected].size()<<",\"rows\":[";
        bool comma=false;
        for(const auto& [row,w]:family_rows[selected]) {
            if(comma)std::cout<<',';
            comma=true;std::cout<<"{\"labels\":[";
            for(size_t i=0;i<6;++i)std::cout<<(i?",":"")<<w.labels[i];
            std::cout<<"],\"overall_sign\":"<<w.sign<<",\"row\":";ec_emit_sparse(norm_test::dense(row,688));std::cout<<'}';
        }
    } else {
        std::cout<<"],\"family_count\":4,\"family_row_counts\":[";
        for(size_t i=0;i<4;++i)std::cout<<(i?",":"")<<family_rows[i].size();
    }
    std::cout<<"],\"auxiliary_fixture\":{\"full_rank\":2,\"auxiliary_rank\":1,\"closed_rank\":1,\"numerical_mutant_closed_rank\":2},"
             <<"\"fixture_rows_in_actual_matrix\":0,\"completed_target_enumerations_rerun\":0,\"degree_two_support_inventory_complete\":true,\"only_new_family_executable\":true,\"two_auxiliary_fixture\":{\"full_rank\":2,\"auxiliary_rank\":1,\"closed_rank\":1,\"numerical_mutant_closed_rank\":2},\"positive_gain_established\":false,"
             <<"\"standing\":\"producer-unverified;necessary image candidates require actual RR/KM and separate replay\"}\n";
    return 0;
}catch(const std::exception& e){std::cerr<<"REJECT "<<e.what()<<'\n';return 1;}
