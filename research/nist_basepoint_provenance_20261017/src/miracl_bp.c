// MIRACL base-point test. brand(): Marsaglia-Zaman subtract-with-borrow on NK 32-bit words (lag NJ),
// irand(seed): fill per mrcore.c, then 1000 warm-up calls; default (unseeded) state per mirsys().
// bigrand(p): words appended least-significant first until w0 >= p, then w0 mod p; base 2^32 or 10^9
// (r mod base). Base point x = bigrand(p) + j, j in [0, 256), at every stream offset < OFFMAX.
// Parameter sets: (NK,NJ,NV) = (37,24,14) current and (21,6,8) older.
#include <gmp.h>
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <string.h>

typedef struct { int nk, nj, nv; uint32_t ira[64]; uint32_t borrow; int ptr; } rng_t;

static uint32_t brand(rng_t *g) {
    g->ptr++;
    if (g->ptr < g->nk) return g->ira[g->ptr];
    g->ptr = 0;
    for (int i = 0, k = g->nk - g->nj; i < g->nk; i++, k++) {
        if (k == g->nk) k = 0;
        uint32_t t = g->ira[k], pdiff = t - g->ira[i] - g->borrow;
        if (pdiff < t) g->borrow = 0;
        if (pdiff > t) g->borrow = 1;
        g->ira[i] = pdiff;
    }
    return g->ira[0];
}
static void irand(rng_t *g, uint32_t seed) {
    uint32_t m = 1, t;
    g->borrow = 0; g->ptr = 0; g->ira[0] = seed;
    for (int i = 1; i < g->nk; i++) { int in = (g->nv * i) % g->nk; g->ira[in] = m; t = m; m = seed - m; seed = t; }
    for (int i = 0; i < 1000; i++) brand(g);
}
static void dflt(rng_t *g) {
    g->ira[0] = 0x55555555u; g->ira[1] = 0x12345678u;
    for (int i = 2; i < g->nk; i++) g->ira[i] = g->ira[i - 1] + g->ira[i - 2] + 0x1379BDF1u;
    g->ptr = g->nk; g->borrow = 0;
}

typedef struct { const char *name, *p_hex, *gx_hex; mpz_t p, gx; } curve_t;
static curve_t C[] = {
 {"P-224", "ffffffffffffffffffffffffffffffff000000000000000000000001", "b70e0cbd6bb4bf7f321390b94a03c1d356c21122343280d6115c1d21"},
 {"P-256", "ffffffff00000001000000000000000000000000ffffffffffffffffffffffff", "6b17d1f2e12c4247f8bce6e563a440f277037d812deb33a0f4a13945d898c296"},
 {"P-384", "fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffeffffffff0000000000000000ffffffff", "aa87ca22be8b05378eb1c71ef320ad746e1d3b628ba79b9859f741e082542a385502f25dbf55296c3a545e3872760ab7"},
 {"P-521", "1ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff", "c6858e06b70404e9cd9e3ecb662395b4429c648139053fb521f828af606b4d3dbaa14b5e77efe75928fe1dc127a2ffa8de3348b3c1856a429bf97e7e31c2e5bd66"},
 {"CONTROL P-256 (37,24,14) seed 12345 base 2^32 off 40 j 3", "ffffffff00000001000000000000000000000000ffffffffffffffffffffffff", NULL},
 {"CONTROL P-384 (21,6,8) default state base 10^9 off 70000 j 0", "fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffeffffffff0000000000000000ffffffff", NULL},
};
#define NC 6
static const int PS[2][3] = {{37, 24, 14}, {21, 6, 8}};
static const uint64_t BASES[2] = {4294967296ULL, 1000000000ULL};

// bigrand from words[0..], returns words used; x = w0 mod p
static int bigrand(mpz_t x, mpz_t tmp, const uint32_t *w, int avail, uint64_t base, const mpz_t p) {
    mpz_set_ui(x, 0); mpz_set_ui(tmp, 1); int m = 0;
    do {
        if (m >= avail) return -1;
        uint64_t r = base == 4294967296ULL ? w[m] : (w[m] % base);
        mpz_addmul_ui(x, tmp, r); mpz_mul_ui(tmp, tmp, base); m++;
    } while (mpz_cmp(x, p) < 0);
    mpz_mod(x, x, p); return m;
}

