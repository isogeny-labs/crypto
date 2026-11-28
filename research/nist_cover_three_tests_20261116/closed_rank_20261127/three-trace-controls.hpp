#pragma once

namespace shifted {
struct TraceWitness {std::array<int,4> support{};std::array<AP,2> anchors{};AP trace;};
void trace_closure_fixture() {
    audit::SSL ssl;auto generator=EC_GROUP_get0_generator(ssl.group.get());audit::BN x(BN_new()),y(BN_new());
    require(EC_POINT_get_affine_coordinates(ssl.group.get(),generator,x.get(),y.get(),ssl.context.get())==1,"trace fixture generator");
    auto integer=[](const audit::BN& n){char* text=BN_bn2dec(n.get());require(text,"trace fixture integer");SCBig out(text);OPENSSL_free(text);return out;};
    AP g{false,integer(x),integer(y)};const AP twice=audit::plus(g,g),six=norm_test::multiply(g,6);
    std::vector<AP> traces{g,g,audit::negative(twice)},base{g,twice};
    for(const auto& t:traces)require(!t.zero,"three nonzero public fixture traces");
    require(audit::plus(audit::plus(traces[0],traces[1]),traces[2]).zero&&named_sum(SCRow{1,1,1},traces,{},ssl),"nonempty three-trace fixture closure both engines");
    const Word word{0,0,1,1,3,3};const Sparse reference{{0,6}};const auto row=coefficients(word,reference);
    require(row==Sparse{{0,-4},{1,2}}&&norm_test::row_sum(norm_test::dense(row,2),base).zero&&named_sum(norm_test::dense(row,2),base,{},ssl),"nonzero trace fixture coefficient row");
    require(named_sum(SCRow{2,2},base,six,ssl)&&sc_rank(SCMat{SCRow{-4,2}})==1,"trace fixture six-anchor sum and row rank");
    auto changed=traces;changed[0]=audit::plus(changed[0],g);
    require(!audit::plus(audit::plus(changed[0],changed[1]),changed[2]).zero&&!named_sum(SCRow{1,1,1},changed,{},ssl),"changed fixture anchor breaks trace closure");
}
std::vector<TraceWitness> actual_trace_controls(const BP& curve,const std::array<BP,4>& map,
                                               const Json& baseline,const AP& kappa,const AP& nonzero_mutation) {
    audit::SSL ssl;std::vector<TraceWitness> witnesses;
    for(size_t section=0;section<3;++section) {
        const auto& ids=baseline.at("pool").at("split_fibers").arr.at(section).at("point_indices").arr;
        require(ids.size()==4,"actual trace control complete section");TraceWitness w;
        std::array<std::array<F,2>,4> xy{};std::array<AP,4> images{};
        for(size_t i=0;i<4;++i) {
            const int id=ids[i].integer();w.support[i]=id;const auto& p=baseline.at("pool").at("points").arr.at(size_t(id));
            const F x=coefficient(p.at("x")),y=coefficient(p.at("y"));xy[i]={x,y};
            require(fs_eval(curve,x,y)==F(0),"actual trace control curve incidence");
            const int old_precision=precision;const auto mapped=fs_image(curve,map,x,y).value;precision=old_precision;
            require(mapped==point(baseline.at("images").arr.at(size_t(id)).at("point")),"fresh actual trace control map");images[i]=gmp_point(mapped);
        }
        const F dx=xy[1][0]-xy[0][0],dy=xy[1][1]-xy[0][1];require(dx!=F(0)||dy!=F(0),"distinct trace control anchors");
        const auto restriction=ss_restrict(curve,xy[0][0],xy[0][1],dx,dy);require(restriction.degree()==4,"trace control section degree four");
        Poly product{F(1)};std::set<std::string> seen;
        for(const auto& p:xy) {
            require(seen.insert(sp_key(p[0],p[1])).second&&(p[0]-xy[0][0])*dy==(p[1]-xy[0][1])*dx,"actual trace control distinct collinear support");
            const F t=dx!=F(0)?(p[0]-xy[0][0])/dx:(p[1]-xy[0][1])/dy;product=product*Poly{-t,F(1)};
        }
        require(product.scale(restriction.at(4))==restriction,"actual trace control exact factor product");
        w.anchors={images[0],images[1]};w.trace=audit::plus(images[2],images[3]);
        const AP expected=audit::plus(kappa,audit::negative(audit::plus(images[0],images[1])));
        std::vector<AP> relation{images[0],images[1],w.trace,kappa};const SCRow coefficients{1,1,1,-1};
        require(w.trace==expected&&norm_test::row_sum(coefficients,relation).zero&&named_sum(coefficients,relation,{},ssl),"actual residual trace identity both engines");
        relation[2]=audit::plus(relation[2],nonzero_mutation);
        require(!norm_test::row_sum(coefficients,relation).zero&&!named_sum(coefficients,relation,{},ssl),"changed actual residual trace rejects");
        witnesses.push_back(w);
    }
    return witnesses;
}
void emit_trace_witnesses(const std::vector<TraceWitness>& witnesses) {
    std::cout<<'[';
    for(size_t i=0;i<witnesses.size();++i) {
        if(i)std::cout<<',';
        const auto& w=witnesses[i];std::cout<<"{\"section\":"<<i<<",\"curve_support\":[";
        for(size_t j=0;j<4;++j)std::cout<<(j?",":"")<<w.support[j];
        std::cout<<"],\"anchor_images\":[";emit_point(w.anchors[0]);std::cout<<',';emit_point(w.anchors[1]);
        std::cout<<"],\"residual_trace\":";emit_point(w.trace);std::cout<<",\"identity_verified_both_engines\":true}";
    }
    std::cout<<']';
}
}
