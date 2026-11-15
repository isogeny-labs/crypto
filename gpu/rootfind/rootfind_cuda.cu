/* rootfind_cuda.cu -- CUDA backend for the root-finding 2-decomposition
 * oracle.  The kernels are thin wrappers around rf_decompose_thread
 * (rootfind.cuh), which is the function the host emulation runs; this
 * file adds only the launch structure, device memory, and the
 * shared-memory staging of the hash table.
 *
 * Target: NVIDIA RTX PRO 6000 Blackwell (sm_120, compute capability 12.0,
 * CUDA 12.8+).  Build (see Makefile `lib-cuda` / `bench`):
 *
 *   nvcc -O3 -std=c++17 -lineinfo \
 *        -gencode arch=compute_120,code=sm_120 \
 *        -gencode arch=compute_120,code=compute_120 \
 *        -c rootfind_cuda.cu
 *
 * The second -gencode embeds PTX so the binary JIT-compiles on later
 * architectures (and on sm_100/sm_90 builds the PTX line is swapped for
 * compute_100 / compute_90).  Add `-Xptxas -v` to read the per-kernel
 * register/spill counts the tuning notes below refer to.
 *
 * THIS FILE HAS NOT BEEN EXECUTED ON A GPU.  The machine it was written
 * on has no NVIDIA device and no nvcc.  Everything below the launch line
 * is the host-verified rootfind.cuh; the launch structure, memory
 * layout and shared-memory staging are untested.  Run `./bench selftest`
 * first on real hardware: it compares every kernel configuration against
 * the host emulation on the same inputs.
 *
 * Blackwell tuning notes (sm_120)
 * ------------------------------
 * Block size.  RF_BLOCK = 128 (4 warps).  The thread -> (target, slice)
 *   map in rootfind.cuh puts consecutive targets in consecutive lanes,
 *   so a block of 128 reads 128 consecutive (x_R, y_R) pairs: two fully
 *   coalesced 1 KiB segments for Fp64, 4 KiB for P-256.
 * Registers.  The per-thread batched inversion holds 2*CHUNK field
 *   elements (d[] and the prefix products).  Audit note (2026-11-12,
 *   clang 18 + ptxas 12.9, no GPU): the sweep indexes d[w] with a
 *   run-time w, so d[] is placed in local memory, not registers: stack
 *   frames are 240/368/624 bytes (Fp64, CHUNK 16/32/64) and
 *   1296/2320/4368 bytes (P-256, where pre[] is local too); see
 *   docs/perf/20261112-ic-gpu-audit.md.  For Fp64 that is 4*CHUNK
 *   32-bit registers: CHUNK=32 -> ~128 + ~40 for temporaries, i.e. about
 *   170 registers and 12 warps/SM out of 48 on sm_120 (64K regs per SM).
 *   CHUNK=16 roughly doubles occupancy at the cost of twice the Fermat
 *   inversions per target (~95 muls each).  For P-256 (16 regs per
 *   element) CHUNK=16 is already at the 255-register ceiling and CHUNK=32
 *   spills to local memory; the P-256 kernels therefore default to 16.
 *   __launch_bounds__(RF_BLOCK, RF_MIN_BLOCKS) lets ptxas trade spills
 *   for occupancy; RF_MIN_BLOCKS=0 leaves it free.  Measure both.
 * 64-bit multiply.  There is no 64-bit integer multiplier; `a*b` lowers
 *   to 3 IMAD.WIDE.U32 and `__umul64hi` to 4 plus carries, so one Fp64
 *   Montgomery multiplication is ~14 IMAD-class instructions.  The IMAD
 *   pipe on sm_120 issues at half the FP32 rate (64 lanes/SM/clk), which
 *   is the basis for the throughput estimate in the research note.
 * Memory.  Factor base is SoA (x[] and y[] separate) and read with
 *   warp-uniform indices inside the slice loop: each load is one 8-byte
 *   (Fp64) or 32-byte (P-256) broadcast per warp, served from L1 after
 *   the first touch.  Targets are read once per thread into registers.
 * Lookup.  Small factor bases (hash capacity <= RF_SHARED_HASH_CAP =
 *   4096 slots, i.e. |F| <= 2048) stage the open-addressing table in
 *   dynamic shared memory sized to it (12 bytes per slot, at most
 *   48 KiB); probes are then ~1-2 shared loads
 *   with no global traffic.  Larger factor bases use the global sorted
 *   array + binary search: log2(N) dependent 8-byte loads per probe,
 *   L2-resident for N up to a few million.  A global hash table would
 *   cut that to ~2 loads and is the obvious next step for N > 2048.
 * Atomics.  Hits are rare (a random target hits with probability about
 *   2N^2/p), so the single global atomicAdd on the output counter is not
 *   a contention point.
 */
