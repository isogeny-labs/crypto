/* P-256 host inverse: frozen radix-16 chain versus fixed-block chain.
 * Uses actual fp256x64.cuh operations and deterministic public inputs.
 * Isolated modes allow hardware counters on PMU-enabled hosts.
 */
#include "fp256x64.cuh"
#include <algorithm>
#include <chrono>
#include <cstdio>
#include <cstring>
#include <random>
#include <vector>
static volatile uint64_t sink_chain256 = 0;
#if defined(__GNUC__) || defined(__clang__)
#define RF_CHAIN_NOINLINE __attribute__((noinline))
#else
#define RF_CHAIN_NOINLINE
#endif

/* Previous host implementation, preserved for a fair same-header comparison. */
static RF_CHAIN_NOINLINE U256x64 chain_before(const Fp256Ctx &f, const U256x64 &a) {
    const U256x64 a2 = fp256_sqr(f, a);
    const U256x64 a3 = fp256_mul(f, a2, a);
    const U256x64 a12 = fp256_sqr(f, fp256_sqr(f, a3));
    const U256x64 a13 = fp256_mul(f, a12, a);
    const U256x64 a15 = fp256_mul(f, a12, a3);
    U256x64 e = f.p;
    e.v[0] -= 2;
    U256x64 acc = a15;
    for (int i = 62; i >= 0; --i) {
        acc = fp256_sqr(f, acc);
        acc = fp256_sqr(f, acc);
        acc = fp256_sqr(f, acc);
        acc = fp256_sqr(f, acc);
        const unsigned digit = (unsigned)((e.v[i >> 4] >> (4 * (i & 15))) & 15u);
        if (digit == 15) acc = fp256_mul(f, acc, a15);
        else if (digit == 13) acc = fp256_mul(f, acc, a13);
        else if (digit == 1) acc = fp256_mul(f, acc, a);
    }
    return acc;
}
static RF_CHAIN_NOINLINE U256x64 chain_after(const Fp256Ctx &f, const U256x64 &a) {
    return fp256_inv(f, a);
}
typedef U256x64 (*ChainFn)(const Fp256Ctx&, const U256x64&);
static double benchmark(ChainFn fn, const Fp256Ctx &f, const std::vector<U256x64> &xs) {
    std::vector<double> ns;
    for (int rep = 0; rep < 9; ++rep) {
        uint64_t sum = 0;
        const auto start = std::chrono::steady_clock::now();
        for (const U256x64 &x : xs) sum ^= fn(f, x).v[0];
        const auto stop = std::chrono::steady_clock::now();
        sink_chain256 ^= sum;
        ns.push_back(std::chrono::duration<double, std::nano>(stop-start).count() / xs.size());
    }
    std::sort(ns.begin(), ns.end());
    return ns[4];
}
int main(int argc, char** argv) {
    const Fp256Ctx f = fp256_ctx_p256();
    std::mt19937_64 rng(0x20111725611ull);
    std::vector<U256x64> xs(4096);
    for (U256x64 &x : xs) {
        U256x64 canonical = {{rng(), rng(), rng(), rng() & ((1ull << 62)-1)}};
        x = fp256_to_mont(f, canonical);
    }
    xs[0] = f.one;
    const U256x64 zero = {{0,0,0,0}};
    if (!u256_eq(chain_before(f,zero),chain_after(f,zero))) {
        std::fprintf(stderr, "P-256 zero inverse mismatch\n");
        return 1;
    }
    for (const U256x64 &x : xs) {
        const U256x64 b = chain_before(f,x), a = chain_after(f,x);
        if (!u256_eq(b,a) || !u256_eq(fp256_mul(f,x,a),f.one)) {
            std::fprintf(stderr, "P-256 chain output mismatch\n");
            return 1;
        }
    }
    if (argc == 2) {
        if (!std::strcmp(argv[1],"--perf-before"))
            std::printf("P256 old radix16 %.3f ns\n",benchmark(chain_before,f,xs));
        else if (!std::strcmp(argv[1],"--perf-after"))
            std::printf("P256 fixed blocks %.3f ns\n",benchmark(chain_after,f,xs));
        else {std::fprintf(stderr,"Unknown mode\n");return 2;}
        return 0;
    }
    if (argc != 1) return 2;
    const double old_ns = benchmark(chain_before,f,xs);
    const double new_ns = benchmark(chain_after,f,xs);
    std::printf("P256 host inverse fixed-block: old=%.3f ns new=%.3f ns speedup=%.3fx (4096 verified inputs)\n",
                old_ns,new_ns,old_ns/new_ns);
    return 0;
}
