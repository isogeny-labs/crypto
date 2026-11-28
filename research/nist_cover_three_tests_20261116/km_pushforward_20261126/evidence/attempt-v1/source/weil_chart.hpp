// Formal-arc evaluation for the determinant chart in Tian section 5.
// Include after the build-only weil3_arithmetic.hpp.
constexpr int arc_precision=24;
template<class T> struct Jet {
    std::array<T,arc_precision+1> c{};
    Jet(long x=0){c[0]=T(x);}
    explicit Jet(const T& x){c[0]=x;}
    bool operator==(const Jet& b)const{return c==b.c;}
    bool operator!=(const Jet& b)const{return !(*this==b);}
    Jet operator+(const Jet& b)const{Jet r;for(int i=0;i<=arc_precision;++i)r.c[i]=c[i]+b.c[i];return r;}
    Jet operator-()const{Jet r;for(int i=0;i<=arc_precision;++i)r.c[i]=-c[i];return r;}
    Jet operator-(const Jet& b)const{return *this+-b;}
    Jet operator*(const Jet& b)const{
        Jet r;
        for(int i=0;i<=arc_precision;++i)if(c[i]!=T(0))
            for(int j=0;i+j<=arc_precision;++j)if(b.c[j]!=T(0))r.c[i+j]=r.c[i+j]+c[i]*b.c[j];
        return r;
    }
    Jet inv()const{
        require(c[0]!=T(0),"formal series inverse requires a unit");Jet r;r.c[0]=c[0].inv();
        for(int n=1;n<=arc_precision;++n){T sum;for(int i=1;i<=n;++i)sum=sum+c[i]*r.c[n-i];r.c[n]=-r.c[0]*sum;}
        return r;
    }
    Jet operator/(const Jet& b)const{return *this*b.inv();}
    int valuation()const{for(int i=0;i<=arc_precision;++i)if(c[i]!=T(0))return i;return arc_precision+1;}
};
template<class T> using Matrix3=std::array<std::array<T,3>,3>;
template<class T> T determinant3(const Matrix3<T>& m){
    return m[0][0]*(m[1][1]*m[2][2]-m[1][2]*m[2][1])
         - m[0][1]*(m[1][0]*m[2][2]-m[1][2]*m[2][0])
         + m[0][2]*(m[1][0]*m[2][1]-m[1][1]*m[2][0]);
}
template<class T> Matrix3<T> multiplication3(const Poly<T>& u,Poly<T> a){
    require(u.degree()==3&&u.at(3)==T(1),"formal monic cubic input");
    Matrix3<T> m{};Poly<T> x{T(0),T(1)};a=a%u;
    for(int j=0;j<3;++j){for(int i=0;i<3;++i)m[i][j]=a.at(i);a=(a*x)%u;}
    return m;
}
template<class T> Poly<Jet<T>> constant_jet(const Poly<T>& p){
    std::vector<Jet<T>> out;for(auto& c:p.c)out.emplace_back(c);return Poly<Jet<T>>(out);
}
template<class T> struct CurveArc {Poly<Jet<T>> u,v;};

template<class T> CurveArc<T> curve_arc(const Poly<T>& u,const Poly<T>& v,const Poly<T>& f){
    require(u.degree()==3,"formal curve arc requires degree3 support");
    std::vector<T> dc;for(int i=1;i<=u.degree();++i)dc.push_back(u.at(i)*T(i));
    auto [g,s,t]=xgcd(u,Poly<T>(dc));require(g.degree()==0,"formal curve arc requires squarefree support");
    using S=Jet<T>;using SP=Poly<S>;SP um=constant_jet(u),x{S(0),S(1)},y=constant_jet(v);
    std::vector<T> fp;for(int i=1;i<=f.degree();++i)fp.push_back(f.at(i)*T(i));
    for(int n=0;n<arc_precision;++n){
        SP rhs;
        for(int i=int(fp.size())-1;i>=0;--i)rhs=(rhs*x+SP{S(fp[i])})%um;
        for(int j=0;j<3;++j){
            if(j>=int(x.c.size()))x.c.resize(j+1);
            if(j>=int(y.c.size()))y.c.resize(j+1);
            x.c[j].c[n+1]=T(2)*y.at(j).c[n]/T(n+1);
            y.c[j].c[n+1]=rhs.at(j).c[n]/T(n+1);
        }
    }
    SP residual;
    for(int i=f.degree();i>=0;--i)residual=(residual*x+SP{S(f.at(i))})%um;
    require(((y*y-residual)%um).degree()<0,"formal Hamiltonian arc curve equation");
    auto m=multiplication3(um,x);S trace=m[0][0]+m[1][1]+m[2][2],trace2;
    for(int i=0;i<3;++i)for(int j=0;j<3;++j)trace2=trace2+m[i][j]*m[j][i];
    SP ut{-determinant3(m),(trace*trace-trace2)/S(2),-trace,S(1)};
    Matrix3<S> interpolation{};SP power{S(1)};
    for(int j=0;j<3;++j){for(int i=0;i<3;++i)interpolation[i][j]=power.at(i);power=(power*x)%um;}
    S denominator=determinant3(interpolation);require(denominator.c[0]==T(1),"formal interpolation chart at origin");
    std::vector<S> vc;
    for(int j=0;j<3;++j){auto a=interpolation;for(int i=0;i<3;++i)a[i][j]=y.at(i);vc.push_back(determinant3(a)/denominator);}
    SP vt(vc);require(((vt*vt-constant_jet(f))%ut).degree()<0,"deformed Mumford equation");
    for(int i=0;i<=3;++i)require(ut.at(i).c[0]==u.at(i),"arc U origin");
    for(int i=0;i<3;++i)require(vt.at(i).c[0]==v.at(i),"arc V origin");
    return {ut,vt};
}

template<class T> T evaluate_arc(const Poly<T>& support_u,const Poly<T>& support_v,
                                const Poly<T>& a,const Poly<T>& b,const CurveArc<T>& arc){
    using S=Jet<T>;using SP=Poly<S>;
    auto m=multiplication3(arc.u,constant_jet(support_u));
    S determinant=determinant3(m);SP w=(arc.v+constant_jet(support_v))%arc.u;
    for(int i=0;i<3;++i)m[i][2]=w.at(i);
    S numerator=determinant3(m);
    S norm=determinant3(multiplication3(arc.u,constant_jet(a)+constant_jet(b)*arc.v));
    S top=numerator*numerator*numerator*norm,bottom=determinant*determinant*determinant;
    int order=bottom.valuation();
    require(order<=18,"formal denominator unresolved within fixed valuation bound");
    for(int i=0;i<order;++i)require(top.c[i]==T(0),"unexpected pole outside theta divisor");
    return top.c[order]/bottom.c[order];
}

F regularized(const Weil3& f,const CurveArc<F>& arc){
    return f.identity?F(1):evaluate_arc(f.support.u,f.support.v,f.a,f.b,arc);
}
