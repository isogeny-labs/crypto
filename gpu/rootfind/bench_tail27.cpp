/* Host-only rootfinder tail benchmark: previous 24/32 cutoff vs 27/32.
 * Uses the repository's actual fp64.cuh field arithmetic; no GPU claim.
 * The prefix/unwind loops mirror rootfind.cuh::rf_decompose_thread.
 * Independent correctness check covers every count 1..32.
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
#define RF_NOINLINE __attribute__((noinline))
#else
#define RF_NOINLINE
#endif

constexpr int CHUNK = 32;
static volatile uint64_t tail27_sink = 0;
using CoreFn = void (*)(const Fp64Ctx &, const uint64_t *, int, uint64_t *);

template <int CUTOFF>
static RF_NOINLINE void core(const Fp64Ctx &f, const uint64_t *xs,
                             int cnt, uint64_t *out) {
    uint64_t d[CHUNK], pre[CHUNK];
    for (int i = 0; i < CHUNK; ++i) d[i] = i < cnt ? xs[i] : f.one;
    const int len = cnt <= CUTOFF ? cnt : CHUNK;
    uint64_t acc = d[0];
    for (int i = 1; i < len; ++i) {
        pre[i] = acc;  // previous prefix, as in rootfind.cuh
        acc = fp64_mul(f, acc, d[i]);
    }
    uint64_t inverse = fp64_inv(f, acc);
    for (int i = len - 1; i > 0; --i) {
        const uint64_t di = d[i];
        d[i] = fp64_mul(f, inverse, pre[i]);
        inverse = fp64_mul(f, inverse, di);
    }
    d[0] = inverse;
    for (int i = 0; i < cnt; ++i) out[i] = d[i];
}

static double median_ns(CoreFn fn, const Fp64Ctx &f,
                        const std::vector<uint64_t> &xs, int cnt) {
    const size_t n = xs.size() / CHUNK;
    std::array<double, 9> samples{};
    for (double &ns : samples) {
        uint64_t sum = 0;
        const auto start = std::chrono::steady_clock::now();
        for (int rep = 0; rep < 8; ++rep) {
            for (size_t j = 0; j < n; ++j) {
                uint64_t out[CHUNK];
                fn(f, &xs[j * CHUNK], cnt, out);
                sum ^= out[(j + rep) % cnt];
            }
        }
        const auto stop = std::chrono::steady_clock::now();
        tail27_sink ^= sum;
        ns = std::chrono::duration<double, std::nano>(stop - start).count() / (8.0 * n);
    }
    std::sort(samples.begin(), samples.end());
    return samples[samples.size() / 2];
}

int main(int argc, char **argv) {
    const Fp64Ctx f = fp64_ctx_new(2305843009213693951ull);
    std::mt19937_64 rng(0x1727B47C11ull);
    const size_t n = 2048;
    std::vector<uint64_t> xs(n * CHUNK);
    for (uint64_t &x : xs) x = fp64_to_mont(f, 1 + rng() % (f.p - 1));

    for (int cnt = 1; cnt <= CHUNK; ++cnt) {
        for (size_t j = 0; j < n; ++j) {
            uint64_t before[CHUNK], after[CHUNK];
            core<24>(f, &xs[j * CHUNK], cnt, before);
            core<27>(f, &xs[j * CHUNK], cnt, after);
            for (int i = 0; i < cnt; ++i) {
                if (before[i] != after[i] ||
                    fp64_mul(f, xs[j * CHUNK + i], after[i]) != f.one) {
                    std::fprintf(stderr, "rootfinder tail mismatch: cnt=%d j=%zu i=%d\n",
                                 cnt, j, i);
                    return 1;
                }
            }
        }
    }
    std::puts("tail27 correctness: PASS (all 32 lengths, 2048 batches per length)");

    if (argc == 2) {
        struct Mode { const char *name; int cnt; CoreFn fn; };
        const Mode modes[] = {
            {"--perf-tail25-before", 25, core<24>},
            {"--perf-tail25-after", 25, core<27>},
            {"--perf-tail26-before", 26, core<24>},
            {"--perf-tail26-after", 26, core<27>},
            {"--perf-tail27-before", 27, core<24>},
            {"--perf-tail27-after", 27, core<27>},
            {"--perf-tail28-before", 28, core<24>},
            {"--perf-tail28-after", 28, core<27>},
            {"--perf-full32-before", 32, core<24>},
            {"--perf-full32-after", 32, core<27>},
        };
        for (const Mode &mode : modes) {
            if (std::strcmp(mode.name, argv[1]) == 0) {
                std::printf("%s %.3f ns/run\n", mode.name, median_ns(mode.fn, f, xs, mode.cnt));
                return 0;
            }
        }
        std::fprintf(stderr, "unknown benchmark arm: %s\n", argv[1]);
        return 2;
    }
    if (argc != 1) return 2;
    for (int cnt : {24, 25, 26, 27, 28, 32}) {
        const double before = median_ns(core<24>, f, xs, cnt);
        const double after = median_ns(core<27>, f, xs, cnt);
        std::printf("tail27 cnt=%d before=%.3f after=%.3f ns/core speedup=%.3fx\n",
                    cnt, before, after, before / after);
    }
    return 0;
}
