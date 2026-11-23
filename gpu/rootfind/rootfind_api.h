/* rootfind_api.h -- C ABI for the GPU root-finding 2-decomposition oracle.
 *
 * Consumers: the Rust FFI crate gpu/rootfind/rs (its build.rs compiles
 * rootfind_api.cpp with the emulation backend, or links a prebuilt
 * librootfind_cuda.a when GPU_ROOTFIND_CUDA_LIB_DIR is set; the root
 * crate's src/cryptanalysis/gpu_rootfind.rs is pure Rust and does not
 * link this library), the CUDA driver bench.cu, and the CPU test harness
 * test_cpu.cpp.  Two backends implement the same contract:
 *
 *   emulate -- the kernel's per-thread function (rootfind.cuh) run on the
 *              host for every (target, slice) thread index.  Always
 *              available; what this repository's validation ran.
 *   cuda    -- rootfind_cuda.cu, compiled only when the library is built
 *              with RF_HAVE_CUDA (make lib-cuda).  rf_cuda_available()
 *              reports whether it is linked in *and* a device is present.
 *
 * Coordinates cross this boundary in canonical form (integers < p,
 * little-endian limbs for P-256); the library converts to Montgomery
 * form internally.  Results are index-level relations:
 *
 *   target k:  R_k = eps * F_i + tau * F_j          (tau = +-1, j >= i)
 *              R_k = eps * F_i                      (tau = 0, j == i)
 *
 * Every result must still be re-added in the group by the caller (the
 * Rust harness does); the library itself only promises to report what
 * the kernel computed.
 */
#ifndef GPU_ROOTFIND_API_H
#define GPU_ROOTFIND_API_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct rf_relation {
    uint32_t target;
    uint32_t i;
    uint32_t j;
    int8_t eps;
    int8_t tau;
    uint8_t pad[2];
} rf_relation;   /* 16 bytes */

enum rf_backend { RF_BACKEND_AUTO = 0, RF_BACKEND_EMULATE = 1, RF_BACKEND_CUDA = 2 };
/* RF_LOOKUP_HASH_DEVICE (sessions only): the hash table of the lookup set is built on the
 * device (CUDA backend; packed slots, one atomicCAS insert per key) instead of on the host.
 * The table answers the same lookups; only slot order may differ.  Emulation builds it on
 * the host. */
enum rf_lookup  { RF_LOOKUP_AUTO = 0, RF_LOOKUP_SORTED = 1, RF_LOOKUP_HASH = 2, RF_LOOKUP_HASH_DEVICE = 3,
                  RF_LOOKUP_PAIRS_DEVICE = 4 /* rf64_session_create_pairs only */ };

/* Library default for chunk == 0, per field.  The slice length sets the
 * per-thread register footprint (2 * chunk field elements live during
 * the batched inversion): 32 one-word elements fit comfortably, 32
 * four-limb P-256 elements do not, hence the smaller P-256 default.
 * Wrappers pass 0 through; the policy lives here only. */
#define RF_DEFAULT_CHUNK_64  32u
#define RF_DEFAULT_CHUNK_256 16u

typedef struct rf_options {
    int backend;        /* enum rf_backend */
    int lookup;         /* enum rf_lookup */
    uint32_t chunk;     /* factor-base slice per thread: 16, 32 or 64;
                           0 = RF_DEFAULT_CHUNK_64 (rf64_decompose) or
                               RF_DEFAULT_CHUNK_256 (rf256_decompose) */
} rf_options;

typedef struct rf_result {
    uint32_t n_written;   /* relations stored in out[] (<= cap) */
    uint32_t n_found;     /* relations the kernel found; > cap means overflow */
    int backend_used;     /* enum rf_backend actually run */
    int lookup_used;      /* enum rf_lookup actually run */
    uint32_t chunk_used;
    double seconds;       /* wall time of the oracle call, excluding table build */
} rf_result;

enum rf_status {
    RF_OK = 0,
    RF_ERR_OVERFLOW = 1,          /* out[] too small; n_found tells how many */
    RF_ERR_BAD_ARG = -1,
    RF_ERR_NO_CUDA = -2,          /* CUDA backend requested but not linked / no device */
    RF_ERR_CUDA = -3,             /* a CUDA runtime call failed */
    RF_ERR_BAD_CHUNK = -4,
    RF_ERR_NOMEM = -5
};

