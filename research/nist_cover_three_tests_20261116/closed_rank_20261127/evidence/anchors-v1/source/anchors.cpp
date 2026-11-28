// Parse the frozen GMP helper before the BN geometry backend declares Point.
#define main frozen_anchor_norm_entrypoint
#include "norms.cpp"
#undef main
#include "partial_arithmetic.hpp"

using ASparse=norm_test::Sparse;
struct ACurvePoint {F x,y;::Point image;};
struct ASeed {
    int i=0,j=0;
    bool tangent=false;
    F dx,dy;
    Poly restriction;
    std::array<ACurvePoint,2> residual;
    ASparse row;
};
struct AFibre {
    size_t projective=0,affine=0,candidates=0,filtered=0;
    int chart=-1;
    Poly characteristic;
    std::vector<ACurvePoint> points;
};

audit::Point a_gmp(const ::Point& p) {
    if(p.zero)return {};
    return {false,audit::Big(decimal(p.x.p.at(0))),audit::Big(decimal(p.y.p.at(0)))};
}
bool a_ssl_zero(const ASparse& row,const std::vector<audit::Point>& base,const audit::SSL& ssl) {
    auto sum=ssl.make({}),term=ssl.make({});
    for(auto [column,coefficient]:row) {
        auto p=ssl.make(base.at(size_t(column)));
        auto scalar=audit::bn(audit::Big(std::abs(coefficient)));
        require(EC_POINT_mul(ssl.group.get(),term.get(),nullptr,p.get(),scalar.get(),ssl.context.get())==1,
                "anchor named-curve coefficient");
        if(coefficient<0)require(EC_POINT_invert(ssl.group.get(),term.get(),ssl.context.get())==1,
                                 "anchor named-curve negation");
        require(EC_POINT_add(ssl.group.get(),sum.get(),sum.get(),term.get(),ssl.context.get())==1,
                "anchor named-curve row addition");
    }
    return EC_POINT_is_at_infinity(ssl.group.get(),sum.get())==1;
}
bool a_matches(const ::Point& image,const F& target) {
    return !image.zero&&image.x.p.at(0)==target;
}
AFibre a_fibre(const BP& curve,const std::array<BP,4>& map,const ::Point& target,
               const ACurvePoint& witness,bool controls) {
    require(a_matches(witness.image,target.x.p.at(0)),"known residual target binding");
    const BP g=map[0]-map[1].scale(target.x.p.at(0));
    const auto algebra=fs_algebra(curve,g);
    AFibre out;
    out.projective=algebra.projective;
    out.affine=algebra.affine;
    out.chart=algebra.chart;
    out.characteristic=fs_charpoly(algebra.x);
    if(controls) {
        require(!algebra.x.empty()&&fs_annihilates(algebra.x,out.characteristic),
                "anchor characteristic annihilation");
        require(!fs_annihilates(algebra.x,out.characteristic+Poly{F(1)}),
                "changed anchor characteristic constant rejected");
    }
    std::set<std::string> keys;
    for(const auto& x:fs_roots(out.characteristic,controls))for(const auto& y:fs_roots(fs_y(curve,x))) {
        if(fs_eval(g,x,y)!=F(0))continue;
        ++out.candidates;
        auto image=fs_image(curve,map,x,y).value;
        if(!a_matches(image,target.x.p.at(0))){++out.filtered;continue;}
        require(image.y==target.y||image.y==-target.y,"complete signed target y binding");
        require(keys.insert(sp_key(x,y)).second,"distinct anchor fibre point");
        out.points.push_back({x,y,image});
    }
    const auto known=sp_key(witness.x,witness.y);
    require(keys.count(known)==1,"actual residual fibre witness recovered");
    require(out.points.size()<=12,"degree-six signed fibre point bound");
    if(controls) {
        auto omitted=keys;
        omitted.erase(known);
        fs_reject([&]{require(omitted.count(known)==1,"omitted actual residual witness");});
        require(!a_matches(witness.image,target.x.p.at(0)+F(1)),"changed actual target x rejected");
    }
    return out;
}
void a_emit_sparse(const ASparse& row) {
    std::cout<<'[';
    bool comma=false;
    for(auto [column,coefficient]:row) {
        if(comma)std::cout<<',';
        comma=true;
        std::cout<<'['<<column<<','<<coefficient<<']';
    }
    std::cout<<']';
}
void a_emit_curve(const ACurvePoint& point) {
    std::cout<<"{\"x\":\""<<decimal(point.x)<<"\",\"y\":\""<<decimal(point.y)<<"\",\"image\":";
    emit(point.image);
    std::cout<<'}';
}

