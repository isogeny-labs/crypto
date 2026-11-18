/* rootfind.cuh -- the batched root-finding 2-decomposition oracle, written
 * once as a per-thread function that both the CUDA kernels
 * (rootfind_cuda.cu) and the host emulation backend (rootfind_host.hpp)
 * execute unchanged.
 *
 * Mathematics (RESEARCH_SEMAEV_ROOTFIND_VS_GROEBNER.md, Lean-checked):
 * for y^2 = x^3 + ax + b the roots of Semaev's S3(x_R, x_i, X) in X are
 * exactly x(R + F_i) and x(R - F_i).  So R decomposes over the factor
 * base F = {F_1..F_N} as R = eps*F_i + tau*F_j iff x(R - eps*F_i) is the
 * x-coordinate of some F_j.  Per (target, i) that is
 *
 *     d      = x_R - x_i                      (shared by both signs)
 *     lam+   = (y_R + y_i) * d^-1              eps = +1  (R - F_i)
 *     lam-   = (y_R - y_i) * d^-1              eps = -1  (R + F_i)
 *     x3     = lam^2 - x_R - x_i
 *     hit    = x3 in {x_j}  ->  y3 = lam*(x_R - x3) - y_R, tau = (y3 == y_j) ? +1 : -1
 *
 * which is precisely the loop in prime_fast::collect_relations; the Rust
 * harness checks the two agree on every target.
 *
 * Thread mapping
 * --------------
 * One thread handles one target and a CHUNK-long slice of the factor
 * base.  Thread t -> (target k = t mod M, slice c = t div M).  Consecutive
 * lanes of a warp therefore hold consecutive *targets* and the *same*
 * slice: the target loads (x_R, y_R) are coalesced, and every factor-base
 * load inside the slice loop is warp-uniform -- one 8-byte broadcast per
 * warp instead of 32 scattered loads.  The factor base is SoA (separate x
 * and y arrays) so a thread touching only x never drags y through L1.
 *
 * Inversions
 * ----------
 * Each thread runs Montgomery's simultaneous inversion over its CHUNK
 * differences: one Fermat inversion (~62 sqr + ~31 mul for a 62-bit p,
 * no divergence because the exponent is the public modulus) plus
 * 3(CHUNK-1) multiplications.  The alternative, one Fermat inversion per
 * (target, i) pair, costs ~95 multiplications per pair against
 * ~(95/CHUNK + 3) here; at CHUNK = 32 that is 6 vs 95.  The price is
 * 2*CHUNK 64-bit values of per-thread scratch (d[] and the prefix
 * products), i.e. 4*CHUNK 32-bit registers -- the register budget is set
 * by CHUNK, not by the field arithmetic.  CHUNK = 32 (~170 registers,
 * ~12 warps/SM on sm_120) is the default; CHUNK = 16 trades ~30% more
 * multiplications for ~2x occupancy, CHUNK = 64 spills.  Measure on the
 * device; see RESEARCH_GPU_ROOTFIND_DECOMPOSITION.md §3.
 *
 * Lookup
 * ------
 * The table is keyed by the *Montgomery form* of x (its low 64 bits for
 * P-256), never by the canonical value: the Montgomery representative of
 * a residue is unique, so equality of representations is equality of
 * values, and skipping the from_mont conversion saves one multiplication
 * per probe.  Two table shapes implement the same probe interface:
 *   SortedTable  -- keys sorted ascending + original index; lower_bound
 *                   binary search, log2(N) dependent loads per probe.
 *                   Global memory; any N.
 *   HashTable    -- open addressing, linear probing, load <= 1/2;
 *                   ~1-2 loads per probe.  The CUDA kernel stages it in
 *                   shared memory when it fits (RF_SHARED_HASH_CAP slots).
 * A probe may yield several candidates with the same 64-bit key (P-256
 * only); the full-width equality test below rejects false ones.
 *
 * Output
 * ------
 * Relations are appended to out[] through an atomic counter.  Only j >= i
 * is reported (the pair (i, j) and (j, i) are the same relation, and
 * this is the convention collect_relations uses).  The degenerate case
 * x_R == x_i (R = +-F_i) is reported as (i, eps, j = i, tau = 0).  If
 * the buffer fills, the counter keeps counting past the capacity so the
 * host can detect the overflow and fail closed.
 */