#include <cuda_runtime.h>
#include <cstdio>
#include <vector>

#include "rootfind_host.hpp"

#ifndef RF_BLOCK
#define RF_BLOCK 128
#endif
#ifndef RF_MIN_BLOCKS
#define RF_MIN_BLOCKS 0
#endif
#if RF_MIN_BLOCKS
#define RF_BOUNDS __launch_bounds__(RF_BLOCK, RF_MIN_BLOCKS)
#else
#define RF_BOUNDS __launch_bounds__(RF_BLOCK)
#endif

/* `rc` and the `cleanup` label live in run_device.  Every failure jumps
 * there so device buffers and events allocated earlier are released. */
#define CU(call)                                                                   \
    do {                                                                           \
        cudaError_t e_ = (call);                                                   \
        if (e_ != cudaSuccess) {                                                   \
            std::fprintf(stderr, "CUDA error %s at %s:%d: %s\n", #call, __FILE__,  \
                         __LINE__, cudaGetErrorString(e_));                        \
            rc = RF_ERR_CUDA;                                                      \
            goto cleanup;                                                          \
        }                                                                          \
    } while (0)

/* Publish the pointer only after a successful alloc, so cleanup never
 * frees a handle a failed call may have written. */
template <class T>
static cudaError_t rf_cu_alloc(T **dst, size_t bytes) {
    void *p = nullptr;
    cudaError_t e = cudaMalloc(&p, bytes);
    if (e == cudaSuccess) *dst = static_cast<T *>(p);
    return e;
}

static cudaError_t rf_cu_event(cudaEvent_t *dst) {
    cudaEvent_t ev = nullptr;
    cudaError_t e = cudaEventCreate(&ev);
    if (e == cudaSuccess) *dst = ev;
    return e;
}

__device__ __forceinline__ uint64_t rf_global_tid() {
    return (uint64_t)blockIdx.x * (uint64_t)blockDim.x + threadIdx.x;
}

/* ------------------------------------------------------------------ *
 * Kernels
 * ------------------------------------------------------------------ */

/* Global sorted table: every thread probes global memory. */
template <class F, int CHUNK>
__global__ void RF_BOUNDS k_decompose_sorted(DecompArgs<F> a, SortedTable table) {
    rf_decompose_thread<F, SortedTable, CHUNK>(a, table, rf_global_tid());
}

/* Shared-memory hash table: the block copies the table (keys + values)
 * into shared memory, then probes it.  `cap` must be
 * <= RF_SHARED_HASH_CAP; the host checks before launching.
 *
 * The staging buffer is dynamic shared memory sized to the table's
 * capacity (cap * 12 bytes, launched with that many bytes).  It used to
 * be a static RF_SHARED_HASH_CAP-slot array, i.e. 48 KiB per block
 * whatever the factor base, which caps residency at 1-4 blocks per SM
 * (96-228 KiB of shared memory) even for a 64-slot table.  The probes
 * see the same keys and values at the same slots.  Untested on a GPU. */
