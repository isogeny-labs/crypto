// General contact support, including vertical bitangents, over a quadratic algebra.
struct ContactAlgebra {TP modulus,x,y,xnorm;};
struct ContactTarget:TargetWeil {
    std::map<int,ContactAlgebra> algebra_contacts;
    int vertical_contacts=0;
    explicit ContactTarget(const std::string& root):TargetWeil(root){}
    ContactAlgebra contact_algebra(int label){
        auto hit=algebra_contacts.find(label);if(hit!=algebra_contacts.end())return hit->second;
        const auto& l=lines.at(label);TP t{T24(0),T24(1)},x,y;
        bool vertical=l[1]==T24(0);
        if(vertical){require(l[0]!=T24(0),"finite contact line");x=TP{-l[2]/l[0]};y=t;}
        else{x=t;y=TP{-l[2]/l[1],-l[0]/l[1]};}
        TP restriction;for(auto& [e,c]:f.c)restriction=restriction+(tpower(x,e[0])*tpower(y,e[1])).scale(c);
        require(restriction.degree()==4,"parameter contact restriction degree");T24 lc=restriction.at(4),b=restriction.at(3)/(T24(2)*lc),c=(restriction.at(2)/lc-b*b)/T24(2);TP u{c,b,T24(1)};
        require(restriction==(u*u).scale(lc),"parameter contact square factorization");require(b*b-T24(4)*c!=T24(0),"distinct contact parameter support");
        x=x%u;y=y%u;require(equation(x,y,u).degree()<0,"contact algebra curve equation");require((TP{l[2]}+x.scale(l[0])+y.scale(l[1])).degree()<0,"contact algebra line equation");
        TP xn=vertical?tpower(TP{-x.at(0),T24(1)},2):u;
        ContactAlgebra a{u,x,y,xn};algebra_contacts.emplace(label,a);++contact_checks;if(vertical)++vertical_contacts;return a;
    }
    TSpace contact_conditions(const ContactAlgebra& a)const{
        std::array<TP,6> mon{TP{T24(1)},a.x,a.y,a.x*a.x,a.x*a.y,a.y*a.y};TSpace out;
        for(int i=0;i<2;++i){TVector row;for(auto& p:mon)row.push_back((p%a.modulus).at(i));out.push_back(row);}return out;
    }
    void check_contact_conic(const Conic& q,const ContactAlgebra& a)const{require((conic_value(q,a.x,a.y)%a.modulus).degree()<0,"known contact conic support");}
    Conic contact_conic(const FiniteDivisor& e,const ContactAlgebra& a,bool has_infinity1){
        std::vector<Line> inf;if(has_infinity1)inf.push_back(infinity1);auto rows=conditions({e},inf),extra=contact_conditions(a);rows.insert(rows.end(),extra.begin(),extra.end());
        auto basis=kernel_matrix(rows,6);require(basis.size()==1,"unique contact residual conic");Conic q;std::copy(basis[0].begin(),basis[0].end(),q.begin());conic_witnesses.push_back(q);return q;
    }
    FiniteDivisor contact_residual(const Conic& q,const FiniteDivisor& e,const ContactAlgebra& a,bool has_infinity1){
        require(q[5]!=T24(0),"contact conic y-leading unit");TP B{q[2],q[4]},C{q[0],q[1],q[3]},N,L,p0{T24(1)},p1;std::array<TP,5> ys;
        for(auto& [powers,c]:f.c){auto v=ys[powers[1]].c;if(v.size()<=size_t(powers[0]))v.resize(size_t(powers[0]+1));v[powers[0]]=v[powers[0]]+c;ys[powers[1]]=TP(v);}
        for(int j=0;j<=4;++j){N=N+ys[j]*p0;L=L+ys[j]*p1;TP next0=-(C*p1).scale(q[5].inv()),next1=p0-(B*p1).scale(q[5].inv());p0=next0;p1=next1;}
        TP result=(N*N).scale(q[5])-B*L*N+C*L*L,known=e.u*a.xnorm;
        require((conic_value(q,TP{T24(0),T24(1)},e.v)%e.u).degree()<0,"known finite conic support");check_contact_conic(q,a);
        if(has_infinity1){T24 value;auto row=infinity_constraint(infinity1);for(int j=0;j<6;++j)value=value+q[j]*row[j];require(value==T24(0),"known infinity conic support");}
        require(result.degree()==known.degree()+3,"contact residual exact degree");TP u=result.exact(known).monic();auto[g,s,t]=xgcd(L,u);require(g.degree()==0,"contact residual y recovery unit");TP v=(-N*s)%u;
        FiniteDivisor d{u,v};check(d,3);require((conic_value(q,TP{T24(0),T24(1)},v)%u).degree()<0,"contact residual conic equation");require(result.monic()==known.monic()*u,"full contact intersection factorization");++intersections;return d;
    }
    FiniteDivisor translate(const FiniteDivisor& e,int label,bool has_infinity1=false){
        auto a=contact_algebra(label);auto q=contact_conic(e,a,has_infinity1);auto r=contact_residual(q,e,a,has_infinity1);auto q2=unique_conic({r},{infinity1,infinity2});return residual(q2,{r},{infinity1,infinity2});
    }
    std::array<Conic,3> weil_basis(int label){
        auto a=contact_algebra(label);auto rows=contact_conditions(a);rows.push_back(infinity_constraint(infinity2));auto basis=kernel_matrix(rows,6);require(basis.size()==3,"contact Weil RR dimension3");std::array<Conic,3> out{};for(int i=0;i<3;++i)std::copy(basis[i].begin(),basis[i].end(),out[i].begin());return out;
    }
    T24 raw(int label,const FiniteDivisor& e){
        check(e,3);const auto& l=lines.at(label);TP x{T24(0),T24(1)},ell=TP{l[2]}+x.scale(l[0])+e.v.scale(l[1]);T24 norm=determinant3(multiplication3(e.u,ell));require(norm!=T24(0)&&e.v.at(2)!=T24(0),"contact Weil regular denominator");
        auto basis=weil_basis(label);Matrix3<T24> m{};for(int j=0;j<3;++j){TP value=conic_value(basis[j],x,e.v)%e.u;for(int i=0;i<3;++i)m[i][j]=value.at(i);}T24 d=determinant3(m);return d*d/(e.v.at(2)*e.v.at(2)*norm);
    }
};