static long check_stream(const uint32_t *w, long n, long offmax, int ps, const char *tag, unsigned long seed) {
    long hits = 0; mpz_t x, tmp, d; mpz_inits(x, tmp, d, NULL);
    for (long off = 0; off < offmax; off++)
        for (int b = 0; b < 2; b++)
            for (int c = 0; c < NC; c++) {
                if (bigrand(x, tmp, w + off, (int)(n - off), BASES[b], C[c].p) < 0) continue;
                mpz_sub(d, C[c].gx, x); mpz_mod(d, d, C[c].p);
                if (mpz_cmp_ui(d, 256) < 0) {
                    #pragma omp critical
                    printf("HIT %-58s %s seed=%lu params=(%d,%d,%d) base=%llu offset=%ld j=%lu\n", C[c].name, tag, seed,
                           PS[ps][0], PS[ps][1], PS[ps][2], (unsigned long long)BASES[b], off, mpz_get_ui(d));
                    hits++;
                }
            }
    mpz_clears(x, tmp, d, NULL); return hits;
}

int main(int argc, char **argv) {
    for (int c = 0; c < NC; c++) { mpz_init_set_str(C[c].p, C[c].p_hex, 16); mpz_init(C[c].gx); if (C[c].gx_hex) mpz_set_str(C[c].gx, C[c].gx_hex, 16); }
    // controls
    { rng_t g = {37, 24, 14}; irand(&g, 12345); uint32_t w[200]; for (int i = 0; i < 200; i++) w[i] = brand(&g);
      mpz_t t; mpz_init(t); bigrand(C[4].gx, t, w + 40, 160, BASES[0], C[4].p); mpz_add_ui(C[4].gx, C[4].gx, 3); mpz_clear(t); }
    { rng_t g = {21, 6, 8}; dflt(&g); static uint32_t w[70100]; for (int i = 0; i < 70100; i++) w[i] = brand(&g);
      mpz_t t; mpz_init(t); bigrand(C[5].gx, t, w + 70000, 100, BASES[1], C[5].p); mpz_clear(t); }
    long hits = 0;
    // 1. default (unseeded) state, offsets < 2^20
    for (int ps = 0; ps < 2; ps++) {
        long N = (1L << 20) + 200; uint32_t *w = malloc(N * 4);
        rng_t g = {PS[ps][0], PS[ps][1], PS[ps][2]}; dflt(&g);
        for (long i = 0; i < N; i++) w[i] = brand(&g);
        long chunk = (1L << 20) / 64;
        #pragma omp parallel for reduction(+:hits) schedule(dynamic, 1)
        for (int q = 0; q < 64; q++) hits += check_stream(w + q * chunk, chunk + 200, chunk, ps, "default", 0);
        free(w);
    }
    fprintf(stderr, "default state done\n");
    // 2. irand(seed): seeds 0..2^22, and the four run timestamps +-1 day (t and t/2), offsets < 256
    static const unsigned long runs[4] = {891607927UL, 871696354UL, 872952608UL, 871002724UL};
    long nlist = (1L << 22) + 1 + 4 * 2 * 172801; unsigned long *S = malloc(nlist * sizeof(unsigned long)); long k = 0;
    for (unsigned long s = 0; s <= (1UL << 22); s++) S[k++] = s;
    for (int r = 0; r < 4; r++) for (long dt = -86400; dt <= 86400; dt++) { S[k++] = runs[r] + dt; S[k++] = (runs[r] + dt) / 2; }
    for (int ps = 0; ps < 2; ps++) {
        #pragma omp parallel for reduction(+:hits) schedule(dynamic, 4096)
        for (long i = 0; i < k; i++) {
            rng_t g = {PS[ps][0], PS[ps][1], PS[ps][2]}; irand(&g, (uint32_t)S[i]);
            uint32_t w[256 + 64]; for (int t = 0; t < 256 + 64; t++) w[t] = brand(&g);
            hits += check_stream(w, 256 + 64, 256, ps, "irand", S[i]);
        }
        fprintf(stderr, "irand params %d done\n", ps);
    }
    printf("done: default state offsets < 2^20; irand seeds 0..2^22 and run times +-1 day (t, t/2), offsets < 256; hits %ld\n", hits);
    return 0;
}
