// Carry-less unit issue-pattern probe (follow-up to benchmarks/clmad-price).
//
// benchmarks/clmad-price measured a stream of CLMAD.lo at 1.69 lane-CLMADs per
// SM-clock on the RTX PRO 6000 and a stream of lo+hi pairs at 1.99, while the
// 20 B/s kernel runs the unit at 1.62 (ONE-BLOCK-GEOMETRY.md section 1).  The
// pair stream differs from the lo stream in two ways at once: the hi follows
// the lo immediately, and it reads the same operands.  This probe separates
// them, so the floor the kernel is priced against can be stated for the issue
// pattern the kernel actually produces:
//
//   lo            one clmad.lo per chain, self-chained (the clmad-price stream)
//   hi            one clmad.hi per chain
//   pair-same     clmad.lo then clmad.hi of the same operands, adjacent
//   pair-other    clmad.lo of (a,b) then clmad.hi of (b,a): adjacent, operands swapped
//   lo-lo-same    two clmad.lo of the same operands, adjacent
//   flat          3 lo then 3 hi of three operand pairs (the PACKED_CLMUL_FLAT order)
//   karat         3 (lo,hi) pairs of three operand pairs (the shipping clmul64 order)
//   pair-far      lo of (a,b), then four LOP3, then hi of (a,b)
//
// Every stream is counted in lane-CLMADs issued (LOP3 filler is not counted),
// so the number to compare across rows is lane-CLMADs per SM-clock.
#include <cstdint>
#include <cstdio>
#include <cstdlib>
#include <vector>
#include <algorithm>
#include <cuda_runtime.h>

#define CUDA_CHECK(x) do { cudaError_t e = (x); if (e != cudaSuccess) { \
    fprintf(stderr, "%s:%d: %s\n", __FILE__, __LINE__, cudaGetErrorString(e)); exit(1); } } while (0)

enum Op { OP_LO = 0, OP_HI, OP_PAIR_SAME, OP_PAIR_OTHER, OP_LOLO_SAME, OP_FLAT, OP_KARAT, OP_PAIR_FAR,
          OP_LO_ACC, OP_PAIR_ACC, OP_RUN_FRESH, OP_PAIRACC_ADJ, OP_RUN_ACC_INDEP,
          OP_HI_ACC, OP_HI_ADD_OTHER, OP_LO_ADD_OTHER, OP_PAIR_HIADD,
          OP_PAIR_ZREG, OP_KARAT_ZREG, OP_PAIR_ZEROOP, OP_PAIR_ACC_LIVE,
          OP_PAIR_SELF, OP_KARAT_SELF, OP_COUNT };
static const char *opName[OP_COUNT] = {"lo", "hi", "pair-same", "pair-other", "lo-lo-same",
                                       "flat 3lo+3hi", "karat 3x(lo,hi)", "pair-far (4 lop3 between)",
                                       "lo-acc (clmad-price form)", "pair-acc (clmad-price form)",
                                       "16 lo then 16 hi, fresh dest", "pair-acc forced adjacent",
                                       "16 lo-acc then 16 hi-acc, hi reads old a",
                                       "hi-acc alone (addend = dest)", "hi, addend another register",
                                       "lo, addend another register", "lo fresh + hi with addend",
                                       "pair, addend = zero register (kernel arg)", "karat 3x(lo,hi), addends = zero register",
                                       "pair, fresh dest, multiplicand = 0 (per-lane register)",
                                       "pair-acc, operands kept nonzero every round",
                                       "pair, self-addend on copies, corrected", "karat 3x(lo,hi), self-addend on copies, corrected"};
// lane-CLMADs issued per chain per round
static const int opClmads[OP_COUNT] = {1, 1, 2, 2, 2, 6, 6, 2, 1, 2, 2, 2, 2, 1, 1, 1, 2, 2, 6, 2, 2, 2, 6};
static const int CHAINS = 16;