#ifndef GPU_ROOTFIND_ROOTFIND_CUH
#define GPU_ROOTFIND_ROOTFIND_CUH

#include <stdint.h>
#include "fp64.cuh"
#include "fp256x64.cuh"
#include "rootfind_api.h"

#define RF_NO_INDEX 0xFFFFFFFFu

/* Hash-table capacity (slots) the CUDA kernel is willing to stage in
 * shared memory: 4096 * (8 + 4) bytes = 48 KiB, the static shared-memory
 * ceiling, which on sm_120 (100 KiB configurable per SM) still allows
 * two resident blocks.  Factor bases up to RF_SHARED_HASH_CAP / 2
 * entries use it; larger ones fall back to the global sorted table. */
#ifndef RF_SHARED_HASH_CAP
#define RF_SHARED_HASH_CAP 4096
#endif

/* ------------------------------------------------------------------ *
 * Field adaptors: the thread function is templated on one of these.
 * ------------------------------------------------------------------ */
struct FieldFp64 {
    typedef uint64_t elt;
    typedef Fp64Ctx ctx;
    static RF_HD elt add(const ctx &f, elt a, elt b) { return fp64_add(f, a, b); }
    static RF_HD elt sub(const ctx &f, elt a, elt b) { return fp64_sub(f, a, b); }
    static RF_HD elt mul(const ctx &f, elt a, elt b) { return fp64_mul(f, a, b); }
    static RF_HD elt sqr(const ctx &f, elt a) { return fp64_sqr(f, a); }
    static RF_HD elt inv(const ctx &f, elt a) { return fp64_inv(f, a); }
    static RF_HD elt one(const ctx &f) { return f.one; }
    static RF_HD int is_zero(const ctx &, elt a) { return a == 0; }
    static RF_HD int eq(const ctx &, elt a, elt b) { return a == b; }
    /* lookup key: the Montgomery value itself (unique, < 2^62) */
    static RF_HD uint64_t key(const ctx &, elt a) { return a; }
};

struct FieldP256 {
    typedef U256x64 elt;
    typedef Fp256Ctx ctx;
    static RF_HD elt add(const ctx &f, const elt &a, const elt &b) { return fp256_add(f, a, b); }
    static RF_HD elt sub(const ctx &f, const elt &a, const elt &b) { return fp256_sub(f, a, b); }
    static RF_HD elt mul(const ctx &f, const elt &a, const elt &b) { return fp256_mul(f, a, b); }
    static RF_HD elt sqr(const ctx &f, const elt &a) { return fp256_sqr(f, a); }
    static RF_HD elt inv(const ctx &f, const elt &a) { return fp256_inv(f, a); }
    static RF_HD elt one(const ctx &f) { return f.one; }
    static RF_HD int is_zero(const ctx &, const elt &a) { return u256_is_zero(a); }
    static RF_HD int eq(const ctx &, const elt &a, const elt &b) { return u256_eq(a, b); }
    /* lookup key: low limb of the Montgomery form (collisions resolved
     * by the full eq() above) */
    static RF_HD uint64_t key(const ctx &, const elt &a) { return a.v[0]; }
};

/* ------------------------------------------------------------------ *
 * Lookup tables.  Probe protocol:
 *     uint32_t pos = T.begin(key);
 *     while (T.valid(pos, key)) { uint32_t j = T.index(pos); ...; pos = T.next(pos, key); }
 * ------------------------------------------------------------------ */
/* Lookup variants (compile-time, all ON by default since the A100
 * measurement of research/ic_gpu_rootfind_lookup_20261114: together 1.12x at
 * the best configuration of both workloads, faster than the old lookups at
 * every configuration; set a flag to 0 for the old path).  Every variant returns
 * the same candidates in the same order, or extra ones the sweep's full-x
 * check discards, so the relation set and its order are unchanged):
 *   RF_SORTED_EYT=1   lower_bound over an Eytzinger (BFS) copy of the keys,
 *                     mapped back to the sorted position by a rank array;
 *   RF_HASH_PACKED=1  one u64 per hash slot (key tag << 32 | index), one load
 *                     per probe step instead of keys[] + vals[];
 *   RF_PROBE_PAIR=1   the two eps lookups of an entry start together
 *                     (begin2), so their dependent loads overlap. */
