// Specialize the actual graph at H infinity and bind the retained E_ref.
#include "projective_base.hpp"
Poly read_long(const Json& j) {
    require(j.kind==JsonKind::array && j.arr.size()<=129,"reference coefficient cap");
    std::vector<F> r;for(const auto& x:j.arr)r.push_back(coefficient(x));
    require(r.empty() || r.back()!=F(0),"canonical reference coefficient tail");return Poly(std::move(r));
}
F limit(const Poly& a,const Poly& b,const Poly& c) {
    require(!c.c.empty() && a.degree()<=c.degree(),"regular rational part at H infinity");
    require(b.c.empty() || 2*b.degree()+7<2*c.degree(),"vanishing Y part at H infinity");
    return a.degree()==c.degree()?a.c.back()/c.c.back():F(0);
}
F limit(const Json& row) { return limit(read_long(row.at("a")),read_long(row.at("b")),read_long(row.at("c"))); }
F value(const Poly& p,const F& x) { F r;for(int i=p.degree();i>=0;--i)r=r*x+p.at(i);return r; }
F value(const BP& p,const F& x,const F& y) {
    F r;for(const auto& [e,c]:p.t) { F t=c;for(int i=0;i<e[0];++i)t=t*x;for(int i=0;i<e[1];++i)t=t*y;r=r+t; }return r;
}
int main(int argc,char** argv) { try {
    auto begin=std::chrono::steady_clock::now();require(argc==5,"usage: reference-check quartic global pool formal");
    auto model=read_json(argv[1]),global=read_json(argv[2]),pool=read_json(argv[3]),formal=read_json(argv[4]);
    BP f=read_bp(model.at("terms"),true);std::vector<F> us,vs;
    require(global.at("U").arr.size()==4 && global.at("V").arr.size()==3,"reference cubic graph shape");
    for(const auto& row:global.at("U").arr)us.push_back(limit(row));
    for(const auto& row:global.at("V").arr)vs.push_back(limit(row));
    Poly U(us),V(vs),expected{F(1)};require(U.degree()==3 && U.c.back()==F(1),"complete monic infinity special fiber");
    const auto& ids=formal.at("seed_indices").arr;
    require(ids.size()==3 && ids[0].integer()==225 && ids[1].integer()==224 && ids[2].integer()==223,"fixed software reference indices");
    std::array<F,3> xs,ys;
    for(size_t i=0;i<3;++i) {
        const auto& row=pool.at("points").arr.at(size_t(ids[i].integer()));xs[i]=coefficient(row.at("x"));ys[i]=coefficient(row.at("y"));
        require(value(f,xs[i],ys[i])==F(0),"reference quartic incidence");
        expected=expected*Poly{-xs[i],F(1)};
        require(value(V,xs[i])==ys[i],"specialized graph reference ordinate");
    }
    require(U==expected && std::get<0>(bezout(U,derivative(U))).degree()==0,"entire distinct reference special fiber");
    require(value(V+Poly{F(1)},xs[0])!=ys[0],"changed reference ordinate rejects");
    require(U!=Poly{-xs[0],F(1)}*Poly{-xs[1],F(1)},"dropped reference factor rejects");
    const auto& row=global.at("V").arr[0];Poly a=read_long(row.at("a")),b=read_long(row.at("b")),c=read_long(row.at("c"));
    bool rejected=false;try { static_cast<void>(limit(a,b+c,c)); } catch(const std::exception&) { rejected=true; }
    require(rejected,"unramified source infinity Y-pole rejects");
    auto us_elapsed=std::chrono::duration_cast<std::chrono::microseconds>(std::chrono::steady_clock::now()-begin).count();
    std::cout<<"{\"scope\":\"actual-generic-correspondence-reference-specialization\",\"reference_indices\":[225,224,223],\"coefficient_limits_checked\":7,\"distinct_reference_points_checked\":3,\"complete_special_fiber_matches\":true,\"actual_subtraction_convention_bound\":true,\"mutation_controls_rejected\":3,\"signed_independent_verdict\":false,\"U_at_source_infinity\":";emit(U);
    std::cout<<",\"V_at_source_infinity\":";emit(V);std::cout<<",\"references\":[";
    for(size_t i=0;i<3;++i) { if(i)std::cout<<',';std::cout<<"{\"x\":\""<<decimal(xs[i])<<"\",\"y\":\""<<decimal(ys[i])<<"\"}"; }
    std::cout<<"],\"wall_us\":"<<us_elapsed<<"}\n";
} catch(const std::exception& e) { std::cerr<<"REJECT "<<e.what()<<'\n';return 1; } }
