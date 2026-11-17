#pragma once
#include "weil3_arithmetic.hpp"
#include "weil_chart.hpp"
#include "geometry_field.hpp"
#include <optional>

T24 sqrt24(const T24& value){
    if(value==T24(0))return T24(0);
    mpz_class q8,q;mpz_pow_ui(q8.get_mpz_t(),prime.get_mpz_t(),8);mpz_pow_ui(q.get_mpz_t(),prime.get_mpz_t(),24);
    require(value.power(mpz_class((q-1)/2))==T24(1),"required theta normalization square root outside working field");
    mpz_class odd=q-1;int s=0;while(mpz_even_p(odd.get_mpz_t())){odd>>=1;++s;}
    F nonsquare;bool found=false;
    for(int i=0;i<16&&!found;++i){nonsquare=F(Poly<Z>{Z(i),Z(1)});found=nonsquare.power(mpz_class((q8-1)/2))==F(-1);}
    require(found,"working-field nonsquare bound");
    T24 c(nonsquare.power(odd)),t=value.power(odd),r=value.power(mpz_class((odd+1)/2));int m=s;
    while(t!=T24(1)){
        int i=1;T24 u=t*t;while(i<m&&u!=T24(1)){u=u*u;++i;}require(i<m,"theta square-root order");
        T24 b=c;for(int j=0;j<m-i-1;++j)b=b*b;r=r*b;c=b*b;t=t*c;m=i;
    }
    require(r*r==value,"theta square root equation");return r;
}

T24 eta2(const GT24& s,const GT24& z){
    int m=s.u.degree();if(m==0)return T24(1);
    require(z.u.degree()==3,"source2 Weil evaluation outside generic degree3 chart");
    auto matrix=multiplication3(z.u,s.u);T24 denominator=determinant3(matrix);
    if(m==1)return denominator;
    if(denominator!=T24(0)){
        for(int i=0;i<3;++i)matrix[i][2]=z.v.at(i);
        T24 numerator=determinant3(matrix);return numerator*numerator/denominator;
    }
    auto arc=curve_arc(z.u,z.v,GT24::f);
    auto a=multiplication3(arc.u,constant_jet(s.u));auto bottom=determinant3(a);
    for(int i=0;i<3;++i)a[i][2]=arc.v.at(i);
    auto n=determinant3(a),top=n*n;int order=bottom.valuation();
    require(order<=6,"source2 Weil denominator valuation bound");
    for(int i=0;i<order;++i)require(top.c[i]==T24(0),"source2 Weil unexpected pole");
    return top.c[order]/bottom.c[order];
}

int d2(int p,int q){return (__builtin_popcount(unsigned((p&7)&(q>>3)))%2)?-1:1;}

struct QuotientSection {
    std::vector<G> source_kernel;
    std::vector<GT24> kernel;
    std::vector<Weil3> functions;
    std::vector<F> alpha;
    std::vector<F> collisions;
    int cached_collisions=0,formal_charts=0;
    std::map<std::string,T24> theta_cache;
    explicit QuotientSection(const std::string& root){
        auto receipt=read_json(root+"/evidence/weil3-v2/result.json");
        auto chart=read_json(root+"/evidence/weil-chart-v1/result.json");
        for(auto& row:receipt.at("functions").arr){
            G p{polynomial(row.at("P_U")),polynomial(row.at("P_V"))};source_kernel.push_back(p);kernel.emplace_back(p);
            functions.emplace_back(p);alpha.push_back(field(row.at("alpha")));
            require(functions.back().a==polynomial(row.at("A"))&&functions.back().b==polynomial(row.at("B")),"principal source binding");
        }
        for(auto& v:chart.at("collision_values").arr)collisions.push_back(field(v));
        require(kernel.size()==27&&collisions.size()==52,"pinned function/chart sizes");
    }
    T24 theta(const GT24& z){
        auto found=theta_cache.find(z.key());if(found!=theta_cache.end())return found->second;
        require(z.u.degree()==3,"quotient theta outside generic degree3 chart");
        int exact_kernel=-1;
        for(int j=1;j<27;++j)if(kernel[j]==z)exact_kernel=j;
        T24 value(1);std::optional<CurveArc<T24>> arc;
        for(int i=1;i<27;++i){
            const auto& f=functions[i];auto u=embed_polynomial<T24>(f.support.u),v=embed_polynomial<T24>(f.support.v);
            auto a=embed_polynomial<T24>(f.a),b=embed_polynomial<T24>(f.b);
            auto matrix=multiplication3(z.u,u);T24 denominator=determinant3(matrix),raw;
            if(denominator!=T24(0)){
                auto w=(z.v+v)%z.u;for(int j=0;j<3;++j)matrix[j][2]=w.at(j);
                T24 c=determinant3(matrix),norm=determinant3(multiplication3(z.u,a+b*z.v));
                raw=c*c*c*norm/(denominator*denominator*denominator);
            }else if(exact_kernel>=0&&(source_kernel[i]==source_kernel[exact_kernel]||source_kernel[i]==source_kernel[exact_kernel].neg())){
                bool same=source_kernel[i]==source_kernel[exact_kernel];raw=T24(collisions[2*(exact_kernel-1)+(same?0:1)]);++cached_collisions;
            }else{
                if(!arc){arc=curve_arc(z.u,z.v,GT24::f);++formal_charts;}
                raw=evaluate_arc(u,v,a,b,*arc);
            }
            value=value+T24(alpha[i])*raw;
        }
        theta_cache.emplace(z.key(),value);return value;
    }
};