#ifndef RF_SORTED_EYT
#define RF_SORTED_EYT 1
#endif
#ifndef RF_HASH_PACKED
#define RF_HASH_PACKED 1
#endif
#ifndef RF_PROBE_PAIR
#define RF_PROBE_PAIR 1
#endif

RF_HD int rf_ffs32(uint32_t v) {   /* 1-based index of the lowest set bit, 0 for v == 0 */
#ifdef __CUDA_ARCH__
    return __ffs((int)v);
#else
    return v ? __builtin_ctz(v) + 1 : 0;
#endif
}

/* RF_EYT_PREFETCH = d > 0: while the Eytzinger descent is at node k, prefetch
 * eyt[k << d], the first of the 2^d nodes d levels down.  d = 4 is one 128-byte
 * line of 16 keys (eyt[16k .. 16k+15]; the device copy is 256-byte aligned).
 * RF_EYT_PREFETCH_L2 = 1 prefetches into L2 only.  0 = off (default). */
#ifndef RF_EYT_PREFETCH
#define RF_EYT_PREFETCH 0
#endif
#ifndef RF_EYT_PREFETCH_L2
#define RF_EYT_PREFETCH_L2 0
#endif
RF_HD void rf_prefetch(const void *p) {
#if defined(__CUDA_ARCH__)
#if RF_EYT_PREFETCH_L2
    asm volatile("prefetch.global.L2 [%0];" ::"l"(p));
#else
    asm volatile("prefetch.global.L1 [%0];" ::"l"(p));
#endif
#else
    __builtin_prefetch(p);
#endif
}
#if RF_EYT_PREFETCH
#define RF_EYT_PF(k)                                                             \
    do {                                                                         \
        uint32_t q_ = (k) << RF_EYT_PREFETCH;                                    \
        if (q_ <= n && q_ > (k)) rf_prefetch(eyt + q_);                          \
    } while (0)
#else
#define RF_EYT_PF(k) do { } while (0)
#endif

/* ---- negation-folded pair table, built from the base (RF_LOOKUP_PAIRS_DEVICE) ----
 *
 * The CPU build (ic_boundary PairTable::build_negation_folded) visits, for each
 * abscissa index ai, the pairs (P_ai, Q) with Q in {+P_bi, -P_bi} for bi >= ai, in
 * that order, and keeps the first pair seen for each abscissa of the sum.  Build
 * index b = rf_pair_offset(ai, n) + 2 (bi - ai) + s, s = 0 for +P_bi, 1 for -P_bi;
 * (ai, ai, 1) is P - P, the identity, which takes an index but makes no entry.
 * x, y: the base's representatives (Montgomery form); a_m: the curve's a.       */
RF_HD uint64_t rf_pair_offset(uint64_t ai, uint64_t n) { return 2 * (ai * n - ai * (ai - 1) / 2); }

/* x(P_ai + s P_bi) into *out (Montgomery); 0 when the sum is the identity. */
RF_HD int rf_pair_x(const Fp64Ctx &f, uint64_t a_m, const uint64_t *x, const uint64_t *y,
                    uint32_t ai, uint32_t bi, int s, uint64_t *out) {
    typedef FieldFp64 F;
    const uint64_t x1 = x[ai], y1 = y[ai];
    uint64_t lam;
    if (bi == ai) {
        if (s) return 0;                                         /* P - P */
        const uint64_t x2 = F::sqr(f, x1);
        const uint64_t num = F::add(f, F::add(f, F::add(f, x2, x2), x2), a_m);
        lam = F::mul(f, num, F::inv(f, F::add(f, y1, y1)));     /* 2P (y != 0) */
        *out = F::sub(f, F::sqr(f, lam), F::add(f, x1, x1));
        return 1;
    }
    const uint64_t x2 = x[bi];
    const uint64_t y2 = s ? F::sub(f, 0, y[bi]) : y[bi];
    lam = F::mul(f, F::sub(f, y2, y1), F::inv(f, F::sub(f, x2, x1)));
    *out = F::sub(f, F::sub(f, F::sqr(f, lam), x1), x2);
    return 1;
}

