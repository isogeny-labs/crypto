/* Per-thread decomposition target/chunk mapping, actual rootfind.cuh.
 * Compare frozen 64-bit division with guarded 32-bit host quotient/remainder.
 * The fallback is checked above 2^32 as well.
 */
#include "rootfind.cuh"
#include <algorithm>
#include <chrono>
#include <cstdint>
#include <cstdio>
#include <cstring>
#include <random>
#include <vector>
#if defined(__GNUC__) || defined(__clang__)
#define RF_MAP_NOINLINE __attribute__((noinline))
#else
#define RF_MAP_NOINLINE
#endif
static volatile uint64_t sink_map = 0;
struct Pair {uint32_t target,i0;};
static RF_MAP_NOINLINE Pair map_before(uint64_t tid,uint32_t n,uint32_t chunk) {
    return {(uint32_t)(tid % (uint64_t)n),
            (uint32_t)(tid / (uint64_t)n) * chunk};
}
static RF_MAP_NOINLINE Pair map_after(uint64_t tid,uint32_t n,uint32_t chunk) {
    Pair p;
    rf_thread_map(tid,n,chunk,p.target,p.i0);
    return p;
}
typedef Pair(*MapFn)(uint64_t,uint32_t,uint32_t);
static double bench(MapFn fn,const std::vector<uint64_t> &ids,uint32_t n) {
    std::vector<double> times;
    for (int rep=0;rep<9;rep++) {
        uint64_t sum=0;
        const auto start=std::chrono::steady_clock::now();
        for (uint64_t tid:ids) {
            const Pair p=fn(tid,n,32);
            sum+=p.target+p.i0;
        }
        const auto stop=std::chrono::steady_clock::now();
        sink_map ^= sum;
        times.push_back(std::chrono::duration<double,std::nano>(stop-start).count()/ids.size());
    }
    std::sort(times.begin(),times.end());
    return times[4];
}
int main(int argc,char**argv) {
    const uint32_t n=10007;
    std::mt19937_64 rng(0xA11FA501ull);
    std::vector<uint64_t> ids32(65536),ids64(65536);
    for (size_t i=0;i<ids32.size();i++) {
        ids32[i]=rng() % ((uint64_t)n*256);
        ids64[i]=(1ull<<32)+(rng() % (1ull<<36));
    }
    ids32[0]=0;
    ids32[1]=((uint64_t)n*256)-1;
    ids64[0]=(1ull<<32);
    ids64[1]=~0ull;
    for (const auto &ids : {ids32,ids64}) {
        for (uint64_t t:ids) {
            const Pair a=map_before(t,n,32), b=map_after(t,n,32);
            if(a.target!=b.target || a.i0!=b.i0) {
                std::fprintf(stderr,"Thread map mismatch tid=%llu\n",
                             (unsigned long long)t);
                return 1;
            }
        }
    }
    if(argc==2) {
        const char*mode=argv[1];
        MapFn fn=nullptr;
        const std::vector<uint64_t>*xs=nullptr;
        if (!std::strcmp(mode,"--perf-map32-before")) {fn=map_before;xs=&ids32;}
        else if (!std::strcmp(mode,"--perf-map32-after")) {fn=map_after;xs=&ids32;}
        else if (!std::strcmp(mode,"--perf-map64-before")) {fn=map_before;xs=&ids64;}
        else if (!std::strcmp(mode,"--perf-map64-after")) {fn=map_after;xs=&ids64;}
        else return 2;
        std::printf("%s %.3f ns/op\n",mode,bench(fn,*xs,n));
        return 0;
    }
    if(argc!=1) return 2;
    for (int mode=0;mode<2;mode++) {
        const std::vector<uint64_t> &ids=mode==0?ids32:ids64;
        const double a=bench(map_before,ids,n),b=bench(map_after,ids,n);
        std::printf("rootfind thread-map %s: old=%.3f ns new=%.3f ns speedup=%.3fx\n",
                    mode==0?"32":"64",a,b,a/b);
    }
    return 0;
}
