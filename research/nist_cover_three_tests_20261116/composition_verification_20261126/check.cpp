// Actual-coordinate differential and degree verification, separate OpenSSL backend.
#include "projective_base.hpp"

Poly source_h;
Poly load_poly(const Json& j) {
    require(j.kind==JsonKind::array && j.arr.size()<=129,"function coefficient degree cap");
    std::vector<F> c; for (const auto& x:j.arr) c.push_back(coefficient(x));
    require(c.empty() || c.back()!=F(0),"canonical function coefficient tail");
    return Poly(std::move(c));
}
Poly pgcd(const Poly& a,const Poly& b) {
    if (a.c.empty()) return b.monic();
    if (b.c.empty()) return a.monic();
    return std::get<0>(bezout(a,b));
}
struct NF {
    Poly a,b,c{F(1)};
    NF(long n=0):a{F(n)} {}
    NF(Poly aa,Poly bb={},Poly cc={F(1)}):a(std::move(aa)),b(std::move(bb)),c(std::move(cc)) {
        require(!c.c.empty(),"nonzero function denominator");
        if (a.c.empty() && b.c.empty()) { c=Poly{F(1)}; return; }
        if (c.degree()>0) {
            Poly g=pgcd(a,c); if (g.degree()>0) g=pgcd(b,g);
            if (g.degree()>0) { a=a.exact(g); b=b.exact(g); c=c.exact(g); }
        }
        F unit=c.c.back().inverse(); a=a.scale(unit); b=b.scale(unit); c=c.scale(unit);
    }
    bool operator==(const NF& z) const { return a==z.a && b==z.b && c==z.c; }
    bool operator!=(const NF& z) const { return !(*this==z); }
    NF operator-() const { return NF(-a,-b,c); }
    NF operator+(const NF& z) const {
        if (c==z.c) return NF(a+z.a,b+z.b,c);
        return NF(a*z.c+z.a*c,b*z.c+z.b*c,c*z.c);
    }
    NF operator-(const NF& z) const { return *this+(-z); }
    NF operator*(const NF& z) const {
        return NF(a*z.a+(b*z.b)*source_h,a*z.b+b*z.a,c*z.c);
    }
    NF inverse() const {
        require(*this!=NF(0),"nonzero source function inverse");
        NF r=b.c.empty()?NF(c,{},a):NF(a*c,(-b)*c,a*a-(b*b)*source_h);
        require(*this*r==NF(1),"exact source function inverse product"); return r;
    }
    NF operator/(const NF& z) const { return *this*z.inverse(); }
    NF diff() const {
        Poly dh=derivative(source_h),twice_h=source_h.scale(F(2));
        return NF((derivative(a)*c-a*derivative(c))*twice_h,
                  (derivative(b)*c-b*derivative(c))*twice_h+b*c*dh,
                  c*c*twice_h);
    }
};
std::array<NF,4> cubic;
struct QA {
    std::array<NF,3> v;
    QA(long n=0) { v[0]=NF(n); }
    explicit QA(const NF& z) { v[0]=z; }
    explicit QA(std::array<NF,3> z):v(std::move(z)) {}
    bool operator==(const QA& z) const { return v==z.v; }
    bool operator!=(const QA& z) const { return !(*this==z); }
    QA operator-() const { return QA(std::array<NF,3>{-v[0],-v[1],-v[2]}); }
    QA operator+(const QA& z) const { return QA(std::array<NF,3>{v[0]+z.v[0],v[1]+z.v[1],v[2]+z.v[2]}); }
    QA operator-(const QA& z) const { return *this+(-z); }
    QA operator*(const QA& z) const {
        std::array<NF,5> q;
        for (int i=0;i<3;++i) for (int j=0;j<3;++j) q[size_t(i+j)]=q[size_t(i+j)]+v[size_t(i)]*z.v[size_t(j)];
        for (int i=4;i>=3;--i) for (int j=0;j<3;++j) q[size_t(i-3+j)]=q[size_t(i-3+j)]-q[size_t(i)]*cubic[size_t(j)];
        return QA(std::array<NF,3>{q[0],q[1],q[2]});
    }
    std::array<std::array<NF,3>,3> matrix() const {
        std::array<std::array<NF,3>,3> m;
        for (int j=0;j<3;++j) {
            std::array<NF,3> unit; unit[size_t(j)]=NF(1); QA col=*this*QA(unit);
            for (int i=0;i<3;++i) m[size_t(i)][size_t(j)]=col.v[size_t(i)];
        }
        return m;
    }
    QA inverse() const {
        auto m=matrix();
        NF n0=m[1][1]*m[2][2]-m[1][2]*m[2][1];
        NF n1=m[1][2]*m[2][0]-m[1][0]*m[2][2];
        NF n2=m[1][0]*m[2][1]-m[1][1]*m[2][0];
        NF determinant=m[0][0]*n0+m[0][1]*n1+m[0][2]*n2;
        NF inv=determinant.inverse(); QA r(std::array<NF,3>{n0*inv,n1*inv,n2*inv});
        require(*this*r==QA(1),"exact cubic algebra inverse product"); return r;
    }
    QA operator/(const QA& z) const { return *this*z.inverse(); }
    NF trace() const {
        auto m=matrix(); NF direct=m[0][0]+m[1][1]+m[2][2];
        NF newton=NF(3)*v[0]-cubic[2]*v[1]+(cubic[2]*cubic[2]-NF(2)*cubic[1])*v[2];
        require(direct==newton,"multiplication and Newton traces agree"); return direct;
    }
};
QA qpow(QA z,int n) { QA r(1); for (int i=0;i<n;++i) r=r*z; return r; }
QA eval(const BP& p,const QA& x,const QA& y) {
    QA r; for (const auto& [e,c]:p.t) r=r+QA(NF(Poly{c}))*qpow(x,e[0])*qpow(y,e[1]); return r;
}
NF read_nf(const Json& row) { return NF(load_poly(row.at("a")),load_poly(row.at("b")),load_poly(row.at("c"))); }
void emit_nf(const NF& z) {
    std::cout<<"{\"a\":";emit(z.a);std::cout<<",\"b\":";emit(z.b);std::cout<<",\"c\":";emit(z.c);std::cout<<'}';
}
long bound_difference(long M) {
    require(M>0 && M<=4096,"fixed degree-envelope cap");
    long root=0; while (root*root<72*M) ++root;
    return (root+6)*(root+6);
}
void below_characteristic(const BIGNUM* bound) { require(BN_cmp(bound,prime())<0,"degree envelope strictly below characteristic"); }
int main(int argc,char** argv) { try {
    using Clock=std::chrono::steady_clock; auto last=Clock::now(); std::vector<std::pair<std::string,long long>> phases;
    auto mark=[&](const std::string& s) { auto now=Clock::now(); auto us=std::chrono::duration_cast<std::chrono::microseconds>(now-last).count();phases.emplace_back(s,us);last=now;std::cerr<<s<<" passed wall_us="<<us<<'\n'; };
    require(argc==4,"usage: composition-check quartic.json backmap.json global.json");
    Json model=read_json(argv[1]),back=read_json(argv[2]),global=read_json(argv[3]);
    BP f=read_bp(model.at("terms"),true),L=read_bp(back.at("L"));
    std::array<BP,4> maps{read_bp(back.at("X_numerator")),read_bp(back.at("X_denominator")),read_bp(back.at("Y_numerator")),read_bp(back.at("Y_denominator"))};
    require(global_identity(f,maps),"actual elliptic map equation");
    const BP& A=maps[0];const BP& B=maps[1];const BP& C=maps[2];const BP& D=maps[3];
    auto fx=f.partial(0),fy=f.partial(1);
    BP numerator=(fy*A.partial(0)-fx*A.partial(1))*B-A*(fy*B.partial(0)-fx*B.partial(1));
    auto differential_ok=[&](const BP& candidate) { return (D*numerator-(candidate*C*B*B).scale(F(2))).reduced(f).t.empty(); };
    require(differential_ok(L),"actual elliptic map differential identity");
    require(hilbert(f,A,B,14)==12 && hilbert(f,A,B,15)==12,"degree-six Hilbert certificate");
    mark("elliptic_map_equation_differential_degree");
    source_h=cover(); F c(1024); Poly N{F(1),F(2)*c,F(8)*c*c},den{F(1),F(2)*c};
    F c4=c*c*c*c; Poly l{F(0),F(16)/(c*c),F(16)/c};
    require(pgcd(N,den).degree()==0 && N.degree()==2,"degree-two cover x-coordinate");
    require((power(N,3)-(N*den*den).scale(F(3))+power(den,3).scale(F(b_text)))*den==source_h.scale(c4*c4),"actual degree-two cover equation");
    require(derivative(N)*den-N*derivative(den)==l.scale(c4),"actual cover differential normalization");
    NF S(Poly{F(0),F(1)}),Y({},Poly{F(1)});
    require(Y*Y==NF(source_h) && S*S.inverse()==NF(1) && Y*Y.inverse()==NF(1),"source field algebra controls");
    require((Y*Y).diff()==NF(derivative(source_h)) && (S*Y).diff()==Y+S*Y.diff(),"source differentiation controls");
    mark("source_cover_and_field_controls");
    const auto& uj=global.at("U").arr; const auto& vj=global.at("V").arr;
    require(uj.size()==4 && vj.size()==3,"actual cubic graph dimensions");
    Poly common{F(1)};
    for (size_t i=0;i<4;++i) { cubic[i]=read_nf(uj[i]);Poly q=load_poly(uj[i].at("c"));common=(common*q).exact(pgcd(common,q)).monic(); }
    require(cubic[3]==NF(1),"actual graph monic cubic");
    int max_a=-1,max_b=-1;std::array<Poly,4> aa,bb;
    for (size_t i=0;i<4;++i) {
        Poly scale=common.exact(load_poly(uj[i].at("c")));aa[i]=load_poly(uj[i].at("a"))*scale;bb[i]=load_poly(uj[i].at("b"))*scale;
        require(NF(aa[i],bb[i],common)==cubic[i],"complete denominator clearing reconstruction");
        max_a=std::max(max_a,aa[i].degree());max_b=std::max(max_b,bb[i].degree());
    }
    require(aa[3]==common && bb[3].c.empty(),"nonzero norm leading T-six coefficient");
    long M=std::max(2*max_a,max_b<0?-1:source_h.degree()+2*max_b),difference=bound_difference(M);
    BN envelope=number(std::to_string(difference));below_characteristic(envelope.get());
    mark("actual_correspondence_degree_envelope");
    QA T(std::array<NF,3>{NF(0),NF(1),NF(0)}),V(std::array<NF,3>{read_nf(vj[0]),read_nf(vj[1]),read_nf(vj[2])});
    require(eval(f,T,V)==QA(0),"actual generic graph quartic incidence");
    mark("generic_quartic_incidence");
    QA us(std::array<NF,3>{cubic[0].diff(),cubic[1].diff(),cubic[2].diff()});
    QA ut(std::array<NF,3>{cubic[1],NF(2)*cubic[2],NF(3)});
    QA dt=-us/ut; mark("generic_separability_and_dx");
    QA vs(std::array<NF,3>{V.v[0].diff(),V.v[1].diff(),V.v[2].diff()});
    QA vt(std::array<NF,3>{V.v[1],NF(2)*V.v[2],NF(0)}),dv=vs+vt*dt;
    require(eval(fx,T,V)*dt+eval(fy,T,V)*dv==QA(0),"generic quartic differentiated incidence");
    mark("generic_differentiated_incidence");
    QA form=eval(L,T,V)*dt/eval(fy,T,V);
    NF actual=form.trace(),pi_form=NF(l)/(NF(2)*Y);
    require(actual==NF(3)*pi_form,"actual weighted differential trace equals three pullbacks");
    mark("actual_weighted_differential_trace");
    int controls=0;
    require(!differential_ok(L+constant(F(1))),"changed elliptic differential rejects");++controls;
    require(eval(f,T,V+QA(1))!=QA(0),"changed generic graph ordinate rejects");++controls;
    require(actual!=NF(2)*pi_form,"wrong differential multiplier rejects");++controls;
    require(cubic[0]!=NF(0) && NF(Poly{},Poly{},common)!=cubic[0],"omitted cubic coefficient reconstruction rejects");++controls;
    bool rejected=false;try { below_characteristic(prime()); } catch (const std::exception&) { rejected=true; }
    require(rejected,"characteristic-sized degree envelope rejects");++controls;mark("mutation_controls");
    std::cout<<"{\"scope\":\"P192-actual-f-pushforward-phi-pi-pullback-composition\",\"signed_independent_verdict\":false,\"generic_quartic_incidence\":true,\"generic_separability\":true,\"elliptic_map_degree\":6,\"source_cover_degree\":2,\"weighted_trace_multiplier\":3,\"trace_algorithms_agree\":true,\"common_denominator_degree\":"<<common.degree()<<",\"norm_S_degree_bound\":"<<M<<",\"projection_degree_bound\":"<<2*M<<",\"composition_degree_bound\":"<<72*M<<",\"difference_from_six_degree_bound\":"<<difference<<",\"difference_bound_below_characteristic\":true,\"mutation_controls_rejected\":"<<controls<<",\"trace\":";emit_nf(actual);
    std::cout<<",\"pi_pullback_differential\":";emit_nf(pi_form);
    std::cout<<",\"general_homomorphism_identity\":\"f_* phi pi^*=[6], under the stated smooth-model and standard Jacobian trace/polarization hypotheses\",\"older_software_transpose_globally_identified\":false,\"phase_us\":{";
    for(size_t i=0;i<phases.size();++i) { if(i)std::cout<<',';std::cout<<'"'<<phases[i].first<<"\":"<<phases[i].second; }
    std::cout<<"}}\n";
} catch (const std::exception& e) { std::cerr<<"REJECT "<<e.what()<<'\n';return 1; } }
