/* Independent P-256 host limb carry/borrow benchmark.
 * Frozen bitwise-comparison implementations are the immediate before arms.
 * The new header uses x86 carry-chain intrinsics where available. CUDA
 * and non-x86 host builds retain the portable reference implementation.
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
static volatile uint64_t carry_sink = 0;

static RF_BENCH_NOINLINE uint64_t adc_before(U256x64 &r, const U256x64 &a, const U256x64 &b) {
    uint64_t carry = 0;
    for (int i = 0; i < 4; ++i) {
        const uint64_t s = a.v[i] + b.v[i];
        const uint64_t c1 = s < a.v[i];
        const uint64_t t = s + carry;
        const uint64_t c2 = t < s;
        r.v[i] = t;
        carry = c1 | c2;
    }
    return carry;
}
static RF_BENCH_NOINLINE uint64_t sbb_before(U256x64 &r, const U256x64 &a, const U256x64 &b) {
    uint64_t borrow = 0;
    for (int i = 0; i < 4; ++i) {
        const uint64_t d = a.v[i] - b.v[i];
        const uint64_t b1 = a.v[i] < b.v[i];
        const uint64_t t = d - borrow;
        const uint64_t b2 = d < borrow;
        r.v[i] = t;
        borrow = b1 | b2;
    }
    return borrow;
}
static RF_BENCH_NOINLINE uint64_t adc_after(U256x64 &r, const U256x64 &a, const U256x64 &b) {
    return u256_adc(r, a, b);
}
static RF_BENCH_NOINLINE uint64_t sbb_after(U256x64 &r, const U256x64 &a, const U256x64 &b) {
    return u256_sbb(r, a, b);
}

using CarryFn = uint64_t(*)(U256x64 &, const U256x64 &, const U256x64 &);
static double time_ns(CarryFn fn, const std::vector<U256x64> &aa,
                      const std::vector<U256x64> &bb) {
    std::array<double, 9> samples{};
    for (auto &ns : samples) {
        uint64_t sum = 0;
        U256x64 out;
        auto t0 = std::chrono::steady_clock::now();
        for (int r = 0; r < 64; r++)
            for (size_t i = 0; i < aa.size(); ++i) {
                sum ^= fn(out, aa[i], bb[i]);
                sum ^= out.v[(i + r) & 3];
            }
        auto t1 = std::chrono::steady_clock::now();
        carry_sink ^= sum;
        ns = std::chrono::duration<double, std::nano>(t1 - t0).count()
             / (64.0 * aa.size());
    }
    std::sort(samples.begin(), samples.end());
    return samples[4];
}

int main(int argc, char **argv) {
    std::mt19937_64 rng(0xdefcba9876543210ull);
    std::vector<U256x64> aa(32768), bb(32768);
    for (size_t i = 0; i < aa.size(); ++i)
        for (int j = 0; j < 4; j++) {
            aa[i].v[j] = rng();
            bb[i].v[j] = rng();
        }
    aa[0] = {{0, 0, 0, 0}};
    bb[0] = {{0, 0, 0, 0}};
    aa[1] = {{~0ull, ~0ull, ~0ull, ~0ull}};
    bb[1] = {{1, 0, 0, 0}};
    aa[2] = {{0, 0, 0, 0}};
    bb[2] = {{1, 0, 0, 0}};
    aa[3] = aa[1];
    bb[3] = aa[1];

    for (size_t i = 0; i < aa.size(); i++) {
        U256x64 x, y;
        const uint64_t ca = adc_before(x, aa[i], bb[i]);
        const uint64_t cb = adc_after(y, aa[i], bb[i]);
        if (ca != cb || !u256_eq(x, y)) {
            std::fprintf(stderr, "ADC mismatch at %zu\n", i);
            return 1;
        }
        const uint64_t sa = sbb_before(x, aa[i], bb[i]);
        const uint64_t sb = sbb_after(y, aa[i], bb[i]);
        if (sa != sb || !u256_eq(x, y)) {
            std::fprintf(stderr, "SBB mismatch at %zu\n", i);
            return 1;
        }
    }

    if (argc == 2) {
        const char *arg = argv[1];
        if (!std::strcmp(arg, "--perf-adc-before")) { std::printf("%.3f\n",time_ns(adc_before,aa,bb)); return 0; }
        if (!std::strcmp(arg, "--perf-adc-after"))  { std::printf("%.3f\n",time_ns(adc_after,aa,bb)); return 0; }
        if (!std::strcmp(arg, "--perf-sbb-before")) { std::printf("%.3f\n",time_ns(sbb_before,aa,bb)); return 0; }
        if (!std::strcmp(arg, "--perf-sbb-after"))  { std::printf("%.3f\n",time_ns(sbb_after,aa,bb)); return 0; }
    }
    if (argc != 1) {
        std::fprintf(stderr, "Usage: %s [--perf-{adc,sbb}-{before,after}]\n",argv[0]);
        return 2;
    }
    const double a0 = time_ns(adc_before,aa,bb), a1 = time_ns(adc_after,aa,bb);
    const double s0 = time_ns(sbb_before,aa,bb), s1 = time_ns(sbb_after,aa,bb);
    std::printf("P-256 4-limb ADC before %.3f after %.3f ns speedup %.3fx\n",a0,a1,a0/a1);
    std::printf("P-256 4-limb SBB before %.3f after %.3f ns speedup %.3fx\n",s0,s1,s0/s1);
    return 0;
}
