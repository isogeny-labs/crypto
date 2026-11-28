#include "elliptic_backmap_arithmetic.hpp"
using GCPoly=Poly<Z>;
GCPoly gc_gcd(GCPoly a,GCPoly b){while(b.degree()>=0){auto r=a%b;a=b;b=r;}return a.monic();}
GCPoly gc_derivative(const GCPoly& a){std::vector<Z> d;for(int i=1;i<=a.degree();++i)d.push_back(a.at(i)*Z(i));return GCPoly(d);}
struct GCField {
    inline static GCPoly h;
    GCPoly a,b,c{Z(1)};
    GCField(long n=0):a{Z(n)}{}
    GCField(GCPoly x,GCPoly y,GCPoly z):a(std::move(x)),b(std::move(y)),c(std::move(z)){normalize();}
    void normalize(){require(c.degree()>=0,"global source-field denominator");if(a.degree()<0&&b.degree()<0){c=GCPoly{Z(1)};return;}auto g=gc_gcd(gc_gcd(a,b),c);if(g.degree()>0){a=a.exact(g);b=b.exact(g);c=c.exact(g);}auto z=c.c.back().inv();a=a.scale(z);b=b.scale(z);c=c.scale(z);}
    GCField operator-()const{return GCField(-a,-b,c);}
    GCField operator+(const GCField& o)const{if(c==o.c)return GCField(a+o.a,b+o.b,c);return GCField(a*o.c+o.a*c,b*o.c+o.b*c,c*o.c);}
    GCField operator-(const GCField& o)const{return *this+(-o);}
    GCField operator*(const GCField& o)const{return GCField(a*o.a+b*o.b*h,a*o.b+b*o.a,c*o.c);}
    GCField inv()const{GCField r(a*c,(-b)*c,a*a-b*b*h);require(*this*r==GCField(1),"source-field inverse identity");return r;}
    GCField operator/(const GCField& o)const{return *this*o.inv();}
    bool operator==(const GCField& o)const{return a*o.c==o.a*c&&b*o.c==o.b*c;}
    bool operator!=(const GCField& o)const{return !(*this==o);}
    GCField derivative()const{auto da=gc_derivative(a),db=gc_derivative(b),dc=gc_derivative(c),dh=gc_derivative(h);return GCField((da*c-a*dc)*h.scale(Z(2)),(db*c-b*dc)*h.scale(Z(2))+b*c*dh,c*c*h.scale(Z(2)));}
    Z eval(const Z& x,const Z& y)const{return (a.eval(x)+b.eval(x)*y)/c.eval(x);}
};
using GCP=Poly<GCField>;
struct GCResidue {
    inline static GCP u;
    GCP p;
    GCResidue(long n=0):p{GCField(n)}{}
    explicit GCResidue(const GCField& f):p{f}{}
    explicit GCResidue(GCP q):p(q%u){}
    GCResidue operator+(const GCResidue& o)const{return GCResidue(p+o.p);}
    GCResidue operator-()const{return GCResidue(-p);}
    GCResidue operator-(const GCResidue& o)const{return GCResidue(p-o.p);}
    GCResidue operator*(const GCResidue& o)const{return GCResidue((p*o.p)%u);}
    GCResidue inv()const{
        std::array<std::array<GCField,3>,3> m{};GCP t{GCField(0),GCField(1)},q=p;
        for(int j=0;j<3;++j){for(int i=0;i<3;++i)m[i][j]=q.at(i);q=(q*t)%u;}
        std::array<GCField,3> adj{m[1][1]*m[2][2]-m[1][2]*m[2][1],m[1][2]*m[2][0]-m[1][0]*m[2][2],m[1][0]*m[2][1]-m[1][1]*m[2][0]};
        GCField det=m[0][0]*adj[0]+m[0][1]*adj[1]+m[0][2]*adj[2];require(det!=GCField(0),"generic cubic-algebra nonunit");auto den=det.inv();for(auto& z:adj)z=z*den;
        GCResidue r(GCP(std::vector<GCField>(adj.begin(),adj.end())));require(*this*r==GCResidue(1),"global cubic inverse identity");return r;
    }
    GCResidue operator/(const GCResidue& o)const{return *this*o.inv();}
    bool operator==(const GCResidue& o)const{return p==o.p;}
    bool operator!=(const GCResidue& o)const{return !(*this==o);}
};
using GCRP=Poly<GCResidue>;
GCResidue gc_curve(const EBPoly& f,const GCResidue& x,const GCResidue& y){GCResidue r;for(auto& [e,c]:f.terms){GCResidue t(GCField(GCPoly{c},{},GCPoly{Z(1)}));for(int i=0;i<e[0];++i)t=t*x;for(int i=0;i<e[1];++i)t=t*y;r=r+t;}return r;}
GCField gc_trace(const GCResidue& q){auto& u=GCResidue::u;return GCField(3)*q.p.at(0)-u.at(2)*q.p.at(1)+(u.at(2)*u.at(2)-GCField(2)*u.at(1))*q.p.at(2);}
void gc_emit_poly(std::ostream& out,const GCPoly& p){out<<'[';for(int i=0;i<=p.degree();++i)out<<(i?",":"")<<'"'<<p.at(i).x<<'"';out<<']';}
void gc_emit(std::ostream& out,const GCField& z){out<<"{\"a\":";gc_emit_poly(out,z.a);out<<",\"b\":";gc_emit_poly(out,z.b);out<<",\"c\":";gc_emit_poly(out,z.c);out<<'}';}
void gc_emit(std::ostream& out,const GCP& p){out<<'[';for(int i=0;i<=p.degree();++i){if(i)out<<',';gc_emit(out,p.at(i));}out<<']';}
GCField gc_fit(const Json& j){auto& v=j.at("candidate_coefficients_A37_B33_C37").arr;require(v.size()==107&&j.at("success").scalar=="true","fixed successful formal fit");std::vector<Z> a,b,c;for(int i=0;i<37;++i)a.emplace_back(mpz_class(v[i].scalar));for(int i=0;i<33;++i)b.emplace_back(mpz_class(v[37+i].scalar));for(int i=0;i<37;++i)c.emplace_back(mpz_class(v[70+i].scalar));return GCField(GCPoly(a),GCPoly(b),GCPoly(c));}
int main(int argc,char** argv){try{
    require(argc==3,"usage: global_correspondence kernel3-study formal-result");std::string root=argv[1];initialize_cover(read_json(root+"/inputs/p192-d7-prym.json"));Level2Input input;auto fit=read_json(argv[2]),model=read_json(root+"/evidence/p192-d7-kernel3-plane-quartic.json"),points=read_json(root+"/evidence/destination-relations-v2/result.json");std::vector<Z> h;for(auto& c:curve.c)h.push_back(scalar(c));GCField::h=GCPoly(h);GCField sx(GCPoly{Z(0),Z(1)},{},GCPoly{Z(1)}),sy({},GCPoly{Z(1)},GCPoly{Z(1)});require(sy*sy==GCField(GCField::h,{},GCPoly{Z(1)}),"source quadratic relation control");require(sx*sx.inv()==GCField(1)&&sy*sy.inv()==GCField(1),"source inverses control");require((sx*sy).derivative()==sy+sx*sy.derivative(),"source product derivative control");require((sy*sy).derivative()==GCField(gc_derivative(GCField::h),{},GCPoly{Z(1)}),"source quadratic derivative control");
    EBPoly f;for(auto& t:model.at("terms").arr){auto& e=t.at("exponents");f.add({e.arr[0].integer(),e.arr[1].integer()},Z(mpz_class(t.at("coefficient").scalar)));}auto fx=f.partial(0),fy=f.partial(1);std::array<GCField,6> symmetric;for(int i=0;i<6;++i)symmetric[i]=gc_fit(fit.at("fits").arr[i]);
    for(int coordinate=0;coordinate<2;++coordinate){std::array<Z,3> r{};for(int i=0;i<3;++i){int index=fit.at("seed_indices").arr[i].integer();r[i]=Z(mpz_class(points.at("points").arr[index].at(coordinate?"y":"x").scalar));}std::array<Z,3> expected{r[0]+r[1]+r[2],r[0]*r[1]+r[0]*r[2]+r[1]*r[2],r[0]*r[1]*r[2]};for(int i=0;i<3;++i){auto& q=symmetric[coordinate*3+i];require(q.a.degree()<=q.c.degree()&&(q.b.degree()<0||2*q.b.degree()+7<2*q.c.degree()),"regular reference limit at infinity");Z value=q.a.degree()==q.c.degree()?q.a.c.back()/q.c.c.back():Z(0);require(value==expected[i],"global rational reference limit");}}
    GCResidue::u=GCP{GCField(-2),GCField(0),GCField(0),GCField(1)};
    for(auto q:{GCP{GCField(0),GCField(1)},GCP{GCField(1),GCField(1),GCField(1)}}){GCResidue a(q);require(a*a.inv()==GCResidue(1),"adjugate cubic control");}
    bool rejected=false;try{GCResidue(0).inv();}catch(const std::exception&){rejected=true;}require(rejected,"adjugate zero nonunit control");
    GCResidue::u=GCP{-symmetric[2],symmetric[1],-symmetric[0],GCField(1)};GCResidue T(GCP{GCField(0),GCField(1)});GCRP Q{GCResidue(-symmetric[5]),GCResidue(symmetric[4]),GCResidue(-symmetric[3]),GCResidue(1)},F;
    std::vector<GCResidue> coeff(5);for(auto& [e,c]:f.terms){GCResidue t(GCField(GCPoly{c},{},GCPoly{Z(1)}));for(int i=0;i<e[0];++i)t=t*T;coeff[e[1]]=coeff[e[1]]+t;}F=GCRP(coeff);auto a=F,b=Q;int divisions=0;while(b.degree()>1){std::cerr<<"global adjugate y-gcd step "<<divisions<<" degrees="<<a.degree()<<','<<b.degree()<<'\n';auto r=a%b;a=b;b=r;require(++divisions<=6,"global y-gcd bound");}require(b.degree()==1,"unique global x/y pairing");auto gcd=b.monic();GCResidue V=-gcd.at(0);std::cerr<<"linear global pairing recovered\n";require(F.eval(V)==GCResidue(0),"original gcd quartic incidence");
    require(gc_curve(f,T,V)==GCResidue(0),"global quartic incidence");require(Q.eval(V)==GCResidue(0),"global y cubic incidence");auto tr=gc_trace(V),tr2=gc_trace(V*V),tr3=gc_trace(V*V*V);require(tr==symmetric[3]&&(tr*tr-tr2)/GCField(2)==symmetric[4]&&(tr*tr*tr-GCField(3)*tr*tr2+GCField(2)*tr3)/GCField(6)==symmetric[5],"global y characteristic polynomial");std::cerr<<"global incidences and y characteristic polynomial pass\n";
    bool changed=gc_curve(f,T,V+GCResidue(1))!=GCResidue(0)||gc_trace(V+GCResidue(1))!=symmetric[3];require(changed,"changed global paired-y rejection");
    std::vector<GCField> ux,ut,vx,vt;for(int i=0;i<=3;++i){ux.push_back(GCResidue::u.at(i).derivative());if(i)ut.push_back(GCResidue::u.at(i)*GCField(i));}for(int i=0;i<3;++i){vx.push_back(V.p.at(i).derivative());if(i)vt.push_back(V.p.at(i)*GCField(i));}GCResidue dx=-GCResidue(GCP(ux))/GCResidue(GCP(ut)),dy=GCResidue(GCP(vx))+GCResidue(GCP(vt))*dx;require(gc_curve(fx,T,V)*dx+gc_curve(fy,T,V)*dy==GCResidue(0),"global quartic derivative");GCResidue base=dx/gc_curve(fy,T,V);std::array<GCResidue,3> canonical{T,V,GCResidue(1)};std::array<GCField,3> traces;
    for(int i=0;i<3;++i){traces[i]=gc_trace(canonical[i]*base);GCField expected;for(int j=0;j<3;++j){std::vector<Z> coefficients(size_t(j+1),Z(0));coefficients[j]=Z(mpz_class(fit.at("differential").arr[i].arr[j].scalar));GCPoly monomial(coefficients);expected=expected+GCField(monomial,{},GCPoly{Z(1)})/(GCField(2)*sy);}require(traces[i]==expected,"global candidate differential trace");std::cerr<<"global differential row "<<i<<" passes\n";}
    std::cout<<"{\"scope\":\"P192-kernel3-global-curve-correspondence\",\"standing\":\"producer-unverified\",\"y_gcd_steps\":"<<divisions<<",\"linear_pairing\":true,\"quartic_incidence\":true,\"y_characteristic_polynomial\":true,\"quartic_derivative\":true,\"differential_trace_rows\":3,\"changed_pairing_rejected\":true,\"actual_kernel_checked\":false,\"polarization_degree_established\":false,\"dual_composition_checked\":false,\"U\":";gc_emit(std::cout,GCResidue::u);std::cout<<",\"V\":";gc_emit(std::cout,V.p);std::cout<<",\"differential_traces\":[";for(int i=0;i<3;++i){if(i)std::cout<<',';gc_emit(std::cout,traces[i]);}std::cout<<"]}\n";
}catch(const std::exception& e){std::cerr<<"REJECT "<<e.what()<<'\n';return 1;}}
