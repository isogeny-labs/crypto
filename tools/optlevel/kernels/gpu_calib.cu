// gpu_calib: device ceilings for tools/optlevel (`optlevel gpu calibrate`, which builds this file with nvcc at first use).
//
// STATUS: written on a host with no GPU and no nvcc.  First compiled and run
// on 2026-11-17 on a rented RTX 2000 Ada (research/hky_pod_runs_20261117): the
// host re-checks passed, but the driver withheld the performance counters
// (ERR_NVGPUCTRPERM), so the ncu cross-check below has NOT been done.  Compare
// each class with `ncu` (sm__throughput for the arithmetic streams,
// dram__throughput for the triad) and with ecc2k130/benchmarks/roofline/pipes.cu
// before trusting a record scored against it.  The calibration it produces
// carries the flag GPU_CEILING_UNVALIDATED_BY_AUTHOR.
//
// Each arithmetic stream runs CHAINS independent accumulator chains per
// thread, register-resident, with every SM occupied, so the issue rate and
// not the latency is measured.  The host recomputes thread 0's result, so a
// stream the compiler folded away cannot report a rate.  The triad moves
// three device arrays of 256 MiB each; bytes are loads + stores (24 per
// element; a GPU store has no write-allocate read).
//
// Output: one optlevel.ops/v1 JSON line per class.
//   nvcc -O3 -std=c++17 gpu_calib.cu -o gpu_calib && ./gpu_calib
#include <cstring>
#include <chrono>
#include <cstdint>
#include <cstdio>
#include <cstdlib>
#include <vector>

#define CK(x) do { cudaError_t e_ = (x); if (e_ != cudaSuccess) { \
    fprintf(stderr, "CUDA error %s at %s:%d\n", cudaGetErrorString(e_), __FILE__, __LINE__); exit(1); } } while (0)
#define CHAINS 8

enum Cls { MULADD_U32, MULADD_U64, XOR_U64, POPCNT_U64, FMA_F64, NCLS };
static const char *name[NCLS] = {"muladd_u32", "muladd_u64", "xor_u64", "popcnt_u64", "fma_f64"};

template <typename T> __host__ __device__ inline T seed(uint32_t tid, int c) {
    return (T)(0x9E3779B97F4A7C15ull * (uint64_t)(tid + 1) + (uint64_t)c * 0xBF58476D1CE4E5B9ull);
}
// One round = CHAINS operations of class k.  Shared by device and host.
__host__ __device__ inline uint64_t run_chain(int k, uint32_t tid, uint32_t rounds) {
    uint64_t fold = 0;
    if (k == MULADD_U32) {
        uint32_t a[CHAINS];
        for (int c = 0; c < CHAINS; c++) a[c] = seed<uint32_t>(tid, c);
        for (uint32_t r = 0; r < rounds; r++) for (int c = 0; c < CHAINS; c++) a[c] = a[c] * 2654435761u + (uint32_t)(c + r);
        for (int c = 0; c < CHAINS; c++) fold += a[c];
    } else if (k == MULADD_U64) {
        uint64_t a[CHAINS];
        for (int c = 0; c < CHAINS; c++) a[c] = seed<uint64_t>(tid, c);
        for (uint32_t r = 0; r < rounds; r++) for (int c = 0; c < CHAINS; c++) a[c] = a[c] * 0x9E3779B97F4A7C15ull + (uint64_t)(c + r);
        for (int c = 0; c < CHAINS; c++) fold += a[c];
    } else if (k == XOR_U64) {
        uint64_t a[CHAINS], b = seed<uint64_t>(tid, 99);
        for (int c = 0; c < CHAINS; c++) a[c] = seed<uint64_t>(tid, c);
        for (uint32_t r = 0; r < rounds; r++) { b = (b << 1) | (b >> 63); for (int c = 0; c < CHAINS; c++) a[c] ^= b + (uint64_t)c; }
        for (int c = 0; c < CHAINS; c++) fold += a[c];
    } else if (k == POPCNT_U64) {
        uint64_t a[CHAINS], s[CHAINS];
        for (int c = 0; c < CHAINS; c++) { a[c] = seed<uint64_t>(tid, c); s[c] = 0; }
        for (uint32_t r = 0; r < rounds; r++) for (int c = 0; c < CHAINS; c++) {
#ifdef __CUDA_ARCH__
            s[c] += (uint64_t)__popcll(a[c] + s[c]);
#else
            s[c] += (uint64_t)__builtin_popcountll(a[c] + s[c]);
#endif
        }
        for (int c = 0; c < CHAINS; c++) fold += s[c];
    } else {
        double a[CHAINS];
        for (int c = 0; c < CHAINS; c++) a[c] = (double)(seed<uint32_t>(tid, c) >> 8);
#ifdef __CUDA_ARCH__
        for (uint32_t r = 0; r < rounds; r++) for (int c = 0; c < CHAINS; c++) a[c] = fma(a[c], 0.5, (double)(c + 1));
#else
        for (uint32_t r = 0; r < rounds; r++) for (int c = 0; c < CHAINS; c++) a[c] = __builtin_fma(a[c], 0.5, (double)(c + 1));
#endif
        for (int c = 0; c < CHAINS; c++) fold += (uint64_t)a[c];
    }
    return fold;
}

