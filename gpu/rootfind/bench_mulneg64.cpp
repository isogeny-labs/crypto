/* Host Fp64 Montgomery wide-MAC and branchless negation: before vs after.
 * Same actual field header, same deterministic public corpus; isolated
 * --perf-* arms for hardware counters on a suitable runner.
 */
#include "fp64.cuh"
#include <algorithm>
#include <chrono>
#include <cstdint>
#include <cstdio>
#include <cstring>
#include <random>
#include <vector>
static volatile uint64_t sink_mulneg64 = 0;
#if defined(__GNUC__) || defined(__clang__)
#define RF_MN_NOINLINE __attribute__((noinline))
#else
#define RF_MN_NOINLINE
#endif
struct Entry { uint64_t a,b; };
static RF_MN_NOINLINE uint64_t mul_before(const Fp64Ctx &f, uint64_t a, uint64_t b) {
    return fp64_reduce(f, rf_mulhi64(a,b), a*b);
}
static RF_MN_NOINLINE uint64_t mul_after(const Fp64Ctx &f, uint64_t a, uint64_t b) {
    return fp64_mul(f,a,b);
}
static RF_MN_NOINLINE uint64_t neg_before(const Fp64Ctx &f, uint64_t a, uint64_t) {
    return a == 0 ? 0 : f.p-a;
}
static RF_MN_NOINLINE uint64_t neg_after(const Fp64Ctx &f, uint64_t a, uint64_t) {
    return fp64_neg(f,a);
}
typedef uint64_t (*Fn)(const Fp64Ctx&,uint64_t,uint64_t);
static double benchmark(Fn fn, const Fp64Ctx &f, const std::vector<Entry> &xs) {
    std::vector<double> ns;
    for (int r=0; r<9; ++r) {
        uint64_t sum=0;
        auto start=std::chrono::steady_clock::now();
        for (const Entry &x : xs) sum ^= fn(f,x.a,x.b);
        auto stop=std::chrono::steady_clock::now();
        sink_mulneg64 ^= sum;
        ns.push_back(std::chrono::duration<double,std::nano>(stop-start).count()/xs.size());
    }
    std::sort(ns.begin(),ns.end());
    return ns[4];
}
int main(int argc,char**argv) {
    // 2^61 - 1: prime, under the documented p<2^63 arithmetic bound.
    const Fp64Ctx f=fp64_ctx_new((1ull<<61)-1);
    std::mt19937_64 rng(0x642561711ull);
    std::vector<Entry> xs(65536);
    for (Entry &x : xs) {x.a=rng()%f.p; x.b=rng()%f.p;}
    xs[0]={0,0}; xs[1]={1,0}; xs[2]={f.p-1,f.p-1};
    for (const Entry &x:xs) {
        const uint64_t bm=mul_before(f,x.a,x.b),am=mul_after(f,x.a,x.b);
        const uint64_t bn=neg_before(f,x.a,x.b),an=neg_after(f,x.a,x.b);
        if (bm!=am || bn!=an) {
            std::fprintf(stderr, "Fp64 before/after mismatch\n");
            return 1;
        }
    }
    if (argc==2) {
        Fn fn=nullptr;
        if (!std::strcmp(argv[1],"--perf-mul-before")) fn=mul_before;
        else if (!std::strcmp(argv[1],"--perf-mul-after")) fn=mul_after;
        else if (!std::strcmp(argv[1],"--perf-neg-before")) fn=neg_before;
        else if (!std::strcmp(argv[1],"--perf-neg-after")) fn=neg_after;
        else {std::fprintf(stderr,"Unknown mode\n");return 2;}
        std::printf("Fp64 isolated %s: %.3f ns/op\n",argv[1],benchmark(fn,f,xs));
        return 0;
    }
    if(argc!=1) return 2;
    const double mb=benchmark(mul_before,f,xs),ma=benchmark(mul_after,f,xs);
    const double nb=benchmark(neg_before,f,xs),na=benchmark(neg_after,f,xs);
    std::printf("Fp64 Montgomery mul old=%.3f ns new=%.3f ns speedup=%.3fx\n",mb,ma,mb/ma);
    std::printf("Fp64 modular neg old=%.3f ns new=%.3f ns speedup=%.3fx\n",nb,na,nb/na);
    return 0;
}