/* The keys of one run of a row: x(P_ai +- P_bi) for bi in [b0, min(n, b0 + RF_PAIR_CHUNK)),
 * written at their build indices (rf_pair_offset), RF_PAIR_EMPTY where the sum is the
 * identity.  The run's denominators (2 y_ai on the diagonal, x_bi - x_ai elsewhere; both
 * signs share one) are inverted together: one field inversion per run instead of two per
 * cell.  The keys are rf_pair_x's, bit for bit (the inverse is unique; a zero denominator,
 * which a base of distinct abscissae with y != 0 never has, falls back to rf_pair_x). */
#ifndef RF_PAIR_CHUNK
#define RF_PAIR_CHUNK 16
#endif
static const uint64_t RF_PAIR_EMPTY = ~0ull;
RF_HD void rf_pair_run(const Fp64Ctx &f, uint64_t a_m, const uint64_t *x, const uint64_t *y, uint32_t n,
                       uint32_t ai, uint32_t b0, uint64_t *keys) {
    typedef FieldFp64 F;
    if (b0 >= n) return;
    const uint32_t m = (n - b0 < RF_PAIR_CHUNK) ? n - b0 : RF_PAIR_CHUNK;
    const uint64_t x1 = x[ai], y1 = y[ai];
    uint64_t d[RF_PAIR_CHUNK], pre[RF_PAIR_CHUNK];
    uint32_t bad = 0;
    /* m >= 1.  Seed the prefix with d[0], not 1*d[0], and avoid the
     * unused final unwind multiplication: two field muls saved per run. */
    uint64_t d0 = (b0 == ai) ? F::add(f, y1, y1) : F::sub(f, x[b0], x1);
    if (d0 == 0) { bad |= 1u; d0 = F::one(f); }
    d[0] = pre[0] = d0;
    uint64_t acc = d0;
    for (uint32_t k = 1; k < m; k++) {
        const uint32_t bi = b0 + k;
        /* b0 >= ai and k >= 1, so this is never the doubling case. */
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
            keys[b + 1] = RF_PAIR_EMPTY; /* P - P */
            continue;
        }
        /* The negative-sign slope is -(y2+y1)/d, whose square is
         * identical to +(y2+y1)^2/d^2. Avoid neg(y2) and share x1+x2
         * across both signs. This saves a negation and one modular
         * reduction per non-diagonal pair without changing its keys. */
        const uint64_t x2 = x[bi], y2 = y[bi];
        const uint64_t xsum = F::add(f, x1, x2);
        const uint64_t lam0 = F::mul(f, F::sub(f, y2, y1), d[k]);
        const uint64_t lam1 = F::mul(f, F::add(f, y2, y1), d[k]);
        keys[b] = F::sub(f, F::sqr(f, lam0), xsum);
        keys[b + 1] = F::sub(f, F::sqr(f, lam1), xsum);
    }
}

struct SortedTable {
    const uint64_t *keys;   /* ascending */
    const uint32_t *idx;    /* factor-base index of keys[pos] */
    uint32_t n;
    const uint64_t *eyt;      /* RF_SORTED_EYT: keys in BFS order, 1-indexed (eyt[0] unused) */
    const uint32_t *eyt_rank; /* RF_SORTED_EYT: sorted position of eyt[k] */

