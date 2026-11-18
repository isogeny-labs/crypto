/* P-256 sparse-modulus Montgomery reduction, host only.
 * Frozen pre-change reduction uses the current u64_mac (native u128 MAC);
 * this isolates the new modulus specialization from previous MAC changes.
 * All benchmark operands and comparisons are deterministic/public.
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
static volatile uint64_t sink_sparse256 = 0;

static RF_BENCH_NOINLINE U256x64 before_sparse(const Fp256Ctx &f,
                                               const U256x64 &a, const U256x64 &b) {
    uint64_t t[9] = {};
    for (int i = 0; i < 4; i++) {
        uint64_t carry = 0;
        for (int j = 0; j < 4; j++) {
            uint64_t lo, hi;
            u64_mac(t[i + j], a.v[j], b.v[i], carry, lo, hi);
            t[i + j] = lo;
            carry = hi;
        }
        t[i + 4] = carry;
    }
    for (int i = 0; i < 4; i++) {
        const uint64_t m = t[i] * f.pinv;
        uint64_t carry = 0;
        for (int j = 0; j < 4; j++) {
            uint64_t lo, hi;
            u64_mac(t[i + j], m, f.p.v[j], carry, lo, hi);
            t[i + j] = lo;
            carry = hi;
        }
        for (int k = i + 4; k < 9; k++) {
            uint64_t s = t[k] + carry;
            carry = s < t[k];
            t[k] = s;
        }
    }
    U256x64 r, s;
    for (int i = 0; i < 4; i++) r.v[i] = t[i + 4];
    const uint64_t borrow = u256_sbb(s, r, f.p);
    u256_cmov(r, s, (t[8] & 1) | (borrow ^ 1));
    return r;
}

static RF_BENCH_NOINLINE U256x64 after_sparse(const Fp256Ctx &f,
                                              const U256x64 &a, const U256x64 &b) {
    return fp256_mul(f, a, b);
}

using Fn = U256x64(*)(const Fp256Ctx &, const U256x64 &, const U256x64 &);
static double ns_per_op(Fn fn, const Fp256Ctx &f,
                        const std::vector<U256x64> &aa,
                        const std::vector<U256x64> &bb) {
    std::array<double, 9> samples{};
    for (double &ns : samples) {
        uint64_t checksum = 0;
        const auto start = std::chrono::steady_clock::now();
        for (int round = 0; round < 16; round++)
            for (size_t i = 0; i < aa.size(); ++i)
                checksum ^= fn(f, aa[i], bb[i]).v[round & 3];
        const auto stop = std::chrono::steady_clock::now();
        sink_sparse256 ^= checksum;
        ns = std::chrono::duration<double, std::nano>(stop - start).count()
             / (16.0 * aa.size());
    }
    std::sort(samples.begin(), samples.end());
    return samples[samples.size() / 2];
}

int main(int argc, char **argv) {
    const Fp256Ctx f = fp256_ctx_p256();
    std::mt19937_64 rng(0x20261117u);
    std::vector<U256x64> aa(32768), bb(32768);
    for (size_t i = 0; i < aa.size(); ++i) {
        for (int j = 0; j < 4; j++) {
            aa[i].v[j] = rng();
            bb[i].v[j] = rng();
        }
        aa[i].v[3] &= 0x7fffffffffffffffull;
        bb[i].v[3] &= 0x7fffffffffffffffull;
    }
    aa[0] = {{0, 0, 0, 0}};
    bb[0] = {{0, 0, 0, 0}};
    aa[1] = f.one;
    bb[1] = f.one;
    aa[2] = f.p;
    bb[2] = f.p;
    --aa[2].v[0];
    --bb[2].v[0];
    aa[3] = f.p;
    bb[3] = f.one;
    --aa[3].v[0];

    /* Verify *all* inputs before either perf-only arm, so failures cannot
     * be hidden by the benchmark CLI mode. */
    for (size_t i = 0; i < aa.size(); ++i) {
        if (!u256_eq(before_sparse(f, aa[i], bb[i]),
                     after_sparse(f, aa[i], bb[i]))) {
            std::fprintf(stderr, "sparse P-256 reduction mismatch at %zu\n", i);
            return 1;
        }
    }

    if (argc == 2) {
        if (!std::strcmp(argv[1], "--perf-before")) {
            std::printf("before %.3f ns/op\n", ns_per_op(before_sparse, f, aa, bb));
            return 0;
        }
        if (!std::strcmp(argv[1], "--perf-after")) {
            std::printf("after %.3f ns/op\n", ns_per_op(after_sparse, f, aa, bb));
            return 0;
        }
    }
    if (argc != 1) {
        std::fprintf(stderr, "Usage: %s [--perf-before|--perf-after]\n", argv[0]);
        return 2;
    }
    const double old_ns = ns_per_op(before_sparse, f, aa, bb);
    const double new_ns = ns_per_op(after_sparse, f, aa, bb);
    std::printf("P-256 sparse REDC: before %.3f ns after %.3f ns speedup %.3fx (%zu checked)\n",
                old_ns, new_ns, old_ns / new_ns, aa.size());
    return 0;
}