template <class F, int CHUNK>
__global__ void RF_BOUNDS k_decompose_shared_hash(DecompArgs<F> a, HashTable gtable) {
    extern __shared__ uint64_t s_dyn[];
    const uint32_t cap = gtable.mask + 1;
    HashTable table = gtable;
#if RF_HASH_PACKED
    /* packed slots only: 8 bytes per slot instead of 12 */
    for (uint32_t s = threadIdx.x; s < cap; s += blockDim.x) s_dyn[s] = gtable.slots[s];
    __syncthreads();
    table.slots = s_dyn;
#else
    uint64_t *s_keys = s_dyn;
    uint32_t *s_vals = reinterpret_cast<uint32_t *>(s_dyn + cap);
    for (uint32_t s = threadIdx.x; s < cap; s += blockDim.x) {
        s_keys[s] = gtable.keys[s];
        s_vals[s] = gtable.vals[s];
    }
    __syncthreads();
    table.keys = s_keys;
    table.vals = s_vals;
#endif
    rf_decompose_thread<F, HashTable, CHUNK>(a, table, rf_global_tid());
}

/* Global hash table (no staging) for factor bases too large for shared
 * memory but where the caller explicitly asked for hashing. */
template <class F, int CHUNK>
__global__ void RF_BOUNDS k_decompose_global_hash(DecompArgs<F> a, HashTable table) {
    rf_decompose_thread<F, HashTable, CHUNK>(a, table, rf_global_tid());
}

/* ------------------------------------------------------------------ *
 * Host driver
 * ------------------------------------------------------------------ */

