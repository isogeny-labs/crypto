/* P-256 Montgomery-to-canonical conversion, actual header vs frozen
 * generic multiply-by-one baseline.  Public fixed corpus; byte-for-byte
 * equality is verified before either timed / perf arm.
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
static volatile uint64_t from_sink = 0;

static RF_BENCH_NOINLINE U256x64 from_before(const Fp256Ctx &f, const U256x64 &a) {
    const U256x64 one = {{1, 0, 0, 0}};
    return fp256_mul(f, a, one);
}
static RF_BENCH_NOINLINE U256x64 from_after(const Fp256Ctx &f, const U256x64 &a) {
    return fp256_from_mont(f, a);
}
static bool below(const U256x64 &a, const U256x64 &b) {
    for (int i = 3; i >= 0; --i)
        if (a.v[i] != b.v[i]) return a.v[i] < b.v[i];
    return false;
}
using Fn = U256x64 (*)(const Fp256Ctx &, const U256x64 &);
static double median_ns(Fn fn, const Fp256Ctx &f, const std::vector<U256x64> &inputs) {
    std::array<double, 9> samples{};
    for (double &ns : samples) {
        uint64_t check = 0;
        const auto start = std::chrono::steady_clock::now();
        for (int rep = 0; rep < 16; ++rep) {
            for (size_t i = 0; i < inputs.size(); ++i) {
                const U256x64 v = fn(f, inputs[i]);
                check ^= v.v[(i + (size_t)rep) & 3];
            }
        }
        const auto stop = std::chrono::steady_clock::now();
        from_sink ^= check;
        ns = std::chrono::duration<double, std::nano>(stop - start).count()
             / (16.0 * inputs.size());
    }
    std::sort(samples.begin(), samples.end());
    return samples[4];
}

int main(int argc, char **argv) {
    const Fp256Ctx f = fp256_ctx_p256();
    std::mt19937_64 rng(0x3ac4a617b9fda10bull);
    const size_t n = 32768;
    std::vector<U256x64> inputs(n);
    for (size_t i = 0; i < n; ++i) {
        do {
            for (int limb = 0; limb < 4; ++limb) inputs[i].v[limb] = rng();
        } while (!below(inputs[i], f.p));
    }
    inputs[0] = {{0, 0, 0, 0}};
    inputs[1] = f.p;
    inputs[1].v[0] -= 1;
    inputs[2] = f.one;
    inputs[3] = {{1, 0, 0, 0}};
    for (size_t i = 0; i < n; ++i) {
        if (!u256_eq(from_before(f, inputs[i]), from_after(f, inputs[i]))) {
            std::fprintf(stderr, "P-256 from_mont mismatch at %zu\n", i);
            return 1;
        }
    }
    if (argc == 2) {
        if (!std::strcmp(argv[1], "--perf-before")) {
            std::printf("%.3f\n", median_ns(from_before, f, inputs)); return 0;
        }
        if (!std::strcmp(argv[1], "--perf-after")) {
            std::printf("%.3f\n", median_ns(from_after, f, inputs)); return 0;
        }
    }
    if (argc != 1) {
        std::fprintf(stderr, "Usage: %s [--perf-before|--perf-after]\n", argv[0]);
        return 2;
    }
    const double before = median_ns(from_before, f, inputs);
    const double after = median_ns(from_after, f, inputs);
    std::printf("P-256 from_mont before %.3f after %.3f ns/op speedup %.3fx\n",
                before, after, before / after);
    return 0;
}
