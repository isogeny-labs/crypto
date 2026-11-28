#pragma once
// Tian 2020 sections 2 and 4: actual normalized level-3 Weil functions.
// This is a geometric prerequisite, not a reconstructed quotient curve.
#include "quotient_arithmetic.hpp"
#include <array>

P derivative(const P& a) {
    std::vector<F> out;
    for (int i=1;i<=a.degree();++i) out.push_back(a.at(i)*F(i));
    return P(out);
}

F resultant3(const P& u,const P& a) {
    require(u.degree()==3 && u.at(3)==F(1),"resultant monic cubic");
    std::array<std::array<F,3>,3> m{};
    P x{F(0),F(1)}, v=a%u;
    for(int j=0;j<3;++j) {
        for(int i=0;i<3;++i) m[i][j]=v.at(i);
        v=(v*x)%u;
    }
    return m[0][0]*(m[1][1]*m[2][2]-m[1][2]*m[2][1])
         - m[0][1]*(m[1][0]*m[2][2]-m[1][2]*m[2][0])
         + m[0][2]*(m[1][0]*m[2][1]-m[1][1]*m[2][0]);
}

std::array<F,7> null_vector(std::vector<std::array<F,7>> m) {
    std::vector<int> pivots;
    int row=0;
    for(int col=0;col<7 && row<int(m.size());++col) {
        int pick=row;
        while(pick<int(m.size()) && m[pick][col]==F(0)) ++pick;
        if(pick==int(m.size())) continue;
        std::swap(m[pick],m[row]);
        F inv=m[row][col].inv();
        for(auto& v:m[row]) v=v*inv;
        for(int i=0;i<int(m.size());++i) if(i!=row) {
            F c=m[i][col];
            for(int j=0;j<7;++j) m[i][j]=m[i][j]-c*m[row][j];
        }
        pivots.push_back(col); ++row;
    }
    require(pivots.size()==6,"principal function nullity must be one");
    int free=0;
    while(std::find(pivots.begin(),pivots.end(),free)!=pivots.end()) ++free;
    std::array<F,7> out{}; out[free]=F(1);
    for(int i=0;i<6;++i) out[pivots[i]]=-m[i][free];
    return out;
}

struct Weil3 {
    G index, support;
    P a,b;
    F norm_scale;
    bool identity;
    explicit Weil3(const G& p):index(p),support(p.neg()),identity(p==G{}) {
        if(identity) return;
        support.check();
        require(support.times(3)==G{} && support.u.degree()==3,"order3 function input");
        const P& u=support.u;
        P u2=u*u,u3=u2*u;
        auto [g,s,t]=xgcd(support.v.scale(F(2)),u);
        require(g.degree()==0,"unramified order3 support");
        P lift=support.v;
        lift=lift+u*((((curve-lift*lift).exact(u))*s)%u);
        require(((lift*lift-curve)%u2).degree()<0,"first Hensel lift");
        lift=lift+u2*((((curve-lift*lift).exact(u2))*s)%u);
        require(((lift*lift-curve)%u3).degree()<0,"second Hensel lift");
        std::array<P,7> columns;
        P x{F(0),F(1)},power{F(1)};
        for(int i=0;i<5;++i) {columns[i]=power; power=power*x;}
        columns[5]=lift; columns[6]=(x*lift)%u3;
        std::vector<std::array<F,7>> matrix(9);
        for(int i=0;i<9;++i) for(int j=0;j<7;++j) matrix[i][j]=columns[j].at(i);
        auto n=null_vector(matrix);
        a=P(std::vector<F>(n.begin(),n.begin()+5)); b=P{n[5],n[6]};
        require(((a+b*lift)%u3).degree()<0,"triple support vanishing");
        P quotient=(a*a-b*b*curve).exact(u3);
        require(quotient.degree()==0 && quotient.at(0)!=F(0),"principal norm identity");
        norm_scale=quotient.at(0);
        check();
    }
    void check()const {
        if(identity) return;
        require(a*a-b*b*curve==(support.u*support.u*support.u).scale(norm_scale),"exact h norm identity");
        require(((a+b*support.v)%support.u).degree()<0,"correct h support sign");
    }
    F operator()(const G& z)const {
        if(identity) return F(1);
        require(z.u.degree()==3,"Weil evaluation outside generic degree3 chart");
        auto [g,s,t]=xgcd(support.u,z.u);
        require(g.degree()==0,"Weil support collision");
        P value=((z.v+support.v)*s)%z.u;
        F ratio=value.at(2);
        return ratio*ratio*ratio*resultant3(z.u,a+b*z.v);
    }
};

std::vector<G> generic_inputs(const std::vector<G>& kernel) {
    std::vector<G> points;
    for(int i=1;i<=128 && points.size()<18;++i) {
        F x(i); Z value=scalar(curve.eval(x));
        mpz_class symbol,y;
        mpz_powm(symbol.get_mpz_t(),value.x.get_mpz_t(),mpz_class((prime-1)/2).get_mpz_t(),prime.get_mpz_t());
        if(symbol!=1) continue;
        mpz_powm(y.get_mpz_t(),value.x.get_mpz_t(),mpz_class((prime+1)/4).get_mpz_t(),prime.get_mpz_t());
        G point{P{-x,F(1)},P{F(Poly<Z>{Z(y)})}};point.check();points.push_back(point);
    }
    require(points.size()>=9,"generic support input bound");
    std::vector<G> result;
    for(int i=0;i<16 && i+2<int(points.size()) && result.size()<3;++i) {
        G z=points[i].add(points[i+1]).add(points[i+2]);
        bool usable=true;
        for(auto& k:kernel) {
            G translated=z.add(k);
            if(translated.u.degree()!=3) {usable=false;break;}
            auto [g,s,t]=xgcd(translated.u,derivative(translated.u));
            if(g.degree()!=0) {usable=false;break;}
            for(auto& p:kernel) if(!(p==G{})) {
                auto [gg,ss,tt]=xgcd(translated.u,p.u);
                if(gg.degree()!=0) {usable=false;break;}
            }
            if(!usable) break;
        }
        std::cerr<<"generic candidate "<<i<<" usable="<<usable<<'\n';
        if(usable) result.push_back(z);
    }
    require(result.size()==3,"three regular test classes within fixed bound");
    return result;
}

