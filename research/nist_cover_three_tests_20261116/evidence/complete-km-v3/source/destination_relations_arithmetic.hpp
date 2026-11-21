#pragma once
#include "point_arithmetic.hpp"
#include <array>
#include <map>


Z zpow(Z a,int n){Z r(1);for(int i=0;i<n;++i)r=r*a;return r;}
void split_linear_part(Poly<Z> f,std::vector<Z>& roots){
    f=f.monic();if(f.degree()<1)return;
    if(f.degree()==1){roots.push_back(-f.at(0));return;}
    Poly<Z> x{Z(0),Z(1)};
    for(int shift=0;shift<64;++shift){
        auto h=powmod(x+Poly<Z>{Z(shift)},mpz_class((prime-1)/2),f)-Poly<Z>{Z(1)};
        auto[g,s,t]=xgcd(f,h);
        if(g.degree()>0&&g.degree()<f.degree()){split_linear_part(g,roots);split_linear_part(f.exact(g),roots);return;}
    }
    throw std::runtime_error("linear-factor splitting exceeded fixed64-shift bound");
}
struct BasePoint {int x;Z y;};
struct Fiber {int x;std::vector<int> support;};
using Row=std::vector<mpz_class>;
int coefficient_rank(std::vector<Row> a,const mpz_class& r){
    if(a.empty())return 0;
    int rank=0,width=int(a[0].size());
    for(int j=0;j<width&&rank<int(a.size());++j){
        int pick=rank;while(pick<int(a.size())&&a[pick][j]==0)++pick;if(pick==int(a.size()))continue;
        std::swap(a[pick],a[rank]);mpz_class inv;require(mpz_invert(inv.get_mpz_t(),a[rank][j].get_mpz_t(),r.get_mpz_t())!=0,"coefficient inverse");
        for(int k=j;k<width;++k){a[rank][k]=(a[rank][k]*inv)%r;if(a[rank][k]<0)a[rank][k]+=r;}
        for(int i=rank+1;i<int(a.size());++i){mpz_class c=a[i][j];for(int k=j;k<width;++k){a[i][k]=(a[i][k]-c*a[rank][k])%r;if(a[i][k]<0)a[i][k]+=r;}}
        ++rank;
    }
    return rank;
}
