// CUDA kernels and launcher for the ECCp-131 and ECC2-131 rho walks of gpu_walk.cuh.
//
// On a CUDA device this program:
//   1. builds the same set-up as host_emul.cpp (header arithmetic, deterministic seeds);
//   2. runs the per-thread functions on the CPU to obtain the reference end state;
//   3. runs the kernels and compares the device end state with the CPU state word for word
//      (the device parity gate; any difference exits nonzero);
//   4. times a larger launch with CUDA events and prints the measured rate with the device name.
// Nothing here has been executed on a GPU in the container that wrote it: see the study report.
#include <cuda_runtime.h>

#include <string>

#include "host_common.hpp"

using namespace hc;

__global__ void prime_kernel(PrimeTable T, uint32_t *S, int L, int nthreads, int B, int steps, int *exc) {
    const int tid = blockIdx.x * blockDim.x + threadIdx.x;
    if (tid >= nthreads) return;
    const int e = prime_thread(T, S, L, tid, nthreads, B, steps);
    if (e) atomicAdd(exc, e);
}
__global__ void bin_kernel(const W *tabx, const W *taby, const W *amask, W *X, W *Y, int nthreads, int S, int steps, int *exc) {
    const int tid = blockIdx.x * blockDim.x + threadIdx.x;
    if (tid >= nthreads) return;
    const int e = bin_thread(tabx, taby, amask, X, Y, tid, nthreads, S, steps);
    if (e) atomicAdd(exc, e);
}

static void check(cudaError_t e, const char *what) {
    if (e != cudaSuccess) {
        fprintf(stderr, "%s: %s\n", what, cudaGetErrorString(e));
        exit(2);
    }
}
template <class T>
static T *upload(const std::vector<T> &v) {
    T *d = nullptr;
    check(cudaMalloc(&d, v.size() * sizeof(T)), "cudaMalloc");
    check(cudaMemcpy(d, v.data(), v.size() * sizeof(T), cudaMemcpyHostToDevice), "cudaMemcpy H2D");
    return d;
}
template <class T>
static std::vector<T> download(const T *d, size_t n) {
    std::vector<T> v(n);
    check(cudaMemcpy(v.data(), d, n * sizeof(T), cudaMemcpyDeviceToHost), "cudaMemcpy D2H");
    return v;
}

static int run_prime(const char *dev, int timing_threads, int tsteps_) {
    int rc = 0;
    const int nth = 64, B = 8, LANES = nth * B, steps = 12;
    PrimeSetup su = make_prime(LANES, 0x1318);
    std::vector<uint32_t> ref = prime_state(su.start);
    for (int t = 0; t < nth; ++t) prime_thread(su.tab, ref.data(), LANES, t, nth, B, steps);
    std::vector<uint32_t> S0 = prime_state(su.start);
    uint32_t *dS = upload(S0);
    int *dexc = nullptr, zero = 0;
    check(cudaMalloc(&dexc, 4), "cudaMalloc exc");
    check(cudaMemcpy(dexc, &zero, 4, cudaMemcpyHostToDevice), "memcpy exc");
    prime_kernel<<<(nth + 63) / 64, 64>>>(su.tab, dS, LANES, nth, B, steps, dexc);
    check(cudaGetLastError(), "prime_kernel launch");
    check(cudaDeviceSynchronize(), "prime_kernel run");
    const std::vector<uint32_t> got = download(dS, S0.size());
    int exc = 0;
    check(cudaMemcpy(&exc, dexc, 4, cudaMemcpyDeviceToHost), "memcpy exc back");
    size_t diff = 0;
    for (size_t i = 0; i < got.size(); ++i) diff += got[i] != ref[i];
    printf("{\"kind\":\"device_parity\",\"arm\":\"ECCp-131/u32x5\",\"device\":\"%s\",\"lanes\":%d,\"steps\":%d,\"words_differing\":%zu,\"exceptional\":%d}\n", dev, LANES, steps, diff, exc);
    rc |= diff != 0 || exc != 0;
    cudaFree(dS);
    // timing: independent walks, `timing_threads` threads x B lanes
    const int L = timing_threads * B;
    PrimeSetup big = make_prime(L < 4096 ? L : 4096, 0x1320);
    std::vector<h::FpPoint> pts(L);
    for (int g = 0; g < L; ++g) pts[g] = big.start[g % big.start.size()];
    std::vector<uint32_t> S = prime_state(pts);
    uint32_t *dB = upload(S);
    prime_kernel<<<(timing_threads + 63) / 64, 64>>>(big.tab, dB, L, timing_threads, B, 1, dexc);  // warm-up
    check(cudaDeviceSynchronize(), "warm-up");
    cudaEvent_t e0, e1;
    cudaEventCreate(&e0);
    cudaEventCreate(&e1);
    cudaEventRecord(e0);
    prime_kernel<<<(timing_threads + 63) / 64, 64>>>(big.tab, dB, L, timing_threads, B, tsteps_, dexc);
    cudaEventRecord(e1);
    check(cudaEventSynchronize(e1), "timed run");
    float ms = 0;
    cudaEventElapsedTime(&ms, e0, e1);
    printf("{\"kind\":\"device_rate\",\"arm\":\"ECCp-131/u32x5\",\"device\":\"%s\",\"threads\":%d,\"batch\":%d,\"steps\":%d,\"kernel_ms\":%.3f,\"steps_per_s\":%.0f,\"note\":\"single timed launch, not a campaign rate\"}\n",
           dev, timing_threads, B, tsteps_, ms, (double)L * tsteps_ / (ms * 1e-3));
    cudaFree(dB);
    cudaFree(dexc);
    return rc;
}

