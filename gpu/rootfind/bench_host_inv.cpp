/* Host-only comparison of the original Fermat inversion / batch path with
 * the current fp64.cuh implementations.  Public synthetic inputs only.
 *
 * make -C gpu/rootfind bench-host-inv
 * taskset -c 0 gpu/rootfind/bench_host_inv
 * perf stat -r 5 -- gpu/rootfind/bench_host_inv   # if perf is available
 *
 * All inversions and batches are checked for bitwise equality before timing.
 * The reported timings are medians of seven sequential, single-threaded runs
 * on the same input stream. No GPU performance inference is made.
 */
#include "fp64.cuh"
#include <algorithm>
#include <chrono>
#include <cstdint>
#include <cstdio>
#include <random>
#include <vector>

static volatile uint64_t sink = 0;

#if defined(__GNUC__) || defined(__clang__)
#define BENCH_NOINLINE __attribute__((noinline))
#else
#define BENCH_NOINLINE
#endif

/* Frozen pre-optimization reference, including the otherwise redundant
 * last square in the Fermat exponentiation. */
static BENCH_NOINLINE uint64_t inv_before(const Fp64Ctx &f, uint64_t a) {
    uint64_t acc = f.one, base = a, e = f.pm2;
    while (e) {
        if (e & 1) acc = fp64_mul(f, acc, base);
        base = fp64_sqr(f, base);
        e >>= 1;
    }
    return acc;
}
static BENCH_NOINLINE uint64_t inv_after(const Fp64Ctx &f, uint64_t a) {
    return fp64_inv(f, a);
}

/* Original batch inversion: n prefix multiplies and 2n unwind multiplies.
 * The new implementation performs 3n-3 multiplies. */
static BENCH_NOINLINE void batch_before(const Fp64Ctx &f, uint64_t *x,
                                       int n, uint64_t *scratch) {
    uint64_t acc = f.one;
    for (int i = 0; i < n; i++) {
        scratch[i] = acc;
        acc = fp64_mul(f, acc, x[i]);
    }
    uint64_t inv_acc = inv_before(f, acc);
    for (int i = n - 1; i >= 0; i--) {
        uint64_t xi = x[i];
        x[i] = fp64_mul(f, inv_acc, scratch[i]);
        inv_acc = fp64_mul(f, inv_acc, xi);
    }
}
static BENCH_NOINLINE void batch_after(const Fp64Ctx &f, uint64_t *x,
                                      int n, uint64_t *scratch) {
    fp64_batch_inv(f, x, n, scratch);
}

template <class Fn>
static double ns_per_inv(Fn fn, const Fp64Ctx &f, const std::vector<uint64_t> &xs) {
    std::vector<double> t;
    for (int repeat = 0; repeat < 7; repeat++) {
        uint64_t check = 0;
        const auto begin = std::chrono::steady_clock::now();
        for (uint64_t x : xs) check ^= fn(f, x);
        const auto end = std::chrono::steady_clock::now();
        sink = check;
        t.push_back(std::chrono::duration<double, std::nano>(end - begin).count() / xs.size());
    }
    std::sort(t.begin(), t.end());
    return t[3];
}

template <class Fn>
static double ns_per_batch(Fn fn, const Fp64Ctx &f, const std::vector<uint64_t> &xs,
                           int n) {
    std::vector<double> t;
    std::vector<uint64_t> x(n), scratch(n);
    const size_t batches = xs.size() / (size_t)n;
    for (int repeat = 0; repeat < 7; repeat++) {
        uint64_t check = 0;
        const auto begin = std::chrono::steady_clock::now();
        for (size_t k = 0; k < batches; k++) {
            std::copy_n(xs.data() + k * n, n, x.data());
            fn(f, x.data(), n, scratch.data());
            check ^= x[k % n];
        }
        const auto end = std::chrono::steady_clock::now();
        sink = check;
        t.push_back(std::chrono::duration<double, std::nano>(end - begin).count() / batches);
    }
    std::sort(t.begin(), t.end());
    return t[3];
}

int main() {
    std::mt19937_64 rng(0x426c);
    const uint64_t primes[] = {1099511627689ull, 72057594037927931ull,
                               2305843009213693951ull, 4611686018427387847ull};
    for (uint64_t p : primes) {
        const Fp64Ctx f = fp64_ctx_new(p);
        std::vector<uint64_t> xs(65536);
        for (auto &x : xs) x = fp64_to_mont(f, 1 + rng() % (p - 1));
        for (uint64_t x : xs) {
            const uint64_t before = inv_before(f, x), after = inv_after(f, x);
            if (before != after || fp64_mul(f, after, x) != f.one) {
                std::fprintf(stderr, "inverse mismatch p=%llu\n", (unsigned long long)p);
                return 1;
            }
        }
        for (int n : {1, 2, 16, 32, 64}) {
            std::vector<uint64_t> a(xs.begin(), xs.begin() + n), b = a, s(n);
            batch_before(f, a.data(), n, s.data());
            batch_after(f, b.data(), n, s.data());
            if (a != b) {
                std::fprintf(stderr, "batch mismatch p=%llu n=%d\n", (unsigned long long)p, n);
                return 1;
            }
        }
        const double base = ns_per_inv(inv_before, f, xs);
        const double opt = ns_per_inv(inv_after, f, xs);
        std::printf("p=%llu inverse old=%.2f ns new=%.2f ns speedup=%.3fx\n",
                    (unsigned long long)p, base, opt, base / opt);
        for (int n : {16, 32, 64}) {
            const double b0 = ns_per_batch(batch_before, f, xs, n);
            const double b1 = ns_per_batch(batch_after, f, xs, n);
            std::printf("  batch n=%d old=%.2f ns new=%.2f ns speedup=%.3fx\n",
                        n, b0, b1, b0 / b1);
        }
    }
    return 0;
}
