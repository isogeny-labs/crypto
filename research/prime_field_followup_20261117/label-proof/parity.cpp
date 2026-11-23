#define main census_entry
#include "census.cpp"
#undef main

int main() {
    int masks_checked=0;
    for (int m=3;m<=16;++m) {
        for (int mask=0;mask<(1<<m);++mask) {
            int b=0;
            for (int i=0;i<m;++i) {
                int weight=(m%2==0 && i==m-1)?2:1;
                b+=mask>>i&1?-weight:weight;
            }
            assert(b && (b%2!=0) && std::abs(b)<=m+1);
            ++masks_checked;
        }
    }
    int point_checks=0;
    for (int p:{101,211,503,1009}) {
        auto curve=first_curve(p);
        int r=curve.n,q=17;
        auto scalar=[&](int x){return (x%r+r)%r;};
        const std::array<int,4> low{1,1,1,2}, old{1,2,4,8};
        for (int i=0;i<4;++i) for (int u=0;u<r;++u) {
            int coupled=scalar(u+(old[i]-low[i])*q);
            auto original=curve.add(curve.multiples[u],curve.multiples[scalar(old[i]*q)]);
            auto substitute=curve.add(curve.multiples[coupled],curve.multiples[scalar(low[i]*q)]);
            assert(original==substitute);
            ++point_checks;
        }
    }
    std::cout<<"signed_masks_checked="<<masks_checked<<" coupled_point_checks="<<point_checks<<'\n';
}
