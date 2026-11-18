/* Host P-256 Montgomery multiplication: original explicit carry chain
 * versus the native unsigned __int128 implementation in fp256x64.cuh.
 *
 * make -C gpu/rootfind bench-mul256
 * gpu/rootfind/bench_mul256
 * perf stat -r 5 -e cycles,instructions -- gpu/rootfind/bench_mul256 --perf-before
 * perf stat -r 5 -e cycles,instructions -- gpu/rootfind/bench_mul256 --perf-after
 *
 * Public deterministic inputs only. CUDA MAC is intentionally unchanged.
 */
#include "fp256x64.cuh"
#include <algorithm>
#include <array>
#include <chrono>
#include <cstdint>
#include <cstdio>
#include <cstring>
#include <random>
#include <vector>

#if defined(__GNUC__) || defined(__clang__)
#define RF_BENCH_NOINLINE __attribute__((noinline))
#else
#define RF_BENCH_NOINLINE
#endif

static volatile uint64_t bench_sink_mul256 = 0;

/* Frozen pre-optimization carry chain. Must not call u64_mac, because that
 * symbol is the candidate under test in the new header. */
static inline void mac_before(uint64_t cur, uint64_t a, uint64_t b,
                              uint64_t carry, uint64_t &lo, uint64_t &hi) {
    uint64_t plo = a * b;
    uint64_t phi = rf_mulhi64(a, b);
    uint64_t s1 = plo + cur;
    phi += (s1 < plo);
    uint64_t s2 = s1 + carry;
    phi += (s2 < s1);
    lo = s2;
    hi = phi;
}

static RF_BENCH_NOINLINE U256x64 mul_before(const Fp256Ctx &f,
                                           const U256x64 &a, const U256x64 &b) {
    uint64_t t[9];
    for (int i = 0; i < 9; ++i) t[i] = 0;
    for (int i = 0; i < 4; ++i) {
        uint64_t carry = 0;
        for (int j = 0; j < 4; ++j) {
            uint64_t lo, hi;
            mac_before(t[i+j], a.v[j], b.v[i], carry, lo, hi);
            t[i+j] = lo;
            carry = hi;
        }
        t[i+4] = carry;
    }
    for (int i = 0; i < 4; ++i) {
        uint64_t m = t[i] * f.pinv;
        uint64_t carry = 0;
        for (int j = 0; j < 4; ++j) {
            uint64_t lo, hi;
            mac_before(t[i+j], m, f.p.v[j], carry, lo, hi);
            t[i+j] = lo;
            carry = hi;
        }
        for (int k = i+4; k < 9; ++k) {
            uint64_t s = t[k] + carry;
            carry = s < t[k];
            t[k] = s;
        }
    }
    U256x64 r, s;
    for (int i = 0; i < 4; ++i) r.v[i] = t[i+4];
    uint64_t borrow = u256_sbb(s, r, f.p);
    u256_cmov(r, s, (t[8] & 1) | (borrow ^ 1));
    return r;
}

static RF_BENCH_NOINLINE U256x64 mul_after(const Fp256Ctx &f,
                                          const U256x64 &a, const U256x64 &b) {
    return fp256_mul(f, a, b);
}

using MulFn = U256x64(*)(const Fp256Ctx &, const U256x64 &, const U256x64 &);

static double time_ns(MulFn fn, const Fp256Ctx &f,
                      const std::vector<U256x64> &as,
                      const std::vector<U256x64> &bs) {
    std::array<double, 9> times{};
    for (double &ns : times) {
        uint64_t checksum = 0;
        auto begin = std::chrono::steady_clock::now();
        for (int repeat = 0; repeat < 16; ++repeat) {
            for (size_t i = 0; i < as.size(); ++i)
                checksum ^= fn(f, as[i], bs[i]).v[repeat & 3];
        }
        auto end = std::chrono::steady_clock::now();
        bench_sink_mul256 ^= checksum;
        ns = std::chrono::duration<double, std::nano>(end-begin).count()
             / (16.0 * as.size());
    }
    std::sort(times.begin(), times.end());
    return times[4];
}

int main(int argc, char **argv) {
    const Fp256Ctx f = fp256_ctx_p256();
    std::mt19937_64 rng(0x20261117ull);
    std::vector<U256x64> as(32768), bs(32768);
    for (size_t i = 0; i < as.size(); ++i) {
        for (int j = 0; j < 4; ++j) {
            as[i].v[j] = rng();
            bs[i].v[j] = rng();
        }
        /* top bit cleared, so both operands are < P-256 */
        as[i].v[3] &= 0x7fffffffffffffffull;
        bs[i].v[3] &= 0x7fffffffffffffffull;
    }
    as[0] = {{0, 0, 0, 0}};
    bs[0] = {{0, 0, 0, 0}};
    as[1] = f.one;
    bs[1] = f.one;
    as[2] = f.p;
    as[2].v[0] -= 1;
    bs[2] = f.one;

    if (argc == 2) {
        if (std::strcmp(argv[1], "--perf-before") == 0) {
            std::printf("old %.3f ns/mul\n", time_ns(mul_before,f,as,bs));
            return 0;
        }
        if (std::strcmp(argv[1], "--perf-after") == 0) {
            std::printf("new %.3f ns/mul\n", time_ns(mul_after,f,as,bs));
            return 0;
        }
    }
    if (argc != 1) {
        std::fprintf(stderr, "Usage: %s [--perf-before|--perf-after]\n",argv[0]);
        return 2;
    }
    for (size_t i = 0; i < as.size(); ++i) {
        if (!u256_eq(mul_before(f,as[i],bs[i]), mul_after(f,as[i],bs[i]))) {
            std::fprintf(stderr, "P-256 Montgomery mismatch at %zu\n", i);
            return 1;
        }
    }
    const double before = time_ns(mul_before, f, as, bs);
    const double after = time_ns(mul_after, f, as, bs);
    std::printf("P256 Montgomery mul: old %.3f ns new %.3f ns speedup %.3fx (%zu verified pairs)\n",
                before,after,before/after,as.size());
    return 0;
}