__global__ void stream(int k, uint32_t rounds, uint64_t *out) {
    uint32_t tid = blockIdx.x * blockDim.x + threadIdx.x;
    out[tid] = run_chain(k, tid, rounds);
}
__global__ void triad(double *a, const double *b, const double *c, double s, size_t n) {
    size_t i = (size_t)blockIdx.x * blockDim.x + threadIdx.x;
    if (i < n) a[i] = b[i] + s * c[i];
}
__global__ void fill(double *b, double *c, size_t n) {
    size_t i = (size_t)blockIdx.x * blockDim.x + threadIdx.x;
    if (i < n) { b[i] = (double)(i & 1023); c[i] = 1.0; }
}
static double now() {
    return std::chrono::duration<double>(std::chrono::steady_clock::now().time_since_epoch()).count();
}

int main(int argc, char **argv) {
    // Optional argument: run one operation class only (a name from name[] or "triad"), so that a
    // single kernel can be measured and profiled as a positive control.
    const char *only = argc > 1 ? argv[1] : nullptr;
    cudaDeviceProp prop;
    CK(cudaGetDeviceProperties(&prop, 0));
    const int block = 256;
    const int grid = prop.multiProcessorCount * 8;        // 2048 threads per SM: full occupancy at 8 chains of registers
    const uint32_t threads = (uint32_t)grid * block;
    uint64_t *out;
    CK(cudaMalloc(&out, sizeof(uint64_t) * threads));
    for (int k = 0; k < NCLS; k++) {
        if (only && strcmp(only, name[k]) != 0) continue;
        uint32_t rounds = 1u << 12;
        double secs = 0;
        for (;;) {                                           // grow until one launch lasts at least 0.5 s
            stream<<<grid, block>>>(k, rounds, out);         // warm-up at this size
            CK(cudaDeviceSynchronize());
            double t0 = now();
            stream<<<grid, block>>>(k, rounds, out);
            CK(cudaDeviceSynchronize());
            secs = now() - t0;
            if (secs >= 0.5 || rounds >= (1u << 30)) break;
            rounds *= 2;
        }
        uint64_t got;
        CK(cudaMemcpy(&got, out, sizeof got, cudaMemcpyDeviceToHost));
        if (got != run_chain(k, 0, rounds)) { fprintf(stderr, "gpu_calib: %s failed the host re-check\n", name[k]); return 1; }
        printf("{\"schema\":\"optlevel.ops/v1\",\"op_class\":\"%s\",\"ops\":%llu,\"kernel_seconds\":%.9f,"
               "\"bytes_declared\":0,\"threads\":%u,\"device\":\"%s\"}\n",
               name[k], (unsigned long long)threads * rounds * CHAINS, secs, threads, prop.name);
    }
    if (only && strcmp(only, "triad") != 0) return 0;
    const size_t n = (size_t)32 << 20;                       // 256 MiB per array
    double *a, *b, *c;
    CK(cudaMalloc(&a, 8 * n)); CK(cudaMalloc(&b, 8 * n)); CK(cudaMalloc(&c, 8 * n));
    const int tgrid = (int)((n + block - 1) / block);
    fill<<<tgrid, block>>>(b, c, n);
    triad<<<tgrid, block>>>(a, b, c, 3.0, n);                // warm-up
    CK(cudaDeviceSynchronize());
    int reps = 0;
    double t0 = now(), secs = 0;
    do { triad<<<tgrid, block>>>(a, b, c, 3.0, n); CK(cudaDeviceSynchronize()); reps++; secs = now() - t0; } while (secs < 0.5);
    double probe[2];
    CK(cudaMemcpy(probe, a + 1022, sizeof probe, cudaMemcpyDeviceToHost));
    if (probe[0] != 1022.0 + 3.0 || probe[1] != 1023.0 + 3.0) { fprintf(stderr, "gpu_calib: triad failed the host re-check\n"); return 1; }
    printf("{\"schema\":\"optlevel.ops/v1\",\"op_class\":\"triad\",\"ops\":%llu,\"kernel_seconds\":%.9f,\"bytes_declared\":%.0f,\"device\":\"%s\"}\n",
           (unsigned long long)n * reps, secs, 24.0 * (double)n * reps, prop.name);
    return 0;
}