int main(int argc,char** argv) {
try {
    require(argc==6,"usage: anchors QUARTIC BACKMAP BASELINE TANGENTS LIFTED_SCAN");
    const auto model=read_json(argv[1]),back=read_json(argv[2]),baseline=read_json(argv[3]),
               tangents=read_json(argv[4]),scan=read_json(argv[5]);
    ss_private(baseline,tangents);
    const auto curve=read_bp(model.at("terms"),true);
    std::array<BP,4> map;
    size_t position=0;
    for(const auto* name:{"X_numerator","X_denominator","Y_numerator","Y_denominator"})
        map[position++]=read_bp(back.at(name));
    require(global_identity(curve,map),"actual anchor map identity");
    std::vector<F> leading(5);
    for(const auto& [e,coefficient]:curve.t)if(e[0]+e[1]==4)leading[size_t(e[1])]=coefficient;
    const Poly infinity(std::move(leading)),z{F(0),F(1)};
    require(infinity.degree()==4&&gcd(infinity,wide_power(z,prime(),infinity)-z).degree()==0,
            "no rational source infinity omitted");

    std::vector<audit::Point> domain,original_domain;
    std::set<std::string> original_x;
    for(const auto& q:tangents.at("projected_base").arr) {
        const auto p=point(q);
        valid(p);
        require(!p.zero&&BN_cmp(p.y.p.at(0).n.get(),(-p.y.p.at(0)).n.get())<=0,
                "original canonical image");
        require(original_x.insert(decimal(p.x.p.at(0))).second,"distinct original429image labels");
        original_domain.push_back(a_gmp(p));
        if(domain.size()<225)domain.push_back(a_gmp(p));
    }
    require(original_domain.size()==429&&domain.size()==225,"frozen original widths");
    for(size_t i=0;i<225;++i)require(domain[i]==audit::read_point(baseline.at("projected_base").arr.at(i)),
                                  "original225prefix retained");
    std::vector<SPoint> old;
    std::set<std::string> old_keys;
    for(const auto& q:scan.at("points").arr) {
        const auto x=coefficient(q.at("x")),y=coefficient(q.at("y"));
        const auto actual=fs_image(curve,map,x,y).value;
        require(actual==point(q.at("image"))&&old_keys.insert(sp_key(x,y)).second,
                "actual distinct old anchor-pool image");
        const int index=q.at("index").integer(),sign=q.at("sign").integer();
        if(index<0)require(index==-1&&sign==0&&actual.zero,"old identity image label");
        else {
            require(index<225&&(sign==1||sign==-1),"old signed image label");
            auto expected=domain.at(size_t(index));
            if(sign<0)expected=audit::negative(expected);
            require(a_gmp(actual)==expected,"old image coordinate/sign binding");
        }
        old.push_back({x,y,actual,index,sign});
    }
    require(old.size()==652,"frozen652-point old pool");
    for(size_t i=0;i<226;++i) {
        const auto& q=baseline.at("pool").at("points").arr.at(i);
        require(decimal(old[i].x)==q.at("x").scalar&&decimal(old[i].y)==q.at("y").scalar,
                "original226curve points retain ordering");
    }
    ASparse reference;
    ::Point kappa;
    for(const auto& id:baseline.at("pool").at("split_fibers").arr[0].at("point_indices").arr) {
        const auto& p=old.at(size_t(id.integer()));
        kappa=add(kappa,p.image);
        if(p.index>=0)reference[p.index]+=p.sign;
    }
    require(!kappa.zero&&norm_test::row_sum(norm_test::dense(reference,225),domain)==a_gmp(kappa),
            "unchanged actual reference trace expression");
    std::vector<ASeed> seeds;
    std::vector<::Point> selected;
    std::set<std::string> selected_x;
    size_t attempts=0,split=0;
    auto select=[&](int i,int j,bool tangent) {
        require(++attempts<=4096,"anchor selection prefix cap; incomplete if exceeded");
        const auto& a=old.at(size_t(i));
        const auto& b=old.at(size_t(j));
        F dx,dy;
        if(tangent){dx=fs_eval(curve.partial(1),a.x,a.y);dy=-fs_eval(curve.partial(0),a.x,a.y);}
        else{dx=b.x-a.x;dy=b.y-a.y;}
        require(dx!=F(0)||dy!=F(0),"proper anchor selection line");
        const auto restriction=ss_restrict(curve,a.x,a.y,dx,dy);
        require(restriction.degree()==4&&restriction.at(0)==F(0)&&
                (tangent?restriction.at(1)==F(0):ss_eval(restriction,F(1))==F(0)),
                "exact old anchor line restriction");
        const auto residual=restriction.exact(tangent?Poly{F(0),F(0),F(1)}:Poly{F(0),F(-1),F(1)});
        const auto roots=ss_quadratic(residual);
        if(roots.empty())return;
        ++split;
        std::array<ACurvePoint,2> points;
        std::array<std::string,2> xkeys;
        for(size_t k=0;k<2;++k) {
            const auto x=a.x+dx*roots[k],y=a.y+dy*roots[k];
            const auto image=fs_image(curve,map,x,y).value;
            if(image.zero)return;
            points[k]={x,y,image};
            xkeys[k]=decimal(image.x.p.at(0));
            if(original_x.count(xkeys[k])||selected_x.count(xkeys[k]))return;
        }
        if(xkeys[0]==xkeys[1])return;
        auto negative=kappa;
        negative.y=-negative.y;
        require(add(add(add(add(a.image,b.image),points[0].image),points[1].image),negative).zero,
                "actual selected canonical section image sum");
        ASeed seed{i,j,tangent,dx,dy,restriction,points,{}};
        for(const auto* p:{&a,&b})if(p->index>=0)seed.row[p->index]+=p->sign;
        for(size_t k=0;k<2;++k) {
            auto canonical=points[k].image;
            if(BN_cmp(canonical.y.p.at(0).n.get(),(-canonical.y.p.at(0)).n.get())>0)canonical.y=-canonical.y;
            seed.row[int(domain.size())]+=points[k].image.y==canonical.y?1:-1;
            domain.push_back(a_gmp(canonical));
            selected.push_back(canonical);
            selected_x.insert(xkeys[k]);
        }
        for(auto [column,coefficient]:reference)seed.row[column]-=coefficient;
        seed.row=norm_test::sparse(norm_test::dense(seed.row,domain.size()));
        seeds.push_back(std::move(seed));
    };
    for(size_t i=0;i<old.size()&&seeds.size()<8;++i) {
        for(size_t j=i+1;j<old.size()&&seeds.size()<8;++j)select(int(i),int(j),false);
        if(seeds.size()<8)select(int(i),int(i),true);
    }
    require(seeds.size()==8&&selected.size()==16&&domain.size()==241,"complete deterministic anchor selection");
    auto incomplete=selected;
    incomplete.pop_back();
    fs_reject([&]{require(incomplete.size()==16,"omitted target rejected");});

    std::vector<AFibre> fibres;
    std::set<std::string> new_keys;
    size_t fibre_points=0;
    for(size_t i=0;i<selected.size();++i) {
        auto fibre=a_fibre(curve,map,selected[i],seeds[i/2].residual[i%2],i==0);
        for(const auto& p:fibre.points) {
            const auto key=sp_key(p.x,p.y);
            require(!old_keys.count(key)&&new_keys.insert(key).second,"new cover anchors disjoint from old pool");
        }
        fibre_points+=fibre.points.size();
        std::cerr<<"anchor fibre "<<i<<" rational points "<<fibre.points.size()<<'\n';
        fibres.push_back(std::move(fibre));
    }
    require(fibres.size()==16&&fibre_points==new_keys.size(),"complete selected fibre inventory");
    const auto stream=norm_test::enumerate(domain,a_gmp(kappa));
    const auto found=norm_test::grouped(stream.pairs,reference),checked=norm_test::sorted(stream.pairs,reference);
    require(found==checked,"complete anchor grouped/sorted joins agree");
    audit::SSL ssl;
    for(const auto& seed:seeds)require(found.count(seed.row)==1,"all actual seed section rows recovered");
    auto bad=seeds[0].row;
    ++bad[0];
    require(!norm_test::row_sum(norm_test::dense(bad,domain.size()),domain).zero&&!a_ssl_zero(bad,domain,ssl),
            "changed seed coefficient rejected by both implementations");
    audit::Point g{false,audit::Big("188DA80EB03090F67CBF20EB43A18800F4FF0AFD82FF1012",16),
                        audit::Big("07192B95FFC8DA78631011ED6B24CDD573F977A11E794811",16)};
    auto generator=ssl.make(g);
    require(EC_POINT_cmp(ssl.group.get(),generator.get(),EC_GROUP_get0_generator(ssl.group.get()),ssl.context.get())==0,
            "public generator fixture binding");
    std::vector<audit::Point> fixture{g,audit::plus(g,g)};
    auto control_stream=norm_test::enumerate(fixture,norm_test::multiply(g,4));
    auto control=norm_test::grouped(control_stream.pairs,{{0,4}});
    require(control==norm_test::sorted(control_stream.pairs,{{0,4}})&&control.count(ASparse{{0,-4},{1,2}}),
            "nonempty public fixture detects known relation");

    auto matrix=sc_read_matrix(tangents.at("matrix"));
    require(matrix.size()+found.size()<=256&&429+selected.size()<=1024,"unchanged dense matrix caps");
    for(auto& row:matrix) {
        require(norm_test::row_sum(row,original_domain).zero&&a_ssl_zero(norm_test::sparse(row),original_domain,ssl),
                "original matrix actual sums checked by both implementations");
        row.resize(429+selected.size());
    }
    for(const auto& [sparse,witness]:found) {
        (void)witness;
        require(norm_test::row_sum(norm_test::dense(sparse,domain.size()),domain).zero&&a_ssl_zero(sparse,domain,ssl),
                "all anchor norm rows actual elliptic sums");
        SCRow row(429+selected.size());
        for(auto [column,coefficient]:sparse)row.at(size_t(column<225?column:column+204))=coefficient;
        matrix.push_back(std::move(row));
    }
    const size_t full=sc_rank(matrix);
    SCMat auxiliary;
    for(const auto& row:matrix)auxiliary.emplace_back(row.begin()+225,row.end());
    const size_t arank=sc_rank(auxiliary);
    require(full>=arank+6,"anchor baseline closure retained");
    std::cout<<"{\"scope\":\"actual-residual-fibre-expansion-with-complete-image-norm-bound\","
             <<"\"old_columns\":225,\"original_auxiliary_columns\":204,\"new_auxiliary_columns\":16,"
             <<"\"selection_prefix_attempts\":"<<attempts<<",\"selection_split_sections\":"<<split
             <<",\"selected_sections\":8,\"old_curve_points\":652,\"new_curve_points\":"<<fibre_points
             <<",\"expanded_curve_pool\":"<<652+fibre_points<<",\"known_residual_witnesses_recovered\":16,"
             <<"\"signed_image_alphabet\":"<<2*domain.size()+1<<",\"signed_norm_pairs\":"<<stream.pairs.size()
             <<",\"pair_stream_sha256\":\""<<stream.digest<<"\",\"distinct_norm_rows_including_zero\":"<<found.size()
             <<",\"matrix_rows\":"<<matrix.size()<<",\"matrix_columns\":"<<matrix[0].size()
             <<",\"full_rank\":"<<full<<",\"auxiliary_rank\":"<<arank<<",\"closed_rank\":"<<full-arank
             <<",\"candidate_gain\":"<<full-arank-6<<",\"controls_pass\":true,\"positive_gain_established\":false,"
             <<"\"selected_images\":[";
    for(size_t i=0;i<selected.size();++i){if(i)std::cout<<',';emit(selected[i]);}
    std::cout<<"],\"seed_sections\":[";
    for(size_t i=0;i<seeds.size();++i) {
        if(i)std::cout<<',';
        const auto& seed=seeds[i];
        std::cout<<"{\"i\":"<<seed.i<<",\"j\":"<<seed.j<<",\"tangent\":"<<(seed.tangent?"true":"false")
                 <<",\"direction\":[\""<<decimal(seed.dx)<<"\",\""<<decimal(seed.dy)<<"\"],\"restriction\":";
        emit(seed.restriction);
        std::cout<<",\"residual_points\":[";
        a_emit_curve(seed.residual[0]);std::cout<<',';a_emit_curve(seed.residual[1]);
        std::cout<<"],\"row\":";a_emit_sparse(seed.row);std::cout<<'}';
    }
    std::cout<<"],\"fibres\":[";
    for(size_t i=0;i<fibres.size();++i) {
        if(i)std::cout<<',';
        const auto& fibre=fibres[i];
        std::cout<<"{\"target\":"<<i<<",\"projective_dimension\":"<<fibre.projective
                 <<",\"affine_dimension\":"<<fibre.affine<<",\"unit_chart\":"<<fibre.chart
                 <<",\"intersection_rational_candidates\":"<<fibre.candidates
                 <<",\"base_points_filtered\":"<<fibre.filtered<<",\"characteristic\":";
        emit(fibre.characteristic);
        std::cout<<",\"points\":[";
        for(size_t j=0;j<fibre.points.size();++j){if(j)std::cout<<',';a_emit_curve(fibre.points[j]);}
        std::cout<<"]}";
    }
    std::cout<<"],\"norm_rows\":[";
    bool comma=false;
    for(const auto& [row,word]:found) {
        if(comma)std::cout<<',';
        comma=true;
        std::cout<<"{\"row\":";a_emit_sparse(row);std::cout<<",\"symbols\":[";
        for(size_t j=0;j<word.size();++j)std::cout<<(j?",":"")<<word[j];
        std::cout<<"]}";
    }
    std::cout<<"],\"standing\":\"producer-unverified; complete image bound for this expanded fibre pool; positive candidates require actual cover/KM and separate replay\"}\n";
    return 0;
}catch(const std::exception& e){std::cerr<<"REJECT "<<e.what()<<'\n';return 1;}
}
