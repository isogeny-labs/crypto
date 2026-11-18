/* P-256 host modular subtraction and negation comparisons.
 * Uses the actual updated fp256x64.cuh and frozen previous formulas.
 * Values are canonical residues in [0,p). CUDA is intentionally unchanged.
 * Run each --perf-<operation>-<arm> separately under perf stat on a
 * PMU-enabled machine. A full run checks exact equality first.
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

static volatile uint64_t subneg_sink = 0;

static inline U256x64 old_sub(const Fp256Ctx &f, const U256x64 &a, const U256x64 &b) {
    U256x64 d, t;
    const uint64_t bw = u256_sbb(d, a, b);
    u256_adc(t, d, f.p);
    u256_cmov(d, t, bw);
    return d;
}

static RF_BENCH_NOINLINE U256x64 sub_before(
    const Fp256Ctx &f, const U256x64 &a, const U256x64 &b) {
    return old_sub(f, a, b);
}
static RF_BENCH_NOINLINE U256x64 sub_after(
    const Fp256Ctx &f, const U256x64 &a, const U256x64 &b) {
    return fp256_sub(f, a, b);
}
static RF_BENCH_NOINLINE U256x64 neg_before(
    const Fp256Ctx &f, const U256x64 &a, const U256x64 &) {
    const U256x64 zero = {{0, 0, 0, 0}};
    return old_sub(f, zero, a);
}
static RF_BENCH_NOINLINE U256x64 neg_after(
    const Fp256Ctx &f, const U256x64 &a, const U256x64 &) {
    return fp256_neg(f, a);
}

using Fn = U256x64(*)(const Fp256Ctx &, const U256x64 &, const U256x64 &);

static int cmp256(const U256x64 &a, const U256x64 &b) {
    for (int i = 3; i >= 0; --i) {
        if (a.v[i] != b.v[i]) return a.v[i] < b.v[i] ? -1 : 1;
    }
    return 0;
}

static double median_ns(Fn fn, const Fp256Ctx &f,
                        const std::vector<U256x64> &aa,
                        const std::vector<U256x64> &bb) {
    std::array<double, 9> samples{};
    for (double &ns : samples) {
        uint64_t checksum = 0;
        const auto start = std::chrono::steady_clock::now();
        for (int r = 0; r < 32; ++r) {
            for (size_t i = 0; i < aa.size(); ++i) {
                const U256x64 c = fn(f, aa[i], bb[i]);
                checksum ^= c.v[(i + r) & 3];
            }
        }
        const auto stop = std::chrono::steady_clock::now();
        subneg_sink ^= checksum;
        ns = std::chrono::duration<double, std::nano>(stop - start).count()
             / (32.0 * aa.size());
    }
    std::sort(samples.begin(), samples.end());
    return samples[samples.size() / 2];
}

int main(int argc, char **argv) {
    const Fp256Ctx f = fp256_ctx_p256();
    std::mt19937_64 rng(0xb1f05e23aabbccddull);
    const size_t n = 32768;
    std::vector<U256x64> a(n), b(n);
    for (size_t i = 0; i < n; ++i) {
        do { for (int k = 0; k < 4; ++k) a[i].v[k] = rng(); }
        while (cmp256(a[i], f.p) >= 0);
        do { for (int k = 0; k < 4; ++k) b[i].v[k] = rng(); }
        while (cmp256(b[i], f.p) >= 0);
    }
    a[0] = {{0, 0, 0, 0}};
    b[0] = a[0];
    a[1] = f.p;
    a[1].v[0] -= 1;
    b[1] = a[1];
    a[2] = a[0];
    b[2] = a[1];
    a[3] = a[1];
    b[3] = a[0];

    for (size_t i = 0; i < n; ++i) {
        if (!u256_eq(sub_before(f, a[i], b[i]), sub_after(f, a[i], b[i]))) {
            std::fprintf(stderr, "sub mismatch at %zu\n", i);
            return 1;
        }
        if (!u256_eq(neg_before(f, a[i], b[i]), neg_after(f, a[i], b[i]))) {
            std::fprintf(stderr, "neg mismatch at %zu\n", i);
            return 1;
        }
    }

    if (argc == 2) {
        const char *arm = argv[1];
        if (!std::strcmp(arm, "--perf-sub-before")) {
            std::printf("%.3f\n", median_ns(sub_before, f, a, b)); return 0;
        }
        if (!std::strcmp(arm, "--perf-sub-after")) {
            std::printf("%.3f\n", median_ns(sub_after, f, a, b)); return 0;
        }
        if (!std::strcmp(arm, "--perf-neg-before")) {
            std::printf("%.3f\n", median_ns(neg_before, f, a, b)); return 0;
        }
        if (!std::strcmp(arm, "--perf-neg-after")) {
            std::printf("%.3f\n", median_ns(neg_after, f, a, b)); return 0;
        }
    }
    if (argc != 1) {
        std::fprintf(stderr, "Usage: %s [--perf-{sub,neg}-{before,after}]\n", argv[0]);
        return 2;
    }
    const double sb = median_ns(sub_before, f, a, b);
    const double sa = median_ns(sub_after, f, a, b);
    const double nb = median_ns(neg_before, f, a, b);
    const double na = median_ns(neg_after, f, a, b);
    std::printf("P-256 host subtraction: before %.3f after %.3f ns/op speedup %.3fx\n",
                sb, sa, sb / sa);
    std::printf("P-256 host negation: before %.3f after %.3f ns/op speedup %.3fx\n",
                nb, na, nb / na);
    return 0;
}
