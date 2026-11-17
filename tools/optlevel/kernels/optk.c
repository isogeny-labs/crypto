/* optk: calibration and control kernels for tools/optlevel.
 *
 * One process, one thread, one kernel.  The kernel repeats a pass over its
 * arrays until --seconds of wall time has elapsed, then prints one line of
 * JSON in the optlevel.ops/v1 protocol (docs/OPTIMIZATION_LEVEL.md, section
 * "Operation-count protocol"): the counted operations, the seconds spent
 * inside the timed region, the working set, and the bytes the loop moves
 * ("bytes_declared": loads, stores and write-allocate reads).  The scorer
 * charges those bytes to DRAM only when the working set exceeds the
 * last-level cache.
 *
 * The same source is built twice by tools/optlevel/optlevel.py:
 *   optk         -O3 -march=native                 (SIMD ceiling)
 *   optk_scalar  -O3 -march=native -fno-tree-vectorize -fno-tree-slp-vectorize
 * so the scalar/SIMD ratio of a class is measured, not assumed.
 *
 * Every kernel folds its result into a checksum the caller can compare
 * across builds; a pass the compiler removed cannot report a rate.
 */
#define _GNU_SOURCE
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

typedef unsigned __int128 u128;
#define P31 2013265921u            /* 15 * 2^27 + 1 */
#define P31_NINV 2013265919u       /* -P31^-1 mod 2^32 */
#define P62 4611686018427387847ull /* 2^62 - 57 */

static double now(void) {
    struct timespec t;
    clock_gettime(CLOCK_MONOTONIC, &t);
    return (double)t.tv_sec + 1e-9 * (double)t.tv_nsec;
}
static uint64_t rng_state = 0x9E3779B97F4A7C15ull;
static uint64_t rng(void) { /* splitmix64 */
    uint64_t z = (rng_state += 0x9E3779B97F4A7C15ull);
    z = (z ^ (z >> 30)) * 0xBF58476D1CE4E5B9ull;
    z = (z ^ (z >> 27)) * 0x94D049BB133111EBull;
    return z ^ (z >> 31);
}
static void *xalloc(size_t bytes) {
    void *p = NULL;
    if (posix_memalign(&p, 4096, bytes ? bytes : 4096)) { fprintf(stderr, "optk: out of memory\n"); exit(2); }
    return p;
}
#define BARRIER(x) __asm__ volatile("" : "+r"(x))

/* Montgomery product mod P31 for a, b < P31; result < P31.  Written so that
 * the loop over it vectorizes with widening 32x32->64 multiplies. */
static inline uint32_t mont31(uint32_t a, uint32_t b) {
    uint64_t t = (uint64_t)a * b;
    uint32_t q = (uint32_t)t * P31_NINV;
    uint32_t r = (uint32_t)((t + (uint64_t)q * P31) >> 32);
    return r >= P31 ? r - P31 : r;
}
static inline uint32_t add31(uint32_t a, uint32_t b) {
    uint32_t s = a + b;
    return s >= P31 ? s - P31 : s;
}
static uint64_t p62_ninv;
static inline uint64_t mont62(uint64_t a, uint64_t b) {
    u128 t = (u128)a * b;
    uint64_t q = (uint64_t)t * p62_ninv;
    uint64_t r = (uint64_t)((t + (u128)q * P62) >> 64);
    return r >= P62 ? r - P62 : r;
}