template <class F>
static int run_device(const typename F::ctx &f,
                      const typename F::elt *fb_x, const typename F::elt *fb_y, uint32_t n_fb,
                      const typename F::elt *tx, const typename F::elt *ty, uint32_t n_targets,
                      int lookup, uint32_t chunk,
                      rf_relation *out, uint32_t cap, uint32_t *n_found, double *seconds) {
    typedef typename F::elt elt;
    if (!rf::chunk_ok(chunk)) return RF_ERR_BAD_CHUNK;

    /* gridDim.x maximum is 2^31-1.  The (target, slice) product is 2^32
     * at the batch cited in the note, and a uint32 count wraps to an
     * empty grid that still returns RF_OK. */
    const uint64_t total = rf_thread_count(n_targets, n_fb, (int)chunk);
    const uint64_t grid64 = (total + (uint64_t)RF_BLOCK - 1) / (uint64_t)RF_BLOCK;
    if (grid64 == 0 || grid64 > 2147483647ull) return RF_ERR_BAD_ARG;
    const unsigned int grid = (unsigned int)grid64;

    /* Table (built on the host; the view below points into device
     * copies of its arrays). */
    rf::SortedTableHost sorted;
    rf::HashTableHost hash;
    if (lookup == RF_LOOKUP_SORTED) sorted = rf::build_sorted<F>(f, fb_x, n_fb);
    else hash = rf::build_hash<F>(f, fb_x, n_fb);

    int rc = RF_OK;
    elt *d_fbx = nullptr, *d_fby = nullptr, *d_tx = nullptr, *d_ty = nullptr;
    uint64_t *d_keys = nullptr, *d_aux64 = nullptr;
    uint32_t *d_idx = nullptr, *d_cnt = nullptr, *d_aux32 = nullptr;
    rf_relation *d_out = nullptr;
    cudaEvent_t e0 = nullptr, e1 = nullptr;
    SortedTable sview{};
    HashTable hview{};
    DecompArgs<F> a{};
    float ms = 0;
    uint32_t found = 0, nw = 0, hcap = 0;
    size_t hash_smem = 0;

    CU(rf_cu_alloc(&d_fbx, n_fb * sizeof(elt)));
    CU(rf_cu_alloc(&d_fby, n_fb * sizeof(elt)));
    CU(rf_cu_alloc(&d_tx, n_targets * sizeof(elt)));
    CU(rf_cu_alloc(&d_ty, n_targets * sizeof(elt)));
    CU(rf_cu_alloc(&d_cnt, sizeof(uint32_t)));
    CU(rf_cu_alloc(&d_out, (cap ? cap : 1) * sizeof(rf_relation)));
    CU(cudaMemcpy(d_fbx, fb_x, n_fb * sizeof(elt), cudaMemcpyHostToDevice));
    CU(cudaMemcpy(d_fby, fb_y, n_fb * sizeof(elt), cudaMemcpyHostToDevice));
    CU(cudaMemcpy(d_tx, tx, n_targets * sizeof(elt), cudaMemcpyHostToDevice));
    CU(cudaMemcpy(d_ty, ty, n_targets * sizeof(elt), cudaMemcpyHostToDevice));
    CU(cudaMemset(d_cnt, 0, sizeof(uint32_t)));

    /* Device views: the same POD structs the kernels take, pointing at
     * device copies of the host-built arrays. */
    if (lookup == RF_LOOKUP_SORTED) {
        CU(rf_cu_alloc(&d_keys, n_fb * sizeof(uint64_t)));
        CU(rf_cu_alloc(&d_idx, n_fb * sizeof(uint32_t)));
        CU(cudaMemcpy(d_keys, sorted.keys.data(), n_fb * sizeof(uint64_t), cudaMemcpyHostToDevice));
        CU(cudaMemcpy(d_idx, sorted.idx.data(), n_fb * sizeof(uint32_t), cudaMemcpyHostToDevice));
        sview.keys = d_keys; sview.idx = d_idx; sview.n = n_fb;
        /* Eytzinger copy (RF_SORTED_EYT): n_fb + 1 entries, 1-indexed */
        CU(rf_cu_alloc(&d_aux64, (n_fb + 1) * sizeof(uint64_t)));
        CU(rf_cu_alloc(&d_aux32, (n_fb + 1) * sizeof(uint32_t)));
        CU(cudaMemcpy(d_aux64, sorted.eyt.data(), (n_fb + 1) * sizeof(uint64_t), cudaMemcpyHostToDevice));
        CU(cudaMemcpy(d_aux32, sorted.eyt_rank.data(), (n_fb + 1) * sizeof(uint32_t), cudaMemcpyHostToDevice));
        sview.eyt = d_aux64; sview.eyt_rank = d_aux32;
    } else {
        hcap = hash.capacity();
        CU(rf_cu_alloc(&d_keys, hcap * sizeof(uint64_t)));
        CU(rf_cu_alloc(&d_idx, hcap * sizeof(uint32_t)));
        CU(cudaMemcpy(d_keys, hash.keys.data(), hcap * sizeof(uint64_t), cudaMemcpyHostToDevice));
        CU(cudaMemcpy(d_idx, hash.vals.data(), hcap * sizeof(uint32_t), cudaMemcpyHostToDevice));
        hview.keys = d_keys; hview.vals = d_idx; hview.mask = hash.mask;
        /* packed slots (RF_HASH_PACKED) */
        CU(rf_cu_alloc(&d_aux64, hcap * sizeof(uint64_t)));
        CU(cudaMemcpy(d_aux64, hash.slots.data(), hcap * sizeof(uint64_t), cudaMemcpyHostToDevice));
        hview.slots = d_aux64;
    }

    a.f = f;
    a.fb_x = d_fbx; a.fb_y = d_fby; a.n_fb = n_fb;
    a.probe_x = d_fbx; a.probe_mode = 0;
    a.t_x = d_tx; a.t_y = d_ty; a.n_targets = n_targets;
    a.out = d_out; a.out_count = d_cnt; a.out_cap = cap;

#if RF_HASH_PACKED
    hash_smem = (size_t)hcap * sizeof(uint64_t);
#else
    hash_smem = (size_t)hcap * (sizeof(uint64_t) + sizeof(uint32_t));
#endif
    CU(rf_cu_event(&e0));
    CU(rf_cu_event(&e1));
    CU(cudaEventRecord(e0));

    /* Dispatch on (lookup, chunk).  Device views are passed by value. */
#define RF_LAUNCH(CH)                                                                         \
    if (lookup == RF_LOOKUP_SORTED) {                                                         \
        k_decompose_sorted<F, CH><<<grid, RF_BLOCK>>>(a, sview);                              \
    } else if (hash.capacity() <= RF_SHARED_HASH_CAP) {                                       \
        k_decompose_shared_hash<F, CH><<<grid, RF_BLOCK, hash_smem>>>(a, hview);              \
    } else {                                                                                  \
        k_decompose_global_hash<F, CH><<<grid, RF_BLOCK>>>(a, hview);                         \
    }
    switch (chunk) {
        case 16: RF_LAUNCH(16) break;
        case 32: RF_LAUNCH(32) break;
        case 64: RF_LAUNCH(64) break;
        default: rc = RF_ERR_BAD_CHUNK; goto cleanup;
    }
#undef RF_LAUNCH
    CU(cudaGetLastError());
    CU(cudaEventRecord(e1));
    CU(cudaEventSynchronize(e1));
    CU(cudaEventElapsedTime(&ms, e0, e1));
    *seconds = ms / 1000.0;

    CU(cudaMemcpy(&found, d_cnt, sizeof(uint32_t), cudaMemcpyDeviceToHost));
    *n_found = found;
    nw = found < cap ? found : cap;
    if (nw) CU(cudaMemcpy(out, d_out, nw * sizeof(rf_relation), cudaMemcpyDeviceToHost));

cleanup:
    /* A failed launch or memcpy leaves a sticky runtime error.  Later
     * calls then return that error and do not free, so clear it first. */
    cudaGetLastError();
    cudaFree(d_fbx); cudaFree(d_fby); cudaFree(d_tx); cudaFree(d_ty);
    cudaFree(d_keys); cudaFree(d_idx); cudaFree(d_cnt); cudaFree(d_out);
    cudaFree(d_aux64); cudaFree(d_aux32);
    if (e0) cudaEventDestroy(e0);
    if (e1) cudaEventDestroy(e1);
    return rc;
}

