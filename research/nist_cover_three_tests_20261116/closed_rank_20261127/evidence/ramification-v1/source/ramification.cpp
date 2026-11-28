#include "ramification-arithmetic.hpp"
#include "ramification-support.hpp"

int main(int argc,char** argv)try {
    require(argc==8,"QUARTIC BACKMAP BASELINE TANGENTS OLD_SCAN EXPANDED FULL_BOUND");
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

    const auto prior=read_json(argv[7]);
    require(prior.at("controls_pass").scalar=="true"&&prior.at("full_rank").integer()==236&&
            prior.at("auxiliary_rank").integer()==230&&prior.at("closed_rank").integer()==6,
            "successful frozen full-image bound dependency");
    std::cerr<<"REBOUND 1434 points,685 columns,236/230/6\n";
    const auto line=read_bp(back.at("L"));
    const auto ell=ramification::linear_coefficients(line);
    require(line.degree()==1&&ell[1]!=F(0),"retained ramification line with nonzero y coefficient");
    const auto fx=curve.partial(0),fy=curve.partial(1);
    const auto numerator=(fy*map[0].partial(0)-fx*map[0].partial(1))*map[1]-
                          map[0]*(fy*map[1].partial(0)-fx*map[1].partial(1));
    auto differential_ok=[&](const BP& candidate) {
        return (map[3]*numerator-(candidate*map[2]*map[1]*map[1]).scale(F(2))).reduced(curve).t.empty();
    };
    require(differential_ok(line),"exact retained pullback differential identity");
    norm_test::rejects_norm([&]{require(differential_ok(line+constant(F(1))),"mutated differential L");});
    const F intercept=-ell[2]/ell[1],slope=-ell[0]/ell[1];
    const auto restriction=ss_restrict(curve,F(0),intercept,F(1),slope);
    require(restriction.degree()==4,"complete affine ramification degree four");
    const auto q=restriction.monic();
    const auto infinity_gcd=gcd(infinity,Poly{ell[0],ell[1]});
    require(infinity_gcd.degree()==0,"ramification line has no omitted infinity support");
    const auto factors=checked_factors(q);
    require(!factors.empty(),"nonempty ramification factorization");
    auto missing=factors;missing.pop_back();
    norm_test::rejects_norm([&]{verify_factor_product(missing,q);});
    std::vector<std::pair<Poly,int>> distinct;
    for(const auto& f:factors) {
        auto at=std::find_if(distinct.begin(),distinct.end(),[&](const auto& item){return item.first==f;});
        if(at==distinct.end())distinct.emplace_back(f,1);else ++at->second;
    }
    std::vector<ramification::Branch> branches;
    audit::Point trace_sum;
    int total_ramification=0,finite_image_controls=0;
    for(const auto& [factor,multiplicity]:distinct) {
        residue_modulus=factor;
        const K root(Poly{F(0),F(1)}),y=K(intercept)+K(slope)*root;
        const auto local=ramification::local_map(curve,map,line,root,y);
        require(local.line_order==multiplicity,"local L order equals factor multiplicity");
        norm_test::rejects_norm([&]{require(local.index+1==multiplicity+1,"mutated local ramification index");});
        ramification::Branch branch;
        branch.factor=factor;branch.x=root.p;branch.y=y.p;branch.multiplicity=multiplicity;
        branch.degree=factor.degree();branch.local=local;
        if(factor.degree()>1) {
            const auto conjugate=ramification::local_map(curve,map,line,root.frobenius(),y.frobenius());
            require(conjugate.image==ramification::frobenius(local.image)&&
                    conjugate.line_order==local.line_order&&conjugate.index==local.index&&
                    conjugate.solved==local.solved&&conjugate.free==local.free&&
                    conjugate.x.valuation==local.x.valuation&&conjugate.y.valuation==local.y.valuation&&
                    conjugate.x.numerator_order==local.x.numerator_order&&conjugate.x.denominator_order==local.x.denominator_order&&
                    conjugate.y.numerator_order==local.y.numerator_order&&conjugate.y.denominator_order==local.y.denominator_order&&
                    conjugate.x.leading==local.x.leading.frobenius()&&conjugate.y.leading==local.y.leading.frobenius(),
                    "fresh conjugate local map covariance");
        } else require(root.frobenius()==root&&y.frobenius()==y&&
                       ramification::frobenius(local.image)==local.image,"rational branch Frobenius check");
        branch.conjugate_checked=true;
        ::Point trace,conjugate=local.image;
        for(int j=1;j<=factor.degree();++j) {
            trace=add(trace,conjugate);conjugate=ramification::frobenius(conjugate);
            if(branch.image_degree==0&&conjugate==local.image)branch.image_degree=j;
        }
        require(conjugate==local.image&&branch.image_degree>0&&factor.degree()%branch.image_degree==0,
                "source-degree conjugate trace includes repeated image orbit");
        require(ramification::frobenius(trace)==trace&&
                (trace.zero||(trace.x.p.degree()<=0&&trace.y.p.degree()<=0)),"branch trace descends to Fp");
        branch.trace=ec_gmp(trace);require(audit::on_curve(branch.trace),"rational GMP branch trace incidence");
        (void)ssl.make(branch.trace);
        trace_sum=audit::plus(trace_sum,norm_test::multiply(branch.trace,multiplicity));
        if(!local.image.zero) {
            auto bad=local.image;bad.y=bad.y==K(0)?K(1):K(0);
            norm_test::rejects_norm([&]{valid(bad);});++finite_image_controls;
        }
        total_ramification+=multiplicity*factor.degree();branches.push_back(std::move(branch));
        std::cerr<<"BRANCH degree="<<factor.degree()<<" multiplicity="<<multiplicity<<" index="<<local.index<<'\n';
    }
    residue_modulus=Poly{F(0),F(1)};
    require(total_ramification==4&&finite_image_controls>0,"total tame ramification four and nonempty finite-image control");
    require(trace_sum==ec_gmp(kappa),"weighted rational branch traces equal canonical reference");
    std::vector<audit::Point> trace_base;
    SCRow trace_row(branches.size()+1);
    for(size_t i=0;i<branches.size();++i){trace_base.push_back(branches[i].trace);trace_row[i]=branches[i].multiplicity;}
    trace_base.push_back(ec_gmp(kappa));trace_row.back()=-1;
    require(ec_ssl_zero(ec_sparse(trace_row),trace_base,ssl),"named OpenSSL weighted branch-trace sum");
    std::vector<ramification::Exceptional> exceptional;
    for(size_t i=0;i<scan.at("points").arr.size();++i)if(points[i].value.image.zero) {
        const auto& source=points[i].value;
        const auto local=ramification::local_map(curve,map,line,K(source.x),K(source.y));
        require(local.image.zero,"retained rational exceptional endpoint O");
        exceptional.push_back({int(i),source.x,source.y,local});
    }
    require(exceptional.size()==2,"all two retained rational points over O checked");
    const auto axis_controls=ramification::axis_controls();
    const auto axis=ramification::axis_test(curve,line,&map);
    require(axis.complete,"targeted axis symmetry classification complete");
    std::cerr<<"AXIS polar rank="<<axis.rank<<" invariant="<<axis.quartic_invariant<<'\n';
    std::vector<audit::Point> full_base=std::move(original_base);
    full_base.insert(full_base.end(),gmp_base.begin()+225,gmp_base.end());
    require(full_base.size()==685,"all original429 plus new256 images retained");
    auto principal=ec_embed(reference);for(auto& value:principal)value=-value;
    std::vector<audit::Point> fresh_images;
    for(auto& branch:branches) {
        if(branch.trace.zero)continue;
        auto canonical=branch.trace;int sign=1;
        if(canonical.y>audit::mod(-canonical.y)){canonical=audit::negative(canonical);sign=-1;}
        auto at=std::find(full_base.begin(),full_base.end(),canonical);
        size_t column;
        if(at==full_base.end()) {
            column=full_base.size();full_base.push_back(canonical);fresh_images.push_back(canonical);
            principal.resize(full_base.size());
        } else column=size_t(at-full_base.begin());
        principal[column]+=branch.multiplicity*sign;branch.trace_column=int(column);branch.trace_sign=sign;
    }
    require(fresh_images.size()<=4&&full_base.size()<=1024,"bounded new trace auxiliary columns");
    for(auto& row:matrix)row.resize(full_base.size());
    require(norm_test::row_sum(principal,full_base).zero&&ec_ssl_zero(ec_sparse(principal),full_base,ssl),
            "actual Ram-H principal row in independent GMP/named OpenSSL");
    MIBasis basis(matrix);
    int unit_column=-1;
    for(size_t i=0;i<225;++i){SCRow unit(full_base.size());unit[i]=1;if(!basis.representation(unit).first){unit_column=int(i);break;}}
    require(unit_column>=0,"nonempty numerical rank-gain control");
    SCRow unit(full_base.size());unit[size_t(unit_column)]=1;
    auto fixture=matrix;fixture.push_back(unit);
    require(sc_rank(fixture)==237&&ec_aux_rank(fixture)==230&&
            !norm_test::row_sum(unit,full_base).zero&&!ec_ssl_zero(ec_sparse(unit),full_base,ssl),
            "isolated old-unit fixture reaches closed seven but is not an actual relation");
    const auto representation=basis.representation(principal);
    if(!representation.first)basis.append(principal,"actual:Ram-H");
    auto all=matrix;all.push_back(principal);
    const size_t full=sc_rank(all),auxiliary=ec_aux_rank(all);
    require(full==basis.source.size()&&full>=auxiliary+6,"direct full/auxiliary rank consistency");
    std::cout<<"{\"schema\":\"p192-ramification-support-v1\",\"controls_pass\":true,\"differential_identity\":true,"
             <<"\"pool_size\":"<<points.size()<<",\"retained_columns\":685,\"retained_rows\":236,\"retained_auxiliary_columns\":460,"
             <<"\"new_auxiliary_columns\":"<<fresh_images.size()<<",\"column_count\":"<<full_base.size()<<",\"row_count\":"<<all.size()
             <<",\"full_rank\":"<<full<<",\"auxiliary_rank\":"<<auxiliary<<",\"closed_rank\":"<<full-auxiliary
             <<",\"additional_closed_rank\":"<<full-auxiliary-6<<",\"total_ramification_degree\":"<<total_ramification
             <<",\"separate_positive_replay_required\":"<<(full-auxiliary>6?"true":"false")
             <<",\"precision\":45,\"ramification_line_coefficients_X_Y_Z\":[";
    for(size_t i=0;i<3;++i)std::cout<<(i?",":"")<<'"'<<decimal(ell[i])<<'"';
    std::cout<<"],\"affine_restriction\":";emit(restriction);std::cout<<",\"monic_restriction\":";emit(q);
    std::cout<<",\"infinity_gcd\":";emit(infinity_gcd);std::cout<<",\"reference_kappa\":";ramification::emit_ap(ec_gmp(kappa));
    std::cout<<",\"branches\":[";
    for(size_t i=0;i<branches.size();++i) {
        const auto& branch=branches[i];std::cout<<(i?",":"")<<"{\"factor\":";emit(branch.factor);
        std::cout<<",\"multiplicity\":"<<branch.multiplicity<<",\"source_degree\":"<<branch.degree<<",\"image_degree\":"<<branch.image_degree
                 <<",\"conjugate_checked\":true,\"source_x_coefficients\":";emit(branch.x);
        std::cout<<",\"source_y_coefficients\":";emit(branch.y);std::cout<<",\"local\":";ramification::emit_local(branch.local);
        std::cout<<",\"rational_trace\":";ramification::emit_ap(branch.trace);
        std::cout<<",\"trace_column\":"<<branch.trace_column<<",\"trace_sign\":"<<branch.trace_sign<<'}';
    }
    std::cout<<"],\"rational_points_over_O\":[";
    for(size_t i=0;i<exceptional.size();++i) {
        const auto& item=exceptional[i];std::cout<<(i?",":"")<<"{\"pool_id\":"<<item.pool_id<<",\"x\":\""<<decimal(item.x)
                  <<"\",\"y\":\""<<decimal(item.y)<<"\",\"local\":";ramification::emit_local(item.local);std::cout<<'}';
    }
    std::cout<<"],\"axis_test\":{\"polar_matrix\":";ramification::emit_matrix(axis.polar);
    std::cout<<",\"polar_rank\":"<<axis.rank<<",\"complete\":true,\"center_off_axis\":"<<(axis.center_off_axis?"true":"false")
             <<",\"quartic_invariant\":"<<(axis.quartic_invariant?"true":"false")<<",\"map_compatible\":"<<(axis.map_compatible?"true":"false")
             <<",\"center\":[";
    for(size_t i=0;i<axis.center.size();++i)std::cout<<(i?",":"")<<'"'<<decimal(axis.center[i])<<'"';
    std::cout<<"],\"reflection\":";ramification::emit_matrix(axis.reflection);
    std::cout<<"},\"fixture_polar_ranks\":["<<axis_controls.positive_rank<<','<<axis_controls.sheared_rank<<','<<axis_controls.negative_rank
             <<"],\"finite_image_mutations_rejected\":"<<finite_image_controls<<",\"principal_row\":";ec_emit_sparse(principal);
    std::cout<<",\"principal_dependent\":"<<(representation.first?"true":"false")<<",\"principal_basis_coefficients\":[";
    bool comma=false;
    if(representation.first)for(size_t i=0;i<representation.second.size();++i)if(representation.second[i]!=0) {
        if(comma)std::cout<<',';
        comma=true;std::cout<<'['<<i<<",\""<<representation.second[i]<<"\"]";
    }
    std::cout<<"],\"fresh_auxiliary_images\":[";
    for(size_t i=0;i<fresh_images.size();++i){if(i)std::cout<<',';ramification::emit_ap(fresh_images[i]);}
    std::cout<<"],\"numerical_fixture\":{\"old_unit_column\":"<<unit_column
             <<",\"full_rank\":237,\"auxiliary_rank\":230,\"closed_rank\":7,\"actual_relation\":false},"
             <<"\"cover_validity_basis\":\"retained degree-six map plus exact differential and local checks\","
             <<"\"all_old_auxiliaries_retained\":true,\"old_pair_or_six_searches_rerun\":0,\"PCC_claims\":0}\n";
    return 0;
}catch(const std::exception& error){std::cerr<<"REJECT "<<error.what()<<'\n';return 1;}
