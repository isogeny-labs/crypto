/* Microbenchmark the host short-tail inversion-core change in rootfind.cuh.
 * Header fp64.cuh is used directly; 'before' freezes the full CHUNK loop,
 * and 'after' mirrors the host inv_len selection in rf_decompose_thread.
 * Full rootfinder correctness is also covered by make test.
 */
#include "fp64.cuh"
#include <algorithm>
#include <array>
#include <chrono>
#include <cstdio>
#include <cstring>
#include <random>
#include <vector>

#if defined(__GNUC__) || defined(__clang__)
#define RF_NOINLINE __attribute__((noinline))
#else
#define RF_NOINLINE
#endif

static const int CHUNK = 32;
static volatile uint64_t tail_sink = 0;
using CoreFn = void (*)(const Fp64Ctx &, const uint64_t *, int, uint64_t *);

static RF_NOINLINE void core_before(const Fp64Ctx &f, const uint64_t *x, int cnt, uint64_t *out) {
    uint64_t d[CHUNK], pre[CHUNK];
    for (int i = 0; i < CHUNK; ++i) d[i] = i < cnt ? x[i] : f.one;
    uint64_t acc = d[0];
    for (int i = 1; i < CHUNK; ++i) {
        pre[i] = acc;
        acc = fp64_mul(f, acc, d[i]);
    }
    uint64_t inverse = fp64_inv(f, acc);
    for (int i = CHUNK - 1; i > 0; --i) {
        const uint64_t di = d[i];
        d[i] = fp64_mul(f, inverse, pre[i]);
        inverse = fp64_mul(f, inverse, di);
    }
    d[0] = inverse;
    for (int i = 0; i < cnt; ++i) out[i] = d[i];
}

static RF_NOINLINE void core_after(const Fp64Ctx &f, const uint64_t *x, int cnt, uint64_t *out) {
    uint64_t d[CHUNK], pre[CHUNK];
    for (int i = 0; i < CHUNK; ++i) d[i] = i < cnt ? x[i] : f.one;
    const int len = cnt <= CHUNK / 2 ? cnt : CHUNK;
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
        uint64_t check = 0;
        const auto start = std::chrono::steady_clock::now();
        for (int r = 0; r < 4; ++r) {
            for (size_t k = 0; k < n; ++k) {
                uint64_t out[CHUNK];
                fn(f, &xs[k * CHUNK], cnt, out);
                check ^= out[(k + r) % cnt];
            }
        }
        const auto stop = std::chrono::steady_clock::now();
        tail_sink ^= check;
        ns = std::chrono::duration<double, std::nano>(stop - start).count() / (4.0 * n);
    }
    std::sort(samples.begin(), samples.end());
    return samples[samples.size() / 2];
}

int main(int argc, char **argv) {
    const Fp64Ctx f = fp64_ctx_new(2305843009213693951ull);
    std::mt19937_64 rng(0x123987e8b1e5ull);
    const size_t n = 2048;
    std::vector<uint64_t> xs(n * CHUNK);
    for (uint64_t &x : xs) x = fp64_to_mont(f, 1 + rng() % (f.p - 1));
    for (int cnt : {1, 2, 8, 16, 17, 31, 32}) {
        for (size_t k = 0; k < n; ++k) {
            uint64_t oldv[CHUNK], newv[CHUNK];
            core_before(f, &xs[k * CHUNK], cnt, oldv);
            core_after(f, &xs[k * CHUNK], cnt, newv);
            for (int j = 0; j < cnt; ++j) {
                if (oldv[j] != newv[j]) {
                    std::fprintf(stderr, "tail mismatch cnt=%d k=%zu j=%d\n", cnt, k, j);
                    return 1;
                }
            }
        }
    }
    if (argc == 2) {
        const char *arg = argv[1];
        if (!std::strcmp(arg, "--perf-tail8-before")) { std::printf("%.3f\n", median_ns(core_before, f, xs, 8)); return 0; }
        if (!std::strcmp(arg, "--perf-tail8-after"))  { std::printf("%.3f\n", median_ns(core_after, f, xs, 8)); return 0; }
        if (!std::strcmp(arg, "--perf-full32-before")) { std::printf("%.3f\n", median_ns(core_before, f, xs, 32)); return 0; }
        if (!std::strcmp(arg, "--perf-full32-after"))  { std::printf("%.3f\n", median_ns(core_after, f, xs, 32)); return 0; }
    }
    if (argc != 1) {
        std::fprintf(stderr, "Usage: %s [--perf-{tail8,full32}-{before,after}]\n", argv[0]);
        return 2;
    }
    for (int cnt : {1, 8, 16, 32}) {
        const double b = median_ns(core_before, f, xs, cnt);
        const double a = median_ns(core_after, f, xs, cnt);
        std::printf("tail-core cnt=%d before %.3f after %.3f ns/run speedup %.3fx\n", cnt, b, a, b / a);
    }
    return 0;
}
