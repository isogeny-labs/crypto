/* Host Fp64 modular subtraction: frozen ternary baseline versus the
 * current header.  Public deterministic residues in the 61-bit prime
 * field; all outputs are checked before any timing or perf arm.
 */
#include "fp64.cuh"
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

static volatile uint64_t sub64_sink = 0;

static RF_BENCH_NOINLINE uint64_t sub_before(const Fp64Ctx &f, uint64_t a, uint64_t b) {
    return a >= b ? a - b : a + f.p - b;
}
static RF_BENCH_NOINLINE uint64_t sub_after(const Fp64Ctx &f, uint64_t a, uint64_t b) {
    return fp64_sub(f, a, b);
}

using Fn = uint64_t(*)(const Fp64Ctx &, uint64_t, uint64_t);

static double median_ns(Fn fn, const Fp64Ctx &f, const std::vector<uint64_t> &a,
                        const std::vector<uint64_t> &b) {
    std::array<double, 9> samples{};
    for (double &ns : samples) {
        uint64_t checksum = 0;
        const auto start = std::chrono::steady_clock::now();
        for (int r = 0; r < 16; ++r)
            for (size_t i = 0; i < a.size(); ++i)
                checksum ^= fn(f, a[i], b[i]);
        const auto stop = std::chrono::steady_clock::now();
        sub64_sink ^= checksum;
        ns = std::chrono::duration<double, std::nano>(stop - start).count()
             / (16.0 * a.size());
    }
    std::sort(samples.begin(), samples.end());
    return samples[samples.size() / 2];
}

int main(int argc, char **argv) {
    constexpr uint64_t prime = 2305843009213693951ull; /* 2^61 - 1 */
    const Fp64Ctx f = fp64_ctx_new(prime);
    std::mt19937_64 rng(0x1234567887654321ull);
    const size_t n = 65536;
    std::vector<uint64_t> a(n), b(n);
    for (size_t i = 0; i < n; ++i) {
        a[i] = rng() % prime;
        b[i] = rng() % prime;
    }
    a[0] = b[0] = 0;
    a[1] = prime - 1;
    b[1] = 0;
    a[2] = 0;
    b[2] = prime - 1;
    a[3] = b[3] = prime - 1;

    for (size_t i = 0; i < n; ++i) {
        if (sub_before(f, a[i], b[i]) != sub_after(f, a[i], b[i])) {
            std::fprintf(stderr, "Fp64 subtraction mismatch at %zu\n", i);
            return 1;
        }
    }

    if (argc == 2) {
        if (!std::strcmp(argv[1], "--perf-before")) {
            std::printf("%.3f\n", median_ns(sub_before, f, a, b)); return 0;
        }
        if (!std::strcmp(argv[1], "--perf-after")) {
            std::printf("%.3f\n", median_ns(sub_after, f, a, b)); return 0;
        }
    }
    if (argc != 1) {
        std::fprintf(stderr, "Usage: %s [--perf-before|--perf-after]\n", argv[0]);
        return 2;
    }
    const double before = median_ns(sub_before, f, a, b);
    const double after = median_ns(sub_after, f, a, b);
    std::printf("Fp64 host subtraction before %.3f after %.3f ns/op speedup %.3fx\n",
                before, after, before / after);
    return 0;
}
