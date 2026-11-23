/* CPU-only P-256 host inversion before/after benchmark. Public inputs.
 *
 * make -C gpu/rootfind bench-host-inv256
 * taskset -c <cpu> gpu/rootfind/bench_host_inv256
 * perf stat -r 5 -e cycles,instructions,branches,branch-misses \
 *   -- gpu/rootfind/bench_host_inv256
 *
 * Both algorithms use the same fp256x64.cuh Montgomery multiplication.
 * The bitwise reference is the original implementation. GPU performance
 * is not measured by this program.
 */
#include "fp256x64.cuh"
#include <algorithm>
#include <chrono>
#include <cstdint>
#include <cstdio>
#include <cstring>
#include <random>
#include <vector>

static volatile uint64_t bench_sink256 = 0;
#if defined(__GNUC__) || defined(__clang__)
#define RF_BENCH_NOINLINE __attribute__((noinline))
#else
#define RF_BENCH_NOINLINE
#endif

static RF_BENCH_NOINLINE U256x64 inv_before256(const Fp256Ctx &f,
                                              const U256x64 &a) {
    U256x64 e = f.p;
    e.v[0] -= 2;
    U256x64 acc = f.one;
    for (int i = 255; i >= 0; i--) {
        acc = fp256_sqr(f, acc);
        if ((e.v[i >> 6] >> (i & 63)) & 1) acc = fp256_mul(f, acc, a);
    }
    return acc;
}
static RF_BENCH_NOINLINE U256x64 inv_after256(const Fp256Ctx &f,
                                             const U256x64 &a) {
    return fp256_inv(f, a);
}

static RF_BENCH_NOINLINE void batch_before256(const Fp256Ctx &f,
                                U256x64 *x, int n, U256x64 *scratch) {
    U256x64 acc = f.one;
    for (int i = 0; i < n; i++) {
        scratch[i] = acc;
        acc = fp256_mul(f, acc, x[i]);
    }
    U256x64 inv_acc = inv_before256(f, acc);
    for (int i = n - 1; i >= 0; i--) {
        U256x64 xi = x[i];
        x[i] = fp256_mul(f, inv_acc, scratch[i]);
        inv_acc = fp256_mul(f, inv_acc, xi);
    }
}
static RF_BENCH_NOINLINE void batch_after256(const Fp256Ctx &f,
                               U256x64 *x, int n, U256x64 *scratch) {
    fp256_batch_inv(f, x, n, scratch);
}

template<class Fn>
static double inversion_ns(Fn fn, const Fp256Ctx &f,
                           const std::vector<U256x64> &xs) {
    std::vector<double> times;
    for (int rep = 0; rep < 9; rep++) {
        uint64_t sum = 0;
        auto start = std::chrono::steady_clock::now();
        for (const auto &x : xs) sum += fn(f, x).v[0];
        auto stop = std::chrono::steady_clock::now();
        bench_sink256 = sum;
        times.push_back(std::chrono::duration<double, std::nano>(stop-start).count()/xs.size());
    }
    std::sort(times.begin(), times.end());
    return times[4];
}
template<class Fn>
static double batch_ns(Fn fn, const Fp256Ctx &f,
                       const std::vector<U256x64> &xs, int n) {
    std::vector<double> times;
    std::vector<U256x64> a(n), scratch(n);
    const size_t count = xs.size() / (size_t)n;
    for (int rep = 0; rep < 9; rep++) {
        uint64_t sum = 0;
        auto start = std::chrono::steady_clock::now();
        for (size_t k = 0; k < count; k++) {
            std::copy_n(xs.data() + k*n, n, a.data());
            fn(f, a.data(), n, scratch.data());
            sum += a[0].v[0];
        }
        auto stop = std::chrono::steady_clock::now();
        bench_sink256 = sum;
        times.push_back(std::chrono::duration<double, std::nano>(stop-start).count()/count);
    }
    std::sort(times.begin(), times.end());
    return times[4];
}

int main(int argc, char **argv) {
    const Fp256Ctx f = fp256_ctx_p256();
    std::mt19937_64 rng(0x25620261117ull);
    std::vector<U256x64> xs(8192);
    for (auto &x : xs) {
        /* Top limb below 2^62 => strictly below P-256. */
        U256x64 canonical = {{rng(), rng(), rng(), rng() & ((1ull << 62)-1)}};
        x = fp256_to_mont(f, canonical);
    }
    xs[0] = f.one;
    /* Perf mode runs only one arm on the identical deterministic corpus.
     * This avoids counting two algorithms together in perf stat. */
    if (argc == 2) {
        const char *mode = argv[1];
        if (std::strcmp(mode, "--perf-inv-before") == 0)
            std::printf("P256 before inverse %.1f ns/op\n", inversion_ns(inv_before256, f, xs));
        else if (std::strcmp(mode, "--perf-inv-after") == 0)
            std::printf("P256 after inverse %.1f ns/op\n", inversion_ns(inv_after256, f, xs));
        else if (std::strcmp(mode, "--perf-batch32-before") == 0)
            std::printf("P256 before batch32 %.1f ns/op\n", batch_ns(batch_before256, f, xs, 32));
        else if (std::strcmp(mode, "--perf-batch32-after") == 0)
            std::printf("P256 after batch32 %.1f ns/op\n", batch_ns(batch_after256, f, xs, 32));
        else { std::fprintf(stderr, "Unknown perf mode: %s\n", mode); return 2; }
        return 0;
    }
    if (argc != 1) { std::fprintf(stderr, "Usage: %s [--perf-inv-before|--perf-inv-after|--perf-batch32-before|--perf-batch32-after]\n", argv[0]); return 2; }
    for (const auto &x : xs) {
        const U256x64 a = inv_before256(f, x), b = inv_after256(f, x);
        if (!u256_eq(a, b) || !u256_eq(fp256_mul(f, x, b), f.one)) {
            std::fprintf(stderr, "P256 inverse mismatch\n"); return 1;
        }
    }
    const U256x64 zero = {{0, 0, 0, 0}};
    if (!u256_eq(inv_before256(f, zero), inv_after256(f, zero))) return 1;
    for (int n : {1,2,16,32,64}) {
        std::vector<U256x64> a(xs.begin(), xs.begin()+n), b = a, scratch(n);
        batch_before256(f, a.data(), n, scratch.data());
        batch_after256(f, b.data(), n, scratch.data());
        for (int i = 0; i < n; i++) {
            if (!u256_eq(a[i], b[i])) {
                std::fprintf(stderr, "P256 batch mismatch n=%d i=%d\n", n, i);
                return 1;
            }
        }
    }
    const double old_ns = inversion_ns(inv_before256, f, xs);
    const double new_ns = inversion_ns(inv_after256, f, xs);
    std::printf("P256 inverse old=%.1f ns new=%.1f ns speedup=%.3fx\n",
                old_ns, new_ns, old_ns/new_ns);
    for (int n : {1,2,16,32,64}) {
        const double old_b = batch_ns(batch_before256, f, xs, n);
        const double new_b = batch_ns(batch_after256, f, xs, n);
        std::printf("P256 batch n=%d old=%.1f ns new=%.1f ns speedup=%.3fx\n",
                    n, old_b, new_b, old_b/new_b);
    }
    return 0;
}
