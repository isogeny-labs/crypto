/* Isolated CPU timing for the rootfind.cuh batch-inversion core loops.
 * Uses the actual Fp64 field header. NOT a full rootfinder or GPU benchmark.
 * All inputs are public. No inference about secret-data timing.
 */
#include "fp64.cuh"
#include <algorithm>
#include <array>
#include <chrono>
#include <cstdio>
#include <random>
#include <string>
#include <vector>

#define RF_BENCH_NOINLINE __attribute__((noinline))
static volatile uint64_t bench_sink = 0;

template<int N> static RF_BENCH_NOINLINE uint64_t oracle_before(const Fp64Ctx &f, const uint64_t *x, uint64_t *out) {
    uint64_t d[N], pre[N];
    for (int j=0;j<N;j++) d[j]=x[j];
    uint64_t acc=f.one;
    for (int j=0;j<N;j++) { pre[j]=acc; acc=fp64_mul(f,acc,d[j]); }
    uint64_t iv=fp64_inv(f,acc);
    for (int j=N-1;j>=0;j--) {
        const uint64_t dj=d[j];
        d[j]=fp64_mul(f,iv,pre[j]);
        iv=fp64_mul(f,iv,dj);
    }
    uint64_t checksum=0;
    for (int j=0;j<N;j++) {out[j]=d[j]; checksum^=d[j];}
    return checksum;
}
template<int N> static RF_BENCH_NOINLINE uint64_t oracle_after(const Fp64Ctx &f, const uint64_t *x, uint64_t *out) {
    uint64_t d[N], pre[N];
    for (int j=0;j<N;j++) d[j]=x[j];
    uint64_t acc=d[0];
    for (int j=1;j<N;j++) {pre[j]=acc; acc=fp64_mul(f,acc,d[j]);}
    uint64_t iv=fp64_inv(f,acc);
    for (int j=N-1;j>0;j--) {
        const uint64_t dj=d[j];
        d[j]=fp64_mul(f,iv,pre[j]);
        iv=fp64_mul(f,iv,dj);
    }
    d[0]=iv;
    uint64_t checksum=0;
    for (int j=0;j<N;j++) {out[j]=d[j]; checksum^=d[j];}
    return checksum;
}
static RF_BENCH_NOINLINE uint64_t pair_before(const Fp64Ctx &f, const uint64_t *x, uint64_t *out, int n) {
    uint64_t d[16], pre[16], acc=f.one;
    for (int j=0;j<n;j++) {
        const uint64_t dj=x[j]?x[j]:f.one;
        d[j]=dj; acc=fp64_mul(f,acc,dj); pre[j]=acc;
    }
    uint64_t iv=fp64_inv(f,acc);
    for (int j=n;j-->0;) {
        const uint64_t dj=d[j];
        d[j]=j?fp64_mul(f,iv,pre[j-1]):iv;
        iv=fp64_mul(f,iv,dj);
    }
    uint64_t checksum=0;
    for (int j=0;j<n;j++) {out[j]=d[j]; checksum^=d[j];}
    return checksum;
}
static RF_BENCH_NOINLINE uint64_t pair_after(const Fp64Ctx &f, const uint64_t *x, uint64_t *out, int n) {
    uint64_t d[16], pre[16];
    const uint64_t d0=x[0]?x[0]:f.one;
    d[0]=pre[0]=d0;
    uint64_t acc=d0;
    for (int j=1;j<n;j++) {
        const uint64_t dj=x[j]?x[j]:f.one;
        d[j]=dj; acc=fp64_mul(f,acc,dj); pre[j]=acc;
    }
    uint64_t iv=fp64_inv(f,acc);
    for (int j=n;j-->1;) {
        const uint64_t dj=d[j];
        d[j]=fp64_mul(f,iv,pre[j-1]);
        iv=fp64_mul(f,iv,dj);
    }
    d[0]=iv;
    uint64_t checksum=0;
    for (int j=0;j<n;j++) {out[j]=d[j]; checksum^=d[j];}
    return checksum;
}
template<class Fn> static double sample_ns(Fn fn,int n) {
    const auto a=std::chrono::steady_clock::now();
    uint64_t checksum=0;
    for (int i=0;i<n;i++) checksum^=fn(i);
    bench_sink=bench_sink^checksum;
    const auto b=std::chrono::steady_clock::now();
    return std::chrono::duration<double,std::nano>(b-a).count()/n;
}
template<class Fn> static double median_ns(Fn fn,int n) {
    std::array<double,7> values{};
    for (double &v:values) v=sample_ns(fn,n);
    std::sort(values.begin(),values.end());
    return values[3];
}
int main(int argc,char **argv) {
    const Fp64Ctx f=fp64_ctx_new((1ull<<61)-1);
    const int rows=8192, iterations=2*rows;
    std::mt19937_64 rng(0x5eedabba45ull);
    std::vector<std::array<uint64_t,32>> corpus(rows);
    for (auto &a:corpus) for (auto &x:a)
        x=fp64_to_mont(f,1+rng()%(f.p-1));
    for (int i=0;i<rows;i+=71) corpus[i][0]=f.one;
    for (int i=0;i<rows;i+=79) corpus[i][1]=f.one;
    std::array<uint64_t,32> before{},after{};
    if (argc==1) for (const auto &v:corpus) {
        if (oracle_before<16>(f,v.data(),before.data())!=oracle_after<16>(f,v.data(),after.data())
                || !std::equal(before.begin(),before.begin()+16,after.begin())) return 1;
        if (oracle_before<32>(f,v.data(),before.data())!=oracle_after<32>(f,v.data(),after.data())
                || before!=after) return 2;
        for (int n:{1,16}) if (pair_before(f,v.data(),before.data(),n)!=pair_after(f,v.data(),after.data(),n)
                || !std::equal(before.begin(),before.begin()+n,after.begin())) return 3;
    }
    if (argc==1) std::puts("PASS: 8192 public rows; batch outputs bitwise identical");
    auto b16=[&](int i){return oracle_before<16>(f,corpus[i%rows].data(),before.data());};
    auto a16=[&](int i){return oracle_after<16>(f,corpus[i%rows].data(),after.data());};
    auto b32=[&](int i){return oracle_before<32>(f,corpus[i%rows].data(),before.data());};
    auto a32=[&](int i){return oracle_after<32>(f,corpus[i%rows].data(),after.data());};
    auto bp16=[&](int i){return pair_before(f,corpus[i%rows].data(),before.data(),16);};
    auto ap16=[&](int i){return pair_after(f,corpus[i%rows].data(),after.data(),16);};
    auto bp1=[&](int i){return pair_before(f,corpus[i%rows].data(),before.data(),1);};
    auto ap1=[&](int i){return pair_after(f,corpus[i%rows].data(),after.data(),1);};
    const char *names[]={"oracle16","oracle32","pair16","pair1"};
    const char *arms[]={"before","after"};
    for (int k=0;k<4;k++) {
        double t[2]={};
        for (int j=0;j<2;j++) {
            if (argc>1 && std::string(argv[1])!=std::string("--perf-")+names[k]+"-"+arms[j]) continue;
            t[j] = k==0 ? (j==0 ? median_ns(b16,iterations) : median_ns(a16,iterations))
                 : k==1 ? (j==0 ? median_ns(b32,iterations) : median_ns(a32,iterations))
                 : k==2 ? (j==0 ? median_ns(bp16,iterations) : median_ns(ap16,iterations))
                        : (j==0 ? median_ns(bp1,iterations) : median_ns(ap1,iterations));
            std::printf("%s %s: %.2f ns/op\n",names[k],arms[j],t[j]);
        }
        if (t[0]>0 && t[1]>0) std::printf("%s speedup %.4fx\n",names[k],t[0]/t[1]);
    }
    return 0;
}
