/* bench.cu -- device driver for the batched Macaulay reduction.
 *
 *   ./bench selftest     every kernel against the host reference
 *   ./bench throughput   matrices per second, swept over batch size
 *   ./bench occupancy    launch configuration report
 *
 * `selftest` is the one that matters and the one to run first on real
 * hardware.  The CPU harness (`make test`) verifies `mac_rref_serial`
 * against `mref.py`; this verifies `mac_rref_block` against
 * `mac_rref_serial` *on the device*, which is the only gap the CPU
 * harness structurally cannot close -- the block kernel does not exist
 * outside `__CUDACC__`.
 *
 * NOTE: this file has never been run.  The environment it was written in
 * had no NVIDIA device.  Treat every number it would print as unmeasured
 * until it has been.
 */
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <vector>

#include "params.h"
#include "macaulay.cuh"
#include "vectors.h"

#ifndef MAC_THREADS
#  define MAC_THREADS 256
#endif

#define CUDA_OK(call)                                                        \
    do {                                                                     \
        cudaError_t e_ = (call);                                             \
        if (e_ != cudaSuccess) {                                             \
            printf("CUDA error %s at %s:%d\n", cudaGetErrorString(e_),       \
                   __FILE__, __LINE__);                                      \
            exit(1);                                                         \
        }                                                                    \
    } while (0)

/* One block per matrix. */
__global__ void rref_batch_kernel(uint32_t* a, int rows, int cols, int* piv,
                                  int* rank) {
    extern __shared__ uint32_t smem[];
    const int inst = blockIdx.x;
    uint32_t* mine = a + (size_t)inst * rows * cols;
    int* mypiv = piv + (size_t)inst * rows;
    int r = mac_rref_block<MAC_THREADS>(mine, rows, cols, mypiv, smem);
    if (threadIdx.x == 0) rank[inst] = r;
}

/* One block per matrix, the matrix staged in shared memory as uint16_t
 * (p < 2^16).  The reduction is the same mac_rref_block, so the result is
 * the same matrix; the global matrix is read once and written once
 * instead of once per pivot column. */
template <int THREADS>
__global__ void rref_batch_kernel_smem(uint32_t* a, int rows, int cols,
                                       int* piv, int* rank) {
    extern __shared__ uint32_t smem[];
    const int inst = blockIdx.x;
    const size_t n = (size_t)rows * cols;
    uint32_t* mine = a + (size_t)inst * n;
    uint32_t* scratch = smem;
    uint16_t* sa = (uint16_t*)(smem + mac_block_scratch_words(cols));
    for (size_t i = threadIdx.x; i < n; i += THREADS) sa[i] = (uint16_t)mine[i];
    __syncthreads();
    int r = mac_rref_block<THREADS, uint16_t>(sa, rows, cols,
                                              piv + (size_t)inst * rows, scratch);
    __syncthreads();
    for (size_t i = threadIdx.x; i < n; i += THREADS) mine[i] = sa[i];
    if (threadIdx.x == 0) rank[inst] = r;
}

static size_t smem16_bytes() {
    return mac_block_scratch_words(MAC_COLS) * sizeof(uint32_t) +
           (size_t)MAC_ROWS * MAC_COLS * sizeof(uint16_t);
}

/* Which reduction kernel runs: the shared-resident one when p < 2^16 and
 * the device's opt-in shared memory per block holds the matrix (sm_80:
 * 163 KiB, sm_90: 227 KiB; sm_86/sm_89: 99 KiB does not hold 226x286),
 * else the global one.  MAC_SMEM=0 forces global; MAC_SMEM_THREADS picks
 * 256, 512 or 1024 (default) threads for the shared kernel.  A100, batch
 * 8192, 226x286: 718 ms global, 417 / 253 / 197 ms shared at 256 / 512 /
 * 1024 threads (research/ic_gpu_measure_20261112/REPORT.md). */