static int run_bin(const char *dev, int timing_threads, int tsteps_) {
    int rc = 0;
    const int nth = 8, S = 4, LANES = nth * S * 32, steps = 6;
    BinSetup su = make_bin(LANES, 0x131a);
    std::vector<W> X, Y;
    bin_state(su.start, nth, S, X, Y);
    std::vector<W> RX = X, RY = Y;
    for (int t = 0; t < nth; ++t) bin_thread(su.tabx.data(), su.taby.data(), su.amask.data(), RX.data(), RY.data(), t, nth, S, steps);
    W *dtx = upload(su.tabx), *dty = upload(su.taby), *dam = upload(su.amask), *dX = upload(X), *dY = upload(Y);
    int *dexc = nullptr, zero = 0;
    check(cudaMalloc(&dexc, 4), "cudaMalloc exc");
    check(cudaMemcpy(dexc, &zero, 4, cudaMemcpyHostToDevice), "memcpy exc");
    bin_kernel<<<1, 64>>>(dtx, dty, dam, dX, dY, nth, S, steps, dexc);
    check(cudaGetLastError(), "bin_kernel launch");
    check(cudaDeviceSynchronize(), "bin_kernel run");
    const std::vector<W> gx = download(dX, X.size()), gy = download(dY, Y.size());
    int exc = 0;
    check(cudaMemcpy(&exc, dexc, 4, cudaMemcpyDeviceToHost), "memcpy exc back");
    size_t diff = 0;
    for (size_t i = 0; i < gx.size(); ++i) diff += (gx[i] != RX[i]) + (gy[i] != RY[i]);
    printf("{\"kind\":\"device_parity\",\"arm\":\"ECC2-131/bitsliced32\",\"device\":\"%s\",\"lanes\":%d,\"steps\":%d,\"words_differing\":%zu,\"exceptional\":%d}\n", dev, LANES, steps, diff, exc);
    rc |= diff != 0 || exc != 0;
    cudaFree(dX); cudaFree(dY);
    // timing: `timing_threads` threads x S slots x 32 lanes
    const int L = timing_threads * S * 32;
    BinSetup big = make_bin(1024, 0x1321);
    std::vector<h::F2Point> pts(L);
    for (int q = 0; q < L; ++q) pts[q] = big.start[q % big.start.size()];
    std::vector<W> BX, BY;
    bin_state(pts, timing_threads, S, BX, BY);
    W *dbx = upload(BX), *dby = upload(BY), *dtx2 = upload(big.tabx), *dty2 = upload(big.taby), *dam2 = upload(big.amask);
    bin_kernel<<<(timing_threads + 63) / 64, 64>>>(dtx2, dty2, dam2, dbx, dby, timing_threads, S, 1, dexc);  // warm-up
    check(cudaDeviceSynchronize(), "warm-up");
    cudaEvent_t e0, e1;
    cudaEventCreate(&e0);
    cudaEventCreate(&e1);
    cudaEventRecord(e0);
    bin_kernel<<<(timing_threads + 63) / 64, 64>>>(dtx2, dty2, dam2, dbx, dby, timing_threads, S, tsteps_, dexc);
    cudaEventRecord(e1);
    check(cudaEventSynchronize(e1), "timed run");
    float ms = 0;
    cudaEventElapsedTime(&ms, e0, e1);
    printf("{\"kind\":\"device_rate\",\"arm\":\"ECC2-131/bitsliced32\",\"device\":\"%s\",\"threads\":%d,\"slots\":%d,\"steps\":%d,\"kernel_ms\":%.3f,\"steps_per_s\":%.0f,\"note\":\"single timed launch, not a campaign rate\"}\n",
           dev, timing_threads, S, tsteps_, ms, (double)L * tsteps_ / (ms * 1e-3));
    return rc;
}

int main(int argc, char **argv) {
    int tth = 65536, tsteps = 8;
    for (int i = 1; i < argc; ++i) {
        if (!strcmp(argv[i], "--threads") && i + 1 < argc) tth = atoi(argv[++i]);
        else if (!strcmp(argv[i], "--steps") && i + 1 < argc) tsteps = atoi(argv[++i]);
    }
    int n = 0;
    check(cudaGetDeviceCount(&n), "cudaGetDeviceCount");
    if (n == 0) { fprintf(stderr, "no CUDA device\n"); return 2; }
    cudaDeviceProp prop;
    check(cudaGetDeviceProperties(&prop, 0), "cudaGetDeviceProperties");
    int rc = run_prime(prop.name, tth, tsteps) | run_bin(prop.name, tth / 8, tsteps);
    printf("{\"kind\":\"summary\",\"device\":\"%s\",\"pass\":%s}\n", prop.name, rc ? "false" : "true");
    return rc;
}