template <int OP>
__global__ void __launch_bounds__(256) chains(uint64_t *out, uint32_t rounds, uint64_t z) {
    uint64_t a[CHAINS], b[CHAINS], c[CHAINS];
#pragma unroll
    for (int j = 0; j < CHAINS; ++j) {
        a[j] = 0x9e3779b97f4a7c15ull * (threadIdx.x + j + 1);
        b[j] = 0xc2b2ae3d27d4eb4full * (blockIdx.x + j + 1) | 1ull;
        c[j] = a[j] ^ b[j];
    }
#pragma unroll 1
    for (uint32_t i = 0; i < rounds; ++i) {
#pragma unroll
        for (int j = 0; j < CHAINS; ++j) {
            if (OP == OP_LO)
                asm volatile("clmad.lo.u64 %0, %1, %2, 0;" : "=l"(a[j]) : "l"(a[j]), "l"(b[j]));
            if (OP == OP_HI)
                asm volatile("clmad.hi.u64 %0, %1, %2, 0;" : "=l"(a[j]) : "l"(a[j]), "l"(b[j]));
            if (OP == OP_PAIR_SAME) {
                uint64_t lo, hi;
                asm volatile("clmad.lo.u64 %0, %2, %3, 0;\n\tclmad.hi.u64 %1, %2, %3, 0;"
                             : "=l"(lo), "=l"(hi) : "l"(a[j]), "l"(b[j]));
                a[j] = lo; b[j] = hi | 1ull;
            }
            if (OP == OP_PAIR_OTHER) {
                uint64_t lo, hi;
                asm volatile("clmad.lo.u64 %0, %2, %3, 0;\n\tclmad.hi.u64 %1, %3, %2, 0;"
                             : "=l"(lo), "=l"(hi) : "l"(a[j]), "l"(b[j] ^ c[j]));
                a[j] = lo; b[j] = hi | 1ull;
            }
            if (OP == OP_LOLO_SAME) {
                uint64_t lo, lo2;
                asm volatile("clmad.lo.u64 %0, %2, %3, 0;\n\tclmad.lo.u64 %1, %2, %3, 1;"
                             : "=l"(lo), "=l"(lo2) : "l"(a[j]), "l"(b[j]));
                a[j] = lo; b[j] = lo2 | 1ull;
            }
            if (OP == OP_FLAT) {
                const uint64_t a0 = a[j], a1 = b[j], as = a0 ^ a1;
                const uint64_t b0 = c[j], b1 = a0 ^ c[j], bs = b0 ^ b1;
                uint64_t l0, l1, lm, h0, h1, hm;
                asm volatile("clmad.lo.u64 %0, %6, %7, 0;\n\tclmad.lo.u64 %1, %8, %9, 0;\n\tclmad.lo.u64 %2, %10, %11, 0;\n\t"
                             "clmad.hi.u64 %3, %6, %7, 0;\n\tclmad.hi.u64 %4, %8, %9, 0;\n\tclmad.hi.u64 %5, %10, %11, 0;"
                             : "=l"(l0), "=l"(l1), "=l"(lm), "=l"(h0), "=l"(h1), "=l"(hm)
                             : "l"(a0), "l"(b0), "l"(a1), "l"(b1), "l"(as), "l"(bs));
                a[j] = l0 ^ h1; b[j] = (l1 ^ hm) | 1ull; c[j] = lm ^ h0;
            }
            if (OP == OP_KARAT) {
                const uint64_t a0 = a[j], a1 = b[j], as = a0 ^ a1;
                const uint64_t b0 = c[j], b1 = a0 ^ c[j], bs = b0 ^ b1;
                uint64_t l0, l1, lm, h0, h1, hm;
                asm volatile("clmad.lo.u64 %0, %6, %7, 0;\n\tclmad.hi.u64 %3, %6, %7, 0;\n\t"
                             "clmad.lo.u64 %1, %8, %9, 0;\n\tclmad.hi.u64 %4, %8, %9, 0;\n\t"
                             "clmad.lo.u64 %2, %10, %11, 0;\n\tclmad.hi.u64 %5, %10, %11, 0;"
                             : "=l"(l0), "=l"(l1), "=l"(lm), "=l"(h0), "=l"(h1), "=l"(hm)
                             : "l"(a0), "l"(b0), "l"(a1), "l"(b1), "l"(as), "l"(bs));
                a[j] = l0 ^ h1; b[j] = (l1 ^ hm) | 1ull; c[j] = lm ^ h0;
            }
            // The two streams of benchmarks/clmad-price/probe.cu, verbatim: the
            // destination is also the addend, and hi reads the lo just written.
            if (OP == OP_LO_ACC || OP == OP_PAIR_ACC)
                asm volatile("clmad.lo.u64 %0, %1, %2, %0;" : "+l"(a[j]) : "l"(a[j]), "l"(b[j]));
            if (OP == OP_PAIR_ACC)
                asm volatile("clmad.hi.u64 %0, %1, %2, %0;" : "+l"(b[j]) : "l"(a[j]), "l"(b[j]));
            if (OP == OP_PAIRACC_ADJ)
                asm volatile("clmad.lo.u64 %0, %0, %1, %0;\n\tclmad.hi.u64 %1, %0, %1, %1;"
                             : "+l"(a[j]), "+l"(b[j]));
            if (OP == OP_HI_ACC)
                asm volatile("clmad.hi.u64 %0, %0, %1, %0;" : "+l"(a[j]) : "l"(b[j]));
            if (OP == OP_HI_ADD_OTHER)
                asm volatile("clmad.hi.u64 %0, %0, %1, %2;" : "+l"(a[j]) : "l"(b[j]), "l"(c[j]));
            if (OP == OP_LO_ADD_OTHER)
                asm volatile("clmad.lo.u64 %0, %0, %1, %2;" : "+l"(a[j]) : "l"(b[j]), "l"(c[j]));
            if (OP == OP_PAIR_HIADD) {
                uint64_t lo, hi;
                asm volatile("clmad.lo.u64 %0, %2, %3, 0;\n\tclmad.hi.u64 %1, %2, %3, %4;"
                             : "=l"(lo), "=l"(hi) : "l"(a[j]), "l"(b[j]), "l"(c[j]));
                a[j] = lo; b[j] = hi | 1ull;
            }
            // z is a kernel argument the host passes as 0: ptxas cannot fold it to RZ.
            if (OP == OP_PAIR_ZREG) {
                uint64_t lo, hi;
                asm volatile("clmad.lo.u64 %0, %2, %3, %4;\n\tclmad.hi.u64 %1, %2, %3, %4;"
                             : "=l"(lo), "=l"(hi) : "l"(a[j]), "l"(b[j]), "l"(z));
                a[j] = lo; b[j] = hi | 1ull;
            }
            if (OP == OP_KARAT_ZREG) {
                const uint64_t a0 = a[j], a1 = b[j], as = a0 ^ a1;
                const uint64_t b0 = c[j], b1 = a0 ^ c[j], bs = b0 ^ b1;
                uint64_t l0, l1, lm, h0, h1, hm;
                asm volatile("clmad.lo.u64 %0, %6, %7, %12;\n\tclmad.hi.u64 %3, %6, %7, %12;\n\t"
                             "clmad.lo.u64 %1, %8, %9, %12;\n\tclmad.hi.u64 %4, %8, %9, %12;\n\t"
                             "clmad.lo.u64 %2, %10, %11, %12;\n\tclmad.hi.u64 %5, %10, %11, %12;"
                             : "=l"(l0), "=l"(l1), "=l"(lm), "=l"(h0), "=l"(h1), "=l"(hm)
                             : "l"(a0), "l"(b0), "l"(a1), "l"(b1), "l"(as), "l"(bs), "l"(z));
                a[j] = l0 ^ h1; b[j] = (l1 ^ hm) | 1ull; c[j] = lm ^ h0;
            }
            // Data dependence: the accumulate recurrences above collapse a to 0
            // within ~80 rounds.  These two streams separate "zero operand" from
            // "accumulate form".  z is 0 at run time, so a & z is a per-lane zero.
            if (OP == OP_PAIR_ZEROOP) {
                uint64_t lo, hi; const uint64_t zero = a[j] & z;
                asm volatile("clmad.lo.u64 %0, %2, %3, 0;\n\tclmad.hi.u64 %1, %2, %3, 0;"
                             : "=l"(lo), "=l"(hi) : "l"(zero), "l"(b[j]));
                a[j] ^= lo; b[j] = (b[j] ^ hi) | 1ull;
            }
            if (OP == OP_PAIR_ACC_LIVE) {
                asm volatile("clmad.lo.u64 %0, %0, %1, %0;" : "+l"(a[j]) : "l"(b[j]));
                asm volatile("clmad.hi.u64 %0, %1, %0, %0;" : "+l"(b[j]) : "l"(a[j]));
                // keep both operands nonzero and changing: two ALU ops, ~0.05 CLMAD slots
                a[j] = (a[j] ^ (b[j] >> 7)) | 0x8000000000000001ull;
                b[j] |= 1ull;
            }
            // The product-usable form: clmad r, r, y, r on a copy r = x gives
            // lo/hi(x*y) ^ x; one XOR on the logic pipe cancels the x.
            if (OP == OP_PAIR_SELF) {
                uint64_t lo = a[j], hi = a[j];
                asm volatile("clmad.lo.u64 %0, %0, %2, %0;\n\tclmad.hi.u64 %1, %1, %2, %1;"
                             : "+l"(lo), "+l"(hi) : "l"(b[j]));
                lo ^= a[j]; hi ^= a[j];
                a[j] = lo | 0x8000000000000000ull; b[j] = hi | 1ull;
            }
            if (OP == OP_KARAT_SELF) {
                const uint64_t a0 = a[j], a1 = b[j], as = a0 ^ a1;
                const uint64_t b0 = c[j], b1 = a0 ^ c[j], bs = b0 ^ b1;
                uint64_t l0 = a0, h0 = a0, l1 = a1, h1 = a1, lm = as, hm = as;
                asm volatile("clmad.lo.u64 %0, %0, %6, %0;\n\tclmad.hi.u64 %3, %3, %6, %3;\n\t"
                             "clmad.lo.u64 %1, %1, %7, %1;\n\tclmad.hi.u64 %4, %4, %7, %4;\n\t"
                             "clmad.lo.u64 %2, %2, %8, %2;\n\tclmad.hi.u64 %5, %5, %8, %5;"
                             : "+l"(l0), "+l"(l1), "+l"(lm), "+l"(h0), "+l"(h1), "+l"(hm)
                             : "l"(b0), "l"(b1), "l"(bs));
                l0 ^= a0; h0 ^= a0; l1 ^= a1; h1 ^= a1; lm ^= as; hm ^= as;
                a[j] = (l0 ^ h1) | 0x8000000000000000ull; b[j] = (l1 ^ hm) | 1ull; c[j] = (lm ^ h0) | 2ull;
            }
            if (OP == OP_PAIR_FAR) {
                uint64_t lo, hi; uint32_t t = uint32_t(c[j]);
                asm volatile("clmad.lo.u64 %0, %3, %4, 0;\n\t"
                             "lop3.b32 %2, %2, %5, %6, 0x96;\n\tlop3.b32 %2, %2, %5, %6, 0x96;\n\t"
                             "lop3.b32 %2, %2, %5, %6, 0x96;\n\tlop3.b32 %2, %2, %5, %6, 0x96;\n\t"
                             "clmad.hi.u64 %1, %3, %4, 0;"
                             : "=l"(lo), "=l"(hi), "+r"(t) : "l"(a[j]), "l"(b[j]), "r"(uint32_t(a[j])), "r"(uint32_t(b[j] >> 32)));
                a[j] = lo; b[j] = hi | 1ull; c[j] = (c[j] & ~0xffffffffull) | t;
            }
        }
        if (OP == OP_RUN_FRESH) {
            uint64_t lo[CHAINS];
#pragma unroll
            for (int j = 0; j < CHAINS; ++j)
                asm volatile("clmad.lo.u64 %0, %1, %2, 0;" : "=l"(lo[j]) : "l"(a[j]), "l"(b[j]));
#pragma unroll
            for (int j = 0; j < CHAINS; ++j) {
                uint64_t hi;
                asm volatile("clmad.hi.u64 %0, %1, %2, 0;" : "=l"(hi) : "l"(a[j]), "l"(b[j]));
                a[j] = lo[j]; b[j] = hi | 1ull;
            }
        }
        if (OP == OP_RUN_ACC_INDEP) {
            uint64_t old[CHAINS];
#pragma unroll
            for (int j = 0; j < CHAINS; ++j) {
                old[j] = a[j];
                asm volatile("clmad.lo.u64 %0, %0, %1, %0;" : "+l"(a[j]) : "l"(b[j]));
            }
#pragma unroll
            for (int j = 0; j < CHAINS; ++j)
                asm volatile("clmad.hi.u64 %0, %1, %0, %0;" : "+l"(b[j]) : "l"(old[j]));
        }
    }
    uint64_t acc = 0;
#pragma unroll
    for (int j = 0; j < CHAINS; ++j) acc ^= a[j] ^ b[j] ^ c[j];
    out[blockIdx.x * blockDim.x + threadIdx.x] = acc;
}