static int smem_threads() {
    static int t = -1;
    if (t >= 0) return t;
    t = 0;
    const char* e = getenv("MAC_SMEM");
    if (e && !strcmp(e, "0")) return t;
    if (MAC_P >= 65536u) return t;
    int dev = 0, optin = 0;
    CUDA_OK(cudaGetDevice(&dev));
    CUDA_OK(cudaDeviceGetAttribute(&optin, cudaDevAttrMaxSharedMemoryPerBlockOptin, dev));
    if ((size_t)optin < smem16_bytes()) return t;
    const char* te = getenv("MAC_SMEM_THREADS");
    int want = te ? atoi(te) : 1024;
    if (want != 256 && want != 512) want = 1024;
    CUDA_OK(cudaFuncSetAttribute(rref_batch_kernel_smem<256>, cudaFuncAttributeMaxDynamicSharedMemorySize, (int)smem16_bytes()));
    CUDA_OK(cudaFuncSetAttribute(rref_batch_kernel_smem<512>, cudaFuncAttributeMaxDynamicSharedMemorySize, (int)smem16_bytes()));
    CUDA_OK(cudaFuncSetAttribute(rref_batch_kernel_smem<1024>, cudaFuncAttributeMaxDynamicSharedMemorySize, (int)smem16_bytes()));
    t = want;
    return t;
}

static const char* rref_path() {
    static char buf[64];
    int t = smem_threads();
    if (t) snprintf(buf, sizeof buf, "shared-u16 (%d threads, %zu B)", t, smem16_bytes());
    else snprintf(buf, sizeof buf, "global (%d threads)", MAC_THREADS);
    return buf;
}

static size_t shared_bytes();
static void launch_rref(uint32_t* d_a, int batch, int rows, int cols, int* d_piv,
                        int* d_rank) {
    switch (smem_threads()) {
    case 256: rref_batch_kernel_smem<256><<<batch, 256, smem16_bytes()>>>(d_a, rows, cols, d_piv, d_rank); break;
    case 512: rref_batch_kernel_smem<512><<<batch, 512, smem16_bytes()>>>(d_a, rows, cols, d_piv, d_rank); break;
    case 1024: rref_batch_kernel_smem<1024><<<batch, 1024, smem16_bytes()>>>(d_a, rows, cols, d_piv, d_rank); break;
    default: rref_batch_kernel<<<batch, MAC_THREADS, shared_bytes()>>>(d_a, rows, cols, d_piv, d_rank);
    }
}

/* Montgomery conversion, flat over the whole batch. */
__global__ void to_mont_kernel(uint32_t* a, size_t n) {
    for (size_t i = (size_t)blockIdx.x * blockDim.x + threadIdx.x; i < n;
         i += (size_t)gridDim.x * blockDim.x) {
        a[i] = fp_to_mont(a[i]);
    }
}
__global__ void from_mont_kernel(uint32_t* a, size_t n) {
    for (size_t i = (size_t)blockIdx.x * blockDim.x + threadIdx.x; i < n;
         i += (size_t)gridDim.x * blockDim.x) {
        a[i] = fp_from_mont(a[i]);
    }
}

static size_t shared_bytes() {
    return mac_block_scratch_words(MAC_COLS) * sizeof(uint32_t);
}

/* Build `batch` instances on the host, reduce them on the device, and
 * compare against the host reference instance by instance. */