/* ---- passes: each performs n operations --------------------------------- */
__attribute__((noinline)) static void pass_muladd_u32(uint32_t *a, const uint32_t *c, uint32_t m, size_t n) {
    for (size_t i = 0; i < n; i++) a[i] = a[i] * m + c[i];
}
__attribute__((noinline)) static void pass_muladd_u64(uint64_t *a, const uint64_t *c, uint64_t m, size_t n) {
    for (size_t i = 0; i < n; i++) a[i] = a[i] * m + c[i];
}
__attribute__((noinline)) static void pass_mulwide_u64(uint64_t *a, const uint64_t *c, uint64_t m, size_t n) {
    for (size_t i = 0; i < n; i++) a[i] = (uint64_t)(((u128)a[i] * m) >> 64) + c[i];
}
/* row operation a <- a + m*b mod p: the inner loop of dense elimination */
__attribute__((noinline)) static void pass_modmuladd_p31(uint32_t *a, const uint32_t *b, uint32_t m, size_t n) {
    for (size_t i = 0; i < n; i++) a[i] = add31(a[i], mont31(b[i], m));
}
__attribute__((noinline)) static void pass_modmuladd_p62(uint64_t *a, const uint64_t *b, uint64_t m, size_t n) {
    for (size_t i = 0; i < n; i++) {
        uint64_t s = a[i] + mont62(b[i], m);
        a[i] = s >= P62 ? s - P62 : s;
    }
}
/* the same arithmetic with a serialising dependency: Horner evaluation */
__attribute__((noinline)) static uint32_t pass_horner_p31(const uint32_t *b, uint32_t m, size_t n, uint32_t acc) {
    for (size_t i = 0; i < n; i++) acc = add31(mont31(acc, m), b[i]);
    return acc;
}
__attribute__((noinline)) static void pass_xor_u64(uint64_t *a, const uint64_t *b, size_t n) {
    for (size_t i = 0; i < n; i++) a[i] ^= b[i];
}
__attribute__((noinline)) static uint64_t pass_popcnt_u64(const uint64_t *a, const uint64_t *b, size_t n) {
    uint64_t s = 0;
    for (size_t i = 0; i < n; i++) s += (uint64_t)__builtin_popcountll(a[i] & b[i]);
    return s;
}
__attribute__((noinline)) static void pass_fma_f64(double *a, const double *b, double s, size_t n) {
    for (size_t i = 0; i < n; i++) a[i] = a[i] * s + b[i];
}
__attribute__((noinline)) static void pass_triad(double *a, const double *b, const double *c, double s, size_t n) {
    for (size_t i = 0; i < n; i++) a[i] = b[i] + s * c[i];
}
/* register-resident FMA: 16 independent vector accumulators, no loads or
 * stores in the loop; the port-limited peak a blocked matrix product can
 * approach.  n counts scalar FMAs and must be a multiple of 128. */
typedef double v8d __attribute__((vector_size(64)));
__attribute__((noinline)) static double pass_fma_reg(size_t n, double s0) {
    v8d acc[16], s, c;
    for (int j = 0; j < 8; j++) { s[j] = s0; c[j] = 1.0 + j; }
    for (int i = 0; i < 16; i++) for (int j = 0; j < 8; j++) acc[i][j] = i + j;
    for (size_t it = 0; it < n / 128; it++) {
        for (int i = 0; i < 16; i++) acc[i] = acc[i] * s + c;
        BARRIER(it);
    }
    double r = 0;
    for (int i = 0; i < 16; i++) for (int j = 0; j < 8; j++) r += acc[i][j];
    return r;
}
struct node { struct node *next; uint64_t pad[7]; };
__attribute__((noinline)) static struct node *pass_chase(struct node *p, size_t n) {
    for (size_t i = 0; i < n; i++) p = p->next;
    return p;
}
__attribute__((noinline)) static uint64_t pass_clock(uint64_t x, size_t n) {
    for (size_t i = 0; i < n; i++) { x += 1; BARRIER(x); }
    return x;
}

static void usage(void) {
    fprintf(stderr,
        "usage: optk KERNEL [--n ELEMENTS] [--seconds S]\n"
        "kernels: muladd_u32 muladd_u64 mulwide_u64 modmuladd_p31 modmuladd_p62 horner_p31\n"
        "         xor_u64 popcnt_u64 fma_f64 fma_f64_reg triad chase clock\n");
    exit(2);
}

