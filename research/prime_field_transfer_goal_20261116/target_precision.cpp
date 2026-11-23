// Target-side classical Tate coordinate controls; never a source-point map.
#include <cstdint>
#include <iostream>
#include <map>
#include <stdexcept>
#include <utility>

using I = std::int64_t;
I mod(I a,I m) {a%=m;return a<0?a+m:a;}
I inverse(I a,I m) {
    a=mod(a,m);I original=m,u=1,v=0;
    while(m) {I q=a/m,na=a-q*m,nu=u-q*v;a=m;m=na;u=v;v=nu;}
    if(a!=1)throw std::runtime_error("nonunit Tate-series denominator");
    return mod(u,original);
}
int main() try {
    for(I r:{19,83,127}) {
        for(unsigned n:{3U,5U,8U}) {
            I m=I(1)<<n,zeros=0;
            std::map<std::pair<I,I>,unsigned> buckets;
            for(I k=1;k<r;++k) {
                I u=k>=n?0:(I(1)<<k),v=r-k>=n?0:(I(1)<<(r-k));
                I du=inverse(1-u,m),dv=inverse(1-v,m);
                I x=mod(u*du*du+v*dv*dv,m);
                I y=mod(u*u*du*du*du-v*dv*dv*dv,m);
                if(mod(y*y+x*y-x*x*x,m)!=0)throw std::runtime_error("Tate residue left reduced model");
                ++buckets[{x,y}];
                if(x==0&&y==0)++zeros;
            }
            I predicted=r-2*n+1;
            if(zeros!=predicted)throw std::runtime_error("unexpected target collision count");
            if(buckets.size()>=static_cast<unsigned>(r-1))throw std::runtime_error("bad injectivity control");
            std::cout<<"{\"r\":"<<r<<",\"absolute_precision\":"<<n<<",\"nonzero_auditor_labels\":"
                <<r-1<<",\"distinct_coordinate_pairs\":"<<buckets.size()<<",\"labels_at_zero_pair\":"
                <<zeros<<",\"predicted_zero_pair\":"<<predicted
                <<",\"faithful_encoding\":false,\"source_point_transfer\":false}\n";
        }
    }
} catch(const std::exception& e) {std::cerr<<e.what()<<'\n';return 1;}
