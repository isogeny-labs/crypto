#include "quartic_descent_arithmetic.hpp"
#include <fstream>
constexpr int EB_PRECISION=112;
using EBExponent=std::array<int,2>;
Z eb_z(const T24& c){require(c.a.degree()<=0&&c.a.at(0).a.degree()<=0,"backmap original-field coefficient");return c.a.at(0).a.at(0);}
T24 eb_t(const Z& c){return T24(F(Poly<Z>{c}));}
struct EBSeries {
    std::array<Z,EB_PRECISION> c{};
    EBSeries()=default;
    explicit EBSeries(const Z& a){c[0]=a;}
    EBSeries scale(const Z& a)const{EBSeries r;for(int i=0;i<EB_PRECISION;++i)r.c[i]=c[i]*a;return r;}
    EBSeries operator+(const EBSeries& b)const{EBSeries r;for(int i=0;i<EB_PRECISION;++i)r.c[i]=c[i]+b.c[i];return r;}
    EBSeries operator-()const{return scale(Z(-1));}
    EBSeries operator-(const EBSeries& b)const{return *this+(-b);}
    EBSeries operator*(const EBSeries& b)const{EBSeries r;for(int i=0;i<EB_PRECISION;++i)if(c[i]!=Z(0))for(int j=0;j+i<EB_PRECISION;++j)if(b.c[j]!=Z(0))r.c[i+j]=r.c[i+j]+c[i]*b.c[j];return r;}
    EBSeries inv()const{require(c[0]!=Z(0),"backmap series unit");EBSeries r;r.c[0]=c[0].inv();for(int n=1;n<EB_PRECISION;++n){Z sum;for(int i=1;i<=n;++i)sum=sum+c[i]*r.c[n-i];r.c[n]=-sum*r.c[0];}return r;}
    EBSeries operator/(const EBSeries& b)const{return *this*b.inv();}
    EBSeries shift(int n)const{EBSeries r;for(int i=0;i<EB_PRECISION;++i)if(i+n>=0&&i+n<EB_PRECISION)r.c[i+n]=c[i];return r;}
    EBSeries derivative()const{EBSeries r;for(int i=1;i<EB_PRECISION;++i)r.c[i-1]=c[i]*Z(i);return r;}
    EBSeries integral()const{EBSeries r;for(int i=1;i<EB_PRECISION;++i)r.c[i]=c[i-1]/Z(i);return r;}
    int order()const{int n=0;while(n<EB_PRECISION&&c[n]==Z(0))++n;return n;}
};
struct EBPoly {
    std::map<EBExponent,Z> terms;
    EBPoly()=default;
    explicit EBPoly(const Z& a){if(a!=Z(0))terms.emplace(EBExponent{},a);}
    void add(EBExponent e,const Z& a){if(a==Z(0))return;auto p=terms.find(e);if(p==terms.end())terms.emplace(e,a);else{p->second=p->second+a;if(p->second==Z(0))terms.erase(p);}}
    EBPoly scale(const Z& a)const{EBPoly r;for(auto& [e,c]:terms)r.add(e,c*a);return r;}
    EBPoly operator+(const EBPoly& b)const{EBPoly r=*this;for(auto& [e,c]:b.terms)r.add(e,c);return r;}
    EBPoly operator-()const{return scale(Z(-1));}
    EBPoly operator-(const EBPoly& b)const{return *this+(-b);}
    EBPoly operator*(const EBPoly& b)const{EBPoly r;for(auto& [e,c]:terms)for(auto& [f,d]:b.terms)r.add({e[0]+f[0],e[1]+f[1]},c*d);return r;}
    EBPoly partial(int i)const{EBPoly r;for(auto& [e,c]:terms)if(e[i]){auto f=e;--f[i];r.add(f,c*Z(e[i]));}return r;}
    Z eval(const Z& x,const Z& y)const{Z r;for(auto& [e,c]:terms){Z a=c;for(int i=0;i<e[0];++i)a=a*x;for(int j=0;j<e[1];++j)a=a*y;r=r+a;}return r;}
    EBSeries eval(const EBSeries& x,const EBSeries& y)const{std::array<EBSeries,33> xp,yp;xp[0]=EBSeries(Z(1));yp[0]=EBSeries(Z(1));for(int i=1;i<33;++i){xp[i]=xp[i-1]*x;yp[i]=yp[i-1]*y;}EBSeries r;for(auto& [e,c]:terms){require(e[0]<33&&e[1]<33,"backmap polynomial series exponent");r=r+(xp[e[0]]*yp[e[1]]).scale(c);}return r;}
    EBPoly reduced(const EBPoly& f)const{auto leading=f.terms.find({0,4});require(leading!=f.terms.end()&&leading->second!=Z(0),"quartic monic-y reduction chart");EBPoly r=*this;int steps=0;for(;;){auto p=std::find_if(r.terms.begin(),r.terms.end(),[](const auto& t){return t.first[1]>=4;});if(p==r.terms.end())break;require(++steps<10000,"quartic polynomial reduction limit");auto e=p->first;Z a=p->second/leading->second;for(auto& [g,c]:f.terms)r.add({e[0]+g[0],e[1]-4+g[1]},-a*c);}return r;}
};
using EBRow=std::vector<Z>;
struct EBEchelon {
    int width;
    std::map<int,EBRow> rows;
    explicit EBEchelon(int n):width(n){}
    void insert(EBRow row){require(int(row.size())==width,"backmap linear row width");for(auto& [p,r]:rows){Z a=row[p];if(a!=Z(0))for(int j=p;j<width;++j)row[j]=row[j]-a*r[j];}int pivot=0;while(pivot<width&&row[pivot]==Z(0))++pivot;if(pivot==width)return;Z inv=row[pivot].inv();for(int j=pivot;j<width;++j)row[j]=row[j]*inv;rows.emplace(pivot,row);}
    std::vector<EBRow> kernel()const{std::vector<EBRow> out;for(int j=0;j<width;++j)if(!rows.count(j)){EBRow v(size_t(width),Z(0));v[j]=Z(1);for(auto it=rows.rbegin();it!=rows.rend();++it){Z sum;for(int k=it->first+1;k<width;++k)sum=sum+it->second[k]*v[k];v[it->first]=-sum;}out.push_back(v);}return out;}
};
void eb_emit_poly(std::ostream& out,const EBPoly& p){out<<'[';bool first=true;for(auto& [e,c]:p.terms){if(!first)out<<',';first=false;out<<"{\"exponents\":["<<e[0]<<','<<e[1]<<"],\"coefficient\":\""<<c.x<<"\"}";}out<<']';}
void eb_emit_series(const EBSeries& s,int count){std::cout<<'[';for(int i=0;i<count;++i)std::cout<<(i?",":"")<<'"'<<s.c[i].x<<'"';std::cout<<']';}
int eb_hilbert(const EBPoly& curve,const EBPoly& a,const EBPoly& b,int n,int& rank){std::map<EBExponent,int> index;int width=0;for(int i=0;i<=n;++i)for(int j=0;j+i<=n;++j)index.emplace(EBExponent{i,j},width++);EBEchelon echelon(width);
    std::array<const EBPoly*,3> generators{&curve,&a,&b};for(int k=0;k<3;++k){int degree=k?6:4;for(int i=0;i<=n-degree;++i)for(int j=0;i+j<=n-degree;++j){EBRow row(size_t(width),Z(0));for(auto& [e,c]:generators[k]->terms)row.at(size_t(index.at({i+e[0],j+e[1]})))=c;echelon.insert(row);}}
    rank=int(echelon.rows.size());return width-rank;
}
int main(int argc,char** argv){try{
    require(argc==3,"usage: elliptic_backmap kernel3-study candidate.json");std::string root=argv[1];initialize_cover(read_json(root+"/inputs/p192-d7-prym.json"));Level2Input input;auto model=read_json(root+"/evidence/p192-d7-kernel3-plane-quartic.json"),twist=read_json(root+"/evidence/differential-twist-v2/result.json"),points=read_json(root+"/evidence/destination-relations-v2/result.json");require(model.at("kernel").integer()==3&&twist.at("scalar_quadratic_character").integer()==1,"fixed square-character kernel3 input");EBPoly curve;for(auto& term:model.at("terms").arr){auto& e=term.at("exponents");curve.add({e.arr[0].integer(),e.arr[1].integer()},Z(mpz_class(term.at("coefficient").scalar)));}auto fx=curve.partial(0),fy=curve.partial(1);
    Z scalar=eb_z(read_t24(twist.at("common_scalar")));mpz_class square;mpz_powm(square.get_mpz_t(),scalar.x.get_mpz_t(),mpz_class((prime+1)/4).get_mpz_t(),prime.get_mpz_t());if(square>prime-square)square=prime-square;Z root_s(square);require(root_s*root_s==scalar,"base-field differential square root");Matrix3<T24> differential{},inverse{};for(int i=0;i<3;++i){auto row=read_line(twist.at("projective_differential").arr[i]);for(int j=0;j<3;++j)differential[i][j]=row[j]*eb_t(root_s);}for(int j=0;j<3;++j){Line rhs{};rhs[j]=T24(1);auto column=solve3(differential,rhs);for(int i=0;i<3;++i)inverse[i][j]=column[i];}
    Z c(1024);Poly<Z> source_l{Z(0),Z(16)/(c*c),Z(16)/c},N{Z(1),Z(2)*c,Z(8)*c*c},D{Z(1),Z(2)*c};auto derivative=[](const Poly<Z>& p){std::vector<Z> d;for(int i=1;i<=p.degree();++i)d.push_back(p.at(i)*Z(i));return Poly<Z>(d);};require(derivative(N)*D-N*derivative(D)==source_l.scale(c*c*c*c),"native cover differential normalization identity");
    std::array<Z,3> l{};for(int j=0;j<3;++j)for(int i=0;i<3;++i)l[j]=l[j]+Z(3)*source_l.at(i)*eb_z(inverse[i][j]);EBPoly L;L.add({1,0},l[0]);L.add({0,1},l[1]);L.add({0,0},l[2]);std::cerr<<"candidate dual differential L="<<l[0].x<<','<<l[1].x<<','<<l[2].x<<'\n';
    int selected=-1;Z xr,yr;for(size_t i=0;i<points.at("points").arr.size();++i){auto& p=points.at("points").arr[i];Z x(mpz_class(p.at("x").scalar)),y(mpz_class(p.at("y").scalar));require(curve.eval(x,y)==Z(0),"retained base-quartic point");if(fy.eval(x,y)!=Z(0)&&L.eval(x,y)!=Z(0)){selected=int(i);xr=x;yr=y;break;}}require(selected>=0,"first unramified base point chart");EBSeries x(xr),y(yr);x.c[1]=Z(1);for(int precision=1;precision<EB_PRECISION;precision*=2)y=y-curve.eval(x,y)/fy.eval(x,y);auto local=curve.eval(x,y);for(auto& v:local.c)require(v==Z(0),"full quartic Newton series");EBSeries omega=L.eval(x,y)/fy.eval(x,y);require(omega.c[0]!=Z(0),"nonzero elliptic backmap differential at base point");auto u=omega.integral(),unit=u.shift(-1);EBSeries W=unit.inv()*unit.inv(),upower(Z(1));Z a(-3),b(mpz_class("2455155546008943817740293915197451784769108058161191238065"));std::array<Z,EB_PRECISION> wp{};wp[2]=-a/Z(5);wp[4]=-b/Z(7);
    for(int n=5;n<EB_PRECISION;++n){Z sum;for(int i=2;i<=n-4;++i)sum=sum+wp[i]*wp[n-2-i];wp[n]=Z(6)*sum/(Z(n-4)*Z(n+3));}
    for(int n=1;n<EB_PRECISION-2;++n){upower=upower*u;if(wp[n]!=Z(0))W=W+upower.shift(2).scale(wp[n]);}auto scaledY=(W.derivative().shift(1)-W.scale(Z(2)))/(omega.scale(Z(2)));auto formal_error=scaledY*scaledY-W*W*W-W.shift(4).scale(a)-EBSeries(b).shift(6);for(int n=0;n<96;++n)require(formal_error.c[n]==Z(0),"scaled formal elliptic equation");std::cerr<<"112-term curve lift and96 elliptic-series coefficients pass\n";
    std::vector<EBExponent> monomials;std::vector<EBSeries> basis;for(int i=0;i<=6;++i)for(int j=0;j<=3&&i+j<=6;++j){monomials.push_back({i,j});EBPoly p;p.add({i,j},Z(1));basis.push_back(p.eval(x,y));}require(basis.size()==22,"degree6 quartic section dimension22");std::vector<EBSeries> columns;for(auto& f:basis)columns.push_back(f.shift(2));for(auto& f:basis)columns.push_back(-(W*f));EBEchelon echelon(44);for(int n=0;n<64;++n){EBRow row;for(auto& p:columns)row.push_back(p.c[n]);echelon.insert(row);}auto kernel=echelon.kernel();std::cerr<<"rational reconstruction rank="<<echelon.rows.size()<<",nullity="<<kernel.size()<<'\n';
    std::ofstream candidate(argv[2]);require(bool(candidate),"write backmap candidate receipt");candidate<<"{\"scope\":\"P192-kernel3-elliptic-backmap-candidate\",\"point_receipt_index\":"<<selected<<",\"rank\":"<<echelon.rows.size()<<",\"nullity\":"<<kernel.size()<<",\"L\":";eb_emit_poly(candidate,L);candidate<<"}\n";candidate.close();require(bool(candidate),"complete candidate receipt");require(echelon.rows.size()==34&&kernel.size()==10,"degree6 backmap rank34/nullity10");
    std::vector<std::pair<EBPoly,EBPoly>> functions;int heldout=0;for(auto& vector:kernel){for(int n=64;n<96;++n){Z value;for(int j=0;j<44;++j)value=value+vector[j]*columns[j].c[n];require(value==Z(0),"unused rational-reconstruction coefficient");++heldout;}EBPoly A,B;for(int j=0;j<22;++j){A.add(monomials[j],vector[j]);B.add(monomials[j],vector[j+22]);}require(!B.terms.empty(),"nonzero recovered rational denominator");functions.emplace_back(A,B);}
    auto A=functions[0].first,B=functions[0].second;require(B.eval(x,y).order()-A.eval(x,y).order()==2,"nonconstant elliptic X pole at chosen point");for(auto& [aa,bb]:functions)require((aa*B-A*bb).reduced(curve).terms.empty(),"all reconstructed ratios agree globally");auto derivation=[&](const EBPoly& p){return (fy*p.partial(0)-fx*p.partial(1)).reduced(curve);};auto V=(B*derivation(A)-A*derivation(B)).reduced(curve);require(!V.terms.empty(),"nonconstant separable backmap X");
    auto identity=[&](const EBPoly& numerator){auto v=(B*derivation(numerator)-numerator*derivation(B)).reduced(curve);auto cubic=(numerator*numerator*numerator+(numerator*B*B).scale(a)+(B*B*B).scale(b)).reduced(curve);return (v*v-(L*L*B*cubic).scale(Z(4))).reduced(curve).terms.empty();};require(identity(A),"global original-field elliptic backmap equation");rejects([&]{require(identity(A+EBPoly(Z(1))),"changed elliptic backmap numerator");},"changed elliptic backmap numerator");std::cerr<<"global elliptic backmap equation passes\n";
    int rank14=0,rank15=0;int dimension14=eb_hilbert(curve,A,B,14,rank14),dimension15=eb_hilbert(curve,A,B,15,rank15);require(dimension14==12&&dimension15==12,"common base divisor degree12 and map degree6");
    std::cout<<"{\"scope\":\"P192-kernel3-original-field-elliptic-backmap\",\"standing\":\"producer-unverified\",\"point_receipt_index\":"<<selected<<",\"sqrt_scalar\":\""<<root_s.x<<"\",\"section_degree\":6,\"section_dimension\":22,\"training_rank\":34,\"kernel_dimension\":10,\"heldout_series_checks\":"<<heldout<<",\"global_ratio_checks\":10,\"global_elliptic_equation\":true,\"base_divisor_Hilbert_dimensions\":["<<dimension14<<','<<dimension15<<"],\"Hilbert_matrix_ranks\":["<<rank14<<','<<rank15<<"],\"curve_map_degree\":6,\"divisor_class_transfer_executed\":false,\"L\":";eb_emit_poly(std::cout,L);std::cout<<",\"X_numerator\":";eb_emit_poly(std::cout,A);std::cout<<",\"X_denominator\":";eb_emit_poly(std::cout,B);std::cout<<",\"Y_numerator\":";eb_emit_poly(std::cout,V);std::cout<<",\"Y_denominator\":";eb_emit_poly(std::cout,(L*B*B).scale(Z(2)).reduced(curve));std::cout<<",\"point\":[\""<<xr.x<<"\",\""<<yr.x<<"\"],\"local_y\":";eb_emit_series(y,96);std::cout<<",\"local_omega\":";eb_emit_series(omega,96);std::cout<<",\"local_scaled_X\":";eb_emit_series(W,96);std::cout<<"}\n";
}catch(const std::exception& e){std::cerr<<"REJECT "<<e.what()<<'\n';return 1;}}
