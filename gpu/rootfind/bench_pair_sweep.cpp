/* CPU microbenchmarks for the pair-table and 2-sign S3 rootfinder sweeps.
 * This compiles the CURRENT rootfind.cuh implementation, compares it against
 * frozen previous formulas, and checks bitwise equality before timing.
 * All inputs are synthetic/public. Neither throughput nor CUDA is measured.
 */
#include "rootfind.cuh"
#include <algorithm>
#include <array>
#include <chrono>
#include <cstdio>
#include <random>
#include <string>
#include <vector>

#define RF_NOINLINE __attribute__((noinline))
static volatile uint64_t sink = 0;

/* Frozen original rf_pair_run before modular-operation sharing. */
static RF_NOINLINE void pair_before(const Fp64Ctx &f, uint64_t a_m,
                                  const uint64_t *x, const uint64_t *y,
                                  uint32_t n, uint32_t ai, uint32_t b0,
                                  uint64_t *keys) {
    typedef FieldFp64 F;
    if (b0 >= n) return;
    const uint32_t m = (n - b0 < RF_PAIR_CHUNK) ? n - b0 : RF_PAIR_CHUNK;
    const uint64_t x1 = x[ai], y1 = y[ai];
    uint64_t d[RF_PAIR_CHUNK], pre[RF_PAIR_CHUNK];
    uint32_t bad = 0;
    uint64_t d0 = (b0 == ai) ? F::add(f, y1, y1) : F::sub(f, x[b0], x1);
    if (d0 == 0) { bad |= 1u; d0 = F::one(f); }
    d[0] = pre[0] = d0;
    uint64_t acc = d0;
    for (uint32_t k = 1; k < m; k++) {
        const uint32_t bi = b0 + k;
        uint64_t dk = F::sub(f, x[bi], x1);
        if (dk == 0) { bad |= 1u << k; dk = F::one(f); }
        d[k] = dk;
        acc = F::mul(f, acc, dk);
        pre[k] = acc;
    }
    uint64_t inv = F::inv(f, acc);
    for (uint32_t k = m; k-- > 1;) {
        const uint64_t dk = d[k];
        d[k] = F::mul(f, inv, pre[k - 1]);
        inv = F::mul(f, inv, dk);
    }
    d[0] = inv;
    const uint64_t base = rf_pair_offset(ai, n);
    for (uint32_t k = 0; k < m; k++) {
        const uint32_t bi = b0 + k;
        const uint64_t b = base + 2ull * (bi - ai);
        if (bad & (1u << k)) {
            for (int sg = 0; sg < 2; sg++) {
                uint64_t xk;
                keys[b + sg] = rf_pair_x(f, a_m, x, y, ai, bi, sg, &xk) ? xk : RF_PAIR_EMPTY;
            }
            continue;
        }
        if (bi == ai) {
            const uint64_t x2 = F::sqr(f, x1);
            const uint64_t num = F::add(f, F::add(f, F::add(f, x2, x2), x2), a_m);
            const uint64_t lam = F::mul(f, num, d[k]);
            keys[b] = F::sub(f, F::sqr(f, lam), F::add(f, x1, x1));
            keys[b + 1] = RF_PAIR_EMPTY;
            continue;
        }
        const uint64_t x2 = x[bi];
        for (int sg = 0; sg < 2; sg++) {
            const uint64_t y2 = sg ? F::sub(f, 0, y[bi]) : y[bi];
            const uint64_t lam = F::mul(f, F::sub(f, y2, y1), d[k]);
            keys[b + sg] = F::sub(f, F::sub(f, F::sqr(f, lam), x1), x2);
        }
    }
}
struct SweepInput { uint64_t xr, xi, yr, yi, di; };
static RF_NOINLINE uint64_t sweep_before(const Fp64Ctx &f, const SweepInput &a) {
    const uint64_t m = fp64_mul(f, fp64_add(f, a.yr, a.yi), a.di);
    const uint64_t p = fp64_mul(f, fp64_sub(f, a.yr, a.yi), a.di);
    const uint64_t x0 = fp64_sub(f, fp64_sub(f, fp64_sqr(f, m), a.xr), a.xi);
    const uint64_t x1 = fp64_sub(f, fp64_sub(f, fp64_sqr(f, p), a.xr), a.xi);
    return x0 ^ x1;
}
static RF_NOINLINE uint64_t sweep_after(const Fp64Ctx &f, const SweepInput &a) {
    const uint64_t m = fp64_mul(f, fp64_add(f, a.yr, a.yi), a.di);
    const uint64_t p = fp64_mul(f, fp64_sub(f, a.yr, a.yi), a.di);
    const uint64_t xsum = fp64_add(f, a.xr, a.xi);
    const uint64_t x0 = fp64_sub(f, fp64_sqr(f, m), xsum);
    const uint64_t x1 = fp64_sub(f, fp64_sqr(f, p), xsum);
    return x0 ^ x1;
}