static void clmulHost(uint64_t x, uint64_t y, uint64_t *lo, uint64_t *hi) {
    uint64_t l = 0, h = 0;
    for (int i = 0; i < 64; ++i)
        if ((y >> i) & 1u) { l ^= x << i; if (i) h ^= x >> (64 - i); }
    *lo = l; *hi = h;
}
__global__ void clmadOnce(uint64_t *out, uint64_t x, uint64_t y) {
    uint64_t lo, hi;
    asm("clmad.lo.u64 %0, %1, %2, 0;" : "=l"(lo) : "l"(x), "l"(y));
    asm("clmad.hi.u64 %0, %1, %2, 0;" : "=l"(hi) : "l"(x), "l"(y));
    out[0] = lo; out[1] = hi;
}

template <int OP>
static double measure(uint64_t *out, int blocks, int threads, uint32_t rounds) {
    chains<OP><<<blocks, threads>>>(out, 64, 0ull);
    CUDA_CHECK(cudaDeviceSynchronize());
    cudaEvent_t a, b;
    CUDA_CHECK(cudaEventCreate(&a)); CUDA_CHECK(cudaEventCreate(&b));
    CUDA_CHECK(cudaEventRecord(a));
    chains<OP><<<blocks, threads>>>(out, rounds, 0ull);
    CUDA_CHECK(cudaEventRecord(b));
    CUDA_CHECK(cudaEventSynchronize(b));
    float ms = 0;
    CUDA_CHECK(cudaEventElapsedTime(&ms, a, b));
    CUDA_CHECK(cudaEventDestroy(a)); CUDA_CHECK(cudaEventDestroy(b));
    return double(blocks) * threads * CHAINS * rounds * opClmads[OP] / (ms * 1e-3);
}