int main(int argc, char **argv) {
    if (argc < 2) usage();
    const char *k = argv[1];
    size_t n = 0;
    double seconds = 1.0;
    for (int i = 2; i < argc; i++) {
        if (!strcmp(argv[i], "--n") && i + 1 < argc) n = (size_t)strtoull(argv[++i], NULL, 10);
        else if (!strcmp(argv[i], "--seconds") && i + 1 < argc) seconds = atof(argv[++i]);
        else usage();
    }
    const char *cls = k;
    double bytes_per_op = 0; /* traffic one op implies once the arrays exceed cache */
    size_t ws = 0;
    uint64_t ops = 0, sum = 0;
    double t0 = 0, t1 = 0;
    p62_ninv = 1; /* Newton: -P62^-1 mod 2^64 */
    for (int i = 0; i < 6; i++) p62_ninv *= 2 - P62 * p62_ninv;
    p62_ninv = (uint64_t)0 - p62_ninv;

#define TIMED(BODY) do { BODY; /* warm-up, untimed: page faults, caches */ \
        t0 = now(); do { BODY; ops += n; t1 = now(); } while (t1 - t0 < seconds); } while (0)

    if (!strcmp(k, "muladd_u32")) {
        if (!n) n = 2048;
        uint32_t *a = xalloc(4 * n), *c = xalloc(4 * n); ws = 8 * n; bytes_per_op = 12;
        for (size_t i = 0; i < n; i++) { a[i] = (uint32_t)rng(); c[i] = (uint32_t)rng(); }
        TIMED(pass_muladd_u32(a, c, 2654435761u, n));
        for (size_t i = 0; i < n; i++) sum += a[i];
    } else if (!strcmp(k, "muladd_u64") || !strcmp(k, "mulwide_u64")) {
        if (!n) n = 1024;
        uint64_t *a = xalloc(8 * n), *c = xalloc(8 * n); ws = 16 * n; bytes_per_op = 24;
        for (size_t i = 0; i < n; i++) { a[i] = rng(); c[i] = rng(); }
        if (k[3] == 'a') TIMED(pass_muladd_u64(a, c, 0x9E3779B97F4A7C15ull, n));
        else TIMED(pass_mulwide_u64(a, c, 0x9E3779B97F4A7C15ull, n));
        for (size_t i = 0; i < n; i++) sum += a[i];
    } else if (!strcmp(k, "modmuladd_p31")) {
        if (!n) n = 2048;
        uint32_t *a = xalloc(4 * n), *b = xalloc(4 * n); ws = 8 * n; bytes_per_op = 12;
        for (size_t i = 0; i < n; i++) { a[i] = (uint32_t)(rng() % P31); b[i] = (uint32_t)(rng() % P31); }
        TIMED(pass_modmuladd_p31(a, b, 1234567891u, n));
        for (size_t i = 0; i < n; i++) sum += a[i];
    } else if (!strcmp(k, "modmuladd_p62")) {
        if (!n) n = 1024;
        uint64_t *a = xalloc(8 * n), *b = xalloc(8 * n); ws = 16 * n; bytes_per_op = 24;
        for (size_t i = 0; i < n; i++) { a[i] = rng() % P62; b[i] = rng() % P62; }
        TIMED(pass_modmuladd_p62(a, b, 0x123456789ABCDEFull, n));
        for (size_t i = 0; i < n; i++) sum += a[i];
    } else if (!strcmp(k, "horner_p31")) {
        cls = "modmuladd_p31";
        if (!n) n = 2048;
        uint32_t *b = xalloc(4 * n), acc = 1; ws = 4 * n; bytes_per_op = 4;
        for (size_t i = 0; i < n; i++) b[i] = (uint32_t)(rng() % P31);
        TIMED(acc = pass_horner_p31(b, 1234567891u, n, acc));
        sum = acc;
    } else if (!strcmp(k, "xor_u64") || !strcmp(k, "popcnt_u64")) {
        if (!n) n = 1024;
        uint64_t *a = xalloc(8 * n), *b = xalloc(8 * n); ws = 16 * n;
        for (size_t i = 0; i < n; i++) { a[i] = rng(); b[i] = rng(); }
        if (k[0] == 'x') { bytes_per_op = 24; TIMED(pass_xor_u64(a, b, n)); for (size_t i = 0; i < n; i++) sum += a[i]; }
        else { bytes_per_op = 16; TIMED(sum += pass_popcnt_u64(a, b, n)); }
    } else if (!strcmp(k, "fma_f64")) {
        if (!n) n = 1024;
        double *a = xalloc(8 * n), *b = xalloc(8 * n); ws = 16 * n; bytes_per_op = 24;
        for (size_t i = 0; i < n; i++) { a[i] = (double)(rng() >> 40); b[i] = (double)(rng() >> 40); }
        TIMED(pass_fma_f64(a, b, 0.5, n));
        for (size_t i = 0; i < n; i++) sum += (uint64_t)a[i];
    } else if (!strcmp(k, "fma_f64_reg")) {
        cls = "fma_f64";
        if (!n) n = 128 << 13;
        n -= n % 128;
        double r = 0;
        TIMED(r += pass_fma_reg(n, 0.5));
        sum = (uint64_t)r;
    } else if (!strcmp(k, "triad")) {
        /* STREAM triad.  bytes_declared counts the traffic the loop really
         * causes with ordinary stores: two reads, one write and the
         * write-allocate read of the destination line, 32 bytes per element
         * (STREAM itself reports 24). */
        cls = "fma_f64";
        if (!n) n = (size_t)48 << 20;
        double *a = xalloc(8 * n), *b = xalloc(8 * n), *c = xalloc(8 * n); ws = 24 * n; bytes_per_op = 32;
        for (size_t i = 0; i < n; i++) { a[i] = 0; b[i] = (double)(i & 1023); c[i] = 1.0; }
        TIMED(pass_triad(a, b, c, 3.0, n));
        for (size_t i = 0; i < n; i += 4097) sum += (uint64_t)a[i];
    } else if (!strcmp(k, "chase")) {
        /* one dependent load per operation over a random single cycle of
         * 64-byte nodes (Sattolo's algorithm) */
        cls = "ptr_hop";
        if (!n) n = 256;
        struct node *v = xalloc(sizeof(struct node) * n); ws = sizeof(struct node) * n; bytes_per_op = 64;
        size_t *perm = xalloc(sizeof(size_t) * n);
        for (size_t i = 0; i < n; i++) perm[i] = i;
        for (size_t i = n - 1; i > 0; i--) { size_t j = rng() % i, t = perm[i]; perm[i] = perm[j]; perm[j] = t; }
        for (size_t i = 0; i < n; i++) { v[i].next = &v[perm[i]]; v[i].pad[0] = i; }
        free(perm);
        struct node *p = v;
        TIMED(p = pass_chase(p, n));
        sum = p->pad[0];
    } else if (!strcmp(k, "clock")) {
        /* dependent 1-cycle additions: operations per second = effective core clock */
        cls = "clock";
        if (!n) n = 1 << 20;
        uint64_t x = 0;
        TIMED(x = pass_clock(x, n));
        sum = x;
    } else usage();

    printf("{\"schema\":\"optlevel.ops/v1\",\"kernel\":\"%s\",\"op_class\":\"%s\",\"ops\":%llu,"
           "\"kernel_seconds\":%.9f,\"bytes_declared\":%.0f,\"working_set_bytes\":%zu,\"elements\":%zu,"
           "\"checksum\":\"%016llx\"}\n",
           k, cls, (unsigned long long)ops, t1 - t0, bytes_per_op * (double)ops, ws, n, (unsigned long long)sum);
    return 0;
}
