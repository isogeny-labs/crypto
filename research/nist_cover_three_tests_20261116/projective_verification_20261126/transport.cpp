#include "projective_base.hpp"
F fp_power(F a,int n) { F r(1);for (int i=0;i<n;++i) r=r*a;return r; }
void emit_closed(const Point& p) {
    std::cout<<"{\"infinity\":"<<(p.zero?"true":"false");
    if (!p.zero) { std::cout<<",\"x\":";emit(p.x.p);std::cout<<",\"y\":";emit(p.y.p); }
    std::cout<<'}';
}
int main(int argc,char** argv) { try {
    auto start=std::chrono::steady_clock::now();require(argc==4,"usage: projective-transport quartic backmap relations");
    Json model=read_json(argv[1]),data=read_json(argv[2]),relations=read_json(argv[3]);
    require(model.at("p").scalar==prime_text,"projective transport field");BP f=read_bp(model.at("terms"),true);
    std::array<BP,4> maps;const std::array<std::string,4> names{"X_numerator","X_denominator","Y_numerator","Y_denominator"};int max_degree=0;
    for (size_t i=0;i<4;++i) { maps[i]=read_bp(data.at(names[i]));max_degree=std::max(max_degree,maps[i].degree()); }
    require(max_degree>0 && max_degree<=32 && global_identity(f,maps),"transport global map identity");precision=4*max_degree+1;
    HP curve=homogenize(f,4);std::array<HP,4> hm;for (size_t i=0;i<4;++i) hm[i]=homogenize(maps[i]);
    std::vector<F> qc(5,F(0));for (const auto& [e,c]:f.t) if (e[0]+e[1]==4) qc[size_t(e[1])]=qc[size_t(e[1])]+c;
    Poly q(qc);require(q.degree()==4,"complete infinity hyperplane");q=q.monic();auto factors=checked_factors(q);
    Point infinity_sum,last;int conjugates=0;
    for (const auto& factor:factors) {
        residue_modulus=factor;K root(Poly{F(0),F(1)});Boundary r=boundary(curve,hm,root,factor,1);
        require(!r.infinity,"recorded infinity image is a nonidentity elliptic point");Point image(K(r.x),K(r.y)),trace,conjugate=image;
        for (int i=0;i<factor.degree();++i) { trace=add(trace,conjugate);last=conjugate;conjugate=Point(conjugate.x.frobenius(),conjugate.y.frobenius());++conjugates; }
        require(conjugate==image,"complete projective image orbit");
        Point missing=trace;missing=add(missing,Point(last.x,-last.y));require(!(missing==trace),"dropped nonidentity infinity conjugate rejected");
        if (!trace.zero) {
            require(trace.x.p.degree()<=0 && trace.y.p.degree()<=0,"projective norm prime-field descent");
            F x=trace.x.p.at(0),y=trace.y.p.at(0);residue_modulus=Poly{F(0),F(1)};infinity_sum=add(infinity_sum,Point(K(x),K(y)));
        }
        residue_modulus=Poly{F(0),F(1)};
    }
    require(conjugates==4,"all four geometric infinity points retained");
    const auto& reference=relations.at("pool").at("split_fibers").arr.at(0);
    require(reference.at("x").integer()==56 && reference.at("point_indices").arr.size()==4,"fixed complete split reference");
    const std::array<int,4> ids{42,43,44,45};Poly finite_product{F(1)};Point finite_sum;
    for (size_t i=0;i<4;++i) {
        require(reference.at("point_indices").arr[i].integer()==ids[i],"fixed reference index");
        const auto& p=relations.at("pool").at("points").arr.at(size_t(ids[i]));F x=coefficient(p.at("x")),y=coefficient(p.at("y"));require(x==F(56),"reference x binding");
        finite_product=finite_product*Poly{-y,F(1)};
        Jet local=lift(curve,{R{K(x)},R{K(y)},R{K(1)}},2);
        Point image=endpoint(ratio(hm[0],hm[1],local),ratio(hm[2],hm[3],local));
        require(image==point(relations.at("images").arr.at(size_t(ids[i])).at("point")),"fresh reference image agrees with original map");
        finite_sum=add(finite_sum,image);
    }
    std::vector<F> fc(5,F(0));for (const auto& [e,c]:f.t) fc[size_t(e[1])]=fc[size_t(e[1])]+c*fp_power(F(56),e[0]);
    Poly finite(fc);require(finite_product==finite.monic(),"complete reference hyperplane factor product");
    rejects([&]{auto& p=relations.at("pool").at("points").arr.at(45);Poly omitted=finite_product.exact(Poly{-coefficient(p.at("y")),F(1)});require(omitted==finite.monic(),"dropped reference point");});
    require(infinity_sum==finite_sum,"projective principal relation elliptic sums");
    Point difference=finite_sum.zero?infinity_sum:add(infinity_sum,Point(finite_sum.x,-finite_sum.y));require(difference.zero,"transported projective relation is elliptic identity");
    auto us=std::chrono::duration_cast<std::chrono::microseconds>(std::chrono::steady_clock::now()-start).count();
    std::cout<<"{\"standing\":\"unregistered separate OpenSSL projective transport\",\"infinity_geometric_points\":4,\"reference_indices\":[42,43,44,45],\"fresh_reference_image_checks\":4,\"complete_hyperplane_products_checked\":true,\"projective_relation_elliptic_zero\":true,\"mutation_controls_rejected\":2,\"infinity_sum\":";
    emit_closed(infinity_sum);std::cout<<",\"reference_sum\":";emit_closed(finite_sum);std::cout<<",\"wall_us\":"<<us<<",\"signed_independent_verdict\":false}\n";
    return 0;
} catch (const std::exception& e) { std::cerr<<"REJECT "<<e.what()<<'\n';return 1; } }
