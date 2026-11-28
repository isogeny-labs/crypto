// Exact projective evaluation using the separate OpenSSL arithmetic backend.
#include "correspondence_base.hpp"

using Exponent=std::array<int,2>;
struct BP {
    std::map<Exponent,F> t;
    void term(Exponent e,const F& c) {
        if (c==F(0)) return;
        auto p=t.find(e);
        if (p==t.end()) t.emplace(e,c);
        else { p->second=p->second+c; if (p->second==F(0)) t.erase(p); }
    }
    int degree() const { int d=-1; for (const auto& a:t) d=std::max(d,a.first[0]+a.first[1]); return d; }
    BP operator+(const BP& a) const { BP r=*this; for (const auto& [e,c]:a.t) r.term(e,c); return r; }
    BP scale(const F& a) const { BP r; for (const auto& [e,c]:t) r.term(e,c*a); return r; }
    BP operator-(const BP& a) const { return *this+a.scale(F(-1)); }
    BP operator*(const BP& a) const {
        BP r; for (const auto& [e,c]:t) for (const auto& [g,d]:a.t) r.term({e[0]+g[0],e[1]+g[1]},c*d);
        return r;
    }
    BP partial(int axis) const {
        BP r; for (const auto& [e,c]:t) if (e[size_t(axis)]) { auto g=e; --g[size_t(axis)]; r.term(g,c*F(e[size_t(axis)])); }
        return r;
    }
    BP reduced(const BP& f) const {
        auto lead=f.t.find({0,4}); require(lead!=f.t.end() && lead->second!=F(0),"quartic y-four chart");
        F inverse=lead->second.inverse(); BP r=*this; int steps=0;
        for (;;) {
            auto p=r.t.end();
            for (auto q=r.t.begin();q!=r.t.end();++q)
                if (q->first[1]>=4 && (p==r.t.end() || q->first[1]>p->first[1])) p=q;
            if (p==r.t.end()) break;
            require(++steps<=200000,"bounded bivariate reduction"); auto e=p->first; F a=p->second*inverse;
            for (const auto& [g,c]:f.t) r.term({e[0]+g[0],e[1]-4+g[1]},-a*c);
        }
        return r;
    }
};
BP constant(const F& c) { BP r; r.term({0,0},c); return r; }
BP read_bp(const Json& input,bool homogeneous=false) {
    require(input.kind==JsonKind::array && input.arr.size()<=256,"bounded sparse polynomial"); BP r;
    for (const auto& row:input.arr) {
        const auto& e=row.at("exponents").arr; require(e.size()==(homogeneous?3u:2u),"polynomial exponent shape");
        int i=e[0].integer(),j=e[1].integer(); require(i>=0 && j>=0 && i+j<=32,"polynomial total-degree bound");
        if (homogeneous) require(e[2].integer()>=0 && i+j+e[2].integer()==4,"homogeneous quartic degree");
        F c=coefficient(row.at("coefficient")); require(c!=F(0),"sparse nonzero coefficient");
        require(r.t.emplace(Exponent{i,j},c).second,"unique sparse monomial");
    }
    return r;
}
bool global_identity(const BP& f,const std::array<BP,4>& map) {
    const BP& A=map[0];const BP& B=map[1];const BP& C=map[2];const BP& D=map[3];
    BP cubic=A*A*A-(A*B*B).scale(F(3))+(B*B*B).scale(F(b_text));
    return (C*C*B*B*B-D*D*cubic).reduced(f).t.empty();
}
int hilbert(const BP& f,const BP& a,const BP& b,int n) {
    std::map<Exponent,int> index; int width=0;
    for (int i=0;i<=n;++i) for (int j=0;i+j<=n;++j) index.emplace(Exponent{i,j},width++);
    std::vector<std::vector<F>> rows; const std::array<const BP*,3> gens{&f,&a,&b};
    for (size_t k=0;k<gens.size();++k) {
        int degree=k?6:4;
        for (int i=0;i<=n-degree;++i) for (int j=0;i+j<=n-degree;++j) {
            std::vector<F> row(size_t(width),F(0));
            for (const auto& [e,c]:gens[k]->t) row.at(size_t(index.at({e[0]+i,e[1]+j})))=c;
            rows.push_back(std::move(row));
        }
    }
    return width-matrix_rank(std::move(rows));
}
struct HP {
    int degree=0;
    std::map<std::array<int,3>,F> t;
    HP partial(int axis) const {
        HP r;r.degree=std::max(0,degree-1);
        for (const auto& [e,c]:t) if (e[size_t(axis)]) { auto g=e;--g[size_t(axis)];r.t.emplace(g,c*F(e[size_t(axis)])); }
        return r;
    }
};
HP homogenize(const BP& p,int degree=-1) {
    HP r;r.degree=degree<0?p.degree():degree;require(r.degree>=p.degree(),"homogenization degree");
    for (const auto& [e,c]:p.t) r.t.emplace(std::array<int,3>{e[0],e[1],r.degree-e[0]-e[1]},c);
    return r;
}
int precision=0;
R multiply(const R& a,const R& b) {
    if (a.c.empty() || b.c.empty()) return {};
    std::vector<K> r(size_t(std::min(precision,a.degree()+b.degree()+1)));
    for (int i=0;i<=a.degree() && i<precision;++i) if (a.at(i)!=K(0))
        for (int j=0;j<=b.degree() && i+j<precision;++j) if (b.at(j)!=K(0))
            r[size_t(i+j)]=r[size_t(i+j)]+a.at(i)*b.at(j);
    return R(std::move(r));
}
R inverse(const R& a) {
    require(a.at(0)!=K(0),"local-series unit");std::vector<K> r(size_t(precision),K(0));r[0]=a.at(0).inverse();
    for (int n=1;n<precision;++n) {
        K sum;
        for (int i=1;i<=n && i<=a.degree();++i) if (a.at(i)!=K(0) && r[size_t(n-i)]!=K(0)) sum=sum+a.at(i)*r[size_t(n-i)];
        r[size_t(n)]=-sum*r[0];
    }
    return R(std::move(r));
}
R evaluate(const HP& p,const std::array<R,3>& xyz) {
    std::array<std::vector<R>,3> powers;
    for (size_t k=0;k<3;++k) {
        powers[k].push_back(R{K(1)});
        for (int i=1;i<=p.degree;++i) powers[k].push_back(multiply(powers[k].back(),xyz[k]));
    }
    R result;
    for (const auto& [e,c]:p.t)
        result=result+multiply(multiply(powers[0][size_t(e[0])],powers[1][size_t(e[1])]),powers[2][size_t(e[2])]).scale(K(c));
    return result;
}
int order(const R& a) { int n=0;while (n<precision && a.at(n)==K(0)) ++n;require(n<precision,"precision exhausted for nonzero function");return n; }
K power(K a,int n) { if (n<0) { a=a.inverse();n=-n; }K r(1);for (int i=0;i<n;++i) r=r*a;return r; }
struct Jet { std::array<R,3> xyz;int solved=0,free=0; };
Jet lift(const HP& f,std::array<R,3> xyz,int chart) {
    require(evaluate(f,xyz).c.empty(),"local starting point incidence");
    const std::array<int,2> choices=chart==0?std::array<int,2>{1,2}:std::array<int,2>{1,0};
    int solved=-1;
    for (int axis:choices) if (evaluate(f.partial(axis),xyz).at(0)!=K(0)) { solved=axis;break; }
    require(solved>=0,"smooth supported local chart");int free=3-chart-solved;
    xyz[size_t(free)]=xyz[size_t(free)]+R{K(0),K(1)};
    for (int n=1;n<precision;n*=2) xyz[size_t(solved)]=xyz[size_t(solved)]-multiply(evaluate(f,xyz),inverse(evaluate(f.partial(solved),xyz)));
    require(evaluate(f,xyz).c.empty(),"full local quartic lift");return {xyz,solved,free};
}
struct Value { int valuation=0;K leading;int numerator_order=0,denominator_order=0; };
Value ratio(const HP& a,const HP& b,const Jet& jet) {
    R aa=evaluate(a,jet.xyz),bb=evaluate(b,jet.xyz);int oa=order(aa),ob=order(bb),oz=order(jet.xyz[2]);
    int exponent=b.degree-a.degree;
    return {oa-ob+exponent*oz,aa.at(oa)/bb.at(ob)*power(jet.xyz[2].at(oz),exponent),oa,ob};
}
Point endpoint(const Value& x,const Value& y) {
    if (x.valuation>=0 && y.valuation>=0) {
        Point p(x.valuation?K(0):x.leading,y.valuation?K(0):y.leading);valid(p);return p;
    }
    require(x.valuation<0 && y.valuation<0 && x.valuation%2==0 && 2*y.valuation==3*x.valuation,"elliptic infinity pole orders");
    require(y.leading*y.leading==x.leading*x.leading*x.leading,"elliptic infinity leading equation");return {};
}
struct Boundary { Poly factor,x,y;int multiplicity=0,solved=0,free=0,z_order=0,vx=0,vy=0;bool infinity=false; };
Boundary boundary(const HP& f,const std::array<HP,4>& map,const K& root,const Poly& factor,int multiplicity) {
    Jet jet=lift(f,{R{K(1)},R{root},R{}},0);
    Value x=ratio(map[0],map[1],jet),y=ratio(map[2],map[3],jet);Point p=endpoint(x,y);int z=order(jet.xyz[2]);
    require(z==multiplicity,"line-intersection multiplicity matches local order");
    return {factor,p.x.p,p.y.p,multiplicity,jet.solved,jet.free,z,x.valuation,y.valuation,p.zero};
}
R stripped(const R& a,int n) { std::vector<K> c;for (int i=n;i<precision;++i) c.push_back(a.at(i));return R(std::move(c)); }
void verify_factor_product(const std::vector<Poly>& factors,const Poly& q) {
    Poly product{F(1)};for (const auto& f:factors) product=product*f;require(product==q,"all infinity factors retained");
}
int main(int argc,char** argv) { try {
    using Clock=std::chrono::steady_clock;auto last=Clock::now();std::vector<std::pair<std::string,long long>> phases;
    auto mark=[&](const std::string& s){auto now=Clock::now();auto us=std::chrono::duration_cast<std::chrono::microseconds>(now-last).count();phases.emplace_back(s,us);last=now;std::cerr<<s<<" passed wall_us="<<us<<'\n';};
    require(argc==3,"usage: projective-check quartic.json backmap.json");Json model=read_json(argv[1]),data=read_json(argv[2]);
    require(model.at("p").scalar==prime_text && data.at("section_degree").integer()==6,"fixed model/section pins");
    BP f=read_bp(model.at("terms"),true);std::array<BP,4> maps;
    const std::array<std::string,4> names{"X_numerator","X_denominator","Y_numerator","Y_denominator"};int max_degree=0;
    for (size_t i=0;i<4;++i) { maps[i]=read_bp(data.at(names[i]));require(!maps[i].reduced(f).t.empty(),"nonzero rational-map function");max_degree=std::max(max_degree,maps[i].degree()); }
    require(max_degree<=32 && max_degree>0,"bounded map degree");precision=4*max_degree+1;
    require(global_identity(f,maps),"separate full rational elliptic identity");
    require(F(-108)+F(27)*F(b_text)*F(b_text)!=F(0),"smooth elliptic codomain");mark("input_and_global_identity");
    int h14=hilbert(f,maps[0],maps[1],14),h15=hilbert(f,maps[0],maps[1],15);
    require(h14==12 && h15==12,"independent homogeneous base-divisor dimensions");mark("degree_certificate");
    HP curve=homogenize(f,4);std::array<HP,4> hm;
    for (size_t i=0;i<4;++i) hm[i]=homogenize(maps[i]);
    std::vector<F> qc(5,F(0));for (const auto& [e,c]:f.t) if (e[0]+e[1]==4) qc[size_t(e[1])]=qc[size_t(e[1])]+c;
    Poly q(qc);require(q.degree()==4 && q.at(4)!=F(0),"all infinity points in X-one chart");q=q.monic();
    auto factors=checked_factors(q);verify_factor_product(factors,q);std::vector<Poly> distinct;std::vector<int> multiplicity;
    for (const auto& p:factors) {
        size_t i=0;while (i<distinct.size() && !(distinct[i]==p)) ++i;
        if (i==distinct.size()) { distinct.push_back(p);multiplicity.push_back(1); }else ++multiplicity[i];
    }
    std::vector<Boundary> results;int conjugate_checks=0;
    for (size_t i=0;i<distinct.size();++i) {
        residue_modulus=distinct[i];K root(Poly{F(0),F(1)});Boundary r=boundary(curve,hm,root,distinct[i],multiplicity[i]);
        Point image;image.zero=r.infinity;if (!image.zero) image=Point(K(r.x),K(r.y));
        Point conjugate=image;
        for (int k=0;k<distinct[i].degree();++k) if (!conjugate.zero) conjugate=Point(conjugate.x.frobenius(),conjugate.y.frobenius());
        require(conjugate==image,"endpoint field Frobenius closure");
        if (distinct[i].degree()>1) {
            Boundary next=boundary(curve,hm,root.frobenius(),distinct[i],multiplicity[i]);Point expected=image;
            if (!expected.zero) expected=Point(expected.x.frobenius(),expected.y.frobenius());
            Point got;if (!next.infinity) got=Point(K(next.x),K(next.y));require(got==expected,"fresh conjugate endpoint agrees");++conjugate_checks;
        }
        results.push_back(r);mark("infinity_closed_point_"+std::to_string(i));
    }
    residue_modulus=Poly{F(0),F(1)};
    require(data.at("point").arr.size()==2 && data.at("point_receipt_index").integer()==0,"retained affine base point");
    Jet affine=lift(curve,{R{K(coefficient(data.at("point").arr[0]))},R{K(coefficient(data.at("point").arr[1]))},R{K(1)}},2);
    require(affine.solved==1 && affine.free==0,"retained affine y-of-x chart");
    Value ax=ratio(hm[0],hm[1],affine),ay=ratio(hm[2],hm[3],affine);
    require(ax.valuation==-2 && ay.valuation==-3 && endpoint(ax,ay).zero,"affine denominator-zero image is identity");
    require(data.at("local_y").arr.size()>=size_t(precision),"recorded source-series length");
    for (int i=0;i<precision;++i) require(affine.xyz[1].at(i)==K(coefficient(data.at("local_y").arr[size_t(i)])),"source-y coefficient matches original GMP series");
    R aa=evaluate(hm[0],affine.xyz),bb=evaluate(hm[1],affine.xyz);
    require(precision-std::max(ax.numerator_order,ax.denominator_order)>=24,"guaranteed scaled-X comparison precision");
    R scaled=multiply(stripped(aa,ax.numerator_order),inverse(stripped(bb,ax.denominator_order)));
    for (int i=0;i<24;++i) require(scaled.at(i)==K(coefficient(data.at("local_scaled_X").arr.at(size_t(i)))),"scaled-X coefficient matches original GMP series");
    mark("affine_base_point_and_cross_library_series");
    rejects([&]{auto changed=maps;changed[0]=changed[0]+constant(F(1));require(global_identity(f,changed),"changed X numerator");});
    rejects([&]{auto changed=maps;changed[2]=changed[2]+constant(F(1));require(global_identity(f,changed),"changed Y numerator");});
    rejects([&]{auto changed=factors;changed.pop_back();verify_factor_product(changed,q);});
    rejects([&]{auto changed=affine.xyz;auto c=changed[size_t(affine.solved)].c;c.resize(size_t(precision));c.back()=c.back()+K(1);changed[size_t(affine.solved)]=R(std::move(c));require(evaluate(curve,changed).c.empty(),"changed highest local-jet coefficient");});
    rejects([&]{auto changed=ax;++changed.valuation;(void)endpoint(changed,ay);});mark("five_mutation_controls");
    std::cout<<"{\"standing\":\"unregistered producer-unverified separate native point-map verification\",\"precision\":"<<precision<<",\"global_map_identity_checked\":true,\"Hilbert_dimensions\":["<<h14<<','<<h15<<"],\"curve_map_degree\":"<<(24-h14)/2<<",\"infinity_factor_instances\":"<<factors.size()<<",\"infinity_closed_points\":"<<results.size()<<",\"fresh_conjugate_checks\":"<<conjugate_checks<<",\"affine_base_point_identity_checked\":true,\"source_y_coefficients_compared\":"<<precision<<",\"scaled_X_coefficients_compared\":24,\"mutation_controls_rejected\":5,\"full_projective_Jacobian_transpose_implemented\":false,\"signed_independent_verdict\":false,\"infinity_polynomial\":";
    emit(q);std::cout<<",\"points\":[";
    for (size_t i=0;i<results.size();++i) {
        if (i) std::cout<<',';const auto& r=results[i];std::cout<<"{\"factor\":";emit(r.factor);
        std::cout<<",\"multiplicity\":"<<r.multiplicity<<",\"solved_axis\":"<<r.solved<<",\"free_axis\":"<<r.free<<",\"Z_order\":"<<r.z_order<<",\"X_valuation\":"<<r.vx<<",\"Y_valuation\":"<<r.vy<<",\"image\":{\"infinity\":"<<(r.infinity?"true":"false");
        if (!r.infinity) { std::cout<<",\"x\":";emit(r.x);std::cout<<",\"y\":";emit(r.y); }std::cout<<"}}";
    }
    std::cout<<"],\"phase_us\":{";for (size_t i=0;i<phases.size();++i) { if (i) std::cout<<',';std::cout<<'"'<<phases[i].first<<"\":"<<phases[i].second; }std::cout<<"}}\n";
    return 0;
} catch (const std::exception& e) { std::cerr<<"REJECT "<<e.what()<<'\n';return 1; } }
