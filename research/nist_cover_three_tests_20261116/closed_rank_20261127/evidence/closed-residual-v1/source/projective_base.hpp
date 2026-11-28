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
