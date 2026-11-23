#include <algorithm>
#include <array>
#include <cassert>
#include <cstdio>
#include <vector>
#include "normalized-cubic.inc"
#include "g4-resolvent.inc"

template<int P> void audit(){
    Quartics<P> table;
    auto mod=table.mod;
    using Tuple=std::array<int,4>;
    std::vector<Tuple> quartics(P*P*P,Tuple{-1,-1,-1,-1});
    std::vector<Tuple> cubics(P*P,Tuple{-1,-1,-1,-1});
    for(int i=0;i<P;i++)for(int j=i;j<P;j++){
        int k=mod(-i-j);
        if(j<=k){
            int a=mod(i*j+i*k+j*k),b=mod(-i*j*k);
            cubics[a*P+b]={i,j,k,-1};
        }
        for(int k=j;k<P;k++){
            int l=mod(-i-j-k);if(l<k)continue;
            int a=mod(i*j+k*l+(i+j)*(k+l));
            int b=mod(-i*j*(k+l)-k*l*(i+j)),c=mod(1LL*i*j*k*l);
            quartics[(a*P+b)*P+c]={i,j,k,l};
        }
    }
    int n3=0,n4=0;
    for(int a=0;a<P;a++)for(int b=0;b<P;b++){
        int h=mod(a+2*b+1),hh=mod(h*h);
        auto roots=table.split(mod(3*h),mod(a+3*hh),mod(b+h*(hh+a)));
        const auto &cubic=cubics[a*P+b];assert((roots.n==3)==(cubic[0]>=0));
        if(roots.n==3){
            for(int &x:roots.r)x=mod(x+h);
            std::sort(roots.r.begin(),roots.r.end());
            assert(std::equal(roots.r.begin(),roots.r.end(),cubic.begin()));++n3;
        }
        for(int c=0;c<P;c++){
            int q[]={mod(c+h*(b+h*(a+hh))),mod(b+2*a*h+4*h*hh),mod(a+6*hh),mod(4*h),1};
            int scale=1+(a+b+c)%(P-1);for(int &v:q)v=mod(v*scale);
            Tuple found{};int n=table.split4(q,found.data());
            const auto &expected=quartics[(a*P+b)*P+c];
            assert((n==4)==(expected[0]>=0));
            if(n==4){
                for(int &v:found)v=mod(v+h);
                std::sort(found.begin(),found.end());assert(found==expected);++n4;
            }
        }
    }
    printf("%d %d %d %d %d\n",P,P*P,n3,P*P*P,n4);
}
int main(){
    audit<31>();audit<37>();audit<41>();audit<43>();audit<47>();
    audit<53>();audit<59>();audit<61>();audit<67>();
}