    RF_HD uint32_t begin(uint64_t key) const {
#if RF_SORTED_EYT
        /* Eytzinger lower_bound: descend, then strip the trailing ones of k
         * (the right turns taken after the last left turn). */
        uint32_t k = 1;
        while (k <= n) {
            RF_EYT_PF(k);
            k = 2 * k + (eyt[k] < key ? 1u : 0u);
        }
        k >>= rf_ffs32(~k);
        return k ? eyt_rank[k] : n;
#else
        /* lower_bound: first pos with keys[pos] >= key.  Branch-free body
         * (the loop count is log2(n), uniform across the warp). */
        uint32_t lo = 0, len = n;
        while (len > 0) {
            uint32_t half = len >> 1;
            uint32_t mid = lo + half;
            if (keys[mid] < key) { lo = mid + 1; len = len - half - 1; }
            else len = half;
        }
        return lo;
#endif
    }
    /* Both lookups at once: one loop with two independent load chains. */
    RF_HD void begin2(uint64_t k0, uint64_t k1, uint32_t &p0, uint32_t &p1) const {
#if RF_SORTED_EYT
        /* The bottom level is partial unless n = 2^h - 1, so the two
         * descents can leave the tree one level apart: each advances only
         * while it is inside. */
        uint32_t a = 1, b = 1;
        while (a <= n || b <= n) {
            if (a <= n) { RF_EYT_PF(a); a = 2 * a + (eyt[a] < k0 ? 1u : 0u); }
            if (b <= n) { RF_EYT_PF(b); b = 2 * b + (eyt[b] < k1 ? 1u : 0u); }
        }
        a >>= rf_ffs32(~a);
        b >>= rf_ffs32(~b);
        p0 = a ? eyt_rank[a] : n;
        p1 = b ? eyt_rank[b] : n;
#else
        uint32_t lo0 = 0, lo1 = 0, len0 = n, len1 = n;
        while (len0 > 0 || len1 > 0) {
            if (len0 > 0) {
                uint32_t h = len0 >> 1, m = lo0 + h;
                if (keys[m] < k0) { lo0 = m + 1; len0 = len0 - h - 1; } else len0 = h;
            }
            if (len1 > 0) {
                uint32_t h = len1 >> 1, m = lo1 + h;
                if (keys[m] < k1) { lo1 = m + 1; len1 = len1 - h - 1; } else len1 = h;
            }
        }
        p0 = lo0;
        p1 = lo1;
#endif
    }
    RF_HD int valid(uint32_t pos, uint64_t key) const { return pos < n && keys[pos] == key; }
    RF_HD uint32_t index(uint32_t pos) const { return idx[pos]; }
    RF_HD uint32_t next(uint32_t pos, uint64_t) const { return pos + 1; }
};

RF_HD uint32_t rf_hash64(uint64_t key, uint32_t mask) {
    /* Fibonacci multiplicative hash; the Montgomery form of a small-x
     * factor base is already well scrambled, but random-point factor
     * bases need the mixing step. */
    uint64_t h = key * 0x9E3779B97F4A7C15ull;
    return (uint32_t)(h >> 32) & mask;
}

struct HashTable {
    const uint64_t *keys;
    const uint32_t *vals;   /* RF_NO_INDEX marks an empty slot */
    uint32_t mask;          /* capacity - 1, capacity a power of two */
    const uint64_t *slots;  /* RF_HASH_PACKED: (uint32 tag << 32) | index, low word RF_NO_INDEX if empty */

    static RF_HD uint32_t tag(uint64_t key) { return (uint32_t)key; }
#if RF_HASH_PACKED
    /* Same slots and probe order as the split arrays; a slot whose tag
     * matches but whose key differs is returned as a candidate and dropped by
     * the sweep's full-x comparison, so the emitted relations are unchanged. */
    RF_HD uint32_t skip_mismatch(uint32_t slot, uint64_t key) const {
        const uint32_t t = tag(key);
        for (;;) {
            const uint64_t v = slots[slot];
            if ((uint32_t)v == RF_NO_INDEX || (uint32_t)(v >> 32) == t) return slot;
            slot = (slot + 1) & mask;
        }
    }
    RF_HD int valid(uint32_t pos, uint64_t) const { return (uint32_t)slots[pos] != RF_NO_INDEX; }
    RF_HD uint32_t index(uint32_t pos) const { return (uint32_t)slots[pos]; }
#else
    RF_HD uint32_t skip_mismatch(uint32_t slot, uint64_t key) const {
        /* advance over occupied slots whose key differs */
        while (vals[slot] != RF_NO_INDEX && keys[slot] != key) slot = (slot + 1) & mask;
        return slot;
    }
    RF_HD int valid(uint32_t pos, uint64_t key) const { return vals[pos] != RF_NO_INDEX; }
    RF_HD uint32_t index(uint32_t pos) const { return vals[pos]; }
#endif
    RF_HD uint32_t begin(uint64_t key) const { return skip_mismatch(rf_hash64(key, mask), key); }
    RF_HD void begin2(uint64_t k0, uint64_t k1, uint32_t &p0, uint32_t &p1) const {
#if RF_HASH_PACKED
        /* issue both first-slot loads before either is examined */
        const uint32_t s0 = rf_hash64(k0, mask), s1 = rf_hash64(k1, mask);
        const uint64_t v0 = slots[s0], v1 = slots[s1];
        const uint32_t t0 = tag(k0), t1 = tag(k1);
        p0 = ((uint32_t)v0 == RF_NO_INDEX || (uint32_t)(v0 >> 32) == t0) ? s0 : skip_mismatch((s0 + 1) & mask, k0);
        p1 = ((uint32_t)v1 == RF_NO_INDEX || (uint32_t)(v1 >> 32) == t1) ? s1 : skip_mismatch((s1 + 1) & mask, k1);
#else
        p0 = begin(k0);
        p1 = begin(k1);
#endif
    }
    /* keep skipping mismatching occupied slots after a candidate */
    RF_HD uint32_t next(uint32_t pos, uint64_t key) const { return skip_mismatch((pos + 1) & mask, key); }
};

