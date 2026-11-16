#pragma once
#include "exact.hpp"
namespace weber {
using Matrix=std::vector<std::vector<I>>;
inline I coefficient_inner(const std::vector<I>& a,const std::vector<I>& b){
    require(a.size()==b.size()&&a.size()%2==0,"Eisenstein metric dimension");I out=0;
    for(size_t j=0;j<a.size();j+=2)out+=2*a[j]*b[j]-a[j]*b[j+1]-a[j+1]*b[j]+2*a[j+1]*b[j+1];
    return out;
}
struct GS {std::vector<std::vector<Q>> mu;std::vector<Q> norm;};
inline GS gram_schmidt(const Matrix& b){
    size_t n=b.size();GS g{std::vector<std::vector<Q>>(n,std::vector<Q>(n)),std::vector<Q>(n)};
    for(size_t i=0;i<n;++i){
        for(size_t j=0;j<i;++j){
            I dot=coefficient_inner(b[i],b[j]);
            Q value(dot);for(size_t k=0;k<j;++k)value-=g.mu[i][k]*g.mu[j][k]*g.norm[k];
            g.mu[i][j]=value/g.norm[j];
        }
        I square=coefficient_inner(b[i],b[i]);
        g.norm[i]=Q(square);
        for(size_t j=0;j<i;++j)g.norm[i]-=g.mu[i][j]*g.mu[i][j]*g.norm[j];
        require(g.norm[i]>0,"dependent lattice basis");
    }
    return g;
}
inline I nearest(const Q& a){
    I n=2*a.get_num()+a.get_den(),d=2*a.get_den(),out;
    mpz_fdiv_q(out.get_mpz_t(),n.get_mpz_t(),d.get_mpz_t());return out;
}
inline I determinant(Matrix b){
    size_t n=b.size();I previous=1;int sign=1;
    for(size_t k=0;k+1<n;++k){
        size_t p=k;while(p<n&&b[p][k]==0)++p;
        if(p==n)return 0;
        if(p!=k){std::swap(b[p],b[k]);sign=-sign;}
        I pivot=b[k][k];
        for(size_t i=k+1;i<n;++i)for(size_t j=k+1;j<n;++j){
            I numerator=b[i][j]*pivot-b[i][k]*b[k][j];
            require(numerator%previous==0,"nonexact Bareiss division");b[i][j]=numerator/previous;
        }
        for(size_t i=k+1;i<n;++i)b[i][k]=0;
        previous=pivot;
    }
    return sign*b.back().back();
}
struct Reduced {Matrix basis,change;unsigned swaps=0,reductions=0,steps=0;};
inline Reduced lll(Matrix b){
    const Matrix original=b;size_t n=b.size();Matrix u(n,std::vector<I>(n));
    for(size_t i=0;i<n;++i)u[i][i]=1;
    GS g=gram_schmidt(b);size_t k=1;Reduced out;
    while(k<n){
        require(++out.steps<=10000,"LLL step cap");
        for(size_t jj=k;jj>0;--jj){
            size_t j=jj-1;I q=nearest(g.mu[k][j]);
            if(q==0)continue;
            for(size_t t=0;t<n;++t){b[k][t]-=q*b[j][t];u[k][t]-=q*u[j][t];}
            for(size_t t=0;t<j;++t)g.mu[k][t]-=Q(q)*g.mu[j][t];
            g.mu[k][j]-=Q(q);++out.reductions;
        }
        Q mu=g.mu[k][k-1];
        if(g.norm[k]>=(Q(3,4)-mu*mu)*g.norm[k-1]){++k;continue;}
        Q oldprev=g.norm[k-1],oldnext=g.norm[k],newprev=oldnext+mu*mu*oldprev;
        Q newmu=mu*oldprev/newprev;
        std::swap(b[k],b[k-1]);std::swap(u[k],u[k-1]);
        for(size_t j=0;j+1<k;++j)std::swap(g.mu[k][j],g.mu[k-1][j]);
        for(size_t i=k+1;i<n;++i){
            Q a=g.mu[i][k],c=g.mu[i][k-1];g.mu[i][k]=c-mu*a;
            g.mu[i][k-1]=a+newmu*g.mu[i][k];
        }
        g.norm[k-1]=newprev;g.norm[k]=oldprev*oldnext/newprev;g.mu[k][k-1]=newmu;
        ++out.swaps;if(k>1)--k;
    }
    GS checked=gram_schmidt(b);
    for(size_t i=0;i<n;++i){
        require(checked.norm[i]==g.norm[i],"incremental GS norm mismatch");
        for(size_t j=0;j<i;++j){
            require(checked.mu[i][j]==g.mu[i][j],"incremental GS coefficient mismatch");
            require(2*abs(checked.mu[i][j])<=1,"basis not size-reduced");
        }
        if(i)require(checked.norm[i]>=(Q(3,4)-checked.mu[i][i-1]*checked.mu[i][i-1])*checked.norm[i-1],
                     "Lovasz condition failed");
        for(size_t j=0;j<n;++j){
            I v=0;for(size_t t=0;t<n;++t)v+=u[i][t]*original[t][j];
            require(v==b[i][j],"basis-change certificate failed");
        }
    }
    require(abs(determinant(u))==1,"nonunimodular basis change");
    out.basis=std::move(b);out.change=std::move(u);return out;
}
} // namespace weber