int main(int argc, char **argv) {
    const uint32_t rounds = argc > 1 ? uint32_t(atoi(argv[1])) : 4000u;
    const int passes = argc > 2 ? atoi(argv[2]) : 3;
    const int blocksPerSm = argc > 3 ? atoi(argv[3]) : 8;
    cudaDeviceProp prop;
    CUDA_CHECK(cudaGetDeviceProperties(&prop, 0));
    const int blocks = prop.multiProcessorCount * blocksPerSm, threads = 256;
    int clockKHz = 0;
    CUDA_CHECK(cudaDeviceGetAttribute(&clockKHz, cudaDevAttrClockRate, 0));
    printf("{\"gpu\":\"%s\",\"sms\":%d,\"clockMaxKHz\":%d,\"blocks\":%d,\"threads\":%d,\"chains\":%d,\"rounds\":%u}\n",
           prop.name, prop.multiProcessorCount, clockKHz, blocks, threads, CHAINS, rounds);
    uint64_t *out = NULL;
    CUDA_CHECK(cudaMalloc(&out, sizeof(uint64_t) * size_t(blocks) * threads));
    const uint64_t x = 0x0123456789abcdefull, y = 0xfedcba9876543210ull;
    uint64_t want[2], got[2];
    clmulHost(x, y, &want[0], &want[1]);
    clmadOnce<<<1, 1>>>(out, x, y);
    CUDA_CHECK(cudaMemcpy(got, out, sizeof got, cudaMemcpyDeviceToHost));
    if (got[0] != want[0] || got[1] != want[1]) { printf("clmad disagrees with the host reference\n"); return 1; }
    printf("clmad verified against host carryless multiply\n");
    std::vector<double> best(OP_COUNT, 0.0);
    for (int pass = 0; pass < passes; ++pass) {
        double r[OP_COUNT];
        r[OP_LO] = measure<OP_LO>(out, blocks, threads, rounds);
        r[OP_HI] = measure<OP_HI>(out, blocks, threads, rounds);
        r[OP_PAIR_SAME] = measure<OP_PAIR_SAME>(out, blocks, threads, rounds);
        r[OP_PAIR_OTHER] = measure<OP_PAIR_OTHER>(out, blocks, threads, rounds);
        r[OP_LOLO_SAME] = measure<OP_LOLO_SAME>(out, blocks, threads, rounds);
        r[OP_FLAT] = measure<OP_FLAT>(out, blocks, threads, rounds);
        r[OP_KARAT] = measure<OP_KARAT>(out, blocks, threads, rounds);
        r[OP_PAIR_FAR] = measure<OP_PAIR_FAR>(out, blocks, threads, rounds);
        r[OP_LO_ACC] = measure<OP_LO_ACC>(out, blocks, threads, rounds);
        r[OP_PAIR_ACC] = measure<OP_PAIR_ACC>(out, blocks, threads, rounds);
        r[OP_RUN_FRESH] = measure<OP_RUN_FRESH>(out, blocks, threads, rounds);
        r[OP_PAIRACC_ADJ] = measure<OP_PAIRACC_ADJ>(out, blocks, threads, rounds);
        r[OP_RUN_ACC_INDEP] = measure<OP_RUN_ACC_INDEP>(out, blocks, threads, rounds);
        r[OP_HI_ACC] = measure<OP_HI_ACC>(out, blocks, threads, rounds);
        r[OP_HI_ADD_OTHER] = measure<OP_HI_ADD_OTHER>(out, blocks, threads, rounds);
        r[OP_LO_ADD_OTHER] = measure<OP_LO_ADD_OTHER>(out, blocks, threads, rounds);
        r[OP_PAIR_HIADD] = measure<OP_PAIR_HIADD>(out, blocks, threads, rounds);
        r[OP_PAIR_ZREG] = measure<OP_PAIR_ZREG>(out, blocks, threads, rounds);
        r[OP_KARAT_ZREG] = measure<OP_KARAT_ZREG>(out, blocks, threads, rounds);
        r[OP_PAIR_ZEROOP] = measure<OP_PAIR_ZEROOP>(out, blocks, threads, rounds);
        r[OP_PAIR_ACC_LIVE] = measure<OP_PAIR_ACC_LIVE>(out, blocks, threads, rounds);
        r[OP_PAIR_SELF] = measure<OP_PAIR_SELF>(out, blocks, threads, rounds);
        r[OP_KARAT_SELF] = measure<OP_KARAT_SELF>(out, blocks, threads, rounds);
        for (int op = 0; op < OP_COUNT; ++op) best[op] = std::max(best[op], r[op]);
    }
    // lane-CLMADs per SM-clock at the device's maximum clock (the sustained
    // clock is printed by the job after each pass for the correction)
    const double smClocksPerSecond = double(prop.multiProcessorCount) * clockKHz * 1e3;
    for (int op = 0; op < OP_COUNT; ++op)
        printf("{\"stream\":\"%s\",\"laneClmadsPerSecond\":%.6e,\"laneClmadsPerSmClockAtMax\":%.4f,\"vsLo\":%.4f}\n",
               opName[op], best[op], best[op] / smClocksPerSecond, best[op] / best[OP_LO]);
    CUDA_CHECK(cudaFree(out));
    return 0;
}