/* ------------------------------------------------------------------ *
 * Kernel arguments (POD, passed by value).
 * ------------------------------------------------------------------ */
template <class F>
struct DecompArgs {
    typename F::ctx f;
    const typename F::elt *fb_x;   /* Montgomery form, SoA */
    const typename F::elt *fb_y;
    uint32_t n_fb;
    const typename F::elt *t_x;    /* targets, Montgomery form */
    const typename F::elt *t_y;
    uint32_t n_targets;
    rf_relation *out;
    uint32_t *out_count;           /* atomic; may exceed out_cap */
    uint32_t out_cap;
    /* Probe mode (probe_mode = 1): candidates are looked up in the set
     * probe_x (Montgomery form; the lookup table is built on it) instead of
     * the factor base, every hit is emitted whatever its index (no j >= i),
     * and tau is +1 (the caller re-derives the summands).  This is the
     * three-summand sweep against the abscissae of a pair table.  With
     * probe_mode = 0, probe_x must equal fb_x. */
    const typename F::elt *probe_x;
    uint32_t probe_mode;
};

RF_HD uint32_t rf_atomic_inc(uint32_t *ctr) {
#ifdef __CUDA_ARCH__
    return atomicAdd(ctr, 1u);
#else
    return __atomic_fetch_add(ctr, 1u, __ATOMIC_RELAXED);
#endif
}

RF_HD void rf_emit(rf_relation *out, uint32_t *ctr, uint32_t cap,
                   uint32_t target, uint32_t i, uint32_t j, int8_t eps, int8_t tau) {
    uint32_t slot = rf_atomic_inc(ctr);
    if (slot < cap) {
        rf_relation r;
        r.target = target; r.i = i; r.j = j; r.eps = eps; r.tau = tau; r.pad[0] = r.pad[1] = 0;
        out[slot] = r;
    }
}

/* 64-bit: n_targets and ceil(n_fb / chunk) are independent uint32
 * counts, and their product is exactly 2^32 at ~2^20 targets,
 * |F| = 2^16, chunk = 16.  A uint32 multiply wraps to 0 there. */
RF_HD uint64_t rf_n_chunks(uint32_t n_fb, int chunk) {
    return ((uint64_t)n_fb + (uint64_t)chunk - 1) / (uint64_t)chunk;
}

RF_HD uint64_t rf_thread_count(uint32_t n_targets, uint32_t n_fb, int chunk) {
    return (uint64_t)n_targets * rf_n_chunks(n_fb, chunk);
}

/* Map linear tid to target and factor-base chunk.
 * Most host batches contain fewer than 2^32 logical threads, so 32-bit
 * division is sufficient and substantially cheaper on x86-64.  Keep a
 * 64-bit fallback for large jobs (the product of target and chunk counts
 * can exceed 2^32); CUDA retains its previous uniform mapping. */
RF_HD void rf_thread_map(uint64_t tid, uint32_t n_targets, uint32_t chunk,
                         uint32_t &target, uint32_t &i0) {
#ifdef __CUDA_ARCH__
    target = (uint32_t)(tid % (uint64_t)n_targets);
    i0 = (uint32_t)(tid / (uint64_t)n_targets) * chunk;
#else
    if (tid <= 0xFFFFFFFFull) {
        const uint32_t t = (uint32_t)tid;
        target = t % n_targets;
        i0 = (t / n_targets) * chunk;
    } else {
        target = (uint32_t)(tid % (uint64_t)n_targets);
        i0 = (uint32_t)(tid / (uint64_t)n_targets) * chunk;
    }
#endif
}