extern "C" int rf_cuda_decompose64(const Fp64Ctx *f,
                                   const uint64_t *fb_x, const uint64_t *fb_y, uint32_t n_fb,
                                   const uint64_t *tx, const uint64_t *ty, uint32_t n_targets,
                                   int lookup, uint32_t chunk,
                                   rf_relation *out, uint32_t cap, uint32_t *n_found, double *seconds) {
    return run_device<FieldFp64>(*f, fb_x, fb_y, n_fb, tx, ty, n_targets, lookup, chunk, out, cap, n_found, seconds);
}

extern "C" int rf_cuda_decompose256(const Fp256Ctx *f,
                                    const U256x64 *fb_x, const U256x64 *fb_y, uint32_t n_fb,
                                    const U256x64 *tx, const U256x64 *ty, uint32_t n_targets,
                                    int lookup, uint32_t chunk,
                                    rf_relation *out, uint32_t cap, uint32_t *n_found, double *seconds) {
    /* See the register note above: P-256 at CHUNK=32/64 spills.  We
     * honour the request (the caller may be measuring exactly that) but
     * the default the API applies for P-256 is 16. */
    return run_device<FieldP256>(*f, fb_x, fb_y, n_fb, tx, ty, n_targets, lookup, chunk, out, cap, n_found, seconds);
}

/* ------------------------------------------------------------------ *
 * Persistent Fp64 session (rootfind_serve): the factor base, its table and
 * the target/output buffers stay on the device across calls, so a batch
 * costs two copies in, one launch and the copies out -- no table build,
 * no cudaMalloc.  The kernels and their arguments are the ones
 * run_device launches.
 * ------------------------------------------------------------------ */

struct RfCudaSession64 {
    Fp64Ctx f;
    int lookup;
    uint32_t chunk, n_fb, max_targets, cap, hcap, n_probe;
    uint64_t *d_fbx, *d_fby, *d_tx, *d_ty, *d_keys, *d_aux64, *d_px;
    uint32_t *d_idx, *d_aux32, *d_cnt;
    rf_relation *d_out;
    SortedTable sview;
    HashTable hview;
    cudaEvent_t e0, e1;
};