static int selftest(int batch) {
    const int rows = MAC_ROWS, cols = MAC_COLS;
    const size_t sz = (size_t)rows * cols;
    printf("selftest: p=%u shape=%dx%d batch=%d threads=%d path=%s\n", MAC_P, rows, cols,
           batch, MAC_THREADS, rref_path());

    std::vector<uint32_t> host(sz * batch);
    for (int b = 0; b < batch; b++) {
        mac_gen_matrix(host.data() + sz * b, rows, cols,
                       BIG_SEED + 1000ull * b, BIG_DEFICIT);
    }

    /* Host reference: the function `make test` verified against mref.py. */
    std::vector<uint64_t> want_dig(batch);
    std::vector<int> want_rank(batch);
    std::vector<std::vector<int>> want_piv(batch);
    for (int b = 0; b < batch; b++) {
        std::vector<uint32_t> one(host.begin() + sz * b, host.begin() + sz * (b + 1));
        mac_to_mont(one.data(), sz);
        want_piv[b].assign(rows, -1);
        want_rank[b] = mac_rref_serial(one.data(), rows, cols, want_piv[b].data());
        mac_from_mont(one.data(), sz);
        want_dig[b] = mac_digest(one.data(), sz);
    }

    uint32_t* d_a = nullptr;
    int *d_piv = nullptr, *d_rank = nullptr;
    CUDA_OK(cudaMalloc(&d_a, sz * batch * sizeof(uint32_t)));
    CUDA_OK(cudaMalloc(&d_piv, (size_t)rows * batch * sizeof(int)));
    CUDA_OK(cudaMalloc(&d_rank, (size_t)batch * sizeof(int)));
    CUDA_OK(cudaMemcpy(d_a, host.data(), sz * batch * sizeof(uint32_t),
                       cudaMemcpyHostToDevice));
    CUDA_OK(cudaMemset(d_piv, 0xFF, (size_t)rows * batch * sizeof(int)));

    to_mont_kernel<<<256, 256>>>(d_a, sz * batch);
    launch_rref(d_a, batch, rows, cols, d_piv, d_rank);
    from_mont_kernel<<<256, 256>>>(d_a, sz * batch);
    CUDA_OK(cudaGetLastError());
    CUDA_OK(cudaDeviceSynchronize());

    std::vector<uint32_t> got(sz * batch);
    std::vector<int> got_piv((size_t)rows * batch), got_rank(batch);
    CUDA_OK(cudaMemcpy(got.data(), d_a, sz * batch * sizeof(uint32_t),
                       cudaMemcpyDeviceToHost));
    CUDA_OK(cudaMemcpy(got_piv.data(), d_piv, (size_t)rows * batch * sizeof(int),
                       cudaMemcpyDeviceToHost));
    CUDA_OK(cudaMemcpy(got_rank.data(), d_rank, (size_t)batch * sizeof(int),
                       cudaMemcpyDeviceToHost));

    int bad = 0;
    for (int b = 0; b < batch; b++) {
        if (got_rank[b] != want_rank[b]) {
            printf("  instance %d: rank %d, host says %d\n", b, got_rank[b],
                   want_rank[b]);
            bad++;
            continue;
        }
        if (mac_digest(got.data() + sz * b, sz) != want_dig[b]) {
            printf("  instance %d: reduced form differs from the host\n", b);
            bad++;
            continue;
        }
        for (int i = 0; i < want_rank[b]; i++) {
            if (got_piv[(size_t)b * rows + i] != want_piv[b][i]) {
                printf("  instance %d: pivot %d is %d, host says %d\n", b, i,
                       got_piv[(size_t)b * rows + i], want_piv[b][i]);
                bad++;
                break;
            }
        }
    }
    /* The batch must not be accidentally uniform, or this proves little. */
    if (batch > 1 && want_dig[0] == want_dig[1]) {
        printf("  instances 0 and 1 are identical: the comparison is vacuous\n");
        bad++;
    }
    /* And the shape must be the rank-deficient one the real matrices are. */
    if (want_rank[0] >= rows) {
        printf("  warning: instance 0 has full rank; the deficient path is untested\n");
    }

    cudaFree(d_a);
    cudaFree(d_piv);
    cudaFree(d_rank);
    printf("%s (%d of %d instances differ)\n", bad ? "SELFTEST FAILED" : "selftest ok",
           bad, batch);
    return bad ? 1 : 0;
}