/* ------------------------------------------------------------------ *
 * The per-thread oracle.  CHUNK must be a multiple of 8 and <= 64.
 * ------------------------------------------------------------------ */
/* ------------------------------------------------------------------ *
 * Opt-in phase profile (RF_PROFILE=1, device only).  clock64() at the
 * phase boundaries of rf_decompose_thread; one thread in 1024 (tid % 1024
 * == 0) adds its cycles to rf_prof_cycles[] with one atomic per phase, so
 * the totals are a sample of where a thread's cycles go, not a timing.
 * Slots: 0 load d[], 1 prefix products, 2 inversion, 3 unwind, 4 sweep
 * (all of it), 5 table probes inside the sweep, 6 sampled threads.
 * RF_PROFILE=0 (default) compiles every macro to nothing.
 * ------------------------------------------------------------------ */
#ifndef RF_PROFILE
#define RF_PROFILE 0
#endif
#if RF_PROFILE && defined(__CUDACC__)
static __device__ unsigned long long rf_prof_cycles[8];
#endif
#if RF_PROFILE && defined(__CUDA_ARCH__)
#define RF_PROF_BEGIN(tid) long long rf_pa[6] = {0, 0, 0, 0, 0, 0}; long long rf_pt = clock64(); \
    const bool rf_ps = ((tid) & 1023u) == 0
#define RF_PROF_STEP(slot) do { long long rf_now = clock64(); rf_pa[slot] += rf_now - rf_pt; rf_pt = rf_now; } while (0)
#define RF_PROF_PROBE_START() long long rf_pp = clock64()
#define RF_PROF_PROBE_END() (rf_pa[5] += clock64() - rf_pp)
#define RF_PROF_FLUSH() do { if (rf_ps) { for (int rf_s = 0; rf_s < 6; rf_s++) \
    atomicAdd(&rf_prof_cycles[rf_s], (unsigned long long)rf_pa[rf_s]); atomicAdd(&rf_prof_cycles[6], 1ull); } } while (0)
#else
#define RF_PROF_BEGIN(tid) do { } while (0)
#define RF_PROF_STEP(slot) do { } while (0)
#define RF_PROF_PROBE_START() do { } while (0)
#define RF_PROF_PROBE_END() do { } while (0)
#define RF_PROF_FLUSH() do { } while (0)
#endif

