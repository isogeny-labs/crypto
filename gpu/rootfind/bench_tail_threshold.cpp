/* Measure only the incremental host-tail cutoff change: <= 16/32 to <= 24/32.
 * This is an inversion-core benchmark, not end-to-end rootfinder throughput.
 * No CUDA path changes. Baseline is frozen from the preceding PR revision.
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

static constexpr int CHUNK = 32;
static volatile uint64_t threshold_sink = 0;
using CoreFn = void (*)(const Fp64Ctx &, const uint64_t *, int, uint64_t *);

/* threshold 16: previous host behavior; threshold 24: candidate behavior.
 * Pad with Montgomery one and leave all unneeded output elements untouched. */
template <int THRESHOLD>
static RF_NOINLINE void inversion_core(const Fp64Ctx &f, const uint64_t *xs,
                                      int cnt, uint64_t *out) {
    uint64_t d[CHUNK], pre[CHUNK];
    for (int i = 0; i < CHUNK; ++i) d[i] = i < cnt ? xs[i] : f.one;
    const int len = cnt <= THRESHOLD ? cnt : CHUNK;
    uint64_t acc = d[0];
    for (int i = 1; i < len; ++i) {
        pre[i] = acc;
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
        for (int rep = 0; rep < 4; ++rep) {
            for (size_t j = 0; j < n; ++j) {
                uint64_t out[CHUNK];
                fn(f, &xs[j * CHUNK], cnt, out);
                sum ^= out[(j + rep) % cnt];
            }
        }
        const auto stop = std::chrono::steady_clock::now();
        threshold_sink ^= sum;
        ns = std::chrono::duration<double, std::nano>(stop - start).count() / (4.0 * n);
    }
    std::sort(samples.begin(), samples.end());
    return samples[samples.size() / 2];
}

int main(int argc, char **argv) {
    const Fp64Ctx f = fp64_ctx_new(2305843009213693951ull);
    std::mt19937_64 rng(0x71A17E57u);
    const size_t n = 2048;
    std::vector<uint64_t> xs(n * CHUNK);
    for (uint64_t &x : xs) x = fp64_to_mont(f, 1 + rng() % (f.p - 1));

    for (int cnt = 1; cnt <= CHUNK; ++cnt) {
        for (size_t j = 0; j < n; ++j) {
            uint64_t b[CHUNK], a[CHUNK];
            inversion_core<CHUNK / 2>(f, &xs[j * CHUNK], cnt, b);
            inversion_core<3 * CHUNK / 4>(f, &xs[j * CHUNK], cnt, a);
            for (int i = 0; i < cnt; ++i) {
                if (a[i] != b[i] || fp64_mul(f, xs[j * CHUNK + i], a[i]) != f.one) {
                    std::fprintf(stderr, "tail threshold mismatch cnt=%d batch=%zu i=%d\n", cnt, j, i);
                    return 1;
                }
            }
        }
    }

    if (argc == 2) {
        struct Mode { const char *name; int cnt; CoreFn fn; };
        const Mode modes[] = {
            {"--perf-tail17-before", 17, inversion_core<16>},
            {"--perf-tail17-after", 17, inversion_core<24>},
            {"--perf-tail20-before", 20, inversion_core<16>},
            {"--perf-tail20-after", 20, inversion_core<24>},
            {"--perf-tail24-before", 24, inversion_core<16>},
            {"--perf-tail24-after", 24, inversion_core<24>},
            {"--perf-tail28-before", 28, inversion_core<16>},
            {"--perf-tail28-after", 28, inversion_core<24>},
            {"--perf-full32-before", 32, inversion_core<16>},
            {"--perf-full32-after", 32, inversion_core<24>},
        };
        for (const auto &mode : modes) {
            if (!std::strcmp(mode.name, argv[1])) {
                std::printf("%s %.3f ns/run\n", mode.name, median_ns(mode.fn, f, xs, mode.cnt));
                return 0;
            }
        }
        std::fprintf(stderr, "Unknown perf arm: %s\n", argv[1]);
        return 2;
    }
    if (argc != 1) return 2;
    for (int cnt : {16, 17, 20, 24, 28, 32}) {
        const double before = median_ns(inversion_core<16>, f, xs, cnt);
        const double after  = median_ns(inversion_core<24>, f, xs, cnt);
        std::printf("tail-threshold cnt=%d before %.3f after %.3f ns/run speedup %.3fx\n",
                    cnt, before, after, before / after);
    }
    return 0;
}