template<class Fn> static double median_ns(Fn run, int repetitions) {
    std::array<double, 9> times{};
    for (double &t : times) {
        uint64_t check = 0;
        const auto start = std::chrono::steady_clock::now();
        for (int k = 0; k < repetitions; ++k) check ^= run(k);
        const auto stop = std::chrono::steady_clock::now();
        sink = sink ^ check;
        t = std::chrono::duration<double, std::nano>(stop - start).count() / repetitions;
    }
    std::sort(times.begin(), times.end());
    return times[4];
}
int main(int argc, char **argv) {
    const Fp64Ctx f = fp64_ctx_new((1ull << 61) - 1);
    const uint32_t n = 128;
    std::mt19937_64 rng(0x311c0ded);
    std::vector<uint64_t> x(n), y(n);
    for (uint32_t i = 0; i < n; ++i) {
        x[i] = fp64_to_mont(f, i + 1);  // guaranteed distinct x
        y[i] = fp64_to_mont(f, 1 + rng() % (f.p - 1));
    }
    const uint64_t a_m = fp64_to_mont(f, f.p - 3);
    std::vector<uint64_t> k0((size_t)n * (n + 1), 0), k1(k0.size(), 0);
    std::vector<SweepInput> inputs(4096);
    for (auto &v : inputs) {
        v = {fp64_to_mont(f, rng() % f.p), fp64_to_mont(f, rng() % f.p),
             fp64_to_mont(f, rng() % f.p), fp64_to_mont(f, rng() % f.p),
             fp64_to_mont(f, 1 + rng() % (f.p - 1))};
    }
    // Check the entire pair table, including doubling and the short tail.
    for (uint32_t ai = 0; ai < n; ++ai) {
        for (uint32_t b0 = ai; b0 < n; b0 += RF_PAIR_CHUNK) {
            pair_before(f, a_m, x.data(), y.data(), n, ai, b0, k0.data());
            rf_pair_run(f, a_m, x.data(), y.data(), n, ai, b0, k1.data());
        }
    }
    if (k0 != k1) { std::fputs("FAIL: pair-table mismatch\n", stderr); return 2; }
    // Repeated x and y=0 force the old and new zero-denominator fallbacks.
    x[6] = x[5]; y[0] = 0;
    std::fill(k0.begin(), k0.end(), 0);
    std::fill(k1.begin(), k1.end(), 0);
    for (uint32_t ai = 0; ai < n; ++ai)
        for (uint32_t b0 = ai; b0 < n; b0 += RF_PAIR_CHUNK) {
            pair_before(f, a_m, x.data(), y.data(), n, ai, b0, k0.data());
            rf_pair_run(f, a_m, x.data(), y.data(), n, ai, b0, k1.data());
        }
    if (k0 != k1) { std::fputs("FAIL: degenerate pair-table mismatch\n", stderr); return 3; }
    x[6] = fp64_to_mont(f, 7); y[0] = fp64_to_mont(f, 11);
    for (const auto &v : inputs)
        if (sweep_before(f, v) != sweep_after(f, v)) {
            std::fputs("FAIL: x3 mismatch\n", stderr); return 4;
        }
    if (argc == 1) std::puts("PASS: pair table (including degenerates), x3 sweep: bitwise identical");
    const char *mode = argc > 1 ? argv[1] : "";
    auto pb = [&](int k) {
        const uint32_t ai = 4 * (k % 16), b0 = ai + 16;
        pair_before(f, a_m, x.data(), y.data(), n, ai, b0, k0.data());
        return k0[rf_pair_offset(ai, n) + 32];
    };
    auto pa = [&](int k) {
        const uint32_t ai = 4 * (k % 16), b0 = ai + 16;
        rf_pair_run(f, a_m, x.data(), y.data(), n, ai, b0, k1.data());
        return k1[rf_pair_offset(ai, n) + 32];
    };
    auto sb = [&](int k) { return sweep_before(f, inputs[k % inputs.size()]); };
    auto sa = [&](int k) { return sweep_after(f, inputs[k % inputs.size()]); };
    struct Arm { const char *name; double (*fn)(void); };
    const int pairN = 32768, sweepN = 65536;
    if (argc == 1 || std::string(mode) == "--perf-pair-before")
        std::printf("pair16 before: %.3f ns/run\n", median_ns(pb, pairN));
    if (argc == 1 || std::string(mode) == "--perf-pair-after")
        std::printf("pair16 after: %.3f ns/run\n", median_ns(pa, pairN));
    if (argc == 1 || std::string(mode) == "--perf-sweep-before")
        std::printf("x3 two-sign before: %.3f ns/point\n", median_ns(sb, sweepN));
    if (argc == 1 || std::string(mode) == "--perf-sweep-after")
        std::printf("x3 two-sign after: %.3f ns/point\n", median_ns(sa, sweepN));
    return 0;
}