extern "C" void rf_cuda_session64_destroy(void *h) {
    RfCudaSession64 *s = static_cast<RfCudaSession64 *>(h);
    if (!s) return;
    cudaGetLastError();
    cudaFree(s->d_fbx); cudaFree(s->d_fby); cudaFree(s->d_tx); cudaFree(s->d_ty);
    cudaFree(s->d_keys); cudaFree(s->d_aux64); cudaFree(s->d_idx); cudaFree(s->d_aux32);
    cudaFree(s->d_cnt); cudaFree(s->d_out); cudaFree(s->d_px);
    if (s->e0) cudaEventDestroy(s->e0);
    if (s->e1) cudaEventDestroy(s->e1);
    delete s;
}

#define CUS(call)                                                                  \
    do {                                                                           \
        cudaError_t e_ = (call);                                                   \
        if (e_ != cudaSuccess) {                                                   \
            std::fprintf(stderr, "CUDA error %s at %s:%d: %s\n", #call, __FILE__,  \
                         __LINE__, cudaGetErrorString(e_));                        \
            rc = RF_ERR_CUDA;                                                      \
            goto fail;                                                             \
        }                                                                          \
    } while (0)

/* fb_x, fb_y: Montgomery form.  Returns RF_OK and *out = session. */
/* probe_x (Montgomery form, n_probe entries) non-null: probe mode, the table is
 * built on probe_x; null: the factor base, as rf64_decompose. */
extern "C" int rf_cuda_session64_create(const Fp64Ctx *f, const uint64_t *fb_x, const uint64_t *fb_y,
                                        uint32_t n_fb, const uint64_t *probe_x, uint32_t n_probe,
                                        int lookup, uint32_t chunk,
                                        uint32_t max_targets, uint32_t cap, void **out) {
    int rc = RF_OK;
    if (!rf::chunk_ok(chunk)) return RF_ERR_BAD_CHUNK;
    if (n_fb == 0 || max_targets == 0) return RF_ERR_BAD_ARG;
    RfCudaSession64 *s = new RfCudaSession64();
    s->f = *f; s->lookup = lookup; s->chunk = chunk; s->n_fb = n_fb;
    s->max_targets = max_targets; s->cap = cap ? cap : 1;
    s->n_probe = probe_x ? n_probe : 0;
    {
        const uint64_t *tx_keys = s->n_probe ? probe_x : fb_x;
        const uint32_t n_keys = s->n_probe ? s->n_probe : n_fb;
        rf::SortedTableHost sorted;
        rf::HashTableHost hash;
        if (lookup == RF_LOOKUP_SORTED) sorted = rf::build_sorted<FieldFp64>(*f, tx_keys, n_keys);
        else hash = rf::build_hash<FieldFp64>(*f, tx_keys, n_keys);
        if (s->n_probe) {
            CUS(rf_cu_alloc(&s->d_px, s->n_probe * sizeof(uint64_t)));
            CUS(cudaMemcpy(s->d_px, probe_x, s->n_probe * sizeof(uint64_t), cudaMemcpyHostToDevice));
        }
        CUS(rf_cu_alloc(&s->d_fbx, n_fb * sizeof(uint64_t)));
        CUS(rf_cu_alloc(&s->d_fby, n_fb * sizeof(uint64_t)));
        CUS(rf_cu_alloc(&s->d_tx, max_targets * sizeof(uint64_t)));
        CUS(rf_cu_alloc(&s->d_ty, max_targets * sizeof(uint64_t)));
        CUS(rf_cu_alloc(&s->d_cnt, sizeof(uint32_t)));
        CUS(rf_cu_alloc(&s->d_out, s->cap * sizeof(rf_relation)));
        CUS(cudaMemcpy(s->d_fbx, fb_x, n_fb * sizeof(uint64_t), cudaMemcpyHostToDevice));
        CUS(cudaMemcpy(s->d_fby, fb_y, n_fb * sizeof(uint64_t), cudaMemcpyHostToDevice));
        if (lookup == RF_LOOKUP_SORTED) {
            CUS(rf_cu_alloc(&s->d_keys, n_keys * sizeof(uint64_t)));
            CUS(rf_cu_alloc(&s->d_idx, n_keys * sizeof(uint32_t)));
            CUS(rf_cu_alloc(&s->d_aux64, (n_keys + 1) * sizeof(uint64_t)));
            CUS(rf_cu_alloc(&s->d_aux32, (n_keys + 1) * sizeof(uint32_t)));
            CUS(cudaMemcpy(s->d_keys, sorted.keys.data(), n_keys * sizeof(uint64_t), cudaMemcpyHostToDevice));
            CUS(cudaMemcpy(s->d_idx, sorted.idx.data(), n_keys * sizeof(uint32_t), cudaMemcpyHostToDevice));
            CUS(cudaMemcpy(s->d_aux64, sorted.eyt.data(), (n_keys + 1) * sizeof(uint64_t), cudaMemcpyHostToDevice));
            CUS(cudaMemcpy(s->d_aux32, sorted.eyt_rank.data(), (n_keys + 1) * sizeof(uint32_t), cudaMemcpyHostToDevice));
            s->sview.keys = s->d_keys; s->sview.idx = s->d_idx; s->sview.n = n_keys;
            s->sview.eyt = s->d_aux64; s->sview.eyt_rank = s->d_aux32;
        } else {
            s->hcap = hash.capacity();
            CUS(rf_cu_alloc(&s->d_keys, s->hcap * sizeof(uint64_t)));
            CUS(rf_cu_alloc(&s->d_idx, s->hcap * sizeof(uint32_t)));
            CUS(rf_cu_alloc(&s->d_aux64, s->hcap * sizeof(uint64_t)));
            CUS(cudaMemcpy(s->d_keys, hash.keys.data(), s->hcap * sizeof(uint64_t), cudaMemcpyHostToDevice));
            CUS(cudaMemcpy(s->d_idx, hash.vals.data(), s->hcap * sizeof(uint32_t), cudaMemcpyHostToDevice));
            CUS(cudaMemcpy(s->d_aux64, hash.slots.data(), s->hcap * sizeof(uint64_t), cudaMemcpyHostToDevice));
            s->hview.keys = s->d_keys; s->hview.vals = s->d_idx; s->hview.mask = hash.mask;
            s->hview.slots = s->d_aux64;
        }
        CUS(rf_cu_event(&s->e0));
        CUS(rf_cu_event(&s->e1));
    }
    *out = s;
    return RF_OK;
fail:
    rf_cuda_session64_destroy(s);
    return rc;
}