/* 64-bit field: odd prime 3 < p < 2^62 (same bound as prime_fast::Fp64).
 * fb_x/fb_y: n_fb canonical coordinates; tx/ty: n_targets canonical
 * coordinates.  Writes up to `cap` relations to out[]. */
int rf64_decompose(uint64_t p,
                   const uint64_t *fb_x, const uint64_t *fb_y, uint32_t n_fb,
                   const uint64_t *tx, const uint64_t *ty, uint32_t n_targets,
                   rf_relation *out, uint32_t cap,
                   const rf_options *opt, rf_result *res);

/* NIST P-256 field (fixed modulus).  Each coordinate is 4 little-endian
 * 64-bit limbs, so fb_x has 4*n_fb words etc. */
int rf256_decompose(const uint64_t *fb_x, const uint64_t *fb_y, uint32_t n_fb,
                    const uint64_t *tx, const uint64_t *ty, uint32_t n_targets,
                    rf_relation *out, uint32_t cap,
                    const rf_options *opt, rf_result *res);

/* Persistent 64-bit session (the rootfind_serve worker): the factor base
 * and its lookup table are set up once and kept (on the device for the
 * CUDA backend); each rf64_session_decompose call then takes one batch of
 * at most `max_targets` targets.  Results and their meaning are those of
 * rf64_decompose on the same factor base and targets; `cap` bounds the
 * relations one batch may report (overflow returns RF_ERR_OVERFLOW with
 * res->n_found set, and the caller may retry with a smaller batch).
 *
 * probe_x (n_probe canonical abscissae) selects probe mode: each candidate
 * x(R -/+ F_i) is looked up in that set instead of the factor base, and every
 * hit is reported as (target, i, j = index into probe_x, eps, tau = +1), with no
 * j >= i filter -- the three-summand sweep against a pair table's abscissae.
 * probe_x = NULL: the factor base, exactly as rf64_decompose. */
typedef struct rf64_session rf64_session;
int rf64_session_create(uint64_t p, const uint64_t *fb_x, const uint64_t *fb_y, uint32_t n_fb,
                        const uint64_t *probe_x, uint32_t n_probe, uint32_t max_targets, uint32_t cap, const rf_options *opt,
                        rf64_session **out, rf_result *info);
int rf64_session_decompose(rf64_session *s, const uint64_t *tx, const uint64_t *ty,
                           uint32_t n_targets, rf_relation *out, uint32_t cap, rf_result *res);
void rf64_session_destroy(rf64_session *s);
/* Probe mode against the negation-folded pair table of the base itself, built by
 * the library (on the device for the CUDA backend): the abscissae of P_ai +/- P_bi,
 * bi >= ai, first pair per abscissa in build order (rootfind.cuh rf_pair_x).  Each
 * reported hit (tau != 0) carries j = that first pair's build index
 * b = rf_pair_offset(ai, n) + 2 (bi - ai) + s.  curve_a: the curve's a (canonical);
 * every base y must be non-zero.  *n_entries: distinct abscissae in the table. */
int rf64_session_create_pairs(uint64_t p, uint64_t curve_a, const uint64_t *fb_x, const uint64_t *fb_y,
                              uint32_t n_fb, uint32_t max_targets, uint32_t cap, const rf_options *opt,
                              rf64_session **out, rf_result *info, uint32_t *n_entries);

/* Create the CUDA context now when the CUDA backend is available (0 on success
 * or when there is nothing to do). */
int rf_warmup(void);

/* Threads the emulation backend runs on (RF_EMU_THREADS, else the CPU count capped by a
 * cgroup quota). */
unsigned rf_emu_threads(void);

/* CUDA only: the pair-table key kernels (per cell, batched runs) on a random base of n
 * abscissae over p: device times of each and the count of cells whose keys differ.
 * -1 without the CUDA backend. */
int rf_pairs_bench(uint64_t p, uint32_t n, uint64_t seed, double *t_cell, double *t_run, uint64_t *mismatch);

/* 1 if the CUDA backend is compiled in and at least one device answers. */
int rf_cuda_available(void);

/* Human-readable backend description, e.g. "emulate (host)" or the
 * device name when CUDA is available. */
const char *rf_backend_info(void);

const char *rf_strerror(int status);

#ifdef __cplusplus
}
#endif

#endif /* GPU_ROOTFIND_API_H */