static void throughput() {
    const int rows = MAC_ROWS, cols = MAC_COLS;
    const size_t sz = (size_t)rows * cols;
    printf("throughput: p=%u shape=%dx%d threads=%d path=%s\n", MAC_P, rows, cols,
           MAC_THREADS, rref_path());
    printf("  %8s %12s %14s %12s\n", "batch", "ms", "matrices/s", "MB");
    for (int batch = 64; batch <= 8192; batch *= 2) {
        std::vector<uint32_t> host(sz * batch);
        for (int b = 0; b < batch; b++) {
            mac_gen_matrix(host.data() + sz * b, rows, cols,
                           BIG_SEED + 1000ull * b, BIG_DEFICIT);
        }
        uint32_t* d_a = nullptr;
        int *d_piv = nullptr, *d_rank = nullptr;
        if (cudaMalloc(&d_a, sz * batch * sizeof(uint32_t)) != cudaSuccess) {
            printf("  %8d   (out of memory)\n", batch);
            break;
        }
        CUDA_OK(cudaMalloc(&d_piv, (size_t)rows * batch * sizeof(int)));
        CUDA_OK(cudaMalloc(&d_rank, (size_t)batch * sizeof(int)));
        cudaEvent_t t0, t1;
        CUDA_OK(cudaEventCreate(&t0));
        CUDA_OK(cudaEventCreate(&t1));
        /* One warm-up launch, then the timed one.
         *
         * The re-upload inside the loop is not optional.  Both kernels
         * work **in place**: after the warm-up, `d_a` holds the reduced
         * form in Montgomery representation, so a second pass would
         * Montgomery-convert an already-converted matrix and then
         * eliminate a matrix that is already in echelon form -- far
         * less work than the real reduction, and the timing would be
         * of that instead.  Restoring the input each rep is what makes
         * the timed launch measure the same thing the warm-up did.
         *
         * `cudaMemcpy` on the default stream is synchronous, and the
         * copy sits before `cudaEventRecord`, so only the kernels are
         * timed. */
        for (int rep = 0; rep < 2; rep++) {
            CUDA_OK(cudaMemcpy(d_a, host.data(), sz * batch * sizeof(uint32_t),
                               cudaMemcpyHostToDevice));
            if (rep == 1) CUDA_OK(cudaEventRecord(t0));
            to_mont_kernel<<<256, 256>>>(d_a, sz * batch);
            launch_rref(d_a, batch, rows, cols, d_piv, d_rank);
            if (rep == 1) CUDA_OK(cudaEventRecord(t1));
            CUDA_OK(cudaDeviceSynchronize());
        }
        float ms = 0;
        CUDA_OK(cudaEventElapsedTime(&ms, t0, t1));
        printf("  %8d %12.2f %14.0f %12.1f\n", batch, ms,
               batch / (ms / 1000.0), sz * batch * 4.0 / (1 << 20));
        cudaEventDestroy(t0);
        cudaEventDestroy(t1);
        cudaFree(d_a);
        cudaFree(d_piv);
        cudaFree(d_rank);
    }
    printf("\nCompare against the CPU: research/notes/index-calculus/RESEARCH_RESIDUAL_WALKS.md section 11.6\n");
    printf("measures one residual at 0.88e6 F_p multiplications, of which the\n");
    printf("Macaulay step is 57%%.  Convert with the residual rate of the run\n");
    printf("being compared; do not quote matrices/s on its own.\n");
}

static void occupancy() {
    int dev = 0;
    cudaDeviceProp prop{};
    CUDA_OK(cudaGetDevice(&dev));
    CUDA_OK(cudaGetDeviceProperties(&prop, dev));
    printf("device: %s, sm_%d%d, %d SMs\n", prop.name, prop.major, prop.minor,
           prop.multiProcessorCount);
    printf("shared per block requested: %zu bytes (max %zu)\n", shared_bytes(),
           (size_t)prop.sharedMemPerBlock);
    int blocks = 0;
    CUDA_OK(cudaOccupancyMaxActiveBlocksPerMultiprocessor(
        &blocks, rref_batch_kernel, MAC_THREADS, shared_bytes()));
    printf("blocks per SM at %d threads: %d (%d resident matrices)\n", MAC_THREADS,
           blocks, blocks * prop.multiProcessorCount);
    printf("matrix bytes: %zu as uint32 (global path), %zu with the shared-u16\n",
           (size_t)MAC_ROWS * MAC_COLS * sizeof(uint32_t), smem16_bytes());
    printf("path's scratch; opt-in shared per block %zu.  Reduction path: %s\n",
           (size_t)prop.sharedMemPerBlockOptin, rref_path());
}

int main(int argc, char** argv) {
    const char* cmd = argc > 1 ? argv[1] : "selftest";
    if (!strcmp(cmd, "selftest")) return selftest(argc > 2 ? atoi(argv[2]) : 32);
    if (!strcmp(cmd, "throughput")) { throughput(); return 0; }
    if (!strcmp(cmd, "occupancy")) { occupancy(); return 0; }
    printf("usage: %s [selftest [batch] | throughput | occupancy]\n", argv[0]);
    return 2;
}