/* tx, ty: Montgomery form, n_targets <= max_targets.  Writes up to the
 * session's cap relations to out[]; *n_found is the kernel's count. */
extern "C" int rf_cuda_session64_run(void *h, const uint64_t *tx, const uint64_t *ty, uint32_t n_targets,
                                     rf_relation *out, uint32_t *n_found, double *seconds) {
    RfCudaSession64 *s = static_cast<RfCudaSession64 *>(h);
    int rc = RF_OK;
    if (!s || n_targets == 0 || n_targets > s->max_targets) return RF_ERR_BAD_ARG;
    const uint64_t total = rf_thread_count(n_targets, s->n_fb, (int)s->chunk);
    const uint64_t grid64 = (total + (uint64_t)RF_BLOCK - 1) / (uint64_t)RF_BLOCK;
    if (grid64 == 0 || grid64 > 2147483647ull) return RF_ERR_BAD_ARG;
    const unsigned int grid = (unsigned int)grid64;
    DecompArgs<FieldFp64> a{};
    float ms = 0;
    uint32_t found = 0, nw = 0;
#if RF_HASH_PACKED
    const size_t hash_smem = (size_t)s->hcap * sizeof(uint64_t);
#else
    const size_t hash_smem = (size_t)s->hcap * (sizeof(uint64_t) + sizeof(uint32_t));
#endif
    a.f = s->f;
    a.fb_x = s->d_fbx; a.fb_y = s->d_fby; a.n_fb = s->n_fb;
    a.probe_x = s->n_probe ? s->d_px : s->d_fbx; a.probe_mode = s->n_probe ? 1u : 0u;
    a.t_x = s->d_tx; a.t_y = s->d_ty; a.n_targets = n_targets;
    a.out = s->d_out; a.out_count = s->d_cnt; a.out_cap = s->cap;
    CUS(cudaMemcpy(s->d_tx, tx, n_targets * sizeof(uint64_t), cudaMemcpyHostToDevice));
    CUS(cudaMemcpy(s->d_ty, ty, n_targets * sizeof(uint64_t), cudaMemcpyHostToDevice));
    CUS(cudaMemset(s->d_cnt, 0, sizeof(uint32_t)));
    CUS(cudaEventRecord(s->e0));
#define RF_SLAUNCH(CH)                                                                        \
    if (s->lookup == RF_LOOKUP_SORTED) {                                                      \
        k_decompose_sorted<FieldFp64, CH><<<grid, RF_BLOCK>>>(a, s->sview);                   \
    } else if (s->hcap <= RF_SHARED_HASH_CAP) {                                               \
        k_decompose_shared_hash<FieldFp64, CH><<<grid, RF_BLOCK, hash_smem>>>(a, s->hview);   \
    } else {                                                                                  \
        k_decompose_global_hash<FieldFp64, CH><<<grid, RF_BLOCK>>>(a, s->hview);              \
    }
    switch (s->chunk) {
        case 16: RF_SLAUNCH(16) break;
        case 32: RF_SLAUNCH(32) break;
        case 64: RF_SLAUNCH(64) break;
        default: return RF_ERR_BAD_CHUNK;
    }
#undef RF_SLAUNCH
    CUS(cudaGetLastError());
    CUS(cudaEventRecord(s->e1));
    CUS(cudaEventSynchronize(s->e1));
    CUS(cudaEventElapsedTime(&ms, s->e0, s->e1));
    *seconds = ms / 1000.0;
    CUS(cudaMemcpy(&found, s->d_cnt, sizeof(uint32_t), cudaMemcpyDeviceToHost));
    *n_found = found;
    nw = found < s->cap ? found : s->cap;
    if (nw) CUS(cudaMemcpy(out, s->d_out, nw * sizeof(rf_relation), cudaMemcpyDeviceToHost));
    return RF_OK;
fail:
    cudaGetLastError();
    return rc;
}
#undef CUS

