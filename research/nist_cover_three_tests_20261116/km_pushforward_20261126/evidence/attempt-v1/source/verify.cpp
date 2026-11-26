// Separate OpenSSL curve-map/Frobenius evaluation; no GMP arithmetic imported.
#include "projective_base.hpp"
struct Cycle { Poly u,x,y;int coordinate=0; };
BP curve;std::array<BP,4> maps;std::array<HP,4> homogeneous_maps;HP homogeneous_curve;
int poles=0,closed_points=0,instances=0;
K eval(const BP& p,const K& x,const K& y) {
    K r;for(const auto& [e,c]:p.t)r=r+K(c)*power(x,e[0])*power(y,e[1]);return r;
}
Point image(const K& x,const K& y) {
    require(eval(curve,x,y)==K(0),"closed point on actual quartic");
    K a=eval(maps[0],x,y),b=eval(maps[1],x,y),c=eval(maps[2],x,y),d=eval(maps[3],x,y);
    if(b!=K(0)&&d!=K(0)) { Point r(a/b,c/d);valid(r);return r; }
    auto jet=lift(homogeneous_curve,{R{x},R{y},R{K(1)}},2);
    Value vx=ratio(homogeneous_maps[0],homogeneous_maps[1],jet),vy=ratio(homogeneous_maps[2],homogeneous_maps[3],jet);
    Point r=endpoint(vx,vy);if(r.zero)++poles;return r;
}
Point negative(const Point& p) { return p.zero?p:Point(p.x,-p.y); }
Cycle read_cycle(const Json& row) {
    Cycle c{read_poly(row.at("support")),read_poly(row.at("x")),read_poly(row.at("y")),row.at("coordinate_lambda").integer()};
    require(c.u.degree()==3&&c.u.c.back()==F(1)&&c.x.degree()<3&&c.y.degree()<3,"entire degree-three cycle");
    require(c.coordinate>=0&&c.coordinate<=308,"retained separating-coordinate cap");
    Poly z{F(0),F(1)};require((c.x+c.y.scale(F(c.coordinate)))%c.u==z%c.u,"separated coordinate reconstruction");
    auto incidence=[&](const Poly& y) { Poly r;for(const auto& [e,a]:curve.t)r=(r+(power_mod(c.x,e[0],c.u)*power_mod(y,e[1],c.u)).scale(a))%c.u;return r; };
    require(incidence(c.y).c.empty(),"complete cycle quartic incidence with multiplicities");
    return c;
}
Poly polynomial_value(const Poly& f,const Poly& x,const Poly& u) { Poly r;for(int i=f.degree();i>=0;--i)r=(r*x+Poly{f.at(i)})%u;return r; }
Point norm(const Cycle& c) {
    auto factors=checked_factors(c.u);Point total;std::vector<Poly> seen;
    for(const auto& factor:factors) {
        bool prior=false;for(const auto& old:seen)if(old==factor)prior=true;if(prior)continue;seen.push_back(factor);
        int m=0;for(const auto& other:factors)if(other==factor)++m;instances+=m;++closed_points;
        residue_modulus=factor;Point p=image(K(c.x),K(c.y)),trace,conjugate=p;
        for(int i=0;i<factor.degree();++i) { trace=add(trace,conjugate);if(!conjugate.zero)conjugate=Point(conjugate.x.frobenius(),conjugate.y.frobenius()); }
        require(conjugate==p,"closed elliptic image Frobenius closure");
        if(!trace.zero)require(trace.x.p.degree()<=0&&trace.y.p.degree()<=0,"elliptic trace prime-field descent");
        total=add(total,times(trace,m));
    }return total;
}
void exact_binding(const Cycle& decoded,const Cycle& original) {
    Poly z=(original.x+original.y.scale(F(decoded.coordinate)))%original.u;
    require(polynomial_value(decoded.u,z,original.u).c.empty()&&polynomial_value(decoded.x,z,original.u)==original.x&&polynomial_value(decoded.y,z,original.u)==original.y,"exact repeated cycle binding");
    std::vector<std::vector<F>> columns;Poly p{F(1)};
    for(int i=0;i<3;++i) { std::vector<F> v;for(int j=0;j<3;++j)v.push_back(p.at(j));columns.push_back(v);p=(p*z)%original.u; }
    require(matrix_rank(columns)==3,"complete repeated cycle algebra rank");
}
int main(int argc,char** argv) { try {
    using Clock=std::chrono::steady_clock;auto last=Clock::now();std::vector<std::pair<std::string,long long>> phases;
    auto mark=[&](const std::string& name) { auto now=Clock::now();auto us=std::chrono::duration_cast<std::chrono::microseconds>(now-last).count();phases.emplace_back(name,us);last=now;std::cerr<<name<<" passed wall_us="<<us<<'\n'; };
    require(argc==7,"usage: verify model backmap extracted fixture pool formal");auto model=read_json(argv[1]),back=read_json(argv[2]),data=read_json(argv[3]),fixture=read_json(argv[4]),pool=read_json(argv[5]),formal=read_json(argv[6]);
    require(model.at("p").scalar==prime_text,"fixed P192 target field");curve=read_bp(model.at("terms"),true);
    std::array<std::string,4> names{"X_numerator","X_denominator","Y_numerator","Y_denominator"};int degree=0;
    for(size_t i=0;i<4;++i) { maps[i]=read_bp(back.at(names[i]));degree=std::max(degree,maps[i].degree());homogeneous_maps[i]=homogenize(maps[i]); }
    require(degree==11&&global_identity(curve,maps),"actual global elliptic-map equation");precision=4*degree+1;homogeneous_curve=homogenize(curve,4);mark("fixed_map_equation_setup");
    Cycle decoded=read_cycle(data.at("decoded").at("cycle"));Point actual=norm(decoded);int decoded_poles=poles;mark("decoded_cycle_factorization_map_and_trace");
    residue_modulus=Poly{F(0),F(1)};Point reference;const auto& ids=formal.at("seed_indices").arr;require(ids.size()==3,"actual complete reference size");
    require(data.at("reference_points").arr.size()==3,"certificate reference size");
    for(size_t i=0;i<3;++i) {
        const auto& p=pool.at("points").arr.at(size_t(ids[i].integer()));const auto& q=data.at("reference_points").arr[i];
        require(coefficient(p.at("x"))==coefficient(q.at("x"))&&coefficient(p.at("y"))==coefficient(q.at("y")),"actual reference coordinates bound");
        reference=add(reference,image(K(coefficient(p.at("x"))),K(coefficient(p.at("y")))));
    }
    actual=add(actual,negative(reference));mark("actual_reference_subtraction");
    const auto& expectation=fixture.at("expectation");std::string type=expectation.at("type").scalar;Point expected;
    if(type=="point_difference") {
        const auto& p=expectation.at("positive");const auto& q=expectation.at("negative");
        expected=add(image(K(coefficient(p.at("x"))),K(coefficient(p.at("y")))),negative(image(K(coefficient(q.at("x"))),K(coefficient(q.at("y"))))));
    } else if(type=="public_multiple") { expected=times(point(expectation.at("point")),expectation.at("multiple").integer()); }
    else if(type=="cycle") {
        auto original=read_cycle(expectation.at("cycle"));expected=add(norm(original),negative(reference));
        require(data.at("decoded").at("rr_dimension").integer()==expectation.at("rr_dimension").integer(),"fixed RR dimension");
        if(expectation.at("exact_support_required").scalar=="true")exact_binding(decoded,original);
        auto required=expectation.obj.find("pole_required");if(required!=expectation.obj.end()&&required->second.scalar=="true")require(decoded_poles>0,"actual decoded map-pole chart reached");
    } else require(type=="zero","known expectation family");
    require(actual==expected,"opaque class elliptic pushforward expectation");
    const auto& id=fixture.at("case_id").scalar;if(id.find("-m")!=std::string::npos)require(data.at("removed_infinity_multiplicity").integer()==id.back()-'0',"fixed infinity multiplicity detected");mark("independent_expectation_and_support_controls");
    Poly changed=decoded.y+Poly{F(1)},error;
    for(const auto& [e,a]:curve.t)error=(error+(power_mod(decoded.x,e[0],decoded.u)*power_mod(changed,e[1],decoded.u)).scale(a))%decoded.u;
    require(!error.c.empty(),"changed decoded ordinate rejects");
    auto factors=checked_factors(decoded.u);require(decoded.u.exact(factors[0]).degree()!=3,"omitted complete-support factor rejects");
    const Point g(K(F("602046282375688656758213480587526111916698976636884684818")),K(F("174050332293622031404857552280219410364023488927386650641")));valid(g);
    require(!g.zero&&!(add(actual,g)==expected),"changed elliptic answer rejects");mark("three_mutation_controls");
    std::cout<<"{\"scope\":\"separate-actual-f-pushforward-from-opaque-KM-decoded-cycle\",\"elliptic_result\":";emit(actual);
    std::cout<<",\"expectation_matched\":true,\"decoded_map_pole_charts\":"<<decoded_poles<<",\"closed_point_evaluations\":"<<closed_points<<",\"factor_instances_with_multiplicity\":"<<instances<<",\"precision\":"<<precision<<",\"mutation_controls_rejected\":3,\"signed_independent_verdict\":false,\"phase_us\":{";
    for(size_t i=0;i<phases.size();++i) { if(i)std::cout<<',';std::cout<<'"'<<phases[i].first<<"\":"<<phases[i].second; }std::cout<<"}}\n";
} catch(const std::exception& e) { std::cerr<<"REJECT "<<e.what()<<'\n';return 1; } }