template <class F, class T, int CHUNK>
RF_HD void rf_decompose_thread(const DecompArgs<F> &a, const T &table, uint64_t tid) {
    typedef typename F::elt elt;
    const typename F::ctx &f = a.f;
    if (tid >= rf_thread_count(a.n_targets, a.n_fb, CHUNK)) return;
    uint32_t k, i0;
    rf_thread_map(tid, a.n_targets, (uint32_t)CHUNK, k, i0);
    const uint32_t cnt = (a.n_fb - i0 < (uint32_t)CHUNK) ? (a.n_fb - i0) : (uint32_t)CHUNK;

    RF_PROF_BEGIN(tid);
    const elt xr = a.t_x[k];
    const elt yr = a.t_y[k];
    const elt one = F::one(f);

    /* d[w] = x_R - x_i, with degenerate (zero) and padding entries
     * replaced by 1 so the fixed-trip-count batch inversion below stays
     * fully unrolled and register-resident. */
    elt d[CHUNK];
    elt pre[CHUNK];
    uint64_t degenerate = 0;
#pragma unroll
    for (int w = 0; w < CHUNK; w++) {
        if ((uint32_t)w < cnt) {
            elt dx = F::sub(f, xr, a.fb_x[i0 + w]);
            int z = F::is_zero(f, dx);
            degenerate |= (uint64_t)(z != 0) << w;
            d[w] = z ? one : dx;
        } else {
            d[w] = one;
        }
    }
    RF_PROF_STEP(0);
    /* Montgomery's trick: prefix products, one inversion, unwind.
     * CHUNK >= 8. Host tail chunks that are <= half full omit padding
     * multiplies entirely. Full/near-full chunks retain the original
     * fixed length (fewer loop-control penalties); CUDA is unchanged
     * for uniform unrolling and warp scheduling.  cnt >= 1 for valid tid. */
#ifdef __CUDA_ARCH__
    const int inv_len = CHUNK;
#else
    const int inv_len = cnt <= (uint32_t)(CHUNK / 2) ? (int)cnt : CHUNK;
#endif
    elt acc = d[0];
#pragma unroll
    for (int w = 1; w < inv_len; w++) {
        pre[w] = acc;
        acc = F::mul(f, acc, d[w]);
    }
    RF_PROF_STEP(1);
    elt inv_acc = F::inv(f, acc);
    RF_PROF_STEP(2);
#pragma unroll
    for (int w = inv_len - 1; w > 0; w--) {
        elt dw = d[w];
        d[w] = F::mul(f, inv_acc, pre[w]);
        inv_acc = F::mul(f, inv_acc, dw);
    }
    d[0] = inv_acc;
    RF_PROF_STEP(3);

    /* The sweep.  Factor-base loads are warp-uniform (every lane holds
     * the same slice), target values live in registers. */
    for (uint32_t w = 0; w < cnt; w++) {
        const uint32_t i = i0 + w;
        const elt xi = a.fb_x[i];
        const elt yi = a.fb_y[i];
        if ((degenerate >> w) & 1) {
            int8_t eps = F::eq(f, yr, yi) ? (int8_t)1 : (int8_t)-1;
            rf_emit(a.out, a.out_count, a.out_cap, k, i, i, eps, 0);
            continue;
        }
        const elt inv = d[w];
        const elt lam_m = F::mul(f, F::add(f, yr, yi), inv);   /* eps = +1: R - F_i */
        const elt lam_p = F::mul(f, F::sub(f, yr, yi), inv);   /* eps = -1: R + F_i */
        /* x3 = lam^2 - (xr+xi), for both signs: one shared modular
         * addition replaces two independent pairs of subtractions. */
        const elt xsum = F::add(f, xr, xi);
#if RF_PROBE_PAIR
        /* both lookups start before either is walked (begin2); the walks
         * below then run eps = +1 first, as in the unpaired loop */
        const elt x3_m = F::sub(f, F::sqr(f, lam_m), xsum);
        const elt x3_p = F::sub(f, F::sqr(f, lam_p), xsum);
        uint32_t pos_m, pos_p;
        {
            RF_PROF_PROBE_START();
            table.begin2(F::key(f, x3_m), F::key(f, x3_p), pos_m, pos_p);
            RF_PROF_PROBE_END();
        }
#endif
#pragma unroll
        for (int s = 0; s < 2; s++) {
            const int8_t eps = s == 0 ? (int8_t)1 : (int8_t)-1;
            const elt lam = s == 0 ? lam_m : lam_p;
#if RF_PROBE_PAIR
            const elt x3 = s == 0 ? x3_m : x3_p;
            const uint64_t key = F::key(f, x3);
            RF_PROF_PROBE_START();
            uint32_t pos = s == 0 ? pos_m : pos_p;
#else
            const elt x3 = F::sub(f, F::sqr(f, lam), xsum);
            const uint64_t key = F::key(f, x3);
            RF_PROF_PROBE_START();
            uint32_t pos = table.begin(key);
#endif
            while (table.valid(pos, key)) {
                const uint32_t j = table.index(pos);
                pos = table.next(pos, key);
                if (a.probe_mode) {
                    if (!F::eq(f, x3, a.probe_x[j])) continue;   /* key collision */
                    rf_emit(a.out, a.out_count, a.out_cap, k, i, j, eps, 1);
                    continue;
                }
                if (j < i) continue;
                if (!F::eq(f, x3, a.fb_x[j])) continue;   /* 64-bit key collision (P-256 only) */
                const elt y3 = F::sub(f, F::mul(f, lam, F::sub(f, xr, x3)), yr);
                const int8_t tau = F::eq(f, y3, a.fb_y[j]) ? (int8_t)1 : (int8_t)-1;
                rf_emit(a.out, a.out_count, a.out_cap, k, i, j, eps, tau);
            }
            RF_PROF_PROBE_END();
        }
    }
    RF_PROF_STEP(4);
    RF_PROF_FLUSH();
}

#endif /* GPU_ROOTFIND_ROOTFIND_CUH */