/* Create this process's CUDA context now (the daemon does, before it listens). */
extern "C" int rf_cuda_warmup(void) {
    return cudaFree(0) == cudaSuccess ? 0 : -1;
}

/* RF_PROFILE builds: reset and read the sampled phase cycles of
 * rf_decompose_thread (rootfind.cuh).  Return 0 on success, -1 when the
 * build has no profile. */
extern "C" int rf_profile_reset(void) {
#if RF_PROFILE
    unsigned long long z[8] = {0, 0, 0, 0, 0, 0, 0, 0};
    return cudaMemcpyToSymbol(rf_prof_cycles, z, sizeof z) == cudaSuccess ? 0 : -1;
#else
    return -1;
#endif
}

extern "C" int rf_profile_read(unsigned long long *out8) {
#if RF_PROFILE
    return cudaMemcpyFromSymbol(out8, rf_prof_cycles, 8 * sizeof(unsigned long long)) == cudaSuccess ? 0 : -1;
#else
    (void)out8;
    return -1;
#endif
}

extern "C" int rf_cuda_device_present(char *name, int name_len) {
    int n = 0;
    if (cudaGetDeviceCount(&n) != cudaSuccess || n <= 0) return 0;
    if (name && name_len > 0) {
        cudaDeviceProp prop;
        if (cudaGetDeviceProperties(&prop, 0) == cudaSuccess) {
            std::snprintf(name, (size_t)name_len, "%s (sm_%d%d, %d SMs)", prop.name, prop.major, prop.minor,
                          prop.multiProcessorCount);
        }
    }
    return 1;
}
