// Exact dual-number and moving-cubic helpers. No numerical differentiation.
template<class T> struct FirstOrder {
    T value,derivative;
    FirstOrder(long x=0):value(x),derivative(0){}
    explicit FirstOrder(const T& x):value(x),derivative(0){}
    FirstOrder(const T& x,const T& d):value(x),derivative(d){}
    bool operator==(const FirstOrder& b)const{return value==b.value&&derivative==b.derivative;}
    bool operator!=(const FirstOrder& b)const{return !(*this==b);}
    FirstOrder operator+(const FirstOrder& b)const{return {value+b.value,derivative+b.derivative};}
    FirstOrder operator-()const{return {-value,-derivative};}
    FirstOrder operator-(const FirstOrder& b)const{return *this+-b;}
    FirstOrder operator*(const FirstOrder& b)const{return {value*b.value,derivative*b.value+value*b.derivative};}
    FirstOrder inv()const{require(value!=T(0),"first-order inverse requires unit");T i=value.inv();return {i,-derivative*i*i};}
    FirstOrder operator/(const FirstOrder& b)const{return *this*b.inv();}
};
using FO=FirstOrder<T24>;
using FOP=Poly<FO>;
using FOG=JGeometry<FO>;
FOP first_constant(const TP& p){std::vector<FO> out;for(auto& c:p.c)out.emplace_back(c);return FOP(out);}
FOP first_polynomial(const TP& p,const TP& d){std::vector<FO> out;for(int i=0;i<=std::max(p.degree(),d.degree());++i)out.emplace_back(p.at(i),d.at(i));return FOP(out);}
FOG first_constant(const GT24& d){return {first_constant(d.u),first_constant(d.v)};}
FOG moving_cubic(const FiniteDivisor& e,const TP& dx,const TP& dy){
    require(e.u.degree()==3,"moving cubic degree3");FOP u0=first_constant(e.u),x=first_polynomial(TP{T24(0),T24(1)},dx),y=first_polynomial(e.v,dy);
    auto m=multiplication3(u0,x);FO trace=m[0][0]+m[1][1]+m[2][2],trace2;
    for(int i=0;i<3;++i)for(int j=0;j<3;++j)trace2=trace2+m[i][j]*m[j][i];
    FOP u{-determinant3(m),(trace*trace-trace2)/FO(2),-trace,FO(1)};
    Matrix3<FO> interpolation{};FOP power{FO(1)};
    for(int j=0;j<3;++j){for(int i=0;i<3;++i)interpolation[i][j]=power.at(i);power=(power*x)%u0;}
    FO denominator=determinant3(interpolation);require(denominator.value==T24(1),"first-order interpolation origin");std::vector<FO> coeff;
    for(int j=0;j<3;++j){auto a=interpolation;for(int i=0;i<3;++i)a[i][j]=y.at(i);coeff.push_back(determinant3(a)/denominator);}
    FOP v(coeff);for(int i=0;i<=3;++i)require(u.at(i).value==e.u.at(i),"first-order U origin");
    for(int i=0;i<3;++i)require(v.at(i).value==e.v.at(i),"first-order V origin");
    return {u,v};
}
template<class T> Poly<T> small_power(Poly<T> p,int n){Poly<T> r{T(1)};for(int j=0;j<n;++j)r=r*p;return r;}
FOP first_equation(const Form& f,const FOG& d){FOP x{FO(0),FO(1)},out;for(auto& [e,c]:f.c)out=(out+(small_power(x,e[0])*small_power(d.v,e[1])).scale(FO(c)))%d.u;return out;}
TP polynomial_partial(const TP& p){std::vector<T24> c;for(int i=1;i<=p.degree();++i)c.push_back(p.at(i)*T24(i));return TP(c);}
T24 cubic_trace(const TP& u,const TP& p){auto m=multiplication3(u,p);return m[0][0]+m[1][1]+m[2][2];}
Line abel_gradient(const Matrix3<T24>& velocity,const Line& directions){Matrix3<T24> trans{};for(int i=0;i<3;++i)for(int j=0;j<3;++j)trans[i][j]=velocity[j][i];return solve3(trans,directions);}
Matrix3<T24> transpose3(const Matrix3<T24>& m){Matrix3<T24> out{};for(int i=0;i<3;++i)for(int j=0;j<3;++j)out[i][j]=m[j][i];return out;}
Matrix3<T24> scaled3(Matrix3<T24> m,const T24& c){for(auto& row:m)for(auto& v:row)v=v*c;return m;}
Matrix3<T24> outer3(const Line& g,const T24& c){Matrix3<T24> m{};for(int i=0;i<3;++i)for(int j=0;j<3;++j)m[i][j]=c*g[i]*g[j];return m;}
