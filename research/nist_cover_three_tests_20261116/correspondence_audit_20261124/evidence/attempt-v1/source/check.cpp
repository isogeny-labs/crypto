// New correspondence replay atop the separate OpenSSL arithmetic, not producer code.
#include "separate_base.hpp"
#include <array>

Poly wide_power(Poly a, const BIGNUM* n, const Poly& m) {
    Poly r{F(1)}; a=a%m;
    for (int bit=BN_num_bits(n)-1; bit>=0; --bit) {
        r=(r*r)%m; if (BN_is_bit_set(n,bit)) r=(r*a)%m;
    }
    return r;
}
Poly gcd(const Poly& a,const Poly& b) { return std::get<0>(bezout(a,b)); }
void equal_factors(Poly f,int d,std::vector<Poly>& out) {
    if (f.degree()==d) { out.push_back(f.monic()); return; }
    require(f.degree()>d && f.degree()%d==0,"equal-degree block");
    BN n=number("1"); for (int i=0;i<d;++i) require(BN_mul(n.get(),n.get(),prime(),context())==1,"extension order");
    require(BN_sub_word(n.get(),1)==1 && BN_rshift1(n.get(),n.get())==1,"half extension order");
    for (int s=0;s<1024;++s) {
        Poly g=gcd(f,wide_power(Poly{F(s),F(1)},n.get(),f)-Poly{F(1)});
        if (g.degree()>0 && g.degree()<f.degree()) {
            equal_factors(g,d,out); equal_factors(f.exact(g).monic(),d,out); return;
        }
    }
    throw std::runtime_error("frozen equal-degree split cap exhausted");
}
void factor(Poly f,std::vector<Poly>& out) {
    f=f.monic(); if (f.degree()==0) return;
    Poly repeat=gcd(f,derivative(f));
    if (repeat.degree()>0) { factor(f.exact(repeat),out); factor(repeat,out); return; }
    Poly x{F(0),F(1)}, p=x;
    for (int d=1;2*d<=f.degree();++d) {
        p=wide_power(p,prime(),f); Poly block=gcd(f,p-x);
        if (block.degree()>0) {
            equal_factors(block,d,out); f=f.exact(block).monic();
            if (f.degree()==0) return;
            p=p%f;
        }
    }
    if (f.degree()>0) out.push_back(f);
}
void irreducible(const Poly& f) {
    Poly x{F(0),F(1)}, p=x%f;
    for (int i=1;i<=f.degree();++i) {
        p=wide_power(p,prime(),f);
        if (i<f.degree() && f.degree()%i==0) require(gcd(f,p-x).degree()==0,"irreducible factor proper subfield");
    }
    require(p==x%f,"factor Frobenius closure");
}
std::vector<Poly> checked_factors(const Poly& f) {
    std::vector<Poly> r; factor(f,r); Poly product{F(1)};
    for (const auto& p:r) { irreducible(p); product=product*p; }
    require(product==f,"complete factor product with multiplicities"); return r;
}
Poly residue_modulus{F(0),F(1)};
struct K {
    Poly p;
    K(long x=0):p{F(x)} {}
    explicit K(const F& x):p{x} {}
    explicit K(Poly a):p(a%residue_modulus) {}
    bool operator==(const K& a) const { return p==a.p; }
    bool operator!=(const K& a) const { return !(*this==a); }
    K operator+(const K& a) const { return K(p+a.p); }
    K operator-() const { return K(-p); }
    K operator-(const K& a) const { return K(p-a.p); }
    K operator*(const K& a) const { return K((p*a.p)%residue_modulus); }
    K inverse() const {
        if (p.degree()==0) return K(p.at(0).inverse());
        auto [g,s,t]=bezout(p,residue_modulus); require(g.degree()==0,"residue-algebra unit"); return K(s);
    }
    K operator/(const K& a) const { return *this*a.inverse(); }
    K frobenius() const { return K(wide_power(p,prime(),residue_modulus)); }
};
struct R {
    std::vector<K> c;
    R()=default; R(std::initializer_list<K> a):c(a) { trim(); }
    explicit R(std::vector<K> a):c(std::move(a)) { trim(); }
    void trim() { while (!c.empty() && c.back()==K(0)) c.pop_back(); }
    int degree() const { return int(c.size())-1; }
    K at(int i) const { return i>=0 && i<=degree()?c[size_t(i)]:K(0); }
    bool operator==(const R& a) const { return c==a.c; }
    R operator+(const R& a) const {
        std::vector<K> r(std::max(c.size(),a.c.size()));
        for (size_t i=0;i<r.size();++i) r[i]=at(int(i))+a.at(int(i));
        return R(std::move(r));
    }
    R operator-() const { auto r=c; for (auto& k:r) k=-k; return R(std::move(r)); }
    R operator-(const R& a) const { return *this+(-a); }
    R scale(const K& a) const { auto r=c; for (auto& k:r) k=k*a; return R(std::move(r)); }
    R operator*(const R& a) const {
        if (c.empty() || a.c.empty()) return {};
        std::vector<K> r(c.size()+a.c.size()-1);
        for (size_t i=0;i<c.size();++i) for (size_t j=0;j<a.c.size();++j) r[i+j]=r[i+j]+c[i]*a.c[j];
        return R(std::move(r));
    }
    std::pair<R,R> divide(const R& a) const {
        require(a.degree()>=0,"extension polynomial divisor"); R r=*this;
        std::vector<K> q(size_t(std::max(0,degree()-a.degree()+1))); K inv=a.c.back().inverse();
        while (!r.c.empty() && r.degree()>=a.degree()) {
            int k=r.degree()-a.degree(); K t=r.c.back()*inv; q[size_t(k)]=t;
            for (int j=0;j<=a.degree();++j) r.c[size_t(j+k)]=r.c[size_t(j+k)]-t*a.c[size_t(j)];
            r.trim();
        }
        return {R(std::move(q)),r};
    }
    R operator%(const R& a) const { return divide(a).second; }
    R exact(const R& a) const { auto qr=divide(a); require(qr.second.c.empty(),"exact extension polynomial quotient"); return qr.first; }
    R monic() const { require(!c.empty(),"nonzero extension polynomial"); return scale(c.back().inverse()); }
};
R lift(const Poly& p) { std::vector<K> c; for (const auto& x:p.c) c.emplace_back(x); return R(std::move(c)); }
Poly descend(const R& p) {
    std::vector<F> c; for (const auto& x:p.c) { require(x.p.degree()<=0,"prime-field descent"); c.push_back(x.p.at(0)); }
    return Poly(std::move(c));
}
std::tuple<R,R,R> bezout(R a,R b) {
    R s{K(1)},t,ns,nt{K(1)};
    while (!b.c.empty()) {
        auto qr=a.divide(b); R ss=s-qr.first*ns,tt=t-qr.first*nt;
        a=b; b=qr.second; s=ns; t=nt; ns=ss; nt=tt;
    }
    require(!a.c.empty(),"nonzero extension gcd"); K unit=a.c.back().inverse();
    return {a.scale(unit),s.scale(unit),t.scale(unit)};
}
R gcd(const R& a,const R& b) { return std::get<0>(bezout(a,b)); }
R evaluate(const Poly& p,const R& x,const R& m) {
    R r; for (int i=p.degree();i>=0;--i) r=(r*x+R{K(p.at(i))})%m; return r;
}
K evaluate(const Poly& p,const K& x) {
    K r; for (int i=p.degree();i>=0;--i) r=r*x+K(p.at(i)); return r;
}
struct J {
    R u{K(1)},v;
    bool operator==(const J& a) const { return u==a.u && v==a.v; }
};
void valid(const J& d,const R& h) {
    require(d.u.degree()>=0 && d.u.degree()<=3 && d.u.c.back()==K(1) && d.v.degree()<d.u.degree(),"extension reduced divisor shape");
    require(((d.v*d.v-h)%d.u).c.empty(),"extension source incidence");
}
J reduce(R u,R v,const R& h) {
    u=u.monic(); v=v%u; require(((v*v-h)%u).c.empty(),"raw source incidence");
    while (u.degree()>3) {
        R next=(v*v-h).exact(u).monic(); require(next.degree()<u.degree(),"extension strict reduction");
        v=(-v)%next; u=next;
    }
    J r{u,v}; valid(r,h); return r;
}
J add(const J& a,const J& b,const R& h) {
    valid(a,h); valid(b,h);
    auto [g,e,f]=bezout(a.u,b.u); auto [s,r,t]=bezout(g,a.v+b.v);
    require(r*e*a.u+r*f*b.u+t*(a.v+b.v)==s,"extension Bezout certificate");
    R u=(a.u*b.u).exact(s*s),v=(r*e*a.u*b.v+r*f*b.u*a.v+t*(a.v*b.v+h)).exact(s)%u;
    return reduce(u,v,h);
}
J frobenius(const J& a) {
    auto u=a.u.c,v=a.v.c; for (auto& x:u) x=x.frobenius(); for (auto& x:v) x=x.frobenius();
    return {R(std::move(u)),R(std::move(v))};
}
struct Graph {
    struct Entry { Poly a,b,c,aa,bb; };
    std::array<std::vector<Entry>,2> rows;
    Poly den{F(1)};
    static Poly read_graph_poly(const Json& j) {
        require(j.kind==JsonKind::array && j.arr.size()<=128,"bounded graph polynomial");
        std::vector<F> c; for (const auto& x:j.arr) c.push_back(coefficient(x));
        require(c.empty() || c.back()!=F(0),"canonical graph polynomial");
        return Poly(std::move(c));
    }
    explicit Graph(const Json& global) {
        for (int k=0;k<2;++k) for (const auto& j:global.at(k?"V":"U").arr) {
            Entry e{read_graph_poly(j.at("a")),read_graph_poly(j.at("b")),read_graph_poly(j.at("c")),{}, {}};
            den=(den*e.c).exact(gcd(den,e.c)).monic(); rows[size_t(k)].push_back(std::move(e));
        }
        require(rows[0].size()==4 && rows[1].size()==3,"global cubic/quadratic shape");
        for (auto& list:rows) for (auto& e:list) { Poly scale=den.exact(e.c); e.aa=e.a*scale; e.bb=e.b*scale; }
    }
};
int matrix_rank(std::vector<std::vector<F>> a) {
    int row=0;
    for (size_t col=0;col<a[0].size() && row<int(a.size());++col) {
        int pivot=row; while (pivot<int(a.size()) && a[size_t(pivot)][col]==F(0)) ++pivot;
        if (pivot==int(a.size())) continue;
        std::swap(a[size_t(row)],a[size_t(pivot)]); F inv=a[size_t(row)][col].inverse();
        for (auto& x:a[size_t(row)]) x=x*inv;
        for (size_t j=size_t(row+1);j<a.size();++j) {
            F scale=a[j][col]; for (size_t k=col;k<a[j].size();++k) a[j][k]=a[j][k]-scale*a[size_t(row)][k];
        }
        ++row;
    }
    return row;
}
void forward(const Graph& graph,const Divisor& source,const Json& record,const Json& model) {
    require(source.u.degree()==3,"fixed forward source degree three"); cycle(record,model);
    residue_modulus=source.u;
    std::array<R,2> maps;
    for (int k=0;k<2;++k) {
        std::vector<K> coeff;
        for (const auto& e:graph.rows[size_t(k)]) coeff.push_back((K(e.a)+K(e.b)*K(source.v))/K(e.c));
        maps[size_t(k)]=R(std::move(coeff));
    }
    R u=maps[0],x{K(0),K(1)},y=maps[1];
    require(u.degree()==3 && u.c.back()==K(1),"rank-three monic forward algebra");
    R z=x+y.scale(K(record.at("coordinate_lambda").integer()));
    require(evaluate(read_poly(record.at("support")),z,u).c.empty(),"recorded forward support binding");
    require(evaluate(read_poly(record.at("x")),z,u)==x%u,"recorded forward x binding");
    require(evaluate(read_poly(record.at("y")),z,u)==y%u,"recorded forward y binding");
    R p{K(1)}; std::vector<std::vector<F>> rows;
    for (int i=0;i<9;++i) {
        std::vector<F> row;
        for (int j=0;j<3;++j) for (int k=0;k<3;++k) row.push_back(p.at(j).p.at(k));
        rows.push_back(std::move(row)); p=(p*z)%u;
    }
    require(matrix_rank(rows)==9,"full nine-dimensional forward power basis");
    residue_modulus=Poly{F(0),F(1)};
}
struct Fiber { J reduced; int raw_degree=0,saturations=0; };
Fiber fiber(const Graph& graph,const Poly& source_curve,const K& tx,const K& ty,int expected) {
    std::array<R,4> eq;
    for (int k=0;k<2;++k) {
        K p(1);
        for (const auto& e:graph.rows[size_t(k)]) {
            eq[size_t(2*k)]=eq[size_t(2*k)]+lift(e.aa).scale(p);
            eq[size_t(2*k+1)]=eq[size_t(2*k+1)]+lift(e.bb).scale(p); p=p*tx;
        }
    }
    eq[2]=eq[2]-lift(graph.den).scale(ty); R h=lift(source_curve);
    R u=gcd(gcd(eq[0]*eq[3]-eq[2]*eq[1],eq[0]*eq[0]-eq[1]*eq[1]*h),eq[2]*eq[2]-eq[3]*eq[3]*h);
    int saturations=0;
    for (;;) { R d=gcd(u,lift(graph.den)); if (d.degree()==0) break; u=u.exact(d).monic(); ++saturations; }
    require(u.degree()==expected,"qualified raw transpose degree");
    auto [g,s,t]=bezout(eq[1]%u,eq[3]%u); auto [unit,ss,tt]=bezout(g,u);
    require(unit.degree()==0,"transpose ordinate recovery unit");
    R v=(-(s*eq[0]+t*eq[2])*ss)%u;
    require(((eq[0]+eq[1]*v)%u).c.empty() && ((eq[2]+eq[3]*v)%u).c.empty(),"both transpose graph equations");
    return {reduce(u,v,h),u.degree(),saturations};
}
Divisor transpose(const Graph& graph,const Poly& h,const Json& record,std::vector<int>& degrees) {
    Poly support=read_poly(record.at("support")),x=read_poly(record.at("x")),y=read_poly(record.at("y"));
    Divisor total;
    for (const auto& f:checked_factors(support)) {
        residue_modulus=f; K z(Poly{F(0),F(1)});
        Fiber raw=fiber(graph,h,evaluate(x,z),evaluate(y,z),9); J sum,conjugate=raw.reduced; R hh=lift(h);
        for (int i=0;i<f.degree();++i) { sum=add(sum,conjugate,hh); conjugate=frobenius(conjugate); }
        require(conjugate==raw.reduced,"transpose Frobenius conjugate closure");
        Divisor descended{descend(sum.u),descend(sum.v)}; valid(descended,h);
        residue_modulus=Poly{F(0),F(1)}; total=add(total,descended,h); degrees.push_back(f.degree());
        std::cerr << "transpose factor degree=" << f.degree() << " raw=" << raw.raw_degree << " saturated=" << raw.saturations << '\n';
    }
    return total;
}
struct Point {
    K x,y; bool zero=true;
    Point()=default; Point(K a,K b):x(std::move(a)),y(std::move(b)),zero(false) {}
    bool operator==(const Point& p) const { return zero==p.zero && (zero || (x==p.x && y==p.y)); }
};
void valid(const Point& p) { if (!p.zero) require(p.y*p.y==p.x*p.x*p.x-K(3)*p.x+K(F(b_text)),"elliptic extension incidence"); }
Point add(const Point& a,const Point& b) {
    if (a.zero) return b;
    if (b.zero) return a;
    if (a.x==b.x && a.y==-b.y) return {};
    K s=(a==b)?(K(3)*a.x*a.x-K(3))/(K(2)*a.y):(b.y-a.y)/(b.x-a.x);
    K xx=s*s-a.x-b.x; Point p(xx,s*(a.x-xx)-a.y); valid(p); return p;
}
Point times(Point a,int n) { Point r; for (int i=0;i<n;++i) r=add(r,a); return r; }
Point point(const Json& j) {
    if (j.at("infinity").scalar=="true") return {};
    Point r(K(coefficient(j.at("x"))),K(coefficient(j.at("y")))); valid(r); return r;
}
Point norm(const Divisor& d) {
    Point total;
    for (const auto& f:checked_factors(d.u)) {
        residue_modulus=f; K X(Poly{F(0),F(1)}),Y(d.v),C(1024); Point image;
        if (X==K(0)) image=Point(K(1),C*C*C*C*Y);
        else {
            K t=K(1)+K(1)/(C*X),den=t*t-K(1);
            if (den!=K(0)) image=Point((t*t+K(7))/den,Y/(X*X*X*X*den*den));
        }
        valid(image); Point trace,conjugate=image;
        for (int i=0;i<f.degree();++i) {
            trace=add(trace,conjugate);
            if (!conjugate.zero) conjugate=Point(conjugate.x.frobenius(),conjugate.y.frobenius());
        }
        require(conjugate==image,"norm Frobenius closure");
        if (!trace.zero) {
            require(trace.x.p.degree()<=0 && trace.y.p.degree()<=0,"elliptic trace descent");
            F xx=trace.x.p.at(0),yy=trace.y.p.at(0); residue_modulus=Poly{F(0),F(1)};
            total=add(total,Point(K(xx),K(yy)));
        }
        residue_modulus=Poly{F(0),F(1)};
    }
    return total;
}
std::string decimal(const F& f) { char* c=BN_bn2dec(f.n.get()); require(c!=nullptr,"BN decimal output"); std::string s(c); OPENSSL_free(c); return s; }
void emit(const Poly& p) { std::cout << '['; for (size_t i=0;i<p.c.size();++i) { if (i) std::cout << ','; std::cout << '"' << decimal(p.c[i]) << '"'; } std::cout << ']'; }
void emit(const Divisor& d) { std::cout << "{\"U\":"; emit(d.u); std::cout << ",\"V\":"; emit(d.v); std::cout << '}'; }
void emit(const Point& p) {
    std::cout << "{\"infinity\":" << (p.zero?"true":"false");
    if (!p.zero) std::cout << ",\"x\":\"" << decimal(p.x.p.at(0)) << "\",\"y\":\"" << decimal(p.y.p.at(0)) << '"';
    std::cout << '}';
}
int main(int argc,char** argv) { try {
    using Clock=std::chrono::steady_clock; auto last=Clock::now();
    std::vector<std::pair<std::string,long long>> phases;
    auto mark=[&](const std::string& s) { auto now=Clock::now(); auto us=std::chrono::duration_cast<std::chrono::microseconds>(now-last).count(); phases.emplace_back(s,us); last=now; std::cerr << s << " passed wall_us=" << us << '\n'; };
    require(argc==7,"usage: correspondence-check cover quartic transfer global formal relations");
    Json source=read_json(argv[1]),model=read_json(argv[2]),input=read_json(argv[3]),global=read_json(argv[4]),formal=read_json(argv[5]),relations=read_json(argv[6]);
    require(source.at("source_model").at("p").scalar==prime_text && source.at("source_model").at("b").scalar==b_text && source.at("cover_parameter_d").integer()==7,"fixed source pins");
    panel(input); Graph graph(global); Poly h=cover(); mark("input_and_graph_setup");
    std::vector<Divisor> refs; Divisor reference; auto& seeds=formal.at("seed_indices").arr;
    require(seeds.size()==3 && seeds[0].integer()==225 && seeds[1].integer()==224 && seeds[2].integer()==223,"fixed reference indices");
    for (const auto& seed:seeds) {
        const auto& p=relations.at("pool").at("points").arr.at(size_t(seed.integer()));
        Fiber f=fiber(graph,h,K(coefficient(p.at("x"))),K(coefficient(p.at("y"))),8);
        Divisor r{descend(f.reduced.u),descend(f.reduced.v)}; valid(r,h); refs.push_back(r); reference=add(reference,r,h);
    }
    mark("three_reference_fibers_degree_eight");
    std::vector<Divisor> duals; std::vector<Point> norms; std::vector<std::vector<int>> factors;
    for (size_t i=0;i<6;++i) {
        const auto& row=input.at("fixtures").arr[i]; Divisor d=pull(row.at("point"),h),dual; std::vector<int> degree;
        if (i) {
            forward(graph,d,row.at("forward_cycle"),model);
            dual=transpose(graph,h,row.at("forward_cycle"),degree);
            Divisor subtraction; for (int j=0;j<d.u.degree();++j) subtraction=add(subtraction,reference,h);
            dual=add(dual,negate(subtraction),h);
        }
        Divisor recorded{read_poly(row.at("source_dual").at("U")),read_poly(row.at("source_dual").at("V"))};
        require(dual==recorded && dual==add(add(d,d,h),d,h),"actual transpose reference cancellation equals triple pullback");
        Point result=norm(dual),expected=times(point(row.at("point")),6);
        require(result==point(row.at("norm")) && result==expected,"actual norm and sixfold source point");
        duals.push_back(dual); norms.push_back(result); factors.push_back(degree); mark("complete_fixture_"+std::to_string(i));
    }
    const auto& first=input.at("fixtures").arr[1]; Divisor d=pull(first.at("point"),h);
    rejects([&]{auto bad=first.at("forward_cycle"); bad.obj["y"].arr[0].scalar="0"; forward(graph,d,bad,model);});
    rejects([&]{valid(negate(duals[1]),h); require(negate(duals[1])==duals[1],"valid wrong dual");});
    rejects([&]{require(add(norms[1],point(first.at("point")))==norms[1],"changed elliptic norm");});
    bool changed_ref=false;
    for (const auto& r:refs) { Divisor triple=add(add(r,r,h),r,h); if (!(triple==Divisor{})) { rejects([&]{require(add(duals[1],triple,h)==duals[1],"dropped reference summand");}); changed_ref=true; break; } }
    require(changed_ref,"nonzero omitted reference control");
    rejects([&]{Graph bad=graph; bad.rows[0][0].a=bad.rows[0][0].a+Poly{F(1)}; forward(bad,d,first.at("forward_cycle"),model);});
    mark("five_mutation_controls");
    std::cout << "{\"standing\":\"producer-unverified separate native correspondence replay\",\"forward_bindings\":5,\"transpose_trace_comparisons\":5,\"full_composition6_checks\":6,\"reference_raw_degrees\":[8,8,8],\"mutation_controls_rejected\":5,\"fixed_panel_correspondence_replayed\":true,\"arbitrary_projective_charts_proved\":false,\"signed_independent_verdict\":false,\"reference\":";
    emit(reference); std::cout << ",\"fixtures\":[";
    for (size_t i=0;i<6;++i) { if (i) std::cout << ','; std::cout << "{\"index\":" << i << ",\"source_dual\":"; emit(duals[i]); std::cout << ",\"norm\":"; emit(norms[i]); std::cout << ",\"factor_degrees\":["; for (size_t j=0;j<factors[i].size();++j) { if (j) std::cout << ','; std::cout << factors[i][j]; } std::cout << "]}"; }
    std::cout << "],\"phase_us\":{"; for (size_t i=0;i<phases.size();++i) { if (i) std::cout << ','; std::cout << '"' << phases[i].first << "\":" << phases[i].second; } std::cout << "}}\n";
    return 0;
} catch (const std::exception& e) { std::cerr << "REJECT " << e.what() << '\n'; return 1; } }
